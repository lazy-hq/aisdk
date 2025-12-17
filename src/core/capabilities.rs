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

/// Macro to define model capabilities for a provider
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
                "** model.\n\n",
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
    };
}
