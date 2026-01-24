//! OpenAI provider implementation.

pub mod capabilities;
pub mod client;
pub mod conversions;
pub mod embedding_model;
pub mod language_model;
pub mod settings;

use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Error;
use crate::providers::openai::client::{OpenAIEmbeddingOptions, OpenAILanguageModelOptions};
use crate::providers::openai::settings::OpenAIProviderSettings;

/// The OpenAI provider.
#[derive(Debug, Clone)]
pub struct OpenAI<M: ModelName> {
    /// Configuration settings for the OpenAI provider.
    pub settings: OpenAIProviderSettings,
    /// Options for Language Model
    pub(crate) lm_options: OpenAILanguageModelOptions,
    /// Options for Embedding Model
    pub(crate) embedding_options: OpenAIEmbeddingOptions,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: ModelName> OpenAI<M> {
    /// OpenAI provider setting builder.
    pub fn builder() -> OpenAIBuilder<M> {
        OpenAIBuilder::default()
    }
}

impl<M: ModelName> Default for OpenAI<M> {
    /// Creates a new OpenAI provider with default settings.
    fn default() -> Self {
        let settings = OpenAIProviderSettings::default();
        let lm_options = OpenAILanguageModelOptions::builder()
            .model(M::MODEL_NAME.to_string())
            .build()
            .unwrap();

        let embedding_options = OpenAIEmbeddingOptions {
            input: vec![],
            model: M::MODEL_NAME.to_string(),
            user: None,
            dimensions: None,
            encoding_format: None,
        };

        Self {
            settings,
            lm_options,
            embedding_options,
            _phantom: std::marker::PhantomData,
        }
    }
}

/// OpenAI Provider Builder
pub struct OpenAIBuilder<M: ModelName> {
    settings: OpenAIProviderSettings,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: ModelName> Default for OpenAIBuilder<M> {
    /// Creates a new OpenAI provider builder with default settings.
    fn default() -> Self {
        let settings = OpenAIProviderSettings::default();

        Self {
            settings,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<M: ModelName> OpenAIBuilder<M> {
    /// Sets the base URL for the OpenAI API.
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

    /// Sets the API key for the OpenAI API.
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

    /// Sets the name of the provider. Defaults to "openai".
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

    /// Builds the OpenAI provider.
    ///
    /// Validates the configuration and creates the provider instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configured `OpenAI` provider or an `Error`.
    pub fn build(self) -> Result<OpenAI<M>, Error> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        let lm_options = OpenAILanguageModelOptions::builder()
            .model(M::MODEL_NAME.to_string())
            .build()
            .unwrap();

        let embedding_options = OpenAIEmbeddingOptions {
            input: vec![],
            model: M::MODEL_NAME.to_string(),
            user: None,
            dimensions: None,
            encoding_format: None,
        };

        Ok(OpenAI {
            settings: OpenAIProviderSettings {
                base_url,
                ..self.settings
            },
            lm_options,
            embedding_options,
            _phantom: std::marker::PhantomData,
        })
    }
}

// Re-exports Models for convenience
pub use capabilities::*;
