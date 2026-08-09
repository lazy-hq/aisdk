//! Inception provider integration tests.
use aisdk::providers::inception::{Inception, Mercury2};

include!("macros.rs");

generate_language_model_tests!(
    provider: Inception,
    api_key_var: "INCEPTION_API_KEY",
    model_struct: Mercury2,
    default_model: Inception::mercury_2(),
    tool_model: Inception::mercury_2(),
    structured_output_model: Inception::mercury_2(),
    reasoning_model: Inception::mercury_2(),
    embedding_model: Inception::mercury_2(),
    skip_reasoning: false,
    skip_tool: false,
    skip_structured_output: false,
    skip_streaming: false,
    skip_embedding: true
);
