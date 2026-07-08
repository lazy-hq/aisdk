//! Defines the settings for the OpenAI provider.

use derive_builder::Builder;
use std::collections::HashMap;

#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
/// Settings for the OpenAI provider.
pub struct OpenAIProviderSettings {
    /// The name of the provider. Defaults to "openai".
    pub provider_name: String,

    /// The API base URL for the OpenAI API.
    pub base_url: String,

    /// The API key for the OpenAI API.
    pub api_key: String,

    /// Custom API path override. When set, this path is used instead of the
    /// provider's default path (e.g., "/v1/responses").
    /// This is useful for connecting to endpoints that use a different path,
    /// such as OpenAI Codex (`/responses`).
    pub path: Option<String>,

    /// Extra headers to include in every request made with this provider.
    /// These are merged with any request-level headers, with request-level taking priority.
    #[builder(setter(skip))]
    pub headers: Option<HashMap<String, String>>,

    /// Extra body fields to include in every request made with this provider.
    /// These are merged with any request-level body, with request-level taking priority.
    #[builder(setter(skip))]
    pub body: Option<serde_json::Map<String, serde_json::Value>>,
}

impl Default for OpenAIProviderSettings {
    fn default() -> Self {
        Self {
            provider_name: "openai".to_string(),
            base_url: "https://api.openai.com".to_string(),
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
            path: None,
            headers: None,
            body: None,
        }
    }
}

impl OpenAIProviderSettings {
    /// Creates a new builder for `OpenAISettings`.
    pub fn builder() -> OpenAIProviderSettingsBuilder {
        OpenAIProviderSettingsBuilder::default()
    }
}
