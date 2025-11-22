//! This module provides the Anthropic client, an HTTP client for interacting with the Anthropic API.
//! It is a thin wrapper around the `reqwest` crate.
//! HTTP requests have this parts:
use super::utils::default_antropic_value;
use crate::error::{Error, Result};
use derive_builder::Builder;
use reqwest::{self, header::CONTENT_TYPE};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

const ANTHROPIC_API_VERSION: &str = "2023-06-01"; // TODO: move this to settings

// ---------------------------------- Antropic API types ----------------------------------

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

pub(super) trait Request {
    type Response: DeserializeOwned;

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

        let body = resp?
            .json::<Self::Response>()
            .await
            .map_err(|e| Error::ApiError(e.to_string()));

        body
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

// ---------------------------------- Legacy Antropic API Impl ----------------------------------
//
// impl Anthropic {
//     fn parse_sse_event(event_text: &str) -> Option<Result<StreamChunkData>> {
//         let data_str = event_text
//             .lines()
//             .find_map(|line| line.strip_prefix("data: "))
//             .map(str::trim)?;
//
//         if data_str == "[DONE]" {
//             return Some(Ok(StreamChunkData {
//                 text: String::new(),
//                 stop_reason: Some("stop".to_string()),
//             }));
//         }
//
//         if let Ok(event) = serde_json::from_str::<AnthropicStreamEvent>(data_str)
//             && event.event_type == "content_block_delta"
//             && let Some(delta) = event.data.get("delta")
//             && let Some(text) = delta.get("text")
//             && let Some(text_str) = text.as_str()
//         {
//             return Some(Ok(StreamChunkData {
//                 text: text_str.to_string(),
//                 stop_reason: None,
//             }));
//         }
//
//         if let Ok(event) = serde_json::from_str::<AnthropicStreamEvent>(data_str)
//             && event.event_type == "message_stop"
//         {
//             return Some(Ok(StreamChunkData {
//                 text: String::new(),
//                 stop_reason: Some("stop".to_string()),
//             }));
//         }
//         None
//     }
// }
