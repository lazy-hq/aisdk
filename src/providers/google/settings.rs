//! Defines the settings for the Google provider.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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
}

impl Default for GoogleProviderSettings {
    /// Returns the default settings for the Google provider.
    fn default() -> Self {
        Self {
            provider_name: "google".to_string(),
            base_url: "https://generativelanguage.googleapis.com".to_string(),
            api_key: std::env::var("GOOGLE_API_KEY").unwrap_or_default(),
        }
    }
}

impl GoogleProviderSettings {
    /// Creates a new builder for `GoogleProviderSettings`.
    pub fn builder() -> GoogleProviderSettingsBuilder {
        GoogleProviderSettingsBuilder::default()
    }
}
