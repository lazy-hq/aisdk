//! This module provides the OpenAI provider, which implements the `LanguageModel`
//! and `Provider` traits for interacting with the OpenAI API.

pub mod client;
pub mod conversions;
pub mod settings;

use crate::core::client::Client;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, LanguageModelStreamChunkType, ProviderStream,
};
use crate::core::messages::AssistantMessage;
use crate::providers::openai::client::{OpenAIOptions, types};
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
    options: OpenAIOptions,
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
        self.options.model.clone()
    }

    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let mut options: OpenAIOptions = options.into();
        options.model = self.options.model.clone();

        self.options = options;

        let response: client::OpenAiResponse = self.send(self.settings.base_url.clone()).await?;

        let mut collected: Vec<LanguageModelResponseContentType> = Vec::new();

        for out in response.output.unwrap_or_default() {
            match out {
                types::MessageItem::OutputMessage { content, .. } => {
                    for c in content {
                        if let types::OutputContent::OutputText { text, .. } = c {
                            collected.push(LanguageModelResponseContentType::new(text))
                        }
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
        let mut options: OpenAIOptions = options.into();
        options.model = self.options.model.to_string();
        options.stream = Some(true);

        self.options = options;

        let openai_stream = self.send_and_stream(self.settings.base_url.clone()).await?;

        let stream = openai_stream.map(|evt_res| {
            match evt_res {
                Ok(client::OpenAiStreamEvent::ResponseOutputTextDelta { delta, .. }) => {
                    Ok(vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::Text(delta),
                    )])
                }
                Ok(client::OpenAiStreamEvent::ResponseOutputTextDone { text, .. }) => {
                    Ok(vec![LanguageModelStreamChunk::Done(
                        AssistantMessage {
                            content: LanguageModelResponseContentType::new(text),
                            usage: None,
                        },
                    )])
                }
                Ok(client::OpenAiStreamEvent::ResponseFunctionCallArgumentsDone {
                    name,
                    item_id,
                    arguments,
                    ..
                }) => {
                    let mut tool_info = ToolCallInfo::new(name);
                    tool_info.id(item_id);
                    tool_info.input(
                        serde_json::from_str(&arguments)
                            .unwrap_or_else(|_| serde_json::Value::String(arguments).into()),
                    );

                    Ok(vec![LanguageModelStreamChunk::Done(
                        AssistantMessage {
                            content: LanguageModelResponseContentType::ToolCall(tool_info),
                            usage: None,
                        },
                    )])
                }
                Ok(client::OpenAiStreamEvent::ResponseCompleted { response, .. }) => {
                    //println!("response completed: {:?}", response);
                    let mut chunks = Vec::new();
                    for out in response.output.unwrap_or_default() {
                        println!("output: {:?}", out);
                        if let types::MessageItem::FunctionCall {
                            call_id,
                            arguments,
                            name,
                            ..
                        } = out
                        {
                            let input = arguments;
                            let mut tool_info = ToolCallInfo::new(name);
                            tool_info.id(call_id);
                            tool_info.input(input);
                            chunks.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                content: LanguageModelResponseContentType::ToolCall(tool_info),
                                usage: response.usage.as_ref().map(|u| u.clone().into()),
                            }));
                        }
                    }
                    Ok(chunks)
                }
                Ok(client::OpenAiStreamEvent::ResponseIncomplete { response, .. }) => {
                    Ok(vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::Incomplete(
                            response
                                .incomplete_details
                                .map(|d| d.reason)
                                .unwrap_or("Unknown".to_string()),
                        ),
                    )])
                }
                Ok(client::OpenAiStreamEvent::ResponseError { code, message, .. }) => {
                    let reason = format!("{}: {}", code.unwrap_or("unknown".to_string()), message);
                    Ok(vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::Failed(reason),
                    )])
                }
                Ok(evt) => Ok(vec![LanguageModelStreamChunk::Delta(
                    LanguageModelStreamChunkType::NotSupported(format!("{evt:?}")),
                )]),
                Err(e) => {
                    let reason = format!("Stream error: {}", e);
                    Ok(vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::Failed(reason),
                    )])
                }
            }
        });

        Ok(Box::pin(stream))
    }
}
