//! This module provides the OpenAI client, an HTTP client for interacting with the OpenAI API.
//! It is a thin wrapper around the `reqwest` crate.
//! HTTP requests have this parts:

pub(crate) mod types;

pub(crate) use types::*;

use crate::core::client::{EmbeddingClient, LanguageModelClient};
use crate::error::Error;
use crate::providers::openai::{ModelName, OpenAI};
use async_stream::try_stream;
use futures::Stream;
use futures::StreamExt;
use reqwest::header::CONTENT_TYPE;
use reqwest_eventsource::{Event, RequestBuilderExt};
use std::collections::HashMap;
use std::pin::Pin;

impl<M: ModelName> LanguageModelClient for OpenAI<M> {
    type Response = types::OpenAIResponse;
    type StreamEvent = types::OpenAiStreamEvent;

    fn path(&self) -> String {
        self.settings
            .path
            .clone()
            .unwrap_or_else(|| "/v1/responses".to_string())
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        // Default headers
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        // Authorization
        default_headers.insert(
            "Authorization",
            format!("Bearer {}", self.settings.api_key.clone())
                .parse()
                .unwrap(),
        );

        for (key, value) in &self.settings.extra_headers {
            if let (Ok(header_name), Ok(header_value)) = (
                reqwest::header::HeaderName::from_bytes(key.as_bytes()),
                reqwest::header::HeaderValue::from_str(value),
            ) {
                default_headers.insert(header_name, header_value);
            }
        }

        default_headers
    }

    fn query_params(&self) -> Vec<(&str, &str)> {
        Vec::new()
    }

    fn body(&self) -> reqwest::Body {
        let body = serde_json::to_string(&self.lm_options).unwrap();
        reqwest::Body::from(body)
    }

    fn parse_stream_sse(
        event: std::result::Result<Event, reqwest_eventsource::Error>,
    ) -> crate::error::Result<Self::StreamEvent> {
        match event {
            Ok(event) => match event {
                Event::Open => Ok(types::OpenAiStreamEvent::NotSupported("{}".to_string())),
                Event::Message(msg) => {
                    if msg.data.trim() == "[DONE]" || msg.data.is_empty() {
                        return Ok(types::OpenAiStreamEvent::NotSupported("[END]".to_string()));
                    }

                    let value: serde_json::Value =
                        serde_json::from_str(&msg.data).map_err(|e| Error::ApiError {
                            status_code: None,
                            details: format!("Invalid JSON in SSE data: {e}"),
                        })?;

                    Ok(serde_json::from_value::<types::OpenAiStreamEvent>(value)
                        .unwrap_or(types::OpenAiStreamEvent::NotSupported(msg.data)))
                }
            },
            Err(e) => {
                // Extract status code if it's an InvalidStatusCode error
                let status_code = match &e {
                    reqwest_eventsource::Error::InvalidStatusCode(status, _) => Some(*status),
                    _ => None,
                };
                Err(Error::ApiError {
                    status_code,
                    details: e.to_string(),
                })
            }
        }
    }

    fn end_stream(event: &Self::StreamEvent) -> bool {
        matches!(event, types::OpenAiStreamEvent::ResponseCompleted { .. })
            || matches!(event, types::OpenAiStreamEvent::NotSupported(json) if json == "[END]")
            || matches!(event, types::OpenAiStreamEvent::ResponseError { .. })
    }

    async fn send_and_stream(
        &self,
        base_url: impl reqwest::IntoUrl,
        additional_headers: Option<HashMap<String, String>>,
    ) -> crate::error::Result<
        Pin<Box<dyn Stream<Item = crate::error::Result<Self::StreamEvent>> + Send>>,
    >
    where
        Self::StreamEvent: Send + 'static,
        Self: Sync,
    {
        let url = crate::core::utils::join_url(base_url, &LanguageModelClient::path(self))?;
        let is_chatgpt_codex =
            url.as_str().contains("chatgpt.com/backend-api/codex") || url.path() == "/responses";

        let mut headers = LanguageModelClient::headers(self);
        if let Some(extra) = additional_headers {
            let extra_map = reqwest::header::HeaderMap::try_from(&extra)
                .map_err(|e| Error::InvalidInput(format!("Invalid headers: {}", e)))?;
            headers.extend(extra_map);
        }

        if !is_chatgpt_codex {
            let client = reqwest::Client::new();
            let events_stream = client
                .request(LanguageModelClient::method(self), url.clone())
                .headers(headers)
                .query(&LanguageModelClient::query_params(self))
                .body(LanguageModelClient::body(self))
                .eventsource()
                .map_err(|e| Error::ApiError {
                    status_code: None,
                    details: format!("SSE stream error: {e}"),
                })?;

            let mapped_stream =
                events_stream.map(|event_result| Self::parse_stream_sse(event_result));
            let ended = std::sync::Arc::new(std::sync::Mutex::new(false));
            let stream = mapped_stream.scan(ended, |ended, res| {
                let mut ended = ended.lock().unwrap();
                if *ended {
                    return futures::future::ready(None);
                }
                *ended = res.as_ref().map_or(true, |evt| Self::end_stream(evt));
                futures::future::ready(Some(res))
            });

            return Ok(Box::pin(stream));
        }

        let client = reqwest::Client::new();
        let response = client
            .request(LanguageModelClient::method(self), url)
            .headers(headers)
            .query(&LanguageModelClient::query_params(self))
            .body(LanguageModelClient::body(self))
            .send()
            .await
            .map_err(|e| Error::ApiError {
                status_code: e.status(),
                details: format!("SSE stream request failed: {e}"),
            })?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::ApiError {
                status_code: Some(status),
                details: body,
            });
        }

        let stream = try_stream! {
            let mut buffer = String::new();
            let mut byte_stream = response.bytes_stream();
            while let Some(chunk) = byte_stream.next().await {
                let bytes = chunk.map_err(|e| Error::ApiError {
                    status_code: None,
                    details: format!("SSE stream read failed: {e}"),
                })?;
                buffer.push_str(&String::from_utf8_lossy(&bytes));

                while let Some(event_end) = buffer.find("\n\n") {
                    let raw_event = buffer[..event_end].to_string();
                    buffer.drain(..event_end + 2);

                    let data = raw_event
                        .lines()
                        .filter_map(|line| line.strip_prefix("data:"))
                        .map(|line| line.trim_start())
                        .collect::<Vec<_>>()
                        .join("\n");

                    if data.trim().is_empty() {
                        continue;
                    }

                    if data.trim() == "[DONE]" {
                        yield types::OpenAiStreamEvent::NotSupported("[END]".to_string());
                        continue;
                    }

                    let value: serde_json::Value =
                        serde_json::from_str(&data).map_err(|e| Error::ApiError {
                            status_code: None,
                            details: format!("Invalid JSON in SSE data: {e}"),
                        })?;

                    let event = serde_json::from_value::<types::OpenAiStreamEvent>(value)
                        .unwrap_or(types::OpenAiStreamEvent::NotSupported(data));
                    let should_end = Self::end_stream(&event);
                    yield event;
                    if should_end {
                        return;
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}

impl<M: ModelName> EmbeddingClient for OpenAI<M> {
    type Response = types::EmbeddingResponse;

    fn path(&self) -> String {
        "/v1/embeddings".to_string()
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        // Default headers
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        // Authorization
        default_headers.insert(
            "Authorization",
            format!("Bearer {}", self.settings.api_key.clone())
                .parse()
                .unwrap(),
        );

        default_headers
    }

    fn query_params(&self) -> Vec<(&str, &str)> {
        Vec::new()
    }

    fn body(&self) -> reqwest::Body {
        let body = serde_json::to_string(&self.embedding_options).unwrap();
        reqwest::Body::from(body)
    }
}
