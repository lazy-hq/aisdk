//! Integration with Dioxus. WIP.

/// Types for the Dioxus integration.
pub mod types {
    use crate::integrations::vercel_aisdk_ui::VercelUIMessage;

    /// Config options for the `use_chat` hook.
    pub struct DioxusUseChatOptions {
        /// Server path to use, defaults to "/api/chat"
        pub api: String,
    }

    impl Default for DioxusUseChatOptions {
        fn default() -> Self {
            Self {
                api: String::from("/api/chat"),
            }
        }
    }

    /// Current state of the chat
    pub enum DioxusChatStatus {
        /// The request has been sent, awaiting a response
        Submitted,
        /// The first response has been received, processing following stream
        Streaming,
        /// The stream has been fully processed, ready for new requests
        Ready,
        /// An error has occurred, ready for new request or regeneration
        Error,
    }

    /// A signal returned by the `use_chat` hook.
    pub struct DioxusChatSignal {
        /// Chat messages
        pub message: Vec<VercelUIMessage>,
        /// Chat state
        pub status: DioxusChatStatus,
    }
}
