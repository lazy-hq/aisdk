//! This module provides the Amazon Bedrock provider, wrapping OpenAI Chat Completions for Bedrock requests.

pub mod capabilities;
pub mod language_model;
pub mod settings;

use crate::Error;
use crate::core::DynamicModel;
use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Result;
use crate::providers::amazon_bedrock::settings::AmazonBedrockProviderSettings;
use crate::providers::openai_chat_completions::OpenAIChatCompletions;

/// The Amazon Bedrock provider, wrapping OpenAI Chat Completions API.
#[derive(Debug, Clone)]
pub struct AmazonBedrock<M: ModelName> {
    /// Configuration settings for the Amazon Bedrock provider.
    pub settings: AmazonBedrockProviderSettings,
    pub(crate) inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> AmazonBedrock<M> {
    /// Amazon Bedrock provider setting builder.
    pub fn builder() -> AmazonBedrockBuilder<M> {
        AmazonBedrockBuilder::default()
    }
}

impl AmazonBedrock<DynamicModel> {
    /// Creates an Amazon Bedrock provider with a dynamic model name using default settings.
    ///
    /// This allows you to specify the model name as a string rather than
    /// using methods like `AmazonBedrock::anthropic_claude_3_5_sonnet_v1_0()`, etc.
    ///
    /// **WARNING**: when using `DynamicModel`, model capabilities are not validated.
    /// This means there is no compile-time guarantee that the model supports requested features.
    ///
    /// For custom configuration (API key, base URL, etc.), use the builder pattern:
    /// `AmazonBedrock::<DynamicModel>::builder().model_name(...).api_key(...).build()`
    ///
    /// # Parameters
    ///
    /// * `model_name` - The Amazon Bedrock model identifier (e.g., "anthropic.claude-3-5-sonnet-20241022-v2:0")
    ///
    /// # Returns
    ///
    /// A configured `AmazonBedrock<DynamicModel>` provider instance with default settings.
    pub fn model_name(name: impl Into<String>) -> Self {
        let settings = AmazonBedrockProviderSettings::default();
        let inner = OpenAIChatCompletions::<DynamicModel>::model_name(name);

        AmazonBedrock { settings, inner }
    }
}

impl<M: ModelName> Default for AmazonBedrock<M> {
    /// Creates a new Amazon Bedrock provider with default settings.
    fn default() -> AmazonBedrock<M> {
        AmazonBedrockBuilder::default().build().unwrap()
    }
}

/// Amazon Bedrock provider builder
pub struct AmazonBedrockBuilder<M: ModelName> {
    settings: AmazonBedrockProviderSettings,
    inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> Default for AmazonBedrockBuilder<M> {
    /// Creates a new Amazon Bedrock provider with default settings.
    fn default() -> Self {
        let settings = AmazonBedrockProviderSettings::default();
        let mut inner = OpenAIChatCompletions::default();
        inner.settings.provider_name = settings.provider_name.clone();
        inner.settings.base_url = settings.base_url.clone();
        inner.settings.api_key = settings.api_key.clone();

        Self { settings, inner }
    }
}

impl<M: ModelName> AmazonBedrockBuilder<M> {
    /// Sets the provider name for the Amazon Bedrock provider.
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

    /// Sets the base URL for the Amazon Bedrock provider.
    ///
    /// # Parameters
    ///
    /// * `base_url` - The base URL string for API requests.
    ///   Format: https://bedrock-runtime.{region}.amazonaws.com/openai/
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

    /// Sets the API key for the Amazon Bedrock provider.
    ///
    /// # Parameters
    ///
    /// * `api_key` - The API key string for authentication (AWS Bearer Token).
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

    /// Builds the Amazon Bedrock provider.
    ///
    /// Validates the configuration and creates the provider instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configured `AmazonBedrock<M>` or an `Error`.
    pub fn build(mut self) -> Result<AmazonBedrock<M>> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        // Update the inner provider with the validated base_url
        self.inner.settings.base_url = base_url.to_string();
        self.settings.base_url = base_url.to_string();

        Ok(AmazonBedrock {
            settings: self.settings,
            inner: self.inner,
        })
    }
}

impl AmazonBedrockBuilder<DynamicModel> {
    /// Sets the model name from a string. e.g., "anthropic.claude-3-5-sonnet-20241022-v2:0"
    ///
    /// **WARNING**: when using `DynamicModel`, model capabilities are not validated.
    /// This means there is no compile-time guarantee that the model supports requested features.
    ///
    /// For compile-time model validation, use the constructor methods like `AmazonBedrock::anthropic_claude_3_5_sonnet_v1_0()`.
    ///
    /// # Parameters
    ///
    /// * `model_name` - The Amazon Bedrock model identifier (e.g., "anthropic.claude-3-5-sonnet-20241022-v2:0")
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
