//! Macros for generating OpenAI-compatible provider implementations.
//!
//! These macros reduce boilerplate for providers that wrap the OpenAI Chat Completions API.

/// Generates the settings module for an OpenAI-compatible provider.
///
/// # Arguments
///
/// * `$settings_struct` - The name of the settings struct (e.g., `DeepSeekProviderSettings`)
/// * `$settings_builder` - The name of the settings builder struct (e.g., `DeepSeekProviderSettingsBuilder`)
/// * `$provider_display_name` - Display name for the provider (e.g., `"DeepSeek"`)
/// * `$default_base_url` - Default API base URL (e.g., `"https://api.deepseek.com/v1/"`)
/// * `$api_key_env` - Environment variable name for the API key (e.g., `"DEEPSEEK_API_KEY"`)
#[macro_export]
macro_rules! openai_compatible_settings {
    (
        $settings_struct:ident,
        $settings_builder:ident,
        $provider_display_name:literal,
        $default_base_url:literal,
        $api_key_env:literal
    ) => {
        pub mod settings {
            //! Defines the settings for this provider.

            use derive_builder::Builder;

            /// Settings for this provider (delegates to OpenAI Chat Completions).
            #[derive(Debug, Clone, Builder)]
            #[builder(setter(into), default)]
            pub struct $settings_struct {
                /// The name of the provider.
                pub provider_name: String,

                /// The base URL for the API.
                pub base_url: String,

                /// The API key for authentication.
                pub api_key: String,
            }

            impl Default for $settings_struct {
                fn default() -> Self {
                    Self {
                        provider_name: $provider_display_name.to_string(),
                        base_url: $default_base_url.to_string(),
                        api_key: std::env::var($api_key_env).unwrap_or_default(),
                    }
                }
            }

            impl $settings_struct {
                /// Creates a new builder for the settings.
                pub fn builder() -> $settings_builder {
                    $settings_builder::default()
                }
            }
        }
    };
}

/// Generates the language model implementation for an OpenAI-compatible provider.
///
/// # Arguments
///
/// * `$provider_struct` - The name of the provider struct (e.g., `DeepSeek`)
#[macro_export]
macro_rules! openai_compatible_language_model {
    ($provider_struct:ident) => {
        pub mod language_model {
            //! Language model implementation for this provider.

            use async_trait::async_trait;

            use super::$provider_struct;
            use $crate::{
                Result,
                core::{
                    LanguageModel,
                    capabilities::ModelName,
                    language_model::{LanguageModelOptions, LanguageModelResponse, ProviderStream},
                },
            };

            #[async_trait]
            impl<M: ModelName> LanguageModel for $provider_struct<M> {
                /// Returns the name of the model.
                fn name(&self) -> String {
                    self.inner.name()
                }

                #[doc = concat!("Generates text using the ", stringify!($provider_struct), " provider.")]
                async fn generate_text(
                    &mut self,
                    options: LanguageModelOptions,
                ) -> Result<LanguageModelResponse> {
                    self.inner.generate_text(options).await
                }

                #[doc = concat!("Streams text using the ", stringify!($provider_struct), " provider.")]
                async fn stream_text(
                    &mut self,
                    options: LanguageModelOptions,
                ) -> Result<ProviderStream> {
                    self.inner.stream_text(options).await
                }
            }
        }
    };
}

/// Generates the embedding model implementation for an OpenAI-compatible provider.
///
/// # Arguments
///
/// * `$provider_struct` - The name of the provider struct (e.g., `OpenRouter`)
#[macro_export]
macro_rules! openai_compatible_embedding_model {
    ($provider_struct:ident) => {
        pub mod embedding_model {
            //! Embedding model implementation for this provider.

            use async_trait::async_trait;

            use super::$provider_struct;
            use $crate::{
                Result,
                core::{
                    capabilities::ModelName,
                    embedding_model::{
                        EmbeddingModel, EmbeddingModelOptions, EmbeddingModelResponse,
                    },
                },
            };

            #[async_trait]
            impl<M: ModelName> EmbeddingModel for $provider_struct<M> {
                async fn embed(
                    &self,
                    input: EmbeddingModelOptions,
                ) -> Result<EmbeddingModelResponse> {
                    // Delegate to OpenAIChatCompletions' embedding implementation
                    self.inner.embed(input).await
                }
            }
        }
    };
}

/// Generates the main provider struct, builder, and implementations for an OpenAI-compatible provider.
///
/// # Arguments
///
/// * `$provider_struct` - The name of the provider struct (e.g., `DeepSeek`)
/// * `$builder_struct` - The name of the builder struct (e.g., `DeepSeekBuilder`)
/// * `$settings_struct` - The name of the settings struct (e.g., `DeepSeekProviderSettings`)
/// * `$example_model` - Example model identifier for docs (e.g., `"deepseek-chat"`)
#[macro_export]
macro_rules! openai_compatible_provider {
    (
        $provider_struct:ident,
        $builder_struct:ident,
        $settings_struct:ident,
        $example_model:literal
    ) => {
        use $crate::Error;
        use $crate::core::DynamicModel;
        use $crate::core::capabilities::ModelName;
        use $crate::core::utils::validate_base_url;
        use $crate::error::Result;
        use $crate::providers::openai_chat_completions::OpenAIChatCompletions;
        use settings::$settings_struct;

        #[doc = concat!("The ", stringify!($provider_struct), " provider, wrapping OpenAI Chat Completions API.")]
        #[derive(Debug, Clone)]
        pub struct $provider_struct<M: ModelName> {
            #[doc = concat!("Configuration settings for the ", stringify!($provider_struct), " provider.")]
            pub settings: $settings_struct,
            pub(crate) inner: OpenAIChatCompletions<M>,
        }

        impl<M: ModelName> $provider_struct<M> {
            #[doc = concat!(stringify!($provider_struct), " provider setting builder.")]
            pub fn builder() -> $builder_struct<M> {
                $builder_struct::default()
            }
        }

        impl $provider_struct<DynamicModel> {
            #[doc = concat!(
                "Creates a ", stringify!($provider_struct), " provider with a dynamic model name using default settings.\n\n",
                "This allows you to specify the model name as a string rather than\n",
                "using typed constructor methods.\n\n",
                "**WARNING**: when using `DynamicModel`, model capabilities are not validated.\n",
                "This means there is no compile-time guarantee that the model supports requested features.\n\n",
                "For custom configuration (API key, base URL, etc.), use the builder pattern:\n",
                "`", stringify!($provider_struct), "::<DynamicModel>::builder().model_name(...).api_key(...).build()`\n\n",
                "# Parameters\n\n",
                "* `model_name` - The model identifier (e.g., \"", $example_model, "\")\n\n",
                "# Returns\n\n",
                "A configured `", stringify!($provider_struct), "<DynamicModel>` provider instance with default settings."
            )]
            pub fn model_name(name: impl Into<String>) -> Self {
                let settings = $settings_struct::default();
                let inner = OpenAIChatCompletions::<DynamicModel>::model_name(name);

                $provider_struct { settings, inner }
            }
        }

        impl<M: ModelName> Default for $provider_struct<M> {
            #[doc = concat!("Creates a new ", stringify!($provider_struct), " provider with default settings.")]
            fn default() -> $provider_struct<M> {
                $builder_struct::default().build().unwrap()
            }
        }

        #[doc = concat!(stringify!($provider_struct), " provider builder")]
        pub struct $builder_struct<M: ModelName> {
            settings: $settings_struct,
            inner: OpenAIChatCompletions<M>,
        }

        impl<M: ModelName> Default for $builder_struct<M> {
            #[doc = concat!("Creates a new ", stringify!($provider_struct), " provider builder with default settings.")]
            fn default() -> Self {
                let settings = $settings_struct::default();
                let mut inner = OpenAIChatCompletions::default();
                inner.settings.provider_name = settings.provider_name.clone();
                inner.settings.base_url = settings.base_url.clone();
                inner.settings.api_key = settings.api_key.clone();

                Self { settings, inner }
            }
        }

        impl<M: ModelName> $builder_struct<M> {
            #[doc = concat!(
                "Sets the provider name for the ", stringify!($provider_struct), " provider.\n\n",
                "# Parameters\n\n",
                "* `provider_name` - The provider name string.\n\n",
                "# Returns\n\n",
                "The builder with the provider name set."
            )]
            pub fn provider_name(mut self, provider_name: impl Into<String>) -> Self {
                let name = provider_name.into();
                self.settings.provider_name = name.clone();
                self.inner.settings.provider_name = name;
                self
            }

            #[doc = concat!(
                "Sets the base URL for the ", stringify!($provider_struct), " provider.\n\n",
                "# Parameters\n\n",
                "* `base_url` - The base URL string for API requests.\n\n",
                "# Returns\n\n",
                "The builder with the base URL set."
            )]
            pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
                let url = base_url.into();
                self.settings.base_url = url.clone();
                self.inner.settings.base_url = url;
                self
            }

            #[doc = concat!(
                "Sets the API key for the ", stringify!($provider_struct), " provider.\n\n",
                "# Parameters\n\n",
                "* `api_key` - The API key string for authentication.\n\n",
                "# Returns\n\n",
                "The builder with the API key set."
            )]
            pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
                let key = api_key.into();
                self.settings.api_key = key.clone();
                self.inner.settings.api_key = key;
                self
            }

            #[doc = concat!(
                "Builds the ", stringify!($provider_struct), " provider.\n\n",
                "Validates the configuration and creates the provider instance.\n\n",
                "# Returns\n\n",
                "A `Result` containing the configured `", stringify!($provider_struct), "<M>` or an `Error`."
            )]
            pub fn build(mut self) -> Result<$provider_struct<M>> {
                // validate base url
                let base_url = validate_base_url(&self.settings.base_url)?;

                // check api key exists
                if self.settings.api_key.is_empty() {
                    return Err(Error::MissingField("api_key".to_string()));
                }

                // Update the inner provider with the validated base_url
                self.inner.settings.base_url = base_url.to_string();
                self.settings.base_url = base_url.to_string();

                Ok($provider_struct {
                    settings: self.settings,
                    inner: self.inner,
                })
            }
        }

        impl $builder_struct<DynamicModel> {
            #[doc = concat!(
                "Sets the model name from a string. e.g., \"", $example_model, "\"\n\n",
                "**WARNING**: when using `DynamicModel`, model capabilities are not validated.\n",
                "This means there is no compile-time guarantee that the model supports requested features.\n\n",
                "For compile-time model validation, use typed constructor methods.\n\n",
                "# Parameters\n\n",
                "* `model_name` - The model identifier (e.g., \"", $example_model, "\")\n\n",
                "# Returns\n\n",
                "The builder with the model name set."
            )]
            pub fn model_name(mut self, model_name: impl Into<String>) -> Self {
                self.inner.options.model = model_name.into();
                self
            }
        }

        // Re-exports Models for convenience
        pub use capabilities::*;
    };
}
