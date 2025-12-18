//! This module provides the Anthropic provider, which implements the `LanguageModel`
//! and `Provider` traits for interacting with the Anthropic API.

pub mod capabilities;
/// Client implementation for Anthropic API.
pub mod client;
/// Conversion utilities for Anthropic types.
pub mod conversions;
pub mod langauge_model;
pub mod settings;

use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Error;
use crate::providers::anthropic::client::AnthropicOptions;
use crate::providers::anthropic::settings::AnthropicProviderSettings;
use serde::Serialize;

/// The API version used for Anthropic requests.
pub const ANTHROPIC_API_VERSION: &str = "2023-06-01";

/// The Anthropic provider.
#[derive(Debug, Serialize, Clone)]
pub struct Anthropic<M: ModelName> {
    /// Configuration settings for the Anthropic provider.
    pub settings: AnthropicProviderSettings,
    options: AnthropicOptions,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: ModelName> Anthropic<M> {
    /// Anthropic provider setting builder.
    pub fn builder() -> AnthropicBuilder<M> {
        AnthropicBuilder::default()
    }
}

impl<M: ModelName> Default for Anthropic<M> {
    /// Creates a new AnthropAI provider with default settings.
    fn default() -> Self {
        let settings = AnthropicProviderSettings::default();
        let options = AnthropicOptions::builder()
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

/// Anthropic Provider Builder
pub struct AnthropicBuilder<M: ModelName> {
    settings: AnthropicProviderSettings,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: ModelName> Default for AnthropicBuilder<M> {
    /// Creates a new AnthropAI provider with default settings.
    fn default() -> Self {
        let settings = AnthropicProviderSettings::default();

        Self {
            settings,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<M: ModelName> AnthropicBuilder<M> {
    /// Sets the base URL for the Anthropic API.
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

    /// Sets the API key for the Anthropic API.
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

    /// Sets the name of the provider. Defaults to "anthropic".
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

    /// Builds the Anthropic provider.
    ///
    /// Validates the configuration and creates the provider instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configured `Anthropic` provider or an `Error`.
    pub fn build(self) -> Result<Anthropic<M>, Error> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        let options = AnthropicOptions::builder()
            .model(M::MODEL_NAME.to_string())
            .build()
            .unwrap();

        Ok(Anthropic {
            settings: AnthropicProviderSettings {
                base_url,
                ..self.settings
            },
            options,
            _phantom: std::marker::PhantomData,
        })
    }
}

// Re-exports for convenience
pub use capabilities::*;
