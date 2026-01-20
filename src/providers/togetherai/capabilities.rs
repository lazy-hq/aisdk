//! Capabilities for togetherai models.
//!
//! This module defines model types and their capabilities for togetherai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::togetherai::TogetherAI;

model_capabilities! {
    provider: TogetherAI,
    models: {
        DeepseekAiDeepseekR1 {
            model_name: "deepseek-ai/DeepSeek-R1",
            constructor_name: deepseek_ai_deepseek_r1,
            display_name: "DeepSeek R1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekV3 {
            model_name: "deepseek-ai/DeepSeek-V3",
            constructor_name: deepseek_ai_deepseek_v3,
            display_name: "DeepSeek V3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31 {
            model_name: "deepseek-ai/DeepSeek-V3-1",
            constructor_name: deepseek_ai_deepseek_v3_1,
            display_name: "DeepSeek V3.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        EssentialaiRnj1Instruct {
            model_name: "essentialai/Rnj-1-Instruct",
            constructor_name: essentialai_rnj_1_instruct,
            display_name: "Rnj-1 Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3370bInstructTurbo {
            model_name: "meta-llama/Llama-3.3-70B-Instruct-Turbo",
            constructor_name: meta_llama_llama_3_3_70b_instruct_turbo,
            display_name: "Llama 3.3 70B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct {
            model_name: "moonshotai/Kimi-K2-Instruct",
            constructor_name: moonshotai_kimi_k2_instruct,
            display_name: "Kimi K2 Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/Kimi-K2-Thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder480bA35bInstructFp8 {
            model_name: "Qwen/Qwen3-Coder-480B-A35B-Instruct-FP8",
            constructor_name: qwen_qwen3_coder_480b_a35b_instruct_fp8,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46 {
            model_name: "zai-org/GLM-4.6",
            constructor_name: zai_org_glm_4_6,
            display_name: "GLM 4.6",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
