//! OpenRouter provider integration tests.
use aisdk::providers::openrouter::{DeepseekDeepseekR10528Free, OpenRouter};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for OpenRouter
generate_language_model_tests!(
    provider: OpenRouter,
    api_key_var: "OPENROUTER_API_KEY",
    model_struct: DeepseekDeepseekR10528Free,
    default_model: OpenRouter::deepseek_deepseek_r1_0528_free(),
    tool_model: OpenRouter::qwen_qwen3_coder_free(),
    structured_output_model: OpenRouter::openai_gpt_5_1(),
    reasoning_model: OpenRouter::deepseek_deepseek_r1_0528_free(),
    embedding_model: OpenRouter::deepseek_deepseek_r1_0528_free(),
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: true,
    skip_streaming: false,
    skip_embedding: true  // Couldn't find free embedding model
);
