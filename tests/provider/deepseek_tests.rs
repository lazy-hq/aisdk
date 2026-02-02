//! Deepseek provider integration tests.
use aisdk::providers::deepseek::{Deepseek, DeepseekChat};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for Deepseek
generate_language_model_tests!(
    provider: Deepseek,
    api_key_var: "DEEPSEEK_API_KEY",
    model_struct: DeepseekChat,
    default_model: Deepseek::deepseek_chat(),
    tool_model: Deepseek::deepseek_chat(),
    structured_output_model: Deepseek::deepseek_chat(),
    reasoning_model: Deepseek::model_name("deepseek-chat".to_string()),
    embedding_model: Deepseek::deepseek_chat(),
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: true,
    skip_streaming: false,
    skip_embedding: true
);
