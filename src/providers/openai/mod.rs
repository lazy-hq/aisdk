//! This module provides the OpenAI provider, which implements the `LanguageModel`
//! and `Provider` traits for interacting with the OpenAI API.

pub mod client;
pub mod conversions;
pub mod settings;

use crate::core::client::Client;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, LanguageModelStreamChunkType, ProviderStream, Usage,
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
                    tool_info.input(serde_json::from_str(&arguments).unwrap_or_default());
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

        let stream = openai_stream.map(|evt_res| match evt_res {
            Ok(client::OpenAiStreamEvent::ResponseOutputTextDelta { delta, .. }) => {
                Ok(vec![LanguageModelStreamChunk::Delta(
                    LanguageModelStreamChunkType::Text(delta),
                )])
            }
            Ok(client::OpenAiStreamEvent::ResponseReasoningSummaryTextDelta { delta, .. }) => {
                Ok(vec![LanguageModelStreamChunk::Delta(
                    LanguageModelStreamChunkType::Reasoning(delta),
                )])
            }
            Ok(client::OpenAiStreamEvent::ResponseCompleted { response, .. }) => {
                let mut result: Vec<LanguageModelStreamChunk> = Vec::new();

                let usage: Usage = response.usage.unwrap_or_default().into();
                let output = response.output.unwrap_or_default();
                let last_message: Option<types::MessageItem> = output.last().cloned();

                match &last_message {
                    // ---- Final OutputMessage ----
                    Some(types::MessageItem::OutputMessage { content, .. }) => {
                        if let Some(types::OutputContent::OutputText { text, .. }) = content.first()
                        {
                            result.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                content: LanguageModelResponseContentType::new(text.clone()),
                                usage: Some(usage.clone()),
                            }));
                        }
                    }

                    // ---- Reasoning ----
                    Some(types::MessageItem::Reasoning { summary, .. }) => {
                        if let Some(types::ReasoningSummary { text, .. }) = summary.first() {
                            result.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                content: LanguageModelResponseContentType::Reasoning(
                                    text.to_owned(),
                                ),
                                usage: Some(usage.clone()),
                            }));
                        }
                    }

                    // ---- FunctionCall ----
                    Some(types::MessageItem::FunctionCall {
                        call_id,
                        name,
                        arguments,
                        ..
                    }) => {
                        let mut tool_info = ToolCallInfo::new(name.clone());
                        tool_info.id(call_id.clone());
                        tool_info.input(serde_json::from_str(arguments).unwrap_or_default());

                        result.push(LanguageModelStreamChunk::Done(AssistantMessage {
                            content: LanguageModelResponseContentType::ToolCall(tool_info),
                            usage: Some(usage.clone()),
                        }));
                    }

                    _ => {}
                }

                Ok(result)
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
            Err(e) => Err(e),
        });

        Ok(Box::pin(stream))
    }
}
