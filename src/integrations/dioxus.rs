//! Integration with Dioxus. WIP

use crate::core::capabilities::ModelName;
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

// ---------------------------------------------------------------------------
// Type-state markers for ChatBuilder
// ---------------------------------------------------------------------------

/// Sealed trait for [`ChatBuilder`] model-name states.
mod builder_state {
    pub trait BuilderState: private::Sealed {}
    mod private {
        pub trait Sealed {}
    }
    /// State indicating the model name has not yet been provided.
    pub struct WithoutName;
    impl private::Sealed for WithoutName {}
    impl BuilderState for WithoutName {}

    /// State indicating the model name has been set and [`super::ChatBuilder::build`]
    /// may be called.
    pub struct WithName;
    impl private::Sealed for WithName {}
    impl BuilderState for WithName {}
}

use builder_state::BuilderState;
pub use builder_state::{WithName, WithoutName};

// ---------------------------------------------------------------------------
// ChatBuilder — type-state builder
// ---------------------------------------------------------------------------

/// Type-safe builder for [`Chat`].
///
/// The two type parameters are:
/// - `M` — a [`ModelName`] type (e.g. `OpenAI<Gpt4o>`, [`DynamicModel`]). The
///   model name string is derived from [`ModelName::MODEL_NAME`] unless
///   overridden via [`.model_name()`][ChatBuilder::model_name].
/// - `S` — the builder state, either [`WithoutName`] or [`WithName`]. This
///   parameter is an implementation detail; you will typically not need to name
///   it explicitly.
///
/// # State transitions
///
/// ```text
/// ChatBuilder::<M, WithoutName>::new(model, api)
///     │
///     ├─ .id() / .messages() / .status()   (available in both states)
///     │
///     ├─ .build()   (available in WithoutName — panics at runtime if
///     │              M::MODEL_NAME is empty, i.e. DynamicModel without a name)
///     │
///     └─ .model_name(name)   (available in WithoutName for any M)
///             │
///             └─ ChatBuilder<M, WithName>
///                     │
///                     └─ .build()   (infallible — model name is guaranteed set)
/// ```
///
/// [`DynamicModel`]: crate::core::capabilities::DynamicModel
pub struct ChatBuilder<M: ModelName, S: BuilderState = WithoutName> {
    id: Option<String>,
    messages: Vec<VercelUIMessage>,
    model_name: String,
    api: String,
    status: ChatStatus,
    _model: std::marker::PhantomData<M>,
    _state: std::marker::PhantomData<S>,
}

impl<M: ModelName> ChatBuilder<M, WithoutName> {
    /// Creates a new [`ChatBuilder`] in the [`WithoutName`] state.
    ///
    /// The model name is initialised from `M::MODEL_NAME`. For concrete model
    /// types (e.g. `OpenAI<Gpt4o>`) this is the correct API string; for
    /// [`DynamicModel`] it is `""` — call [`.model_name()`][ChatBuilder::model_name]
    /// before [`.build()`][ChatBuilder::build] in that case.
    ///
    /// [`DynamicModel`]: crate::core::capabilities::DynamicModel
    pub fn new(model: M, api: impl Into<String>) -> Self {
        // `model` is consumed only to bind `M` at the call site; the name is
        // read from the associated const rather than from the value itself.
        let _ = model;
        Self {
            id: None,
            messages: Vec::new(),
            model_name: M::MODEL_NAME.to_string(),
            api: api.into(),
            status: ChatStatus::Ready,
            _model: std::marker::PhantomData,
            _state: std::marker::PhantomData,
        }
    }

    /// Builds the [`Chat`], deriving the model name from `M::MODEL_NAME`.
    ///
    /// # Panics
    ///
    /// Panics at runtime if `M::MODEL_NAME` is empty, which happens when
    /// [`DynamicModel`] is used without first calling
    /// [`.model_name()`][ChatBuilder::model_name]. To guarantee an infallible
    /// build, call `.model_name("…")` to transition the builder to
    /// [`WithName`] state and use its `.build()` instead.
    ///
    /// [`DynamicModel`]: crate::core::capabilities::DynamicModel
    pub fn build(self) -> Chat {
        assert!(
            !self.model_name.is_empty(),
            "ChatBuilder::build() called with an empty model name. \
             When using DynamicModel, call .model_name(\"...\") before .build().",
        );
        Chat {
            id: self
                .id
                .unwrap_or_else(|| format!("chat_{}", uuid::Uuid::new_v4().simple())),
            messages: self.messages,
            model_name: self.model_name,
            api: self.api,
            status: self.status,
        }
    }

    /// Overrides the model name with a runtime string, transitioning the builder
    /// to the [`WithName`] state.
    ///
    /// This is the only way to obtain a `ChatBuilder<M, WithName>`. It is
    /// required when `M` is [`DynamicModel`] (where `M::MODEL_NAME` is `""`),
    /// and may also be used with concrete model types to override the
    /// compile-time name at runtime.
    ///
    /// After this call, [`.build()`][ChatBuilder::build] is infallible —
    /// it is guaranteed to produce a [`Chat`] with a non-empty `model_name`.
    ///
    /// [`DynamicModel`]: crate::core::capabilities::DynamicModel
    pub fn model_name(self, name: impl Into<String>) -> ChatBuilder<M, WithName> {
        ChatBuilder {
            id: self.id,
            messages: self.messages,
            model_name: name.into(),
            api: self.api,
            status: self.status,
            _model: std::marker::PhantomData,
            _state: std::marker::PhantomData,
        }
    }
}

impl<M: ModelName, S: BuilderState> ChatBuilder<M, S> {
    /// Sets a custom session ID.
    ///
    /// If not called, a random UUID is generated by [`ChatBuilder::build`].
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the initial message history.
    pub fn messages(mut self, messages: Vec<VercelUIMessage>) -> Self {
        self.messages = messages;
        self
    }

    /// Sets the initial [`ChatStatus`].
    ///
    /// Defaults to [`ChatStatus::Ready`] if not called.
    pub fn status(mut self, status: ChatStatus) -> Self {
        self.status = status;
        self
    }
}

impl<M: ModelName> ChatBuilder<M, WithName> {
    /// Builds the [`Chat`] using the model name supplied via
    /// [`.model_name()`][ChatBuilder::model_name].
    ///
    /// This variant is infallible — the [`WithName`] state guarantees that a
    /// non-empty model name was provided before `build()` was called.
    pub fn build(self) -> Chat {
        Chat {
            id: self
                .id
                .unwrap_or_else(|| format!("chat_{}", uuid::Uuid::new_v4().simple())),
            messages: self.messages,
            model_name: self.model_name,
            api: self.api,
            status: self.status,
        }
    }
}

mod hooks {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::capabilities::DynamicModel;

    #[test]
    #[should_panic(expected = "empty model name")]
    fn chat_builder() {
        // --- Concrete model ---
        // Model name is derived from M::MODEL_NAME at compile time.
        #[derive(Debug, Clone)]
        struct MyModel;
        impl ModelName for MyModel {
            const MODEL_NAME: &'static str = "my-model-v1";
        }

        let chat = ChatBuilder::new(MyModel, "/api/chat").build();
        assert_eq!(chat.model_name, "my-model-v1");
        assert_eq!(chat.api, "/api/chat");
        assert_eq!(chat.status, ChatStatus::Ready);
        assert!(chat.messages.is_empty());
        assert!(!chat.id.is_empty());

        // All fluent setters are available in both builder states.
        let chat = ChatBuilder::new(MyModel, "/api/chat")
            .id("session-123")
            .status(ChatStatus::Submitted)
            .build();
        assert_eq!(chat.id, "session-123");
        assert_eq!(chat.status, ChatStatus::Submitted);

        // .model_name() overrides the compile-time constant for any M.
        let chat = ChatBuilder::new(MyModel, "/api/chat")
            .model_name("my-model-v2-finetuned")
            .build();
        assert_eq!(chat.model_name, "my-model-v2-finetuned");

        // --- DynamicModel ---
        // M::MODEL_NAME is "" so .model_name() must be called to transition
        // from WithoutName to WithName before .build() is safe.
        let chat = ChatBuilder::new(DynamicModel, "/api/chat")
            .id("my-session")
            .model_name("gpt-4o") // WithoutName → WithName; .build() is now infallible
            .status(ChatStatus::Ready)
            .build();
        assert_eq!(chat.id, "my-session");
        assert_eq!(chat.model_name, "gpt-4o");

        // Skipping .model_name() on DynamicModel compiles but panics at runtime.
        ChatBuilder::new(DynamicModel, "/api/chat").build();
    }
}
