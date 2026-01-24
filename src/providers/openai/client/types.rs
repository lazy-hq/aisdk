use crate::error::Error;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Configuration options for OpenAI API requests.
#[derive(Debug, Default, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), build_fn(error = "Error"))]
pub(crate) struct OpenAILanguageModelOptions {
    pub(crate) model: String,
    #[builder(default)]
    pub(crate) input: Option<Input>, // open ai requires input to be set
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) text: Option<TextConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) reasoning: Option<ReasoningConfig>,
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

/// Response structure from the OpenAI API.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct OpenAIResponse {
    /// Conversation parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationParam>,
    /// Timestamp of creation.
    pub created_at: Option<f64>,
    /// Error information if present.
    pub error: Option<OpenAIErrorByCode>,
    /// Unique identifier.
    pub id: Option<String>,
    /// Details for incomplete responses.
    pub incomplete_details: Option<IncompleteDetails>,
    /// Maximum output tokens.
    pub max_output_tokens: Option<u32>,
    /// Maximum tool calls.
    pub max_tool_calls: Option<u32>,
    /// Model used.
    pub model: Option<String>,
    /// Output messages.
    pub output: Option<Vec<MessageItem>>,
    /// Whether parallel tool calls are enabled.
    pub parallel_tool_calls: Option<bool>,
    /// Previous response ID.
    pub previous_response_id: Option<String>,
    /// Reasoning configuration.
    pub reasoning: Option<ReasoningConfig>,
    /// Text configuration.
    pub text: Option<TextConfig>,
    /// Usage statistics.
    pub usage: Option<ResponseUsage>,
}
impl OpenAILanguageModelOptions {
    pub(crate) fn builder() -> OpenAILanguageModelOptionsBuilder {
        OpenAILanguageModelOptionsBuilder::default()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
#[serde(tag = "type")]
/// Events emitted during streaming from OpenAI.
pub(crate) enum OpenAiStreamEvent {
    /// Emitted when the model response is complete.
    #[serde(rename = "response.completed")]
    ResponseCompleted {
        sequence_number: u64,
        response: OpenAIResponse,
    },
    /// An event that is emitted when a response finishes as incomplete.
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete {
        sequence_number: u64,
        response: OpenAIResponse,
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
    /// Emitted when a delta is added to a reasoning summary text.
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryTextDelta {
        sequence_number: u64,
        item_id: String,
        output_index: u32,
        summary_index: u32,
        delta: String,
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

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
/// Token usage statistics from OpenAI response.
pub(crate) struct ResponseUsage {
    /// Number of input tokens.
    pub input_tokens: u32,
    /// Details of input tokens.
    pub input_tokens_details: InputTokenDetails,
    /// Number of output tokens.
    pub output_tokens: u32,
    /// Details of output tokens.
    pub output_tokens_details: OutputTokenDetails,
    /// Total tokens used.
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct EmbeddingUsage {
    pub total_tokens: u32,
    pub prompt_tokens: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct InputTokenDetails {
    pub cached_tokens: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct OutputTokenDetails {
    pub reasoning_tokens: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct ConversationParam {
    pub id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct IncompleteDetails {
    pub reason: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct ReasoningConfig {
    pub effort: Option<ReasoningEffort>,
    pub summary: Option<SummaryType>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReasoningEffort {
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
pub(crate) enum ToolParams {
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
pub(crate) enum SummaryType {
    #[default]
    Auto,
    Concise,
    Detailed,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct TextConfig {
    pub format: Option<TextResponseFormat>,
    pub verbosity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum TextResponseFormat {
    Text,
    JsonSchema {
        name: String,
        schema: serde_json::Value,
        description: Option<String>,
        strict: Option<bool>,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct OpenAIErrorByCode {
    pub code: String,
    pub message: String,
}

/// See <https://platform.openai.com/docs/api-reference/responses/create#responses_create-input>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub(crate) enum Input {
    #[serde(untagged)]
    TextInput(String),
    #[serde(untagged)]
    InputItemList(Vec<InputItem>),
}

/// See <https://platform.openai.com/docs/api-reference/responses/create#responses_create-input-input_item_list>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub(crate) enum InputItem {
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
pub(crate) enum InputItemContent {
    Text(String),
    InputItemContentList(Vec<ContentType>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Role {
    User,
    Assistant,
    System,
    Developer,
}

/// See <https://platform.openai.com/docs/api-reference/responses/create#responses_create-input-input_item_list-item>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
#[serde(untagged)]
pub(crate) enum MessageItem {
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
        arguments: String,
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
pub(crate) enum FunctionCallOutput {
    Text(String),
    Other(ContentType),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct ReasoningSummary {
    #[serde(rename = "type")]
    pub type_: String, // always "summary_text"
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct ReasoningTextContent {
    pub type_: String, // always "reasoning_text"
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub(crate) enum ContentType {
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
pub(crate) enum ImageDetail {
    #[default]
    Auto,
    High,
    Low,
}

/// See <https://platform.openai.com/docs/api-reference/responses/create#responses_create-input-input_item_list-item-output_message-content>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum OutputContent {
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
pub(crate) enum OutputTextAnnotation {
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
pub(crate) struct LogProbs {
    pub bytes: Vec<u8>,
    pub logprob: f64,
    pub token: String,
    pub top_logprobs: Vec<TopLogProbs>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct TopLogProbs {
    pub bytes: Vec<u8>,
    pub logprob: f64,
    pub token: String,
}

/// See [OpenAI Embedding API](https://platform.openai.com/docs/api-reference/embeddings/object)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct Embedding {
    pub embedding: Vec<f32>,
    pub index: usize,
    pub object: String, // always "embedding"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct EmbeddingResponse {
    pub object: Option<String>, // always "list"
    pub data: Vec<Embedding>,
    pub model: Option<String>,
    pub usage: Option<EmbeddingUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub(crate) struct OpenAIEmbeddingOptions {
    // TODO: The input must not exceed the max input tokens for the model
    // (8192 tokens for all embedding models), cannot be an empty string, and
    // any array must be 2048 dimensions or less.
    pub input: Vec<String>,
    pub model: String,
    pub user: Option<String>,
    pub dimensions: Option<usize>,
    pub encoding_format: Option<String>,
}
