//! This module provides the Morph provider, wrapping OpenAI Chat Completions for Morph requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    MorphProviderSettings,
    MorphProviderSettingsBuilder,
    "Morph",
    "https://api.morphllm.com/v1",
    "MORPH_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Morph, MorphBuilder, MorphProviderSettings, "morph");

// Generate the language model implementation
crate::openai_compatible_language_model!(Morph);
