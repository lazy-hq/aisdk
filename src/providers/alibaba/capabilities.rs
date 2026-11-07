//! Capabilities for alibaba models.
//!
//! This module defines model types and their capabilities for alibaba providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::alibaba::Alibaba;

model_capabilities! {
    provider: Alibaba,
    models: {
        DeepseekV4Flash0731 {
            model_name: "deepseek-v4-flash-0731",
            constructor_name: deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52 {
            model_name: "glm-5.2",
            constructor_name: glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK3 {
            model_name: "kimi-k3",
            constructor_name: kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QvqMax {
            model_name: "qvq-max",
            constructor_name: qvq_max,
            display_name: "QVQ Max",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenFlash {
            model_name: "qwen-flash",
            constructor_name: qwen_flash,
            display_name: "Qwen Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenMax {
            model_name: "qwen-max",
            constructor_name: qwen_max,
            display_name: "Qwen Max",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenMtPlus {
            model_name: "qwen-mt-plus",
            constructor_name: qwen_mt_plus,
            display_name: "Qwen-MT Plus",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenMtTurbo {
            model_name: "qwen-mt-turbo",
            constructor_name: qwen_mt_turbo,
            display_name: "Qwen-MT Turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenOmniTurbo {
            model_name: "qwen-omni-turbo",
            constructor_name: qwen_omni_turbo,
            display_name: "Qwen-Omni Turbo",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenOmniTurboRealtime {
            model_name: "qwen-omni-turbo-realtime",
            constructor_name: qwen_omni_turbo_realtime,
            display_name: "Qwen-Omni Turbo Realtime",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenPlus {
            model_name: "qwen-plus",
            constructor_name: qwen_plus,
            display_name: "Qwen Plus",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenPlusCharacterJa {
            model_name: "qwen-plus-character-ja",
            constructor_name: qwen_plus_character_ja,
            display_name: "Qwen Plus Character (Japanese)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenTurbo {
            model_name: "qwen-turbo",
            constructor_name: qwen_turbo,
            display_name: "Qwen Turbo",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenVlMax {
            model_name: "qwen-vl-max",
            constructor_name: qwen_vl_max,
            display_name: "Qwen-VL Max",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenVlOcr {
            model_name: "qwen-vl-ocr",
            constructor_name: qwen_vl_ocr,
            display_name: "Qwen-VL OCR",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenVlPlus {
            model_name: "qwen-vl-plus",
            constructor_name: qwen_vl_plus,
            display_name: "Qwen-VL Plus",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen2514bInstruct {
            model_name: "qwen2-5-14b-instruct",
            constructor_name: qwen2_5_14b_instruct,
            display_name: "Qwen2.5 14B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen2532bInstruct {
            model_name: "qwen2-5-32b-instruct",
            constructor_name: qwen2_5_32b_instruct,
            display_name: "Qwen2.5 32B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen2572bInstruct {
            model_name: "qwen2-5-72b-instruct",
            constructor_name: qwen2_5_72b_instruct,
            display_name: "Qwen2.5 72B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen257bInstruct {
            model_name: "qwen2-5-7b-instruct",
            constructor_name: qwen2_5_7b_instruct,
            display_name: "Qwen2.5 7B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen25Omni7b {
            model_name: "qwen2-5-omni-7b",
            constructor_name: qwen2_5_omni_7b,
            display_name: "Qwen2.5-Omni 7B",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen25Vl72bInstruct {
            model_name: "qwen2-5-vl-72b-instruct",
            constructor_name: qwen2_5_vl_72b_instruct,
            display_name: "Qwen2.5-VL 72B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen25Vl7bInstruct {
            model_name: "qwen2-5-vl-7b-instruct",
            constructor_name: qwen2_5_vl_7b_instruct,
            display_name: "Qwen2.5-VL 7B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen314b {
            model_name: "qwen3-14b",
            constructor_name: qwen3_14b,
            display_name: "Qwen3 14B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3235bA22b {
            model_name: "qwen3-235b-a22b",
            constructor_name: qwen3_235b_a22b,
            display_name: "Qwen3 235B-A22B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen332b {
            model_name: "qwen3-32b",
            constructor_name: qwen3_32b,
            display_name: "Qwen3 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen38b {
            model_name: "qwen3-8b",
            constructor_name: qwen3_8b,
            display_name: "Qwen3 8B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3AsrFlash {
            model_name: "qwen3-asr-flash",
            constructor_name: qwen3_asr_flash,
            display_name: "Qwen3-ASR Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        Qwen3Coder30bA3bInstruct {
            model_name: "qwen3-coder-30b-a3b-instruct",
            constructor_name: qwen3_coder_30b_a3b_instruct,
            display_name: "Qwen3-Coder 30B-A3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder480bA35bInstruct {
            model_name: "qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3-Coder 480B-A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3CoderFlash {
            model_name: "qwen3-coder-flash",
            constructor_name: qwen3_coder_flash,
            display_name: "Qwen3 Coder Flash",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3CoderPlus {
            model_name: "qwen3-coder-plus",
            constructor_name: qwen3_coder_plus,
            display_name: "Qwen3 Coder Plus",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3LivetranslateFlashRealtime {
            model_name: "qwen3-livetranslate-flash-realtime",
            constructor_name: qwen3_livetranslate_flash_realtime,
            display_name: "Qwen3-LiveTranslate Flash Realtime",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen3Max {
            model_name: "qwen3-max",
            constructor_name: qwen3_max,
            display_name: "Qwen3 Max",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Next80bA3bInstruct {
            model_name: "qwen3-next-80b-a3b-instruct",
            constructor_name: qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3-Next 80B-A3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Next80bA3bThinking {
            model_name: "qwen3-next-80b-a3b-thinking",
            constructor_name: qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3-Next 80B-A3B (Thinking)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3OmniFlash {
            model_name: "qwen3-omni-flash",
            constructor_name: qwen3_omni_flash,
            display_name: "Qwen3-Omni Flash",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3OmniFlashRealtime {
            model_name: "qwen3-omni-flash-realtime",
            constructor_name: qwen3_omni_flash_realtime,
            display_name: "Qwen3-Omni Flash Realtime",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3Vl235bA22b {
            model_name: "qwen3-vl-235b-a22b",
            constructor_name: qwen3_vl_235b_a22b,
            display_name: "Qwen3-VL 235B-A22B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Vl30bA3b {
            model_name: "qwen3-vl-30b-a3b",
            constructor_name: qwen3_vl_30b_a3b,
            display_name: "Qwen3-VL 30B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3VlPlus {
            model_name: "qwen3-vl-plus",
            constructor_name: qwen3_vl_plus,
            display_name: "Qwen3-VL Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen35122bA10b {
            model_name: "qwen3.5-122b-a10b",
            constructor_name: qwen3_5_122b_a10b,
            display_name: "Qwen3.5 122B-A10B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3527b {
            model_name: "qwen3.5-27b",
            constructor_name: qwen3_5_27b,
            display_name: "Qwen3.5 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3535bA3b {
            model_name: "qwen3.5-35b-a3b",
            constructor_name: qwen3_5_35b_a3b,
            display_name: "Qwen3.5 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen35397bA17b {
            model_name: "qwen3.5-397b-a17b",
            constructor_name: qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B-A17B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen35Flash {
            model_name: "qwen3.5-flash",
            constructor_name: qwen3_5_flash,
            display_name: "Qwen3.5 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen35Plus {
            model_name: "qwen3.5-plus",
            constructor_name: qwen3_5_plus,
            display_name: "Qwen3.5 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3627b {
            model_name: "qwen3.6-27b",
            constructor_name: qwen3_6_27b,
            display_name: "Qwen3.6 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3635bA3b {
            model_name: "qwen3.6-35b-a3b",
            constructor_name: qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen36Flash {
            model_name: "qwen3.6-flash",
            constructor_name: qwen3_6_flash,
            display_name: "Qwen3.6 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen36Plus {
            model_name: "qwen3.6-plus",
            constructor_name: qwen3_6_plus,
            display_name: "Qwen3.6 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen37Flash {
            model_name: "qwen3.7-flash",
            constructor_name: qwen3_7_flash,
            display_name: "Qwen3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen37Max {
            model_name: "qwen3.7-max",
            constructor_name: qwen3_7_max,
            display_name: "Qwen3.7 Max",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen37Plus {
            model_name: "qwen3.7-plus",
            constructor_name: qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38Flash {
            model_name: "qwen3.8-flash",
            constructor_name: qwen3_8_flash,
            display_name: "Qwen3.8 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38Max {
            model_name: "qwen3.8-max",
            constructor_name: qwen3_8_max,
            display_name: "Qwen3.8 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38OmniFlash {
            model_name: "qwen3.8-omni-flash",
            constructor_name: qwen3_8_omni_flash,
            display_name: "Qwen3.8 Omni Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwqPlus {
            model_name: "qwq-plus",
            constructor_name: qwq_plus,
            display_name: "QwQ Plus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
