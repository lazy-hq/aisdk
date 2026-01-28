//! Vercel provider integration tests.
use aisdk::providers::vercel::{OpenaiGpt4oMini, Vercel};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for Vercel
generate_language_model_tests!(
    provider: Vercel,
    api_key_var: "AI_GATEWAY_API_KEY",
    model_struct: OpenaiGpt4oMini,
    default_model: Vercel::openai_gpt_4o_mini(),
    tool_model: Vercel::openai_gpt_4o_mini(),
    structured_output_model: Vercel::openai_gpt_4o_mini(),
    reasoning_model: Vercel::openai_o1(),
    embedding_model: Vercel::openai_gpt_4o_mini(),
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: false,
    skip_streaming: false,
    skip_embedding: true
);
