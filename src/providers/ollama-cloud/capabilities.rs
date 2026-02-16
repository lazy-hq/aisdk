//! Capabilities for ollama_cloud models.
//!
//! This module defines model types and their capabilities for ollama_cloud providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::ollama_cloud::OllamaCloud;

model_capabilities! {
    provider: OllamaCloud,
    models: {
        Cogito21671b {
            model_name: "cogito-2.1:671b",
            constructor_name: cogito_2_1_671b,
            display_name: "cogito-2.1:671b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV31671b {
            model_name: "deepseek-v3.1:671b",
            constructor_name: deepseek_v3_1_671b,
            display_name: "deepseek-v3.1:671b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV32 {
            model_name: "deepseek-v3.2",
            constructor_name: deepseek_v3_2,
            display_name: "deepseek-v3.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Devstral2123b {
            model_name: "devstral-2:123b",
            constructor_name: devstral_2_123b,
            display_name: "devstral-2:123b",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DevstralSmall224b {
            model_name: "devstral-small-2:24b",
            constructor_name: devstral_small_2_24b,
            display_name: "devstral-small-2:24b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini3FlashPreview {
            model_name: "gemini-3-flash-preview",
            constructor_name: gemini_3_flash_preview,
            display_name: "gemini-3-flash-preview",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini3ProPreview {
            model_name: "gemini-3-pro-preview",
            constructor_name: gemini_3_pro_preview,
            display_name: "gemini-3-pro-preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma312b {
            model_name: "gemma3:12b",
            constructor_name: gemma3_12b,
            display_name: "gemma3:12b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemma327b {
            model_name: "gemma3:27b",
            constructor_name: gemma3_27b,
            display_name: "gemma3:27b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemma34b {
            model_name: "gemma3:4b",
            constructor_name: gemma3_4b,
            display_name: "gemma3:4b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Glm46 {
            model_name: "glm-4.6",
            constructor_name: glm_4_6,
            display_name: "glm-4.6",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm47 {
            model_name: "glm-4.7",
            constructor_name: glm_4_7,
            display_name: "glm-4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm5 {
            model_name: "glm-5",
            constructor_name: glm_5,
            display_name: "glm-5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss:120b",
            constructor_name: gpt_oss_120b,
            display_name: "gpt-oss:120b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss20b {
            model_name: "gpt-oss:20b",
            constructor_name: gpt_oss_20b,
            display_name: "gpt-oss:20b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2Thinking {
            model_name: "kimi-k2-thinking",
            constructor_name: kimi_k2_thinking,
            display_name: "kimi-k2-thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK25 {
            model_name: "kimi-k2.5",
            constructor_name: kimi_k2_5,
            display_name: "kimi-k2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK21t {
            model_name: "kimi-k2:1t",
            constructor_name: kimi_k2_1t,
            display_name: "kimi-k2:1t",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM2 {
            model_name: "minimax-m2",
            constructor_name: minimax_m2,
            display_name: "minimax-m2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM21 {
            model_name: "minimax-m2.1",
            constructor_name: minimax_m2_1,
            display_name: "minimax-m2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM25 {
            model_name: "minimax-m2.5",
            constructor_name: minimax_m2_5,
            display_name: "minimax-m2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral314b {
            model_name: "ministral-3:14b",
            constructor_name: ministral_3_14b,
            display_name: "ministral-3:14b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral33b {
            model_name: "ministral-3:3b",
            constructor_name: ministral_3_3b,
            display_name: "ministral-3:3b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral38b {
            model_name: "ministral-3:8b",
            constructor_name: ministral_3_8b,
            display_name: "ministral-3:8b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralLarge3675b {
            model_name: "mistral-large-3:675b",
            constructor_name: mistral_large_3_675b,
            display_name: "mistral-large-3:675b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Nemotron3Nano30b {
            model_name: "nemotron-3-nano:30b",
            constructor_name: nemotron_3_nano_30b,
            display_name: "nemotron-3-nano:30b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3CoderNext {
            model_name: "qwen3-coder-next",
            constructor_name: qwen3_coder_next,
            display_name: "qwen3-coder-next",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder480b {
            model_name: "qwen3-coder:480b",
            constructor_name: qwen3_coder_480b,
            display_name: "qwen3-coder:480b",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Next80b {
            model_name: "qwen3-next:80b",
            constructor_name: qwen3_next_80b,
            display_name: "qwen3-next:80b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Vl235b {
            model_name: "qwen3-vl:235b",
            constructor_name: qwen3_vl_235b,
            display_name: "qwen3-vl:235b",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Vl235bInstruct {
            model_name: "qwen3-vl:235b-instruct",
            constructor_name: qwen3_vl_235b_instruct,
            display_name: "qwen3-vl:235b-instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Rnj18b {
            model_name: "rnj-1:8b",
            constructor_name: rnj_1_8b,
            display_name: "rnj-1:8b",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
