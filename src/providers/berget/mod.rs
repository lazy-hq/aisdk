//! This module provides the Berget provider, wrapping OpenAI Chat Completions for Berget requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    BergetProviderSettings,
    BergetProviderSettingsBuilder,
    "Berget",
    "https://api.berget.ai/v1",
    "BERGET_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Berget,
    BergetBuilder,
    BergetProviderSettings,
    "berget"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Berget);
