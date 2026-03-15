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
        let headers = input.headers.clone();

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
        let response = model.send(&model.settings.base_url, headers).await?;

        // Extract embeddings from response
        Ok(response.embeddings.into_iter().map(|e| e.values).collect())
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

    fn test_model(base_url: String) -> Google<DynamicModel> {
        let mut model = Google::<DynamicModel>::model_name("text-embedding-004");
        model.settings.base_url = base_url;
        model.settings.api_key = "test-key".to_string();
        model
    }

    fn embedding_response() -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(json!({
            "embeddings": [
                { "values": [0.1, 0.2] },
                { "values": [0.3, 0.4] }
            ]
        }))
    }

    #[tokio::test]
    async fn test_embed_sends_input_dimensions_and_custom_headers() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1beta/models/text-embedding-004:batchEmbedContents"))
            .and(header("x-goog-api-key", "test-key"))
            .and(header("x-trace-id", "embed-123"))
            .and(body_partial_json(json!({
                "requests": [
                    {
                        "model": "models/text-embedding-004",
                        "content": {
                            "role": "user",
                            "parts": [{"text": "hello"}]
                        },
                        "outputDimensionality": 128
                    },
                    {
                        "model": "models/text-embedding-004",
                        "content": {
                            "role": "user",
                            "parts": [{"text": "world"}]
                        },
                        "outputDimensionality": 128
                    }
                ]
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
            .and(path("/v1beta/models/text-embedding-004:batchEmbedContents"))
            .and(header("x-goog-api-key", "test-key"))
            .and(body_partial_json(json!({
                "requests": [
                    {
                        "model": "models/text-embedding-004",
                        "content": {
                            "role": "user",
                            "parts": [{"text": "hello"}]
                        },
                        "outputDimensionality": 64
                    }
                ]
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
