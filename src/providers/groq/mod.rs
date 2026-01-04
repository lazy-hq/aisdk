//! This module provides the Groq provider, wrapping OpenAI Chat Completions for Groq requests.

pub mod capabilities;
pub mod language_model;
pub mod settings;

use crate::Error;
use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Result;
use crate::providers::groq::settings::GroqProviderSettings;
use crate::providers::openai_chat_completions::OpenAIChatCompletions;

/// The Groq provider, wrapping OpenAI Chat Completions API.
#[derive(Debug, Clone)]
pub struct Groq<M: ModelName> {
    /// Configuration settings for the Groq provider.
    pub settings: GroqProviderSettings,
    pub(crate) inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> Groq<M> {
    /// Groq provider setting builder.
    pub fn builder() -> GroqBuilder<M> {
        GroqBuilder::default()
    }
}

impl<M: ModelName> Default for Groq<M> {
    /// Creates a new Groq provider with default settings.
    fn default() -> Groq<M> {
        GroqBuilder::default().build().unwrap()
    }
}

/// Groq provider builder
pub struct GroqBuilder<M: ModelName> {
    settings: GroqProviderSettings,
    inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> Default for GroqBuilder<M> {
    /// Creates a new Groq provider with default settings.
    fn default() -> Self {
        let settings = GroqProviderSettings::default();
        let mut inner = OpenAIChatCompletions::default();
        inner.settings.provider_name = settings.provider_name.clone();
        inner.settings.base_url = settings.base_url.clone();
        inner.settings.api_key = settings.api_key.clone();

        Self { settings, inner }
    }
}

impl<M: ModelName> GroqBuilder<M> {
    /// Sets the provider name for the Groq provider.
    ///
    /// # Parameters
    ///
    /// * `provider_name` - The provider name string.
    ///
    /// # Returns
    ///
    /// The builder with the provider name set.
    pub fn provider_name(mut self, provider_name: impl Into<String>) -> Self {
        self.settings.provider_name = provider_name.into();
        self
    }

    /// Sets the base URL for the Groq provider.
    ///
    /// # Parameters
    ///
    /// * `base_url` - The base URL string for API requests.
    ///
    /// # Returns
    ///
    /// The builder with the base URL set.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.settings.base_url = base_url.into();
        self
    }

    /// Sets the API key for the Groq provider.
    ///
    /// # Parameters
    ///
    /// * `api_key` - The API key string for authentication.
    ///
    /// # Returns
    ///
    /// The builder with the API key set.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.settings.api_key = api_key.into();
        self
    }

    /// Builds the Groq provider.
    ///
    /// Validates the configuration and creates the provider instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configured `Groq` provider or an `Error`.
    pub fn build(self) -> Result<Groq<M>> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        Ok(Groq {
            settings: GroqProviderSettings {
                base_url: base_url.to_string(),
                ..self.settings
            },
            inner: self.inner,
        })
    }
}

// Re-exports Models for convenience
pub use capabilities::*;
