//! This module provides the Chutes provider, wrapping OpenAI Chat Completions for Chutes requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ChutesProviderSettings,
    ChutesProviderSettingsBuilder,
    "Chutes",
    "https://llm.chutes.ai/v1",
    "CHUTES_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Chutes, ChutesBuilder, ChutesProviderSettings, "chutes");

// Generate the language model implementation
crate::openai_compatible_language_model!(Chutes);
