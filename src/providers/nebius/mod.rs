//! This module provides the Nebius provider, wrapping OpenAI Chat Completions for Nebius requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    NebiusProviderSettings,
    NebiusProviderSettingsBuilder,
    "Nebius",
    "https://api.tokenfactory.nebius.com/v1",
    "NEBIUS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Nebius, NebiusBuilder, NebiusProviderSettings, "nebius");

// Generate the language model implementation
crate::openai_compatible_language_model!(Nebius);
