//! This module provides the Anthropic client, an HTTP client for interacting with the Anthropic API.
//! It is a thin wrapper around the `reqwest` crate.
//! HTTP requests have this parts:

pub mod types;

pub use types::*;

use crate::core::client::Request;
use crate::error::Error;
use derive_builder::Builder;
use reqwest::{self, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};

const ANTHROPIC_API_VERSION: &str = "2023-06-01"; // TODO: move this to settings

// ---------------------------------- Antropic API types ----------------------------------

#[derive(Debug, Default, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), build_fn(error = "Error"))]
pub struct AnthropicParams {
    pub model: String,
    pub messages: Vec<types::AnthropicMessageParam>,
    pub max_tokens: u32,
    pub stop_sequences: Option<Vec<String>>,
    pub stream: Option<bool>,
    pub system: Option<String>,
    pub temperature: Option<f32>,
    pub thinking: Option<types::AnthropicThinking>,
    pub tools: Option<Vec<types::AnthropicTool>>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
}

impl AnthropicParams {
    pub fn builder() -> AnthropicParamsBuilder {
        AnthropicParamsBuilder::default()
    }
}

impl Request for AnthropicParams {
    type Response = types::AnthropicMessageResponse;
    type StreamEvent = types::AnthropicStreamEvent;

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
        // x-api-key
        default_headers.insert(
            "x-api-key",
            std::env::var("ANTHROPIC_API_KEY")
                .expect("Please set the ANTHROPIC_API_KEY environment variable.")
                .parse()
                .unwrap(),
        );
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
        let mut clone = self.clone();
        clone.stream = Some(true);
        reqwest::Body::from(serde_json::to_string(&clone).unwrap())
    }
}
