//! Conversions between types used by the Google provider and the types used by the core library.
use crate::core::embedding_model::EmbeddingModelOptions;
use crate::core::language_model::{LanguageModelOptions, LanguageModelResponseContentType, Usage};
use crate::core::messages::{Message, TaggedMessage};
use crate::core::tools::Tool;
use crate::providers::google::client::GoogleEmbeddingOptions;
use crate::providers::google::client::types::{
    self, Content, FunctionDeclaration, GenerateContentRequest, Part, Role,
};
use crate::providers::google::extensions::GoogleToolMetadata;
use serde_json::Value;

impl From<Tool> for FunctionDeclaration {
    fn from(value: Tool) -> Self {
        let mut params = value.input_schema.to_value();
        if let Some(obj) = params.as_object_mut() {
            obj.remove("$schema");
        }

        Self {
            name: value.name,
            description: value.description,
            parameters: Some(params),
        }
    }
}

impl From<LanguageModelOptions> for GenerateContentRequest {
    fn from(options: LanguageModelOptions) -> Self {
        let contents = options.messages.into_iter().map(|m| m.into()).collect();

        let system_instruction = options.system.map(|s| Content {
            role: Role::User, // System instructions are often text-only content
            parts: vec![Part {
                text: Some(s),
                ..Default::default()
            }],
        });

        let tools = options.tools.map(|t| {
            let tools_list = t.tools.lock().unwrap_or_else(|p| p.into_inner());
            vec![types::Tool {
                function_declarations: Some(
                    tools_list.iter().map(|tool| tool.clone().into()).collect(),
                ),
                google_search_retrieval: None,
                code_execution: None,
            }]
        });

        let generation_config = Some(types::GenerationConfig {
            stop_sequences: options.stop_sequences,
            response_mime_type: options
                .schema
                .as_ref()
                .map(|_| "application/json".to_string()),
            response_schema: options.schema.map(|s| {
                let mut v = serde_json::to_value(s).unwrap();
                if let Some(obj) = v.as_object_mut() {
                    obj.remove("$schema");
                }
                v
            }),
            candidate_count: None,
            max_output_tokens: options.max_output_tokens.map(|t| t as i32),
            temperature: options.temperature.map(|t| t as f32 / 100.0),
            top_p: options.top_p.map(|t| t as f32 / 100.0),
            top_k: options.top_k.map(|t| t as i32),
            presence_penalty: options.presence_penalty,
            frequency_penalty: options.frequency_penalty,
            response_logprobs: None,
            logprobs: None,
        });

        Self {
            contents,
            tools,
            tool_config: None, // Default to auto
            safety_settings: None,
            system_instruction,
            generation_config,
            cached_content: None,
        }
    }
}

impl From<TaggedMessage> for Content {
    fn from(tagged: TaggedMessage) -> Self {
        tagged.message.into()
    }
}

impl From<Message> for Content {
    fn from(message: Message) -> Self {
        match message {
            Message::User(u) => Content {
                role: Role::User,
                parts: vec![Part {
                    text: Some(u.content),
                    ..Default::default()
                }],
            },
            Message::Assistant(a) => {
                let part = match a.content {
                    LanguageModelResponseContentType::Text(t) => Part {
                        text: Some(t),
                        ..Default::default()
                    },
                    LanguageModelResponseContentType::ToolCall(tc) => {
                        let mut part = Part {
                            function_call: Some(types::FunctionCall {
                                name: tc.tool.name.clone(),
                                args: tc.input.clone(),
                            }),
                            ..Default::default()
                        };
                        // Retrieve Gemini-specific ToolCall metadata from extensions
                        if let Some(sig) = tc
                            .extensions
                            .get::<GoogleToolMetadata>()
                            .thought_signature
                            .as_ref()
                        {
                            part.thought_signature = Some(sig.clone());
                        }
                        part
                    }
                    _ => Part::default(),
                };
                Content {
                    role: Role::Model,
                    parts: vec![part],
                }
            }
            Message::Tool(tr) => {
                let mut response = tr.output.unwrap_or(Value::Null);
                if !response.is_object() {
                    response = serde_json::json!({ "result": response });
                }
                Content {
                    role: Role::User,
                    parts: vec![Part {
                        function_response: Some(types::FunctionResponse {
                            name: tr.tool.name,
                            response,
                        }),
                        ..Default::default()
                    }],
                }
            }
            Message::System(s) => Content {
                role: Role::User,
                parts: vec![Part {
                    text: Some(s.content),
                    ..Default::default()
                }],
            },
            Message::Developer(d) => Content {
                role: Role::User,
                parts: vec![Part {
                    text: Some(d),
                    ..Default::default()
                }],
            },
        }
    }
}

impl From<types::UsageMetadata> for Usage {
    fn from(value: types::UsageMetadata) -> Self {
        Self {
            input_tokens: Some(value.prompt_token_count as usize),
            output_tokens: Some(value.candidates_token_count as usize),
            reasoning_tokens: None, // Gemini doesn't separate reasoning tokens in UsageMetadata v1beta
            cached_tokens: None,
        }
    }
}

impl From<EmbeddingModelOptions> for GoogleEmbeddingOptions {
    fn from(value: EmbeddingModelOptions) -> Self {
        let requests = value
            .input
            .into_iter()
            .map(|text| types::EmbedContentRequest {
                model: String::new(), // will be set in embedding_model.rs
                content: Content {
                    role: Role::User,
                    parts: vec![Part {
                        text: Some(text),
                        ..Default::default()
                    }],
                },
                task_type: None,
                title: None,
                output_dimensionality: value.dimensions,
            })
            .collect();

        GoogleEmbeddingOptions {
            model: String::new(), // will be set in embedding_model.rs
            requests,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Message;
    use crate::core::language_model::LanguageModelOptions;
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
    fn test_scalar_request_options_map_to_google_body() {
        let options = LanguageModelOptions {
            system: Some("You are helpful".to_string()),
            messages: vec![Message::User("Hello".to_string().into()).into()],
            temperature: Some(70),
            top_p: Some(90),
            stop_sequences: Some(vec!["END".to_string()]),
            max_output_tokens: Some(256),
            ..Default::default()
        };

        let req: GenerateContentRequest = options.into();

        let gen_config = req
            .generation_config
            .expect("should have generation config");
        assert!(
            gen_config
                .temperature
                .is_some_and(|value| (value - 0.7).abs() < f32::EPSILON)
        );
        assert!(
            gen_config
                .top_p
                .is_some_and(|value| (value - 0.9).abs() < f32::EPSILON)
        );
        assert_eq!(gen_config.max_output_tokens, Some(256));
        assert_eq!(gen_config.stop_sequences, Some(vec!["END".to_string()]));

        let sys = req
            .system_instruction
            .expect("should have system instruction");
        assert!(matches!(sys.role, Role::User)); // Default role is user in Content
        assert_eq!(sys.parts[0].text.as_deref(), Some("You are helpful"));

        assert_eq!(req.contents.len(), 1);
        assert!(matches!(req.contents[0].role, Role::User));
        assert_eq!(req.contents[0].parts[0].text.as_deref(), Some("Hello"));
    }

    #[test]
    fn test_schema_maps_to_google_body() {
        let options = LanguageModelOptions {
            schema: Some(schema_for!(StructuredOutput)),
            ..Default::default()
        };

        let req: GenerateContentRequest = options.into();

        let gen_config = req
            .generation_config
            .expect("should have generation config");
        assert_eq!(
            gen_config.response_mime_type.as_deref(),
            Some("application/json")
        );

        let schema = gen_config
            .response_schema
            .expect("should have response schema");
        assert_eq!(schema["type"], json!("object"));
        assert_eq!(schema["properties"]["answer"]["type"], json!("string"));
    }

    #[test]
    fn test_tools_map_to_google_body() {
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

        let req: GenerateContentRequest = options.into();

        let tools = req.tools.expect("tools should be present");
        assert_eq!(tools.len(), 1);

        let declarations = tools[0]
            .function_declarations
            .as_ref()
            .expect("should have function declarations");
        assert_eq!(declarations.len(), 1);
        assert_eq!(declarations[0].name, "sum");
        assert_eq!(declarations[0].description, "Adds two numbers");

        let parameters = declarations[0]
            .parameters
            .as_ref()
            .expect("should have parameters");
        assert_eq!(parameters["type"], json!("object"));
        assert!(parameters["properties"].get("a").is_some());
        assert!(parameters["properties"].get("b").is_some());
    }
}
