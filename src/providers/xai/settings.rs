//! Defines the settings for the xAI provider.

use derive_builder::Builder;

/// Settings for the xAI provider (delegates to OpenAI).
#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct XAISettings {
    /// The name of the provider. Defaults to "xAI".
    pub provider_name: String,

    /// The base URL for the xAI API.
    pub base_url: String,

    /// The API key for the xAI API.
    pub api_key: String,
}

impl Default for XAISettings {
    /// Returns the default settings for the xAI provider.
    fn default() -> Self {
        Self {
            provider_name: "xAI".to_string(),
            base_url: "https://api.x.ai/".to_string(),
            api_key: std::env::var("XAI_API_KEY").unwrap_or_default(),
        }
    }
}

impl XAISettings {
    /// Creates a new builder for `XAISettings`.
    pub fn builder() -> XAISettingsBuilder {
        XAISettingsBuilder::default()
    }
}
