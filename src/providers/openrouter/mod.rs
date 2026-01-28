//! This module provides the OpenRouter provider, wrapping OpenAI Chat Completions for OpenRouter requests.

pub mod capabilities;
pub mod embedding_model;
pub mod language_model;
pub mod settings;

use crate::Error;
use crate::core::DynamicModel;
use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Result;
use crate::providers::openai_chat_completions::OpenAIChatCompletions;
use crate::providers::openrouter::settings::OpenRouterProviderSettings;

/// The OpenRouter provider, wrapping OpenAI Chat Completions API.
#[derive(Debug, Clone)]
pub struct OpenRouter<M: ModelName> {
    /// Configuration settings for the OpenRouter provider.
    pub settings: OpenRouterProviderSettings,
    pub(crate) inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> OpenRouter<M> {
    /// OpenRouter provider setting builder.
    pub fn builder() -> OpenRouterBuilder<M> {
        OpenRouterBuilder::default()
    }
}

impl OpenRouter<DynamicModel> {
    /// Creates an OpenRouter provider with a dynamic model name using default settings.
    ///
    /// This allows you to specify the model name as a string rather than
    /// using methods like `OpenRouter::anthropic_claude_3_5_sonnet()`, etc.
    ///
    /// **WARNING**: when using `DynamicModel`, model capabilities are not validated.
    /// This means there is no compile-time guarantee that the model supports requested features.
    ///
    /// For custom configuration (API key, base URL, etc.), use the builder pattern:
    /// `OpenRouter::<DynamicModel>::builder().model_name(...).api_key(...).build()`
    ///
    /// # Parameters
    ///
    /// * `model_name` - The OpenRouter model identifier (e.g., "anthropic/claude-3.5-sonnet")
    ///
    /// # Returns
    ///
    /// A configured `OpenRouter<DynamicModel>` provider instance with default settings.
    pub fn model_name(name: impl Into<String>) -> Self {
        let settings = OpenRouterProviderSettings::default();
        let inner = OpenAIChatCompletions::<DynamicModel>::model_name(name);

        OpenRouter { settings, inner }
    }
}

impl<M: ModelName> Default for OpenRouter<M> {
    /// Creates a new OpenRouter provider with default settings.
    fn default() -> OpenRouter<M> {
        OpenRouterBuilder::default().build().unwrap()
    }
}

/// OpenRouter provider builder
pub struct OpenRouterBuilder<M: ModelName> {
    settings: OpenRouterProviderSettings,
    inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> Default for OpenRouterBuilder<M> {
    /// Creates a new OpenRouter provider with default settings.
    fn default() -> Self {
        let settings = OpenRouterProviderSettings::default();
        let mut inner = OpenAIChatCompletions::default();
        inner.settings.provider_name = settings.provider_name.clone();
        inner.settings.base_url = settings.base_url.clone();
        inner.settings.api_key = settings.api_key.clone();

        Self { settings, inner }
    }
}

impl<M: ModelName> OpenRouterBuilder<M> {
    /// Sets the provider name for the OpenRouter provider.
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

    /// Sets the base URL for the OpenRouter provider.
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

    /// Sets the API key for the OpenRouter provider.
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

    /// Builds the OpenRouter provider.
    ///
    /// Validates the configuration and creates the provider instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configured `OpenRouter<M>` or an `Error`.
    pub fn build(mut self) -> Result<OpenRouter<M>> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        // Update the inner provider with the validated base_url
        self.inner.settings.base_url = base_url.to_string();
        self.settings.base_url = base_url.to_string();

        Ok(OpenRouter {
            settings: self.settings,
            inner: self.inner,
        })
    }
}

impl OpenRouterBuilder<DynamicModel> {
    /// Sets the model name from a string. e.g., "anthropic/claude-3.5-sonnet"
    ///
    /// **WARNING**: when using `DynamicModel`, model capabilities are not validated.
    /// This means there is no compile-time guarantee that the model supports requested features.
    ///
    /// For compile-time model validation, use the constructor methods like `OpenRouter::anthropic_claude_3_5_sonnet()`.
    ///
    /// # Parameters
    ///
    /// * `model_name` - The OpenRouter model identifier (e.g., "anthropic/claude-3.5-sonnet")
    ///
    /// # Returns
    ///
    /// The builder with the model name set.
    pub fn model_name(mut self, model_name: impl Into<String>) -> Self {
        self.inner.options.model = model_name.into();
        self
    }
}

// Re-exports Models for convenience
pub use capabilities::*;
