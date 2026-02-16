//! This module provides the Nvidia provider, wrapping OpenAI Chat Completions for Nvidia requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    NvidiaProviderSettings,
    NvidiaProviderSettingsBuilder,
    "Nvidia",
    "https://integrate.api.nvidia.com/v1",
    "NVIDIA_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Nvidia, NvidiaBuilder, NvidiaProviderSettings, "nvidia");

// Generate the language model implementation
crate::openai_compatible_language_model!(Nvidia);
