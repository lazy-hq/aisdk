//! Defines the settings for the Vercel provider.

use derive_builder::Builder;

/// Settings for the Vercel provider (delegates to OpenAI).
#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct VercelProviderSettings {
    /// The name of the provider. Defaults to "vercel".
    pub provider_name: String,

    /// The base URL for the Vercel API.
    pub base_url: String,

    /// The API key for the Vercel API.
    pub api_key: String,
}

impl Default for VercelProviderSettings {
    /// Returns the default settings for the Vercel provider.
    fn default() -> Self {
        Self {
            provider_name: "Vercel".to_string(),
            base_url: "https://ai-gateway.vercel.sh/".to_string(),
            api_key: std::env::var("AI_GATEWAY_API_KEY").unwrap_or_default(),
        }
    }
}

impl VercelProviderSettings {
    /// Creates a new builder for `VercelProviderSettings`.
    pub fn builder() -> VercelProviderSettingsBuilder {
        VercelProviderSettingsBuilder::default()
    }
}
