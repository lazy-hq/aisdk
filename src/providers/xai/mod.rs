//! This module provides the xAI provider, wrapping OpenAI Chat Completions for xAI requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    XAISettings,
    XAISettingsBuilder,
    "xAI",
    "https://api.x.ai/",
    "XAI_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(XAI, XAIBuilder, XAISettings, "grok-2-latest");

// Generate the language model implementation
crate::openai_compatible_language_model!(XAI);
