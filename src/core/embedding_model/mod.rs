//! Embedding model
//! TODO: add more doc

/// Embedding request builder and related types.
pub mod request;

use async_trait::async_trait;

pub use request::EmbeddingModelRequest;

/// The options for embedding requests.
pub type EmbeddingModelOptions = Vec<String>;

/// The core trait abstracting the capabilities of an embedding model.
#[async_trait]
pub trait EmbeddingModel: Clone + Send + Sync + std::fmt::Debug + 'static {
    /// Embeds text inputs into vectors of floats.
    async fn embed(&self, input: EmbeddingModelOptions) -> EmbeddingModelResponse;
}

/// The response type for embedding requests.
pub type EmbeddingModelResponse = Vec<Vec<f32>>;
