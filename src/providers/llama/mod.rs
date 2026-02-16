//! This module provides the Llama provider, wrapping OpenAI Chat Completions for Llama requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    LlamaProviderSettings,
    LlamaProviderSettingsBuilder,
    "Llama",
    "https://api.llama.com/compat/v1/",
    "LLAMA_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Llama, LlamaBuilder, LlamaProviderSettings, "llama");

// Generate the language model implementation
crate::openai_compatible_language_model!(Llama);
