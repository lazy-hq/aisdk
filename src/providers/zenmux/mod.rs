//! This module provides the Zenmux provider, wrapping OpenAI Chat Completions for Zenmux requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ZenmuxProviderSettings,
    ZenmuxProviderSettingsBuilder,
    "Zenmux",
    "https://zenmux.ai/api/v1",
    "ZENMUX_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Zenmux, ZenmuxBuilder, ZenmuxProviderSettings, "zenmux");

// Generate the language model implementation
crate::openai_compatible_language_model!(Zenmux);
