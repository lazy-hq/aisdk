//! This module provides the Anthropic provider, which implements the `LanguageModel`
//! and `Provider` traits for interacting with the Anthropic API.

pub mod client;
pub mod conversions;
pub mod settings;

use crate::core::client::Client;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, ProviderStream,
};
use crate::core::messages::AssistantMessage;
use crate::core::tools::ToolDetails;
use crate::core::{LanguageModelStreamChunkType, ToolCallInfo};
use crate::providers::anthropic::client::{
    AnthropicContentBlock, AnthropicDelta, AnthropicMessageDeltaUsage, AnthropicOptions,
    AnthropicStreamEvent,
};
use crate::providers::anthropic::settings::{
    AnthropicProviderSettings, AnthropicProviderSettingsBuilder,
};
use crate::{
    core::{language_model::LanguageModel, provider::Provider},
    error::Result,
};
use async_trait::async_trait;
use futures::StreamExt;
use serde::Serialize;
use std::collections::HashMap;

pub const ANTHROPIC_API_VERSION: &str = "2023-06-01";

/// The Anthropic provider.
#[derive(Debug, Serialize)]
pub struct Anthropic {
    pub settings: AnthropicProviderSettings,
    options: AnthropicOptions,
}

impl Anthropic {
    /// Creates a new `Anthropic` provider with the given settings.
    pub fn new(model_name: impl Into<String>) -> Self {
        AnthropicProviderSettingsBuilder::default()
            .model_name(model_name.into())
            .build()
            .expect("Failed to build AnthropicProviderSettings")
    }

    /// Anthropic provider setting builder.
    pub fn builder() -> AnthropicProviderSettingsBuilder {
        AnthropicProviderSettings::builder()
    }
}

impl Provider for Anthropic {}

#[async_trait]
impl LanguageModel for Anthropic {
    fn name(&self) -> String {
        self.options.model.clone()
    }

    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let mut options: AnthropicOptions = options.into();
        options.model = self.options.model.clone();
        self.options = options;

        let response = self.send(self.settings.base_url.clone()).await?;

        let mut collected: Vec<LanguageModelResponseContentType> = Vec::new();

        for out in response.content {
            match out {
                AnthropicContentBlock::Text { text, .. } => {
                    collected.push(LanguageModelResponseContentType::new(text));
                }
                AnthropicContentBlock::Thinking {
                    signature,
                    thinking,
                } => {
                    collected.push(LanguageModelResponseContentType::Reasoning(signature));
                    collected.push(LanguageModelResponseContentType::Reasoning(thinking));
                }
                AnthropicContentBlock::RedactedThinking { data } => {
                    collected.push(LanguageModelResponseContentType::Reasoning(data));
                }
                AnthropicContentBlock::ToolUse { id, input, name } => {
                    collected.push(LanguageModelResponseContentType::ToolCall(ToolCallInfo {
                        input,
                        tool: ToolDetails {
                            id: id.to_string(),
                            name: name.to_string(),
                        },
                    }));
                }
            }
        }

        Ok(LanguageModelResponse {
            contents: collected,
            usage: Some(response.usage.into()),
        })
    }

    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        let mut options: AnthropicOptions = options.into();
        options.stream = Some(true);
        options.model = self.options.model.clone();
        self.options = options;

        let response = self.send_and_stream(self.settings.base_url.clone()).await?;

        #[derive(Default)]
        struct StreamState {
            content_blocks: HashMap<usize, AccumulatedBlock>,
            usage: Option<AnthropicMessageDeltaUsage>,
        }

        #[derive(Debug)]
        enum AccumulatedBlock {
            Text(String),
            Thinking(String),
            RedactedThinking(String),
            ToolUse {
                id: String,
                name: String,
                accumulated_json: String,
            },
        }

        let stream = response.scan::<_, Result<Vec<LanguageModelStreamChunk>>, _, _>(
            StreamState::default(),
            |state, evt_res| {
                futures::future::ready(match evt_res {
                    Ok(event) => match event {
                        AnthropicStreamEvent::MessageStart { .. } => {
                            Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                LanguageModelStreamChunkType::Start,
                            )]))
                        }
                        AnthropicStreamEvent::ContentBlockStart {
                            index,
                            content_block,
                        } => match content_block {
                            AnthropicContentBlock::Text { .. } => {
                                state
                                    .content_blocks
                                    .insert(index, AccumulatedBlock::Text(String::new()));
                                None
                            }
                            AnthropicContentBlock::Thinking { .. } => {
                                state
                                    .content_blocks
                                    .insert(index, AccumulatedBlock::Thinking(String::new()));
                                None
                            }
                            AnthropicContentBlock::RedactedThinking { data } => {
                                state.content_blocks.insert(
                                    index,
                                    AccumulatedBlock::RedactedThinking(data.clone()),
                                );
                                Some(Ok(vec![LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: LanguageModelResponseContentType::Reasoning(data),
                                    usage: None,
                                })]))
                            }
                            AnthropicContentBlock::ToolUse { id, name, .. } => {
                                state.content_blocks.insert(
                                    index,
                                    AccumulatedBlock::ToolUse {
                                        id,
                                        name,
                                        accumulated_json: String::new(),
                                    },
                                );
                                None
                            }
                        },
                        AnthropicStreamEvent::ContentBlockDelta { index, delta } => {
                            if let Some(block) = state.content_blocks.get_mut(&index) {
                                match (block, delta) {
                                    (
                                        AccumulatedBlock::Text(text),
                                        AnthropicDelta::TextDelta { text: delta_text },
                                    ) => {
                                        text.push_str(&delta_text);
                                        Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                            LanguageModelStreamChunkType::Text(delta_text),
                                        )]))
                                    }

                                    // TODO: handle Reasoning delta event
                                    (
                                        AccumulatedBlock::ToolUse {
                                            accumulated_json, ..
                                        },
                                        AnthropicDelta::ToolUseDelta { partial_json },
                                    ) => {
                                        accumulated_json.push_str(&partial_json);
                                        Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                            LanguageModelStreamChunkType::ToolCall(partial_json),
                                        )]))
                                    }
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        }
                        AnthropicStreamEvent::MessageDelta { usage, .. } => {
                            state.usage = Some(usage);
                            None
                        }
                        AnthropicStreamEvent::MessageStop => {
                            let mut collected = vec![];
                            for block in state.content_blocks.values() {
                                match block {
                                    AccumulatedBlock::Text(text) => collected
                                        .push(LanguageModelResponseContentType::new(text.clone())),
                                    AccumulatedBlock::Thinking(thinking) => {
                                        collected.push(LanguageModelResponseContentType::Reasoning(
                                            thinking.clone(),
                                        ))
                                    }
                                    AccumulatedBlock::RedactedThinking(data) => collected.push(
                                        LanguageModelResponseContentType::Reasoning(data.clone()),
                                    ),
                                    AccumulatedBlock::ToolUse {
                                        id,
                                        name,
                                        accumulated_json,
                                    } => {
                                        if let Ok(input) = serde_json::from_str(accumulated_json) {
                                            collected.push(
                                                LanguageModelResponseContentType::ToolCall(
                                                    ToolCallInfo {
                                                        input,
                                                        tool: ToolDetails {
                                                            id: id.clone(),
                                                            name: name.clone(),
                                                        },
                                                    },
                                                ),
                                            );
                                        } else {
                                            collected.push(
                                                LanguageModelResponseContentType::NotSupported(
                                                    format!(
                                                        "Invalid tool json: {}",
                                                        accumulated_json
                                                    ),
                                                ),
                                            );
                                        }
                                    }
                                }
                            }
                            Some(Ok(collected
                                .into_iter()
                                .map(|ref c| {
                                    LanguageModelStreamChunk::Done(AssistantMessage {
                                        content: c.clone(),
                                        usage: state.usage.clone().map(|usage| usage.into()),
                                    })
                                })
                                .collect()))
                        }
                        AnthropicStreamEvent::Error { error } => {
                            let reason = format!("{}: {}", error.type_, error.message);

                            Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                LanguageModelStreamChunkType::Failed(reason),
                            )]))
                        }
                        _ => None,
                    },
                    Err(e) => Some(Err(e)),
                })
            },
        );

        Ok(Box::pin(stream))
    }
}
