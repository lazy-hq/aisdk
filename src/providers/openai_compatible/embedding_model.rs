//! Embedding model implementation for the OpenAI-compatible provider.

use async_trait::async_trait;

use crate::{
    Result,
    core::{
        capabilities::ModelName,
        embedding_model::{EmbeddingModel, EmbeddingModelOptions, EmbeddingModelResponse},
    },
    providers::openai::OpenAI,
    providers::openai_compatible::OpenAICompatible,
};

#[async_trait]
impl<M: ModelName> EmbeddingModel for OpenAICompatible<M> {
    async fn embed(&self, input: EmbeddingModelOptions) -> Result<EmbeddingModelResponse> {
        // Create an OpenAI provider with the same settings as OpenAICompatible
        let openai_provider = OpenAI::<M> {
            settings: crate::providers::openai::settings::OpenAIProviderSettings {
                base_url: self.inner.settings.base_url.clone(),
                api_key: self.inner.settings.api_key.clone(),
                provider_name: self.inner.settings.provider_name.clone(),
                path: self.inner.settings.path.clone(),
                body: self.inner.settings.body.clone(),
                headers: self.inner.settings.headers.clone(),
            },
            lm_options: Default::default(),
            embedding_options: crate::providers::openai::client::OpenAIEmbeddingOptions {
                input: vec![],
                model: self.inner.options.model.clone(),
                user: None,
                dimensions: input.dimensions,
                encoding_format: None,
                extra_body: None,
                extra_headers: None,
            },
            _phantom: std::marker::PhantomData,
        };

        // Delegate to OpenAI's embedding implementation
        openai_provider.embed(input).await
    }
}
