//! This module provides the Inception provider, wrapping OpenAI Chat Completions for Inception requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    InceptionProviderSettings,
    InceptionProviderSettingsBuilder,
    "Inception",
    "https://api.inceptionlabs.ai/v1/",
    "INCEPTION_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Inception,
    InceptionBuilder,
    InceptionProviderSettings,
    "mercury-2"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Inception);
