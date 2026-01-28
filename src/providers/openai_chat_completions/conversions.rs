//! Helper functions and conversions for the OpenAI Chat Completions provider.

use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponseContentType, ReasoningEffort, Usage,
};
use crate::core::messages::Message;
use crate::core::tools::Tool as SdkTool;
use crate::providers::openai_chat_completions::client::{self, types};

// ============================================================================
// LanguageModelOptions -> ChatCompletionsOptions
// ============================================================================

impl From<LanguageModelOptions> for client::ChatCompletionsOptions {
    fn from(options: LanguageModelOptions) -> Self {
        let mut messages: Vec<types::ChatMessage> = Vec::new();

        if let Some(system_prompt) = options.system {
            messages.push(types::ChatMessage {
                role: types::Role::System,
                content: Some(system_prompt),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            });
        }

        messages.extend(
            options
                .messages
                .into_iter()
                .map(|tagged| tagged.message.into()),
        );

        let tools: Option<Vec<types::Tool>> = options.tools.map(|tool_list| {
            tool_list
                .tools
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .iter()
                .map(|t| t.clone().into())
                .collect()
        });

        let response_format = options.schema.map(|schema| {
            let json_value = serde_json::to_value(schema).unwrap();
            types::ResponseFormat::JsonSchema {
                json_schema: types::JsonSchemaDefinition {
                    name: json_value
                        .get("title")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Response")
                        .to_string(),
                    schema: json_value.clone(),
                    description: json_value
                        .get("description")
                        .and_then(|v| v.as_str())
                        .map(str::to_string),
                    strict: Some(true),
                },
            }
        });

        let reasoning_effort = options.reasoning_effort.map(|effort| {
            match effort {
                ReasoningEffort::Low => "low",
                ReasoningEffort::Medium => "medium",
                ReasoningEffort::High => "high",
            }
            .to_string()
        });

        client::ChatCompletionsOptions {
            model: "".to_string(),
            messages,
            frequency_penalty: options.frequency_penalty,
            logit_bias: None,
            logprobs: None,
            top_logprobs: None,
            max_completion_tokens: options.max_output_tokens,
            n: None,
            presence_penalty: options.presence_penalty,
            response_format,
            seed: options.seed,
            stop: options.stop_sequences.map(|seqs| {
                if seqs.len() == 1 {
                    types::StopSequences::Single(seqs[0].clone())
                } else {
                    types::StopSequences::Multiple(seqs.into_iter().take(4).collect())
                }
            }),
            stream: Some(false),
            stream_options: None,
            temperature: options.temperature.map(|t| t as f32 / 100.0),
            top_p: options.top_p.map(|t| t as f32 / 100.0),
            tools,
            tool_choice: Some(types::ToolChoice::String("auto".to_string())),
            parallel_tool_calls: Some(true),
            reasoning_effort,
            verbosity: None,
        }
    }
}

// ============================================================================
// SDK Message -> ChatMessage
// ============================================================================

impl From<Message> for types::ChatMessage {
    fn from(msg: Message) -> Self {
        match msg {
            Message::System(s) => types::ChatMessage {
                role: types::Role::System,
                content: Some(s.content),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            Message::User(u) => types::ChatMessage {
                role: types::Role::User,
                content: Some(u.content),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            Message::Assistant(a) => match a.content {
                LanguageModelResponseContentType::Text(text) => types::ChatMessage {
                    role: types::Role::Assistant,
                    content: Some(text),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
                LanguageModelResponseContentType::ToolCall(tool_info) => types::ChatMessage {
                    role: types::Role::Assistant,
                    content: Some("".to_string()),
                    name: None,
                    tool_calls: Some(vec![types::ToolCall {
                        id: tool_info.tool.id.clone(),
                        type_: "function".to_string(),
                        function: types::FunctionCall {
                            name: tool_info.tool.name.clone(),
                            arguments: tool_info.input.to_string(),
                        },
                    }]),
                    tool_call_id: None,
                },
                LanguageModelResponseContentType::Reasoning { content, .. } => {
                    // Chat Completions doesn't have separate reasoning messages
                    // Include as text with prefix
                    types::ChatMessage {
                        role: types::Role::Assistant,
                        content: Some(format!("[Reasoning]: {}", content)),
                        name: None,
                        tool_calls: None,
                        tool_call_id: None,
                    }
                }
                _ => types::ChatMessage {
                    role: types::Role::Assistant,
                    content: None,
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
            },
            Message::Tool(tool_result) => types::ChatMessage {
                role: types::Role::Tool,
                content: Some(
                    tool_result
                        .output
                        .unwrap_or_else(|e| serde_json::Value::String(e.to_string()))
                        .to_string(),
                ),
                name: Some(tool_result.tool.name),
                tool_calls: None,
                tool_call_id: Some(tool_result.tool.id),
            },
            Message::Developer(d) => types::ChatMessage {
                role: types::Role::Developer,
                content: Some(d),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            // Tool approval messages are handled internally by the SDK
            Message::ToolApproval(_) => types::ChatMessage {
                role: types::Role::User,
                content: None,
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        }
    }
}

// ============================================================================
// SDK Tool -> ChatCompletions Tool
// ============================================================================

impl From<SdkTool> for types::Tool {
    fn from(tool: SdkTool) -> Self {
        let mut params = tool.input_schema.to_value();

        // Remove schema metadata fields that may conflict with strict mode
        if let serde_json::Value::Object(ref mut obj) = params {
            obj.remove("$schema");
            obj.remove("title");
        }

        // Ensure required fields for OpenAI Chat Completions
        params["type"] = serde_json::Value::String("object".to_string());
        params["additionalProperties"] = serde_json::Value::Bool(false);

        if !params
            .get("properties")
            .map(|p| p.is_object())
            .unwrap_or(false)
        {
            params["properties"] = serde_json::Value::Object(serde_json::Map::new());
        }

        types::Tool {
            type_: "function".to_string(),
            function: types::FunctionDefinition {
                name: tool.name,
                description: Some(tool.description),
                parameters: params,
                strict: Some(true),
            },
        }
    }
}

// ============================================================================
// ChatCompletions Usage -> SDK Usage
// ============================================================================

impl From<types::Usage> for Usage {
    fn from(usage: types::Usage) -> Self {
        Self {
            input_tokens: Some(usage.prompt_tokens as usize),
            output_tokens: Some(usage.completion_tokens as usize),
            reasoning_tokens: usage
                .completion_tokens_details
                .map(|d| d.reasoning_tokens as usize),
            cached_tokens: usage
                .prompt_tokens_details
                .map(|d| d.cached_tokens as usize),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_conversion_system() {
        let msg = Message::System("You are helpful".to_string().into());
        let chat_msg: types::ChatMessage = msg.into();

        assert_eq!(chat_msg.role, types::Role::System);
        assert_eq!(chat_msg.content, Some("You are helpful".to_string()));
        assert!(chat_msg.tool_calls.is_none());
    }

    #[test]
    fn test_message_conversion_user() {
        let msg = Message::User("Hello".to_string().into());
        let chat_msg: types::ChatMessage = msg.into();

        assert_eq!(chat_msg.role, types::Role::User);
        assert_eq!(chat_msg.content, Some("Hello".to_string()));
    }

    #[test]
    fn test_stop_sequences_single() {
        let options = LanguageModelOptions {
            stop_sequences: Some(vec!["STOP".to_string()]),
            ..Default::default()
        };

        let completions_opts: client::ChatCompletionsOptions = options.into();
        assert!(matches!(
            completions_opts.stop,
            Some(types::StopSequences::Single(_))
        ));
    }

    #[test]
    fn test_stop_sequences_multiple_truncated() {
        let options = LanguageModelOptions {
            stop_sequences: Some(vec![
                "S1".to_string(),
                "S2".to_string(),
                "S3".to_string(),
                "S4".to_string(),
                "S5".to_string(),
            ]),
            ..Default::default()
        };

        let completions_opts: client::ChatCompletionsOptions = options.into();
        if let Some(types::StopSequences::Multiple(seqs)) = completions_opts.stop {
            assert_eq!(seqs.len(), 4);
        }
    }

    #[test]
    fn test_usage_conversion() {
        let usage = types::Usage {
            prompt_tokens: 100,
            completion_tokens: 50,
            total_tokens: 150,
            prompt_tokens_details: Some(types::PromptTokensDetails {
                cached_tokens: 20,
                audio_tokens: None,
            }),
            completion_tokens_details: Some(types::CompletionTokensDetails {
                reasoning_tokens: 10,
                audio_tokens: None,
                accepted_prediction_tokens: None,
                rejected_prediction_tokens: None,
            }),
        };

        let sdk_usage: Usage = usage.into();
        assert_eq!(sdk_usage.input_tokens, Some(100));
        assert_eq!(sdk_usage.output_tokens, Some(50));
        assert_eq!(sdk_usage.cached_tokens, Some(20));
        assert_eq!(sdk_usage.reasoning_tokens, Some(10));
    }
}
