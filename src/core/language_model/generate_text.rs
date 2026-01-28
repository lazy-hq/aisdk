//! Text Generation impl for the `LanguageModelRequest` trait.

use crate::Error;
use crate::core::{
    AssistantMessage, Message,
    language_model::{
        LanguageModel, LanguageModelOptions, LanguageModelResponse,
        LanguageModelResponseContentType, StopReason, request::LanguageModelRequest,
    },
    messages::{TaggedMessage, TaggedMessageHelpers},
    tools::{ToolApprovalRequest, ToolApprovalResponse, ToolResultInfo},
    utils::resolve_message,
};
use crate::error::Result;
use serde::de::DeserializeOwned;
use serde::ser::Error as SerdeError;
use std::collections::HashMap;
use std::ops::Deref;

// ============================================================================
// Section: Tool Approval Helpers
// ============================================================================

/// Result of collecting tool approvals from messages.
#[derive(Debug, Default)]
struct CollectedApprovals {
    /// Tool calls that were approved and should be executed.
    approved: Vec<(ToolApprovalRequest, ToolApprovalResponse)>,
    /// Tool calls that were denied and should not be executed.
    denied: Vec<(ToolApprovalRequest, ToolApprovalResponse)>,
}

/// Collects tool approval responses from messages and matches them with pending requests.
fn collect_tool_approvals(messages: &[TaggedMessage]) -> CollectedApprovals {
    let mut result = CollectedApprovals::default();

    // Find all pending approval requests
    let pending_requests: Vec<ToolApprovalRequest> = messages
        .extract_tool_approval_requests()
        .unwrap_or_default();

    // Find all approval responses
    let responses: Vec<ToolApprovalResponse> = messages
        .extract_tool_approval_responses()
        .unwrap_or_default();

    // Build a map of approval_id -> response for quick lookup
    let response_map: HashMap<String, ToolApprovalResponse> = responses
        .into_iter()
        .map(|r| (r.approval_id.clone(), r))
        .collect();

    // Match requests with responses
    for request in pending_requests {
        if let Some(response) = response_map.get(&request.approval_id) {
            if response.approved {
                result.approved.push((request, response.clone()));
            } else {
                result.denied.push((request, response.clone()));
            }
        }
    }

    result
}

/// Checks if there are any pending approval requests that haven't been responded to.
fn has_pending_approval_requests(messages: &[TaggedMessage]) -> bool {
    let pending_requests: Vec<ToolApprovalRequest> = messages
        .extract_tool_approval_requests()
        .unwrap_or_default();

    let responses: Vec<ToolApprovalResponse> = messages
        .extract_tool_approval_responses()
        .unwrap_or_default();

    let response_ids: std::collections::HashSet<_> =
        responses.iter().map(|r| &r.approval_id).collect();

    // Check if any request doesn't have a matching response
    pending_requests
        .iter()
        .any(|req| !response_ids.contains(&req.approval_id))
}

impl<M: LanguageModel> LanguageModelRequest<M> {
    /// Generates text and executes tools using the language model.
    ///
    /// This method performs non-streaming text generation, potentially involving multiple
    /// steps of tool calling and execution until the conversation reaches a natural stopping point.
    /// The model may call tools based on the configured options, and responses are processed
    /// iteratively until completion.
    ///
    /// For streaming responses, use [`stream_text`](Self::stream_text) instead.
    ///
    /// # Returns
    ///
    /// A [`GenerateTextResponse`] containing the final conversation state and generated content.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the underlying language model fails to generate a response
    /// or if tool execution encounters an error.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    ///# #[cfg(feature = "openai")]
    ///# {
    ///    use aisdk::{
    ///        core::{LanguageModelRequest},
    ///        providers::OpenAI,
    ///    };
    ///
    ///    async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    ///        let openai = OpenAI::gpt_5();
    ///
    ///        let result = LanguageModelRequest::builder()
    ///            .model(openai)
    ///            .prompt("What is the meaning of life?")
    ///            .build()
    ///            .generate_text()
    ///            .await?;
    ///
    ///        println!("{}", result.text().unwrap());
    ///        Ok(())
    ///    }
    ///# }
    /// ```
    ///
    pub async fn generate_text(&mut self) -> Result<GenerateTextResponse> {
        let (system_prompt, messages) = resolve_message(&self.options, &self.prompt);

        let mut options = LanguageModelOptions {
            system: (!system_prompt.is_empty()).then_some(system_prompt),
            messages,
            schema: self.options.schema.to_owned(),
            stop_sequences: self.options.stop_sequences.to_owned(),
            tools: self.options.tools.to_owned(),
            stop_when: self.options.stop_when.clone(),
            on_step_start: self.options.on_step_start.clone(),
            on_step_finish: self.options.on_step_finish.clone(),
            stop_reason: None,
            ..self.options
        };

        // Process any pending tool approvals at the start
        let collected = collect_tool_approvals(&options.messages);

        // Execute approved tools
        for (request, _response) in collected.approved {
            options.handle_tool_call(&request.tool_call).await;
        }

        // Add denial info for denied tools (so the model knows the tool was denied)
        for (request, response) in collected.denied {
            let denial_message = response
                .reason
                .unwrap_or_else(|| "Tool execution was denied by user".to_string());
            let mut tool_result = ToolResultInfo::new(&request.tool_call.tool.name);
            tool_result.id(&request.tool_call.tool.id);
            tool_result.output = Ok(serde_json::Value::String(format!(
                "Tool execution denied: {}",
                denial_message
            )));
            options.messages.push(TaggedMessage::new(
                options.current_step_id,
                Message::Tool(tool_result),
            ));
        }

        // If we just processed approvals, continue the loop normally
        // But first check if there are still pending approvals
        if has_pending_approval_requests(&options.messages) {
            // There are still pending approval requests - stop and wait for more responses
            options.stop_reason = Some(StopReason::Other("Waiting for tool approval".to_string()));
            return Ok(GenerateTextResponse { options });
        }

        loop {
            // Update the current step
            options.current_step_id += 1;

            // Prepare the next step
            if let Some(hook) = options.on_step_start.clone() {
                hook(&mut options);
            }

            let response: LanguageModelResponse = self
                .model
                .generate_text(options.clone())
                .await
                .inspect_err(|e| {
                options.stop_reason = Some(StopReason::Error(e.clone()));
            })?;

            // Track if we have any tool calls requiring approval in this step
            let mut has_approval_requests = false;

            for output in response.contents.iter() {
                match output {
                    LanguageModelResponseContentType::Text(text) => {
                        let assistant_msg = Message::Assistant(AssistantMessage {
                            content: text.clone().into(),
                            usage: response.usage.clone(),
                        });
                        options
                            .messages
                            .push(TaggedMessage::new(options.current_step_id, assistant_msg));
                    }
                    LanguageModelResponseContentType::Reasoning {
                        content,
                        extensions,
                    } => {
                        let assistant_msg = Message::Assistant(AssistantMessage {
                            content: LanguageModelResponseContentType::Reasoning {
                                content: content.clone(),
                                extensions: extensions.clone(),
                            },
                            usage: response.usage.clone(),
                        });
                        options
                            .messages
                            .push(TaggedMessage::new(options.current_step_id, assistant_msg));
                    }
                    LanguageModelResponseContentType::ToolCall(tool_info) => {
                        // Check if this tool requires approval
                        let needs_approval = if let Some(tools) = &options.tools {
                            let current_messages: Vec<Message> =
                                options.messages.iter().map(|t| t.message.clone()).collect();
                            tools.needs_approval(tool_info, &current_messages)
                        } else {
                            false
                        };

                        if needs_approval {
                            // Create approval request instead of executing
                            let approval_request = ToolApprovalRequest::new(tool_info.clone());
                            let usage = response.usage.clone();
                            options.messages.push(TaggedMessage::new(
                                options.current_step_id,
                                Message::Assistant(AssistantMessage::new(
                                    LanguageModelResponseContentType::ToolApprovalRequest(
                                        approval_request,
                                    ),
                                    usage,
                                )),
                            ));
                            has_approval_requests = true;
                        } else {
                            // Execute tool immediately (original behavior)
                            let usage = response.usage.clone();
                            let _ = &options.messages.push(TaggedMessage::new(
                                options.current_step_id.to_owned(),
                                Message::Assistant(AssistantMessage::new(
                                    LanguageModelResponseContentType::ToolCall(tool_info.clone()),
                                    usage,
                                )),
                            ));
                            options.handle_tool_call(tool_info).await;
                        }
                    }
                    _ => (),
                }
            }

            // Finish the step
            if let Some(ref hook) = options.on_step_finish {
                hook(&options);
            };

            // If we have approval requests, stop and return them to the user
            if has_approval_requests {
                options.stop_reason =
                    Some(StopReason::Other("Waiting for tool approval".to_string()));
                break;
            }

            if response.contents.is_empty() {
                options.stop_reason = Some(StopReason::Error(Error::Other(
                    "Language model returned empty response".to_string(),
                )));
                break;
            }

            // Stop If
            if let Some(hook) = &options.stop_when.clone()
                && hook(&options)
            {
                options.stop_reason = Some(StopReason::Hook);
                break;
            }

            match response.contents.last() {
                Some(LanguageModelResponseContentType::ToolCall(_)) => (),
                _ => {
                    options.stop_reason = Some(StopReason::Finish);
                    break;
                }
            };
        }

        Ok(GenerateTextResponse { options })
    }
}

// ============================================================================
// Section: response types
// ============================================================================

/// Response from a generate call on `GenerateText`.
#[derive(Debug, Clone)]
pub struct GenerateTextResponse {
    /// The options that generated this response
    pub options: LanguageModelOptions,
}

impl GenerateTextResponse {
    /// Deserializes the response text into a structured type.
    ///
    /// This method attempts to parse the generated text as JSON and deserialize it
    /// into the specified type `T`. It requires that the response contains text content.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize into, which must implement [`DeserializeOwned`].
    ///
    /// # Returns
    ///
    /// A result containing the deserialized value or a JSON error.
    ///
    /// # Errors
    ///
    /// Returns an error if there is no text response or if deserialization fails.
    pub fn into_schema<T: DeserializeOwned>(&self) -> std::result::Result<T, serde_json::Error> {
        if let Some(text) = &self.text() {
            serde_json::from_str(text)
        } else {
            Err(serde_json::Error::custom("No text response found"))
        }
    }

    /// Returns any pending tool approval requests that need user response.
    ///
    /// These requests are generated when tools with `needs_approval` set to `Always`
    /// or with a `Dynamic` function that returns `true` are called by the model.
    ///
    /// # Returns
    ///
    /// An `Option<Vec<ToolApprovalRequest>>` containing the pending requests if any exist.
    pub fn pending_tool_approvals(&self) -> Option<Vec<ToolApprovalRequest>> {
        self.options
            .messages
            .as_slice()
            .extract_tool_approval_requests()
    }

    /// Checks if there are any pending tool approval requests.
    ///
    /// This is useful for determining if the response requires user interaction
    /// before continuing the conversation.
    ///
    /// # Returns
    ///
    /// `true` if there are pending approval requests, `false` otherwise.
    pub fn has_pending_approvals(&self) -> bool {
        has_pending_approval_requests(&self.options.messages)
    }

    #[cfg(any(test, feature = "test-access"))]
    /// Returns the step ids of the messages in the response.
    pub fn step_ids(&self) -> Vec<usize> {
        self.options.messages.iter().map(|t| t.step_id).collect()
    }
}

impl Deref for GenerateTextResponse {
    type Target = LanguageModelOptions;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        AssistantMessage,
        language_model::{LanguageModelResponseContentType, Usage},
        messages::TaggedMessage,
        tools::{ToolCallInfo, ToolResultInfo},
    };

    #[test]
    fn test_generate_text_response_step() {
        let options = LanguageModelOptions {
            messages: vec![
                TaggedMessage::new(0, Message::System("System".to_string().into())),
                TaggedMessage::new(0, Message::User("User".to_string().into())),
                TaggedMessage::new(
                    1,
                    Message::Assistant(AssistantMessage {
                        content: LanguageModelResponseContentType::Text("Assistant".to_string()),
                        usage: None,
                    }),
                ),
            ],
            ..Default::default()
        };
        let response = GenerateTextResponse { options };

        let step0 = response.step(0).unwrap();
        assert_eq!(step0.step_id, 0);
        assert_eq!(step0.messages.len(), 2);

        let step1 = response.step(1).unwrap();
        assert_eq!(step1.step_id, 1);
        assert_eq!(step1.messages.len(), 1);

        assert!(response.step(2).is_none());
    }

    #[test]
    fn test_generate_text_response_final_step() {
        let options = LanguageModelOptions {
            messages: vec![
                TaggedMessage::new(0, Message::System("System".to_string().into())),
                TaggedMessage::new(1, Message::User("User".to_string().into())),
                TaggedMessage::new(
                    2,
                    Message::Assistant(AssistantMessage {
                        content: LanguageModelResponseContentType::Text("Assistant".to_string()),
                        usage: None,
                    }),
                ),
            ],
            ..Default::default()
        };
        let response = GenerateTextResponse { options };

        let final_step = response.last_step().unwrap();
        assert_eq!(final_step.step_id, 2);
        assert_eq!(final_step.messages.len(), 1);
    }

    #[test]
    fn test_generate_text_response_steps() {
        let options = LanguageModelOptions {
            messages: vec![
                TaggedMessage::new(0, Message::System("System".to_string().into())),
                TaggedMessage::new(0, Message::User("User".to_string().into())),
                TaggedMessage::new(
                    1,
                    Message::Assistant(AssistantMessage {
                        content: LanguageModelResponseContentType::Text("Assistant1".to_string()),
                        usage: None,
                    }),
                ),
                TaggedMessage::new(
                    2,
                    Message::Assistant(AssistantMessage {
                        content: LanguageModelResponseContentType::Text("Assistant2".to_string()),
                        usage: None,
                    }),
                ),
            ],
            ..Default::default()
        };
        let response = GenerateTextResponse { options };

        let steps = response.steps();
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0].step_id, 0);
        assert_eq!(steps[0].messages.len(), 2);
        assert_eq!(steps[1].step_id, 1);
        assert_eq!(steps[1].messages.len(), 1);
        assert_eq!(steps[2].step_id, 2);
        assert_eq!(steps[2].messages.len(), 1);
    }

    #[test]
    fn test_generate_text_response_usage() {
        let options = LanguageModelOptions {
            messages: vec![
                TaggedMessage::new(0, Message::System("System".to_string().into())),
                TaggedMessage::new(
                    1,
                    Message::Assistant(AssistantMessage {
                        content: LanguageModelResponseContentType::Text("Assistant1".to_string()),
                        usage: Some(Usage {
                            input_tokens: Some(10),
                            output_tokens: Some(5),
                            reasoning_tokens: Some(2),
                            cached_tokens: Some(1),
                        }),
                    }),
                ),
                TaggedMessage::new(
                    2,
                    Message::Assistant(AssistantMessage {
                        content: LanguageModelResponseContentType::Text("Assistant2".to_string()),
                        usage: Some(Usage {
                            input_tokens: Some(5),
                            output_tokens: Some(3),
                            reasoning_tokens: Some(1),
                            cached_tokens: Some(0),
                        }),
                    }),
                ),
            ],
            ..Default::default()
        };
        let response = GenerateTextResponse { options };

        let total_usage = response.usage();
        assert_eq!(total_usage.input_tokens, Some(15));
        assert_eq!(total_usage.output_tokens, Some(8));
        assert_eq!(total_usage.reasoning_tokens, Some(3));
        assert_eq!(total_usage.cached_tokens, Some(1));
    }

    fn create_tool_call_message(step_id: usize, tool_name: &str) -> TaggedMessage {
        TaggedMessage::new(
            step_id,
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::ToolCall(ToolCallInfo::new(tool_name)),
                usage: None,
            }),
        )
    }

    fn create_tool_result_message(step_id: usize, tool_name: &str) -> TaggedMessage {
        TaggedMessage::new(step_id, Message::Tool(ToolResultInfo::new(tool_name)))
    }

    fn create_text_assistant_message(step_id: usize, text: &str) -> TaggedMessage {
        TaggedMessage::new(
            step_id,
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::Text(text.to_string()),
                usage: None,
            }),
        )
    }

    fn create_response_with_messages(messages: Vec<TaggedMessage>) -> GenerateTextResponse {
        let options = LanguageModelOptions {
            messages,
            ..Default::default()
        };
        GenerateTextResponse { options }
    }

    // Tests for GenerateTextResponse tool_calls()
    #[test]
    fn test_generate_text_response_tool_calls_empty_messages() {
        let response = create_response_with_messages(vec![]);
        assert_eq!(response.tool_calls(), None);
    }

    #[test]
    fn test_generate_text_response_tool_calls_only_non_assistant_messages() {
        let messages = vec![
            TaggedMessage::new(0, Message::System("System".to_string().into())),
            TaggedMessage::new(0, Message::User("User".to_string().into())),
            create_tool_result_message(0, "tool1"),
        ];
        let response = create_response_with_messages(messages);
        assert_eq!(response.tool_calls(), None);
    }

    #[test]
    fn test_generate_text_response_tool_calls_single_assistant_with_tool_call() {
        let messages = vec![create_tool_call_message(0, "test_tool")];
        let response = create_response_with_messages(messages);
        let calls = response.tool_calls().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].tool.name, "test_tool");
    }

    #[test]
    fn test_generate_text_response_tool_calls_multiple_assistant_with_tool_calls_different_steps() {
        let messages = vec![
            create_tool_call_message(0, "tool1"),
            create_tool_call_message(1, "tool2"),
            create_tool_call_message(2, "tool3"),
        ];
        let response = create_response_with_messages(messages);
        let calls = response.tool_calls().unwrap();
        assert_eq!(calls.len(), 3);
        assert_eq!(calls[0].tool.name, "tool1");
        assert_eq!(calls[1].tool.name, "tool2");
        assert_eq!(calls[2].tool.name, "tool3");
    }

    #[test]
    fn test_generate_text_response_tool_calls_assistant_without_tool_call() {
        let messages = vec![create_text_assistant_message(0, "Hello")];
        let response = create_response_with_messages(messages);
        assert_eq!(response.tool_calls(), None);
    }

    #[test]
    fn test_generate_text_response_tool_calls_mixed_message_types_multiple_steps() {
        let messages = vec![
            TaggedMessage::new(0, Message::System("System".to_string().into())),
            TaggedMessage::new(0, Message::User("User".to_string().into())),
            create_tool_call_message(1, "test_tool"),
            create_tool_result_message(1, "other_tool"),
            create_tool_call_message(2, "another_tool"),
        ];
        let response = create_response_with_messages(messages);
        let calls = response.tool_calls().unwrap();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].tool.name, "test_tool");
        assert_eq!(calls[1].tool.name, "another_tool");
    }

    #[test]
    fn test_generate_text_response_tool_calls_duplicate_tool_calls() {
        let messages = vec![
            create_tool_call_message(0, "tool1"),
            create_tool_call_message(1, "tool1"), // Same name
            create_tool_call_message(2, "tool1"), // Same name again
        ];
        let response = create_response_with_messages(messages);
        let calls = response.tool_calls().unwrap();
        assert_eq!(calls.len(), 3);
        assert_eq!(calls[0].tool.name, "tool1");
        assert_eq!(calls[1].tool.name, "tool1");
        assert_eq!(calls[2].tool.name, "tool1");
    }

    #[test]
    fn test_generate_text_response_tool_calls_from_specific_steps_only() {
        let messages = vec![
            TaggedMessage::new(0, Message::System("System".to_string().into())),
            create_tool_call_message(1, "tool_from_step1"),
            TaggedMessage::new(1, Message::User("User".to_string().into())),
            create_tool_call_message(2, "tool_from_step2"),
            create_tool_result_message(2, "result_from_step2"),
            create_tool_call_message(3, "tool_from_step3"),
        ];
        let response = create_response_with_messages(messages);
        let calls = response.tool_calls().unwrap();
        assert_eq!(calls.len(), 3);
        assert_eq!(calls[0].tool.name, "tool_from_step1");
        assert_eq!(calls[1].tool.name, "tool_from_step2");
        assert_eq!(calls[2].tool.name, "tool_from_step3");
    }

    // Tests for GenerateTextResponse tool_results()
    #[test]
    fn test_generate_text_response_tool_results_empty_messages() {
        let response = create_response_with_messages(vec![]);
        assert!(response.tool_results().is_none());
    }

    #[test]
    fn test_generate_text_response_tool_results_only_non_tool_messages() {
        let messages = vec![
            TaggedMessage::new(0, Message::System("System".to_string().into())),
            TaggedMessage::new(0, Message::User("User".to_string().into())),
            create_text_assistant_message(0, "Assistant"),
        ];
        let response = create_response_with_messages(messages);
        assert!(response.tool_results().is_none());
    }

    #[test]
    fn test_generate_text_response_tool_results_single_tool_message() {
        let messages = vec![create_tool_result_message(0, "test_tool")];
        let response = create_response_with_messages(messages);
        let results = response.tool_results().unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].tool.name, "test_tool");
    }

    #[test]
    fn test_generate_text_response_tool_results_multiple_tool_messages_different_steps() {
        let messages = vec![
            create_tool_result_message(0, "tool1"),
            create_tool_result_message(1, "tool2"),
            create_tool_result_message(2, "tool3"),
        ];
        let response = create_response_with_messages(messages);
        let results = response.tool_results().unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].tool.name, "tool1");
        assert_eq!(results[1].tool.name, "tool2");
        assert_eq!(results[2].tool.name, "tool3");
    }

    #[test]
    fn test_generate_text_response_tool_results_mixed_message_types() {
        let messages = vec![
            TaggedMessage::new(0, Message::System("System".to_string().into())),
            TaggedMessage::new(0, Message::User("User".to_string().into())),
            create_tool_result_message(1, "test_tool"),
            create_text_assistant_message(1, "Assistant"),
            create_tool_result_message(2, "another_tool"),
        ];
        let response = create_response_with_messages(messages);
        let results = response.tool_results().unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].tool.name, "test_tool");
        assert_eq!(results[1].tool.name, "another_tool");
    }

    #[test]
    fn test_generate_text_response_tool_results_no_tool_messages_but_others_present() {
        let messages = vec![
            TaggedMessage::new(0, Message::System("System".to_string().into())),
            TaggedMessage::new(0, Message::User("User".to_string().into())),
            create_text_assistant_message(0, "Assistant"),
        ];
        let response = create_response_with_messages(messages);
        assert!(response.tool_results().is_none());
    }

    #[test]
    fn test_generate_text_response_tool_results_duplicate_tool_entries() {
        let messages = vec![
            create_tool_result_message(0, "tool1"),
            create_tool_result_message(1, "tool1"), // Same name
            create_tool_result_message(2, "tool1"), // Same name again
        ];
        let response = create_response_with_messages(messages);
        let results = response.tool_results().unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].tool.name, "tool1");
        assert_eq!(results[1].tool.name, "tool1");
        assert_eq!(results[2].tool.name, "tool1");
    }

    #[test]
    fn test_generate_text_response_tool_results_preserving_original_message_order() {
        let messages = vec![
            TaggedMessage::new(0, Message::System("System".to_string().into())),
            create_tool_result_message(1, "tool1"),
            TaggedMessage::new(1, Message::User("User".to_string().into())),
            create_tool_result_message(2, "tool2"),
            create_text_assistant_message(2, "Assistant"),
            create_tool_result_message(3, "tool3"),
        ];
        let response = create_response_with_messages(messages);
        let results = response.tool_results().unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].tool.name, "tool1");
        assert_eq!(results[1].tool.name, "tool2");
        assert_eq!(results[2].tool.name, "tool3");
    }

    #[test]
    fn test_generate_text_response_tool_results_large_number_of_messages() {
        let mut messages = Vec::new();
        // Add 1000 messages with tool results interspersed
        for i in 0..1000 {
            messages.push(create_tool_result_message(0, &format!("tool{}", i)));
            if i % 100 == 0 {
                messages.push(TaggedMessage::new(
                    0,
                    Message::User(format!("User message {}", i).into()),
                ));
            }
        }
        let response = create_response_with_messages(messages);
        let results = response.tool_results().unwrap();
        assert_eq!(results.len(), 1000);
        for (i, result) in results.iter().enumerate() {
            assert_eq!(result.tool.name, format!("tool{}", i));
        }
    }

    // ============================================================================
    // Tool Approval Tests
    // ============================================================================

    fn create_approval_request_message(
        step_id: usize,
        tool_name: &str,
        approval_id: &str,
    ) -> TaggedMessage {
        let mut tool_call = ToolCallInfo::new(tool_name);
        tool_call.id(format!("{}_id", tool_name));
        let approval_request = ToolApprovalRequest::with_id(approval_id, tool_call);
        TaggedMessage::new(
            step_id,
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::ToolApprovalRequest(approval_request),
                usage: None,
            }),
        )
    }

    fn create_approval_response_message(
        step_id: usize,
        approval_id: &str,
        approved: bool,
    ) -> TaggedMessage {
        use crate::core::tools::ToolApprovalResponse;
        TaggedMessage::new(
            step_id,
            Message::ToolApproval(ToolApprovalResponse::new(approval_id, approved)),
        )
    }

    #[test]
    fn test_collect_tool_approvals_empty() {
        let messages: Vec<TaggedMessage> = vec![];
        let collected = collect_tool_approvals(&messages);
        assert!(collected.approved.is_empty());
        assert!(collected.denied.is_empty());
    }

    #[test]
    fn test_collect_tool_approvals_single_approved() {
        let messages = vec![
            create_approval_request_message(0, "test_tool", "approval-1"),
            create_approval_response_message(0, "approval-1", true),
        ];
        let collected = collect_tool_approvals(&messages);
        assert_eq!(collected.approved.len(), 1);
        assert_eq!(collected.approved[0].0.approval_id, "approval-1");
        assert!(collected.denied.is_empty());
    }

    #[test]
    fn test_collect_tool_approvals_single_denied() {
        let messages = vec![
            create_approval_request_message(0, "test_tool", "approval-1"),
            create_approval_response_message(0, "approval-1", false),
        ];
        let collected = collect_tool_approvals(&messages);
        assert!(collected.approved.is_empty());
        assert_eq!(collected.denied.len(), 1);
        assert_eq!(collected.denied[0].0.approval_id, "approval-1");
    }

    #[test]
    fn test_collect_tool_approvals_mixed() {
        let messages = vec![
            create_approval_request_message(0, "tool1", "approval-1"),
            create_approval_request_message(0, "tool2", "approval-2"),
            create_approval_response_message(0, "approval-1", true),
            create_approval_response_message(0, "approval-2", false),
        ];
        let collected = collect_tool_approvals(&messages);
        assert_eq!(collected.approved.len(), 1);
        assert_eq!(collected.denied.len(), 1);
        assert_eq!(collected.approved[0].0.approval_id, "approval-1");
        assert_eq!(collected.denied[0].0.approval_id, "approval-2");
    }

    #[test]
    fn test_collect_tool_approvals_unmatched_request() {
        let messages = vec![
            create_approval_request_message(0, "test_tool", "approval-1"),
            // No response for approval-1
        ];
        let collected = collect_tool_approvals(&messages);
        assert!(collected.approved.is_empty());
        assert!(collected.denied.is_empty());
    }

    #[test]
    fn test_has_pending_approval_requests_none() {
        let messages: Vec<TaggedMessage> = vec![];
        assert!(!has_pending_approval_requests(&messages));
    }

    #[test]
    fn test_has_pending_approval_requests_with_pending() {
        let messages = vec![create_approval_request_message(
            0,
            "test_tool",
            "approval-1",
        )];
        assert!(has_pending_approval_requests(&messages));
    }

    #[test]
    fn test_has_pending_approval_requests_all_responded() {
        let messages = vec![
            create_approval_request_message(0, "test_tool", "approval-1"),
            create_approval_response_message(0, "approval-1", true),
        ];
        assert!(!has_pending_approval_requests(&messages));
    }

    #[test]
    fn test_has_pending_approval_requests_partial_response() {
        let messages = vec![
            create_approval_request_message(0, "tool1", "approval-1"),
            create_approval_request_message(0, "tool2", "approval-2"),
            create_approval_response_message(0, "approval-1", true),
            // approval-2 is still pending
        ];
        assert!(has_pending_approval_requests(&messages));
    }

    #[test]
    fn test_generate_text_response_pending_tool_approvals() {
        let messages = vec![
            create_approval_request_message(0, "test_tool", "approval-1"),
            create_approval_request_message(0, "another_tool", "approval-2"),
        ];
        let response = create_response_with_messages(messages);
        let pending = response.pending_tool_approvals().unwrap();
        assert_eq!(pending.len(), 2);
        assert_eq!(pending[0].approval_id, "approval-1");
        assert_eq!(pending[1].approval_id, "approval-2");
    }

    #[test]
    fn test_generate_text_response_has_pending_approvals() {
        let messages = vec![create_approval_request_message(
            0,
            "test_tool",
            "approval-1",
        )];
        let response = create_response_with_messages(messages);
        assert!(response.has_pending_approvals());
    }

    #[test]
    fn test_generate_text_response_no_pending_approvals() {
        let messages = vec![
            create_approval_request_message(0, "test_tool", "approval-1"),
            create_approval_response_message(0, "approval-1", true),
        ];
        let response = create_response_with_messages(messages);
        assert!(!response.has_pending_approvals());
    }
}
