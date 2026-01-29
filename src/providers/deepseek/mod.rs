//! This module provides the DeepSeek provider, wrapping OpenAI Chat Completions for DeepSeek requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    DeepSeekProviderSettings,
    DeepSeekProviderSettingsBuilder,
    "DeepSeek",
    "https://api.deepseek.com/v1/",
    "DEEPSEEK_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    DeepSeek,
    DeepSeekBuilder,
    DeepSeekProviderSettings,
    "deepseek-chat"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(DeepSeek);
