//! Embedding model implementation for OpenAI Chat Completions API.

use crate::{
    core::{
        capabilities::ModelName,
        client::EmbeddingClient,
        embedding_model::{EmbeddingModel, EmbeddingModelOptions, EmbeddingModelResponse},
    },
    error::Result,
    providers::openai_chat_completions::OpenAIChatCompletions,
};
use async_trait::async_trait;

use super::client::types::{EmbeddingOptions, EmbeddingResponse};

/// Implement EmbeddingClient trait for OpenAIChatCompletions
impl<M: ModelName> EmbeddingClient for OpenAIChatCompletions<M> {
    type Response = EmbeddingResponse;

    fn path(&self) -> String {
        "embeddings".to_string()
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.settings.api_key).parse().unwrap(),
        );
        headers
    }

    fn query_params(&self) -> Vec<(&str, &str)> {
        Vec::new()
    }

    fn body(&self) -> reqwest::Body {
        // This will be set when embedding is called
        reqwest::Body::from("") // Placeholder, will be replaced
    }
}

impl<M: ModelName> OpenAIChatCompletions<M> {
    /// Creates an embedding request body from options.
    fn create_embedding_body(&self, input: EmbeddingModelOptions) -> Result<EmbeddingOptions> {
        Ok(EmbeddingOptions {
            input: input.input,
            model: self.options.model.clone(),
            user: None,
            dimensions: input.dimensions,
            encoding_format: None,
        })
    }

    /// Embeds the given input using the OpenAI Embeddings API.
    pub async fn embed(&self, input: EmbeddingModelOptions) -> Result<EmbeddingModelResponse> {
        let embedding_options = self.create_embedding_body(input)?;

        // Create a temporary client instance for this request
        let embedding_client = EmbeddingClientWrapper {
            settings: self.settings.clone(),
            options: embedding_options,
        };

        let response = embedding_client.send(&self.settings.base_url).await?;

        // Extract embeddings from response
        Ok(response.data.into_iter().map(|e| e.embedding).collect())
    }
}

/// Temporary wrapper for embedding requests.
struct EmbeddingClientWrapper {
    settings: super::settings::OpenAIChatCompletionsSettings,
    options: EmbeddingOptions,
}

impl EmbeddingClient for EmbeddingClientWrapper {
    type Response = EmbeddingResponse;

    fn path(&self) -> String {
        "embeddings".to_string()
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", self.settings.api_key).parse().unwrap(),
        );
        headers
    }

    fn query_params(&self) -> Vec<(&str, &str)> {
        Vec::new()
    }

    fn body(&self) -> reqwest::Body {
        let body = serde_json::to_string(&self.options).unwrap();
        reqwest::Body::from(body)
    }
}

#[async_trait]
impl<M: ModelName> EmbeddingModel for OpenAIChatCompletions<M> {
    async fn embed(&self, input: EmbeddingModelOptions) -> Result<EmbeddingModelResponse> {
        self.embed(input).await
    }
}
