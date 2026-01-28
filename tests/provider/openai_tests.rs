//! OpenAI provider integration tests.

use aisdk::providers::openai::{Gpt5, OpenAI, TextEmbedding3Small};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for OpenAI
generate_language_model_tests!(
    provider: OpenAI,
    api_key_var: "OPENAI_API_KEY",
    model_struct: Gpt5,
    default_model: OpenAI::gpt_5_nano(),
    tool_model: OpenAI::gpt_5_nano(),
    structured_output_model: OpenAI::gpt_5_nano(),
    reasoning_model: OpenAI::gpt_5_nano(),
    embedding_model: OpenAI::<TextEmbedding3Small>::default(),
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: false,
    skip_streaming: false,
    skip_embedding: false
);
