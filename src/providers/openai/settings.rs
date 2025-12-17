//! Defines the settings for the OpenAI provider.

use derive_builder::Builder;

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
}

impl Default for OpenAIProviderSettings {
    /// Returns the default settings for the OpenAI provider.
    fn default() -> Self {
        Self {
            provider_name: "openai".to_string(),
            base_url: "https://api.openai.com/v1/".to_string(),
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
        }
    }
}

impl OpenAIProviderSettings {
    /// Creates a new builder for `OpenAISettings`.
    pub fn builder() -> OpenAIProviderSettingsBuilder {
        OpenAIProviderSettingsBuilder::default()
    }
}
