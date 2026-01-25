//! OpenRouter provider integration tests.
use aisdk::providers::openrouter::{OpenRouter, OpenaiGpt51};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for OpenRouter
generate_language_model_tests!(
    provider: OpenRouter,
    api_key_var: "OPENROUTER_API_KEY",
    model_struct: OpenaiGpt51,
    default_model: OpenRouter::openai_gpt_4o_mini(),
    tool_model: OpenRouter::openai_gpt_5_1(),
    structured_output_model: OpenRouter::openai_gpt_5_1(),
    reasoning_model: OpenRouter::openai_o4_mini(),
    embedding_model: OpenRouter::openai_gpt_5_1(),
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: false,
    skip_streaming: false,
    skip_embedding: true
);
