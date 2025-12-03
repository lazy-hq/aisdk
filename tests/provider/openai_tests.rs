//! OpenAI provider integration tests using the new macro-based system.

use aisdk::providers::openai::OpenAI;

// Include all macro definitions
include!("macros.rs");

// Configuration for OpenAI provider tests
const OPENAI_CONFIG: ProviderConfig = ProviderConfig {
    default_model: "gpt-5-nano-2025-08-07",
    structured_output_model: Some("gpt-5-nano-2025-08-07"),
    reasoning_model: Some("gpt-5-nano-2025-08-07"),
    non_reasoning_model: Some("gpt-5-nano-2025-08-07"),
    error_model: "invalid-model-name",
};

// Generate all standard integration tests for OpenAI using the configuration
generate_language_model_tests!(
    OpenAI,
    OPENAI_CONFIG,
    "OPENAI_API_KEY",
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: false,
    skip_streaming: false
);
