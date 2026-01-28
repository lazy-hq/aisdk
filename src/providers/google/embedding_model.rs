//! Embedding model implementation for the Google provider.

use crate::{
    core::{
        capabilities::ModelName,
        client::EmbeddingClient,
        embedding_model::{EmbeddingModel, EmbeddingModelOptions, EmbeddingModelResponse},
    },
    error::Result,
    providers::google::Google,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
/// Settings for Google that are specific to embedding models.
///
/// This struct is a placeholder for future embedding-specific configuration options.
/// Currently, embedding configuration is handled directly through `GoogleEmbeddingOptions`
/// in the client layer, but this struct exists to maintain API consistency with the
/// `LanguageModel` pattern and to provide a location for embedding-specific settings
/// if they are added in the future.
pub struct GoogleEmbeddingModelOptions {}

#[async_trait]
impl<M: ModelName> EmbeddingModel for Google<M> {
    async fn embed(&self, input: EmbeddingModelOptions) -> Result<EmbeddingModelResponse> {
        // Clone self to allow mutation
        let mut model = self.clone();

        // Convert input to Google embedding options
        let mut options: crate::providers::google::client::GoogleEmbeddingOptions = input.into();

        let embedding_model = model.embedding_options.model.clone();

        // Set the model name from the current model
        options.model = embedding_model.clone();

        // Set the model name inside parts
        let _ = options.requests.iter_mut().for_each(|r| {
            r.model = format!("models/{}", embedding_model.clone());
        });

        // Update the model's embedding options
        model.embedding_options = options;

        // Send the request
        let response = model.send(&model.settings.base_url).await?;

        // Extract embeddings from response
        Ok(response.embeddings.into_iter().map(|e| e.values).collect())
    }
}
