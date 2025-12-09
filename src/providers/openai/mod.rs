//! This module provides the OpenAI provider, which implements the `LanguageModel`
//! and `Provider` traits for interacting with the OpenAI API.

pub mod client;
pub mod conversions;
pub mod settings;

use crate::core::client::Request;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, LanguageModelStreamChunkType, ProviderStream,
};
use crate::core::messages::AssistantMessage;
use crate::providers::openai::client::types;
use crate::providers::openai::settings::{OpenAIProviderSettings, OpenAIProviderSettingsBuilder};
use crate::{
    core::{language_model::LanguageModel, provider::Provider, tools::ToolCallInfo},
    error::Result,
};
use async_trait::async_trait;
use futures::StreamExt;

/// The OpenAI provider.
#[derive(Debug, Clone)]
pub struct OpenAI {
    settings: OpenAIProviderSettings,
}

impl OpenAI {
    /// Creates a new `OpenAI` provider with the given settings.
    pub fn new(model_name: impl Into<String>) -> Self {
        OpenAIProviderSettingsBuilder::default()
            .model_name(model_name.into())
            .build()
            .expect("Failed to build OpenAIProviderSettings")
    }

    /// OpenAI provider setting builder.
    pub fn builder() -> OpenAIProviderSettingsBuilder {
        OpenAIProviderSettings::builder()
    }
}

impl Provider for OpenAI {}

#[async_trait]
impl LanguageModel for OpenAI {
    fn name(&self) -> String {
        self.settings.model_name.clone()
    }

    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let mut request: client::OpenAiParams = options.clone().into();

        request.model = self.settings.model_name.to_string();

        let response: client::OpenAiResponse = request.send(self.settings.base_url.clone()).await?;

        let mut collected: Vec<LanguageModelResponseContentType> = Vec::new();

        for out in response.output.unwrap_or_default() {
            match out {
                types::MessageItem::OutputMessage { content, .. } => {
                    for c in content {
                        match c {
                            types::OutputContent::OutputText { text, .. } => {
                                collected.push(LanguageModelResponseContentType::new(text))
                            }
                            _ => (),
                        };
                    }
                }
                types::MessageItem::FunctionCall {
                    arguments,
                    name,
                    call_id,
                    ..
                } => {
                    let mut tool_info = ToolCallInfo::new(name);
                    tool_info.id(call_id);
                    tool_info.input(arguments);
                    collected.push(LanguageModelResponseContentType::ToolCall(tool_info));
                }
                _ => (),
            }
        }

        Ok(LanguageModelResponse {
            contents: collected,
            usage: response.usage.map(|usage| usage.into()),
        })
    }

    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        let mut request: client::OpenAiParams = options.into();
        request.model = self.settings.model_name.to_string();
        request.stream = Some(true);

        let openai_stream = request
            .send_and_stream(self.settings.base_url.clone())
            .await?;

        #[derive(Default)]
        struct StreamState {
            completed: bool,
        }

        let stream = openai_stream.scan::<_, Result<Vec<LanguageModelStreamChunk>>, _, _>(
            StreamState::default(),
            |state, evt_res| {
                // If already completed, don't emit anything more
                if state.completed {
                    return futures::future::ready(None);
                };

                futures::future::ready(match evt_res {
                    Ok(client::OpenAiStreamEvent::ResponseOutputTextDelta { delta, .. }) => {
                        Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
                            LanguageModelStreamChunkType::Text(delta),
                        )])))
                    }
                    Ok(client::OpenAiStreamEvent::ResponseOutputTextDone { text, .. }) => {
                        state.completed = true;
                        Some(Ok(Vec::from([LanguageModelStreamChunk::Done(
                            AssistantMessage {
                                content: LanguageModelResponseContentType::new(text),
                                usage: None,
                            },
                        )])))
                    }
                    Ok(client::OpenAiStreamEvent::ResponseFunctionCallArgumentsDelta {
                        delta,
                        ..
                    }) => Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::ToolCall(delta),
                    )]))),
                    Ok(client::OpenAiStreamEvent::ResponseFunctionCallArgumentsDone {
                        name,
                        ..
                    }) => Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::NotSupported(format!(
                            "FunctionCall: {name:?}"
                        )),
                    )]))),
                    Ok(client::OpenAiStreamEvent::ResponseCompleted { response, .. }) => {
                        state.completed = true;

                        let mut collected: Vec<LanguageModelResponseContentType> = Vec::new();

                        for out in response.output.unwrap_or_default() {
                            match out {
                                types::MessageItem::FunctionCall {
                                    call_id,
                                    arguments,
                                    name,
                                    ..
                                } => {
                                    let mut tool_info = ToolCallInfo::new(name);
                                    tool_info.id(call_id);
                                    tool_info.input(arguments);
                                    collected.push(LanguageModelResponseContentType::ToolCall(
                                        tool_info,
                                    ));
                                }
                                _ => {}
                            }
                        }

                        Some(Ok(collected
                            .into_iter()
                            .map(|ref c| {
                                LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: c.clone(),
                                    usage: response
                                        .usage
                                        .as_ref()
                                        .map(|usage| usage.clone().into()),
                                })
                            })
                            .collect()))
                    }
                    Ok(client::OpenAiStreamEvent::ResponseIncomplete { response, .. }) => {
                        Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
                            LanguageModelStreamChunkType::Incomplete(
                                response
                                    .incomplete_details
                                    .map(|details| details.reason)
                                    .unwrap_or("Incomplete with unknown reason".to_string()),
                            ),
                        )])))
                    }
                    Ok(client::OpenAiStreamEvent::ResponseError { code, message, .. }) => {
                        state.completed = true;
                        let reason =
                            format!("{}: {}", code.unwrap_or("unknown".to_string()), message);
                        Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
                            LanguageModelStreamChunkType::Failed(reason),
                        )])))
                    }
                    Ok(evt) => Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::NotSupported(format!("{evt:?}")),
                    )]))),
                    Err(e) => {
                        state.completed = true;
                        Some(Err(e))
                    }
                })
            },
        );

        Ok(Box::pin(stream))
    }
}
