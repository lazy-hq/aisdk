//! This module provides the OpenAI client, an HTTP client for interacting with the OpenAI API.
//! It is a thin wrapper around the `reqwest` crate.
//! HTTP requests have this parts:

pub(crate) mod types;

pub(crate) use types::*;

use crate::core::client::{EmbeddingClient, LanguageModelClient};
use crate::error::Error;
use crate::providers::openai::{ModelName, OpenAI};
use reqwest::header::CONTENT_TYPE;
use reqwest_eventsource::Event;

impl<M: ModelName> LanguageModelClient for OpenAI<M> {
    type Response = types::OpenAIResponse;
    type StreamEvent = types::OpenAiStreamEvent;

    fn path(&self) -> String {
        "/v1/responses".to_string()
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
                            details: format!("Invalid JSON in SSE data: {}", e),
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
