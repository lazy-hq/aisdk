//! Embedding model implementation for the Together AI provider.

use async_trait::async_trait;

use crate::{
    Result,
    core::{
        capabilities::ModelName,
        embedding_model::{EmbeddingModel, EmbeddingModelOptions, EmbeddingModelResponse},
    },
    providers::openai::OpenAI,
    providers::togetherai::TogetherAI,
};

#[async_trait]
impl<M: ModelName> EmbeddingModel for TogetherAI<M> {
    async fn embed(&self, input: EmbeddingModelOptions) -> Result<EmbeddingModelResponse> {
        // Create an OpenAI provider with the same settings as Together AI
        let openai_provider = OpenAI::<M> {
            settings: crate::providers::openai::settings::OpenAIProviderSettings {
                base_url: self.settings.base_url.clone(),
                api_key: self.settings.api_key.clone(),
                provider_name: self.settings.provider_name.clone(),
            },
            lm_options: Default::default(),
            embedding_options: crate::providers::openai::client::OpenAIEmbeddingOptions {
                input: vec![],
                model: self.inner.options.model.clone(),
                user: None,
                dimensions: None,
                encoding_format: None,
            },
            _phantom: std::marker::PhantomData,
        };

        // Delegate to OpenAI's embedding implementation
        openai_provider.embed(input).await
    }
}
