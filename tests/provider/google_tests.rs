//! Google provider integration tests.

use aisdk::providers::google::{Gemini25Flash, Google};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for Google
generate_language_model_tests!(
    provider: Google,
    api_key_var: "GOOGLE_API_KEY",
    model_struct: Gemini25Flash,
    default_model: Google::gemini_3_flash_preview(),
    tool_model:  Google::gemini_3_flash_preview(),
    structured_output_model:  Google::gemini_3_flash_preview(),
    reasoning_model: Google::gemini_3_flash_preview(),
    embedding_model: Google::gemini_3_flash_preview(),
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: false,
    skip_streaming: false,
    skip_embedding: true
);
