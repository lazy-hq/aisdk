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
                    // Tool approval requests are handled internally by the SDK
                    LanguageModelResponseContentType::ToolApprovalRequest(_) => {}
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
                                format!("<developer>\n{}\n</developer>", dev),
                            ),
                    });
                }
                // Tool approval messages are handled internally by the SDK
                Message::ToolApproval(_) => {}
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
