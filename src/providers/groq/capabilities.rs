//! Capabilities for groq models.
//!
//! This module defines model types and their capabilities for groq providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::groq::Groq;

model_capabilities! {
    provider: Groq,
    models: {
        DeepseekR1DistillLlama70b {
            model_name: "deepseek-r1-distill-llama-70b",
            constructor_name: deepseek_r1_distill_llama_70b,
            display_name: "DeepSeek R1 Distill Llama 70B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma29b {
            model_name: "gemma-2-9b",
            constructor_name: gemma_2_9b,
            display_name: "Gemma 2 9B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama318bInstant {
            model_name: "llama-3.1-8b-instant",
            constructor_name: llama_3_1_8b_instant,
            display_name: "Llama 3.1 8B Instant",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama3370bVersatile {
            model_name: "llama-3.3-70b-versatile",
            constructor_name: llama_3_3_70b_versatile,
            display_name: "Llama 3.3 70B Versatile",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama370b {
            model_name: "llama-3-70b",
            constructor_name: llama_3_70b,
            display_name: "Llama 3 70B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama38b {
            model_name: "llama-3-8b",
            constructor_name: llama_3_8b,
            display_name: "Llama 3 8B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LlamaGuard38b {
            model_name: "llama-guard-3-8b",
            constructor_name: llama_guard_3_8b,
            display_name: "Llama Guard 3 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralSaba24b {
            model_name: "mistral-saba-24b",
            constructor_name: mistral_saba_24b,
            display_name: "Mistral Saba 24B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwq32b {
            model_name: "qwen-qwq-32b",
            constructor_name: qwen_qwq_32b,
            display_name: "Qwen QwQ 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
