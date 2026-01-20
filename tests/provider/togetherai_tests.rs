//! Together AI provider integration tests.
use aisdk::providers::togetherai::{MetaLlamaLlama3370bInstructTurbo, TogetherAI};

// Include all macro definitions
include!("macros.rs");

// Generate all standard integration tests for Together AI
generate_language_model_tests!(
    provider: TogetherAI,
    api_key_var: "TOGETHER_API_KEY",
    model_struct: MetaLlamaLlama3370bInstructTurbo,
    default_model: TogetherAI::meta_llama_llama_3_3_70b_instruct_turbo(),
    tool_model: TogetherAI::meta_llama_llama_3_3_70b_instruct_turbo(),
    structured_output_model: TogetherAI::meta_llama_llama_3_3_70b_instruct_turbo(),
    reasoning_model: TogetherAI::deepseek_ai_deepseek_r1(),
    skip_reasoning: false,
    skip_tool: false,
    skip_structured_output: true,  // Together AI doesn't seem to have structured output models
    skip_streaming: false
);
