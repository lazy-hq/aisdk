//! Capabilities for Models.
//!
//! This module defines public traits that represent the capabilities of models.
//! These traits are used to determine which models are compatible with a given provider.
//!
//! This ensures that selected models are capable of doing the tasks they are intended for.
//! For example, only models that support tool calls can be used for tool usage.

/// A trait that represents a model name.
/// struct name to actual model name
/// e.g. struct Gpt4 {}, impl ModelName for Gpt3 { const MODEL_NAME: &'static str = "gpt-4"; }
pub trait ModelName: Send + Sync + std::fmt::Debug + Clone + 'static {
    /// The underlying API model name.
    const MODEL_NAME: &'static str;
}

/// Marker trait for models that support tool calls.
pub trait ToolCallSupport {}

/// Marker trait for models that support reasoning.
pub trait ReasoningSupport {}

/// Marker trait for models that support structured output ().
pub trait StructuredOutputSupport {}

/// Marker traits for model that support text input.
pub trait TextInputSupport {}

/// Marker trait for models that support video input.
pub trait VideoInputSupport {}

/// Marker trait for models that support audio input.
pub trait AudioInputSupport {}

/// Marker trait for models that support image input.
pub trait ImageInputSupport {}

/// Marker traits for models that support text output.
pub trait TextOutputSupport {}

/// Marker traits for models that support video output.
pub trait VideoOutputSupport {}

/// Marker traits for models that support audio output.
pub trait AudioOutputSupport {}

/// Marker traits for models that support image output.
pub trait ImageOutputSupport {}

/// A dynamic model that accepts any model name as a string.
///
/// Unlike statically-typed models (like `Gpt4o`, `Claude3`, etc.), this model
/// DynamicModel bypasses compile-time capability checking and allows to set any model name as a string.
///
/// # Use Cases
///
/// - Loading model names from configuration files or environment variables
/// - Runtime model selection based on user input
/// - Supporting custom or fine-tuned models
/// - Forward compatibility with future models not yet defined in the SDK
#[derive(Debug, Clone)]
pub struct DynamicModel;

impl ModelName for DynamicModel {
    const MODEL_NAME: &'static str = ""; // model name injected at runtime
}

/// Macro to define model capabilities for a provider.
///
/// This macro generates model struct definitions, trait implementations,
/// and constructor methods for a provider's supported models.
#[macro_export]
macro_rules! model_capabilities {
    (
        provider: $provider:ident,
        models: {
            $(
                $model:ident {
                    model_name: $model_name:literal,
                    constructor_name: $constructor_name:ident,
                    display_name: $display_name:literal,
                    capabilities: [$($capability:ident),* $(,)?]
                }
            ),* $(,)?
        }
    ) => {
        $(
            #[derive(Debug, Clone)]
            #[doc = concat!(
                "Represents the **",
                $display_name,
                "** model.\\n\\n",
                "- **Model identifier:** `",
                $model_name,
                "`"
            )]
            pub struct $model;

            #[doc = concat!(
                "Associates the **",
                $display_name,
                "** model with its canonical API model identifier."
            )]
            impl ModelName for $model {
                /// The underlying API model name.
                const MODEL_NAME: &'static str = $model_name;
            }

            $(
                #[doc = concat!(
                    "Enables the [`",
                    stringify!($capability),
                    "`] capability for [`",
                    stringify!($provider),
                    "<",
                    stringify!($model),
                    ">`]."
                )]
                impl $capability for $provider<$model> {}
            )*

            impl $provider<$model> {
                #[doc = concat!(
                    "Creates a new [`",
                    stringify!($provider),
                    "`] Provider configured to use the **",
                    $display_name,
                    "** model with default settings."
                )]
                pub fn $constructor_name() -> Self {
                    Self::default()
                }
            }
        )*

        // Auto-generate capability implementations for Provider<DynamicModel>
        // This allows runtime model selection with API-validated capabilities

        impl ToolCallSupport for $provider<DynamicModel> {}
        impl StructuredOutputSupport for $provider<DynamicModel> {}
        impl ReasoningSupport for $provider<DynamicModel> {}
        impl TextInputSupport for $provider<DynamicModel> {}
        impl TextOutputSupport for $provider<DynamicModel> {}
        impl ImageInputSupport for $provider<DynamicModel> {}
        impl VideoInputSupport for $provider<DynamicModel> {}
        impl AudioInputSupport for $provider<DynamicModel> {}
        impl ImageOutputSupport for $provider<DynamicModel> {}
        impl VideoOutputSupport for $provider<DynamicModel> {}
        impl AudioOutputSupport for $provider<DynamicModel> {}
    };
}
