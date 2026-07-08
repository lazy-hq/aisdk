//! Defines the settings for the Anthropic provider.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    /// Custom API path override. When set, this path is used instead of the
    /// default "/messages".
    pub path: Option<String>,

    /// Extra headers to include in every request made with this provider.
    /// These are merged with any request-level headers, with request-level taking priority.
    #[serde(skip)]
    #[builder(setter(skip))]
    pub headers: Option<HashMap<String, String>>,

    /// Extra body fields to include in every request made with this provider.
    /// These are merged with any request-level body, with request-level taking priority.
    #[serde(skip)]
    #[builder(setter(skip))]
    pub body: Option<serde_json::Map<String, serde_json::Value>>,
}

impl Default for AnthropicProviderSettings {
    /// Returns the default settings for the Anthropic provider.
    fn default() -> Self {
        Self {
            provider_name: "anthropic".to_string(),
            base_url: "https://api.anthropic.com/v1/".to_string(),
            api_key: std::env::var("ANTHROPIC_API_KEY").unwrap_or_default(),
            path: None,
            headers: None,
            body: None,
        }
    }
}

impl AnthropicProviderSettings {
    /// Creates a new builder for `AnthropicProviderSettings`.
    pub fn builder() -> AnthropicProviderSettingsBuilder {
        AnthropicProviderSettingsBuilder::default()
    }
}
