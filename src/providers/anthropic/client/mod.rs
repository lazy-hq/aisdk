pub mod types;

pub use types::*;

use crate::Error;
use derive_builder::Builder;
use reqwest::header::CONTENT_TYPE;
use reqwest_eventsource::Event;
use serde::{Deserialize, Serialize};

use crate::{
    core::client::Client,
    providers::anthropic::{ANTHROPIC_API_VERSION, Anthropic},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), build_fn(error = "Error"))]
pub struct AnthropicOptions {
    pub model: String,
    pub messages: Vec<AnthropicMessageParam>,
    pub max_tokens: u32,
    pub stop_sequences: Option<Vec<String>>,
    pub stream: Option<bool>,
    pub system: Option<String>,
    pub temperature: Option<f32>,
    pub thinking: Option<AnthropicThinking>,
    pub tools: Option<Vec<AnthropicTool>>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
}

impl AnthropicOptions {
    pub fn builder() -> AnthropicOptionsBuilder {
        AnthropicOptionsBuilder::default()
    }
}

impl Client for Anthropic {
    type Response = AnthropicMessageResponse;
    type StreamEvent = AnthropicStreamEvent;

    fn path(&self) -> &str {
        "/messages"
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        // Default headers
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        default_headers.insert("x-api-key", self.settings.api_key.parse().unwrap());
        default_headers.insert("anthropic-version", ANTHROPIC_API_VERSION.parse().unwrap());

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
                Event::Open => Ok(AnthropicStreamEvent::NotSupported("{}".to_string())),
                Event::Message(msg) => {
                    if msg.data.trim() == "[DONE]" || msg.data.is_empty() {
                        return Ok(AnthropicStreamEvent::NotSupported("[END]".to_string()));
                    }

                    let value: serde_json::Value = serde_json::from_str(&msg.data)
                        .map_err(|e| Error::ApiError(format!("Invalid JSON in SSE data: {}", e)))?;

                    Ok(serde_json::from_value::<AnthropicStreamEvent>(value)
                        .unwrap_or(AnthropicStreamEvent::NotSupported(msg.data)))
                }
            },
            Err(e) => Err(Error::ApiError(format!("SSE error: {}", e))),
        }
    }

    fn end_stream(event: &Self::StreamEvent) -> bool {
        matches!(event, AnthropicStreamEvent::MessageStop)
            || matches!(event, AnthropicStreamEvent::NotSupported(json) if json == "[END]")
    }
}
