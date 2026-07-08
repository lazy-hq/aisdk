//! Integration with Vercel's AI SDK UI.

#[cfg(feature = "language-model-request")]
use futures::Stream;
#[cfg(feature = "language-model-request")]
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "language-model-request")]
use std::collections::HashMap;
#[cfg(feature = "language-model-request")]
use uuid;

#[cfg(feature = "language-model-request")]
use crate::core::LanguageModelStreamChunkType;
#[cfg(feature = "language-model-request")]
use crate::error::Error;

/// Vercel's ai-sdk UI message chunk types.
/// These represent the JSON chunks sent over SSE to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum VercelUIStream {
    /// Start of text message
    #[serde(rename = "text-start")]
    TextStart {
        /// Message ID
        id: String,
        /// Optional provider metadata
        #[serde(rename = "providerMetadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// Delta of text message
    #[serde(rename = "text-delta")]
    TextDelta {
        /// Message ID
        id: String,
        /// Text delta
        delta: String,
        /// Optional provider metadata
        #[serde(rename = "providerMetadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// End of text message
    #[serde(rename = "text-end")]
    TextEnd {
        /// Message ID
        id: String,
        /// Optional provider metadata
        #[serde(rename = "providerMetadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// Start of reasoning message
    #[serde(rename = "reasoning-start")]
    ReasoningStart {
        /// Message ID
        id: String,
        /// Optional provider metadata
        #[serde(rename = "providerMetadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// Delta of reasoning message
    #[serde(rename = "reasoning-delta")]
    ReasoningDelta {
        /// Message ID
        id: String,
        /// Reasoning delta
        delta: String,
        /// Optional provider metadata
        #[serde(rename = "providerMetadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// End of reasoning message
    #[serde(rename = "reasoning-end")]
    ReasoningEnd {
        /// Message ID
        id: String,
        /// Optional provider metadata
        #[serde(rename = "providerMetadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// Start of tool input
    #[serde(rename = "tool-input-start")]
    ToolInputStart {
        /// Tool call ID
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        /// Tool name
        #[serde(rename = "toolName")]
        tool_name: String,
        /// Whether the tool was already executed on the provider/server side
        #[serde(rename = "providerExecuted")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_executed: Option<bool>,
        /// Optional provider metadata
        #[serde(rename = "providerMetadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
        /// Whether this is a dynamic tool
        #[serde(skip_serializing_if = "Option::is_none")]
        dynamic: Option<bool>,
        /// Optional UI title for the tool invocation
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    /// Delta of tool input
    #[serde(rename = "tool-input-delta")]
    ToolInputDelta {
        /// Tool call ID
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        /// Incremental tool input JSON text
        #[serde(rename = "inputTextDelta")]
        input_text_delta: String,
    },
    /// Tool input fully available
    #[serde(rename = "tool-input-available")]
    ToolInputAvailable {
        /// Tool call ID
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        /// Tool name
        #[serde(rename = "toolName")]
        tool_name: String,
        /// Parsed tool input
        input: Value,
        /// Whether the tool was already executed on the provider/server side
        #[serde(rename = "providerExecuted")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_executed: Option<bool>,
        /// Optional provider metadata
        #[serde(rename = "providerMetadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
        /// Whether this is a dynamic tool
        #[serde(skip_serializing_if = "Option::is_none")]
        dynamic: Option<bool>,
        /// Optional UI title for the tool invocation
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    /// Tool input parsing failed
    #[serde(rename = "tool-input-error")]
    ToolInputError {
        /// Tool call ID
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        /// Tool name
        #[serde(rename = "toolName")]
        tool_name: String,
        /// Raw tool input payload
        input: Value,
        /// Whether the tool was already executed on the provider/server side
        #[serde(rename = "providerExecuted")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_executed: Option<bool>,
        /// Optional provider metadata
        #[serde(rename = "providerMetadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
        /// Whether this is a dynamic tool
        #[serde(skip_serializing_if = "Option::is_none")]
        dynamic: Option<bool>,
        /// Error text
        #[serde(rename = "errorText")]
        error_text: String,
        /// Optional UI title for the tool invocation
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    /// Tool output available
    #[serde(rename = "tool-output-available")]
    ToolOutputAvailable {
        /// Tool call ID
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        /// Tool output/result
        output: Value,
        /// Whether the tool was already executed on the provider/server side
        #[serde(rename = "providerExecuted")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_executed: Option<bool>,
        /// Whether this is a dynamic tool
        #[serde(skip_serializing_if = "Option::is_none")]
        dynamic: Option<bool>,
        /// Whether this is a preliminary tool result
        #[serde(skip_serializing_if = "Option::is_none")]
        preliminary: Option<bool>,
    },
    /// Tool output failed
    #[serde(rename = "tool-output-error")]
    ToolOutputError {
        /// Tool call ID
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        /// Error text
        #[serde(rename = "errorText")]
        error_text: String,
        /// Whether the tool was already executed on the provider/server side
        #[serde(rename = "providerExecuted")]
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_executed: Option<bool>,
        /// Whether this is a dynamic tool
        #[serde(skip_serializing_if = "Option::is_none")]
        dynamic: Option<bool>,
    },
    /// Error chunk
    #[serde(rename = "error")]
    Error {
        /// Error text
        #[serde(rename = "errorText")]
        error_text: String,
    },
    /// Not supported chunk by aisdk.rs
    #[serde(rename = "not-supported")]
    NotSupported {
        /// Error text
        error_text: String,
    },
    // TODO: init - Add additional vercel UI chunks for data parts, sources, etc.
    // as needed for full compatibility
}

#[derive(Default)]
/// Configuration for vercel UI message stream.
pub struct VercelUIStreamOptions {
    /// Whether to send reasoning chunks
    pub send_reasoning: bool,
    /// Whether to send sources (TODO: uncomment when sources are supported)
    //pub send_sources: bool,
    /// Whether to send start chunks
    pub send_start: bool,
    /// Whether to send finish chunks
    pub send_finish: bool,
    /// Custom message ID generator
    pub generate_message_id: Option<Box<VercelUIStreamIdGenerator>>,
}

/// Type alias for custom message ID generator functions.
pub type VercelUIStreamIdGenerator = dyn Fn() -> String + Send + Sync;

/// Builder for vercel UI message stream with fluent API, context, and build closure.
pub struct VercelUIStreamBuilder<C, T> {
    /// Context for the builder. eg. StreamTextResponse
    pub context: C,

    /// Configuration for the Vercel UI message stream.
    pub options: VercelUIStreamOptions,

    /// Build function that creates the final stream response. (implemented by the framework e.g. axum, actix)
    /// where T is the type of the stream response.
    build_fn: Box<dyn Fn(C, VercelUIStreamOptions) -> T + Send + Sync>,
}

impl<C, T> VercelUIStreamBuilder<C, T> {
    /// Creates a new `VercelUIStreamBuilder` with the provided context and build function.
    ///
    /// Initializes the builder with default options, allowing further configuration via fluent methods
    /// before building the final response.
    ///
    /// # Parameters
    /// - `context`: The context object (e.g., `StreamTextResponse`) to be used in the build process.
    /// - `build_fn`: A closure that takes the context and options to produce the final output. implemented by the framework e.g. axum, actix)
    ///
    /// # Returns
    /// A new `VercelUIStreamBuilder` instance ready for configuration.
    pub fn new<B>(context: C, build_fn: B) -> Self
    where
        B: Fn(C, VercelUIStreamOptions) -> T + Send + Sync + 'static,
    {
        Self {
            context,
            options: VercelUIStreamOptions::default(),
            build_fn: Box::new(build_fn),
        }
    }

    /// Enable sending reasoning chunks.
    pub fn send_reasoning(mut self) -> Self {
        self.options.send_reasoning = true;
        self
    }

    /// Enable sending start chunks.
    pub fn send_start(mut self) -> Self {
        self.options.send_start = true;
        self
    }

    /// Enable sending finish chunks.
    pub fn send_finish(mut self) -> Self {
        self.options.send_finish = true;
        self
    }

    /// Set a custom message ID generator.
    pub fn with_id_generator<G>(mut self, generator: G) -> Self
    where
        G: Fn() -> String + Send + Sync + 'static,
    {
        self.options.generate_message_id = Some(Box::new(generator));
        self
    }

    /// Build the final response using the configured options.
    pub fn build(self) -> T {
        (self.build_fn)(self.context, self.options)
    }
}

#[cfg(feature = "language-model-request")]
impl crate::core::StreamTextResponse {
    /// Converts this `StreamTextResponse` into a stream of `VercelUIStream` chunks.
    ///
    /// Transforms the underlying language model stream into Vercel-compatible UI chunks (e.g., text deltas,
    /// reasoning deltas), enabling streaming of the language model output to a frontend using Vercel's ai-sdk-ui.
    ///
    /// # Parameters
    /// - `options`: Configuration options controlling streaming behavior (e.g., enabling reasoning chunks).
    ///
    /// # Returns
    /// A stream yielding `VercelUIStream` items or errors.
    pub fn into_vercel_ui_stream(
        self,
        options: VercelUIStreamOptions,
    ) -> impl Stream<Item = crate::Result<VercelUIStream>> {
        let message_id = options
            .generate_message_id
            .as_ref()
            .map(|f| f())
            .unwrap_or_else(|| format!("msg_{}", uuid::Uuid::new_v4().simple()));

        let mut pending_tool_inputs: HashMap<String, String> = HashMap::new();

        self.stream.flat_map(move |chunk| {
            let ui_chunks = map_language_model_chunk_to_vercel_ui(
                chunk,
                &message_id,
                &options,
                &mut pending_tool_inputs,
            );

            futures::stream::iter(ui_chunks.into_iter().map(Ok))
        })
    }
}

#[cfg(feature = "language-model-request")]
fn map_language_model_chunk_to_vercel_ui(
    chunk: LanguageModelStreamChunkType,
    message_id: &str,
    options: &VercelUIStreamOptions,
    pending_tool_inputs: &mut HashMap<String, String>,
) -> Vec<VercelUIStream> {
    match chunk {
        LanguageModelStreamChunkType::TextStart => {
            if options.send_start {
                vec![VercelUIStream::TextStart {
                    id: message_id.to_string(),
                    provider_metadata: None,
                }]
            } else {
                Vec::new()
            }
        }

        LanguageModelStreamChunkType::TextDelta(delta) => vec![VercelUIStream::TextDelta {
            id: message_id.to_string(),
            delta,
            provider_metadata: None,
        }],

        LanguageModelStreamChunkType::TextEnd => {
            if options.send_finish {
                vec![VercelUIStream::TextEnd {
                    id: message_id.to_string(),
                    provider_metadata: None,
                }]
            } else {
                Vec::new()
            }
        }

        LanguageModelStreamChunkType::ReasoningStart => {
            if options.send_reasoning && options.send_start {
                vec![VercelUIStream::ReasoningStart {
                    id: message_id.to_string(),
                    provider_metadata: None,
                }]
            } else {
                Vec::new()
            }
        }

        LanguageModelStreamChunkType::ReasoningDelta(delta) => {
            if options.send_reasoning {
                vec![VercelUIStream::ReasoningDelta {
                    id: message_id.to_string(),
                    delta,
                    provider_metadata: None,
                }]
            } else {
                Vec::new()
            }
        }

        LanguageModelStreamChunkType::ReasoningEnd => {
            if options.send_reasoning && options.send_finish {
                vec![VercelUIStream::ReasoningEnd {
                    id: message_id.to_string(),
                    provider_metadata: None,
                }]
            } else {
                Vec::new()
            }
        }

        LanguageModelStreamChunkType::ToolCallStart(tool_call) => {
            pending_tool_inputs.insert(tool_call.id.clone(), String::new());

            vec![VercelUIStream::ToolInputStart {
                tool_call_id: tool_call.id,
                tool_name: tool_call.name,
                provider_executed: Some(true),
                provider_metadata: None,
                dynamic: None,
                title: None,
            }]
        }

        LanguageModelStreamChunkType::ToolCallDelta { id, delta } => {
            pending_tool_inputs
                .entry(id.clone())
                .or_default()
                .push_str(&delta);

            vec![VercelUIStream::ToolInputDelta {
                tool_call_id: id,
                input_text_delta: delta,
            }]
        }

        LanguageModelStreamChunkType::ToolCallAvailable(tool_call) => {
            pending_tool_inputs.remove(&tool_call.tool.id);

            vec![VercelUIStream::ToolInputAvailable {
                tool_call_id: tool_call.tool.id,
                tool_name: tool_call.tool.name,
                input: tool_call.input,
                provider_executed: Some(true),
                provider_metadata: None,
                dynamic: None,
                title: None,
            }]
        }

        LanguageModelStreamChunkType::ToolCallEnd(result_info) => {
            let tool_call_id = result_info.tool.id.clone();
            pending_tool_inputs.remove(&tool_call_id);

            let output_chunk = match result_info.output {
                Ok(output) => VercelUIStream::ToolOutputAvailable {
                    tool_call_id,
                    output,
                    provider_executed: Some(true),
                    dynamic: None,
                    preliminary: None,
                },
                Err(error) => VercelUIStream::ToolOutputError {
                    tool_call_id,
                    error_text: format_tool_error_text(error),
                    provider_executed: Some(true),
                    dynamic: None,
                },
            };

            vec![output_chunk]
        }

        LanguageModelStreamChunkType::Failed(error)
        | LanguageModelStreamChunkType::Incomplete(error)
        | LanguageModelStreamChunkType::NotSupported(error) => {
            vec![VercelUIStream::Error { error_text: error }]
        }
    }
}

#[cfg(feature = "language-model-request")]
fn format_tool_error_text(error: Error) -> String {
    match error {
        Error::MissingField(message)
        | Error::InvalidInput(message)
        | Error::ToolCallError(message)
        | Error::PromptError(message)
        | Error::Other(message) => message,
        Error::ApiError { details, .. } => details,
        Error::ProviderError(error) => error.to_string(),
    }
}

/// Represents a part of a UI message from Vercel's useChat hook.
#[derive(Deserialize, Debug)]
pub struct VercelUIMessagePart {
    /// The text content of the part.
    #[serde(default)]
    pub text: Option<String>,
    /// The type of the part (e.g., "text").
    #[serde(rename = "type")]
    pub part_type: String,
    /// Tool call identifier for tool parts.
    #[serde(rename = "toolCallId")]
    #[serde(default)]
    pub tool_call_id: Option<String>,
    /// Tool execution state for tool parts.
    #[serde(default)]
    pub state: Option<String>,
    /// Tool input payload.
    #[serde(default)]
    pub input: Option<Value>,
    /// Tool output payload.
    #[serde(default)]
    pub output: Option<Value>,
    /// Raw tool input for error cases.
    #[serde(rename = "rawInput")]
    #[serde(default)]
    pub raw_input: Option<Value>,
    /// Tool error text for error states.
    #[serde(rename = "errorText")]
    #[serde(default)]
    pub error_text: Option<String>,
    /// Tool name for dynamic tool parts.
    #[serde(rename = "toolName")]
    #[serde(default)]
    pub tool_name: Option<String>,
}

/// Represents a UI message from Vercel's useChat hook.
#[derive(Deserialize, Debug)]
pub struct VercelUIMessage {
    /// Unique identifier for the message.
    pub id: String,
    /// Role of the message sender ("user", "assistant", "system").
    pub role: String,
    /// Array of message parts (e.g., text content).
    pub parts: Vec<VercelUIMessagePart>,
}

/// Represents a request body from Vercel's useChat hook.
#[derive(Deserialize, Debug)]
pub struct VercelUIRequest {
    /// Unique identifier for the chat session.
    pub id: String,
    /// Array of UI messages from the frontend.
    pub messages: Vec<VercelUIMessage>,
    /// Trigger indicating the action (e.g., "submit-message").
    pub trigger: String,
}

impl crate::core::Message {
    /// Converts a slice of Vercel UI messages to the `aisdk::core::Message` format.
    ///
    /// This function extracts text content from UI message parts and maps roles to the
    /// corresponding `Message` variants. Currently only "text" parts are supported; other part types
    /// (e.g., files, tools) are ignored.
    ///
    /// # Parameters
    /// - `ui_messages`: A slice of `VercelUIMessage` to convert.
    ///
    /// # Returns
    /// A vector of `Message` instances.
    ///
    /// # Notes
    /// - Joins multiple text parts into a single string.
    /// - TODO: Add support for file parts (e.g., map to URLs in content).
    pub fn from_vercel_ui_message(
        ui_messages: &[VercelUIMessage],
    ) -> crate::core::messages::Messages {
        ui_messages
            .iter()
            .flat_map(|msg| match msg.role.as_str() {
                "system" => {
                    let content = msg
                        .parts
                        .iter()
                        .filter(|part| part.part_type == "text")
                        .filter_map(|part| part.text.clone())
                        .collect::<Vec<_>>()
                        .join("");

                    if content.is_empty() {
                        Vec::new()
                    } else {
                        vec![crate::core::messages::Message::System(content.into())]
                    }
                }
                "user" => {
                    let content = msg
                        .parts
                        .iter()
                        .filter(|part| part.part_type == "text")
                        .filter_map(|part| part.text.clone())
                        .collect::<Vec<_>>()
                        .join("");

                    if content.is_empty() {
                        Vec::new()
                    } else {
                        vec![crate::core::messages::Message::User(content.into())]
                    }
                }
                "assistant" => msg
                    .parts
                    .iter()
                    .flat_map(VercelUIMessagePart::to_core_messages)
                    .collect::<Vec<_>>(),
                _ => Vec::new(),
            })
            .collect()
    }
}

impl VercelUIMessagePart {
    fn to_core_messages(&self) -> Vec<crate::core::messages::Message> {
        if self.part_type == "text" {
            return self
                .text
                .as_ref()
                .filter(|text| !text.is_empty())
                .map(|text| {
                    vec![crate::core::messages::Message::Assistant(
                        text.clone().into(),
                    )]
                })
                .unwrap_or_default();
        }

        if !self.part_type.starts_with("tool-") && self.part_type != "dynamic-tool" {
            return Vec::new();
        }

        let Some(tool_call_id) = self.tool_call_id.clone() else {
            return Vec::new();
        };

        let Some(state) = self.state.as_deref() else {
            return Vec::new();
        };

        if state == "input-streaming" {
            return Vec::new();
        }

        let tool_name = self.tool_name();
        let input = self
            .input
            .clone()
            .or_else(|| self.raw_input.clone())
            .unwrap_or(Value::Null);

        let mut tool_call = crate::core::ToolCallInfo::new(tool_name.clone());
        tool_call.id(tool_call_id.clone());
        tool_call.input(input.clone());

        let mut messages = vec![crate::core::messages::Message::Assistant(
            crate::core::messages::AssistantMessage::new(
                crate::core::language_model::LanguageModelResponseContentType::ToolCall(tool_call),
                None,
            ),
        )];

        match state {
            "input-available" | "approval-requested" | "approval-responded" => messages,
            "output-available" => {
                let mut result = crate::core::ToolResultInfo::new(tool_name);
                result.id(tool_call_id);
                result.output(self.output.clone().unwrap_or(Value::Null));
                messages.push(crate::core::messages::Message::Tool(result));
                messages
            }
            "output-error" => {
                let mut result = crate::core::ToolResultInfo::new(tool_name);
                result.id(tool_call_id);
                result.output(self.error_output_value());
                messages.push(crate::core::messages::Message::Tool(result));
                messages
            }
            "output-denied" => {
                let mut result = crate::core::ToolResultInfo::new(tool_name);
                result.id(tool_call_id);
                result.output(Value::String(
                    self.error_text
                        .clone()
                        .unwrap_or_else(|| "Tool execution denied.".to_string()),
                ));
                messages.push(crate::core::messages::Message::Tool(result));
                messages
            }
            _ => Vec::new(),
        }
    }

    fn tool_name(&self) -> String {
        if let Some(tool_name) = &self.tool_name {
            return tool_name.clone();
        }

        self.part_type
            .strip_prefix("tool-")
            .unwrap_or(&self.part_type)
            .to_string()
    }

    fn error_output_value(&self) -> Value {
        self.error_text
            .clone()
            .map(Value::String)
            .unwrap_or(Value::Null)
    }
}

/// Converts a VercelUIRequest into native aisdk::core::messages::Message
impl From<VercelUIRequest> for Vec<crate::core::messages::Message> {
    fn from(request: VercelUIRequest) -> Self {
        crate::core::messages::Message::from_vercel_ui_message(&request.messages)
    }
}

#[cfg(all(test, feature = "language-model-request"))]
mod tests {
    use super::*;
    use crate::core::LanguageModelStreamChunkType;
    use crate::core::messages::Message;
    use crate::core::tools::{ToolCallInfo, ToolDetails, ToolResultInfo};
    use crate::error::Error;
    use serde_json::json;

    #[test]
    fn serializes_tool_stream_chunks_with_current_ai_sdk_protocol() {
        let options = VercelUIStreamOptions::default();
        let mut pending_tool_inputs = HashMap::new();

        let start_chunks = map_language_model_chunk_to_vercel_ui(
            LanguageModelStreamChunkType::ToolCallStart(ToolDetails {
                id: "call_1".to_string(),
                name: "get_weather".to_string(),
            }),
            "msg_1",
            &options,
            &mut pending_tool_inputs,
        );
        let delta_chunks = map_language_model_chunk_to_vercel_ui(
            LanguageModelStreamChunkType::ToolCallDelta {
                id: "call_1".to_string(),
                delta: "{\"location\":\"dc\"}".to_string(),
            },
            "msg_1",
            &options,
            &mut pending_tool_inputs,
        );
        let available_chunks = map_language_model_chunk_to_vercel_ui(
            LanguageModelStreamChunkType::ToolCallAvailable(ToolCallInfo {
                tool: ToolDetails {
                    id: "call_1".to_string(),
                    name: "get_weather".to_string(),
                },
                input: json!({
                    "location": "dc",
                }),
                extensions: Default::default(),
            }),
            "msg_1",
            &options,
            &mut pending_tool_inputs,
        );

        let mut result_info = ToolResultInfo::new("get_weather");
        result_info.id("call_1");
        result_info.output(json!("The weather in dc is sunny"));

        let end_chunks = map_language_model_chunk_to_vercel_ui(
            LanguageModelStreamChunkType::ToolCallEnd(result_info),
            "msg_1",
            &options,
            &mut pending_tool_inputs,
        );

        assert_eq!(
            serde_json::to_value(&start_chunks[0]).unwrap(),
            json!({
                "type": "tool-input-start",
                "toolCallId": "call_1",
                "toolName": "get_weather",
                "providerExecuted": true,
            })
        );
        assert_eq!(
            serde_json::to_value(&delta_chunks[0]).unwrap(),
            json!({
                "type": "tool-input-delta",
                "toolCallId": "call_1",
                "inputTextDelta": "{\"location\":\"dc\"}",
            })
        );
        assert_eq!(
            serde_json::to_value(&available_chunks[0]).unwrap(),
            json!({
                "type": "tool-input-available",
                "toolCallId": "call_1",
                "toolName": "get_weather",
                "input": {
                    "location": "dc",
                },
                "providerExecuted": true,
            })
        );
        assert_eq!(
            serde_json::to_value(&end_chunks[0]).unwrap(),
            json!({
                "type": "tool-output-available",
                "toolCallId": "call_1",
                "output": "The weather in dc is sunny",
                "providerExecuted": true,
            })
        );
    }

    #[test]
    fn serializes_tool_output_errors_with_error_text() {
        let options = VercelUIStreamOptions::default();
        let mut pending_tool_inputs =
            HashMap::from([("call_2".to_string(), "{\"location\":\"dc\"}".to_string())]);

        let result_info = ToolResultInfo {
            tool: ToolDetails {
                id: "call_2".to_string(),
                name: "get_weather".to_string(),
            },
            output: Err(Error::Other("weather service unavailable".to_string())),
        };

        let chunks = map_language_model_chunk_to_vercel_ui(
            LanguageModelStreamChunkType::ToolCallEnd(result_info),
            "msg_1",
            &options,
            &mut pending_tool_inputs,
        );

        assert_eq!(
            serde_json::to_value(&chunks[0]).unwrap(),
            json!({
                "type": "tool-output-error",
                "toolCallId": "call_2",
                "errorText": "weather service unavailable",
                "providerExecuted": true,
            })
        );
    }

    #[test]
    fn deserializes_assistant_tool_parts_without_text_field() {
        let request: VercelUIRequest = serde_json::from_value(json!({
            "id": "chat_1",
            "trigger": "submit-message",
            "messages": [
                {
                    "id": "assistant_1",
                    "role": "assistant",
                    "parts": [
                        {
                            "type": "tool-get_weather",
                            "toolCallId": "call_1",
                            "state": "output-available",
                            "input": { "location": "dc" },
                            "output": "The weather in dc is sunny",
                            "providerExecuted": true
                        },
                        {
                            "type": "text",
                            "text": "The weather in dc is sunny."
                        }
                    ]
                }
            ]
        }))
        .expect("request should deserialize");

        let messages: Vec<Message> = request.into();

        assert_eq!(messages.len(), 3);
        assert!(matches!(
            &messages[0],
            Message::Assistant(assistant)
                if matches!(
                    &assistant.content,
                    crate::core::language_model::LanguageModelResponseContentType::ToolCall(tool_call)
                        if tool_call.tool.id == "call_1"
                        && tool_call.tool.name == "get_weather"
                        && tool_call.input == json!({ "location": "dc" })
                )
        ));
        assert!(matches!(
            &messages[1],
            Message::Tool(tool_result)
                if tool_result.tool.id == "call_1"
                && tool_result.tool.name == "get_weather"
                && tool_result.output.as_ref().ok() == Some(&json!("The weather in dc is sunny"))
        ));
        assert!(matches!(
            &messages[2],
            Message::Assistant(assistant)
                if matches!(
                    &assistant.content,
                    crate::core::language_model::LanguageModelResponseContentType::Text(text)
                        if text == "The weather in dc is sunny."
                )
        ));
    }
}
