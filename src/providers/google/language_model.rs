//! Language model implementation for the Google provider.
use crate::core::capabilities::ModelName;
use crate::core::client::LanguageModelClient;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, LanguageModelStreamChunkType, ProviderStream, Usage,
};
use crate::core::messages::AssistantMessage;
use crate::providers::google::{Google, client::types, extensions};
use crate::{
    core::{language_model::LanguageModel, tools::ToolCallInfo},
    error::Result,
};
use async_trait::async_trait;
use futures::StreamExt;

#[async_trait]
impl<M: ModelName> LanguageModel for Google<M> {
    fn name(&self) -> String {
        self.lm_options.model.clone()
    }

    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let additional_headers = options.headers.clone();
        let request: types::GenerateContentRequest = options.into();
        self.lm_options.request = Some(request);
        self.lm_options.streaming = false;

        let response: types::GenerateContentResponse = self
            .send(&self.settings.base_url, additional_headers)
            .await?;

        let mut collected = Vec::new();
        let usage = response.usage_metadata.map(|u| u.into());

        for candidate in response.candidates {
            for part in candidate.content.parts {
                if let Some(t) = part.text {
                    collected.push(LanguageModelResponseContentType::Text(t));
                }
                if let Some(fc) = part.function_call {
                    let mut tool_info = ToolCallInfo::new(fc.name);
                    tool_info.input(fc.args);
                    if let Some(sig) = part.thought_signature {
                        tool_info
                            .extensions
                            .get_mut::<extensions::GoogleToolMetadata>()
                            .thought_signature = Some(sig);
                    }
                    collected.push(LanguageModelResponseContentType::ToolCall(tool_info));
                }
            }
        }

        Ok(LanguageModelResponse {
            contents: collected,
            usage,
        })
    }

    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        let additional_headers = options.headers.clone();
        let request: types::GenerateContentRequest = options.into();
        self.lm_options.request = Some(request);
        self.lm_options.streaming = true;

        // Retry logic for rate limiting
        let max_retries = 5;
        let mut retry_count = 0;
        let mut wait_time = std::time::Duration::from_secs(1);

        let google_stream = loop {
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

        #[derive(Default)]
        struct StreamState {
            accumulated_text: String,
            accumulated_tool_call: Option<ToolCallInfo>,
            usage: Option<Usage>,
        }

        let stream = google_stream.scan(StreamState::default(), |state, evt_res| {
            futures::future::ready(match evt_res {
                Ok(types::GoogleStreamEvent::Response(response)) => {
                    let mut chunks = Vec::new();

                    if let Some(usage) = response.usage_metadata.clone().map(Usage::from) {
                        state.usage = Some(usage);
                    }

                    for candidate in &response.candidates {
                        for part in &candidate.content.parts {
                            if let Some(t) = &part.text {
                                state.accumulated_text.push_str(t);
                                chunks.push(LanguageModelStreamChunk::Delta(
                                    LanguageModelStreamChunkType::Text(t.clone()),
                                ));
                            }
                            if let Some(fc) = &part.function_call {
                                let mut tool_info = ToolCallInfo::new(fc.name.clone());
                                tool_info.input(fc.args.clone());
                                if let Some(sig) = &part.thought_signature {
                                    tool_info
                                        .extensions
                                        .get_mut::<extensions::GoogleToolMetadata>()
                                        .thought_signature = Some(sig.clone());
                                }
                                state.accumulated_tool_call = Some(tool_info);

                                chunks.push(LanguageModelStreamChunk::Delta(
                                    LanguageModelStreamChunkType::ToolCall(
                                        serde_json::to_string(&fc).unwrap_or_default(),
                                    ),
                                ));
                            }
                        }

                        if candidate.finish_reason.is_some() {
                            let content = if let Some(tc) = state.accumulated_tool_call.take() {
                                LanguageModelResponseContentType::ToolCall(tc)
                            } else {
                                let text = std::mem::take(&mut state.accumulated_text);
                                LanguageModelResponseContentType::Text(text)
                            };

                            chunks.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                content,
                                usage: state.usage.clone(),
                            }));
                        }
                    }
                    Some(Ok(chunks))
                }
                Ok(types::GoogleStreamEvent::NotSupported(msg)) => {
                    Some(Ok(vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::NotSupported(msg),
                    )]))
                }
                Err(e) => Some(Err(e)),
            })
        });

        Ok(Box::pin(stream))
    }
}
