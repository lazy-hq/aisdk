//! This module provides the Jiekou provider, wrapping OpenAI Chat Completions for Jiekou requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    JiekouProviderSettings,
    JiekouProviderSettingsBuilder,
    "Jiekou",
    "https://api.jiekou.ai/openai",
    "JIEKOU_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Jiekou,
    JiekouBuilder,
    JiekouProviderSettings,
    "jiekou"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Jiekou);
