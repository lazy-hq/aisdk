//! OpenAI provider integration tests using the new macro-based system.

use aisdk::providers::anthropic::Anthropic;

// Include all macro definitions
include!("macros.rs");

// Configuration for OpenAI provider tests
const ANTHROPIC_CONFIG: ProviderConfig = ProviderConfig {
    default_model: "claude-3-5-haiku-20241022",
    reasoning_model: Some("claude-3-5-haiku-20241022"),
    non_reasoning_model: Some("claude-3-5-haiku-20241022"),
    structured_output_model: Some("claude-3-5-haiku-20241022"),
    error_model: "invalid-model-name",
};

// Generate all standard integration tests for OpenAI using the configuration
generate_language_model_tests!(
    Anthropic,
    ANTHROPIC_CONFIG,
    "ANTHROPIC_API_KEY",
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: false,
    skip_streaming: false
);
