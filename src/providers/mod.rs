//! This module provides the `Provider` trait, which defines the interface for
//! interacting with different AI providers.

#[cfg(feature = "openai")]
pub mod openai;
#[cfg(feature = "openai")]
pub use openai::OpenAI;

// Public OpenAI-compatible provider
#[cfg(feature = "openaicompatible")]
pub mod openai_compatible;
#[cfg(feature = "openaicompatible")]
pub use openai_compatible::OpenAICompatible;

#[cfg(feature = "anthropic")]
pub mod anthropic;
#[cfg(feature = "anthropic")]
pub use anthropic::Anthropic;

#[cfg(feature = "groq")]
pub mod groq;
#[cfg(feature = "groq")]
pub use groq::Groq;

#[cfg(feature = "google")]
pub mod google;
#[cfg(feature = "google")]
pub use google::Google;

#[cfg(feature = "vercel")]
pub mod vercel;
#[cfg(feature = "vercel")]
pub use vercel::Vercel;

#[cfg(feature = "openrouter")]
pub mod openrouter;
#[cfg(feature = "openrouter")]
pub use openrouter::Openrouter;

#[cfg(feature = "deepseek")]
pub mod deepseek;
#[cfg(feature = "deepseek")]
pub use deepseek::Deepseek;

#[cfg(feature = "mistral")]
pub mod mistral;
#[cfg(feature = "mistral")]
pub use mistral::Mistral;

#[cfg(feature = "amazon-bedrock")]
pub mod amazon_bedrock;
#[cfg(feature = "amazon-bedrock")]
pub use amazon_bedrock::AmazonBedrock;

#[cfg(feature = "togetherai")]
pub mod togetherai;
#[cfg(feature = "togetherai")]
pub use togetherai::TogetherAI;

#[cfg(feature = "xai")]
pub mod xai;
#[cfg(feature = "xai")]
pub use xai::XAI;

// Internal module for OpenAI Chat Completions API compatible providers
#[cfg(feature = "openaichatcompletions")]
pub(crate) mod openai_chat_completions;

// OpenAI-compatible providers
