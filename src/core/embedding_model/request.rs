use std::ops::{Deref, DerefMut};

use crate::core::embedding_model::{EmbeddingModel, EmbeddingModelOptions, EmbeddingModelResponse};
use crate::error::Result;

/// Options for embedding generation requests to be used by `embed`.
#[derive(Debug, Clone)]
pub struct EmbeddingModelRequest<M: EmbeddingModel> {
    /// The embedding model to use for generating embeddings.
    pub model: M,
    /// Input and Configuration options for the embedding model request.
    pub(crate) options: EmbeddingModelOptions,
}

impl<M: EmbeddingModel> EmbeddingModelRequest<M> {
    /// Creates a new builder for constructing an `EmbeddingModelRequest`.
    ///
    /// This method initiates the type-state builder pattern, starting with the
    /// [`ModelStage`] where you must specify the embedding model.
    pub fn builder() -> EmbeddingModelRequestBuilder<M> {
        EmbeddingModelRequestBuilder::default()
    }

    /// Generates embeddings for the input text.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of embedding vectors, where each embedding is a vector of floats,
    /// or an error if the embedding request fails.
    pub async fn embed(&self) -> Result<EmbeddingModelResponse> {
        self.model.embed(self.options.clone()).await
    }
}

impl<M: EmbeddingModel> Deref for EmbeddingModelRequest<M> {
    type Target = EmbeddingModelOptions;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl<M: EmbeddingModel> DerefMut for EmbeddingModelRequest<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.options
    }
}

/// Type-state markers for the `EmbeddingModelRequestBuilder`.
///
/// These zero-sized types ensure the builder is used in the correct order,
/// preventing invalid request configurations at compile time.
///
/// The initial builder state where the embedding model must be set.
///
/// Transitions to [`OptionsStage`] after calling [`model`](EmbeddingModelRequestBuilder::model).
pub struct ModelStage {}

/// The final state where input and dimensions can be configured before building.
///
/// Transitions to the completed `EmbeddingModelRequest` after calling [`build`](EmbeddingModelRequestBuilder::build).
pub struct OptionsStage {}

/// A type-state builder for constructing `EmbeddingModelRequest` instances.
///
/// This builder uses phantom types to enforce a specific construction order,
/// ensuring that required fields (like the model) are set before optional ones.
///
/// # Type Parameters
///
/// * `M` - The embedding model type.
/// * `State` - The current builder state, determining available methods.
pub struct EmbeddingModelRequestBuilder<M: EmbeddingModel, State = ModelStage> {
    model: Option<M>,
    options: EmbeddingModelOptions,
    state: std::marker::PhantomData<State>,
}

impl<M: EmbeddingModel, State> Deref for EmbeddingModelRequestBuilder<M, State> {
    type Target = EmbeddingModelOptions;

    /// Dereferences to the underlying `EmbeddingModelOptions`.
    ///
    /// This allows direct access to the options fields during building.
    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl<M: EmbeddingModel, State> DerefMut for EmbeddingModelRequestBuilder<M, State> {
    /// Mutably dereferences to the underlying `EmbeddingModelOptions`.
    ///
    /// This allows direct mutation of the options fields during building.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.options
    }
}

impl<M: EmbeddingModel> EmbeddingModelRequestBuilder<M> {
    fn default() -> Self {
        EmbeddingModelRequestBuilder {
            model: None,
            options: EmbeddingModelOptions::builder()
                .input(vec![])
                .dimensions(0)
                .build()
                .unwrap(),
            state: std::marker::PhantomData,
        }
    }
}

/// Methods available in the [`ModelStage`] state.
impl<M: EmbeddingModel> EmbeddingModelRequestBuilder<M, ModelStage> {
    /// Sets the embedding model for the request.
    ///
    /// This is the first required step in building a request.
    ///
    /// # Parameters
    ///
    /// * `model` - The embedding model instance to use.
    ///
    /// # Returns
    ///
    /// The builder in the [`OptionsStage`] state.
    pub fn model(self, model: M) -> EmbeddingModelRequestBuilder<M, OptionsStage> {
        EmbeddingModelRequestBuilder {
            model: Some(model),
            options: self.options,
            state: std::marker::PhantomData,
        }
    }
}

/// Methods available in the [`OptionsStage`] state.
impl<M: EmbeddingModel> EmbeddingModelRequestBuilder<M, OptionsStage> {
    /// Sets the input text to generate embeddings for.
    ///
    /// # Parameters
    ///
    /// * `input` - A vector of strings to embed.
    ///
    /// # Returns
    ///
    /// The builder with the input set.
    pub fn input(
        mut self,
        input: impl Into<Vec<String>>,
    ) -> EmbeddingModelRequestBuilder<M, OptionsStage> {
        self.options.input = input.into();
        self
    }

    /// Sets the desired number of dimensions for the embeddings.
    ///
    /// # Parameters
    ///
    /// * `dimensions` - The desired embedding dimension size.
    ///
    /// # Returns
    ///
    /// The builder with the dimensions set.
    pub fn dimensions(
        mut self,
        dimensions: usize,
    ) -> EmbeddingModelRequestBuilder<M, OptionsStage> {
        self.options.dimensions = dimensions;
        self
    }

    /// Builds the `EmbeddingModelRequest`.
    ///
    /// This method consumes the builder and returns the configured request.
    ///
    /// # Returns
    ///
    /// The constructed `EmbeddingModelRequest`.
    pub fn build(self) -> EmbeddingModelRequest<M> {
        let model = self
            .model
            .unwrap_or_else(|| unreachable!("Model must be set"));

        EmbeddingModelRequest {
            model,
            options: self.options,
        }
    }
}
