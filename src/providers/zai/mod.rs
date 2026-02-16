//! This module provides the Zai provider, wrapping OpenAI Chat Completions for Zai requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ZaiProviderSettings,
    ZaiProviderSettingsBuilder,
    "Zai",
    "https://api.z.ai/api/paas/v4",
    "ZHIPU_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Zai, ZaiBuilder, ZaiProviderSettings, "zai");

// Generate the language model implementation
crate::openai_compatible_language_model!(Zai);
