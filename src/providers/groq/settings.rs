//! Defines the settings for the Groq provider.

use derive_builder::Builder;

/// Settings for the Groq provider (delegates to OpenAI).
#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct GroqProviderSettings {
    /// The name of the provider. Defaults to "groq".
    pub provider_name: String,

    /// The base URL for the Groq API.
    pub base_url: String,

    /// The API key for the Groq API.
    pub api_key: String,
}

impl Default for GroqProviderSettings {
    /// Returns the default settings for the Groq provider.
    fn default() -> Self {
        Self {
            provider_name: "Groq".to_string(),
            base_url: "https://api.groq.com/openai/v1/".to_string(),
            api_key: std::env::var("GROQ_API_KEY").unwrap_or_default(),
        }
    }
}

impl GroqProviderSettings {
    /// Creates a new builder for `GroqProviderSettings`.
    pub fn builder() -> GroqProviderSettingsBuilder {
        GroqProviderSettingsBuilder::default()
    }
}
