//! This module provides the OpenRouter provider, wrapping OpenAI Chat Completions for OpenRouter requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    OpenRouterProviderSettings,
    OpenRouterProviderSettingsBuilder,
    "OpenRouter",
    "https://openrouter.ai/api/v1",
    "OPENROUTER_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    OpenRouter,
    OpenRouterBuilder,
    OpenRouterProviderSettings,
    "anthropic/claude-3.5-sonnet"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(OpenRouter);

// Generate the embedding model implementation
crate::openai_compatible_embedding_model!(OpenRouter);
