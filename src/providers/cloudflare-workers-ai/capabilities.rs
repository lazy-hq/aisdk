//! Capabilities for cloudflare_workers_ai models.
//!
//! This module defines model types and their capabilities for cloudflare_workers_ai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::cloudflare_workers_ai::CloudflareWorkersAi;

model_capabilities! {
    provider: CloudflareWorkersAi,
    models: {
        Ai4bharatIndictrans2EnIndic1b {
            model_name: "@cf/ai4bharat/indictrans2-en-indic-1B",
            constructor_name: ai4bharat_indictrans2_en_indic_1b,
            display_name: "IndicTrans2 EN-Indic 1B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AisingaporeGemmaSeaLionV427bIt {
            model_name: "@cf/aisingapore/gemma-sea-lion-v4-27b-it",
            constructor_name: aisingapore_gemma_sea_lion_v4_27b_it,
            display_name: "Gemma SEA-LION v4 27B IT",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BaaiBgeBaseEnV15 {
            model_name: "@cf/baai/bge-base-en-v1.5",
            constructor_name: baai_bge_base_en_v1_5,
            display_name: "BGE Base EN v1.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BaaiBgeLargeEnV15 {
            model_name: "@cf/baai/bge-large-en-v1.5",
            constructor_name: baai_bge_large_en_v1_5,
            display_name: "BGE Large EN v1.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BaaiBgeM3 {
            model_name: "@cf/baai/bge-m3",
            constructor_name: baai_bge_m3,
            display_name: "BGE M3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BaaiBgeRerankerBase {
            model_name: "@cf/baai/bge-reranker-base",
            constructor_name: baai_bge_reranker_base,
            display_name: "BGE Reranker Base",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BaaiBgeSmallEnV15 {
            model_name: "@cf/baai/bge-small-en-v1.5",
            constructor_name: baai_bge_small_en_v1_5,
            display_name: "BGE Small EN v1.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepgramAura2En {
            model_name: "@cf/deepgram/aura-2-en",
            constructor_name: deepgram_aura_2_en,
            display_name: "Deepgram Aura 2 (EN)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepgramAura2Es {
            model_name: "@cf/deepgram/aura-2-es",
            constructor_name: deepgram_aura_2_es,
            display_name: "Deepgram Aura 2 (ES)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepgramNova3 {
            model_name: "@cf/deepgram/nova-3",
            constructor_name: deepgram_nova_3,
            display_name: "Deepgram Nova 3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekR1DistillQwen32b {
            model_name: "@cf/deepseek-ai/deepseek-r1-distill-qwen-32b",
            constructor_name: deepseek_ai_deepseek_r1_distill_qwen_32b,
            display_name: "DeepSeek R1 Distill Qwen 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        CfDeepseekAiDeepseekV4Flash0731 {
            model_name: "@cf/deepseek-ai/deepseek-v4-flash-0731",
            constructor_name: cf_deepseek_ai_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CfDeepseekAiDeepseekV4Pro0813 {
            model_name: "@cf/deepseek-ai/deepseek-v4-pro-0813",
            constructor_name: cf_deepseek_ai_deepseek_v4_pro_0813,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CfDeepseekAiDeepseekV41Flash1017 {
            model_name: "@cf/deepseek-ai/deepseek-v4.1-flash-1017",
            constructor_name: cf_deepseek_ai_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        FacebookBartLargeCnn {
            model_name: "@cf/facebook/bart-large-cnn",
            constructor_name: facebook_bart_large_cnn,
            display_name: "BART Large CNN",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleGemma312bIt {
            model_name: "@cf/google/gemma-3-12b-it",
            constructor_name: google_gemma_3_12b_it,
            display_name: "Gemma 3 12B IT",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CfGoogleGemma426bA4bIt {
            model_name: "@cf/google/gemma-4-26b-a4b-it",
            constructor_name: cf_google_gemma_4_26b_a4b_it,
            display_name: "Gemma 4 26B A4B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HuggingfaceDistilbertSst2Int8 {
            model_name: "@cf/huggingface/distilbert-sst-2-int8",
            constructor_name: huggingface_distilbert_sst_2_int8,
            display_name: "DistilBERT SST-2 INT8",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        IbmGraniteGranite40HMicro {
            model_name: "@cf/ibm-granite/granite-4.0-h-micro",
            constructor_name: ibm_granite_granite_4_0_h_micro,
            display_name: "IBM Granite 4.0 H Micro",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama27bChatFp16 {
            model_name: "@cf/meta/llama-2-7b-chat-fp16",
            constructor_name: meta_llama_2_7b_chat_fp16,
            display_name: "Llama 2 7B Chat FP16",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama38bInstruct {
            model_name: "@cf/meta/llama-3-8b-instruct",
            constructor_name: meta_llama_3_8b_instruct,
            display_name: "Llama 3 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama38bInstructAwq {
            model_name: "@cf/meta/llama-3-8b-instruct-awq",
            constructor_name: meta_llama_3_8b_instruct_awq,
            display_name: "Llama 3 8B Instruct AWQ",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama318bInstruct {
            model_name: "@cf/meta/llama-3.1-8b-instruct",
            constructor_name: meta_llama_3_1_8b_instruct,
            display_name: "Llama 3.1 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama318bInstructAwq {
            model_name: "@cf/meta/llama-3.1-8b-instruct-awq",
            constructor_name: meta_llama_3_1_8b_instruct_awq,
            display_name: "Llama 3.1 8B Instruct AWQ",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama318bInstructFp8 {
            model_name: "@cf/meta/llama-3.1-8b-instruct-fp8",
            constructor_name: meta_llama_3_1_8b_instruct_fp8,
            display_name: "Llama 3.1 8B Instruct FP8",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama3211bVisionInstruct {
            model_name: "@cf/meta/llama-3.2-11b-vision-instruct",
            constructor_name: meta_llama_3_2_11b_vision_instruct,
            display_name: "Llama 3.2 11B Vision Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlama321bInstruct {
            model_name: "@cf/meta/llama-3.2-1b-instruct",
            constructor_name: meta_llama_3_2_1b_instruct,
            display_name: "Llama 3.2 1B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama323bInstruct {
            model_name: "@cf/meta/llama-3.2-3b-instruct",
            constructor_name: meta_llama_3_2_3b_instruct,
            display_name: "Llama 3.2 3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama3370bInstructFp8Fast {
            model_name: "@cf/meta/llama-3.3-70b-instruct-fp8-fast",
            constructor_name: meta_llama_3_3_70b_instruct_fp8_fast,
            display_name: "Llama 3.3 70B Instruct FP8 Fast",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama4Scout17b16eInstruct {
            model_name: "@cf/meta/llama-4-scout-17b-16e-instruct",
            constructor_name: meta_llama_4_scout_17b_16e_instruct,
            display_name: "Llama 4 Scout 17B 16E Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaGuard38b {
            model_name: "@cf/meta/llama-guard-3-8b",
            constructor_name: meta_llama_guard_3_8b,
            display_name: "Llama Guard 3 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaM2m10012b {
            model_name: "@cf/meta/m2m100-1.2b",
            constructor_name: meta_m2m100_1_2b,
            display_name: "M2M100 1.2B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralMistral7bInstructV01 {
            model_name: "@cf/mistral/mistral-7b-instruct-v0.1",
            constructor_name: mistral_mistral_7b_instruct_v0_1,
            display_name: "Mistral 7B Instruct v0.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralSmall3124bInstruct {
            model_name: "@cf/mistralai/mistral-small-3.1-24b-instruct",
            constructor_name: mistralai_mistral_small_3_1_24b_instruct,
            display_name: "Mistral Small 3.1 24B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CfMoonshotaiKimiK26 {
            model_name: "@cf/moonshotai/kimi-k2.6",
            constructor_name: cf_moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CfMoonshotaiKimiK27Code {
            model_name: "@cf/moonshotai/kimi-k2.7-code",
            constructor_name: cf_moonshotai_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MyshellAiMelotts {
            model_name: "@cf/myshell-ai/melotts",
            constructor_name: myshell_ai_melotts,
            display_name: "MyShell MeloTTS",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CfNvidiaNemotron3120bA12b {
            model_name: "@cf/nvidia/nemotron-3-120b-a12b",
            constructor_name: cf_nvidia_nemotron_3_120b_a12b,
            display_name: "Nemotron 3 Super 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "@cf/openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b {
            model_name: "@cf/openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PfnetPlamoEmbedding1b {
            model_name: "@cf/pfnet/plamo-embedding-1b",
            constructor_name: pfnet_plamo_embedding_1b,
            display_name: "PLaMo Embedding 1B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        PipecatAiSmartTurnV2 {
            model_name: "@cf/pipecat-ai/smart-turn-v2",
            constructor_name: pipecat_ai_smart_turn_v2,
            display_name: "Pipecat Smart Turn v2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen25Coder32bInstruct {
            model_name: "@cf/qwen/qwen2.5-coder-32b-instruct",
            constructor_name: qwen_qwen2_5_coder_32b_instruct,
            display_name: "Qwen 2.5 Coder 32B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen330bA3bFp8 {
            model_name: "@cf/qwen/qwen3-30b-a3b-fp8",
            constructor_name: qwen_qwen3_30b_a3b_fp8,
            display_name: "Qwen3 30B A3B FP8",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Embedding06b {
            model_name: "@cf/qwen/qwen3-embedding-0.6b",
            constructor_name: qwen_qwen3_embedding_0_6b,
            display_name: "Qwen3 Embedding 0.6B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CfQwenQwen3827b {
            model_name: "@cf/qwen/qwen3.8-27b",
            constructor_name: cf_qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwq32b {
            model_name: "@cf/qwen/qwq-32b",
            constructor_name: qwen_qwq_32b,
            display_name: "QwQ 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        CfZaiOrgGlm47Flash {
            model_name: "@cf/zai-org/glm-4.7-flash",
            constructor_name: cf_zai_org_glm_4_7_flash,
            display_name: "GLM-4.7-Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CfZaiOrgGlm52 {
            model_name: "@cf/zai-org/glm-5.2",
            constructor_name: cf_zai_org_glm_5_2,
            display_name: "Glm 5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CfZaiOrgGlm53 {
            model_name: "@cf/zai-org/glm-5.3",
            constructor_name: cf_zai_org_glm_5_3,
            display_name: "Glm 5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CfZaiOrgGlm53Flash {
            model_name: "@cf/zai-org/glm-5.3-flash",
            constructor_name: cf_zai_org_glm_5_3_flash,
            display_name: "Glm 5.3 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
