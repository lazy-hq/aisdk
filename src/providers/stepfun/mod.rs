//! This module provides the Stepfun provider, wrapping OpenAI Chat Completions for Stepfun requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    StepfunProviderSettings,
    StepfunProviderSettingsBuilder,
    "Stepfun",
    "https://api.stepfun.com/v1",
    "STEPFUN_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Stepfun,
    StepfunBuilder,
    StepfunProviderSettings,
    "stepfun"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Stepfun);
