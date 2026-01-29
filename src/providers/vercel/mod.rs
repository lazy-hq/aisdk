//! This module provides the Vercel provider, wrapping OpenAI Chat Completions for Vercel requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    VercelProviderSettings,
    VercelProviderSettingsBuilder,
    "Vercel",
    "https://api.vercel.ai/",
    "VERCEL_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Vercel,
    VercelBuilder,
    VercelProviderSettings,
    "claude-3-5-haiku-2024-10-22"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Vercel);

// Generate the embedding model implementation
crate::openai_compatible_embedding_model!(Vercel);
