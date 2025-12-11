//! OpenAI provider integration tests using the new macro-based system.
use aisdk::providers::groq::Groq;

// Include all macro definitions
include!("macros.rs");

// Configuration for OpenAI provider tests
const GROQ_CONFIG: ProviderConfig = ProviderConfig {
    default_model: "qwen/qwen3-32b",
    tool_model: None,
    reasoning_model: Some("openai/gpt-oss-120b"),
    non_reasoning_model: Some("llama-3.1-8b-instant"),
    structured_output_model: Some("openai/gpt-oss-120b"),
    error_model: "invalid-model-name",
};

// Generate all standard integration tests for OpenAI using the configuration
generate_language_model_tests!(
    Groq,
    GROQ_CONFIG,
    "GROQ_API_KEY",
    skip_reasoning: false,
    skip_tool: false,
    skip_structured_output: false,
    skip_streaming: false
);
