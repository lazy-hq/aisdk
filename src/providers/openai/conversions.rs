//! Helper functions and conversions for the OpenAI provider.

use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponseContentType, ReasoningEffort, Usage,
};
use crate::core::messages::Message;
use crate::core::tools::Tool;
use crate::providers::openai::client::{self, types};
use schemars::Schema;
use serde_json::Value;

impl From<Tool> for types::ToolParams {
    fn from(value: Tool) -> Self {
        let mut params = value.input_schema.to_value();

        // open ai requires 'additionalProperties' to be false
        params["additionalProperties"] = Value::Bool(false);

        // open ai requires 'properties' to be an object
        let properties = params.get("properties");
        if let Some(Value::Object(_)) = properties {
        } else {
            params["properties"] = Value::Object(serde_json::Map::new());
        }

        types::ToolParams::Function {
            name: value.name,
            description: Some(value.description),
            strict: true,
            parameters: params,
        }
    }
}

impl From<LanguageModelOptions> for client::OpenAIOptions {
    fn from(options: LanguageModelOptions) -> Self {
        let items: Vec<types::InputItem> = options
            .messages
            .into_iter()
            .filter_map(|m| m.message.into())
            .collect();

        // system prompt first since openai likes it at the top
        // if let Some(system) = options.system {
        //     items.insert(
        //         0,
        //         client::OpenAiInputItem::Message(client::OpenAIMessageParam::System {
        //             content: system,
        //         }),
        //     );
        // }

        let tools: Option<Vec<types::ToolParams>> = options.tools.map(|t| {
            t.tools
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .iter()
                .map(|t| types::ToolParams::from(t.clone()))
                .collect()
        });

        let reasoning = options
            .reasoning_effort
            .map(|reasoning| types::ReasoningConfig {
                summary: Some(types::SummaryType::Auto),
                effort: Some(reasoning.into()),
            });

        client::OpenAIOptions {
            model: "".to_string(), // will be set in mod.rs
            input: Some(types::Input::InputItemList(items)),
            text: Some(types::TextConfig {
                verbosity: None,
                format: Some(
                    options
                        .schema
                        .map(from_schema_to_response_format)
                        .unwrap_or(types::TextResponseFormat::Text),
                ),
            }),
            reasoning,
            temperature: options.temperature.map(|t| t as f32 / 100.0),
            max_output_tokens: options.max_output_tokens.map(|t| t as usize),
            stream: Some(false),
            top_p: options.top_p.map(|t| t as f32 / 100.0),
            tools,
        }
    }
}

impl From<Message> for Option<types::InputItem> {
    fn from(m: Message) -> Self {
        match m {
            Message::Tool(ref tool_info) => Some(types::InputItem::Item(
                types::MessageItem::FunctionCallOutput {
                    id: None,
                    type_: "function_call_output".to_string(),
                    status: None,
                    call_id: tool_info.tool.id.clone(),
                    output: types::FunctionCallOutput::Text(
                        tool_info
                            .output
                            .clone()
                            .unwrap_or_else(|e| Value::String(e.to_string()))
                            .to_string(),
                    ),
                },
            )),
            Message::Assistant(ref assistant_msg) => match assistant_msg.content {
                LanguageModelResponseContentType::Text(ref msg) => {
                    Some(types::InputItem::Item(types::MessageItem::OutputMessage {
                        id: None,
                        type_: "message".to_string(),
                        status: None,
                        role: types::Role::Assistant,
                        content: vec![types::OutputContent::OutputText {
                            annotations: vec![],
                            logprobs: vec![],
                            text: msg.to_owned(),
                        }],
                    }))
                }
                LanguageModelResponseContentType::ToolCall(ref tool_info) => {
                    Some(types::InputItem::Item(types::MessageItem::FunctionCall {
                        id: None,
                        status: None,
                        arguments: tool_info.input.clone(),
                        call_id: tool_info.tool.id.clone(),
                        name: tool_info.tool.name.clone(),
                        type_: "function_call".to_string(),
                    }))
                }
                LanguageModelResponseContentType::Reasoning(ref reason) => {
                    Some(types::InputItem::Item(types::MessageItem::Reasoning {
                        id: None,
                        summary: vec![types::ReasoningSummary {
                            type_: "summary_text".to_string(),
                            text: reason.clone(),
                        }],
                        type_: "reasoning".to_string(),
                        content: None,
                        encrypted_content: None,
                        status: None,
                    }))
                }
                _ => None,
            },
            Message::User(u) => Some(types::InputItem::Item(types::MessageItem::InputMessage {
                content: vec![types::ContentType::InputText { text: u.content }],
                role: types::Role::User,
                type_: "message".to_string(),
            })),
            Message::System(s) => Some(types::InputItem::Item(types::MessageItem::InputMessage {
                content: vec![types::ContentType::InputText { text: s.content }],
                role: types::Role::System,
                type_: "message".to_string(),
            })),
            Message::Developer(d) => {
                Some(client::InputItem::Item(types::MessageItem::InputMessage {
                    content: vec![types::ContentType::InputText { text: d }],
                    role: types::Role::Developer,
                    type_: "message".to_string(),
                }))
            }
        }
    }
}

impl From<types::ResponseUsage> for Usage {
    fn from(value: types::ResponseUsage) -> Self {
        Self {
            input_tokens: Some(value.input_tokens as usize),
            output_tokens: Some(value.output_tokens as usize),
            cached_tokens: Some(value.input_tokens_details.cached_tokens as usize),
            reasoning_tokens: Some(value.output_tokens_details.reasoning_tokens as usize),
        }
    }
}

impl From<ReasoningEffort> for types::ReasoningEffort {
    fn from(value: ReasoningEffort) -> Self {
        match value {
            ReasoningEffort::Low => client::ReasoningEffort::Minimal,
            ReasoningEffort::Medium => client::ReasoningEffort::Medium,
            ReasoningEffort::High => client::ReasoningEffort::High,
        }
    }
}

fn from_schema_to_response_format(schema: Schema) -> types::TextResponseFormat {
    let json = serde_json::to_value(schema).expect("Failed to serialize schema");
    types::TextResponseFormat::JsonSchema {
        name: json
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Response Schema")
            .to_owned(),
        description: json
            .get("description")
            .and_then(|v| v.as_str())
            .map(str::to_owned),
        schema: json,
        strict: Some(false),
    }
}

#[cfg(test)]
mod tests {
    use super::client::*;
    use crate::core::language_model::{
        LanguageModelOptions, ReasoningEffort as LMReasoningEffort, Usage,
    };

    #[test]
    fn test_reasoning_effort_conversion_low() {
        let effort = LMReasoningEffort::Low;
        let openai_effort: ReasoningEffort = effort.into();
        assert_eq!(openai_effort, ReasoningEffort::Minimal);
        let _ = openai_effort;
    }

    #[test]
    fn test_reasoning_effort_conversion_medium() {
        let effort = LMReasoningEffort::Medium;
        let openai_effort: ReasoningEffort = effort.into();
        assert_eq!(openai_effort, ReasoningEffort::Medium);
    }

    #[test]
    fn test_reasoning_effort_conversion_high() {
        let effort = ReasoningEffort::High;
        let openai_effort: ReasoningEffort = effort;
        assert_eq!(openai_effort, ReasoningEffort::High);
    }

    #[test]
    fn test_language_model_options_to_create_response_with_reasoning_effort_low() {
        let options = LanguageModelOptions {
            reasoning_effort: Some(LMReasoningEffort::Low),
            ..Default::default()
        };
        let create_response: OpenAIOptions = options.into();
        assert!(create_response.reasoning.is_some());
        let reasoning = create_response.reasoning.unwrap();
        assert_eq!(reasoning.effort, Some(ReasoningEffort::Minimal));
        assert_eq!(reasoning.summary, Some(SummaryType::Auto));
    }

    #[test]
    fn test_language_model_options_to_create_response_with_reasoning_effort_medium() {
        let options = LanguageModelOptions {
            reasoning_effort: Some(LMReasoningEffort::Medium),
            ..Default::default()
        };
        let create_response: OpenAIOptions = options.into();
        assert!(create_response.reasoning.is_some());
        let reasoning = create_response.reasoning.unwrap();
        assert_eq!(reasoning.effort, Some(ReasoningEffort::Medium));
        assert_eq!(reasoning.summary, Some(SummaryType::Auto));
    }

    #[test]
    fn test_language_model_options_to_create_response_with_reasoning_effort_high() {
        let options = LanguageModelOptions {
            reasoning_effort: Some(LMReasoningEffort::High),
            ..Default::default()
        };
        let create_response: OpenAIOptions = options.into();
        assert!(create_response.reasoning.is_some());
        let reasoning = create_response.reasoning.unwrap();
        assert_eq!(reasoning.effort, Some(ReasoningEffort::High));
        assert_eq!(reasoning.summary, Some(SummaryType::Auto));
    }

    #[test]
    fn test_language_model_options_to_create_response_without_reasoning_effort() {
        let options = LanguageModelOptions {
            reasoning_effort: None,
            ..Default::default()
        };
        let create_response: OpenAIOptions = options.into();
        assert!(create_response.reasoning.is_none());
    }

    #[test]
    fn test_openai_usage_to_usage_conversion() {
        let openai_usage = types::ResponseUsage {
            input_tokens: 100,
            output_tokens: 50,
            total_tokens: 150,
            input_tokens_details: Default::default(),
            output_tokens_details: Default::default(),
        };

        let usage: Usage = openai_usage.into();
        assert_eq!(usage.input_tokens, Some(100));
        assert_eq!(usage.output_tokens, Some(50));
        // These will be 0 because the details are default (None)
        assert_eq!(usage.cached_tokens, Some(0));
        assert_eq!(usage.reasoning_tokens, Some(0));
    }
}
