//! This module provides the Anthropic client, an HTTP client for interacting with the Anthropic API.
//! It is a thin wrapper around the `reqwest` crate.
//! HTTP requests have this parts:

use crate::error::{Error, Result};
use reqwest::{self, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};

const ANTHROPIC_BASE_URL: &str = "https://api.anthropic.com/v1";
const ANTHROPIC_API_VERSION: &str = "2023-06-01";

// ---------------------------------- Antropic API types ----------------------------------
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    #[default]
    User,
    Assistant,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Message {
    role: MessageRole,
    content: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Tool {
    name: String,
    description: String,
    input_schema: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BudgetToken(u32);

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Thinking {
    #[default]
    Disable,
    Enable(BudgetToken),
}

pub trait Request {
    fn path(&self) -> &str;
    fn method(&self) -> reqwest::Method;
    fn query_params(&self) -> Vec<(&str, &str)>;
    fn body(&self) -> reqwest::Body;

    /// Sets the default headers for the request
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

    async fn send(&self) -> Result<reqwest::Response> {
        let client = reqwest::Client::new();
        client
            .request(self.method(), self.path())
            .headers(self.headers())
            .query(&self.query_params())
            .body(self.body())
            .send()
            .await
            .map_err(|e| Error::ApiError(e.to_string()))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AntropicRequest {
    pub model: String,
    pub system: Option<String>,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub stop_sequences: Option<Vec<String>>,
    pub stream: Option<bool>,
    pub temperature: Option<f32>,
    pub thinking: Option<Thinking>,
    pub tools: Option<Vec<Tool>>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
}

impl Request for AntropicRequest {
    fn path(&self) -> &str {
        "/messages"
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn query_params(&self) -> Vec<(&str, &str)> {
        todo!()
    }

    fn body(&self) -> reqwest::Body {
        let body = serde_json::to_string(self).unwrap();
        reqwest::Body::from(body)
    }
}
