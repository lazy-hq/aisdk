//! Language model implementation for the Groq provider.

use async_trait::async_trait;

use crate::{
    Result,
    core::{
        LanguageModel,
        capabilities::ModelName,
        language_model::{LanguageModelOptions, LanguageModelResponse, ProviderStream},
    },
    providers::groq::Groq,
};

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
