//! This module provides the Together AI provider, wrapping OpenAI Chat Completions for Together AI requests.
//!

// NOTE: Together AI might not be fully compatible with the OpenAI API. Please refer
// to the Together AI documentation for more information.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    TogetherAIProviderSettings,
    TogetherAIProviderSettingsBuilder,
    "Together AI",
    "https://api.together.xyz/v1/",
    "TOGETHER_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    TogetherAI,
    TogetherAIBuilder,
    TogetherAIProviderSettings,
    "Llama-3.3-70B-Instruct-Turbo"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(TogetherAI);

// Generate the embedding model implementation
crate::openai_compatible_embedding_model!(TogetherAI);
