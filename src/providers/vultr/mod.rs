//! This module provides the Vultr provider, wrapping OpenAI Chat Completions for Vultr requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    VultrProviderSettings,
    VultrProviderSettingsBuilder,
    "Vultr",
    "https://api.vultrinference.com/v1",
    "VULTR_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Vultr, VultrBuilder, VultrProviderSettings, "vultr");

// Generate the language model implementation
crate::openai_compatible_language_model!(Vultr);
