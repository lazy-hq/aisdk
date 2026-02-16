//! This module provides the Nova provider, wrapping OpenAI Chat Completions for Nova requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    NovaProviderSettings,
    NovaProviderSettingsBuilder,
    "Nova",
    "https://api.nova.amazon.com/v1",
    "NOVA_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Nova, NovaBuilder, NovaProviderSettings, "nova");

// Generate the language model implementation
crate::openai_compatible_language_model!(Nova);
