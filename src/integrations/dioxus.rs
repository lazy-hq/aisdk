//! Integration with Dioxus. WIP

use crate::integrations::vercel_aisdk_ui::VercelUIMessage;

/// The lifecycle status of a [`Chat`] request, mirroring Vercel AI SDK v5's `status` field.
///
/// The `Error` variant carries the error message, removing the need for a separate
/// `error` field on [`Chat`].
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ChatStatus {
    /// No request in flight; the hook is idle.
    #[default]
    Ready,
    /// Request has been submitted and is waiting for the first chunk.
    Submitted,
    /// Response chunks are actively being streamed.
    Streaming,
    /// The last request ended in an error; the message describes the failure.
    Error(String),
}

/// Client-side chat state for use with the `use_chat` hook.
///
/// Mirrors the shape of Vercel AI SDK v5's `Chat` instance returned by `useChat`,
/// holding all state required to display and drive a streaming chat UI.
///
/// # Construction
///
/// Prefer [`ChatBuilder`] for type-safe construction with a provider model type.
/// Use [`Chat::new`] when you already have a resolved model name string, or
/// [`Chat::default`] for a zero-config starting point (e.g. as a Dioxus signal
/// initializer).
///
/// ```rust,ignore
/// use aisdk::integrations::dioxus::{Chat, ChatBuilder};
/// use aisdk::core::capabilities::DynamicModel;
///
/// // From a resolved string
/// let chat = Chat::new("gpt-4o", "/api/chat");
///
/// // Default — empty model name, empty endpoint
/// let chat = Chat::default();
///
/// // Builder with a typed model (model name derived from M::MODEL_NAME)
/// # #[cfg(feature = "openai")]
/// let chat = ChatBuilder::new(aisdk::providers::openai::OpenAI::gpt_4o(), "/api/chat")
///     .id("my-session")
///     .build();
///
/// // Builder with DynamicModel — must call .model_name() before .build()
/// let chat = ChatBuilder::new(DynamicModel, "/api/chat")
///     .model_name("gpt-4o")
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct Chat {
    /// Unique identifier for this chat session, sent in every request body.
    pub id: String,

    /// Conversation history in Vercel `UIMessage` format.
    pub messages: Vec<VercelUIMessage>,

    /// The resolved model name string sent to the API (e.g. `"gpt-4o"`).
    pub model_name: String,

    /// Backend API endpoint to POST chat requests to.
    pub api: String,

    /// Current request lifecycle status.
    ///
    /// Replaces the `isLoading: bool` from Vercel AI SDK v4. The
    /// [`ChatStatus::Error`] variant carries the error message inline.
    pub status: ChatStatus,
}

impl Chat {
    /// Creates a new [`Chat`] from an already-resolved model name string and
    /// API endpoint.
    ///
    /// A random UUID is generated for the session `id`. Message history is
    /// empty and status is [`ChatStatus::Ready`].
    ///
    /// Prefer [`ChatBuilder`] when constructing from a typed provider model so
    /// the model name is derived at compile time via [`ModelName::MODEL_NAME`].
    ///
    /// [`ModelName::MODEL_NAME`]: crate::core::capabilities::ModelName::MODEL_NAME
    pub fn new(model_name: impl Into<String>, api: impl Into<String>) -> Self {
        Self {
            id: format!("chat_{}", uuid::Uuid::new_v4().simple()),
            messages: Vec::new(),
            model_name: model_name.into(),
            api: api.into(),
            status: ChatStatus::Ready,
        }
    }
}
