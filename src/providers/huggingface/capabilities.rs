//! Capabilities for huggingface models.
//!
//! This module defines model types and their capabilities for huggingface providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::huggingface::Huggingface;

model_capabilities! {
    provider: Huggingface,
    models: {
        MinimaxaiMinimaxM21 {
            model_name: "MiniMaxAI/MiniMax-M2.1",
            constructor_name: minimaxai_minimax_m2_1,
            display_name: "MiniMax-M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bThinking2507 {
            model_name: "Qwen/Qwen3-235B-A22B-Thinking-2507",
            constructor_name: qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3-235B-A22B-Thinking-2507",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder480bA35bInstruct {
            model_name: "Qwen/Qwen3-Coder-480B-A35B-Instruct",
            constructor_name: qwen_qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3-Coder-480B-A35B-Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Embedding4b {
            model_name: "Qwen/Qwen3-Embedding-4B",
            constructor_name: qwen_qwen3_embedding_4b,
            display_name: "Qwen 3 Embedding 4B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Embedding8b {
            model_name: "Qwen/Qwen3-Embedding-8B",
            constructor_name: qwen_qwen3_embedding_8b,
            display_name: "Qwen 3 Embedding 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Next80bA3bInstruct {
            model_name: "Qwen/Qwen3-Next-80B-A3B-Instruct",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3-Next-80B-A3B-Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bThinking {
            model_name: "Qwen/Qwen3-Next-80B-A3B-Thinking",
            constructor_name: qwen_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3-Next-80B-A3B-Thinking",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomimimoMimoV2Flash {
            model_name: "XiaomiMiMo/MiMo-V2-Flash",
            constructor_name: xiaomimimo_mimo_v2_flash,
            display_name: "MiMo-V2-Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR10528 {
            model_name: "deepseek-ai/DeepSeek-R1-0528",
            constructor_name: deepseek_ai_deepseek_r1_0528,
            display_name: "DeepSeek-R1-0528",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV32 {
            model_name: "deepseek-ai/DeepSeek-V3.2",
            constructor_name: deepseek_ai_deepseek_v3_2,
            display_name: "DeepSeek-V3.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct {
            model_name: "moonshotai/Kimi-K2-Instruct",
            constructor_name: moonshotai_kimi_k2_instruct,
            display_name: "Kimi-K2-Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct0905 {
            model_name: "moonshotai/Kimi-K2-Instruct-0905",
            constructor_name: moonshotai_kimi_k2_instruct_0905,
            display_name: "Kimi-K2-Instruct-0905",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/Kimi-K2-Thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi-K2-Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/Kimi-K2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi-K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZaiOrgGlm47 {
            model_name: "zai-org/GLM-4.7",
            constructor_name: zai_org_glm_4_7,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Flash {
            model_name: "zai-org/GLM-4.7-Flash",
            constructor_name: zai_org_glm_4_7_flash,
            display_name: "GLM-4.7-Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5 {
            model_name: "zai-org/GLM-5",
            constructor_name: zai_org_glm_5,
            display_name: "GLM-5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
