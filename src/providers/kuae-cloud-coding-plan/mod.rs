//! This module provides the KuaeCloudCodingPlan provider, wrapping OpenAI Chat Completions for KuaeCloudCodingPlan requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    KuaeCloudCodingPlanProviderSettings,
    KuaeCloudCodingPlanProviderSettingsBuilder,
    "KuaeCloudCodingPlan",
    "https://coding-plan-endpoint.kuaecloud.net/v1",
    "KUAE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    KuaeCloudCodingPlan,
    KuaeCloudCodingPlanBuilder,
    KuaeCloudCodingPlanProviderSettings,
    "kuae-cloud-coding-plan"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(KuaeCloudCodingPlan);
