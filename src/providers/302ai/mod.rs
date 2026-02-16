//! This module provides the Ai302 provider, wrapping OpenAI Chat Completions for Ai302 requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    Ai302ProviderSettings,
    Ai302ProviderSettingsBuilder,
    "Ai302",
    "https://api.302.ai/v1",
    "302AI_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Ai302, Ai302Builder, Ai302ProviderSettings, "302ai");

// Generate the language model implementation
crate::openai_compatible_language_model!(Ai302);
