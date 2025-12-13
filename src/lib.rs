#![deny(missing_docs)]

//! `aisdk` is An open-source Rust library for building AI-powered applications, inspired by the Vercel AI SDK. It provides a type-safe interface for interacting with Large Language Models (LLMs).

pub mod core;
pub mod error;
#[cfg(feature = "prompt")]
pub mod prompt;
pub mod providers;

// re-exports
pub use error::{Error, Result};
