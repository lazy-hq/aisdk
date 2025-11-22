//! This module provides the Groq provider, wrapping OpenAI for Groq branding.

pub mod settings;

use crate::core::language_model::{
    LanguageModel, LanguageModelOptions, LanguageModelResponse, ProviderStream,
};
use crate::core::provider::Provider;
use crate::error::Result;
use crate::providers::groq::settings::{GroqProviderSettings, GroqProviderSettingsBuilder};
use crate::providers::openai::OpenAI;
use async_trait::async_trait;

/// The Groq provider, wrapping OpenAI.
#[derive(Debug, Clone)]
pub struct Groq {
    inner: OpenAI,
}

impl Groq {
    /// Creates a new Groq provider with default settings.
    pub fn new(model_name: impl Into<String>) -> Self {
        GroqProviderSettingsBuilder::default()
            .model_name(model_name.into())
            .build()
            .expect("Failed to build GroqProviderSettings")
    }

    /// Groq provider setting builder.
    pub fn builder() -> GroqProviderSettingsBuilder {
        GroqProviderSettings::builder()
    }
}

impl Provider for Groq {}

#[async_trait]
impl LanguageModel for Groq {
    fn name(&self) -> String {
        self.inner.name()
    }

    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        self.inner.generate_text(options).await
    }

    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        self.inner.stream_text(options).await
    }
}
