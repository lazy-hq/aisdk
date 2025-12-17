//! This module provides the Groq provider, wrapping OpenAI for Groq requests.

pub mod capabilities;
pub mod language_model;
pub mod settings;

use crate::Error;
use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Result;
use crate::providers::groq::settings::GroqProviderSettings;
use crate::providers::openai::OpenAI;

/// The Groq provider, wrapping OpenAI.
#[derive(Debug, Clone)]
pub struct Groq<M: ModelName> {
    pub settings: GroqProviderSettings,
    inner: OpenAI<M>,
}

impl<M: ModelName> Groq<M> {
    /// Groq provider setting builder.
    pub fn builder() -> GroqBuilder<M> {
        GroqBuilder::default()
    }
}

impl<M: ModelName> Default for Groq<M> {
    /// Creates a new Groq provider with default settings.
    fn default() -> Groq<M> {
        GroqBuilder::default().build().unwrap()
    }
}

/// Groq provider builder
pub struct GroqBuilder<M: ModelName> {
    settings: GroqProviderSettings,
    inner: OpenAI<M>,
}

impl<M: ModelName> Default for GroqBuilder<M> {
    /// Creates a new Groq provider with default settings.
    fn default() -> Self {
        let settings = GroqProviderSettings::default();
        let mut inner = OpenAI::default();
        inner.settings.provider_name = settings.provider_name.clone();
        inner.settings.base_url = settings.base_url.clone();
        inner.settings.api_key = settings.api_key.clone();

        Self { settings, inner }
    }
}

impl<M: ModelName> GroqBuilder<M> {
    /// set the provider name for the Groq provider
    pub fn provider_name(mut self, provider_name: impl Into<String>) -> Self {
        self.settings.provider_name = provider_name.into();
        self
    }

    /// set the base url for the Groq provider
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.settings.base_url = base_url.into();
        self
    }

    /// set the api key for the Groq provider
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.settings.api_key = api_key.into();
        self
    }

    /// build the Groq provider
    pub fn build(self) -> Result<Groq<M>> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        Ok(Groq {
            settings: GroqProviderSettings {
                base_url: base_url.to_string(),
                ..self.settings
            },
            inner: self.inner,
        })
    }
}

// Re-exports Models for convenience
pub use capabilities::*;
