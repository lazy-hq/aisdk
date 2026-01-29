//! OpenAI compatible provider integration tests.

use aisdk::providers::OpenAICompatible;

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for OpenAI
generate_language_model_tests!(
    provider: OpenAICompatible,
    api_key_var: "OPENAI_API_KEY",
    model_struct: DynamicModel,
    default_model: OpenAICompatible::model_name("gpt-5-nano"),
    tool_model: OpenAICompatible::model_name("gpt-5-nano"),
    structured_output_model: OpenAICompatible::model_name("gpt-5-nano"),
    reasoning_model: OpenAICompatible::model_name("gpt-5-nano"),
    embedding_model: OpenAICompatible::model_name("text-embedding-ada-002"),
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: false,
    skip_streaming: false,
    skip_embedding: false
);
