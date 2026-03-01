//! Defines the central `LanguageModel` trait for interacting with text-based AI models.
//!
//! This module provides the `LanguageModel` trait, which establishes the core
//! contract for all language models supported by the SDK. It abstracts the
//! underlying implementation details of different AI providers, offering a
//! unified interface for various operations like text generation or streaming.

#[cfg(feature = "language-model-request")]
pub mod generate_text;
#[cfg(feature = "language-model-request")]
pub mod request;
#[cfg(feature = "language-model-request")]
pub mod stream_text;

use crate::core::messages::{AssistantMessage, TaggedMessage, TaggedMessageHelpers};
use crate::core::tools::ToolList;
use crate::core::{
    Message,
    tools::{ToolCallInfo, ToolResultInfo},
};
use crate::core::{Messages, utils};
use crate::error::{Error, Result};
use async_trait::async_trait;
use derive_builder::Builder;
use futures::Stream;
use schemars::Schema;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Add;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

// ============================================================================
// Section: constants
// ============================================================================

/// Default maximum number of tool calling steps allowed in a single request.
pub const DEFAULT_TOOL_STEP_COUNT: usize = 3;

// ============================================================================
// Section: traits
// ============================================================================

/// The core trait abstracting the capabilities of a language model.
///
/// This trait is the foundation for all text-based AI interactions.
/// Implementors of `LanguageModel` provide the necessary logic to connect to
/// a specific model endpoint and perform operations. The trait is designed to
/// be extensible to support various functionalities, such as single-shot
/// generation and streaming responses.
#[async_trait]
pub trait LanguageModel: Send + Sync + std::fmt::Debug + Clone + 'static {
    /// Returns the name or identifier of the language model.
    ///
    /// This method provides a human-readable name for the model instance,
    /// which can be used for logging, debugging, or display purposes.
    ///
    /// # Returns
    ///
    /// A string representing the model's name (e.g., "gpt-4", "claude-3").
    fn name(&self) -> String;
    /// Performs a single, non-streaming text generation request.
    ///
    /// This method sends a prompt to the model and returns the entire response at once.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the API call fails or the request is invalid.
    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse>;

    /// Performs a streaming text generation request.
    ///
    /// This method sends a prompt to the model and returns a stream of responses.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the API call fails or the request is invalid.
    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream>;
}

// ============================================================================
// Section: hook types
// ============================================================================

/// Type alias for a hook function that determines when to stop generation.
///
/// Returns `true` if generation should stop.
pub type StopWhenHook = Arc<dyn Fn(&LanguageModelOptions) -> bool + Send + Sync>;

/// Type alias for a hook function called before each generation step.
pub type OnStepStartHook = Arc<dyn Fn(&mut LanguageModelOptions) + Send + Sync>;

/// Type alias for a hook function called after each generation step.
pub type OnStepFinishHook = Arc<dyn Fn(&LanguageModelOptions) + Send + Sync>;

// ============================================================================
// Section: structs and impls
// ============================================================================

/// Represents a single step in the language model interaction process.
///
/// A step contains all messages exchanged during one cycle of model interaction,
/// including user input, assistant responses, and tool calls/results.
pub struct Step {
    /// The unique identifier for this step.
    pub step_id: usize,
    /// The messages that occurred during this step.
    pub messages: Messages,
}

impl Step {
    /// Creates a new `Step` with the given ID and messages.
    pub fn new(step_id: usize, messages: Messages) -> Self {
        Self { step_id, messages }
    }

    /// Returns a reference to the messages in this step.
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Calculates the total token usage for this step.
    pub fn usage(&self) -> Usage {
        self.messages()
            .iter()
            .filter_map(|m| match m {
                Message::Assistant(AssistantMessage { usage, .. }) => usage.as_ref(),
                _ => None,
            })
            .fold(Usage::default(), |acc, u| &acc + u)
    }

    /// Returns a vector of all tool calls in the conversation.
    pub fn tool_calls(&self) -> Option<Vec<ToolCallInfo>> {
        let calls: Vec<ToolCallInfo> = self
            .messages()
            .iter()
            .filter_map(|msg| match msg {
                Message::Assistant(AssistantMessage {
                    content: LanguageModelResponseContentType::ToolCall(info),
                    ..
                }) => Some(Some(info.clone())),
                _ => None,
            })
            .flatten()
            .collect();
        if calls.is_empty() { None } else { Some(calls) }
    }

    /// Returns a vector of all tool results in the conversation.
    pub fn tool_results(&self) -> Option<Vec<ToolResultInfo>> {
        let results: Vec<ToolResultInfo> = self
            .messages()
            .iter()
            .filter_map(|msg| match msg {
                Message::Tool(info) => Some(info.clone()),
                _ => None,
            })
            .collect();
        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }
}

// ============================================================================
// Section: options
// ============================================================================

/// Configuration options for language model requests.
///
/// This struct contains all the parameters that can be used to customize
/// text generation, including sampling parameters, tools, and hooks.
#[derive(Clone, Default, Builder)]
#[builder(pattern = "owned", setter(into), build_fn(error = "Error"))]
pub struct LanguageModelOptions {
    /// System prompt to be used for the request.
    pub system: Option<String>,

    /// Output format schema.
    pub schema: Option<Schema>,

    /// The seed (integer) to use for random sampling. If set and supported
    /// by the model, calls will generate deterministic results.
    pub seed: Option<u32>,

    /// Controls randomness in generation (0-100, scaled to 0.0-1.0).
    pub temperature: Option<u32>,

    /// Nucleus sampling parameter (0-100, scaled to 0.0-1.0).
    pub top_p: Option<u32>,

    /// Top-k sampling parameter.
    pub top_k: Option<u32>,

    /// Maximum number of retries for failed requests.
    pub max_retries: Option<u32>,

    /// Maximum number of output tokens to generate.
    pub max_output_tokens: Option<u32>,

    /// Stop sequences.
    /// If set, the model will stop generating text when one of the stop sequences is generated.
    pub stop_sequences: Option<Vec<String>>,

    /// Presence penalty setting. It affects the likelihood of the model to
    /// repeat information that is already in the prompt.
    pub presence_penalty: Option<f32>,

    /// Frequency penalty setting. It affects the likelihood of the model
    /// to repeatedly use the same words or phrases.
    pub frequency_penalty: Option<f32>,

    /// Hook to conditionally stop generation.
    pub stop_when: Option<StopWhenHook>,

    /// Hook called before each generation step.
    pub on_step_start: Option<OnStepStartHook>,

    /// Hook called after each generation step.
    pub on_step_finish: Option<OnStepFinishHook>,

    /// Level of reasoning effort for the model.
    pub reasoning_effort: Option<ReasoningEffort>,

    /// List of tools to use.
    pub(crate) tools: Option<ToolList>,

    /// Current step ID for tracking multi-step interactions.
    pub(crate) current_step_id: usize,

    /// The conversation messages for the request.
    pub(crate) messages: Vec<TaggedMessage>,

    /// The reason why generation stopped.
    pub(crate) stop_reason: Option<StopReason>,
}

impl Debug for LanguageModelOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguageModelOptions")
            .field("system", &self.system)
            .field("messages", &self.messages)
            .field("schema", &self.schema)
            .field("seed", &self.seed)
            .field("temperature", &self.temperature)
            .field("top_p", &self.top_p)
            .field("top_k", &self.top_k)
            .field("max_retries", &self.max_retries)
            .field("max_output_tokens", &self.max_output_tokens)
            .field("stop_sequences", &self.stop_sequences)
            .field("presence_penalty", &self.presence_penalty)
            .field("frequency_penalty", &self.frequency_penalty)
            .field("tools", &self.tools)
            .field("current_step_id", &self.current_step_id)
            .field("stop_when", &self.stop_when.is_some())
            .field("on_step_start", &self.on_step_start.is_some())
            .field("on_step_finish", &self.on_step_finish.is_some())
            .finish()
    }
}

impl LanguageModelOptions {
    /// Creates a new builder for `LanguageModelOptions`.
    pub fn builder() -> LanguageModelOptionsBuilder {
        LanguageModelOptionsBuilder::default()
    }

    /// Returns a vector of all messages in the conversation.
    pub fn messages(&self) -> Messages {
        self.messages.iter().map(|m| m.message.clone()).collect()
    }

    /// Executes a tool call and adds the result to the message history.
    pub(crate) async fn handle_tool_call(&mut self, input: &ToolCallInfo) -> &mut Self {
        if let Some(tools) = &self.tools {
            let tool_result_task = tools.execute(input.clone()).await;
            let tool_result = tool_result_task
                .await
                .map_err(|err| Error::ToolCallError(format!("Error executing tool: {err}")))
                .and_then(|result| result);

            let mut tool_output_infos = Vec::new();

            let mut tool_output_info = ToolResultInfo::new(&input.tool.name);
            let output = match tool_result {
                Ok(result) => serde_json::Value::String(result),
                Err(err) => serde_json::Value::String(format!("Error: {err}")),
            };
            tool_output_info.output(output);
            tool_output_info.id(&input.tool.id);
            tool_output_infos.push(tool_output_info.clone());

            // update messages
            self.messages.push(TaggedMessage::new(
                self.current_step_id,
                Message::Tool(tool_output_info),
            ));

            self
        } else {
            self
        }
    }

    /// Returns the step with the given index, if it exists.
    pub fn step(&self, index: usize) -> Option<Step> {
        let messages: Messages = self
            .messages
            .iter()
            .filter(|t| t.step_id == index)
            .map(|t| t.message.clone())
            .collect();
        if messages.is_empty() {
            None
        } else {
            Some(Step::new(index, messages))
        }
    }

    /// Returns the most recent step, if any.
    pub fn last_step(&self) -> Option<Step> {
        let max_step = self.messages.iter().map(|t| t.step_id).max()?;
        self.step(max_step)
    }

    /// Returns all steps in chronological order.
    pub fn steps(&self) -> Vec<Step> {
        let mut step_map: HashMap<usize, Messages> = HashMap::new();
        for tagged in &self.messages {
            step_map
                .entry(tagged.step_id)
                .or_default()
                .push(tagged.message.clone());
        }
        let mut steps: Vec<Step> = step_map
            .into_iter()
            .map(|(id, msgs)| Step::new(id, msgs))
            .collect();
        steps.sort_by_key(|s| s.step_id);
        steps
    }

    /// Calculates the total token usage across all steps.
    pub fn usage(&self) -> Usage {
        self.steps()
            .iter()
            .map(|s| s.usage())
            .fold(Usage::default(), |acc, u| &acc + &u)
    }

    /// Returns the content of the last assistant message, excluding reasoning.
    pub fn content(&self) -> Option<&LanguageModelResponseContentType> {
        if let Some(msg) = self.messages.last() {
            match msg.message {
                Message::Assistant(ref assistant_msg) => {
                    if let LanguageModelResponseContentType::Reasoning { .. } =
                        assistant_msg.content
                    {
                        None
                    } else {
                        Some(&assistant_msg.content)
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Returns the text content of the last assistant message.
    pub fn text(&self) -> Option<String> {
        if let Some(msg) = self.messages.last() {
            match msg.message {
                Message::Assistant(AssistantMessage {
                    content: LanguageModelResponseContentType::Text(ref text),
                    ..
                }) => Some(text.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Extracts all tool results from the conversation.
    pub fn tool_results(&self) -> Option<Vec<ToolResultInfo>> {
        self.messages.as_slice().extract_tool_results()
    }

    /// Extracts all tool calls from the conversation.
    pub fn tool_calls(&self) -> Option<Vec<ToolCallInfo>> {
        self.messages.as_slice().extract_tool_calls()
    }

    /// Returns the reason why generation stopped.
    pub fn stop_reason(&self) -> Option<StopReason> {
        self.stop_reason.clone()
    }
}

// ============================================================================
// Section: response types
// ============================================================================

/// The different types of content that can be generated by a language model.
#[derive(Debug, Clone)]
pub enum LanguageModelResponseContentType {
    /// Plain text response.
    Text(String),
    /// A tool call request.
    ToolCall(ToolCallInfo),
    /// Reasoning or thinking content.
    Reasoning {
        /// The reasoning/thinking content
        content: String,
        /// Provider-specific extensions
        extensions: crate::extensions::Extensions,
    },
    /// Feature not supported by the provider.
    NotSupported(String),
}

impl Default for LanguageModelResponseContentType {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl From<String> for LanguageModelResponseContentType {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl LanguageModelResponseContentType {
    /// Creates a new text content type.
    pub fn new(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }
}

/// Token usage statistics for a language model operation.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Usage {
    /// Number of input tokens processed.
    pub input_tokens: Option<usize>,
    /// Number of output tokens generated.
    pub output_tokens: Option<usize>,
    /// Number of tokens used for reasoning.
    pub reasoning_tokens: Option<usize>,
    /// Number of cached tokens reused.
    pub cached_tokens: Option<usize>,
}

impl Add for &Usage {
    type Output = Usage;

    fn add(self, rhs: Self) -> Self::Output {
        Usage {
            input_tokens: utils::sum_options(self.input_tokens, rhs.input_tokens),
            output_tokens: utils::sum_options(self.output_tokens, rhs.output_tokens),
            reasoning_tokens: utils::sum_options(self.reasoning_tokens, rhs.reasoning_tokens),
            cached_tokens: utils::sum_options(self.cached_tokens, rhs.cached_tokens),
        }
    }
}

/// Response from a language model.
#[derive(Debug, Clone)]
pub struct LanguageModelResponse {
    /// The generated contents (supports multiple outputs).
    pub contents: Vec<LanguageModelResponseContentType>,

    /// Usage information
    pub usage: Option<Usage>,
}

impl LanguageModelResponse {
    /// Creates a new response with the given text content.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            contents: vec![LanguageModelResponseContentType::new(text.into())],
            usage: None,
        }
    }
}

/// Types of chunks that can be emitted during streaming text generation.
#[derive(Default, Debug, Clone)]
pub enum LanguageModelStreamChunkType {
    /// Indicates the start of generation.
    #[default]
    Start,
    /// A chunk of generated text.
    Text(String),
    /// Reasoning summary text chunk (content delta only)
    Reasoning(String),
    /// Tool call argument chunk
    ToolCall(String),
    /// Successful completion of generation.
    End(AssistantMessage),
    /// Generation failed with an error message.
    Failed(String),
    /// Generation ended with an incomplete response.
    Incomplete(String),
    /// Feature not supported by the provider.
    NotSupported(String),
}

/// A chunk of data from a streaming language model response.
#[derive(Debug, Clone)]
pub enum LanguageModelStreamChunk {
    /// An incremental update during streaming.
    Delta(LanguageModelStreamChunkType),
    /// The final result when streaming is complete.
    Done(AssistantMessage),
}

/// A common interface for stream responses generated by providers (e.g. OpenAI)
pub(crate) type ProviderStream =
    Pin<Box<dyn Stream<Item = Result<Vec<LanguageModelStreamChunk>>> + Send>>;

/// A stream wrapper that provides a channel-based interface for language model streaming.
pub struct LanguageModelStream {
    receiver: UnboundedReceiver<LanguageModelStreamChunkType>,
}

impl LanguageModelStream {
    /// Creates a new stream with an associated sender for streaming chunks.
    ///
    /// This method returns a tuple containing the sender for pushing chunks
    /// and the receiver stream for consuming them.
    ///
    /// # Returns
    ///
    /// A tuple of `(UnboundedSender<LanguageModelStreamChunkType>, LanguageModelStream)`.
    pub fn new() -> (
        UnboundedSender<LanguageModelStreamChunkType>,
        LanguageModelStream,
    ) {
        let (tx, rx) = mpsc::unbounded_channel();
        (tx, LanguageModelStream { receiver: rx })
    }
}

impl Stream for LanguageModelStream {
    type Item = LanguageModelStreamChunkType;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

/// Reasons why text generation might stop.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum StopReason {
    /// Generation completed successfully.
    #[default]
    Finish,
    /// Provider-specific stop reason (e.g., timeout, rate limit).
    Provider(String),
    /// The user has explicitly provided a hook causing to stop
    Hook,
    /// Stopped due to an error.
    Error(Error),
    /// Other unspecified reason.
    Other(String),
}

/// Levels of reasoning effort for language models that support it.
#[derive(Debug, Clone, Copy, Default)]
pub enum ReasoningEffort {
    /// Instant/no reasoning — near-realtime responses.
    Instant,
    /// Low reasoning effort.
    #[default]
    Low,
    /// Medium reasoning effort.
    Medium,
    /// High reasoning effort.
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_add_both_some() {
        let u1 = Usage {
            input_tokens: Some(10),
            output_tokens: Some(20),
            reasoning_tokens: Some(5),
            cached_tokens: Some(2),
        };
        let u2 = Usage {
            input_tokens: Some(15),
            output_tokens: Some(25),
            reasoning_tokens: Some(10),
            cached_tokens: Some(3),
        };
        let result = &u1 + &u2;
        assert_eq!(result.input_tokens, Some(25));
        assert_eq!(result.output_tokens, Some(45));
        assert_eq!(result.reasoning_tokens, Some(15));
        assert_eq!(result.cached_tokens, Some(5));
    }

    #[test]
    fn test_usage_add_first_some_second_none() {
        let u1 = Usage {
            input_tokens: Some(10),
            output_tokens: Some(20),
            reasoning_tokens: Some(5),
            cached_tokens: Some(2),
        };
        let u2 = Usage {
            input_tokens: None,
            output_tokens: None,
            reasoning_tokens: None,
            cached_tokens: None,
        };
        let result = &u1 + &u2;
        assert_eq!(result.input_tokens, Some(10));
        assert_eq!(result.output_tokens, Some(20));
        assert_eq!(result.reasoning_tokens, Some(5));
        assert_eq!(result.cached_tokens, Some(2));
    }

    #[test]
    fn test_usage_add_first_none_second_some() {
        let u1 = Usage {
            input_tokens: None,
            output_tokens: None,
            reasoning_tokens: None,
            cached_tokens: None,
        };
        let u2 = Usage {
            input_tokens: Some(15),
            output_tokens: Some(25),
            reasoning_tokens: Some(10),
            cached_tokens: Some(3),
        };
        let result = &u1 + &u2;
        assert_eq!(result.input_tokens, Some(15));
        assert_eq!(result.output_tokens, Some(25));
        assert_eq!(result.reasoning_tokens, Some(10));
        assert_eq!(result.cached_tokens, Some(3));
    }

    #[test]
    fn test_usage_add_both_none() {
        let u1 = Usage::default();
        let u2 = Usage::default();
        let result = &u1 + &u2;
        assert_eq!(result.input_tokens, None);
        assert_eq!(result.output_tokens, None);
        assert_eq!(result.reasoning_tokens, None);
        assert_eq!(result.cached_tokens, None);
    }

    #[test]
    fn test_usage_add_mixed() {
        let u1 = Usage {
            input_tokens: Some(10),
            output_tokens: None,
            reasoning_tokens: None,
            cached_tokens: Some(2),
        };
        let u2 = Usage {
            input_tokens: None,
            output_tokens: Some(25),
            reasoning_tokens: Some(10),
            cached_tokens: None,
        };
        let result = &u1 + &u2;
        assert_eq!(result.input_tokens, Some(10));
        assert_eq!(result.output_tokens, Some(25));
        assert_eq!(result.reasoning_tokens, Some(10));
        assert_eq!(result.cached_tokens, Some(2));
    }

    #[test]
    fn test_usage_add_zero_values() {
        let u1 = Usage {
            input_tokens: Some(0),
            output_tokens: Some(0),
            reasoning_tokens: Some(0),
            cached_tokens: Some(0),
        };
        let u2 = Usage {
            input_tokens: Some(0),
            output_tokens: Some(0),
            reasoning_tokens: Some(0),
            cached_tokens: Some(0),
        };
        let result = &u1 + &u2;
        assert_eq!(result.input_tokens, Some(0));
        assert_eq!(result.output_tokens, Some(0));
        assert_eq!(result.reasoning_tokens, Some(0));
        assert_eq!(result.cached_tokens, Some(0));
    }

    #[test]
    fn test_step_usage() {
        let messages = vec![
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::Text("Hello".to_string()),
                usage: Some(Usage {
                    input_tokens: Some(10),
                    output_tokens: Some(5),
                    reasoning_tokens: Some(2),
                    cached_tokens: Some(1),
                }),
            }),
            Message::User("Hi".to_string().into()),
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::Text("How are you?".to_string()),
                usage: Some(Usage {
                    input_tokens: Some(5),
                    output_tokens: Some(3),
                    reasoning_tokens: Some(1),
                    cached_tokens: Some(0),
                }),
            }),
        ];
        let step = Step::new(1, messages);
        let usage = step.usage();
        assert_eq!(usage.input_tokens, Some(15));
        assert_eq!(usage.output_tokens, Some(8));
        assert_eq!(usage.reasoning_tokens, Some(3));
        assert_eq!(usage.cached_tokens, Some(1));
    }

    #[test]
    fn test_step_usage_no_assistant() {
        let messages = vec![
            Message::User("Hello".to_string().into()),
            Message::System("System".to_string().into()),
        ];
        let step = Step::new(0, messages);
        let usage = step.usage();
        assert_eq!(usage, Usage::default());
    }

    #[test]
    fn test_step_tool_calls_empty_messages() {
        let step = Step::new(0, vec![]);
        assert_eq!(step.tool_calls(), None);
    }

    #[test]
    fn test_step_tool_calls_only_non_assistant_messages() {
        let messages = vec![
            Message::System("System".to_string().into()),
            Message::User("User".to_string().into()),
            Message::Tool(ToolResultInfo::new("tool1")),
        ];
        let step = Step::new(0, messages);
        assert_eq!(step.tool_calls(), None);
    }

    #[test]
    fn test_step_tool_calls_single_assistant_with_tool_call() {
        let tool_call = ToolCallInfo::new("test_tool");
        let messages = vec![Message::Assistant(AssistantMessage {
            content: LanguageModelResponseContentType::ToolCall(tool_call.clone()),
            usage: None,
        })];
        let step = Step::new(0, messages);
        let calls = step.tool_calls().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].tool.name, "test_tool");
    }

    #[test]
    fn test_step_tool_calls_multiple_assistant_with_tool_calls() {
        let tool_call1 = ToolCallInfo::new("tool1");
        let tool_call2 = ToolCallInfo::new("tool2");
        let messages = vec![
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::ToolCall(tool_call1.clone()),
                usage: None,
            }),
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::ToolCall(tool_call2.clone()),
                usage: None,
            }),
        ];
        let step = Step::new(0, messages);
        let calls = step.tool_calls().unwrap();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].tool.name, "tool1");
        assert_eq!(calls[1].tool.name, "tool2");
    }

    #[test]
    fn test_step_tool_calls_assistant_without_tool_call() {
        let messages = vec![Message::Assistant(AssistantMessage {
            content: LanguageModelResponseContentType::Text("Hello".to_string()),
            usage: None,
        })];
        let step = Step::new(0, messages);
        assert_eq!(step.tool_calls(), None);
    }

    #[test]
    fn test_step_tool_calls_mixed_message_types() {
        let tool_call = ToolCallInfo::new("test_tool");
        let messages = vec![
            Message::System("System".to_string().into()),
            Message::User("User".to_string().into()),
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::ToolCall(tool_call.clone()),
                usage: None,
            }),
            Message::Tool(ToolResultInfo::new("other_tool")),
        ];
        let step = Step::new(0, messages);
        let calls = step.tool_calls().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].tool.name, "test_tool");
    }

    #[test]
    fn test_step_tool_calls_duplicate_tool_calls() {
        let tool_call1 = ToolCallInfo::new("tool1");
        let tool_call2 = ToolCallInfo::new("tool1"); // Same name
        let messages = vec![
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::ToolCall(tool_call1.clone()),
                usage: None,
            }),
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::ToolCall(tool_call2.clone()),
                usage: None,
            }),
        ];
        let step = Step::new(0, messages);
        let calls = step.tool_calls().unwrap();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].tool.name, "tool1");
        assert_eq!(calls[1].tool.name, "tool1");
    }

    #[test]
    fn test_step_tool_results_empty_messages() {
        let step = Step::new(0, vec![]);
        assert!(step.tool_results().is_none());
    }

    #[test]
    fn test_step_tool_results_only_non_tool_messages() {
        let messages = vec![
            Message::System("System".to_string().into()),
            Message::User("User".to_string().into()),
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::Text("Assistant".to_string()),
                usage: None,
            }),
        ];
        let step = Step::new(0, messages);
        assert!(step.tool_results().is_none());
    }

    #[test]
    fn test_step_tool_results_single_tool_message() {
        let tool_result = ToolResultInfo::new("test_tool");
        let messages = vec![Message::Tool(tool_result.clone())];
        let step = Step::new(0, messages);
        let results = step.tool_results().unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].tool.name, "test_tool");
    }

    #[test]
    fn test_step_tool_results_multiple_tool_messages() {
        let tool_result1 = ToolResultInfo::new("tool1");
        let tool_result2 = ToolResultInfo::new("tool2");
        let messages = vec![
            Message::Tool(tool_result1.clone()),
            Message::Tool(tool_result2.clone()),
        ];
        let step = Step::new(0, messages);
        let results = step.tool_results().unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].tool.name, "tool1");
        assert_eq!(results[1].tool.name, "tool2");
    }

    #[test]
    fn test_step_tool_results_mixed_message_types() {
        let tool_result = ToolResultInfo::new("test_tool");
        let messages = vec![
            Message::System("System".to_string().into()),
            Message::User("User".to_string().into()),
            Message::Tool(tool_result.clone()),
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::Text("Assistant".to_string()),
                usage: None,
            }),
        ];
        let step = Step::new(0, messages);
        let results = step.tool_results().unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].tool.name, "test_tool");
    }

    #[test]
    fn test_step_tool_results_no_tool_messages_but_others_present() {
        let messages = vec![
            Message::System("System".to_string().into()),
            Message::User("User".to_string().into()),
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::Text("Assistant".to_string()),
                usage: None,
            }),
        ];
        let step = Step::new(0, messages);
        assert!(step.tool_results().is_none());
    }

    #[test]
    fn test_step_tool_results_duplicate_tool_entries() {
        let tool_result1 = ToolResultInfo::new("tool1");
        let tool_result2 = ToolResultInfo::new("tool1"); // Same name
        let messages = vec![
            Message::Tool(tool_result1.clone()),
            Message::Tool(tool_result2.clone()),
        ];
        let step = Step::new(0, messages);
        let results = step.tool_results().unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].tool.name, "tool1");
        assert_eq!(results[1].tool.name, "tool1");
    }

    #[test]
    fn test_step_tool_results_preserving_original_message_order() {
        let tool_result1 = ToolResultInfo::new("tool1");
        let tool_result2 = ToolResultInfo::new("tool2");
        let tool_result3 = ToolResultInfo::new("tool3");
        let messages = vec![
            Message::System("System".to_string().into()),
            Message::Tool(tool_result1.clone()),
            Message::User("User".to_string().into()),
            Message::Tool(tool_result2.clone()),
            Message::Assistant(AssistantMessage {
                content: LanguageModelResponseContentType::Text("Assistant".to_string()),
                usage: None,
            }),
            Message::Tool(tool_result3.clone()),
        ];
        let step = Step::new(0, messages);
        let results = step.tool_results().unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].tool.name, "tool1");
        assert_eq!(results[1].tool.name, "tool2");
        assert_eq!(results[2].tool.name, "tool3");
    }

    #[test]
    fn test_step_tool_results_large_number_of_messages() {
        let mut messages = Vec::new();
        // Add 1000 messages with tool results interspersed
        for i in 0..1000 {
            messages.push(Message::Tool(ToolResultInfo::new(format!("tool{i}"))));
            if i % 100 == 0 {
                messages.push(Message::User(format!("User message {i}").into()));
            }
        }
        let step = Step::new(0, messages);
        let results = step.tool_results().unwrap();
        assert_eq!(results.len(), 1000);
        for (i, result) in results.iter().enumerate() {
            assert_eq!(result.tool.name, format!("tool{i}"));
        }
    }
}
