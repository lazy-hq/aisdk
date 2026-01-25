//! DeepSeek provider integration tests.
use aisdk::providers::deepseek::{DeepSeek, DeepseekChat};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for DeepSeek
generate_language_model_tests!(
    provider: DeepSeek,
    api_key_var: "DEEPSEEK_API_KEY",
    model_struct: DeepseekChat,
    default_model: DeepSeek::deepseek_chat(),
    tool_model: DeepSeek::deepseek_chat(),
    structured_output_model: DeepSeek::deepseek_chat(),
    reasoning_model: DeepSeek::deepseek_reasoner(),
    embedding_model: DeepSeek::deepseek_chat(),
    skip_reasoning: false,
    skip_tool: false,
    skip_structured_output: true,  // DeepSeek doesn't support structured output
    skip_streaming: false,
    skip_embedding: true
);
