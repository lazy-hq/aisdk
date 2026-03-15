use crate::core::Message;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponseContentType, ReasoningEffort, Usage,
};
use crate::providers::anthropic::client::{
    AnthropicAssistantMessageParamContent, AnthropicMessageDeltaUsage, AnthropicMessageParam,
    AnthropicOptions, AnthropicThinking, AnthropicTool, AnthropicUsage,
};
use crate::providers::anthropic::extensions;

impl From<LanguageModelOptions> for AnthropicOptions {
    fn from(options: LanguageModelOptions) -> Self {
        let mut messages = Vec::new();
        let mut request = AnthropicOptions::builder();
        request.model("");

        // TODO: anthropic max_tokens is required. handle compile
        // time checks if not set in core
        let max_tokens = options.max_output_tokens.unwrap_or(10_000);

        // TODO: temperature, top_p, top_k, stop_sequences, and max_tokens are not mapped for Anthropic yet.
        // Add support once provider behavior is confirmed and covered.

        if let Some(system) = options.system
            && !system.is_empty()
        {
            request.system(Some(system));
        } else {
            request.system(None);
        }

        // convert messages to anthropic messages
        for msg in options.messages {
            match msg.message {
                Message::System(s) => {
                    if !s.content.is_empty() {
                        request.system(Some(s.content));
                    }
                }
                Message::User(u) => {
                    messages.push(AnthropicMessageParam::User {
                        content:
                            crate::providers::anthropic::client::AnthropicUserMessageContent::Text(
                                u.content,
                            ),
                    });
                }
                Message::Assistant(a) => match a.content {
                    LanguageModelResponseContentType::Text(text) => {
                        messages.push(AnthropicMessageParam::Assistant {
                            content: vec![AnthropicAssistantMessageParamContent::Text { text }],
                        });
                    }
                    LanguageModelResponseContentType::ToolCall(tool) => {
                        messages.push(AnthropicMessageParam::Assistant {
                            content: vec![AnthropicAssistantMessageParamContent::ToolUse {
                                id: tool.tool.id,
                                input: tool.input,
                                name: tool.tool.name,
                            }],
                        });
                    }
                    LanguageModelResponseContentType::Reasoning {
                        content,
                        extensions,
                    } => {
                        // Retrieve Anthropic-specific signature from extensions
                        let signature = extensions
                            .get::<extensions::AnthropicThinkingMetadata>()
                            .signature
                            .clone()
                            .unwrap_or_else(|| content.clone());

                        messages.push(AnthropicMessageParam::Assistant {
                            content: vec![AnthropicAssistantMessageParamContent::Thinking {
                                thinking: content.clone(),
                                signature,
                            }],
                        });
                    }
                    LanguageModelResponseContentType::NotSupported(_) => {}
                },
                Message::Tool(tool) => {
                    messages.push(AnthropicMessageParam::User {
                        content: crate::providers::anthropic::client::AnthropicUserMessageContent::Blocks(vec![
                            crate::providers::anthropic::client::AnthropicUserMessageContentBlock::ToolResult {
                                tool_use_id: tool.tool.id,
                                content: tool.output.unwrap_or_default().to_string(),
                            },
                        ]),
                    });
                }
                Message::Developer(dev) => {
                    messages.push(AnthropicMessageParam::User {
                        content:
                            crate::providers::anthropic::client::AnthropicUserMessageContent::Text(
                                format!("<developer>\n{dev}\n</developer>"),
                            ),
                    });
                }
            }
        }
        // update messages
        request.messages(messages);

        // convert tools to anthropic tools
        if let Some(tools) = options.tools {
            request.tools(Some(
                tools
                    .tools
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .iter()
                    .map(|t| {
                        let tool = t.clone();
                        let mut tool_schema = tool.input_schema.to_value();
                        if let Some(schema) = tool_schema.as_object_mut() {
                            schema.remove("$schema");
                        };
                        AnthropicTool {
                            name: tool.name,
                            description: tool.description,
                            input_schema: tool_schema,
                        }
                    })
                    .collect(),
            ));
        }

        // convert reasoning to antropic thinking
        request.thinking(options.reasoning_effort.map(|effort| match effort {
            // Low is 25% of the max_tokens
            ReasoningEffort::Low => AnthropicThinking::Enable {
                budget_tokens: (max_tokens / 4) as usize,
            },
            // Medium is 50% of the max_tokens
            ReasoningEffort::Medium => AnthropicThinking::Enable {
                budget_tokens: (max_tokens / 2) as usize,
            },
            // High is 75% of the max_tokens
            ReasoningEffort::High => AnthropicThinking::Enable {
                budget_tokens: (max_tokens - (max_tokens / 4)) as usize,
            },
        }));

        request.build().expect("Failed to build AntropicRequest")
    }
}

impl From<AnthropicUsage> for Usage {
    fn from(usage: AnthropicUsage) -> Self {
        Self {
            input_tokens: Some(usage.input_tokens),
            output_tokens: Some(usage.output_tokens),
            cached_tokens: Some(usage.cache_creation_input_tokens + usage.cache_read_input_tokens),
            reasoning_tokens: None,
        }
    }
}

impl From<AnthropicMessageDeltaUsage> for Usage {
    fn from(usage: AnthropicMessageDeltaUsage) -> Self {
        Self {
            input_tokens: Some(usage.input_tokens.unwrap_or(0)),
            output_tokens: Some(usage.output_tokens),
            cached_tokens: Some(
                usage.cache_creation_input_tokens.unwrap_or(0)
                    + usage.cache_read_input_tokens.unwrap_or(0),
            ),
            reasoning_tokens: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Message;
    use crate::core::language_model::{LanguageModelOptions, ReasoningEffort};
    use crate::core::tools::{Tool, ToolExecute, ToolList};
    use schemars::{JsonSchema, schema_for};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct SumInput {
        a: i32,
        b: i32,
    }

    #[test]
    fn test_scalar_request_options_map_to_anthropic_body() {
        let options = LanguageModelOptions {
            system: Some("You are helpful".to_string()),
            messages: vec![Message::User("Hello".to_string().into()).into()],
            ..Default::default()
        };

        let req: AnthropicOptions = options.into();

        assert_eq!(req.system.as_deref(), Some("You are helpful"));
        assert_eq!(req.messages.len(), 1);

        match &req.messages[0] {
            AnthropicMessageParam::User { content } => match content {
                crate::providers::anthropic::client::AnthropicUserMessageContent::Text(text) => {
                    assert_eq!(text, "Hello")
                }
                _ => panic!("expected user text message"),
            },
            _ => panic!("expected user message"),
        }
    }

    #[test]
    fn test_reasoning_maps_to_thinking_budget() {
        let options = LanguageModelOptions {
            max_output_tokens: Some(200),
            reasoning_effort: Some(ReasoningEffort::High),
            ..Default::default()
        };

        let req: AnthropicOptions = options.into();

        match req.thinking {
            Some(AnthropicThinking::Enable { budget_tokens }) => {
                assert_eq!(budget_tokens, 150);
            }
            _ => panic!("expected anthropic thinking to be enabled"),
        }
    }

    #[test]
    fn test_tools_map_to_anthropic_body() {
        let tool = Tool::builder()
            .name("sum")
            .description("Adds two numbers")
            .input_schema(schema_for!(SumInput))
            .execute(ToolExecute::new(Box::new(|_| Ok("3".to_string()))))
            .build()
            .expect("tool should build");

        let options = LanguageModelOptions {
            tools: Some(ToolList::new(vec![tool])),
            ..Default::default()
        };

        let req: AnthropicOptions = options.into();
        let tools = req.tools.expect("tools should be present");
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "sum");
        assert_eq!(tools[0].description, "Adds two numbers");
        assert_eq!(tools[0].input_schema["type"], json!("object"));
        assert!(tools[0].input_schema["properties"].get("a").is_some());
        assert!(tools[0].input_schema["properties"].get("b").is_some());
        assert!(tools[0].input_schema.get("$schema").is_none());
    }

    #[test]
    fn test_anthropic_usage_to_usage_conversion() {
        let usage = AnthropicUsage {
            cache_creation: crate::providers::anthropic::client::AnthropicCacheCreation {
                ephemeral_1h_input_tokens: 0,
                ephemeral_5m_input_tokens: 0,
            },
            cache_creation_input_tokens: 4,
            cache_read_input_tokens: 6,
            input_tokens: 100,
            output_tokens: 50,
            server_tool_use: crate::providers::anthropic::client::AnthropicServerToolUsage::default(
            ),
            service_tier: "standard".to_string(),
        };

        let sdk_usage: Usage = usage.into();
        assert_eq!(sdk_usage.input_tokens, Some(100));
        assert_eq!(sdk_usage.output_tokens, Some(50));
        assert_eq!(sdk_usage.cached_tokens, Some(10));
        assert_eq!(sdk_usage.reasoning_tokens, None);
    }

    #[test]
    fn test_anthropic_delta_usage_to_usage_conversion() {
        let usage = AnthropicMessageDeltaUsage {
            cache_creation_input_tokens: Some(3),
            cache_read_input_tokens: Some(2),
            input_tokens: Some(10),
            output_tokens: 7,
            server_tool_use: None,
        };

        let sdk_usage: Usage = usage.into();
        assert_eq!(sdk_usage.input_tokens, Some(10));
        assert_eq!(sdk_usage.output_tokens, Some(7));
        assert_eq!(sdk_usage.cached_tokens, Some(5));
        assert_eq!(sdk_usage.reasoning_tokens, None);
    }
}
