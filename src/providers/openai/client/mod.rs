//! This module provides the OpenAI client, an HTTP client for interacting with the OpenAI API.
//! It is a thin wrapper around the `reqwest` crate.
//! HTTP requests have this parts:

pub(crate) mod types;

pub(crate) use types::*;

use crate::core::client::Client;
use crate::error::Error;
use crate::providers::openai::{ModelName, OpenAI};
use derive_builder::Builder;
use reqwest::header::CONTENT_TYPE;
use reqwest_eventsource::Event;
use serde::{Deserialize, Serialize};

/// Configuration options for OpenAI API requests.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), build_fn(error = "Error"))]
pub(crate) struct OpenAIOptions {
    pub(crate) model: String,
    #[builder(default)]
    pub(crate) input: Option<Input>, // open ai requires input to be set
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) text: Option<types::TextConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) reasoning: Option<types::ReasoningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) max_output_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) tools: Option<Vec<ToolParams>>,
}

impl OpenAIOptions {
    pub(crate) fn builder() -> OpenAIOptionsBuilder {
        OpenAIOptionsBuilder::default()
    }
}

impl<M: ModelName> Client for OpenAI<M> {
    type Response = types::OpenAiResponse;
    type StreamEvent = types::OpenAiStreamEvent;

    fn path(&self) -> String {
        "responses".to_string()
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
        let body = serde_json::to_string(&self.options).unwrap();
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

                    let value: serde_json::Value = serde_json::from_str(&msg.data)
                        .map_err(|e| Error::ApiError(format!("Invalid JSON in SSE data: {}", e)))?;

                    Ok(serde_json::from_value::<types::OpenAiStreamEvent>(value)
                        .unwrap_or(types::OpenAiStreamEvent::NotSupported(msg.data)))
                }
            },
            Err(e) => Err(Error::ApiError(e.to_string())),
        }
    }

    fn end_stream(event: &Self::StreamEvent) -> bool {
        matches!(event, types::OpenAiStreamEvent::ResponseCompleted { .. })
            || matches!(event, types::OpenAiStreamEvent::NotSupported(json) if json == "[END]")
            || matches!(event, types::OpenAiStreamEvent::ResponseError { .. })
    }
}
