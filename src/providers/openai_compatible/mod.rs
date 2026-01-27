//! OpenAI-compatible API provider.
//!
//! This provider can be used for any OpenAI-compatible endpoint that uses the
//! `/v1/chat/completions` API format, such as Z.ai, custom endpoints, or other
//! providers that don't have dedicated implementations.
//!
//! # Example
//!
//! ```rust,no_run
//! use aisdk::providers::OpenAICompatible;
//! use aisdk::core::DynamicModel;
//!
//! let provider = OpenAICompatible::<DynamicModel>::builder()
//!     .base_url("https://api.z.ai/api/coding/paas/v4")
//!     .api_key("your-api-key")
//!     .model_name("glm-4.5")
//!     .build()
//!     .unwrap();
//! ```

use crate::core::DynamicModel;
use crate::core::capabilities::ModelName;
use crate::core::language_model::{
    LanguageModel, LanguageModelOptions, LanguageModelResponse, ProviderStream,
};
use crate::core::utils::validate_base_url;
use crate::error::{Error, Result};
use crate::providers::openai_chat_completions::{
    OpenAIChatCompletions, client::ChatCompletionsOptions, settings::OpenAIChatCompletionsSettings,
};
use async_trait::async_trait;

/// OpenAI-compatible API provider.
///
/// This provider can be used for any OpenAI-compatible endpoint that uses the
/// chat completions API format. It provides a simple, public interface for
/// connecting to custom or less common OpenAI-compatible services.
#[derive(Debug, Clone)]
pub struct OpenAICompatible<M: ModelName> {
    /// The inner OpenAI Chat Completions provider
    #[allow(dead_code)]
    inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> OpenAICompatible<M> {
    /// Creates a new builder for the OpenAICompatible provider.
    ///
    /// # Returns
    ///
    /// An `OpenAICompatibleBuilder` for configuring the provider.
    pub fn builder() -> OpenAICompatibleBuilder<M> {
        OpenAICompatibleBuilder::default()
    }
}

impl<M: ModelName> Default for OpenAICompatible<M> {
    /// Creates a new OpenAICompatible provider with default settings.
    ///
    /// Defaults:
    /// - base_url: "https://api.openai.com/v1"
    /// - path: "chat/completions"
    fn default() -> Self {
        let settings = OpenAIChatCompletionsSettings {
            base_url: "https://api.openai.com/v1".to_string(),
            path: "chat/completions".to_string(),
            ..Default::default()
        };
        let options = ChatCompletionsOptions {
            model: M::MODEL_NAME.to_string(),
            messages: vec![],
            ..Default::default()
        };

        Self {
            inner: OpenAIChatCompletions {
                settings,
                options,
                _phantom: std::marker::PhantomData,
            },
        }
    }
}

impl OpenAICompatible<DynamicModel> {
    /// Creates an OpenAICompatible provider with a dynamic model name using default settings.
    ///
    /// This allows you to specify the model name as a string rather than
    /// using static model types.
    ///
    /// **WARNING**: when using `DynamicModel`, model capabilities are not validated.
    /// This means there is no compile-time guarantee that the model supports requested features.
    ///
    /// For custom configuration (API key, base URL, etc.), use the builder pattern:
    /// `OpenAICompatible::<DynamicModel>::builder().model_name(...).api_key(...).build()`
    ///
    /// # Parameters
    ///
    /// * `model_name` - The model identifier (e.g., "gpt-4o", "glm-4.5")
    ///
    /// # Returns
    ///
    /// A configured `OpenAICompatible<DynamicModel>` provider instance with default settings.
    pub fn model_name(name: impl Into<String>) -> Self {
        let settings = OpenAIChatCompletionsSettings {
            base_url: "https://api.openai.com/v1".to_string(),
            path: "chat/completions".to_string(),
            ..Default::default()
        };
        let options = ChatCompletionsOptions {
            model: name.into(),
            messages: vec![],
            ..Default::default()
        };

        Self {
            inner: OpenAIChatCompletions {
                settings,
                options,
                _phantom: std::marker::PhantomData,
            },
        }
    }
}

/// Builder for the OpenAICompatible provider.
pub struct OpenAICompatibleBuilder<M: ModelName> {
    settings: OpenAIChatCompletionsSettings,
    options: ChatCompletionsOptions,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: ModelName> Default for OpenAICompatibleBuilder<M> {
    fn default() -> Self {
        let settings = OpenAIChatCompletionsSettings {
            base_url: "https://api.openai.com/v1".to_string(),
            path: "chat/completions".to_string(),
            ..Default::default()
        };
        let options = ChatCompletionsOptions {
            model: M::MODEL_NAME.to_string(),
            messages: vec![],
            ..Default::default()
        };

        Self {
            settings,
            options,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl OpenAICompatibleBuilder<DynamicModel> {
    /// Sets the model name from a string.
    ///
    /// **WARNING**: when using `DynamicModel`, model capabilities are not validated.
    ///
    /// # Parameters
    ///
    /// * `model_name` - The model identifier (e.g., "gpt-4o", "glm-4.5")
    pub fn model_name(mut self, model_name: impl Into<String>) -> Self {
        self.options.model = model_name.into();
        self
    }
}

impl<M: ModelName> OpenAICompatibleBuilder<M> {
    /// Sets the base URL for the API.
    ///
    /// # Parameters
    ///
    /// * `base_url` - The base URL string for API requests (e.g., "https://api.z.ai/api/coding/paas/v4")
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.settings.base_url = base_url.into();
        self
    }

    /// Sets the API key for authentication.
    ///
    /// # Parameters
    ///
    /// * `api_key` - The API key string for authentication.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.settings.api_key = api_key.into();
        self
    }

    /// Sets the name of the provider.
    ///
    /// # Parameters
    ///
    /// * `provider_name` - The provider name string.
    pub fn provider_name(mut self, provider_name: impl Into<String>) -> Self {
        self.settings.provider_name = provider_name.into();
        self
    }

    /// Sets the API path for the provider.
    ///
    /// Defaults to "chat/completions" for OpenAI-compatible endpoints.
    /// You typically don't need to change this unless the API uses a different path.
    ///
    /// # Parameters
    ///
    /// * `path` - The API path string (e.g., "chat/completions", "v2/chat/completions").
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.settings.path = path.into();
        self
    }

    /// Builds the OpenAICompatible provider.
    ///
    /// Validates the configuration and creates the provider instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configured `OpenAICompatible` provider or an `Error`.
    pub fn build(self) -> std::result::Result<OpenAICompatible<M>, Error> {
        // Validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // Check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        let settings = OpenAIChatCompletionsSettings {
            base_url,
            ..self.settings
        };

        let inner = OpenAIChatCompletions {
            settings,
            options: self.options,
            _phantom: std::marker::PhantomData,
        };

        Ok(OpenAICompatible { inner })
    }
}

#[async_trait]
impl<M: ModelName> LanguageModel for OpenAICompatible<M> {
    fn name(&self) -> String {
        self.inner.name()
    }

    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        self.inner.generate_text(options).await
    }

    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        self.inner.stream_text(options).await
    }
}
