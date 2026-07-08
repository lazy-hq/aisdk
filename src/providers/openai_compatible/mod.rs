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

pub mod capabilities;
pub mod embedding_model;
pub mod language_model;
pub mod settings;

use crate::Error;
use crate::core::DynamicModel;
use crate::core::capabilities::ModelName;
use crate::core::utils::validate_base_url;
use crate::error::Result;
use crate::providers::openai_chat_completions::OpenAIChatCompletions;
use crate::providers::openai_compatible::settings::OpenAICompatibleSettings;

/// OpenAI-compatible API provider.
///
/// This provider can be used for any OpenAI-compatible endpoint that uses the
/// chat completions API format. It provides a simple, public interface for
/// connecting to custom or less common OpenAI-compatible services.
///
/// # Example
///
/// ```rust,no_run
/// use aisdk::providers::OpenAICompatible;
/// use aisdk::core::DynamicModel;
///
/// let provider = OpenAICompatible::<DynamicModel>::builder()
///     .base_url("https://api.z.ai/api/coding/paas/v4")
///     .api_key("your-api-key")
///     .model_name("glm-4.5")
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct OpenAICompatible<M: ModelName> {
    /// Configuration settings for the OpenAICompatible provider.
    pub settings: OpenAICompatibleSettings,
    pub(crate) inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> OpenAICompatible<M> {
    /// OpenAICompatible provider setting builder.
    pub fn builder() -> OpenAICompatibleBuilder<M> {
        OpenAICompatibleBuilder::default()
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
        let settings = OpenAICompatibleSettings::default();
        let inner = OpenAIChatCompletions::<DynamicModel>::model_name(name);

        OpenAICompatible { settings, inner }
    }
}

impl<M: ModelName> Default for OpenAICompatible<M> {
    /// Creates a new OpenAICompatible provider with default settings.
    fn default() -> OpenAICompatible<M> {
        OpenAICompatibleBuilder::default().build().unwrap()
    }
}

/// OpenAICompatible provider builder
pub struct OpenAICompatibleBuilder<M: ModelName> {
    settings: OpenAICompatibleSettings,
    inner: OpenAIChatCompletions<M>,
}

impl<M: ModelName> Default for OpenAICompatibleBuilder<M> {
    /// Creates a new OpenAICompatible provider with default settings.
    fn default() -> Self {
        let settings = OpenAICompatibleSettings::default();
        let mut inner = OpenAIChatCompletions::default();
        inner.settings.provider_name = settings.provider_name.clone();
        inner.settings.base_url = settings.base_url.clone();
        inner.settings.api_key = settings.api_key.clone();
        inner.settings.path = settings.path.clone();
        inner.settings.headers = settings.headers.clone();
        inner.settings.body = settings.body.clone();

        Self { settings, inner }
    }
}

impl<M: ModelName> OpenAICompatibleBuilder<M> {
    /// Sets the provider name for the OpenAICompatible provider.
    ///
    /// # Parameters
    ///
    /// * `provider_name` - The provider name string.
    ///
    /// # Returns
    ///
    /// The builder with the provider name set.
    pub fn provider_name(mut self, provider_name: impl Into<String>) -> Self {
        let name = provider_name.into();
        self.settings.provider_name = name.clone();
        self.inner.settings.provider_name = name;
        self
    }

    /// Sets the base URL for the OpenAICompatible provider.
    ///
    /// # Parameters
    ///
    /// * `base_url` - The base URL string for API requests.
    ///
    /// # Returns
    ///
    /// The builder with the base URL set.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        let url = base_url.into();
        self.settings.base_url = url.clone();
        self.inner.settings.base_url = url;
        self
    }

    /// Sets the API key for the OpenAICompatible provider.
    ///
    /// # Parameters
    ///
    /// * `api_key` - The API key string for authentication.
    ///
    /// # Returns
    ///
    /// The builder with the API key set.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        let key = api_key.into();
        self.settings.api_key = key.clone();
        self.inner.settings.api_key = key;
        self
    }

    /// Sets a custom API path, overriding the default "chat/completions".
    pub fn path(mut self, path: impl Into<String>) -> Self {
        let p = Some(path.into());
        self.settings.path = p.clone();
        self.inner.settings.path = p;
        self
    }

    /// Sets extra headers to merge into every request.
    pub fn headers(mut self, headers: std::collections::HashMap<String, String>) -> Self {
        self.settings.headers = Some(headers.clone());
        self.inner.settings.headers = Some(headers);
        self
    }

    /// Sets extra body fields to merge into every request.
    pub fn body(mut self, body: serde_json::Value) -> Self {
        if let serde_json::Value::Object(map) = body {
            self.settings.body = Some(map.clone());
            self.inner.settings.body = Some(map);
        }
        self
    }

    /// Builds the OpenAICompatible provider.
    ///
    /// Validates the configuration and creates the provider instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the configured `OpenAICompatible<M>` or an `Error`.
    pub fn build(mut self) -> Result<OpenAICompatible<M>> {
        // validate base url
        let base_url = validate_base_url(&self.settings.base_url)?;

        // check api key exists
        if self.settings.api_key.is_empty() {
            return Err(Error::MissingField("api_key".to_string()));
        }

        // Update the inner provider with the validated base_url
        self.inner.settings.base_url = base_url.to_string();
        self.settings.base_url = base_url.to_string();

        Ok(OpenAICompatible {
            settings: self.settings,
            inner: self.inner,
        })
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
    ///
    /// # Returns
    ///
    /// The builder with the model name set.
    pub fn model_name(mut self, model_name: impl Into<String>) -> Self {
        self.inner.options.model = model_name.into();
        self
    }
}
