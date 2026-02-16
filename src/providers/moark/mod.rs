//! This module provides the Moark provider, wrapping OpenAI Chat Completions for Moark requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    MoarkProviderSettings,
    MoarkProviderSettingsBuilder,
    "Moark",
    "https://moark.com/v1",
    "MOARK_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Moark, MoarkBuilder, MoarkProviderSettings, "moark");

// Generate the language model implementation
crate::openai_compatible_language_model!(Moark);
