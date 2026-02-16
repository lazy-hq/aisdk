//! This module provides the Baseten provider, wrapping OpenAI Chat Completions for Baseten requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    BasetenProviderSettings,
    BasetenProviderSettingsBuilder,
    "Baseten",
    "https://inference.baseten.co/v1",
    "BASETEN_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Baseten, BasetenBuilder, BasetenProviderSettings, "baseten");

// Generate the language model implementation
crate::openai_compatible_language_model!(Baseten);
