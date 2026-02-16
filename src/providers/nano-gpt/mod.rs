//! This module provides the NanoGpt provider, wrapping OpenAI Chat Completions for NanoGpt requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    NanoGptProviderSettings,
    NanoGptProviderSettingsBuilder,
    "NanoGpt",
    "https://nano-gpt.com/api/v1",
    "NANO_GPT_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(NanoGpt, NanoGptBuilder, NanoGptProviderSettings, "nano-gpt");

// Generate the language model implementation
crate::openai_compatible_language_model!(NanoGpt);
