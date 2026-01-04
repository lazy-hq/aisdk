//! Groq provider integration tests.
use aisdk::providers::groq::{Groq, Llama318bInstant};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for Groq
generate_language_model_tests!(
    provider: Groq,
    api_key_var: "GROQ_API_KEY",
    model_struct: Llama318bInstant,
    default_model: Groq::moonshotai_kimi_k2_instruct_0905(),
    tool_model: Groq::qwen_qwen3_32b(),
    structured_output_model: Groq::qwen_qwen3_32b(),
    reasoning_model: Groq::openai_gpt_oss_120b(),
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: true,
    skip_streaming: false
);
