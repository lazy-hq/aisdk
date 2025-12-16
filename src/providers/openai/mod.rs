//! OpenAI provider implementation.

pub mod capabilities;
pub mod client;
pub mod conversions;
pub mod language_model;
pub mod settings;

use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Error;
use crate::providers::openai::client::OpenAIOptions;
use crate::providers::openai::settings::OpenAIProviderSettings;

/// The OpenAI provider.
#[derive(Debug, Clone)]
pub struct OpenAI<M: ModelName> {
    pub settings: OpenAIProviderSettings,
    options: OpenAIOptions,
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
        let options = OpenAIOptions::builder()
            .model(M::MODEL_NAME.to_string())
            .build()
            .unwrap();

        Self {
            settings,
            options,
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
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.settings.base_url = base_url.into();
        self
    }

    /// Sets the API key for the OpenAI API.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.settings.api_key = api_key.into();
        self
    }

    /// Sets the name of the provider. Defaults to "openai".
    pub fn provider_name(mut self, provider_name: impl Into<String>) -> Self {
        self.settings.provider_name = provider_name.into();
        self
    }

    /// Builds the OpenAI provider settings.
    pub fn build(self) -> Result<OpenAI<M>, Error> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        let options = OpenAIOptions::builder()
            .model(M::MODEL_NAME.to_string())
            .build()
            .unwrap();

        Ok(OpenAI {
            settings: OpenAIProviderSettings {
                base_url,
                ..self.settings
            },
            options,
            _phantom: std::marker::PhantomData,
        })
    }
}

// Re-exports Models for convenience
pub use capabilities::*;
