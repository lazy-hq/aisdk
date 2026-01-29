//! Mistral provider integration tests.
use aisdk::providers::mistral::{Mistral, MistralLargeLatest};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for Mistral
generate_language_model_tests!(
    provider: Mistral,
    api_key_var: "MISTRAL_API_KEY",
    model_struct: MistralLargeLatest,
    default_model: Mistral::mistral_large_latest(),
    tool_model: Mistral::mistral_large_latest(),
    structured_output_model: Mistral::mistral_large_latest(),
    reasoning_model: Mistral::magistral_medium_latest(),
    embedding_model: Mistral::mistral_large_latest(),
    skip_reasoning: false,
    skip_tool: false,
    skip_structured_output: true,  // Mistral doesn't support structured output
    skip_streaming: false,
    skip_embedding: true
);
