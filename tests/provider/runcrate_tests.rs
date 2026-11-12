//! Runcrate provider integration tests.
use aisdk::providers::runcrate::{DeepseekAiDeepseekV4Flash, Runcrate};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for Runcrate
generate_language_model_tests!(
    provider: Runcrate,
    api_key_var: "RUNCRATE_API_KEY",
    model_struct: DeepseekAiDeepseekV4Flash,
    default_model: Runcrate::deepseek_ai_deepseek_v4_flash(),
    tool_model: Runcrate::deepseek_ai_deepseek_v4_flash(),
    structured_output_model: Runcrate::deepseek_ai_deepseek_v4_flash(),
    reasoning_model: Runcrate::deepseek_ai_deepseek_v4_flash(),
    embedding_model: Runcrate::qwen_qwen3_embedding_0_6b(),
    skip_reasoning: false,
    skip_tool: false,
    skip_structured_output: false,
    skip_streaming: false,
    skip_embedding: false
);
