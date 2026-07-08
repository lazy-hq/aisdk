//! Defines the settings for the OpenAI-compatible provider.

use derive_builder::Builder;
use std::collections::HashMap;

/// Settings for the OpenAI-compatible provider (delegates to OpenAI).
#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct OpenAICompatibleSettings {
    /// The name of the provider. Defaults to "OpenAICompatible".
    pub provider_name: String,

    /// The base URL for the OpenAI-compatible API.
    pub base_url: String,

    /// The API key for the OpenAI-compatible API.
    pub api_key: String,

    /// Custom API path override.
    pub path: Option<String>,

    /// Extra headers to merge into every request.
    #[builder(setter(skip))]
    pub headers: Option<HashMap<String, String>>,

    /// Extra body fields to merge into every request.
    #[builder(setter(skip))]
    pub body: Option<serde_json::Map<String, serde_json::Value>>,
}

impl Default for OpenAICompatibleSettings {
    /// Returns the default settings for the OpenAI-compatible provider.
    fn default() -> Self {
        Self {
            provider_name: "OpenAICompatible".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            path: None,
            headers: None,
            body: None,
        }
    }
}

impl OpenAICompatibleSettings {
    /// Creates a new builder for `OpenAICompatibleSettings`.
    pub fn builder() -> OpenAICompatibleSettingsBuilder {
        OpenAICompatibleSettingsBuilder::default()
    }
}
