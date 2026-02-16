//! This module provides the Zhipuai provider, wrapping OpenAI Chat Completions for Zhipuai requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ZhipuaiProviderSettings,
    ZhipuaiProviderSettingsBuilder,
    "Zhipuai",
    "https://open.bigmodel.cn/api/paas/v4",
    "ZHIPU_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Zhipuai, ZhipuaiBuilder, ZhipuaiProviderSettings, "zhipuai");

// Generate the language model implementation
crate::openai_compatible_language_model!(Zhipuai);
