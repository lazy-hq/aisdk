//! Embedding model implementation for the OpenAI provider.

use crate::{
    core::{
        capabilities::ModelName,
        client::EmbeddingClient,
        embedding_model::{EmbeddingModel, EmbeddingModelOptions, EmbeddingModelResponse},
    },
    error::Result,
    providers::openai::OpenAI,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
/// Settings for OpenAI that are specific to embedding models.
///
/// This struct is a placeholder for future embedding-specific configuration options.
/// Currently, embedding configuration is handled directly through `OpenAIEmbeddingOptions`
/// in the client layer, but this struct exists to maintain API consistency with the
/// `LanguageModel` pattern and to provide a location for embedding-specific settings
/// if they are added in the future.
pub struct OpenAIEmbeddingModelOptions {}

#[async_trait]
impl<M: ModelName> EmbeddingModel for OpenAI<M> {
    async fn embed(&self, input: EmbeddingModelOptions) -> Result<EmbeddingModelResponse> {
        // Clone self to allow mutation
        let mut model = self.clone();

        // Convert input to OpenAI embedding options
        let mut options: crate::providers::openai::client::OpenAIEmbeddingOptions = input.into();

        // Set the model name from the current model
        options.model = model.embedding_options.model.clone();

        // Update the model's embedding options
        model.embedding_options = options;

        // Send the request
        let response = model.send(&model.settings.base_url).await?;

        // Extract embeddings from response
        Ok(response.data.into_iter().map(|e| e.embedding).collect())
    }
}
