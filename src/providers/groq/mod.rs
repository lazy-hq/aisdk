//! This module provides the Groq provider, wrapping OpenAI for Groq branding.

pub mod capabilities;
pub mod settings;

use crate::core::capabilities::ModelName;
use crate::core::language_model::{
    LanguageModel, LanguageModelOptions, LanguageModelResponse, ProviderStream,
};
use crate::error::Result;
use crate::providers::groq::settings::{GroqProviderSettings, GroqProviderSettingsBuilder};
use crate::providers::openai::OpenAI;
use async_trait::async_trait;

/// The Groq provider, wrapping OpenAI.
#[derive(Debug, Clone)]
pub struct Groq<M: ModelName> {
    inner: OpenAI<M>,
}

impl<M: ModelName> Groq<M> {
    /// Creates a new Groq provider with default settings.
    pub fn default() -> Groq<M> {
        Groq {
            inner: OpenAI::<M>::default(),
        }
    }

    /// Groq provider setting builder.
    pub fn builder() -> GroqProviderSettingsBuilder<M> {
        GroqProviderSettings::builder()
    }
}

#[async_trait]
impl<M: ModelName> LanguageModel for Groq<M> {
    /// Returns the name of the model.
    fn name(&self) -> String {
        self.inner.name()
    }

    /// Generates text using the Groq provider.
    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        self.inner.generate_text(options).await
    }

    /// Streams text using the Groq provider.
    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        self.inner.stream_text(options).await
    }
}
