//! This module provides the Alibaba provider, wrapping OpenAI Chat Completions for Alibaba requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    AlibabaProviderSettings,
    AlibabaProviderSettingsBuilder,
    "Alibaba",
    "https://dashscope-intl.aliyuncs.com/compatible-mode/v1",
    "DASHSCOPE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Alibaba, AlibabaBuilder, AlibabaProviderSettings, "alibaba");

// Generate the language model implementation
crate::openai_compatible_language_model!(Alibaba);
