//! This module provides the Stackit provider, wrapping OpenAI Chat Completions for Stackit requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    StackitProviderSettings,
    StackitProviderSettingsBuilder,
    "Stackit",
    "https://api.openai-compat.model-serving.eu01.onstackit.cloud/v1",
    "STACKIT_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Stackit, StackitBuilder, StackitProviderSettings, "stackit");

// Generate the language model implementation
crate::openai_compatible_language_model!(Stackit);
