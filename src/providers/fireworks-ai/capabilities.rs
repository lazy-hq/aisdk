//! Capabilities for fireworks_ai models.
//!
//! This module defines model types and their capabilities for fireworks_ai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::fireworks_ai::FireworksAi;

model_capabilities! {
    provider: FireworksAi,
    models: {
        ModelsDeepseekR10528 {
            model_name: "accounts/fireworks/models/deepseek-r1-0528",
            constructor_name: models_deepseek_r1_0528,
            display_name: "Deepseek R1 05/28",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsDeepseekV30324 {
            model_name: "accounts/fireworks/models/deepseek-v3-0324",
            constructor_name: models_deepseek_v3_0324,
            display_name: "Deepseek V3 03-24",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsDeepseekV3p1 {
            model_name: "accounts/fireworks/models/deepseek-v3p1",
            constructor_name: models_deepseek_v3p1,
            display_name: "DeepSeek V3.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsDeepseekV3p2 {
            model_name: "accounts/fireworks/models/deepseek-v3p2",
            constructor_name: models_deepseek_v3p2,
            display_name: "DeepSeek V3.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsGlm4p5 {
            model_name: "accounts/fireworks/models/glm-4p5",
            constructor_name: models_glm_4p5,
            display_name: "GLM 4.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsGlm4p5Air {
            model_name: "accounts/fireworks/models/glm-4p5-air",
            constructor_name: models_glm_4p5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsGlm4p6 {
            model_name: "accounts/fireworks/models/glm-4p6",
            constructor_name: models_glm_4p6,
            display_name: "GLM 4.6",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsGlm4p7 {
            model_name: "accounts/fireworks/models/glm-4p7",
            constructor_name: models_glm_4p7,
            display_name: "GLM 4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsGlm5 {
            model_name: "accounts/fireworks/models/glm-5",
            constructor_name: models_glm_5,
            display_name: "GLM 5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsGptOss120b {
            model_name: "accounts/fireworks/models/gpt-oss-120b",
            constructor_name: models_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsGptOss20b {
            model_name: "accounts/fireworks/models/gpt-oss-20b",
            constructor_name: models_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsKimiK2Instruct {
            model_name: "accounts/fireworks/models/kimi-k2-instruct",
            constructor_name: models_kimi_k2_instruct,
            display_name: "Kimi K2 Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsKimiK2Thinking {
            model_name: "accounts/fireworks/models/kimi-k2-thinking",
            constructor_name: models_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsKimiK2p5 {
            model_name: "accounts/fireworks/models/kimi-k2p5",
            constructor_name: models_kimi_k2p5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ModelsMinimaxM2 {
            model_name: "accounts/fireworks/models/minimax-m2",
            constructor_name: models_minimax_m2,
            display_name: "MiniMax-M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsMinimaxM2p1 {
            model_name: "accounts/fireworks/models/minimax-m2p1",
            constructor_name: models_minimax_m2p1,
            display_name: "MiniMax-M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsMinimaxM2p5 {
            model_name: "accounts/fireworks/models/minimax-m2p5",
            constructor_name: models_minimax_m2p5,
            display_name: "MiniMax-M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsQwen3235bA22b {
            model_name: "accounts/fireworks/models/qwen3-235b-a22b",
            constructor_name: models_qwen3_235b_a22b,
            display_name: "Qwen3 235B-A22B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ModelsQwen3Coder480bA35bInstruct {
            model_name: "accounts/fireworks/models/qwen3-coder-480b-a35b-instruct",
            constructor_name: models_qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
