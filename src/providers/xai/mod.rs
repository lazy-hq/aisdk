//! This module provides the xAI provider, wrapping OpenAI Chat Completions for xAI requests.

pub mod capabilities;
pub mod language_model;
pub mod settings;

use crate::Error;
use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Result;
use crate::providers::openai_chat_completions::OpenAIChatCompletions;
use crate::providers::xai::settings::XAISettings;

/// The xAI provider, wrapping OpenAI Chat Completions API.
#[derive(Debug, Clone)]
pub struct XAI<M: ModelName> {
    /// Configuration settings for the xAI provider.
    pub settings: XAISettings,
    pub(crate) inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> XAI<M> {
    /// xAI provider setting builder.
    pub fn builder() -> XAIBuilder<M> {
        XAIBuilder::default()
    }
}

impl<M: ModelName> Default for XAI<M> {
    /// Creates a new xAI provider with default settings.
    fn default() -> XAI<M> {
        XAIBuilder::default().build().unwrap()
    }
}

/// xAI provider builder
pub struct XAIBuilder<M: ModelName> {
    settings: XAISettings,
    inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> Default for XAIBuilder<M> {
    /// Creates a new xAI provider with default settings.
    fn default() -> Self {
        let settings = XAISettings::default();
        let mut inner = OpenAIChatCompletions::default();
        inner.settings.provider_name = settings.provider_name.clone();
        inner.settings.base_url = settings.base_url.clone();
        inner.settings.api_key = settings.api_key.clone();

        Self { settings, inner }
    }
}

impl<M: ModelName> XAIBuilder<M> {
    /// Sets the provider name for the xAI provider.
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

    /// Sets the base URL for the xAI provider.
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

    /// Sets the API key for the xAI provider.
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

    /// Builds the xAI provider.
    ///
    /// Validates the configuration and creates the provider instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configured `XAI` provider or an `Error`.
    pub fn build(mut self) -> Result<XAI<M>> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        // Update the inner provider with the validated base_url
        self.inner.settings.base_url = base_url.to_string();
        self.settings.base_url = base_url.to_string();

        Ok(XAI {
            settings: self.settings,
            inner: self.inner,
        })
    }
}

// Re-exports Models for convenience
pub use capabilities::*;
