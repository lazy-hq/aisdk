#![deny(missing_docs)]

//! `aisdk` is An open-source Rust library for building AI-powered applications, inspired by the Vercel AI SDK.
//! It provides a type-safe interface for interacting with Large Language Models (LLMs).

pub mod core;
pub mod error;
pub mod integrations;
pub mod extensions;
#[cfg(feature = "prompt")]
pub mod prompt;
pub mod providers;

// re-exports
pub use error::{Error, Result};

/// Re-exports the `aisdk_macros::tool` macro for convenient tool definition.
pub mod macros {
    pub use aisdk_macros::tool;
}

/// Re-exports modules required by the `aisdk_macros::tool` macro.
pub mod __private {
    pub use schemars;
    pub use serde_json;
}
