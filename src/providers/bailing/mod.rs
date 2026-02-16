//! This module provides the Bailing provider, wrapping OpenAI Chat Completions for Bailing requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    BailingProviderSettings,
    BailingProviderSettingsBuilder,
    "Bailing",
    "https://api.tbox.cn/api/llm/v1/chat/completions",
    "BAILING_API_TOKEN"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Bailing, BailingBuilder, BailingProviderSettings, "bailing");

// Generate the language model implementation
crate::openai_compatible_language_model!(Bailing);
