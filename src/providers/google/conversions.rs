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
                output_dimensionality: None,
            })
            .collect();

        GoogleEmbeddingOptions {
            model: String::new(), // will be set in embedding_model.rs
            requests,
        }
    }
}
