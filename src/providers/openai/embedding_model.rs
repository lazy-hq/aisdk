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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{DynamicModel, EmbeddingModelRequest};
    use serde_json::json;
    use std::collections::HashMap;
    use wiremock::matchers::{body_partial_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_model(base_url: String) -> OpenAI<DynamicModel> {
        let mut model = OpenAI::<DynamicModel>::model_name("text-embedding-3-small");
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
            .and(path("/v1/embeddings"))
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
            .and(path("/v1/embeddings"))
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

    #[tokio::test]
    async fn test_embed_merges_provider_and_request_body_overrides() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/embeddings"))
            .and(header("authorization", "Bearer test-key"))
            .and(body_partial_json(json!({
                "model": "text-embedding-3-small",
                "input": ["hello"],
                "dimensions": 128,
                "encoding_format": "base64",
                "user": "provider-user"
            })))
            .respond_with(embedding_response())
            .expect(1)
            .mount(&server)
            .await;

        let mut model = test_model(server.uri());
        model.settings.body = Some(
            json!({
                "user": "provider-user"
            })
            .as_object()
            .expect("provider body should be an object")
            .clone(),
        );

        let response = EmbeddingModelRequest::builder()
            .model(model)
            .input(vec!["hello".to_string()])
            .dimensions(64)
            .body(json!({
                "dimensions": 128,
                "encoding_format": "base64"
            }))
            .build()
            .embed()
            .await
            .expect("embedding request should succeed");

        assert_eq!(response.len(), 2);
        assert_eq!(response[0], vec![0.1, 0.2]);
    }
}
