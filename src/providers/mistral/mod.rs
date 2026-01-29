//! This module provides the Mistral AI provider, wrapping OpenAI Chat Completions for Mistral requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    MistralProviderSettings,
    MistralProviderSettingsBuilder,
    "Mistral",
    "https://api.mistral.ai/v1/",
    "MISTRAL_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Mistral,
    MistralBuilder,
    MistralProviderSettings,
    "mistral-large"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Mistral);
