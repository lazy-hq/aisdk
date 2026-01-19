//! This module provides the `Provider` trait, which defines the interface for
//! interacting with different AI providers.

#[cfg(feature = "openai")]
pub mod openai;

#[cfg(feature = "anthropic")]
pub mod anthropic;

#[cfg(feature = "groq")]
pub mod groq;

#[cfg(feature = "google")]
pub mod google;

#[cfg(feature = "vercel")]
pub mod vercel;

// Internal module for OpenAI Chat Completions API compatible providers
#[cfg(feature = "openaichatcompletions")]
pub(crate) mod openai_chat_completions;
