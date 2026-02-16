//! Capabilities for baseten models.
//!
//! This module defines model types and their capabilities for baseten providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::baseten::Baseten;

model_capabilities! {
    provider: Baseten,
    models: {
        QwenQwen3Coder480bA35bInstruct {
            model_name: "Qwen/Qwen3-Coder-480B-A35B-Instruct",
            constructor_name: qwen_qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV32 {
            model_name: "deepseek-ai/DeepSeek-V3.2",
            constructor_name: deepseek_ai_deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct0905 {
            model_name: "moonshotai/Kimi-K2-Instruct-0905",
            constructor_name: moonshotai_kimi_k2_instruct_0905,
            display_name: "Kimi K2 Instruct 0905",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/Kimi-K2-Thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/Kimi-K2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46 {
            model_name: "zai-org/GLM-4.6",
            constructor_name: zai_org_glm_4_6,
            display_name: "GLM 4.6",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47 {
            model_name: "zai-org/GLM-4.7",
            constructor_name: zai_org_glm_4_7,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
