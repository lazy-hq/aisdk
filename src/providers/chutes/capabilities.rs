//! Capabilities for chutes models.
//!
//! This module defines model types and their capabilities for chutes providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::chutes::Chutes;

model_capabilities! {
    provider: Chutes,
    models: {
        MinimaxaiMinimaxM21Tee {
            model_name: "MiniMaxAI/MiniMax-M2.1-TEE",
            constructor_name: minimaxai_minimax_m2_1_tee,
            display_name: "MiniMax M2.1 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxaiMinimaxM25Tee {
            model_name: "MiniMaxAI/MiniMax-M2.5-TEE",
            constructor_name: minimaxai_minimax_m2_5_tee,
            display_name: "MiniMax M2.5 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchDeephermes3Mistral24bPreview {
            model_name: "NousResearch/DeepHermes-3-Mistral-24B-Preview",
            constructor_name: nousresearch_deephermes_3_mistral_24b_preview,
            display_name: "DeepHermes 3 Mistral 24B Preview",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes414b {
            model_name: "NousResearch/Hermes-4-14B",
            constructor_name: nousresearch_hermes_4_14b,
            display_name: "Hermes 4 14B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes4405bFp8Tee {
            model_name: "NousResearch/Hermes-4-405B-FP8-TEE",
            constructor_name: nousresearch_hermes_4_405b_fp8_tee,
            display_name: "Hermes 4 405B FP8 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes470b {
            model_name: "NousResearch/Hermes-4-70B",
            constructor_name: nousresearch_hermes_4_70b,
            display_name: "Hermes 4 70B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes4336b {
            model_name: "NousResearch/Hermes-4.3-36B",
            constructor_name: nousresearch_hermes_4_3_36b,
            display_name: "Hermes 4.3 36B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpengvlabInternvl378bTee {
            model_name: "OpenGVLab/InternVL3-78B-TEE",
            constructor_name: opengvlab_internvl3_78b_tee,
            display_name: "InternVL3 78B TEE",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen2572bInstruct {
            model_name: "Qwen/Qwen2.5-72B-Instruct",
            constructor_name: qwen_qwen2_5_72b_instruct,
            display_name: "Qwen2.5 72B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen25Coder32bInstruct {
            model_name: "Qwen/Qwen2.5-Coder-32B-Instruct",
            constructor_name: qwen_qwen2_5_coder_32b_instruct,
            display_name: "Qwen2.5 Coder 32B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen25Vl32bInstruct {
            model_name: "Qwen/Qwen2.5-VL-32B-Instruct",
            constructor_name: qwen_qwen2_5_vl_32b_instruct,
            display_name: "Qwen2.5 VL 32B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen25Vl72bInstructTee {
            model_name: "Qwen/Qwen2.5-VL-72B-Instruct-TEE",
            constructor_name: qwen_qwen2_5_vl_72b_instruct_tee,
            display_name: "Qwen2.5 VL 72B Instruct TEE",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen314b {
            model_name: "Qwen/Qwen3-14B",
            constructor_name: qwen_qwen3_14b,
            display_name: "Qwen3 14B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22b {
            model_name: "Qwen/Qwen3-235B-A22B",
            constructor_name: qwen_qwen3_235b_a22b,
            display_name: "Qwen3 235B A22B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bInstruct2507Tee {
            model_name: "Qwen/Qwen3-235B-A22B-Instruct-2507-TEE",
            constructor_name: qwen_qwen3_235b_a22b_instruct_2507_tee,
            display_name: "Qwen3 235B A22B Instruct 2507 TEE",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bThinking2507 {
            model_name: "Qwen/Qwen3-235B-A22B-Thinking-2507",
            constructor_name: qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3 235B A22B Thinking 2507",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3b {
            model_name: "Qwen/Qwen3-30B-A3B",
            constructor_name: qwen_qwen3_30b_a3b,
            display_name: "Qwen3 30B A3B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3bInstruct2507 {
            model_name: "Qwen/Qwen3-30B-A3B-Instruct-2507",
            constructor_name: qwen_qwen3_30b_a3b_instruct_2507,
            display_name: "Qwen3 30B A3B Instruct 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen332b {
            model_name: "Qwen/Qwen3-32B",
            constructor_name: qwen_qwen3_32b,
            display_name: "Qwen3 32B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder480bA35bInstructFp8Tee {
            model_name: "Qwen/Qwen3-Coder-480B-A35B-Instruct-FP8-TEE",
            constructor_name: qwen_qwen3_coder_480b_a35b_instruct_fp8_tee,
            display_name: "Qwen3 Coder 480B A35B Instruct FP8 TEE",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderNext {
            model_name: "Qwen/Qwen3-Coder-Next",
            constructor_name: qwen_qwen3_coder_next,
            display_name: "Qwen3 Coder Next",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bInstruct {
            model_name: "Qwen/Qwen3-Next-80B-A3B-Instruct",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3 Next 80B A3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl235bA22bInstruct {
            model_name: "Qwen/Qwen3-VL-235B-A22B-Instruct",
            constructor_name: qwen_qwen3_vl_235b_a22b_instruct,
            display_name: "Qwen3 VL 235B A22B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3guardGen06b {
            model_name: "Qwen/Qwen3Guard-Gen-0.6B",
            constructor_name: qwen_qwen3guard_gen_0_6b,
            display_name: "Qwen3Guard Gen 0.6B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        XiaomimimoMimoV2Flash {
            model_name: "XiaomiMiMo/MiMo-V2-Flash",
            constructor_name: xiaomimimo_mimo_v2_flash,
            display_name: "MiMo V2 Flash",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ChutesaiMistralSmall3124bInstruct2503 {
            model_name: "chutesai/Mistral-Small-3.1-24B-Instruct-2503",
            constructor_name: chutesai_mistral_small_3_1_24b_instruct_2503,
            display_name: "Mistral Small 3.1 24B Instruct 2503",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ChutesaiMistralSmall3224bInstruct2506 {
            model_name: "chutesai/Mistral-Small-3.2-24B-Instruct-2506",
            constructor_name: chutesai_mistral_small_3_2_24b_instruct_2506,
            display_name: "Mistral Small 3.2 24B Instruct 2506",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR10528Tee {
            model_name: "deepseek-ai/DeepSeek-R1-0528-TEE",
            constructor_name: deepseek_ai_deepseek_r1_0528_tee,
            display_name: "DeepSeek R1 0528 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR1DistillLlama70b {
            model_name: "deepseek-ai/DeepSeek-R1-Distill-Llama-70B",
            constructor_name: deepseek_ai_deepseek_r1_distill_llama_70b,
            display_name: "DeepSeek R1 Distill Llama 70B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR1Tee {
            model_name: "deepseek-ai/DeepSeek-R1-TEE",
            constructor_name: deepseek_ai_deepseek_r1_tee,
            display_name: "DeepSeek R1 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekV3 {
            model_name: "deepseek-ai/DeepSeek-V3",
            constructor_name: deepseek_ai_deepseek_v3,
            display_name: "DeepSeek V3",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekV30324Tee {
            model_name: "deepseek-ai/DeepSeek-V3-0324-TEE",
            constructor_name: deepseek_ai_deepseek_v3_0324_tee,
            display_name: "DeepSeek V3 0324 TEE",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31Tee {
            model_name: "deepseek-ai/DeepSeek-V3.1-TEE",
            constructor_name: deepseek_ai_deepseek_v3_1_tee,
            display_name: "DeepSeek V3.1 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31TerminusTee {
            model_name: "deepseek-ai/DeepSeek-V3.1-Terminus-TEE",
            constructor_name: deepseek_ai_deepseek_v3_1_terminus_tee,
            display_name: "DeepSeek V3.1 Terminus TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV32SpecialeTee {
            model_name: "deepseek-ai/DeepSeek-V3.2-Speciale-TEE",
            constructor_name: deepseek_ai_deepseek_v3_2_speciale_tee,
            display_name: "DeepSeek V3.2 Speciale TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekV32Tee {
            model_name: "deepseek-ai/DeepSeek-V3.2-TEE",
            constructor_name: deepseek_ai_deepseek_v3_2_tee,
            display_name: "DeepSeek V3.2 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MiromindAiMirothinkerV15235b {
            model_name: "miromind-ai/MiroThinker-v1.5-235B",
            constructor_name: miromind_ai_mirothinker_v1_5_235b,
            display_name: "MiroThinker V1.5 235B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiDevstral2123bInstruct2512Tee {
            model_name: "mistralai/Devstral-2-123B-Instruct-2512-TEE",
            constructor_name: mistralai_devstral_2_123b_instruct_2512_tee,
            display_name: "Devstral 2 123B Instruct 2512 TEE",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct0905 {
            model_name: "moonshotai/Kimi-K2-Instruct-0905",
            constructor_name: moonshotai_kimi_k2_instruct_0905,
            display_name: "Kimi K2 Instruct 0905",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2ThinkingTee {
            model_name: "moonshotai/Kimi-K2-Thinking-TEE",
            constructor_name: moonshotai_kimi_k2_thinking_tee,
            display_name: "Kimi K2 Thinking TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25Tee {
            model_name: "moonshotai/Kimi-K2.5-TEE",
            constructor_name: moonshotai_kimi_k2_5_tee,
            display_name: "Kimi K2.5 TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaNvidiaNemotron3Nano30bA3bBf16 {
            model_name: "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16",
            constructor_name: nvidia_nvidia_nemotron_3_nano_30b_a3b_bf16,
            display_name: "NVIDIA Nemotron 3 Nano 30B A3B BF16",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120bTee {
            model_name: "openai/gpt-oss-120b-TEE",
            constructor_name: openai_gpt_oss_120b_tee,
            display_name: "gpt oss 120b TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "gpt oss 20b",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RednoteHilabDotsOcr {
            model_name: "rednote-hilab/dots.ocr",
            constructor_name: rednote_hilab_dots_ocr,
            display_name: "dots.ocr",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        TngtechDeepseekR1tChimera {
            model_name: "tngtech/DeepSeek-R1T-Chimera",
            constructor_name: tngtech_deepseek_r1t_chimera,
            display_name: "DeepSeek R1T Chimera",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        TngtechDeepseekTngR1t2Chimera {
            model_name: "tngtech/DeepSeek-TNG-R1T2-Chimera",
            constructor_name: tngtech_deepseek_tng_r1t2_chimera,
            display_name: "DeepSeek TNG R1T2 Chimera",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TngtechTngR1tChimeraTee {
            model_name: "tngtech/TNG-R1T-Chimera-TEE",
            constructor_name: tngtech_tng_r1t_chimera_tee,
            display_name: "TNG R1T Chimera TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TngtechTngR1tChimeraTurbo {
            model_name: "tngtech/TNG-R1T-Chimera-Turbo",
            constructor_name: tngtech_tng_r1t_chimera_turbo,
            display_name: "TNG R1T Chimera Turbo",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UnslothLlama321bInstruct {
            model_name: "unsloth/Llama-3.2-1B-Instruct",
            constructor_name: unsloth_llama_3_2_1b_instruct,
            display_name: "Llama 3.2 1B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        UnslothLlama323bInstruct {
            model_name: "unsloth/Llama-3.2-3B-Instruct",
            constructor_name: unsloth_llama_3_2_3b_instruct,
            display_name: "Llama 3.2 3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        UnslothMistralNemoInstruct2407 {
            model_name: "unsloth/Mistral-Nemo-Instruct-2407",
            constructor_name: unsloth_mistral_nemo_instruct_2407,
            display_name: "Mistral Nemo Instruct 2407",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        UnslothMistralSmall24bInstruct2501 {
            model_name: "unsloth/Mistral-Small-24B-Instruct-2501",
            constructor_name: unsloth_mistral_small_24b_instruct_2501,
            display_name: "Mistral Small 24B Instruct 2501",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UnslothGemma312bIt {
            model_name: "unsloth/gemma-3-12b-it",
            constructor_name: unsloth_gemma_3_12b_it,
            display_name: "gemma 3 12b it",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        UnslothGemma327bIt {
            model_name: "unsloth/gemma-3-27b-it",
            constructor_name: unsloth_gemma_3_27b_it,
            display_name: "gemma 3 27b it",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UnslothGemma34bIt {
            model_name: "unsloth/gemma-3-4b-it",
            constructor_name: unsloth_gemma_3_4b_it,
            display_name: "gemma 3 4b it",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm45Air {
            model_name: "zai-org/GLM-4.5-Air",
            constructor_name: zai_org_glm_4_5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45Fp8 {
            model_name: "zai-org/GLM-4.5-FP8",
            constructor_name: zai_org_glm_4_5_fp8,
            display_name: "GLM 4.5 FP8",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45Tee {
            model_name: "zai-org/GLM-4.5-TEE",
            constructor_name: zai_org_glm_4_5_tee,
            display_name: "GLM 4.5 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46Fp8 {
            model_name: "zai-org/GLM-4.6-FP8",
            constructor_name: zai_org_glm_4_6_fp8,
            display_name: "GLM 4.6 FP8",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46Tee {
            model_name: "zai-org/GLM-4.6-TEE",
            constructor_name: zai_org_glm_4_6_tee,
            display_name: "GLM 4.6 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46v {
            model_name: "zai-org/GLM-4.6V",
            constructor_name: zai_org_glm_4_6v,
            display_name: "GLM 4.6V",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Fp8 {
            model_name: "zai-org/GLM-4.7-FP8",
            constructor_name: zai_org_glm_4_7_fp8,
            display_name: "GLM 4.7 FP8",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Flash {
            model_name: "zai-org/GLM-4.7-Flash",
            constructor_name: zai_org_glm_4_7_flash,
            display_name: "GLM 4.7 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Tee {
            model_name: "zai-org/GLM-4.7-TEE",
            constructor_name: zai_org_glm_4_7_tee,
            display_name: "GLM 4.7 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5Tee {
            model_name: "zai-org/GLM-5-TEE",
            constructor_name: zai_org_glm_5_tee,
            display_name: "GLM 5 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
