use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenerateContentRequest {
    pub(crate) contents: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tool_config: Option<ToolConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) safety_settings: Option<Vec<SafetySetting>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) system_instruction: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) generation_config: Option<GenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cached_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct Content {
    pub(crate) role: Role,
    #[serde(default)]
    pub(crate) parts: Vec<Part>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Role {
    #[default]
    User,
    Model,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) inline_data: Option<Blob>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) function_call: Option<FunctionCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) function_response: Option<FunctionResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_data: Option<FileData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) executable_code: Option<ExecutableCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) code_execution_result: Option<CodeExecutionResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thought_signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Blob {
    pub(crate) mime_type: String,
    pub(crate) data: String, // base64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FileData {
    pub(crate) mime_type: String,
    pub(crate) file_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FunctionCall {
    pub(crate) name: String,
    pub(crate) args: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FunctionResponse {
    pub(crate) name: String,
    pub(crate) response: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExecutableCode {
    pub(crate) language: Language,
    pub(crate) code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum Language {
    Python,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CodeExecutionResult {
    pub(crate) outcome: Outcome,
    pub(crate) output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::enum_variant_names)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum Outcome {
    OutcomeUnspecified,
    OutcomeOk,
    OutcomeFailed,
    OutcomeDeadlineExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Tool {
    pub(crate) function_declarations: Option<Vec<FunctionDeclaration>>,
    pub(crate) google_search_retrieval: Option<serde_json::Value>,
    pub(crate) code_execution: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FunctionDeclaration {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) parameters: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ToolConfig {
    pub(crate) function_calling_config: Option<FunctionCallingConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FunctionCallingConfig {
    pub(crate) mode: FunctionCallingMode,
    pub(crate) allowed_function_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum FunctionCallingMode {
    ModeUnspecified,
    Auto,
    Any,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenerationConfig {
    pub(crate) stop_sequences: Option<Vec<String>>,
    pub(crate) response_mime_type: Option<String>,
    pub(crate) response_schema: Option<serde_json::Value>,
    pub(crate) candidate_count: Option<i32>,
    pub(crate) max_output_tokens: Option<i32>,
    pub(crate) temperature: Option<f32>,
    pub(crate) top_p: Option<f32>,
    pub(crate) top_k: Option<i32>,
    pub(crate) presence_penalty: Option<f32>,
    pub(crate) frequency_penalty: Option<f32>,
    pub(crate) response_logprobs: Option<bool>,
    pub(crate) logprobs: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SafetySetting {
    pub(crate) category: SafetyCategory,
    pub(crate) threshold: SafetyThreshold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::enum_variant_names)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum SafetyCategory {
    HarmCategoryUnspecified,
    HarmCategoryHateSpeech,
    HarmCategorySexuallyExplicit,
    HarmCategoryDangerousContent,
    HarmCategoryHarassment,
    HarmCategoryCivicIntegrity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum SafetyThreshold {
    HarmBlockThresholdUnspecified,
    BlockLowAndAbove,
    BlockMediumAndAbove,
    BlockOnlyHigh,
    BlockNone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenerateContentResponse {
    pub(crate) candidates: Vec<Candidate>,
    pub(crate) prompt_feedback: Option<PromptFeedback>,
    pub(crate) usage_metadata: Option<UsageMetadata>,
    pub(crate) model_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Candidate {
    pub(crate) content: Content,
    pub(crate) finish_reason: Option<FinishReason>,
    pub(crate) safety_ratings: Option<Vec<SafetyRating>>,
    pub(crate) citation_metadata: Option<CitationMetadata>,
    pub(crate) token_count: Option<i32>,
    pub(crate) grounding_metadata: Option<GroundingMetadata>,
    pub(crate) index: Option<i32>,
    pub(crate) finish_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::enum_variant_names)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum FinishReason {
    FinishReasonUnspecified,
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Other,
    Blocklist,
    ProhibitedContent,
    Spii,
    MalformedFunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SafetyRating {
    pub(crate) category: SafetyCategory,
    pub(crate) probability: SafetyProbability,
    pub(crate) blocked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum SafetyProbability {
    HarmProbabilityUnspecified,
    Negligible,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CitationMetadata {
    pub(crate) citation_sources: Vec<CitationSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CitationSource {
    pub(crate) start_index: Option<i32>,
    pub(crate) end_index: Option<i32>,
    pub(crate) uri: Option<String>,
    pub(crate) license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PromptFeedback {
    pub(crate) block_reason: Option<BlockReason>,
    pub(crate) safety_ratings: Vec<SafetyRating>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::enum_variant_names)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum BlockReason {
    BlockReasonUnspecified,
    Safety,
    Other,
    Blocklist,
    ProhibitedContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UsageMetadata {
    pub(crate) prompt_token_count: i32,
    pub(crate) candidates_token_count: i32,
    pub(crate) total_token_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GroundingMetadata {
    pub(crate) search_entry_point: Option<SearchEntryPoint>,
    pub(crate) grounding_chunks: Option<Vec<GroundingChunk>>,
    pub(crate) grounding_supports: Option<Vec<GroundingSupport>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SearchEntryPoint {
    pub(crate) rendered_content: Option<String>,
    pub(crate) sdk_blob: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GroundingChunk {
    pub(crate) web: Option<WebSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct WebSource {
    pub(crate) uri: String,
    pub(crate) title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GroundingSupport {
    pub(crate) grounding_chunk_indices: Vec<i32>,
    pub(crate) confidence_scores: Vec<f32>,
    pub(crate) segment: Segment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Segment {
    pub(crate) part_index: i32,
    pub(crate) start_index: i32,
    pub(crate) end_index: i32,
    pub(crate) text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum GoogleStreamEvent {
    Response(GenerateContentResponse),
    NotSupported(String),
}
