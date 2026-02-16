//! Capabilities for cortecs models.
//!
//! This module defines model types and their capabilities for cortecs providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::cortecs::Cortecs;

model_capabilities! {
    provider: Cortecs,
    models: {
        Claude45Sonnet {
            model_name: "claude-4-5-sonnet",
            constructor_name: claude_4_5_sonnet,
            display_name: "Claude 4.5 Sonnet",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4 {
            model_name: "claude-sonnet-4",
            constructor_name: claude_sonnet_4,
            display_name: "Claude Sonnet 4",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV30324 {
            model_name: "deepseek-v3-0324",
            constructor_name: deepseek_v3_0324,
            display_name: "DeepSeek V3 0324",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Devstral2512 {
            model_name: "devstral-2512",
            constructor_name: devstral_2512,
            display_name: "Devstral 2 2512",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DevstralSmall2512 {
            model_name: "devstral-small-2512",
            constructor_name: devstral_small_2512,
            display_name: "Devstral Small 2 2512",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25Pro {
            model_name: "gemini-2.5-pro",
            constructor_name: gemini_2_5_pro,
            display_name: "Gemini 2.5 Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm4p5 {
            model_name: "glm-4p5",
            constructor_name: glm_4p5,
            display_name: "GLM 4.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm4p5Air {
            model_name: "glm-4p5-air",
            constructor_name: glm_4p5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm4p7 {
            model_name: "glm-4p7",
            constructor_name: glm_4p7,
            display_name: "GLM 4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41 {
            model_name: "gpt-4.1",
            constructor_name: gpt_4_1,
            display_name: "GPT 4.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT Oss 120b",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Intellect3 {
            model_name: "intellect-3",
            constructor_name: intellect_3,
            display_name: "INTELLECT 3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2Instruct {
            model_name: "kimi-k2-instruct",
            constructor_name: kimi_k2_instruct,
            display_name: "Kimi K2 Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2Thinking {
            model_name: "kimi-k2-thinking",
            constructor_name: kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama31405bInstruct {
            model_name: "llama-3.1-405b-instruct",
            constructor_name: llama_3_1_405b_instruct,
            display_name: "Llama 3.1 405B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM2 {
            model_name: "minimax-m2",
            constructor_name: minimax_m2,
            display_name: "MiniMax-M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM2p1 {
            model_name: "minimax-m2p1",
            constructor_name: minimax_m2p1,
            display_name: "MiniMax-M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NovaProV1 {
            model_name: "nova-pro-v1",
            constructor_name: nova_pro_v1,
            display_name: "Nova Pro 1.0",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen332b {
            model_name: "qwen3-32b",
            constructor_name: qwen3_32b,
            display_name: "Qwen3 32B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder480bA35bInstruct {
            model_name: "qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Next80bA3bThinking {
            model_name: "qwen3-next-80b-a3b-thinking",
            constructor_name: qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3 Next 80B A3B Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
