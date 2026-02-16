//! Capabilities for alibaba_cn models.
//!
//! This module defines model types and their capabilities for alibaba_cn providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::alibaba_cn::AlibabaCn;

model_capabilities! {
    provider: AlibabaCn,
    models: {
        DeepseekR1 {
            model_name: "deepseek-r1",
            constructor_name: deepseek_r1,
            display_name: "DeepSeek R1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR10528 {
            model_name: "deepseek-r1-0528",
            constructor_name: deepseek_r1_0528,
            display_name: "DeepSeek R1 0528",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR1DistillLlama70b {
            model_name: "deepseek-r1-distill-llama-70b",
            constructor_name: deepseek_r1_distill_llama_70b,
            display_name: "DeepSeek R1 Distill Llama 70B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR1DistillLlama8b {
            model_name: "deepseek-r1-distill-llama-8b",
            constructor_name: deepseek_r1_distill_llama_8b,
            display_name: "DeepSeek R1 Distill Llama 8B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR1DistillQwen15b {
            model_name: "deepseek-r1-distill-qwen-1-5b",
            constructor_name: deepseek_r1_distill_qwen_1_5b,
            display_name: "DeepSeek R1 Distill Qwen 1.5B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR1DistillQwen14b {
            model_name: "deepseek-r1-distill-qwen-14b",
            constructor_name: deepseek_r1_distill_qwen_14b,
            display_name: "DeepSeek R1 Distill Qwen 14B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR1DistillQwen32b {
            model_name: "deepseek-r1-distill-qwen-32b",
            constructor_name: deepseek_r1_distill_qwen_32b,
            display_name: "DeepSeek R1 Distill Qwen 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR1DistillQwen7b {
            model_name: "deepseek-r1-distill-qwen-7b",
            constructor_name: deepseek_r1_distill_qwen_7b,
            display_name: "DeepSeek R1 Distill Qwen 7B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV3 {
            model_name: "deepseek-v3",
            constructor_name: deepseek_v3,
            display_name: "DeepSeek V3",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV31 {
            model_name: "deepseek-v3-1",
            constructor_name: deepseek_v3_1,
            display_name: "DeepSeek V3.1",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV32Exp {
            model_name: "deepseek-v3-2-exp",
            constructor_name: deepseek_v3_2_exp,
            display_name: "DeepSeek V3.2 Exp",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2Thinking {
            model_name: "kimi-k2-thinking",
            constructor_name: kimi_k2_thinking,
            display_name: "Moonshot Kimi K2 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK25 {
            model_name: "kimi-k2.5",
            constructor_name: kimi_k2_5,
            display_name: "Moonshot Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotKimiK2Instruct {
            model_name: "moonshot-kimi-k2-instruct",
            constructor_name: moonshot_kimi_k2_instruct,
            display_name: "Moonshot Kimi K2 Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QvqMax {
            model_name: "qvq-max",
            constructor_name: qvq_max,
            display_name: "QVQ Max",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenDeepResearch {
            model_name: "qwen-deep-research",
            constructor_name: qwen_deep_research,
            display_name: "Qwen Deep Research",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenDocTurbo {
            model_name: "qwen-doc-turbo",
            constructor_name: qwen_doc_turbo,
            display_name: "Qwen Doc Turbo",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenFlash {
            model_name: "qwen-flash",
            constructor_name: qwen_flash,
            display_name: "Qwen Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenLong {
            model_name: "qwen-long",
            constructor_name: qwen_long,
            display_name: "Qwen Long",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenMathPlus {
            model_name: "qwen-math-plus",
            constructor_name: qwen_math_plus,
            display_name: "Qwen Math Plus",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenMathTurbo {
            model_name: "qwen-math-turbo",
            constructor_name: qwen_math_turbo,
            display_name: "Qwen Math Turbo",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenMax {
            model_name: "qwen-max",
            constructor_name: qwen_max,
            display_name: "Qwen Max",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenPlusCharacter {
            model_name: "qwen-plus-character",
            constructor_name: qwen_plus_character,
            display_name: "Qwen Plus Character",
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
        Qwen25Coder32bInstruct {
            model_name: "qwen2-5-coder-32b-instruct",
            constructor_name: qwen2_5_coder_32b_instruct,
            display_name: "Qwen2.5-Coder 32B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen25Coder7bInstruct {
            model_name: "qwen2-5-coder-7b-instruct",
            constructor_name: qwen2_5_coder_7b_instruct,
            display_name: "Qwen2.5-Coder 7B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen25Math72bInstruct {
            model_name: "qwen2-5-math-72b-instruct",
            constructor_name: qwen2_5_math_72b_instruct,
            display_name: "Qwen2.5-Math 72B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen25Math7bInstruct {
            model_name: "qwen2-5-math-7b-instruct",
            constructor_name: qwen2_5_math_7b_instruct,
            display_name: "Qwen2.5-Math 7B Instruct",
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
            capabilities: [AudioInputSupport, TextOutputSupport]
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
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3CoderPlus {
            model_name: "qwen3-coder-plus",
            constructor_name: qwen3_coder_plus,
            display_name: "Qwen3 Coder Plus",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwq32b {
            model_name: "qwq-32b",
            constructor_name: qwq_32b,
            display_name: "QwQ 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwqPlus {
            model_name: "qwq-plus",
            constructor_name: qwq_plus,
            display_name: "QwQ Plus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TongyiIntentDetectV3 {
            model_name: "tongyi-intent-detect-v3",
            constructor_name: tongyi_intent_detect_v3,
            display_name: "Tongyi Intent Detect V3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
    }
}
