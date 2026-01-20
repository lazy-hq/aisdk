//! Defines the settings for the DeepSeek provider.

use derive_builder::Builder;

/// Settings for the DeepSeek provider (delegates to OpenAI).
#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct DeepSeekProviderSettings {
    /// The name of the provider. Defaults to "DeepSeek".
    pub provider_name: String,

    /// The base URL for the DeepSeek API.
    pub base_url: String,

    /// The API key for the DeepSeek API.
    pub api_key: String,
}

impl Default for DeepSeekProviderSettings {
    /// Returns the default settings for the DeepSeek provider.
    fn default() -> Self {
        Self {
            provider_name: "DeepSeek".to_string(),
            base_url: "https://api.deepseek.com/v1/".to_string(),
            api_key: std::env::var("DEEPSEEK_API_KEY").unwrap_or_default(),
        }
    }
}

impl DeepSeekProviderSettings {
    /// Creates a new builder for `DeepSeekProviderSettings`.
    pub fn builder() -> DeepSeekProviderSettingsBuilder {
        DeepSeekProviderSettingsBuilder::default()
    }
}
