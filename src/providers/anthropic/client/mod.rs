pub mod types;

pub use types::*;

use crate::Error;
use derive_builder::Builder;
use reqwest::header::CONTENT_TYPE;
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
        let body = serde_json::to_string(self).unwrap();
        reqwest::Body::from(body)
    }

    fn streaming_body(&self) -> reqwest::Body {
        let mut clone = self.options.clone();
        clone.stream = Some(true);
        reqwest::Body::from(serde_json::to_string(&clone).unwrap())
    }
}
