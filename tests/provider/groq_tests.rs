//! Groq provider integration tests.
use aisdk::providers::groq::{Gemma29bIt, Groq};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for Groq
generate_language_model_tests!(
    provider: Groq,
    api_key_var: "GROQ_API_KEY",
    model_struct: Gemma29bIt,
    default_model: Groq::llama_3_1_8b_instant(),
    tool_model: Groq::qwen_qwq_32b(),
    structured_output_model: Groq::qwen_qwq_32b(),
    reasoning_model: Groq::deepseek_r1_distill_llama_70b(),
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: true,
    skip_streaming: false
);
