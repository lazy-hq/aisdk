//! This module provides the Groq provider, wrapping OpenAI Chat Completions for Groq requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    GroqProviderSettings,
    GroqProviderSettingsBuilder,
    "Groq",
    "https://api.groq.com/openai/v1/",
    "GROQ_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Groq,
    GroqBuilder,
    GroqProviderSettings,
    "llama-3.3-70b-specdec"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Groq);
