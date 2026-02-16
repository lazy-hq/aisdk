//! This module provides the Wandb provider, wrapping OpenAI Chat Completions for Wandb requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    WandbProviderSettings,
    WandbProviderSettingsBuilder,
    "Wandb",
    "https://api.inference.wandb.ai/v1",
    "WANDB_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Wandb, WandbBuilder, WandbProviderSettings, "wandb");

// Generate the language model implementation
crate::openai_compatible_language_model!(Wandb);
