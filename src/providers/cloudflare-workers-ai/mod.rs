//! This module provides the CloudflareWorkersAi provider, wrapping OpenAI Chat Completions for CloudflareWorkersAi requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    CloudflareWorkersAiProviderSettings,
    CloudflareWorkersAiProviderSettingsBuilder,
    "CloudflareWorkersAi",
    "https://api.cloudflare.com/client/v4/accounts/${CLOUDFLARE_ACCOUNT_ID}/ai/v1",
    "CLOUDFLARE_ACCOUNT_ID"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    CloudflareWorkersAi,
    CloudflareWorkersAiBuilder,
    CloudflareWorkersAiProviderSettings,
    "cloudflare-workers-ai"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(CloudflareWorkersAi);
