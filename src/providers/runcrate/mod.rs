//! This module provides the Runcrate provider, wrapping OpenAI Chat Completions for Runcrate requests.
//!
//! Runcrate serves open-weight models (DeepSeek, Qwen, Kimi, GLM, gpt-oss and more)
//! through an OpenAI-compatible API. See <https://runcrate.ai/docs> for details.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    RuncrateProviderSettings,
    RuncrateProviderSettingsBuilder,
    "Runcrate",
    "https://api.runcrate.ai/v1/",
    "RUNCRATE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Runcrate,
    RuncrateBuilder,
    RuncrateProviderSettings,
    "deepseek-ai/DeepSeek-V4-Flash"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Runcrate);

// Generate the embedding model implementation
crate::openai_compatible_embedding_model!(Runcrate);
