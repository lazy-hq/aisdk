//! This module provides the Anthropic provider, which implements the `LanguageModel`
//! and `Provider` traits for interacting with the Anthropic API.

pub mod client;
pub mod conversions;
pub mod settings;
pub mod utils;

use crate::core::ToolCallInfo;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType, ProviderStream,
};
use crate::core::tools::ToolDetails;
use crate::error::ProviderError;
use crate::providers::anthropic::client::AntropicContentBlock;
use crate::providers::anthropic::{
    client::{AnthropicClient, Request},
    settings::{AnthropicProviderSettings, AnthropicProviderSettingsBuilder},
};
use crate::{
    core::{language_model::LanguageModel, provider::Provider},
    error::Result,
};
use async_trait::async_trait;
use serde::Serialize;

/// The Anthropic provider.
#[derive(Debug, Serialize)]
pub struct Anthropic {
    #[serde(skip)]
    pub client: reqwest::Client,
    pub settings: AnthropicProviderSettings,
}

#[derive(Debug, Clone)]
pub struct AnthropicError;
impl std::fmt::Display for AnthropicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnthropicError")
    }
}
impl std::error::Error for AnthropicError {}

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

impl ProviderError for AnthropicError {}

#[async_trait]
impl LanguageModel for Anthropic {
    fn name(&self) -> String {
        self.settings.model_name.clone()
    }

    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let mut request: AnthropicClient = options.into();
        request.model = self.settings.model_name.clone();
        let response = request.send().await?;

        let mut collected: Vec<LanguageModelResponseContentType> = Vec::new();

        for out in response.content {
            match out {
                AntropicContentBlock::Text { text, .. } => {
                    collected.push(LanguageModelResponseContentType::new(text));
                }
                AntropicContentBlock::Thinking {
                    signature,
                    thinking,
                } => {
                    collected.push(LanguageModelResponseContentType::Reasoning(signature));
                    collected.push(LanguageModelResponseContentType::Reasoning(thinking));
                }
                AntropicContentBlock::RedactedThinking { data } => {
                    collected.push(LanguageModelResponseContentType::Reasoning(data));
                }
                AntropicContentBlock::ToolUse { id, input, name } => {
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
        let mut request: AnthropicClient = options.into();
        request.model = self.settings.model_name.clone();
        request.stream = Some(true);

        // let response = self
        //     .client
        //     .post("https://api.anthropic.com/v1/messages")
        //     .header("Content-Type", "application/json")
        //     .header("x-api-key", &self.settings.api_key)
        //     .header("anthropic-version", "2023-06-01")
        //     .json(&request)
        //     .send()
        //     .await?;
        //
        // let buffer = Arc::new(Mutex::new(String::new()));
        //
        // let stream = response.bytes_stream().map(move |chunk_result| {
        //     match chunk_result {
        //         Ok(chunk) => {
        //             let mut buffer_guard = buffer.lock().unwrap();
        //             buffer_guard.push_str(&String::from_utf8_lossy(&chunk));
        //
        //             // Process complete SSE events (separated by double newlines)
        //             while let Some(event_end) = buffer_guard.find("\n\n") {
        //                 let event_text = &buffer_guard[..event_end];
        //                 if let Some(chunk_data) = Self::parse_sse_event(event_text) {
        //                     buffer_guard.drain(..event_end + 2);
        //                     return chunk_data;
        //                 }
        //                 buffer_guard.drain(..event_end + 2);
        //             }
        //
        //             Ok(StreamChunkData {
        //                 text: String::new(),
        //                 stop_reason: None,
        //             })
        //         }
        //         Err(e) => Err(e.into()),
        //     }
        // });
        //
        // Ok(LanguageModelStreamResponse {
        //     stream: Box::pin(stream),
        //     model: Some(self.settings.model_name.to_string()),
        // })
        todo!()
    }
}
