//! Defines the settings for the Together AI provider.

use derive_builder::Builder;

/// Settings for the Together AI provider (delegates to OpenAI).
#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct TogetherAIProviderSettings {
    /// The name of the provider. Defaults to "TogetherAI".
    pub provider_name: String,

    /// The base URL for the Together AI API.
    pub base_url: String,

    /// The API key for the Together AI API.
    pub api_key: String,
}

impl Default for TogetherAIProviderSettings {
    /// Returns the default settings for the Together AI provider.
    fn default() -> Self {
        Self {
            provider_name: "TogetherAI".to_string(),
            base_url: "https://api.together.xyz/".to_string(),
            api_key: std::env::var("TOGETHER_API_KEY").unwrap_or_default(),
        }
    }
}

impl TogetherAIProviderSettings {
    /// Creates a new builder for `TogetherAIProviderSettings`.
    pub fn builder() -> TogetherAIProviderSettingsBuilder {
        TogetherAIProviderSettingsBuilder::default()
    }
}
