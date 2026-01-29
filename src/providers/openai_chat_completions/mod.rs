//! OpenAI Chat Completions API provider.
//!
//! **INTERNAL USE ONLY**: This provider is not intended for direct use by end users.
//! It implements the OpenAI Chat Completions API (`/v1/chat/completions`) and serves
//! as a base for providers like Groq, OpenRouter, Together AI, and others that are
//! compatible with this API format.

pub(crate) mod client;
pub(crate) mod conversions;
pub(crate) mod language_model;
#[macro_use]
pub mod macros;
pub mod settings;

use crate::core::DynamicModel;
use crate::core::capabilities::ModelName;
use client::ChatCompletionsOptions;
use settings::OpenAIChatCompletionsSettings;

/// Internal OpenAI Chat Completions API provider.
///
/// This is not intended for direct end-user use. Use the OpenAI provider for
/// the latest features, or wrap this provider for compatible APIs.
#[derive(Debug, Clone)]
pub struct OpenAIChatCompletions<M: ModelName> {
    /// Configuration settings for the provider
    pub settings: OpenAIChatCompletionsSettings,
    /// Request options for the API call
    pub(crate) options: ChatCompletionsOptions,
    _phantom: std::marker::PhantomData<M>,
}

impl<M: ModelName> Default for OpenAIChatCompletions<M> {
    fn default() -> Self {
        let settings = OpenAIChatCompletionsSettings::default();
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

impl OpenAIChatCompletions<DynamicModel> {
    /// Creates an OpenAIChatCompletions provider with a dynamic model name using default settings.
    ///
    /// **INTERNAL USE ONLY**: This method is for internal provider implementations.
    ///
    /// # Parameters
    ///
    /// * `model_name` - The model identifier (e.g., "gpt-4o", "llama-3.3-70b-specdec")
    ///
    /// # Returns
    ///
    /// A configured `OpenAIChatCompletions<DynamicModel>` provider instance.
    pub fn model_name(name: impl Into<String>) -> Self {
        let settings = OpenAIChatCompletionsSettings::default();
        let options = ChatCompletionsOptions {
            model: name.into(),
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
