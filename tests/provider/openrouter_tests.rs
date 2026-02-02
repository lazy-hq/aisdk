//! OpenRouter provider integration tests.
use aisdk::providers::openrouter::{AllenaiMolmo28bFree, Openrouter};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for OpenRouter
generate_language_model_tests!(
    provider: Openrouter,
    api_key_var: "OPENROUTER_API_KEY",
    model_struct: AllenaiMolmo28bFree,
    default_model: Openrouter::allenai_molmo_2_8b_free(),
    tool_model: Openrouter::allenai_molmo_2_8b_free(),
    structured_output_model: Openrouter::model_name("openai/gpt-5-1".to_string()),
    reasoning_model: Openrouter::model_name("allenai/molmo-2-8b:free".to_string()),
    embedding_model: Openrouter::model_name("allenai/molmo-2-8b:free".to_string()),
    skip_reasoning: false,
    skip_tool: false,
    skip_structured_output: true,
    skip_streaming: false,
    skip_embedding: true  // Couldn't find free embedding model
);
