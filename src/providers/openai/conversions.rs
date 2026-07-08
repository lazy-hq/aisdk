//! Helper functions and conversions for the OpenAI provider.

use crate::core::embedding_model::EmbeddingModelOptions;
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

impl From<LanguageModelOptions> for client::OpenAILanguageModelOptions {
    fn from(options: LanguageModelOptions) -> Self {
        let extra_body = options.body.clone();
        let extra_headers = options.headers.clone();
        let items: Vec<types::InputItem> = options
            .messages
            .into_iter()
            .filter_map(|m| m.message.into())
            .collect();

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

        client::OpenAILanguageModelOptions {
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
            extra_body,
            extra_headers,
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
                        arguments: tool_info.input.to_string(),
                        call_id: tool_info.tool.id.clone(),
                        name: tool_info.tool.name.clone(),
                        type_: "function_call".to_string(),
                    }))
                }
                LanguageModelResponseContentType::Reasoning { ref content, .. } => {
                    Some(types::InputItem::Item(types::MessageItem::Reasoning {
                        id: None,
                        summary: vec![types::ReasoningSummary {
                            type_: "summary_text".to_string(),
                            text: content.clone(),
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

impl From<EmbeddingModelOptions> for types::OpenAIEmbeddingOptions {
    fn from(value: EmbeddingModelOptions) -> Self {
        let extra_body = value.body.clone();
        let extra_headers = value.headers.clone();
        types::OpenAIEmbeddingOptions {
            input: value.input,
            model: "".to_string(), // will be set in mod.rs
            user: None,
            dimensions: value.dimensions,
            encoding_format: None,
            extra_body,
            extra_headers,
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
    use crate::core::Message;
    use crate::core::language_model::{
        LanguageModelOptions, ReasoningEffort as LMReasoningEffort, Usage,
    };
    use crate::core::tools::{Tool, ToolExecute, ToolList};
    use schemars::{JsonSchema, schema_for};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct StructuredOutput {
        answer: String,
    }

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct SumInput {
        a: i32,
        b: i32,
    }

    #[test]
    fn test_scalar_request_options_map_to_openai_body() {
        let options = LanguageModelOptions {
            messages: vec![
                Message::System("You are helpful".to_string().into()).into(),
                Message::User("Hello".to_string().into()).into(),
            ],
            temperature: Some(70),
            top_p: Some(90),
            max_output_tokens: Some(256),
            ..Default::default()
        };

        let req: OpenAILanguageModelOptions = options.into();

        assert!(
            req.temperature
                .is_some_and(|value| (value - 0.7).abs() < f32::EPSILON)
        );
        assert!(
            req.top_p
                .is_some_and(|value| (value - 0.9).abs() < f32::EPSILON)
        );
        assert_eq!(req.max_output_tokens, Some(256));
        assert_eq!(req.stream, Some(false));

        let input = req.input.expect("input should be present");
        let Input::InputItemList(items) = input else {
            panic!("expected input item list")
        };
        assert_eq!(items.len(), 2);

        match &items[0] {
            InputItem::Item(MessageItem::InputMessage { role, content, .. }) => {
                assert_eq!(role, &Role::System);
                assert!(matches!(
                    content.first(),
                    Some(ContentType::InputText { text }) if text == "You are helpful"
                ));
            }
            _ => panic!("expected first item to be a system input message"),
        }

        match &items[1] {
            InputItem::Item(MessageItem::InputMessage { role, content, .. }) => {
                assert_eq!(role, &Role::User);
                assert!(
                    matches!(content.first(), Some(ContentType::InputText { text }) if text == "Hello")
                );
            }
            _ => panic!("expected second item to be a user input message"),
        }
    }

    #[test]
    fn test_schema_maps_to_openai_body() {
        let options = LanguageModelOptions {
            schema: Some(schema_for!(StructuredOutput)),
            ..Default::default()
        };

        let req: OpenAILanguageModelOptions = options.into();
        let text = req.text.expect("text config should be present");
        let format = text.format.expect("format should be present");

        let TextResponseFormat::JsonSchema {
            name,
            schema,
            strict,
            ..
        } = format
        else {
            panic!("expected json_schema response format")
        };

        assert_eq!(name, "StructuredOutput");
        assert_eq!(strict, Some(false));
        assert_eq!(schema["type"], json!("object"));
        assert_eq!(schema["properties"]["answer"]["type"], json!("string"));
    }

    #[test]
    fn test_tools_map_to_openai_body() {
        let tool = Tool::builder()
            .name("sum")
            .description("Adds two numbers")
            .input_schema(schema_for!(SumInput))
            .execute(ToolExecute::from_sync(|_, _| Ok("3".to_string())))
            .build()
            .expect("tool should build");

        let options = LanguageModelOptions {
            tools: Some(ToolList::new(vec![tool])),
            ..Default::default()
        };

        let req: OpenAILanguageModelOptions = options.into();
        let tools = req.tools.expect("tools should be present");
        assert_eq!(tools.len(), 1);

        match &tools[0] {
            ToolParams::Function {
                name,
                description,
                strict,
                parameters,
            } => {
                assert_eq!(name, "sum");
                assert_eq!(description.as_deref(), Some("Adds two numbers"));
                assert!(*strict);
                assert_eq!(parameters["type"], json!("object"));
                assert_eq!(parameters["additionalProperties"], json!(false));
                assert!(parameters["properties"].get("a").is_some());
                assert!(parameters["properties"].get("b").is_some());
            }
        }
    }

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
        let lm_options: OpenAILanguageModelOptions = options.into();
        assert!(lm_options.reasoning.is_some());
        let reasoning = lm_options.reasoning.unwrap();
        assert_eq!(reasoning.effort, Some(ReasoningEffort::Minimal));
        assert_eq!(reasoning.summary, Some(SummaryType::Auto));
    }

    #[test]
    fn test_language_model_options_to_create_response_with_reasoning_effort_medium() {
        let options = LanguageModelOptions {
            reasoning_effort: Some(LMReasoningEffort::Medium),
            ..Default::default()
        };
        let lm_options: OpenAILanguageModelOptions = options.into();
        assert!(lm_options.reasoning.is_some());
        let reasoning = lm_options.reasoning.unwrap();
        assert_eq!(reasoning.effort, Some(ReasoningEffort::Medium));
        assert_eq!(reasoning.summary, Some(SummaryType::Auto));
    }

    #[test]
    fn test_language_model_options_to_create_response_with_reasoning_effort_high() {
        let options = LanguageModelOptions {
            reasoning_effort: Some(LMReasoningEffort::High),
            ..Default::default()
        };
        let lm_options: OpenAILanguageModelOptions = options.into();
        assert!(lm_options.reasoning.is_some());
        let reasoning = lm_options.reasoning.unwrap();
        assert_eq!(reasoning.effort, Some(ReasoningEffort::High));
        assert_eq!(reasoning.summary, Some(SummaryType::Auto));
    }

    #[test]
    fn test_language_model_options_to_create_response_without_reasoning_effort() {
        let options = LanguageModelOptions {
            reasoning_effort: None,
            ..Default::default()
        };
        let lm_options: OpenAILanguageModelOptions = options.into();
        assert!(lm_options.reasoning.is_none());
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
