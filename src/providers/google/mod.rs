//! This module provides the Google provider, which implements the `LanguageModel`
//! and `Provider` traits for interacting with the Google API.

pub mod capabilities;
pub mod client;
pub mod conversions;
pub mod embedding_model;
pub mod extensions;
pub mod language_model;
pub mod settings;

use crate::core::DynamicModel;
use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Error;
use crate::providers::google::client::{GoogleEmbeddingOptions, GoogleOptions};
use crate::providers::google::settings::GoogleProviderSettings;
use serde::Serialize;

/// The Google provider.
#[derive(Debug, Serialize, Clone)]
pub struct Google<M: ModelName> {
    /// Configuration settings for the Google provider.
    pub settings: GoogleProviderSettings,
    pub(crate) lm_options: GoogleOptions,
    pub(crate) embedding_options: GoogleEmbeddingOptions,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: ModelName> Google<M> {
    /// Google provider setting builder.
    pub fn builder() -> GoogleBuilder<M> {
        GoogleBuilder::default()
    }
}

impl Google<DynamicModel> {
    /// Creates a Google provider with a dynamic model name using default settings.
    ///
    /// This allows you to specify the model name as a string rather than
    /// using methods like `Google::gemini_1_5_pro()`, etc.
    ///
    /// **WARNING**: when using `DynamicModel`, model capabilities are not validated.
    /// This means there is no compile-time guarantee that the model supports requested features.
    ///
    /// For custom configuration (API key, base URL, etc.), use the builder pattern:
    /// `Google::<DynamicModel>::builder().model_name(...).api_key(...).build()`
    ///
    /// # Parameters
    ///
    /// * `model_name` - The Google model identifier (e.g., "gemini-1.5-pro", "gemini-1.5-flash")
    ///
    /// # Returns
    ///
    /// A configured `Google<DynamicModel>` provider instance with default settings.
    pub fn model_name(name: impl Into<String>) -> Self {
        let settings = GoogleProviderSettings::default();
        let model_name = name.into();
        let options = GoogleOptions::builder()
            .model(model_name.clone())
            .build()
            .unwrap();
        let embedding_options = GoogleEmbeddingOptions {
            model: model_name.clone(),
            requests: Vec::new(),
        };

        Self {
            settings,
            lm_options: options,
            embedding_options,
            _phantom: std::marker::PhantomData,
        }
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
        let embedding_options = GoogleEmbeddingOptions {
            model: M::MODEL_NAME.to_string(),
            requests: Vec::new(),
        };

        Self {
            settings,
            lm_options: options,
            embedding_options,
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Google Provider Builder
pub struct GoogleBuilder<M: ModelName> {
    settings: GoogleProviderSettings,
    options: GoogleOptions,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: ModelName> Default for GoogleBuilder<M> {
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

impl GoogleBuilder<DynamicModel> {
    /// Sets the model name from a string. e.g., "gemini-1.5-pro", "gemini-1.5-flash"
    ///
    /// **WARNING**: when using `DynamicModel`, model capabilities are not validated.
    /// This means there is no compile-time guarantee that the model supports requested features.
    ///
    /// For compile-time model validation, use the constructor methods like `Google::gemini_1_5_pro()`.
    ///
    /// # Parameters
    ///
    /// * `model_name` - The Google model identifier (e.g., "gemini-1.5-pro", "gemini-1.5-flash")
    ///
    /// # Returns
    ///
    /// The builder with the model name set.
    pub fn model_name(mut self, model_name: impl Into<String>) -> Self {
        self.options.model = model_name.into();
        self
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

    /// Sets a custom API path, overriding the default dynamic path.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.settings.path = Some(path.into());
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

        let options = self.options;
        let model_name = options.model.clone();
        let embedding_options = GoogleEmbeddingOptions {
            model: model_name,
            requests: Vec::new(),
        };

        Ok(Google {
            settings: GoogleProviderSettings {
                base_url,
                ..self.settings
            },
            lm_options: options,
            embedding_options,
            _phantom: std::marker::PhantomData,
        })
    }
}

// Re-exports for convenience
pub use capabilities::*;
