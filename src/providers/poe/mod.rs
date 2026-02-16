//! This module provides the Poe provider, wrapping OpenAI Chat Completions for Poe requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    PoeProviderSettings,
    PoeProviderSettingsBuilder,
    "Poe",
    "https://api.poe.com/v1",
    "POE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Poe, PoeBuilder, PoeProviderSettings, "poe");

// Generate the language model implementation
crate::openai_compatible_language_model!(Poe);
