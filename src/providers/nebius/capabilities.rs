//! Capabilities for nebius models.
//!
//! This module defines model types and their capabilities for nebius providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::nebius::Nebius;

model_capabilities! {
    provider: Nebius,
    models: {
        BaaiBgeEnIcl {
            model_name: "BAAI/bge-en-icl",
            constructor_name: baai_bge_en_icl,
            display_name: "BGE-ICL",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BaaiBgeMultilingualGemma2 {
            model_name: "BAAI/bge-multilingual-gemma2",
            constructor_name: baai_bge_multilingual_gemma2,
            display_name: "bge-multilingual-gemma2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MinimaxaiMinimaxM21 {
            model_name: "MiniMaxAI/minimax-m2.1",
            constructor_name: minimaxai_minimax_m2_1,
            display_name: "MiniMax-M2.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes4405b {
            model_name: "NousResearch/hermes-4-405b",
            constructor_name: nousresearch_hermes_4_405b,
            display_name: "Hermes-4-405B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes470b {
            model_name: "NousResearch/hermes-4-70b",
            constructor_name: nousresearch_hermes_4_70b,
            display_name: "Hermes-4-70B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PrimeintellectIntellect3 {
            model_name: "PrimeIntellect/intellect-3",
            constructor_name: primeintellect_intellect_3,
            display_name: "INTELLECT-3",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BlackForestLabsFluxDev {
            model_name: "black-forest-labs/flux-dev",
            constructor_name: black_forest_labs_flux_dev,
            display_name: "FLUX.1-dev",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BlackForestLabsFluxSchnell {
            model_name: "black-forest-labs/flux-schnell",
            constructor_name: black_forest_labs_flux_schnell,
            display_name: "FLUX.1-schnell",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        DeepseekAiDeepseekR10528 {
            model_name: "deepseek-ai/deepseek-r1-0528",
            constructor_name: deepseek_ai_deepseek_r1_0528,
            display_name: "DeepSeek-R1-0528",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR10528Fast {
            model_name: "deepseek-ai/deepseek-r1-0528-fast",
            constructor_name: deepseek_ai_deepseek_r1_0528_fast,
            display_name: "DeepSeek R1 0528 Fast",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV30324 {
            model_name: "deepseek-ai/deepseek-v3-0324",
            constructor_name: deepseek_ai_deepseek_v3_0324,
            display_name: "DeepSeek-V3-0324",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV30324Fast {
            model_name: "deepseek-ai/deepseek-v3-0324-fast",
            constructor_name: deepseek_ai_deepseek_v3_0324_fast,
            display_name: "DeepSeek-V3-0324 (Fast)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV32 {
            model_name: "deepseek-ai/deepseek-v3.2",
            constructor_name: deepseek_ai_deepseek_v3_2,
            display_name: "DeepSeek-V3.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma22bIt {
            model_name: "google/gemma-2-2b-it",
            constructor_name: google_gemma_2_2b_it,
            display_name: "Gemma-2-2b-it",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleGemma29bItFast {
            model_name: "google/gemma-2-9b-it-fast",
            constructor_name: google_gemma_2_9b_it_fast,
            display_name: "Gemma-2-9b-it (Fast)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleGemma327bIt {
            model_name: "google/gemma-3-27b-it",
            constructor_name: google_gemma_3_27b_it,
            display_name: "Gemma-3-27b-it",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma327bItFast {
            model_name: "google/gemma-3-27b-it-fast",
            constructor_name: google_gemma_3_27b_it_fast,
            display_name: "Gemma-3-27b-it (Fast)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        IntfloatE5Mistral7bInstruct {
            model_name: "intfloat/e5-mistral-7b-instruct",
            constructor_name: intfloat_e5_mistral_7b_instruct,
            display_name: "e5-mistral-7b-instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama3370bInstruct {
            model_name: "meta-llama/Llama-3.3-70B-Instruct",
            constructor_name: meta_llama_llama_3_3_70b_instruct,
            display_name: "Llama-3.3-70B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3370bInstructFast {
            model_name: "meta-llama/llama-3.3-70b-instruct-fast",
            constructor_name: meta_llama_llama_3_3_70b_instruct_fast,
            display_name: "Llama-3.3-70B-Instruct (Fast)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlamaGuard38b {
            model_name: "meta-llama/llama-guard-3-8b",
            constructor_name: meta_llama_llama_guard_3_8b,
            display_name: "Llama-Guard-3-8B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlamaMetaLlama318bInstruct {
            model_name: "meta-llama/meta-llama-3.1-8b-instruct",
            constructor_name: meta_llama_meta_llama_3_1_8b_instruct,
            display_name: "Meta-Llama-3.1-8B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaMetaLlama318bInstructFast {
            model_name: "meta-llama/meta-llama-3.1-8b-instruct-fast",
            constructor_name: meta_llama_meta_llama_3_1_8b_instruct_fast,
            display_name: "Meta-Llama-3.1-8B-Instruct (Fast)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/Kimi-K2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi-K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct {
            model_name: "moonshotai/kimi-k2-instruct",
            constructor_name: moonshotai_kimi_k2_instruct,
            display_name: "Kimi-K2-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/kimi-k2-thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi-K2-Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaLlama31NemotronUltra253bV1 {
            model_name: "nvidia/llama-3_1-nemotron-ultra-253b-v1",
            constructor_name: nvidia_llama_3_1_nemotron_ultra_253b_v1,
            display_name: "Llama-3.1-Nemotron-Ultra-253B-v1",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNanoV212b {
            model_name: "nvidia/nemotron-nano-v2-12b",
            constructor_name: nvidia_nemotron_nano_v2_12b,
            display_name: "Nemotron-Nano-V2-12b",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNvidiaNemotron3Nano30bA3b {
            model_name: "nvidia/nvidia-nemotron-3-nano-30b-a3b",
            constructor_name: nvidia_nvidia_nemotron_3_nano_30b_a3b,
            display_name: "Nemotron-3-Nano-30B-A3B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "gpt-oss-120b",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "gpt-oss-20b",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen25Coder7bFast {
            model_name: "qwen/qwen2.5-coder-7b-fast",
            constructor_name: qwen_qwen2_5_coder_7b_fast,
            display_name: "Qwen2.5-Coder-7B (Fast)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen25Vl72bInstruct {
            model_name: "qwen/qwen2.5-vl-72b-instruct",
            constructor_name: qwen_qwen2_5_vl_72b_instruct,
            display_name: "Qwen2.5-VL-72B-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bInstruct2507 {
            model_name: "qwen/qwen3-235b-a22b-instruct-2507",
            constructor_name: qwen_qwen3_235b_a22b_instruct_2507,
            display_name: "Qwen3 235B A22B Instruct 2507",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bThinking2507 {
            model_name: "qwen/qwen3-235b-a22b-thinking-2507",
            constructor_name: qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3 235B A22B Thinking 2507",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3bInstruct2507 {
            model_name: "qwen/qwen3-30b-a3b-instruct-2507",
            constructor_name: qwen_qwen3_30b_a3b_instruct_2507,
            display_name: "Qwen3-30B-A3B-Instruct-2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3bThinking2507 {
            model_name: "qwen/qwen3-30b-a3b-thinking-2507",
            constructor_name: qwen_qwen3_30b_a3b_thinking_2507,
            display_name: "Qwen3-30B-A3B-Thinking-2507",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen332b {
            model_name: "qwen/qwen3-32b",
            constructor_name: qwen_qwen3_32b,
            display_name: "Qwen3-32B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen332bFast {
            model_name: "qwen/qwen3-32b-fast",
            constructor_name: qwen_qwen3_32b_fast,
            display_name: "Qwen3-32B (Fast)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder30bA3bInstruct {
            model_name: "qwen/qwen3-coder-30b-a3b-instruct",
            constructor_name: qwen_qwen3_coder_30b_a3b_instruct,
            display_name: "Qwen3-Coder-30B-A3B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder480bA35bInstruct {
            model_name: "qwen/qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen_qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Embedding8b {
            model_name: "qwen/qwen3-embedding-8b",
            constructor_name: qwen_qwen3_embedding_8b,
            display_name: "Qwen3-Embedding-8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Next80bA3bThinking {
            model_name: "qwen/qwen3-next-80b-a3b-thinking",
            constructor_name: qwen_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3-Next-80B-A3B-Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45 {
            model_name: "zai-org/glm-4.5",
            constructor_name: zai_org_glm_4_5,
            display_name: "GLM-4.5",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45Air {
            model_name: "zai-org/glm-4.5-air",
            constructor_name: zai_org_glm_4_5_air,
            display_name: "GLM-4.5-Air",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Fp8 {
            model_name: "zai-org/glm-4.7-fp8",
            constructor_name: zai_org_glm_4_7_fp8,
            display_name: "GLM-4.7 (FP8)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
