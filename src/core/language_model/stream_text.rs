//! Text Streaming impl for the `LanguageModelRequest` trait.

use crate::core::{
    AssistantMessage, LanguageModelStreamChunkType, Message, Messages, ToolCallInfo,
    ToolResultInfo,
    language_model::{
        LanguageModel, LanguageModelOptions, LanguageModelResponseContentType, LanguageModelStream,
        LanguageModelStreamChunk, Step, StopReason, Usage, request::LanguageModelRequest,
    },
    messages::TaggedMessage,
    utils::resolve_message,
};
use crate::error::Result;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

impl<M: LanguageModel> LanguageModelRequest<M> {
    /// Streams text generation and tool execution using the language model.
    ///
    /// This method performs streaming text generation, providing real-time access to response chunks
    /// as they are produced. It supports tool calling and execution in multiple steps, streaming
    /// intermediate results and handling tool interactions dynamically.
    ///
    /// For non-streaming responses, use [`generate_text`](Self::generate_text) instead.
    ///
    /// # Returns
    ///
    /// A [`StreamTextResponse`] containing the stream of chunks and final conversation state.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the underlying language model fails to generate a response
    /// or if tool execution encounters an error.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    ///# #[cfg(feature = "openai")]
    ///# {
    ///    use aisdk::{
    ///        core::{LanguageModelRequest, LanguageModelStreamChunkType},
    ///        providers::OpenAI,
    ///    };
    ///    use futures::StreamExt;
    ///
    ///    async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    ///        let openai = OpenAI::gpt_5();
    ///
    ///        let mut stream = LanguageModelRequest::builder()
    ///            .model(openai)
    ///            .prompt("What is the meaning of life?")
    ///            .build()
    ///            .stream_text()
    ///            .await?
    ///            .stream;
    ///
    ///         while let Some(chunk) = stream.next().await {
    ///             if let LanguageModelStreamChunkType::TextDelta(text) = chunk {
    ///                 println!("{}", text);
    ///             }
    ///         }
    ///        
    ///        Ok(())
    ///    }
    ///# }
    /// ```
    pub async fn stream_text(&mut self) -> Result<StreamTextResponse> {
        let (system_prompt, messages) = resolve_message(&self.options, &self.prompt);

        let options = Arc::new(Mutex::new(LanguageModelOptions {
            system: (!system_prompt.is_empty()).then_some(system_prompt),
            messages,
            schema: self.options.schema.to_owned(),
            stop_sequences: self.options.stop_sequences.to_owned(),
            tools: self.options.tools.to_owned(),
            stop_when: self.options.stop_when.clone(),
            on_step_start: self.options.on_step_start.clone(),
            on_step_finish: self.options.on_step_finish.clone(),
            stop_reason: None,
            headers: self.options.headers.clone(),
            body: self.options.body.clone(),
            ..self.options
        }));

        let (tx, stream) = LanguageModelStream::new();

        let mut model = self.model.clone();

        let thread_options = options.clone();
        tokio::spawn(async move {
            loop {
                let mut options = thread_options.lock().await;
                // Update the current step
                options.current_step_id += 1;
                let current_step_id = options.current_step_id;

                // Prepare the next step
                if let Some(hook) = options.on_step_start.clone() {
                    hook(&mut options);
                }

                let response_result = model.stream_text(options.clone()).await;
                let mut response = match response_result {
                    Ok(r) => r,
                    Err(e) => {
                        options.stop_reason = Some(StopReason::Error(e.clone()));
                        let _ = tx.send(LanguageModelStreamChunkType::Failed(format!(
                            "Model streaming failed: {e}"
                        )));
                        return Err(e);
                    }
                };

                while let Some(ref chunk) = response.next().await {
                    match chunk {
                        Ok(chunk) => {
                            let mut had_tool_call = false;
                            for output in chunk {
                                match output {
                                    LanguageModelStreamChunk::Done(final_msg) => {
                                        match final_msg.content {
                                            LanguageModelResponseContentType::Text(_) => {
                                                let assistant_msg =
                                                    Message::Assistant(AssistantMessage {
                                                        content: final_msg.content.clone(),
                                                        usage: final_msg.usage.clone(),
                                                    });
                                                options.messages.push(TaggedMessage::new(
                                                    current_step_id,
                                                    assistant_msg,
                                                ));
                                                options.stop_reason = Some(StopReason::Finish);
                                            }
                                            LanguageModelResponseContentType::Reasoning {
                                                ref content,
                                                ref extensions,
                                            } => {
                                                options.messages.push(TaggedMessage::new(
                                                current_step_id,
                                                Message::Assistant(AssistantMessage {
                                                    content:
                                                        LanguageModelResponseContentType::Reasoning {
                                                            content: content.clone(),
                                                            extensions: extensions.clone(),
                                                        },
                                                    usage: final_msg.usage.clone(),
                                                    }),
                                                ));
                                                options.stop_reason = Some(StopReason::Finish);
                                            }
                                            LanguageModelResponseContentType::ToolCall(
                                                ref tool_info,
                                            ) => {
                                                // add tool message
                                                let usage = final_msg.usage.clone();
                                                let _ = &options.messages.push(TaggedMessage::new(
                                                    current_step_id.to_owned(),
                                                    Message::Assistant(AssistantMessage::new(
                                                        LanguageModelResponseContentType::ToolCall(
                                                            tool_info.clone(),
                                                        ),
                                                        usage,
                                                    )),
                                                ));

                                                let _ = tx.send(
                                                    LanguageModelStreamChunkType::ToolCallAvailable(
                                                        tool_info.clone(),
                                                    ),
                                                );

                                                options
                                                    .handle_tool_call(tool_info, Some(tx.clone()))
                                                    .await;

                                                // Emit tool result AFTER execution (last message is the result)
                                                if let Some(TaggedMessage {
                                                    message: Message::Tool(result_info),
                                                    ..
                                                }) = options.messages.last()
                                                {
                                                    let _ = tx.send(
                                                        LanguageModelStreamChunkType::ToolCallEnd(
                                                            result_info.clone(),
                                                        ),
                                                    );
                                                }

                                                had_tool_call = true;
                                            }
                                            _ => {}
                                        }

                                        // Finish the step
                                        if let Some(ref hook) = options.on_step_finish {
                                            hook(&options);
                                        }

                                        // Stop If
                                        if let Some(hook) = &options.stop_when.clone()
                                            && hook(&options)
                                        {
                                            let _ =
                                                tx.send(LanguageModelStreamChunkType::Incomplete(
                                                    "Stopped by hook".to_string(),
                                                ));
                                            options.stop_reason = Some(StopReason::Hook);
                                            break;
                                        }
                                    }
                                    LanguageModelStreamChunk::Delta(other) => {
                                        let _ = tx.send(other.clone());
                                    }
                                }
                            }
                            // Prioritize continued tool call execution over text finishes
                            if had_tool_call
                                && matches!(options.stop_reason, Some(StopReason::Finish))
                            {
                                options.stop_reason = None;
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(LanguageModelStreamChunkType::Failed(e.to_string()));
                            options.stop_reason = Some(StopReason::Error(e.clone()));
                            break;
                        }
                    }

                    match options.stop_reason {
                        None => {}
                        _ => break,
                    };
                }

                match options.stop_reason {
                    None => {}
                    _ => break,
                };
            }

            drop(tx);

            Ok(())
        });

        let result = StreamTextResponse { stream, options };

        Ok(result)
    }
}

// ============================================================================
// Section: response types
// ============================================================================

/// Response from a streaming text generation call.
///
/// This struct contains the streaming response from a language model,
/// including the stream of chunks and the final options state.
pub struct StreamTextResponse {
    /// The stream of response chunks from the language model.
    pub stream: LanguageModelStream,
    // The reason the model stopped generating text.
    options: Arc<Mutex<LanguageModelOptions>>,
}

impl StreamTextResponse {
    /// Returns the step IDs of all messages in the conversation.
    ///
    /// This is primarily used for testing and debugging purposes.
    #[cfg(any(test, feature = "test-access"))]
    pub async fn step_ids(&self) -> Vec<usize> {
        self.options
            .lock()
            .await
            .messages
            .iter()
            .map(|t| t.step_id)
            .collect()
    }
}

impl StreamTextResponse {
    /// Returns all messages from the conversation.
    ///
    /// This includes system prompts, user inputs, assistant responses,
    /// and any tool-related messages that occurred during streaming.
    ///
    /// # Returns
    ///
    /// A vector of all [`Message`] instances in the conversation.
    pub async fn messages(&self) -> Messages {
        self.options.lock().await.messages()
    }

    /// Returns the conversation step with the specified index.
    ///
    /// A step represents all messages exchanged during one cycle of model interaction,
    /// including user input, assistant responses, and tool calls/results.
    ///
    /// # Parameters
    ///
    /// * `index` - The step ID to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<Step>` containing the step if it exists.
    pub async fn step(&self, index: usize) -> Option<Step> {
        self.options.lock().await.step(index)
    }

    /// Returns the most recent conversation step.
    ///
    /// This is equivalent to calling `step()` with the highest step ID.
    ///
    /// # Returns
    ///
    /// An `Option<Step>` containing the last step if any steps exist.
    pub async fn last_step(&self) -> Option<Step> {
        self.options.lock().await.last_step()
    }

    /// Returns all conversation steps in chronological order.
    ///
    /// Each step contains all messages exchanged during that cycle of interaction.
    ///
    /// # Returns
    ///
    /// A vector of all [`Step`] instances in order.
    pub async fn steps(&self) -> Vec<Step> {
        self.options.lock().await.steps()
    }

    /// Calculates the total token usage across all conversation steps.
    ///
    /// This aggregates input, output, reasoning, and cached token counts
    /// from all assistant messages in the conversation.
    ///
    /// # Returns
    ///
    /// A [`Usage`] struct containing the aggregated token statistics.
    pub async fn usage(&self) -> Usage {
        self.options.lock().await.usage()
    }

    /// Returns the content of the last assistant message, excluding reasoning.
    ///
    /// This provides access to the final output content from the language model,
    /// filtering out any reasoning content that may be present.
    ///
    /// # Returns
    ///
    /// An `Option<LanguageModelResponseContentType>` containing the content if available.
    pub async fn content(&self) -> Option<LanguageModelResponseContentType> {
        self.options.lock().await.content().cloned()
    }

    /// Returns the text content of the last assistant message.
    ///
    /// This extracts the plain text from the final assistant response,
    /// if the content type is text.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the text if the last message is text content.
    pub async fn text(&self) -> Option<String> {
        self.options.lock().await.text()
    }

    /// Extracts all tool execution results from the conversation.
    ///
    /// This collects all tool result messages that were generated during
    /// the streaming process, including results from tool calls.
    ///
    /// # Returns
    ///
    /// An `Option<Vec<ToolResultInfo>>` containing all tool results if any exist.
    pub async fn tool_results(&self) -> Option<Vec<ToolResultInfo>> {
        self.options.lock().await.tool_results()
    }

    /// Extracts all tool calls from the conversation.
    ///
    /// This collects all tool call requests that were made by the assistant
    /// during the streaming process.
    ///
    /// # Returns
    ///
    /// An `Option<Vec<ToolCallInfo>>` containing all tool calls if any exist.
    pub async fn tool_calls(&self) -> Option<Vec<ToolCallInfo>> {
        self.options.lock().await.tool_calls()
    }
    /// Returns the reason why text generation stopped.
    ///
    /// This indicates how and why the streaming process terminated,
    /// such as completion, error, or user-defined stop conditions.
    ///
    /// # Returns
    ///
    /// An `Option<StopReason>` indicating the termination reason if available.
    pub async fn stop_reason(&self) -> Option<StopReason> {
        self.options.lock().await.stop_reason()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::capabilities::ToolCallSupport;
    use crate::core::language_model::{
        LanguageModel, LanguageModelOptions, LanguageModelResponse,
        LanguageModelResponseContentType, LanguageModelStreamChunk, ProviderStream,
    };
    use crate::core::tools::{Tool, ToolCallInfo, ToolDetails, ToolExecute};
    use crate::core::{LanguageModelRequest, ToolContext};
    use crate::error::Result;
    use async_trait::async_trait;
    use futures::{StreamExt, stream};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[derive(Clone, Debug)]
    struct TestStreamingModel {
        calls: Arc<AtomicUsize>,
    }

    impl ToolCallSupport for TestStreamingModel {}

    #[async_trait]
    impl LanguageModel for TestStreamingModel {
        fn name(&self) -> String {
            "test-streaming-model".to_string()
        }

        async fn generate_text(
            &mut self,
            _options: LanguageModelOptions,
        ) -> Result<LanguageModelResponse> {
            unreachable!("generate_text is not used in this test")
        }

        async fn stream_text(&mut self, _options: LanguageModelOptions) -> Result<ProviderStream> {
            let call_index = self.calls.fetch_add(1, Ordering::SeqCst);

            let chunks = if call_index == 0 {
                let tool_call = ToolCallInfo {
                    tool: ToolDetails {
                        id: "call_1".to_string(),
                        name: "get_weather".to_string(),
                    },
                    input: json!({ "location": "dc" }),
                    extensions: Default::default(),
                };

                vec![Ok(vec![
                    LanguageModelStreamChunk::Delta(LanguageModelStreamChunkType::ToolCallStart(
                        tool_call.tool.clone(),
                    )),
                    LanguageModelStreamChunk::Delta(LanguageModelStreamChunkType::ToolCallDelta {
                        id: "call_1".to_string(),
                        delta: "{\"location\":\"dc\"}".to_string(),
                    }),
                    LanguageModelStreamChunk::Done(AssistantMessage {
                        content: LanguageModelResponseContentType::ToolCall(tool_call),
                        usage: None,
                    }),
                ])]
            } else {
                vec![Ok(vec![
                    LanguageModelStreamChunk::Delta(LanguageModelStreamChunkType::TextStart),
                    LanguageModelStreamChunk::Delta(LanguageModelStreamChunkType::TextDelta(
                        "done".to_string(),
                    )),
                    LanguageModelStreamChunk::Delta(LanguageModelStreamChunkType::TextEnd),
                    LanguageModelStreamChunk::Done(AssistantMessage {
                        content: LanguageModelResponseContentType::Text("done".to_string()),
                        usage: None,
                    }),
                ])]
            };

            Ok(Box::pin(stream::iter(chunks)))
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    struct GetWeatherInput {
        location: String,
    }

    fn get_weather() -> Tool {
        Tool {
            name: "get_weather".to_string(),
            description: "Get weather for a location".to_string(),
            input_schema: schemars::schema_for!(GetWeatherInput),
            execute: ToolExecute::from_sync(|_ctx: ToolContext, params: serde_json::Value| {
                let location = params["location"]
                    .as_str()
                    .ok_or_else(|| crate::Error::ToolCallError("missing location".to_string()))?;
                Ok(format!("The weather in {location} is sunny"))
            }),
        }
    }

    #[tokio::test]
    async fn stream_text_emits_tool_input_available_before_tool_result() {
        let response = LanguageModelRequest::builder()
            .model(TestStreamingModel {
                calls: Arc::new(AtomicUsize::new(0)),
            })
            .prompt("what is the weather in dc?")
            .with_tool(get_weather())
            .build()
            .stream_text()
            .await
            .expect("stream_text should succeed");

        let chunks = response.stream.take(4).collect::<Vec<_>>().await;

        assert!(matches!(
            chunks.first(),
            Some(LanguageModelStreamChunkType::ToolCallStart(_))
        ));
        assert!(matches!(
            chunks.get(1),
            Some(LanguageModelStreamChunkType::ToolCallDelta { .. })
        ));
        assert!(matches!(
            chunks.get(2),
            Some(LanguageModelStreamChunkType::ToolCallAvailable(_))
        ));
        assert!(matches!(
            chunks.get(3),
            Some(LanguageModelStreamChunkType::ToolCallEnd(_))
        ));
    }
}
