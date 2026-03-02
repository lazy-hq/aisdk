//! Language model implementation for the Anthropic provider.

use crate::core::capabilities::ModelName;
use crate::core::client::LanguageModelClient;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, ProviderStream,
};
use crate::core::messages::AssistantMessage;
use crate::core::tools::ToolDetails;
use crate::core::{LanguageModelStreamChunkType, ToolCallInfo};
use crate::extensions::Extensions;
use crate::providers::anthropic::Anthropic;
use crate::providers::anthropic::client::{
    AnthropicContentBlock, AnthropicDelta, AnthropicMessageDeltaUsage, AnthropicOptions,
    AnthropicStreamEvent,
};
use crate::providers::anthropic::extensions;
use crate::{core::language_model::LanguageModel, error::Result};
use async_trait::async_trait;
use futures::StreamExt;
use std::collections::HashMap;

#[async_trait]
impl<M: ModelName> LanguageModel for Anthropic<M> {
    /// Returns the name of the model.
    fn name(&self) -> String {
        self.options.model.clone()
    }

    /// Generates text using the Anthropic provider.
    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let additional_headers = options.headers.clone();
        let mut options: AnthropicOptions = options.into();
        options.model = self.options.model.clone();
        self.options = options;

        let response = self
            .send(self.settings.base_url.clone(), additional_headers)
            .await?;

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
                    let extensions = Extensions::default();
                    extensions
                        .get_mut::<extensions::AnthropicThinkingMetadata>()
                        .signature = Some(signature);
                    collected.push(LanguageModelResponseContentType::Reasoning {
                        content: thinking,
                        extensions,
                    });
                }
                AnthropicContentBlock::RedactedThinking { data } => {
                    collected.push(LanguageModelResponseContentType::Reasoning {
                        content: data,
                        extensions: Extensions::default(),
                    });
                }
                AnthropicContentBlock::ToolUse { id, input, name } => {
                    collected.push(LanguageModelResponseContentType::ToolCall(ToolCallInfo {
                        input,
                        tool: ToolDetails {
                            id: id.to_string(),
                            name: name.to_string(),
                        },
                        extensions: Extensions::default(),
                    }));
                }
            }
        }

        Ok(LanguageModelResponse {
            contents: collected,
            usage: Some(response.usage.into()),
        })
    }

    /// Streams text using the Anthropic provider.
    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        let additional_headers = options.headers.clone();
        let mut options: AnthropicOptions = options.into();
        options.stream = Some(true);
        options.model = self.options.model.clone();
        self.options = options;

        // Retry logic for rate limiting
        let max_retries = 5;
        let mut retry_count = 0;
        let mut wait_time = std::time::Duration::from_secs(1);

        let response = loop {
            match self
                .send_and_stream(self.settings.base_url.clone(), additional_headers.clone())
                .await
            {
                Ok(stream) => break stream,
                Err(crate::error::Error::ApiError {
                    status_code: Some(status),
                    ..
                }) if status == reqwest::StatusCode::TOO_MANY_REQUESTS
                    && retry_count < max_retries =>
                {
                    retry_count += 1;
                    tokio::time::sleep(wait_time).await;
                    wait_time *= 2; // Exponential backoff
                    continue;
                }
                Err(e) => return Err(e),
            }
        };

        #[derive(Default)]
        struct StreamState {
            content_blocks: HashMap<usize, AccumulatedBlock>,
            usage: Option<AnthropicMessageDeltaUsage>,
        }

        #[derive(Debug)]
        enum AccumulatedBlock {
            Text(String),
            Thinking {
                thinking: String,
                signature: Option<String>,
            },
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
                let unsupported =  |event: &str| {
                    vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::NotSupported(format!("AnthropicStreamEvent::{event}")),
                    )]
                };
                futures::future::ready({
                    match evt_res {
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
                                Some(Ok(unsupported("ContentBlockStart::Text")))
                            }
                            AnthropicContentBlock::Thinking { .. } => {
                                state
                                    .content_blocks
                                    .insert(index, AccumulatedBlock::Thinking {
                                        thinking: String::new(),
                                        signature: None,
                                    });
                                Some(Ok(unsupported("ContentBlockStart::Thinking")))
                            }
                            AnthropicContentBlock::RedactedThinking { data } => {
                                state.content_blocks.insert(
                                    index,
                                    AccumulatedBlock::RedactedThinking(data.clone()),
                                );
                                Some(Ok(unsupported("ContentBlockStart::RedactedThinking")))
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
                                Some(Ok(unsupported("ContentBlockStart::ToolUse")))
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
                                    (
                                        AccumulatedBlock::Thinking { thinking, .. },
                                        AnthropicDelta::ThinkingDelta { thinking: delta_thinking },
                                    ) => {
                                        thinking.push_str(&delta_thinking);
                                        Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                            LanguageModelStreamChunkType::Text(delta_thinking),
                                        )]))
                                    }
                                    (
                                        AccumulatedBlock::Thinking { signature, .. },
                                        AnthropicDelta::SignatureDelta { signature: delta_signature },
                                    ) => {
                                        *signature = Some(delta_signature.clone());
                                        Some(Ok(unsupported("SignatureDelta")))
                                    }
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
                                    _ => Some(Ok(unsupported("ContentBlockDelta"))),
                                }
                            } else {
                                unreachable!("Anthropic accumulator must be initialized on AnthropicStreamEvent::ContentBlockStart")
                            }
                        }
                        AnthropicStreamEvent::ContentBlockStop { .. } => {
                            Some(Ok(unsupported("ContentBlockStop")))
                        }
                        AnthropicStreamEvent::MessageDelta { usage, .. } => {
                            state.usage = Some(usage);
                            Some(Ok(unsupported("MessageDelta")))
                        }
                        AnthropicStreamEvent::MessageStop => {
                            let mut collected = vec![];
                            for block in state.content_blocks.values() {
                                match block {
                                    AccumulatedBlock::Text(text) => collected
                                        .push(LanguageModelResponseContentType::new(text.clone())),
                                    AccumulatedBlock::Thinking { thinking, signature } => {
                                        let extensions = Extensions::default();
                                        if let Some(sig) = signature {
                                            extensions
                                                .get_mut::<extensions::AnthropicThinkingMetadata>()
                                                .signature = Some(sig.clone());
                                        }
                                        collected.push(LanguageModelResponseContentType::Reasoning {
                                            content: thinking.clone(),
                                            extensions,
                                        })
                                    }
                                    AccumulatedBlock::RedactedThinking(data) => collected.push(
                                        LanguageModelResponseContentType::Reasoning {
                                            content: data.clone(),
                                            extensions: Extensions::default(),
                                        },
                                    ),
                                    AccumulatedBlock::ToolUse {
                                        id,
                                        name,
                                        accumulated_json,
                                    } => {
                                        let json_str = if accumulated_json.trim().is_empty() {
                                            "{}"
                                        } else {
                                            accumulated_json
                                        };
                                        if let Ok(input) = serde_json::from_str(json_str) {
                                            collected.push(
                                                LanguageModelResponseContentType::ToolCall(
                                                    ToolCallInfo {
                                                        input,
                                                        tool: ToolDetails {
                                                            id: id.clone(),
                                                            name: name.clone(),
                                                        },
                                                        extensions: Extensions::default(),
                                                    },
                                                ),
                                            );
                                        } else {
                                            collected.push(
                                                LanguageModelResponseContentType::NotSupported(
                                                    format!(
                                                        "Invalid tool json: {accumulated_json}"
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
                        AnthropicStreamEvent::NotSupported(txt) => {
                            Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                LanguageModelStreamChunkType::NotSupported(txt),
                            )]))
                        }
                    },
                    Err(e) => Some(Err(e)),
                }})
            },
        );

        Ok(Box::pin(stream))
    }
}
