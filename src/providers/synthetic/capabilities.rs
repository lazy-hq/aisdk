//! Capabilities for synthetic models.
//!
//! This module defines model types and their capabilities for synthetic providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::synthetic::Synthetic;

model_capabilities! {
    provider: Synthetic,
    models: {
        HfMinimaxaiMinimaxM2 {
            model_name: "hf:MiniMaxAI/MiniMax-M2",
            constructor_name: hf_minimaxai_minimax_m2,
            display_name: "MiniMax-M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfMinimaxaiMinimaxM21 {
            model_name: "hf:MiniMaxAI/MiniMax-M2.1",
            constructor_name: hf_minimaxai_minimax_m2_1,
            display_name: "MiniMax-M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfQwenQwen25Coder32bInstruct {
            model_name: "hf:Qwen/Qwen2.5-Coder-32B-Instruct",
            constructor_name: hf_qwen_qwen2_5_coder_32b_instruct,
            display_name: "Qwen2.5-Coder-32B-Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        HfQwenQwen3235bA22bInstruct2507 {
            model_name: "hf:Qwen/Qwen3-235B-A22B-Instruct-2507",
            constructor_name: hf_qwen_qwen3_235b_a22b_instruct_2507,
            display_name: "Qwen 3 235B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfQwenQwen3235bA22bThinking2507 {
            model_name: "hf:Qwen/Qwen3-235B-A22B-Thinking-2507",
            constructor_name: hf_qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3 235B A22B Thinking 2507",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfQwenQwen3Coder480bA35bInstruct {
            model_name: "hf:Qwen/Qwen3-Coder-480B-A35B-Instruct",
            constructor_name: hf_qwen_qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen 3 Coder 480B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfDeepseekAiDeepseekR1 {
            model_name: "hf:deepseek-ai/DeepSeek-R1",
            constructor_name: hf_deepseek_ai_deepseek_r1,
            display_name: "DeepSeek R1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfDeepseekAiDeepseekR10528 {
            model_name: "hf:deepseek-ai/DeepSeek-R1-0528",
            constructor_name: hf_deepseek_ai_deepseek_r1_0528,
            display_name: "DeepSeek R1 (0528)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfDeepseekAiDeepseekV3 {
            model_name: "hf:deepseek-ai/DeepSeek-V3",
            constructor_name: hf_deepseek_ai_deepseek_v3,
            display_name: "DeepSeek V3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfDeepseekAiDeepseekV30324 {
            model_name: "hf:deepseek-ai/DeepSeek-V3-0324",
            constructor_name: hf_deepseek_ai_deepseek_v3_0324,
            display_name: "DeepSeek V3 (0324)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfDeepseekAiDeepseekV31 {
            model_name: "hf:deepseek-ai/DeepSeek-V3.1",
            constructor_name: hf_deepseek_ai_deepseek_v3_1,
            display_name: "DeepSeek V3.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfDeepseekAiDeepseekV31Terminus {
            model_name: "hf:deepseek-ai/DeepSeek-V3.1-Terminus",
            constructor_name: hf_deepseek_ai_deepseek_v3_1_terminus,
            display_name: "DeepSeek V3.1 Terminus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfDeepseekAiDeepseekV32 {
            model_name: "hf:deepseek-ai/DeepSeek-V3.2",
            constructor_name: hf_deepseek_ai_deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfMetaLlamaLlama31405bInstruct {
            model_name: "hf:meta-llama/Llama-3.1-405B-Instruct",
            constructor_name: hf_meta_llama_llama_3_1_405b_instruct,
            display_name: "Llama-3.1-405B-Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfMetaLlamaLlama3170bInstruct {
            model_name: "hf:meta-llama/Llama-3.1-70B-Instruct",
            constructor_name: hf_meta_llama_llama_3_1_70b_instruct,
            display_name: "Llama-3.1-70B-Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfMetaLlamaLlama318bInstruct {
            model_name: "hf:meta-llama/Llama-3.1-8B-Instruct",
            constructor_name: hf_meta_llama_llama_3_1_8b_instruct,
            display_name: "Llama-3.1-8B-Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfMetaLlamaLlama3370bInstruct {
            model_name: "hf:meta-llama/Llama-3.3-70B-Instruct",
            constructor_name: hf_meta_llama_llama_3_3_70b_instruct,
            display_name: "Llama-3.3-70B-Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfMetaLlamaLlama4Maverick17b128eInstructFp8 {
            model_name: "hf:meta-llama/Llama-4-Maverick-17B-128E-Instruct-FP8",
            constructor_name: hf_meta_llama_llama_4_maverick_17b_128e_instruct_fp8,
            display_name: "Llama-4-Maverick-17B-128E-Instruct-FP8",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfMetaLlamaLlama4Scout17b16eInstruct {
            model_name: "hf:meta-llama/Llama-4-Scout-17B-16E-Instruct",
            constructor_name: hf_meta_llama_llama_4_scout_17b_16e_instruct,
            display_name: "Llama-4-Scout-17B-16E-Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfMoonshotaiKimiK2Instruct0905 {
            model_name: "hf:moonshotai/Kimi-K2-Instruct-0905",
            constructor_name: hf_moonshotai_kimi_k2_instruct_0905,
            display_name: "Kimi K2 0905",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfMoonshotaiKimiK2Thinking {
            model_name: "hf:moonshotai/Kimi-K2-Thinking",
            constructor_name: hf_moonshotai_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfMoonshotaiKimiK25 {
            model_name: "hf:moonshotai/Kimi-K2.5",
            constructor_name: hf_moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfNvidiaKimiK25Nvfp4 {
            model_name: "hf:nvidia/Kimi-K2.5-NVFP4",
            constructor_name: hf_nvidia_kimi_k2_5_nvfp4,
            display_name: "Kimi K2.5 (NVFP4)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfOpenaiGptOss120b {
            model_name: "hf:openai/gpt-oss-120b",
            constructor_name: hf_openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfZaiOrgGlm46 {
            model_name: "hf:zai-org/GLM-4.6",
            constructor_name: hf_zai_org_glm_4_6,
            display_name: "GLM 4.6",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HfZaiOrgGlm47 {
            model_name: "hf:zai-org/GLM-4.7",
            constructor_name: hf_zai_org_glm_4_7,
            display_name: "GLM 4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
