//! Language model implementation for the OpenRouter provider.

use async_trait::async_trait;

use crate::{
    Result,
    core::{
        LanguageModel,
        capabilities::ModelName,
        language_model::{LanguageModelOptions, LanguageModelResponse, ProviderStream},
    },
    providers::openrouter::OpenRouter,
};

#[async_trait]
impl<M: ModelName> LanguageModel for OpenRouter<M> {
    /// Returns the name of the model.
    fn name(&self) -> String {
        self.inner.name()
    }

    /// Generates text using the OpenRouter provider.
    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        self.inner.generate_text(options).await
    }

    /// Streams text using the OpenRouter provider.
    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        self.inner.stream_text(options).await
    }
}
