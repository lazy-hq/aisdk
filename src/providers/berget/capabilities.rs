//! Capabilities for berget models.
//!
//! This module defines model types and their capabilities for berget providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::berget::Berget;

model_capabilities! {
    provider: Berget,
    models: {
        BaaiBgeRerankerV2M3 {
            model_name: "BAAI/bge-reranker-v2-m3",
            constructor_name: baai_bge_reranker_v2_m3,
            display_name: "bge-reranker-v2-m3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        KblabKbWhisperLarge {
            model_name: "KBLab/kb-whisper-large",
            constructor_name: kblab_kb_whisper_large,
            display_name: "KB-Whisper-Large",
            capabilities: [AudioInputSupport, TextOutputSupport]
        },
        IntfloatMultilingualE5Large {
            model_name: "intfloat/multilingual-e5-large",
            constructor_name: intfloat_multilingual_e5_large,
            display_name: "Multilingual-E5-large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        IntfloatMultilingualE5LargeInstruct {
            model_name: "intfloat/multilingual-e5-large-instruct",
            constructor_name: intfloat_multilingual_e5_large_instruct,
            display_name: "Multilingual-E5-large-instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama3370bInstruct {
            model_name: "meta-llama/Llama-3.3-70B-Instruct",
            constructor_name: meta_llama_llama_3_3_70b_instruct,
            display_name: "Llama 3.3 70B Instruct",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall3224bInstruct2506 {
            model_name: "mistralai/Mistral-Small-3.2-24B-Instruct-2506",
            constructor_name: mistralai_mistral_small_3_2_24b_instruct_2506,
            display_name: "Mistral Small 3.2 24B Instruct 2506",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT-OSS-120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47 {
            model_name: "zai-org/GLM-4.7",
            constructor_name: zai_org_glm_4_7,
            display_name: "GLM 4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
