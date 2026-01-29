//! Together AI provider integration tests.
use aisdk::providers::togetherai::{DeepseekAiDeepseekR1, TogetherAI};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for Together AI
generate_language_model_tests!(
    provider: TogetherAI,
    api_key_var: "TOGETHER_API_KEY",
    model_struct: DeepseekAiDeepseekR1,
    default_model: TogetherAI::moonshotai_kimi_k2_instruct(),
    tool_model: TogetherAI::moonshotai_kimi_k2_instruct(),
    structured_output_model: TogetherAI::deepseek_ai_deepseek_v3(),
    reasoning_model: TogetherAI::deepseek_ai_deepseek_r1(),
    embedding_model: TogetherAI::deepseek_ai_deepseek_r1(),
    skip_reasoning: false,
    skip_tool: false,
    skip_structured_output: true,
    skip_streaming: false,
    skip_embedding: true
);
