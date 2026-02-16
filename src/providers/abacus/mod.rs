//! This module provides the Abacus provider, wrapping OpenAI Chat Completions for Abacus requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    AbacusProviderSettings,
    AbacusProviderSettingsBuilder,
    "Abacus",
    "https://routellm.abacus.ai/v1",
    "ABACUS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Abacus, AbacusBuilder, AbacusProviderSettings, "abacus");

// Generate the language model implementation
crate::openai_compatible_language_model!(Abacus);
