//! Anthropic provider integration tests.

use aisdk::providers::anthropic::{Anthropic, ClaudeOpus4};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for Anthropic
generate_language_model_tests!(
    provider: Anthropic,
    api_key_var: "ANTHROPIC_API_KEY",
    model_struct: ClaudeOpus4,
    default_model: Anthropic::claude_haiku_3(),
    tool_model: Anthropic::claude_sonnet_3(),
    structured_output_model: Anthropic::claude_sonnet_4_5(),
    reasoning_model: Anthropic::claude_haiku_4_5(),
    skip_reasoning: true,
    skip_tool: false,
    skip_structured_output: true,
    skip_streaming: false
);
