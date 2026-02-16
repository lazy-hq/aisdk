//! Integration with Vercel's AI SDK UI.

#[cfg(feature = "language-model-request")]
use futures::Stream;
#[cfg(feature = "language-model-request")]
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "language-model-request")]
use uuid;

#[cfg(feature = "language-model-request")]
use crate::core::LanguageModelStreamChunkType;

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
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// End of text message
    #[serde(rename = "text-end")]
    TextEnd {
        /// Message ID
        id: String,
        /// Optional provider metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// Start of reasoning message
    #[serde(rename = "reasoning-start")]
    ReasoningStart {
        /// Message ID
        id: String,
        /// Optional provider metadata
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
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// End of reasoning message
    #[serde(rename = "reasoning-end")]
    ReasoningEnd {
        /// Message ID
        id: String,
        /// Optional provider metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// Start of tool call
    #[serde(rename = "tool-call-start")]
    ToolCallStart {
        /// Message ID
        id: String,
        /// Tool call ID
        tool_call_id: String,
        /// Tool name
        tool_name: String,
        /// Optional provider metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// Delta of tool call
    #[serde(rename = "tool-call-delta")]
    ToolCallDelta {
        /// Message ID
        id: String,
        /// Tool call ID
        tool_call_id: String,
        /// Delta
        delta: String,
        /// Optional provider metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// End of tool call
    #[serde(rename = "tool-call-end")]
    ToolCallEnd {
        /// Message ID
        id: String,
        /// Tool call ID
        tool_call_id: String,
        /// Result
        result: Value,
        /// Optional provider metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<Value>,
    },
    /// Error chunk
    #[serde(rename = "error")]
    Error {
        /// Error text
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

        self.stream.filter_map(move |chunk| {
            let ui_chunk = match chunk {
                LanguageModelStreamChunkType::Start if options.send_start => {
                    Some(VercelUIStream::TextStart {
                        id: message_id.clone(),
                        provider_metadata: None,
                    })
                }

                LanguageModelStreamChunkType::Text(delta) => Some(VercelUIStream::TextDelta {
                    id: message_id.clone(),
                    delta,
                    provider_metadata: None,
                }),

                LanguageModelStreamChunkType::Reasoning(delta) if options.send_reasoning => {
                    Some(VercelUIStream::ReasoningDelta {
                        id: message_id.clone(),
                        delta,
                        provider_metadata: None,
                    })
                }

                LanguageModelStreamChunkType::ToolCall(_json_str) => {
                    //TODO: handle tool call streams when they are supported
                    Some(VercelUIStream::ToolCallStart {
                        id: message_id.clone(),
                        tool_call_id: "unknown".to_string(),
                        tool_name: "unknown".to_string(),
                        provider_metadata: None,
                    })
                }

                LanguageModelStreamChunkType::End(_) if options.send_finish => {
                    Some(VercelUIStream::TextEnd {
                        id: message_id.clone(),
                        provider_metadata: None,
                    })
                }

                LanguageModelStreamChunkType::Failed(error)
                | LanguageModelStreamChunkType::Incomplete(error) => {
                    Some(VercelUIStream::Error { error_text: error })
                }

                // Skip and continue
                LanguageModelStreamChunkType::NotSupported(_) => None,

                //TODO: handle other vercel chunk types
                // Skip and continue
                _ => None,
            };

            futures::future::ready(ui_chunk.map(Ok))
        })
    }
}

/// Represents a part of a UI message from Vercel's useChat hook.
#[derive(Deserialize, Debug, Clone)]
pub struct VercelUIMessagePart {
    /// The text content of the part.
    pub text: String,
    /// The type of the part (e.g., "text").
    #[serde(rename = "type")]
    pub part_type: String, // TODO: can this be an enum?
}

/// Represents a UI message from Vercel's useChat hook.
#[derive(Deserialize, Debug, Clone)]
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
    /// - TODO: Add support for tool parts (e.g., map to `Tool` messages).
    pub fn from_vercel_ui_message(
        ui_messages: &[VercelUIMessage],
    ) -> crate::core::messages::Messages {
        ui_messages
            .iter()
            .filter_map(|msg| {
                let content = msg
                    .parts
                    .iter()
                    .filter(|part| part.part_type == "text")
                    .map(|part| part.text.clone())
                    .collect::<Vec<_>>()
                    .join("");

                match msg.role.as_str() {
                    "system" => Some(crate::core::messages::Message::System(content.into())),
                    "user" => Some(crate::core::messages::Message::User(content.into())),
                    "assistant" => Some(crate::core::messages::Message::Assistant(content.into())),
                    _ => None,
                }
            })
            .collect()
    }
}

/// Converts a VercelUIRequest into native aisdk::core::messages::Message
impl From<VercelUIRequest> for Vec<crate::core::messages::Message> {
    fn from(request: VercelUIRequest) -> Self {
        crate::core::messages::Message::from_vercel_ui_message(&request.messages)
    }
}
