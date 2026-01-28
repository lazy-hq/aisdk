//! The core components of the AI SDK, including traits, types, and the main generation function.
//!
//! This module provides the essential building blocks for interacting with language models.
//! It defines the `LanguageModel` trait, which all model providers must implement,
//! and includes the primary `generate_text` function for initiating text generation.
//!
//! Key types like `GenerateTextCallOptions` and `GenerateTextResponse` are also
//! re-exported for convenient access.

pub mod capabilities;
pub mod client;
pub mod language_model;
pub mod messages;
pub mod provider;
pub mod tools;
pub mod utils;

// Re-export key components to provide a clean public API.
pub use capabilities::DynamicModel;
pub use language_model::{
    LanguageModel, LanguageModelStreamChunkType, generate_text::GenerateTextResponse,
    request::LanguageModelRequest, stream_text::StreamTextResponse,
};

pub use messages::{AssistantMessage, Message, Messages, Role, SystemMessage, UserMessage};
pub use provider::Provider;
pub use tools::{
    NeedsApproval, NeedsApprovalContext, NeedsApprovalFn, Tool, ToolApprovalRequest,
    ToolApprovalResponse, ToolCallInfo, ToolResultInfo,
};
