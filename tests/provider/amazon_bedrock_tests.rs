//! Amazon Bedrock provider integration tests.
use aisdk::providers::amazon_bedrock::{AmazonBedrock, AnthropicClaudeHaiku4520251001V10};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for Amazon Bedrock
generate_language_model_tests!(
    provider: AmazonBedrock,
    api_key_var: "BEDROCK_API_KEY",
    model_struct: AnthropicClaudeHaiku4520251001V10,
    default_model: AmazonBedrock::anthropic_claude_3_5_haiku_20241022_v1_0(),
    tool_model: AmazonBedrock::anthropic_claude_3_5_sonnet_20241022_v2_0(),
    structured_output_model: AmazonBedrock::mistral_mixtral_8x7b_instruct_v0_1(),
    reasoning_model: AmazonBedrock::anthropic_claude_sonnet_4_20250514_v1_0(),
    skip_reasoning: false,
    skip_tool: false,
    skip_structured_output: false,
    skip_streaming: false
);
