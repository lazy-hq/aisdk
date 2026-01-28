//! This module provides the Together AI provider, wrapping OpenAI Chat Completions for Together AI requests.

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
use crate::providers::togetherai::settings::TogetherAIProviderSettings;

/// The Together AI provider, wrapping OpenAI Chat Completions API.
#[derive(Debug, Clone)]
pub struct TogetherAI<M: ModelName> {
    /// Configuration settings for the Together AI provider.
    pub settings: TogetherAIProviderSettings,
    pub(crate) inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> TogetherAI<M> {
    /// Together AI provider setting builder.
    pub fn builder() -> TogetherAIBuilder<M> {
        TogetherAIBuilder::default()
    }
}

impl TogetherAI<DynamicModel> {
    /// Creates a Together AI provider with a dynamic model name using default settings.
    ///
    /// This allows you to specify the model name as a string rather than
    /// using methods like `TogetherAI::llama_3_3_70b_spec_dec()`, etc.
    ///
    /// **WARNING**: when using `DynamicModel`, model capabilities are not validated.
    /// This means there is no compile-time guarantee that the model supports requested features.
    ///
    /// For custom configuration (API key, base URL, etc.), use the builder pattern:
    /// `TogetherAI::<DynamicModel>::builder().model_name(...).api_key(...).build()`
    ///
    /// # Parameters
    ///
    /// * `model_name` - The Together AI model identifier (e.g., "Llama-3.3-70B-Instruct-Turbo")
    ///
    /// # Returns
    ///
    /// A configured `TogetherAI<DynamicModel>` provider instance with default settings.
    pub fn model_name(name: impl Into<String>) -> Self {
        let settings = TogetherAIProviderSettings::default();
        let inner = OpenAIChatCompletions::<DynamicModel>::model_name(name);

        TogetherAI { settings, inner }
    }
}

impl<M: ModelName> Default for TogetherAI<M> {
    /// Creates a new Together AI provider with default settings.
    fn default() -> TogetherAI<M> {
        TogetherAIBuilder::default().build().unwrap()
    }
}

/// Together AI provider builder
pub struct TogetherAIBuilder<M: ModelName> {
    settings: TogetherAIProviderSettings,
    inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> Default for TogetherAIBuilder<M> {
    /// Creates a new Together AI provider with default settings.
    fn default() -> Self {
        let settings = TogetherAIProviderSettings::default();
        let mut inner = OpenAIChatCompletions::default();
        inner.settings.provider_name = settings.provider_name.clone();
        inner.settings.base_url = settings.base_url.clone();
        inner.settings.api_key = settings.api_key.clone();

        Self { settings, inner }
    }
}

impl<M: ModelName> TogetherAIBuilder<M> {
    /// Sets the provider name for the Together AI provider.
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

    /// Sets the base URL for the Together AI provider.
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

    /// Sets the API key for the Together AI provider.
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

    /// Builds the Together AI provider.
    ///
    /// Validates the configuration and creates the provider instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configured `TogetherAI<M>` or an `Error`.
    pub fn build(mut self) -> Result<TogetherAI<M>> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        // Update the inner provider with the validated base_url
        self.inner.settings.base_url = base_url.to_string();
        self.settings.base_url = base_url.to_string();

        Ok(TogetherAI {
            settings: self.settings,
            inner: self.inner,
        })
    }
}

impl TogetherAIBuilder<DynamicModel> {
    /// Sets the model name from a string. e.g., "Llama-3.3-70B-Instruct-Turbo"
    ///
    /// **WARNING**: when using `DynamicModel`, model capabilities are not validated.
    /// This means there is no compile-time guarantee that the model supports requested features.
    ///
    /// For compile-time model validation, use the constructor methods like `TogetherAI::llama_3_3_70b_spec_dec()`.
    ///
    /// # Parameters
    ///
    /// * `model_name` - The Together AI model identifier (e.g., "Llama-3.3-70B-Instruct-Turbo")
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
