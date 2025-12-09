//! Defines the settings for the OpenAI provider.

use reqwest::{IntoUrl, Url};

use crate::{error::Error, providers::openai::OpenAI};

/// Settings for the OpenAI provider.
#[derive(Debug, Clone)]
pub struct OpenAIProviderSettings {
    /// The API base URL for the OpenAI API.
    pub base_url: Url,

    /// The API key for the OpenAI API.
    pub api_key: String,

    /// The name of the provider.
    pub provider_name: String,

    /// The name of the model to use.
    pub model_name: String,
}

impl OpenAIProviderSettings {
    /// Creates a new builder for `OpenAISettings`.
    pub fn builder() -> OpenAIProviderSettingsBuilder {
        OpenAIProviderSettingsBuilder::default()
    }
}

pub struct OpenAIProviderSettingsBuilder {
    base_url: Option<Url>,
    api_key: Option<String>,
    provider_name: Option<String>,
    model_name: Option<String>,
}

impl OpenAIProviderSettingsBuilder {
    pub fn base_url(mut self, base_url: impl IntoUrl) -> Self {
        self.base_url = Some(base_url.into_url().expect("Invalid base URL"));
        self
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn provider_name(mut self, provider_name: impl Into<String>) -> Self {
        self.provider_name = Some(provider_name.into());
        self
    }

    pub fn model_name(mut self, model_name: impl Into<String>) -> Self {
        self.model_name = Some(model_name.into());
        self
    }

    pub fn build(self) -> Result<OpenAI, Error> {
        let settings = OpenAIProviderSettings {
            base_url: self.base_url.expect("Missing base URL"),
            api_key: self.api_key.unwrap_or_default(),
            provider_name: self.provider_name.unwrap_or_else(|| "openai".to_string()),
            model_name: self.model_name.unwrap_or_else(|| "gpt-4o".to_string()),
        };

        Ok(OpenAI { settings })
    }
}

impl Default for OpenAIProviderSettingsBuilder {
    fn default() -> Self {
        Self {
            base_url: Some(Url::parse("https://api.openai.com/v1/").unwrap()),
            api_key: Some(std::env::var("OPENAI_API_KEY").unwrap_or_default()),
            provider_name: Some("openai".to_string()),
            model_name: Some("gpt-4o".to_string()),
        }
    }
}
