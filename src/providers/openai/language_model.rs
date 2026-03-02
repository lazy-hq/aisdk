//! Language model implementation for the OpenAI provider.

use crate::core::capabilities::ModelName;
use crate::core::client::LanguageModelClient;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, LanguageModelStreamChunkType, ProviderStream, Usage,
};
use crate::core::messages::AssistantMessage;
use crate::providers::openai::client::{OpenAILanguageModelOptions, types};
use crate::providers::openai::{OpenAI, client};
use crate::{
    core::{language_model::LanguageModel, tools::ToolCallInfo},
    error::Result,
};
use async_trait::async_trait;
use futures::StreamExt;

#[async_trait]
impl<M: ModelName> LanguageModel for OpenAI<M> {
    /// Returns the name of the model.
    fn name(&self) -> String {
        self.lm_options.model.clone()
    }

    /// Generates text using the OpenAI provider.
    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let additional_headers = options.headers.clone();
        let mut options: OpenAILanguageModelOptions = options.into();

        options.model = self.lm_options.model.clone();

        self.lm_options = options;

        let response: client::OpenAIResponse = self
            .send(&self.settings.base_url, additional_headers)
            .await?;

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

    /// Streams text using the OpenAI provider.
    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        let additional_headers = options.headers.clone();
        let mut options: OpenAILanguageModelOptions = options.into();

        options.model = self.lm_options.model.to_string();
        options.stream = Some(true);

        self.lm_options = options;

        // Retry logic for rate limiting
        let max_retries = 5;
        let mut retry_count = 0;
        let mut wait_time = std::time::Duration::from_secs(1);

        let openai_stream = loop {
            match self
                .send_and_stream(&self.settings.base_url, additional_headers.clone())
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

                for msg in output {
                    match &msg {
                        // ---- Final OutputMessage ----
                        types::MessageItem::OutputMessage { content, .. } => {
                            if let Some(types::OutputContent::OutputText { text, .. }) =
                                content.first()
                            {
                                result.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: LanguageModelResponseContentType::new(text.clone()),
                                    usage: Some(usage.clone()),
                                }));
                            }
                        }

                        // ---- Reasoning ----
                        types::MessageItem::Reasoning { summary, .. } => {
                            if let Some(types::ReasoningSummary { text, .. }) = summary.first() {
                                result.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: LanguageModelResponseContentType::Reasoning {
                                        content: text.to_owned(),
                                        extensions: crate::extensions::Extensions::default(),
                                    },
                                    usage: Some(usage.clone()),
                                }));
                            }
                        }

                        // ---- FunctionCall ----
                        types::MessageItem::FunctionCall {
                            call_id,
                            name,
                            arguments,
                            ..
                        } => {
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
