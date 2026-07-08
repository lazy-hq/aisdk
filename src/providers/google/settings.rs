//! Defines the settings for the Google provider.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Settings for the Google provider.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), default)]
pub struct GoogleProviderSettings {
    /// The name of the provider.
    pub provider_name: String,

    /// The API base URL for the Google API.
    pub base_url: String,

    /// The API key for the Google API.
    pub api_key: String,

    /// Custom API path override. When set, this path is used instead of the
    /// default dynamic path (e.g., "/v1beta/models/{model}:generateContent").
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

impl Default for GoogleProviderSettings {
    /// Returns the default settings for the Google provider.
    fn default() -> Self {
        Self {
            provider_name: "google".to_string(),
            base_url: "https://generativelanguage.googleapis.com".to_string(),
            api_key: std::env::var("GOOGLE_API_KEY").unwrap_or_default(),
            path: None,
            headers: None,
            body: None,
        }
    }
}

impl GoogleProviderSettings {
    /// Creates a new builder for `GoogleProviderSettings`.
    pub fn builder() -> GoogleProviderSettingsBuilder {
        GoogleProviderSettingsBuilder::default()
    }
}
