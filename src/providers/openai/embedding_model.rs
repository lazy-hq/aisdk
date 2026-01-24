//! Embedding model implementation for the OpenAI provider.

use crate::{
    core::{
        capabilities::ModelName,
        client::EmbeddingClient,
        embedding_model::{EmbeddingModel, EmbeddingModelOptions, EmbeddingModelResponse},
    },
    providers::openai::OpenAI,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
/// Settings for OpenAI that are specific to embedding models.
pub struct OpenAIEmbeddingModelOptions {}

#[async_trait]
impl<M: ModelName> EmbeddingModel for OpenAI<M> {
    async fn embed(&self, input: EmbeddingModelOptions) -> EmbeddingModelResponse {
        // Clone self to allow mutation
        let mut model = self.clone();

        // Convert input to OpenAI embedding options
        let mut options: crate::providers::openai::client::OpenAIEmbeddingOptions = input.into();

        // Set the model name from the current model
        options.model = model.embedding_options.model.clone();

        // Update the model's embedding options
        model.embedding_options = options;

        // Send the request
        let response = model.send(&model.settings.base_url).await.unwrap();

        // Extract embeddings from response
        response.data.into_iter().map(|e| e.embedding).collect()
    }
}
