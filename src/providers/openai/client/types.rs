use crate::{core::client::NotSupportedEvent, error::ProviderError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenAiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationParam>,
    pub created_at: Option<f64>,
    pub error: Option<OpenAIErrorByCode>,
    pub id: Option<String>,
    pub incomplete_details: Option<IncompleteDetails>,
    pub max_output_tokens: Option<u32>,
    pub max_tool_calls: Option<u32>,
    pub model: Option<String>,
    pub output: Option<Vec<MessageItem>>,
    pub parallel_tool_calls: Option<bool>,
    pub previous_response_id: Option<String>,
    pub reasoning: Option<ReasoningConfig>,
    pub text: Option<TextConfig>,
    pub usage: Option<ResponseUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
#[serde(tag = "type")]
pub enum OpenAiStreamEvent {
    /// Emitted when the model response is complete.
    #[serde(rename = "response.completed")]
    ResponseCompleted {
        sequence_number: u64,
        response: OpenAiResponse,
    },
    /// An event that is emitted when a response finishes as incomplete.
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete {
        sequence_number: u64,
        response: OpenAiResponse,
    },
    /// Emitted when there is an additional text delta.
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta {
        sequence_number: u64,
        item_id: String,
        output_index: u32,
        content_index: u32,
        delta: String,
        logprobs: Option<Vec<LogProbs>>,
    },
    /// Emitted when text content is finalized.
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone {
        sequence_number: u64,
        item_id: String,
        output_index: u32,
        content_index: u32,
        text: String,
        logprobs: Option<Vec<LogProbs>>,
    },
    /// Emitted when there is a partial function-call arguments delta.
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta {
        sequence_number: u64,
        item_id: String,
        output_index: u32,
        delta: String,
    },
    /// Emitted when function-call arguments are finalized.
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone {
        name: Option<String>,
        sequence_number: u64,
        item_id: String,
        output_index: u32,
        arguments: String,
    },
    /// Emitted when an error occurs.
    #[serde(rename = "error")]
    ResponseError {
        sequence_number: u64,
        code: Option<String>,
        message: String,
        param: Option<String>,
    },
    NotSupported(String),
}

impl From<NotSupportedEvent> for OpenAiStreamEvent {
    fn from(event: NotSupportedEvent) -> Self {
        OpenAiStreamEvent::NotSupported(event.json)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseUsage {
    pub input_tokens: u32,
    pub input_tokens_details: InputTokenDetails,
    pub output_tokens: u32,
    pub output_tokens_details: OutputTokenDetails,
    pub total_tokens: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputTokenDetails {
    pub cached_tokens: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutputTokenDetails {
    pub reasoning_tokens: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationParam {
    pub id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct IncompleteDetails {
    pub reason: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReasoningConfig {
    pub effort: Option<ReasoningEffort>,
    pub summary: Option<SummaryType>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
    #[default]
    None,
    Minimal,
    Low,
    Medium,
    High,
    XHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ToolParams {
    Function {
        name: String,
        parameters: serde_json::Value,
        strict: bool,
        description: Option<String>,
    },
}

// auto, concise, or detailed
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SummaryType {
    #[default]
    Auto,
    Concise,
    Detailed,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextConfig {
    pub format: Option<TextResponseFormat>,
    pub verbosity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextResponseFormat {
    Text,
    JsonSchema {
        name: String,
        schema: serde_json::Value,
        description: Option<String>,
        strict: Option<bool>,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum ErrorType {
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
pub struct OpenAIError {
    #[serde(rename = "type")]
    pub type_: ErrorType,
    pub message: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenAIErrorByCode {
    pub code: String,
    pub message: String,
}

/// See <https://platform.openai.com/docs/api-reference/responses/create#responses_create-input>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Input {
    #[serde(untagged)]
    TextInput(String),
    #[serde(untagged)]
    InputItemList(Vec<InputItem>),
}

/// See <https://platform.openai.com/docs/api-reference/responses/create#responses_create-input-input_item_list>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputItem {
    /// See <https://platform.openai.com/docs/api-reference/responses/create#responses_create-input-input_item_list-input_message>
    InputMessage {
        content: InputItemContent,
        role: Role,
    },
    /// See <https://platform.openai.com/docs/api-reference/responses/create#responses_create-input-input_item_list-item>
    Item(MessageItem),
    /// See <https://platform.openai.com/docs/api-reference/responses/create#responses_create-input-input_item_list-item_reference>
    ItemReference { id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum InputItemContent {
    Text(String),
    InputItemContentList(Vec<ContentType>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
    System,
    Developer,
}

/// See <https://platform.openai.com/docs/api-reference/responses/create#responses_create-input-input_item_list-item>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
#[serde(untagged)]
pub enum MessageItem {
    InputMessage {
        content: Vec<ContentType>,
        role: Role,
        #[serde(rename = "type")]
        type_: String, // always "message"
    },
    #[serde(rename = "output")]
    OutputMessage {
        content: Vec<OutputContent>,
        id: Option<String>,
        role: Role,
        status: Option<String>,
        #[serde(rename = "type")]
        type_: String, // always "message"
    },
    FunctionCall {
        arguments: serde_json::Value,
        call_id: String,
        name: String,
        #[serde(rename = "type")]
        type_: String, // always "function_call"
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<String>,
    },
    FunctionCallOutput {
        call_id: String,
        output: FunctionCallOutput,
        #[serde(rename = "type")]
        type_: String, // always "function_call_output"
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<String>,
    },
    Reasoning {
        id: Option<String>,
        summary: Vec<ReasoningSummary>,
        #[serde(rename = "type")]
        type_: String, // always "reasoning"
        content: Option<Vec<ReasoningTextContent>>,
        encrypted_content: Option<String>,
        status: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum FunctionCallOutput {
    Text(String),
    Other(ContentType),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReasoningSummary {
    #[serde(rename = "type")]
    pub type_: String, // always "summary_text"
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReasoningTextContent {
    pub type_: String, // always "reasoning_text"
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentType {
    InputText {
        text: String,
    },
    InputImage {
        detail: ImageDetail,
        file_id: Option<String>,
        image_url: Option<String>,
    },
    InputFile {
        file_data: Option<String>,
        filename: Option<String>,
        file_url: Option<String>,
        file_id: Option<String>,
    },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum ImageDetail {
    #[default]
    Auto,
    High,
    Low,
}

/// See <https://platform.openai.com/docs/api-reference/responses/create#responses_create-input-input_item_list-item-output_message-content>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputContent {
    OutputText {
        annotations: Vec<OutputTextAnnotation>,
        logprobs: Vec<LogProbs>,
        text: String,
    },
    Refusal {
        refusal: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputTextAnnotation {
    FileCitation {
        file_id: String,
        filename: String,
        index: usize,
    },
    UrlCitation {
        end_index: usize,
        start_index: usize,
        url: String,
        title: String,
    },
    ContainerFileCitation {
        file_id: String,
        filename: String,
        start_index: usize,
    },
    FilePath {
        file_id: String,
        index: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogProbs {
    pub bytes: Vec<u8>,
    pub logprob: f64,
    pub token: String,
    pub top_logprobs: Vec<TopLogProbs>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopLogProbs {
    pub bytes: Vec<u8>,
    pub logprob: f64,
    pub token: String,
}

// ---------------------------------- Trait implementations ----------------------------------

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::InvalidRequestError => write!(f, "invalid_request_error"),
            ErrorType::AuthenticationError => write!(f, "authentication_error"),
            ErrorType::PermissionError => write!(f, "permission_error"),
            ErrorType::NotFoundError => write!(f, "not_found_error"),
            ErrorType::RequestTooLarge => write!(f, "request_too_large"),
            ErrorType::RateLimitError => write!(f, "rate_limit_error"),
            ErrorType::ApiError => write!(f, "api_error"),
            ErrorType::OverloadedError => write!(f, "overloaded_error"),
        }
    }
}

impl std::fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpenAIError: {:?} - {:?}", self.type_, self.message)
    }
}

impl std::error::Error for OpenAIError {}

impl ProviderError for OpenAIError {}

impl Default for Input {
    fn default() -> Self {
        Self::InputItemList(Vec::new())
    }
}
