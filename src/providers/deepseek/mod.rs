//! This module provides the DeepSeek provider, wrapping OpenAI Chat Completions for DeepSeek requests.

pub mod capabilities;
pub mod language_model;
pub mod settings;

use crate::Error;
use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Result;
use crate::providers::deepseek::settings::DeepSeekProviderSettings;
use crate::providers::openai_chat_completions::OpenAIChatCompletions;

/// The DeepSeek provider, wrapping OpenAI Chat Completions API.
#[derive(Debug, Clone)]
pub struct DeepSeek<M: ModelName> {
    /// Configuration settings for the DeepSeek provider.
    pub settings: DeepSeekProviderSettings,
    pub(crate) inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> DeepSeek<M> {
    /// DeepSeek provider setting builder.
    pub fn builder() -> DeepSeekBuilder<M> {
        DeepSeekBuilder::default()
    }
}

impl<M: ModelName> Default for DeepSeek<M> {
    /// Creates a new DeepSeek provider with default settings.
    fn default() -> DeepSeek<M> {
        DeepSeekBuilder::default().build().unwrap()
    }
}

/// DeepSeek provider builder
pub struct DeepSeekBuilder<M: ModelName> {
    settings: DeepSeekProviderSettings,
    inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> Default for DeepSeekBuilder<M> {
    /// Creates a new DeepSeek provider with default settings.
    fn default() -> Self {
        let settings = DeepSeekProviderSettings::default();
        let mut inner = OpenAIChatCompletions::default();
        inner.settings.provider_name = settings.provider_name.clone();
        inner.settings.base_url = settings.base_url.clone();
        inner.settings.api_key = settings.api_key.clone();

        Self { settings, inner }
    }
}

impl<M: ModelName> DeepSeekBuilder<M> {
    /// Sets the provider name for the DeepSeek provider.
    ///
    /// # Parameters
    ///
    /// * `provider_name` - The provider name string.
    ///
    /// # Returns
    ///
    /// The builder with the provider name set.
    pub fn provider_name(mut self, provider_name: impl Into<String>) -> Self {
        let name = provider_name.into();
        self.settings.provider_name = name.clone();
        self.inner.settings.provider_name = name;
        self
    }

    /// Sets the base URL for the DeepSeek provider.
    ///
    /// # Parameters
    ///
    /// * `base_url` - The base URL string for API requests.
    ///
    /// # Returns
    ///
    /// The builder with the base URL set.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        let url = base_url.into();
        self.settings.base_url = url.clone();
        self.inner.settings.base_url = url;
        self
    }

    /// Sets the API key for the DeepSeek provider.
    ///
    /// # Parameters
    ///
    /// * `api_key` - The API key string for authentication.
    ///
    /// # Returns
    ///
    /// The builder with the API key set.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        let key = api_key.into();
        self.settings.api_key = key.clone();
        self.inner.settings.api_key = key;
        self
    }

    /// Builds the DeepSeek provider.
    ///
    /// Validates the configuration and creates the provider instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configured `DeepSeek` provider or an `Error`.
    pub fn build(mut self) -> Result<DeepSeek<M>> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        // Update the inner provider with the validated base_url
        self.inner.settings.base_url = base_url.to_string();
        self.settings.base_url = base_url.to_string();

        Ok(DeepSeek {
            settings: self.settings,
            inner: self.inner,
        })
    }
}

// Re-exports Models for convenience
pub use capabilities::*;
