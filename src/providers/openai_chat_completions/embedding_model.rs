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
        let headers = input.headers.clone();
        let embedding_options = self.create_embedding_body(input)?;

        // Create a temporary client instance for this request
        let embedding_client = EmbeddingClientWrapper {
            settings: self.settings.clone(),
            options: embedding_options,
        };

        let response = embedding_client
            .send(&self.settings.base_url, headers)
            .await?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{DynamicModel, EmbeddingModelRequest};
    use serde_json::json;
    use std::collections::HashMap;
    use wiremock::matchers::{body_partial_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_model(base_url: String) -> OpenAIChatCompletions<DynamicModel> {
        let mut model = OpenAIChatCompletions::<DynamicModel>::model_name("text-embedding-3-small");
        model.settings.base_url = base_url;
        model.settings.api_key = "test-key".to_string();
        model
    }

    fn embedding_response() -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(json!({
            "object": "list",
            "data": [
                {
                    "object": "embedding",
                    "index": 0,
                    "embedding": [0.1, 0.2]
                },
                {
                    "object": "embedding",
                    "index": 1,
                    "embedding": [0.3, 0.4]
                }
            ],
            "model": "text-embedding-3-small",
            "usage": {
                "prompt_tokens": 2,
                "total_tokens": 2
            }
        }))
    }

    #[tokio::test]
    async fn test_embed_sends_input_dimensions_and_custom_headers() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/embeddings"))
            .and(header("authorization", "Bearer test-key"))
            .and(header("x-trace-id", "embed-123"))
            .and(body_partial_json(json!({
                "model": "text-embedding-3-small",
                "input": ["hello", "world"],
                "dimensions": 128
            })))
            .respond_with(embedding_response())
            .expect(1)
            .mount(&server)
            .await;

        let response = EmbeddingModelRequest::builder()
            .model(test_model(server.uri()))
            .input(vec!["hello".to_string(), "world".to_string()])
            .dimensions(128)
            .headers(HashMap::from([(
                "x-trace-id".to_string(),
                "embed-123".to_string(),
            )]))
            .build()
            .embed()
            .await
            .expect("embedding request should succeed");

        assert_eq!(response.len(), 2);
        assert_eq!(response[0], vec![0.1, 0.2]);
        assert_eq!(response[1], vec![0.3, 0.4]);
    }

    #[tokio::test]
    async fn test_embed_sends_input_and_dimensions_without_custom_headers() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/embeddings"))
            .and(header("authorization", "Bearer test-key"))
            .and(body_partial_json(json!({
                "model": "text-embedding-3-small",
                "input": ["hello"],
                "dimensions": 64
            })))
            .respond_with(embedding_response())
            .expect(1)
            .mount(&server)
            .await;

        let response = EmbeddingModelRequest::builder()
            .model(test_model(server.uri()))
            .input(vec!["hello".to_string()])
            .dimensions(64)
            .build()
            .embed()
            .await
            .expect("embedding request should succeed");

        assert_eq!(response.len(), 2);
        assert_eq!(response[0], vec![0.1, 0.2]);
    }
}
