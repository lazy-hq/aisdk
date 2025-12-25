//! This module provides the Google provider, which implements the `LanguageModel`
//! and `Provider` traits for interacting with the Google API.

pub mod capabilities;
pub mod client;
pub mod conversions;
pub mod extensions;
pub mod language_model;
pub mod settings;

use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Error;
use crate::providers::google::client::GoogleOptions;
use crate::providers::google::settings::GoogleProviderSettings;
use serde::Serialize;

/// The Google provider.
#[derive(Debug, Serialize, Clone)]
pub struct Google<M: ModelName> {
    /// Configuration settings for the Google provider.
    pub settings: GoogleProviderSettings,
    options: GoogleOptions,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: ModelName> Google<M> {
    /// Google provider setting builder.
    pub fn builder() -> GoogleBuilder<M> {
        GoogleBuilder::default()
    }
}

impl<M: ModelName> Default for Google<M> {
    /// Creates a new Google provider with default settings.
    fn default() -> Self {
        let settings = GoogleProviderSettings::default();
        let options = GoogleOptions::builder()
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

/// Google Provider Builder
pub struct GoogleBuilder<M: ModelName> {
    settings: GoogleProviderSettings,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: ModelName> Default for GoogleBuilder<M> {
    /// Creates a new Google provider with default settings.
    fn default() -> Self {
        let settings = GoogleProviderSettings::default();

        Self {
            settings,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<M: ModelName> GoogleBuilder<M> {
    /// Sets the base URL for the Google API.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.settings.base_url = base_url.into();
        self
    }

    /// Sets the API key for the Google API.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.settings.api_key = api_key.into();
        self
    }

    /// Sets the name of the provider. Defaults to "google".
    pub fn provider_name(mut self, provider_name: impl Into<String>) -> Self {
        self.settings.provider_name = provider_name.into();
        self
    }

    /// Builds the Google provider settings.
    pub fn build(self) -> Result<Google<M>, Error> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        let options = GoogleOptions::builder()
            .model(M::MODEL_NAME.to_string())
            .build()
            .unwrap();

        Ok(Google {
            settings: GoogleProviderSettings {
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
