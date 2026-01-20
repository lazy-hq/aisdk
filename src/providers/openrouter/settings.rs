//! Defines the settings for the OpenRouter provider.

use derive_builder::Builder;

/// Settings for the OpenRouter provider (delegates to OpenAI).
#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct OpenRouterProviderSettings {
    /// The name of the provider. Defaults to "OpenRouter".
    pub provider_name: String,

    /// The base URL for the OpenRouter API.
    pub base_url: String,

    /// The API key for the OpenRouter API.
    pub api_key: String,
}

impl Default for OpenRouterProviderSettings {
    /// Returns the default settings for the OpenRouter provider.
    fn default() -> Self {
        Self {
            provider_name: "OpenRouter".to_string(),
            base_url: "https://openrouter.ai/api/".to_string(),
            api_key: std::env::var("OPENROUTER_API_KEY").unwrap_or_default(),
        }
    }
}

impl OpenRouterProviderSettings {
    /// Creates a new builder for `OpenRouterProviderSettings`.
    pub fn builder() -> OpenRouterProviderSettingsBuilder {
        OpenRouterProviderSettingsBuilder::default()
    }
}
