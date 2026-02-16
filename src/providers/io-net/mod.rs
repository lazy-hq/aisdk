//! This module provides the IoNet provider, wrapping OpenAI Chat Completions for IoNet requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    IoNetProviderSettings,
    IoNetProviderSettingsBuilder,
    "IoNet",
    "https://api.intelligence.io.solutions/api/v1",
    "IOINTELLIGENCE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(IoNet, IoNetBuilder, IoNetProviderSettings, "io-net");

// Generate the language model implementation
crate::openai_compatible_language_model!(IoNet);
