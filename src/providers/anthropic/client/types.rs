use serde::{Deserialize, Serialize};

use crate::error::ProviderError;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) enum AnthropicErrorType {
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct AnthropicError {
    #[serde(rename = "type")]
    pub type_: AnthropicErrorType,
    pub message: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) enum AnthropicStopReason {
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
pub(crate) struct AnthropicMessageResponse {
    pub id: String,
    pub content: Vec<AnthropicContentBlock>,
    pub model: String,
    #[serde(default = "assistant")]
    role: String, // always "assistant"
    pub stop_reason: Option<String>,
    pub stop_sequences: Option<Vec<String>>,
    #[serde(rename = "type", default = "text")]
    type_: String,
    pub usage: AnthropicUsage,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct AnthropicUsage {
    pub cache_creation: AnthropicCacheCreation,
    pub cache_creation_input_tokens: usize,
    pub cache_read_input_tokens: usize,
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub server_tool_use: AnthropicServerToolUsage,
    pub service_tier: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct AnthropicCacheCreation {
    pub ephemeral_1h_input_tokens: usize,
    pub ephemeral_5m_input_tokens: usize,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct AnthropicServerToolUsage {
    pub web_search_requests: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum AnthropicContentBlock {
    #[serde(rename = "text")]
    Text {
        text: String,
        citations: Vec<AnthropicCitation>,
    },
    #[serde(rename = "thinking")]
    Thinking { signature: String, thinking: String },
    #[serde(rename = "redacted_thinking")]
    RedactedThinking { data: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        input: String,
        name: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(clippy::enum_variant_names)]
pub(crate) enum AnthropicCitation {
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
pub(crate) enum AnthropicMessageParam {
    #[serde(rename = "user")]
    User { content: String },
    #[serde(rename = "assistant")]
    Assistant {
        content: AnthropicAssistantMessageParamContent,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum AnthropicAssistantMessageParamContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking { thinking: String, signature: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        input: String,
        name: String,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct AnthropicTool {
    pub name: String,
    pub description: String,
    pub input_schema: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum AnthropicThinking {
    #[default]
    #[serde(rename = "disable")]
    Disable,
    #[serde(rename = "enable")]
    Enable { budget_tokens: usize },
}

// ---------------------------------- Streaming types ----------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum AnthropicStreamEvent {
    #[serde(rename = "message_start")]
    MessageStart {
        message: AnthropicMessageResponse,
    },
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: usize,
        content_block: AnthropicContentBlock,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta {
        index: usize,
        delta: AnthropicDelta,
    },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop {
        index: usize,
    },
    #[serde(rename = "message_delta")]
    MessageDelta {
        delta: AnthropicMessageDelta,
        usage: AnthropicMessageDeltaUsage,
    },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "error")]
    Error {
        error: AnthropicError,
    },
    NotSupported(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(clippy::enum_variant_names)]
pub(crate) enum AnthropicDelta {
    #[serde(rename = "text_delta")]
    TextDelta { text: String },
    #[serde(rename = "thinking_delta")]
    ThinkingDelta { thinking: String },
    #[serde(rename = "input_json_delta")]
    ToolUseDelta { partial_json: String },
    #[serde(rename = "citation_delta")]
    CitationDelta { citation: AnthropicCitation },
    #[serde(rename = "signature_delta")]
    SignatureDelta { signature: String },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct AnthropicMessageDeltaUsage {
    pub cache_creation_input_tokens: Option<usize>,
    pub cache_read_input_tokens: Option<usize>,
    pub input_tokens: Option<usize>,
    pub output_tokens: usize,
    pub server_tool_use: Option<AnthropicServerToolUsage>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct AnthropicMessageDelta {
    pub stop_reason: Option<AnthropicStopReason>,
    pub stop_sequence: Option<String>,
}

// ---------------------------------- Trait implementations ----------------------------------

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

impl std::fmt::Display for AnthropicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnthropicError: {:?} - {:?}", self.type_, self.message)
    }
}

impl std::error::Error for AnthropicError {}

impl ProviderError for AnthropicError {}

// ---------------------------------- Helper functions ----------------------------------
fn assistant() -> String {
    "assistant".to_string()
}

fn text() -> String {
    "text".to_string()
}
