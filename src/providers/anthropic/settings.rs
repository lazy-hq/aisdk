//! Defines the settings for the Anthropic provider.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Settings for the Anthropic provider.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), default)]
pub struct AnthropicProviderSettings {
    /// The name of the provider.
    pub provider_name: String,

    /// The API base URL for the Anthropic API.
    pub base_url: String,

    /// The API key for the Anthropic API.
    pub api_key: String,
}

impl Default for AnthropicProviderSettings {
    /// Returns the default settings for the Anthropic provider.
    fn default() -> Self {
        Self {
            provider_name: "anthropic".to_string(),
            base_url: "https://api.anthropic.com/v1/".to_string(),
            api_key: std::env::var("ANTHROPIC_API_KEY").unwrap_or_default(),
        }
    }
}

impl AnthropicProviderSettings {
    /// Creates a new builder for `AnthropicProviderSettings`.
    pub fn builder() -> AnthropicProviderSettingsBuilder {
        AnthropicProviderSettingsBuilder::default()
    }
}
