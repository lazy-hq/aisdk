//! Defines the settings for the Groq provider.

use crate::{error::Error, providers::groq::Groq, providers::openai::OpenAI};

/// Settings for the Groq provider (delegates to OpenAI).
#[derive(Debug, Clone)]
pub struct GroqProviderSettings;

impl GroqProviderSettings {
    /// Creates a new builder for GroqSettings.
    pub fn builder() -> GroqProviderSettingsBuilder {
        GroqProviderSettingsBuilder::default()
    }
}

pub struct GroqProviderSettingsBuilder {
    base_url: Option<String>,
    api_key: Option<String>,
    provider_name: Option<String>,
    model_name: Option<String>,
}

impl GroqProviderSettingsBuilder {
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
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

    pub fn build(self) -> Result<Groq, Error> {
        let openai = OpenAI::builder()
            .base_url(
                self.base_url
                    .unwrap_or_else(|| "https://api.groq.com/openai/v1".to_string()),
            )
            .api_key(
                self.api_key
                    .unwrap_or_else(|| std::env::var("GROQ_API_KEY").unwrap_or_default()),
            )
            .provider_name(self.provider_name.unwrap_or_else(|| "groq".to_string()))
            .model_name(
                self.model_name
                    .unwrap_or_else(|| "llama-3.3-70b-versatile".to_string()),
            )
            .build()?;
        Ok(Groq { inner: openai })
    }
}

impl Default for GroqProviderSettingsBuilder {
    fn default() -> Self {
        Self {
            base_url: Some("https://api.groq.com/openai/v1".to_string()),
            api_key: Some(std::env::var("GROQ_API_KEY").unwrap_or_default()),
            provider_name: Some("groq".to_string()),
            model_name: Some("llama-3.3-70b-versatile".to_string()),
        }
    }
}
