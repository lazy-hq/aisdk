//! This module provides the Deepseek provider, wrapping OpenAI Chat Completions for Deepseek requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    DeepseekProviderSettings,
    DeepseekProviderSettingsBuilder,
    "Deepseek",
    "https://api.deepseek.com",
    "DEEPSEEK_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Deepseek,
    DeepseekBuilder,
    DeepseekProviderSettings,
    "deepseek"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Deepseek);
