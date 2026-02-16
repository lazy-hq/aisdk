//! This module provides the Upstage provider, wrapping OpenAI Chat Completions for Upstage requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    UpstageProviderSettings,
    UpstageProviderSettingsBuilder,
    "Upstage",
    "https://api.upstage.ai/v1/solar",
    "UPSTAGE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Upstage, UpstageBuilder, UpstageProviderSettings, "upstage");

// Generate the language model implementation
crate::openai_compatible_language_model!(Upstage);
