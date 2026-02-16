//! Capabilities for stackit models.
//!
//! This module defines model types and their capabilities for stackit providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::stackit::Stackit;

model_capabilities! {
    provider: Stackit,
    models: {
        QwenQwen3Vl235bA22bInstructFp8 {
            model_name: "Qwen/Qwen3-VL-235B-A22B-Instruct-FP8",
            constructor_name: qwen_qwen3_vl_235b_a22b_instruct_fp8,
            display_name: "Qwen3-VL 235B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3VlEmbedding8b {
            model_name: "Qwen/Qwen3-VL-Embedding-8B",
            constructor_name: qwen_qwen3_vl_embedding_8b,
            display_name: "Qwen3-VL Embedding 8B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        CortecsLlama3370bInstructFp8Dynamic {
            model_name: "cortecs/Llama-3.3-70B-Instruct-FP8-Dynamic",
            constructor_name: cortecs_llama_3_3_70b_instruct_fp8_dynamic,
            display_name: "Llama 3.3 70B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma327bIt {
            model_name: "google/gemma-3-27b-it",
            constructor_name: google_gemma_3_27b_it,
            display_name: "Gemma 3 27B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        IntfloatE5Mistral7bInstruct {
            model_name: "intfloat/e5-mistral-7b-instruct",
            constructor_name: intfloat_e5_mistral_7b_instruct,
            display_name: "E5 Mistral 7B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NeuralmagicMetaLlama318bInstructFp8 {
            model_name: "neuralmagic/Meta-Llama-3.1-8B-Instruct-FP8",
            constructor_name: neuralmagic_meta_llama_3_1_8b_instruct_fp8,
            display_name: "Llama 3.1 8B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NeuralmagicMistralNemoInstruct2407Fp8 {
            model_name: "neuralmagic/Mistral-Nemo-Instruct-2407-FP8",
            constructor_name: neuralmagic_mistral_nemo_instruct_2407_fp8,
            display_name: "Mistral Nemo",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT-OSS 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
