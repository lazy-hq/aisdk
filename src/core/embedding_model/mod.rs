//! Embedding model
//! TODO: add more doc

/// Embedding request builder and related types.
#[cfg(feature = "embedding-model-request")]
pub mod request;

use crate::error::Result;
use async_trait::async_trait;

use derive_builder::Builder;
#[cfg(feature = "embedding-model-request")]
pub use request::EmbeddingModelRequest;

/// The options for embedding requests.
// pub type EmbeddingModelOptions = Vec<String>;
#[derive(Debug, Clone, Builder)]
pub struct EmbeddingModelOptions {
    /// The input text to generate embeddings for
    pub input: Vec<String>,
    /// The desired number of embeddings to generate. This allowed value depends on the
    /// model used and if the provider returns an error for invalid dimensions `embed`
    /// will propagate the error.
    pub dimensions: Option<usize>,
}

impl EmbeddingModelOptions {
    /// Returns the OpenAI Embeddings builder.
    pub fn builder() -> EmbeddingModelOptionsBuilder {
        EmbeddingModelOptionsBuilder::default()
    }
}

/// The core trait abstracting the capabilities of an embedding model.
#[async_trait]
pub trait EmbeddingModel: Clone + Send + Sync + std::fmt::Debug + 'static {
    /// Embeds text inputs into vectors of floats.
    async fn embed(&self, input: EmbeddingModelOptions) -> Result<EmbeddingModelResponse>;
}

/// The response type for embedding requests.
pub type EmbeddingModelResponse = Vec<Vec<f32>>;
