//! This module provides the Xiaomi provider, wrapping OpenAI Chat Completions for Xiaomi requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    XiaomiProviderSettings,
    XiaomiProviderSettingsBuilder,
    "Xiaomi",
    "https://api.xiaomimimo.com/v1",
    "XIAOMI_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Xiaomi, XiaomiBuilder, XiaomiProviderSettings, "xiaomi");

// Generate the language model implementation
crate::openai_compatible_language_model!(Xiaomi);
