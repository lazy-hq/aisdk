//! This module provides the Cortecs provider, wrapping OpenAI Chat Completions for Cortecs requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    CortecsProviderSettings,
    CortecsProviderSettingsBuilder,
    "Cortecs",
    "https://api.cortecs.ai/v1",
    "CORTECS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Cortecs, CortecsBuilder, CortecsProviderSettings, "cortecs");

// Generate the language model implementation
crate::openai_compatible_language_model!(Cortecs);
