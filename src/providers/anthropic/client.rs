//! This module provides the Anthropic client, an HTTP client for interacting with the Anthropic API.
//! It is a thin wrapper around the `reqwest` crate.
//! HTTP requests have this parts:
use super::utils::default_antropic_value;
use crate::error::{Error, Result};
use derive_builder::Builder;
use futures::{Stream, StreamExt};
use reqwest::{self, header::CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

const ANTHROPIC_API_VERSION: &str = "2023-06-01"; // TODO: move this to settings

// ---------------------------------- Antropic API types ----------------------------------

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum AnthropicErrorType {
    #[serde(rename = "invalid_request_error")]
    #[default]
    InvalidRequestError,
    AuthenticationError,
    PermissionError,
    NotFoundError,
    RequestTooLarge,
    RateLimitError,
    ApiError,
    OverloadedError,
}

impl std::fmt::Display for AnthropicErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnthropicErrorType::InvalidRequestError => write!(f, "invalid_request_error"),
            AnthropicErrorType::AuthenticationError => write!(f, "authentication_error"),
            AnthropicErrorType::PermissionError => write!(f, "permission_error"),
            AnthropicErrorType::NotFoundError => write!(f, "not_found_error"),
            AnthropicErrorType::RequestTooLarge => write!(f, "request_too_large"),
            AnthropicErrorType::RateLimitError => write!(f, "rate_limit_error"),
            AnthropicErrorType::ApiError => write!(f, "api_error"),
            AnthropicErrorType::OverloadedError => write!(f, "overloaded_error"),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnthropicError {
    #[serde(rename = "type")]
    pub type_: AnthropicErrorType,
    pub message: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum AnthropicStopReason {
    #[default]
    #[serde(rename = "end_turn")]
    EndTurn,
    #[serde(rename = "max_tokens")]
    MaxTokens,
    #[serde(rename = "stop_sequence")]
    StopSequence,
    #[serde(rename = "tool_use")]
    ToolUse,
    #[serde(rename = "pause_turn")]
    PauseTurn,
    #[serde(rename = "refusal")]
    Refusal,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AntropicMessageResponse {
    pub id: String,
    pub content: Vec<AntropicContentBlock>,
    pub model: String,
    #[serde(default = "default_antropic_value::assistant")]
    role: String, // always "assistant"
    pub stop_reason: Option<String>,
    pub stop_sequences: Option<Vec<String>>,
    #[serde(rename = "type", default = "default_antropic_value::text")]
    type_: String,
    pub usage: AntropicUsage,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AntropicUsage {
    pub cache_creation: AntropicCacheCreation,
    pub cache_creation_input_tokens: usize,
    pub cache_read_input_tokens: usize,
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub server_tool_use: AntropicServerToolUsage,
    pub service_tier: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AntropicCacheCreation {
    pub ephemeral_1h_input_tokens: usize,
    pub ephemeral_5m_input_tokens: usize,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AntropicServerToolUsage {
    pub web_search_requests: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AntropicContentBlock {
    #[serde(rename = "text")]
    Text {
        text: String,
        citations: Vec<AntropicCitation>,
    },
    #[serde(rename = "thinking")]
    Thinking { signature: String, thinking: String },
    #[serde(rename = "redacted_thinking")]
    RedactedThinking { data: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        input: serde_json::Value, // TODO: not sure if this is 'Value' check with Anthropic docs
        name: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AntropicCitation {
    CitationCharLocation {
        cited_text: String,
        document_index: usize,
        document_title: String,
        end_char_index: usize,
        file_id: String,
        start_char_index: usize,
    },
    CitationPageLocation {
        cited_text: String,
        document_index: usize,
        document_title: String,
        end_page_number: usize,
        file_id: String,
        start_page_number: usize,
    },
    CitationContentBlockLocation {
        cited_text: String,
        document_index: usize,
        document_title: String,
        end_block_index: usize,
        file_id: String,
        start_block_index: usize,
    },
    CitationsWebSearchResultLocation {
        cited_text: String,
        encrypted_index: String,
        title: String,
    },
    CitationsSearchResultLocation {
        cited_text: String,
        end_block_index: usize,
        search_result_index: usize,
        source: String,
        start_block_index: usize,
        title: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "role")]
pub enum AntropicMessageParam {
    #[serde(rename = "user")]
    User { content: String },
    #[serde(rename = "assistant")]
    Assistant {
        content: AnthropicAssistantMessageParamContent,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnthropicAssistantMessageParamContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking { thinking: String, signature: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        input: serde_json::Value, // TODO: not sure if this is 'Value' check with Anthropic docs
        name: String,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AntropicTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AntropicThinking {
    #[default]
    #[serde(rename = "disable")]
    Disable,
    #[serde(rename = "enable")]
    Enable { budget_tokens: usize },
}

// ---------------------------------- Streaming types ----------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnthropicStreamEvent {
    #[serde(rename = "message_start")]
    MessageStart { message: AntropicMessageResponse },
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: usize,
        content_block: AntropicContentBlock,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta { index: usize, delta: AnthropicDelta },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: usize },
    #[serde(rename = "message_delta")]
    MessageDelta {
        delta: AnthropicMessageDelta,
        usage: AnthropicMessageDeltaUsage,
    },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "error")]
    Error { error: AnthropicError },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnthropicDelta {
    #[serde(rename = "text_delta")]
    TextDelta { text: String },
    #[serde(rename = "thinking_delta")]
    ThinkingDelta { thinking: String },
    #[serde(rename = "input_json_delta")]
    ToolUseDelta { partial_json: String },
    #[serde(rename = "citation_delta")]
    CitationDelta { citation: AntropicCitation },
    #[serde(rename = "signature_delta")]
    SignatureDelta { signature: String },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnthropicMessageDeltaUsage {
    pub cache_creation_input_tokens: Option<usize>,
    pub cache_read_input_tokens: Option<usize>,
    pub input_tokens: Option<usize>,
    pub output_tokens: usize,
    pub server_tool_use: Option<AntropicServerToolUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnthropicMessageDelta {
    pub stop_reason: Option<AnthropicStopReason>,
    pub stop_sequence: Option<String>,
}

pub(super) trait Request {
    type Response: DeserializeOwned;
    type StreamEvent: DeserializeOwned;

    fn path(&self) -> &str;
    fn method(&self) -> reqwest::Method;
    fn query_params(&self) -> Vec<(&str, &str)>;
    fn body(&self) -> reqwest::Body;
    fn streaming_body(&self) -> reqwest::Body;

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

    async fn send(&self) -> Result<Self::Response> {
        let client = reqwest::Client::new();
        let resp = client
            .request(self.method(), self.path())
            .headers(self.headers())
            .query(&self.query_params())
            .body(self.body())
            .send()
            .await
            .and_then(|response| response.error_for_status())
            .map_err(|e| Error::ApiError(e.to_string()));

        resp?
            .json::<Self::Response>()
            .await
            .map_err(|e| Error::ApiError(e.to_string()))
    }

    async fn send_and_stream(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Self::StreamEvent>> + Send>>> {
        let client = reqwest::Client::new();
        let resp = client
            .request(self.method(), self.path())
            .headers(self.headers())
            .query(&self.query_params())
            .body(self.streaming_body())
            .send()
            .await
            .and_then(|response| response.error_for_status())
            .map_err(|e| Error::ApiError(e.to_string()))?;

        let stream = resp.bytes_stream().map(|result| {
            result
                .map_err(|e| Error::ApiError(e.to_string()))
                .and_then(|bytes| {
                    let text = String::from_utf8(bytes.to_vec())
                        .map_err(|e| Error::ApiError(format!("UTF-8 error: {}", e)))?;
                    let mut events = vec![];
                    for message in text.split("\n\n") {
                        if let Some(data_line) =
                            message.lines().find(|line| line.starts_with("data: "))
                        {
                            let json_str = &data_line[6..];
                            if json_str.trim().is_empty() || json_str.trim() == "[DONE]" {
                                continue;
                            }
                            let event: Self::StreamEvent = serde_json::from_str(json_str)
                                .map_err(|e| Error::ApiError(format!("JSON parse error: {}", e)))?;
                            events.push(event);
                        }
                    }
                    if events.is_empty() {
                        Err(Error::ApiError("No events parsed".to_string()))
                    } else {
                        Ok(events.into_iter().next().unwrap())
                    }
                })
        });

        Ok(Box::pin(stream))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), build_fn(error = "Error"))]
pub struct AnthropicClient {
    pub model: String,
    pub messages: Vec<AntropicMessageParam>,
    pub max_tokens: u32,
    pub stop_sequences: Option<Vec<String>>,
    pub stream: Option<bool>,
    pub system: Option<String>,
    pub temperature: Option<f32>,
    pub thinking: Option<AntropicThinking>,
    pub tools: Option<Vec<AntropicTool>>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
}

impl AnthropicClient {
    pub fn builder() -> AnthropicClientBuilder {
        AnthropicClientBuilder::default()
    }
}

impl Request for AnthropicClient {
    type Response = AntropicMessageResponse;
    type StreamEvent = AnthropicStreamEvent;

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

    fn streaming_body(&self) -> reqwest::Body {
        let mut clone = self.clone();
        clone.stream = Some(true);
        reqwest::Body::from(serde_json::to_string(&clone).unwrap())
    }
}
