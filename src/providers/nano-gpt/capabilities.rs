//! Capabilities for nano_gpt models.
//!
//! This module defines model types and their capabilities for nano_gpt providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::nano_gpt::NanoGpt;

model_capabilities! {
    provider: NanoGpt,
    models: {
        DoctorShotgunMs3224bMagnumDiamond {
            model_name: "Doctor-Shotgun/MS3.2-24B-Magnum-Diamond",
            constructor_name: doctor_shotgun_ms3_2_24b_magnum_diamond,
            display_name: "MS3.2 24B Magnum Diamond",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EvaUnit01EvaLlama33370bV00 {
            model_name: "EVA-UNIT-01/EVA-LLaMA-3.33-70B-v0.0",
            constructor_name: eva_unit_01_eva_llama_3_33_70b_v0_0,
            display_name: "EVA Llama 3.33 70B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EvaUnit01EvaLlama33370bV01 {
            model_name: "EVA-UNIT-01/EVA-LLaMA-3.33-70B-v0.1",
            constructor_name: eva_unit_01_eva_llama_3_33_70b_v0_1,
            display_name: "EVA-LLaMA-3.33-70B-v0.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EvaUnit01EvaQwen2532bV02 {
            model_name: "EVA-UNIT-01/EVA-Qwen2.5-32B-v0.2",
            constructor_name: eva_unit_01_eva_qwen2_5_32b_v0_2,
            display_name: "EVA-Qwen2.5-32B-v0.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EvaUnit01EvaQwen2572bV02 {
            model_name: "EVA-UNIT-01/EVA-Qwen2.5-72B-v0.2",
            constructor_name: eva_unit_01_eva_qwen2_5_72b_v0_2,
            display_name: "EVA-Qwen2.5-72B-v0.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EnvoidLlama305NtStorybreakerMinistral70b {
            model_name: "Envoid/Llama-3.05-NT-Storybreaker-Ministral-70B",
            constructor_name: envoid_llama_3_05_nt_storybreaker_ministral_70b,
            display_name: "Llama 3.05 Storybreaker Ministral 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EnvoidLlama305NemotronTenyxchatStorybreaker70b {
            model_name: "Envoid/Llama-3.05-Nemotron-Tenyxchat-Storybreaker-70B",
            constructor_name: envoid_llama_3_05_nemotron_tenyxchat_storybreaker_70b,
            display_name: "Nemotron Tenyxchat Storybreaker 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm46DerestrictedV5 {
            model_name: "GLM-4.6-Derestricted-v5",
            constructor_name: glm_4_6_derestricted_v5,
            display_name: "GLM 4.6 Derestricted v5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GalrionsoftworksMnLoosecannon12bV1 {
            model_name: "GalrionSoftworks/MN-LooseCannon-12B-v1",
            constructor_name: galrionsoftworks_mn_loosecannon_12b_v1,
            display_name: "MN-LooseCannon-12B-v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemma426bA4bMeromero {
            model_name: "Gemma-4-26B-A4B-MeroMero",
            constructor_name: gemma_4_26b_a4b_meromero,
            display_name: "Gemma 4 26B A4B MeroMero",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma426bA4bMeromeroThinking {
            model_name: "Gemma-4-26B-A4B-MeroMero:thinking",
            constructor_name: gemma_4_26b_a4b_meromero_thinking,
            display_name: "Gemma 4 26B A4B MeroMero Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bAgaresV1 {
            model_name: "Gemma-4-31B-Agares-v1",
            constructor_name: gemma_4_31b_agares_v1,
            display_name: "Gemma 4 31B Agares v1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bAnimusV141 {
            model_name: "Gemma-4-31B-Animus-V14.1",
            constructor_name: gemma_4_31b_animus_v14_1,
            display_name: "Gemma 4 31B Animus V14.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bAssguard {
            model_name: "Gemma-4-31B-AssGuard",
            constructor_name: gemma_4_31b_assguard,
            display_name: "Gemma 4 31B AssGuard",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bClaude46OpusReasoningDistilled {
            model_name: "Gemma-4-31B-Claude-4.6-Opus-Reasoning-Distilled",
            constructor_name: gemma_4_31b_claude_4_6_opus_reasoning_distilled,
            display_name: "Gemma 4 31B Claude 4.6 Opus Reasoning Distilled",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bCognitiveUnshackled {
            model_name: "Gemma-4-31B-Cognitive-Unshackled",
            constructor_name: gemma_4_31b_cognitive_unshackled,
            display_name: "Gemma 4 31B Cognitive Unshackled",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bDarkGemistry {
            model_name: "Gemma-4-31B-Dark-Gemistry",
            constructor_name: gemma_4_31b_dark_gemistry,
            display_name: "Gemma 4 31B Dark Gemistry",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bDarkidol {
            model_name: "Gemma-4-31B-DarkIdol",
            constructor_name: gemma_4_31b_darkidol,
            display_name: "Gemma 4 31B DarkIdol",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bFabled {
            model_name: "Gemma-4-31B-Fabled",
            constructor_name: gemma_4_31b_fabled,
            display_name: "Gemma 4 31B Fabled (ArliAI)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bGarnetv2 {
            model_name: "Gemma-4-31B-GarnetV2",
            constructor_name: gemma_4_31b_garnetv2,
            display_name: "Gemma 4 31B Garnet V2",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bGembrainXCore {
            model_name: "Gemma-4-31B-Gembrain-X-Core",
            constructor_name: gemma_4_31b_gembrain_x_core,
            display_name: "Gemma 4 31B Gembrain X Core",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bGembrainUncensoredHeretic {
            model_name: "Gemma-4-31B-Gembrain-uncensored-heretic",
            constructor_name: gemma_4_31b_gembrain_uncensored_heretic,
            display_name: "Gemma 4 31B Gembrain Uncensored Heretic",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bIsometryRp {
            model_name: "Gemma-4-31B-Isometry-RP",
            constructor_name: gemma_4_31b_isometry_rp,
            display_name: "Gemma 4 31B Isometry RP",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bK1V5 {
            model_name: "Gemma-4-31B-K1-v5",
            constructor_name: gemma_4_31b_k1_v5,
            display_name: "Gemma 4 31B K1 v5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bMeromero {
            model_name: "Gemma-4-31B-MeroMero",
            constructor_name: gemma_4_31b_meromero,
            display_name: "Gemma 4 31B MeroMero",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bMeromeroV2 {
            model_name: "Gemma-4-31B-MeroMero-v2",
            constructor_name: gemma_4_31b_meromero_v2,
            display_name: "Gemma 4 31B MeroMero v2",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bMeromeroV2Thinking {
            model_name: "Gemma-4-31B-MeroMero-v2:thinking",
            constructor_name: gemma_4_31b_meromero_v2_thinking,
            display_name: "Gemma 4 31B MeroMero v2 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bNovelist {
            model_name: "Gemma-4-31B-Novelist",
            constructor_name: gemma_4_31b_novelist,
            display_name: "Gemma 4 31B Novelist (ArliAI)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bQueen {
            model_name: "Gemma-4-31B-Queen",
            constructor_name: gemma_4_31b_queen,
            display_name: "Gemma 4 31B Queen",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bSdftHereticRp {
            model_name: "Gemma-4-31B-SDFT-Heretic-RP",
            constructor_name: gemma_4_31b_sdft_heretic_rp,
            display_name: "Gemma 4 31B SDFT Heretic RP",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bStyletune {
            model_name: "Gemma-4-31B-StyleTune",
            constructor_name: gemma_4_31b_styletune,
            display_name: "Gemma 4 31B StyleTune",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GrypheMythomaxL213b {
            model_name: "Gryphe/MythoMax-L2-13b",
            constructor_name: gryphe_mythomax_l2_13b,
            display_name: "MythoMax 13B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llm360K2Think {
            model_name: "LLM360/K2-Think",
            constructor_name: llm360_k2_think,
            display_name: "K2-Think",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        LatitudegamesWayfarerLarge70bLlama33 {
            model_name: "LatitudeGames/Wayfarer-Large-70B-Llama-3.3",
            constructor_name: latitudegames_wayfarer_large_70b_llama_3_3,
            display_name: "Llama 3.3 70B Wayfarer",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MarinaraspaghettiNemomixUnleashed12b {
            model_name: "MarinaraSpaghetti/NemoMix-Unleashed-12B",
            constructor_name: marinaraspaghetti_nemomix_unleashed_12b,
            display_name: "NemoMix 12B Unleashed",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama318bInstructFp8 {
            model_name: "Meta-Llama-3-1-8B-Instruct-FP8",
            constructor_name: meta_llama_3_1_8b_instruct_fp8,
            display_name: "Llama 3.1 8B (decentralized)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MinimaxM1 {
            model_name: "MiniMax-M1",
            constructor_name: minimax_m1,
            display_name: "MiniMax M1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MinimaxaiMinimaxM180k {
            model_name: "MiniMaxAI/MiniMax-M1-80k",
            constructor_name: minimaxai_minimax_m1_80k,
            display_name: "MiniMax M1 80K",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NeversleepLumimaidV0270b {
            model_name: "NeverSleep/Lumimaid-v0.2-70B",
            constructor_name: neversleep_lumimaid_v0_2_70b,
            display_name: "Lumimaid v0.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen3527bBlossomV64Derestricted {
            model_name: "Qwen3.5-27B-Blossom-V6.4-Derestricted",
            constructor_name: qwen3_5_27b_blossom_v6_4_derestricted,
            display_name: "Qwen3.5 27B Blossom V6.4 Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3527bBluestarV3Derestricted {
            model_name: "Qwen3.5-27B-BlueStar-v3-Derestricted",
            constructor_name: qwen3_5_27b_bluestar_v3_derestricted,
            display_name: "Qwen3.5 27B BlueStar v3 Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3527bClaude46OpusReasoningDistilledDerestricted {
            model_name: "Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled-Derestricted",
            constructor_name: qwen3_5_27b_claude_4_6_opus_reasoning_distilled_derestricted,
            display_name: "Qwen3.5 27B Claude 4.6 Opus Reasoning Distilled Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3527bClaude46OpusReasoningDistilledDerestrictedLite {
            model_name: "Qwen3.5-27B-Claude-4.6-Opus-Reasoning-Distilled-Derestricted-Lite",
            constructor_name: qwen3_5_27b_claude_4_6_opus_reasoning_distilled_derestricted_lite,
            display_name: "Qwen3.5 27B Claude 4.6 Opus Reasoning Distilled Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3527bQueenDerestricted {
            model_name: "Qwen3.5-27B-Queen-Derestricted",
            constructor_name: qwen3_5_27b_queen_derestricted,
            display_name: "Qwen3.5 27B Queen Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ReadyartMs32TheOmegaDirective24bUnslopV20 {
            model_name: "ReadyArt/MS3.2-The-Omega-Directive-24B-Unslop-v2.0",
            constructor_name: readyart_ms3_2_the_omega_directive_24b_unslop_v2_0,
            display_name: "Omega Directive 24B Unslop v2.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SalesforceLlamaXlam270bFcR {
            model_name: "Salesforce/Llama-xLAM-2-70b-fc-r",
            constructor_name: salesforce_llama_xlam_2_70b_fc_r,
            display_name: "Llama-xLAM-2 70B fc-r",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Sao10kL38bSthenoV32 {
            model_name: "Sao10K/L3-8B-Stheno-v3.2",
            constructor_name: sao10k_l3_8b_stheno_v3_2,
            display_name: "Sao10K Stheno 8b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Sao10kL3170bEuryaleV22 {
            model_name: "Sao10K/L3.1-70B-Euryale-v2.2",
            constructor_name: sao10k_l3_1_70b_euryale_v2_2,
            display_name: "Llama 3.1 70B Euryale",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Sao10kL3170bHanamiX1 {
            model_name: "Sao10K/L3.1-70B-Hanami-x1",
            constructor_name: sao10k_l3_1_70b_hanami_x1,
            display_name: "Llama 3.1 70B Hanami",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Sao10kL3370bEuryaleV23 {
            model_name: "Sao10K/L3.3-70B-Euryale-v2.3",
            constructor_name: sao10k_l3_3_70b_euryale_v2_3,
            display_name: "Llama 3.3 70B Euryale",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SteelskullL33CuMaiR170b {
            model_name: "Steelskull/L3.3-Cu-Mai-R1-70b",
            constructor_name: steelskull_l3_3_cu_mai_r1_70b,
            display_name: "Llama 3.3 70B Cu Mai",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SteelskullL33ElectraR170b {
            model_name: "Steelskull/L3.3-Electra-R1-70b",
            constructor_name: steelskull_l3_3_electra_r1_70b,
            display_name: "Steelskull Electra R1 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SteelskullL33MsNevoria70b {
            model_name: "Steelskull/L3.3-MS-Nevoria-70b",
            constructor_name: steelskull_l3_3_ms_nevoria_70b,
            display_name: "Steelskull Nevoria 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SteelskullL33NevoriaR170b {
            model_name: "Steelskull/L3.3-Nevoria-R1-70b",
            constructor_name: steelskull_l3_3_nevoria_r1_70b,
            display_name: "Steelskull Nevoria R1 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeDeepseekV32 {
            model_name: "TEE/deepseek-v3.2",
            constructor_name: tee_deepseek_v3_2,
            display_name: "DeepSeek V3.2 TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeDeepseekV41Flash {
            model_name: "TEE/deepseek-v4.1-flash",
            constructor_name: tee_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGemma426bA4bUncensored {
            model_name: "TEE/gemma-4-26b-a4b-uncensored",
            constructor_name: tee_gemma_4_26b_a4b_uncensored,
            display_name: "Gemma 4 26B A4B Uncensored TEE",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGemma431bIt {
            model_name: "TEE/gemma-4-31b-it",
            constructor_name: tee_gemma_4_31b_it,
            display_name: "Gemma 4 31B IT TEE",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGemma431b {
            model_name: "TEE/gemma4-31b",
            constructor_name: tee_gemma4_31b,
            display_name: "Gemma 4 31B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGemma431bThinking {
            model_name: "TEE/gemma4-31b:thinking",
            constructor_name: tee_gemma4_31b_thinking,
            display_name: "Gemma 4 31B Thinking TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGlm51 {
            model_name: "TEE/glm-5.1",
            constructor_name: tee_glm_5_1,
            display_name: "GLM 5.1 TEE",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGlm51Thinking {
            model_name: "TEE/glm-5.1-thinking",
            constructor_name: tee_glm_5_1_thinking,
            display_name: "GLM 5.1 Thinking TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGlm52 {
            model_name: "TEE/glm-5.2",
            constructor_name: tee_glm_5_2,
            display_name: "GLM 5.2 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGlm52Thinking {
            model_name: "TEE/glm-5.2:thinking",
            constructor_name: tee_glm_5_2_thinking,
            display_name: "GLM 5.2 Thinking TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGlm53 {
            model_name: "TEE/glm-5.3",
            constructor_name: tee_glm_5_3,
            display_name: "GLM 5.3 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGlm53Flash {
            model_name: "TEE/glm-5.3-flash",
            constructor_name: tee_glm_5_3_flash,
            display_name: "GLM 5.3 Flash TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGptOss120b {
            model_name: "TEE/gpt-oss-120b",
            constructor_name: tee_gpt_oss_120b,
            display_name: "GPT-OSS 120B TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeKimiK26 {
            model_name: "TEE/kimi-k2.6",
            constructor_name: tee_kimi_k2_6,
            display_name: "Kimi K2.6 TEE",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeKimiK3 {
            model_name: "TEE/kimi-k3",
            constructor_name: tee_kimi_k3,
            display_name: "Kimi K3 TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeKimiK31Pro {
            model_name: "TEE/kimi-k3.1-pro",
            constructor_name: tee_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeLlama3370b {
            model_name: "TEE/llama3-3-70b",
            constructor_name: tee_llama3_3_70b,
            display_name: "Llama 3.3 70B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeQwen25Vl72bInstruct {
            model_name: "TEE/qwen2.5-vl-72b-instruct",
            constructor_name: tee_qwen2_5_vl_72b_instruct,
            display_name: "Qwen2.5 VL 72B TEE",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        TeeQwen35397bA17b {
            model_name: "TEE/qwen3.5-397b-a17b",
            constructor_name: tee_qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B A17B TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeQwen3627b {
            model_name: "TEE/qwen3.6-27b",
            constructor_name: tee_qwen3_6_27b,
            display_name: "Qwen3.6 27B TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        TeeQwen3635bA3b {
            model_name: "TEE/qwen3.6-35b-a3b",
            constructor_name: tee_qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B A3B TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeQwen3827b {
            model_name: "TEE/qwen3.8-27b",
            constructor_name: tee_qwen3_8_27b,
            display_name: "Qwen3.8 27B TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ThedrummerAnubis70bV1 {
            model_name: "TheDrummer/Anubis-70B-v1",
            constructor_name: thedrummer_anubis_70b_v1,
            display_name: "Anubis 70B v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerAnubis70bV11 {
            model_name: "TheDrummer/Anubis-70B-v1.1",
            constructor_name: thedrummer_anubis_70b_v1_1,
            display_name: "Anubis 70B v1.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerArtemisV11 {
            model_name: "TheDrummer/Artemis-v1.1",
            constructor_name: thedrummer_artemis_v1_1,
            display_name: "TheDrummer/Artemis v1.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThedrummerCydonia24bV2 {
            model_name: "TheDrummer/Cydonia-24B-v2",
            constructor_name: thedrummer_cydonia_24b_v2,
            display_name: "The Drummer Cydonia 24B v2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerCydonia24bV4 {
            model_name: "TheDrummer/Cydonia-24B-v4",
            constructor_name: thedrummer_cydonia_24b_v4,
            display_name: "The Drummer Cydonia 24B v4",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerCydonia24bV41 {
            model_name: "TheDrummer/Cydonia-24B-v4.1",
            constructor_name: thedrummer_cydonia_24b_v4_1,
            display_name: "The Drummer Cydonia 24B v4.1",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThedrummerCydonia24bV43 {
            model_name: "TheDrummer/Cydonia-24B-v4.3",
            constructor_name: thedrummer_cydonia_24b_v4_3,
            display_name: "The Drummer Cydonia 24B v4.3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerMagidonia24bV43 {
            model_name: "TheDrummer/Magidonia-24B-v4.3",
            constructor_name: thedrummer_magidonia_24b_v4_3,
            display_name: "The Drummer Magidonia 24B v4.3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerRocinante12bV11 {
            model_name: "TheDrummer/Rocinante-12B-v1.1",
            constructor_name: thedrummer_rocinante_12b_v1_1,
            display_name: "Rocinante 12b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerUnslopnemo12bV41 {
            model_name: "TheDrummer/UnslopNemo-12B-v4.1",
            constructor_name: thedrummer_unslopnemo_12b_v4_1,
            display_name: "UnslopNemo 12b v4",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ThedrummerArtemis31bV12 {
            model_name: "TheDrummer/artemis-31b-v1.2",
            constructor_name: thedrummer_artemis_31b_v1_2,
            display_name: "Artemis 31B v1.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerSkyfall36bV2 {
            model_name: "TheDrummer/skyfall-36b-v2",
            constructor_name: thedrummer_skyfall_36b_v2,
            display_name: "TheDrummer Skyfall 36B V2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        VongolachoukoStarcannonUnleashed12bV10 {
            model_name: "VongolaChouko/Starcannon-Unleashed-12B-v1.0",
            constructor_name: vongolachouko_starcannon_unleashed_12b_v1_0,
            display_name: "Mistral Nemo Starcannon 12b v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AbacusaiDracarys72bInstruct {
            model_name: "abacusai/Dracarys-72B-Instruct",
            constructor_name: abacusai_dracarys_72b_instruct,
            display_name: "Llama 3.1 70B Dracarys 2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AbliterationAiAbliteratedModel {
            model_name: "abliteration-ai/abliterated-model",
            constructor_name: abliteration_ai_abliterated_model,
            display_name: "Abliterated Model",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        AbliterationAiAbliteratedModelLarge {
            model_name: "abliteration-ai/abliterated-model-large",
            constructor_name: abliteration_ai_abliterated_model_large,
            display_name: "Abliterated Model Large",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AbliterationAiAbliteratedModelLargeFlash {
            model_name: "abliteration-ai/abliterated-model-large-flash",
            constructor_name: abliteration_ai_abliterated_model_large_flash,
            display_name: "Abliterated Large Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AbliterationAiAbliteratedModelLargeV2 {
            model_name: "abliteration-ai/abliterated-model-large-v2",
            constructor_name: abliteration_ai_abliterated_model_large_v2,
            display_name: "Abliterated Model Large V2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Agnes30Flash {
            model_name: "agnes-3.0-flash",
            constructor_name: agnes_3_0_flash,
            display_name: "Agnes 3.0 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AionLabsAion20 {
            model_name: "aion-labs/aion-2.0",
            constructor_name: aion_labs_aion_2_0,
            display_name: "AionLabs: Aion-2.0",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AionLabsAion30 {
            model_name: "aion-labs/aion-3.0",
            constructor_name: aion_labs_aion_3_0,
            display_name: "AionLabs: Aion 3.0",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AionLabsAion30Mini {
            model_name: "aion-labs/aion-3.0-mini",
            constructor_name: aion_labs_aion_3_0_mini,
            display_name: "AionLabs: Aion 3.0 Mini",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AionLabsAion35 {
            model_name: "aion-labs/aion-3.5",
            constructor_name: aion_labs_aion_3_5,
            display_name: "AionLabs: Aion 3.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AionLabsAion35Mini {
            model_name: "aion-labs/aion-3.5-mini",
            constructor_name: aion_labs_aion_3_5_mini,
            display_name: "AionLabs: Aion 3.5 Mini",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AionLabsAionRpLlama318b {
            model_name: "aion-labs/aion-rp-llama-3.1-8b",
            constructor_name: aion_labs_aion_rp_llama_3_1_8b,
            display_name: "Llama 3.1 8b (uncensored)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AisingaporeGemmaSeaLionV427bIt {
            model_name: "aisingapore/gemma-sea-lion-v4-27b-it",
            constructor_name: aisingapore_gemma_sea_lion_v4_27b_it,
            display_name: "Gemma SEA-LION v4 27B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AisingaporeQwenSeaLionV432bIt {
            model_name: "aisingapore/qwen-sea-lion-v4-32b-it",
            constructor_name: aisingapore_qwen_sea_lion_v4_32b_it,
            display_name: "Qwen SEA-LION v4 32B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlephAlphaKolibri1 {
            model_name: "aleph-alpha/kolibri-1",
            constructor_name: aleph_alpha_kolibri_1,
            display_name: "Kolibri 1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AmazonNova2LiteV1 {
            model_name: "amazon/nova-2-lite-v1",
            constructor_name: amazon_nova_2_lite_v1,
            display_name: "Amazon Nova 2 Lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AmazonNovaLiteV1 {
            model_name: "amazon/nova-lite-v1",
            constructor_name: amazon_nova_lite_v1,
            display_name: "Amazon Nova Lite 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AmazonNovaProV1 {
            model_name: "amazon/nova-pro-v1",
            constructor_name: amazon_nova_pro_v1,
            display_name: "Amazon Nova Pro 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AnthraciteOrgMagnumV272b {
            model_name: "anthracite-org/magnum-v2-72b",
            constructor_name: anthracite_org_magnum_v2_72b,
            display_name: "Magnum V2 72B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AnthraciteOrgMagnumV472b {
            model_name: "anthracite-org/magnum-v4-72b",
            constructor_name: anthracite_org_magnum_v4_72b,
            display_name: "Magnum v4 72B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        AnthropicClaudeFable5 {
            model_name: "anthropic/claude-fable-5",
            constructor_name: anthropic_claude_fable_5,
            display_name: "Claude Fable 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeFable51 {
            model_name: "anthropic/claude-fable-5.1",
            constructor_name: anthropic_claude_fable_5_1,
            display_name: "Claude Fable 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeFable52 {
            model_name: "anthropic/claude-fable-5.2",
            constructor_name: anthropic_claude_fable_5_2,
            display_name: "Claude Fable 5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeFableLatest {
            model_name: "anthropic/claude-fable-latest",
            constructor_name: anthropic_claude_fable_latest,
            display_name: "Claude Fable Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku45 {
            model_name: "anthropic/claude-haiku-4.5",
            constructor_name: anthropic_claude_haiku_4_5,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku45Thinking {
            model_name: "anthropic/claude-haiku-4.5:thinking",
            constructor_name: anthropic_claude_haiku_4_5_thinking,
            display_name: "Claude Haiku 4.5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku55 {
            model_name: "anthropic/claude-haiku-5.5",
            constructor_name: anthropic_claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaikuLatest {
            model_name: "anthropic/claude-haiku-latest",
            constructor_name: anthropic_claude_haiku_latest,
            display_name: "Claude Haiku Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus41 {
            model_name: "anthropic/claude-opus-4.1",
            constructor_name: anthropic_claude_opus_4_1,
            display_name: "Claude 4.1 Opus",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus41Thinking {
            model_name: "anthropic/claude-opus-4.1:thinking",
            constructor_name: anthropic_claude_opus_4_1_thinking,
            display_name: "Claude 4.1 Opus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus41Thinking1024 {
            model_name: "anthropic/claude-opus-4.1:thinking:1024",
            constructor_name: anthropic_claude_opus_4_1_thinking_1024,
            display_name: "Claude 4.1 Opus Thinking (1K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus41Thinking32768 {
            model_name: "anthropic/claude-opus-4.1:thinking:32768",
            constructor_name: anthropic_claude_opus_4_1_thinking_32768,
            display_name: "Claude 4.1 Opus Thinking (32K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus41Thinking8192 {
            model_name: "anthropic/claude-opus-4.1:thinking:8192",
            constructor_name: anthropic_claude_opus_4_1_thinking_8192,
            display_name: "Claude 4.1 Opus Thinking (8K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus45 {
            model_name: "anthropic/claude-opus-4.5",
            constructor_name: anthropic_claude_opus_4_5,
            display_name: "Claude 4.5 Opus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus45Thinking {
            model_name: "anthropic/claude-opus-4.5:thinking",
            constructor_name: anthropic_claude_opus_4_5_thinking,
            display_name: "Claude 4.5 Opus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46 {
            model_name: "anthropic/claude-opus-4.6",
            constructor_name: anthropic_claude_opus_4_6,
            display_name: "Claude 4.6 Opus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46Thinking {
            model_name: "anthropic/claude-opus-4.6:thinking",
            constructor_name: anthropic_claude_opus_4_6_thinking,
            display_name: "Claude 4.6 Opus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46ThinkingLow {
            model_name: "anthropic/claude-opus-4.6:thinking:low",
            constructor_name: anthropic_claude_opus_4_6_thinking_low,
            display_name: "Claude 4.6 Opus Thinking Low",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46ThinkingMax {
            model_name: "anthropic/claude-opus-4.6:thinking:max",
            constructor_name: anthropic_claude_opus_4_6_thinking_max,
            display_name: "Claude 4.6 Opus Thinking Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46ThinkingMedium {
            model_name: "anthropic/claude-opus-4.6:thinking:medium",
            constructor_name: anthropic_claude_opus_4_6_thinking_medium,
            display_name: "Claude 4.6 Opus Thinking Medium",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus47 {
            model_name: "anthropic/claude-opus-4.7",
            constructor_name: anthropic_claude_opus_4_7,
            display_name: "Claude 4.7 Opus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus47Thinking {
            model_name: "anthropic/claude-opus-4.7:thinking",
            constructor_name: anthropic_claude_opus_4_7_thinking,
            display_name: "Claude 4.7 Opus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus48 {
            model_name: "anthropic/claude-opus-4.8",
            constructor_name: anthropic_claude_opus_4_8,
            display_name: "Claude Opus 4.8",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus48Thinking {
            model_name: "anthropic/claude-opus-4.8:thinking",
            constructor_name: anthropic_claude_opus_4_8_thinking,
            display_name: "Claude Opus 4.8 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus5 {
            model_name: "anthropic/claude-opus-5",
            constructor_name: anthropic_claude_opus_5,
            display_name: "Claude Opus 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus55 {
            model_name: "anthropic/claude-opus-5.5",
            constructor_name: anthropic_claude_opus_5_5,
            display_name: "Claude Opus 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpusLatest {
            model_name: "anthropic/claude-opus-latest",
            constructor_name: anthropic_claude_opus_latest,
            display_name: "Claude Opus Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet45 {
            model_name: "anthropic/claude-sonnet-4.5",
            constructor_name: anthropic_claude_sonnet_4_5,
            display_name: "Claude Sonnet 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet45Thinking {
            model_name: "anthropic/claude-sonnet-4.5:thinking",
            constructor_name: anthropic_claude_sonnet_4_5_thinking,
            display_name: "Claude Sonnet 4.5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet46 {
            model_name: "anthropic/claude-sonnet-4.6",
            constructor_name: anthropic_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet46Thinking {
            model_name: "anthropic/claude-sonnet-4.6:thinking",
            constructor_name: anthropic_claude_sonnet_4_6_thinking,
            display_name: "Claude Sonnet 4.6 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet5 {
            model_name: "anthropic/claude-sonnet-5",
            constructor_name: anthropic_claude_sonnet_5,
            display_name: "Claude Sonnet 5",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet55 {
            model_name: "anthropic/claude-sonnet-5.5",
            constructor_name: anthropic_claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet55Thinking {
            model_name: "anthropic/claude-sonnet-5.5:thinking",
            constructor_name: anthropic_claude_sonnet_5_5_thinking,
            display_name: "Claude Sonnet 5.5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet5Thinking {
            model_name: "anthropic/claude-sonnet-5:thinking",
            constructor_name: anthropic_claude_sonnet_5_thinking,
            display_name: "Claude Sonnet 5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnetLatest {
            model_name: "anthropic/claude-sonnet-latest",
            constructor_name: anthropic_claude_sonnet_latest,
            display_name: "Claude Sonnet Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ArceeAiTrinityLargeThinking {
            model_name: "arcee-ai/trinity-large-thinking",
            constructor_name: arcee_ai_trinity_large_thinking,
            display_name: "Trinity Large Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Asi1Mini {
            model_name: "asi1-mini",
            constructor_name: asi1_mini,
            display_name: "ASI1 Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AutoModel {
            model_name: "auto-model",
            constructor_name: auto_model,
            display_name: "Auto model",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoModelBasic {
            model_name: "auto-model-basic",
            constructor_name: auto_model_basic,
            display_name: "Auto model (Basic)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoModelPremium {
            model_name: "auto-model-premium",
            constructor_name: auto_model_premium,
            display_name: "Auto model (Premium)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoModelStandard {
            model_name: "auto-model-standard",
            constructor_name: auto_model_standard,
            display_name: "Auto model (Standard)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BaiduErnieLightDt10Intl {
            model_name: "baidu/ernie-light-dt-1.0-intl",
            constructor_name: baidu_ernie_light_dt_1_0_intl,
            display_name: "ERNIE Light DT 1.0 (Intl)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BytedanceSeedSeed21Turbo {
            model_name: "bytedance-seed/seed-2-1-turbo",
            constructor_name: bytedance_seed_seed_2_1_turbo,
            display_name: "ByteDance Seed 2.1 Turbo",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        BytedanceSeedSeed20Code {
            model_name: "bytedance-seed/seed-2.0-code",
            constructor_name: bytedance_seed_seed_2_0_code,
            display_name: "ByteDance Seed 2.0 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        BytedanceSeedSeed20Lite {
            model_name: "bytedance-seed/seed-2.0-lite",
            constructor_name: bytedance_seed_seed_2_0_lite,
            display_name: "ByteDance Seed 2.0 Lite",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        BytedanceDoubaoSeed21Pro {
            model_name: "bytedance/doubao-seed-2.1-pro",
            constructor_name: bytedance_doubao_seed_2_1_pro,
            display_name: "Doubao Seed 2.1 Pro",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BytedanceDoubaoSeed21Turbo {
            model_name: "bytedance/doubao-seed-2.1-turbo",
            constructor_name: bytedance_doubao_seed_2_1_turbo,
            display_name: "Doubao Seed 2.1 Turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BytedanceDoubaoSeedCharacter {
            model_name: "bytedance/doubao-seed-character",
            constructor_name: bytedance_doubao_seed_character,
            display_name: "Doubao Seed Character",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Celeris1 {
            model_name: "celeris-1",
            constructor_name: celeris_1,
            display_name: "Celeris 1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ClawHigh {
            model_name: "claw-high",
            constructor_name: claw_high,
            display_name: "Claw High",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClawLow {
            model_name: "claw-low",
            constructor_name: claw_low,
            display_name: "Claw Low",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClawMedium {
            model_name: "claw-medium",
            constructor_name: claw_medium,
            display_name: "Claw Medium",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereCommandRPlus082024 {
            model_name: "cohere/command-r-plus-08-2024",
            constructor_name: cohere_command_r_plus_08_2024,
            display_name: "Cohere: Command R+",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CommandAReasoning082025 {
            model_name: "command-a-reasoning-08-2025",
            constructor_name: command_a_reasoning_08_2025,
            display_name: "Cohere Command A (08/2025)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepcogitoCogitoV1PreviewQwen32b {
            model_name: "deepcogito/cogito-v1-preview-qwen-32B",
            constructor_name: deepcogito_cogito_v1_preview_qwen_32b,
            display_name: "Cogito v1 Preview Qwen 32B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekR10528 {
            model_name: "deepseek-ai/DeepSeek-R1-0528",
            constructor_name: deepseek_ai_deepseek_r1_0528,
            display_name: "DeepSeek R1 0528",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31 {
            model_name: "deepseek-ai/DeepSeek-V3.1",
            constructor_name: deepseek_ai_deepseek_v3_1,
            display_name: "DeepSeek V3.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31Terminus {
            model_name: "deepseek-ai/DeepSeek-V3.1-Terminus",
            constructor_name: deepseek_ai_deepseek_v3_1_terminus,
            display_name: "DeepSeek V3.1 Terminus",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31TerminusThinking {
            model_name: "deepseek-ai/DeepSeek-V3.1-Terminus:thinking",
            constructor_name: deepseek_ai_deepseek_v3_1_terminus_thinking,
            display_name: "DeepSeek V3.1 Terminus (Thinking)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31Thinking {
            model_name: "deepseek-ai/DeepSeek-V3.1:thinking",
            constructor_name: deepseek_ai_deepseek_v3_1_thinking,
            display_name: "DeepSeek V3.1 Thinking",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV32Exp {
            model_name: "deepseek-ai/deepseek-v3.2-exp",
            constructor_name: deepseek_ai_deepseek_v3_2_exp,
            display_name: "DeepSeek V3.2 Exp",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekV32ExpThinking {
            model_name: "deepseek-ai/deepseek-v3.2-exp-thinking",
            constructor_name: deepseek_ai_deepseek_v3_2_exp_thinking,
            display_name: "DeepSeek V3.2 Exp Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekChat {
            model_name: "deepseek-chat",
            constructor_name: deepseek_chat,
            display_name: "DeepSeek V3/Deepseek Chat",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekChatCheaper {
            model_name: "deepseek-chat-cheaper",
            constructor_name: deepseek_chat_cheaper,
            display_name: "DeepSeek V3/Chat Cheaper",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV30324 {
            model_name: "deepseek-v3-0324",
            constructor_name: deepseek_v3_0324,
            display_name: "DeepSeek Chat 0324",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekLatest {
            model_name: "deepseek/deepseek-latest",
            constructor_name: deepseek_deepseek_latest,
            display_name: "DeepSeek Latest",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekR1 {
            model_name: "deepseek/deepseek-r1",
            constructor_name: deepseek_deepseek_r1,
            display_name: "Deepseek R1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32 {
            model_name: "deepseek/deepseek-v3.2",
            constructor_name: deepseek_deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32Thinking {
            model_name: "deepseek/deepseek-v3.2:thinking",
            constructor_name: deepseek_deepseek_v3_2_thinking,
            display_name: "Deepseek V3.2 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash {
            model_name: "deepseek/deepseek-v4-flash",
            constructor_name: deepseek_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash0423 {
            model_name: "deepseek/deepseek-v4-flash-0423",
            constructor_name: deepseek_deepseek_v4_flash_0423,
            display_name: "DeepSeek V4 Flash 0423",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash0423Thinking {
            model_name: "deepseek/deepseek-v4-flash-0423:thinking",
            constructor_name: deepseek_deepseek_v4_flash_0423_thinking,
            display_name: "DeepSeek V4 Flash 0423 (Thinking)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash0731 {
            model_name: "deepseek/deepseek-v4-flash-0731",
            constructor_name: deepseek_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash0731Thinking {
            model_name: "deepseek/deepseek-v4-flash-0731:thinking",
            constructor_name: deepseek_deepseek_v4_flash_0731_thinking,
            display_name: "DeepSeek V4 Flash 0731 (Thinking)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4FlashLatest {
            model_name: "deepseek/deepseek-v4-flash-latest",
            constructor_name: deepseek_deepseek_v4_flash_latest,
            display_name: "DeepSeek V4 Flash Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4FlashVisionExp {
            model_name: "deepseek/deepseek-v4-flash-vision-exp",
            constructor_name: deepseek_deepseek_v4_flash_vision_exp,
            display_name: "DeepSeek V4 Flash Vision Exp",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4FlashThinking {
            model_name: "deepseek/deepseek-v4-flash:thinking",
            constructor_name: deepseek_deepseek_v4_flash_thinking,
            display_name: "DeepSeek V4 Flash (Thinking)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Pro {
            model_name: "deepseek/deepseek-v4-pro",
            constructor_name: deepseek_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Pro0813 {
            model_name: "deepseek/deepseek-v4-pro-0813",
            constructor_name: deepseek_deepseek_v4_pro_0813,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Pro0813Thinking {
            model_name: "deepseek/deepseek-v4-pro-0813:thinking",
            constructor_name: deepseek_deepseek_v4_pro_0813_thinking,
            display_name: "DeepSeek V4 Pro 0813 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4ProThinking {
            model_name: "deepseek/deepseek-v4-pro:thinking",
            constructor_name: deepseek_deepseek_v4_pro_thinking,
            display_name: "DeepSeek V4 Pro (Thinking)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash {
            model_name: "deepseek/deepseek-v4.1-flash",
            constructor_name: deepseek_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash0910Uncensored {
            model_name: "deepseek/deepseek-v4.1-flash-0910-uncensored",
            constructor_name: deepseek_deepseek_v4_1_flash_0910_uncensored,
            display_name: "DeepSeek V4.1 Flash 0910 Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash0910UncensoredThinking {
            model_name: "deepseek/deepseek-v4.1-flash-0910-uncensored:thinking",
            constructor_name: deepseek_deepseek_v4_1_flash_0910_uncensored_thinking,
            display_name: "DeepSeek V4.1 Flash 0910 Uncensored Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash1017 {
            model_name: "deepseek/deepseek-v4.1-flash-1017",
            constructor_name: deepseek_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash1017Uncensored {
            model_name: "deepseek/deepseek-v4.1-flash-1017-uncensored",
            constructor_name: deepseek_deepseek_v4_1_flash_1017_uncensored,
            display_name: "DeepSeek V4.1 Flash 1017 Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash1017UncensoredThinking {
            model_name: "deepseek/deepseek-v4.1-flash-1017-uncensored:thinking",
            constructor_name: deepseek_deepseek_v4_1_flash_1017_uncensored_thinking,
            display_name: "DeepSeek V4.1 Flash 1017 Uncensored Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash1017Thinking {
            model_name: "deepseek/deepseek-v4.1-flash-1017:thinking",
            constructor_name: deepseek_deepseek_v4_1_flash_1017_thinking,
            display_name: "DeepSeek V4.1 Flash 1017 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41FlashUncensored {
            model_name: "deepseek/deepseek-v4.1-flash-uncensored",
            constructor_name: deepseek_deepseek_v4_1_flash_uncensored,
            display_name: "DeepSeek V4.1 Flash Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41FlashUncensoredThinking {
            model_name: "deepseek/deepseek-v4.1-flash-uncensored:thinking",
            constructor_name: deepseek_deepseek_v4_1_flash_uncensored_thinking,
            display_name: "DeepSeek V4.1 Flash Uncensored Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41FlashThinking {
            model_name: "deepseek/deepseek-v4.1-flash:thinking",
            constructor_name: deepseek_deepseek_v4_1_flash_thinking,
            display_name: "DeepSeek V4.1 Flash Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DoubaoSeed20CodePreview260215 {
            model_name: "doubao-seed-2-0-code-preview-260215",
            constructor_name: doubao_seed_2_0_code_preview_260215,
            display_name: "Doubao Seed 2.0 Code Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DoubaoSeed20Lite260215 {
            model_name: "doubao-seed-2-0-lite-260215",
            constructor_name: doubao_seed_2_0_lite_260215,
            display_name: "Doubao Seed 2.0 Lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DoubaoSeed20Mini260215 {
            model_name: "doubao-seed-2-0-mini-260215",
            constructor_name: doubao_seed_2_0_mini_260215,
            display_name: "Doubao Seed 2.0 Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DoubaoSeed20Pro260215 {
            model_name: "doubao-seed-2-0-pro-260215",
            constructor_name: doubao_seed_2_0_pro_260215,
            display_name: "Doubao Seed 2.0 Pro",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ElorianGlimpse {
            model_name: "elorian/glimpse",
            constructor_name: elorian_glimpse,
            display_name: "Elorian Glimpse",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Ernie50ThinkingPreview {
            model_name: "ernie-5.0-thinking-preview",
            constructor_name: ernie_5_0_thinking_preview,
            display_name: "Ernie 5.0 Thinking Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Ernie51 {
            model_name: "ernie-5.1",
            constructor_name: ernie_5_1,
            display_name: "ERNIE 5.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Ernie51Thinking {
            model_name: "ernie-5.1:thinking",
            constructor_name: ernie_5_1_thinking,
            display_name: "ERNIE 5.1 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        FailspyMetaLlama370bInstructAbliteratedV35 {
            model_name: "failspy/Meta-Llama-3-70B-Instruct-abliterated-v3.5",
            constructor_name: failspy_meta_llama_3_70b_instruct_abliterated_v3_5,
            display_name: "Llama 3 70B abliterated",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Fastgpt {
            model_name: "fastgpt",
            constructor_name: fastgpt,
            display_name: "Web Answer",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        FeatherlessAiQwerky72b {
            model_name: "featherless-ai/Qwerky-72B",
            constructor_name: featherless_ai_qwerky_72b,
            display_name: "Qwerky 72B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        FireworksEmber1 {
            model_name: "fireworks/ember-1",
            constructor_name: fireworks_ember_1,
            display_name: "Ember 1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini20ProExp0205 {
            model_name: "gemini-2.0-pro-exp-02-05",
            constructor_name: gemini_2_0_pro_exp_02_05,
            display_name: "Gemini 2.0 Pro 0205",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini20ProReasoner {
            model_name: "gemini-2.0-pro-reasoner",
            constructor_name: gemini_2_0_pro_reasoner,
            display_name: "Gemini 2.0 Pro Reasoner",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25Flash {
            model_name: "gemini-2.5-flash",
            constructor_name: gemini_2_5_flash,
            display_name: "Gemini 2.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashLite {
            model_name: "gemini-2.5-flash-lite",
            constructor_name: gemini_2_5_flash_lite,
            display_name: "Gemini 2.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashLitePreview0617 {
            model_name: "gemini-2.5-flash-lite-preview-06-17",
            constructor_name: gemini_2_5_flash_lite_preview_06_17,
            display_name: "Gemini 2.5 Flash Lite Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashNothinking {
            model_name: "gemini-2.5-flash-nothinking",
            constructor_name: gemini_2_5_flash_nothinking,
            display_name: "Gemini 2.5 Flash (No Thinking)",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Gemini25FlashPreview092025 {
            model_name: "gemini-2.5-flash-preview-09-2025",
            constructor_name: gemini_2_5_flash_preview_09_2025,
            display_name: "Gemini 2.5 Flash Preview (09/2025)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashPreview092025Thinking {
            model_name: "gemini-2.5-flash-preview-09-2025-thinking",
            constructor_name: gemini_2_5_flash_preview_09_2025_thinking,
            display_name: "Gemini 2.5 Flash Preview (09/2025) – Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25Pro {
            model_name: "gemini-2.5-pro",
            constructor_name: gemini_2_5_pro,
            display_name: "Gemini 2.5 Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25ProExp0325 {
            model_name: "gemini-2.5-pro-exp-03-25",
            constructor_name: gemini_2_5_pro_exp_03_25,
            display_name: "Gemini 2.5 Pro Experimental 0325",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25ProPreview0325 {
            model_name: "gemini-2.5-pro-preview-03-25",
            constructor_name: gemini_2_5_pro_preview_03_25,
            display_name: "Gemini 2.5 Pro Preview 0325",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25ProPreview0506 {
            model_name: "gemini-2.5-pro-preview-05-06",
            constructor_name: gemini_2_5_pro_preview_05_06,
            display_name: "Gemini 2.5 Pro Preview 0506",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25ProPreview0605 {
            model_name: "gemini-2.5-pro-preview-06-05",
            constructor_name: gemini_2_5_pro_preview_06_05,
            display_name: "Gemini 2.5 Pro Preview 0605",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini3ProImagePreview {
            model_name: "gemini-3-pro-image-preview",
            constructor_name: gemini_3_pro_image_preview,
            display_name: "Gemini 3 Pro Image",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GeminiExp1206 {
            model_name: "gemini-exp-1206",
            constructor_name: gemini_exp_1206,
            display_name: "Gemini 2.0 Pro 1206",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemma412bIt {
            model_name: "gemma-4-12b-it",
            constructor_name: gemma_4_12b_it,
            display_name: "Gemma 4 12B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma412bItSemancer {
            model_name: "gemma-4-12b-it-semancer",
            constructor_name: gemma_4_12b_it_semancer,
            display_name: "Gemma 4 12B Semancer",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma412bItStarryOcean {
            model_name: "gemma-4-12b-it-starry-ocean",
            constructor_name: gemma_4_12b_it_starry_ocean,
            display_name: "Gemma 4 12B Starry Ocean",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma412bItStationKeeper {
            model_name: "gemma-4-12b-it-station-keeper",
            constructor_name: gemma_4_12b_it_station_keeper,
            display_name: "Gemma 4 12B StationKeeper",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma426bA4bItChimerax {
            model_name: "gemma-4-26b-a4b-it-chimerax",
            constructor_name: gemma_4_26b_a4b_it_chimerax,
            display_name: "Chimera X",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma426bA4bItChimeraxheretic {
            model_name: "gemma-4-26b-a4b-it-chimeraxheretic",
            constructor_name: gemma_4_26b_a4b_it_chimeraxheretic,
            display_name: "Gemma 4 26B A4B Chimera X Heretic",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma426bA4bItDarksoul {
            model_name: "gemma-4-26b-a4b-it-darksoul",
            constructor_name: gemma_4_26b_a4b_it_darksoul,
            display_name: "Dark Soul",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma426bA4bItLuminous {
            model_name: "gemma-4-26b-a4b-it-luminous",
            constructor_name: gemma_4_26b_a4b_it_luminous,
            display_name: "Luminous Mirror",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma426bA4bItMoonlight {
            model_name: "gemma-4-26b-a4b-it-moonlight",
            constructor_name: gemma_4_26b_a4b_it_moonlight,
            display_name: "Moonlight Dusk",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma426bA4bItMusica {
            model_name: "gemma-4-26b-a4b-it-musica",
            constructor_name: gemma_4_26b_a4b_it_musica,
            display_name: "Musica",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma426bA4bItOpusdistill {
            model_name: "gemma-4-26b-a4b-it-opusdistill",
            constructor_name: gemma_4_26b_a4b_it_opusdistill,
            display_name: "Opus Distill",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma426bA4bItShadowsiren {
            model_name: "gemma-4-26b-a4b-it-shadowsiren",
            constructor_name: gemma_4_26b_a4b_it_shadowsiren,
            display_name: "Shadow Siren",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bItDarkidol {
            model_name: "gemma-4-31b-it-darkidol",
            constructor_name: gemma_4_31b_it_darkidol,
            display_name: "DarkIdol",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bItFabled {
            model_name: "gemma-4-31b-it-fabled",
            constructor_name: gemma_4_31b_it_fabled,
            display_name: "Fabled",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bItGarnet {
            model_name: "gemma-4-31b-it-garnet",
            constructor_name: gemma_4_31b_it_garnet,
            display_name: "Garnet",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bItGembrain {
            model_name: "gemma-4-31b-it-gembrain",
            constructor_name: gemma_4_31b_it_gembrain,
            display_name: "Gembrain",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bItGemsicle {
            model_name: "gemma-4-31b-it-gemsicle",
            constructor_name: gemma_4_31b_it_gemsicle,
            display_name: "Gemsicle",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bItIsometry {
            model_name: "gemma-4-31b-it-isometry",
            constructor_name: gemma_4_31b_it_isometry,
            display_name: "Isometry",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bItNovelist {
            model_name: "gemma-4-31b-it-novelist",
            constructor_name: gemma_4_31b_it_novelist,
            display_name: "Novelist",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bItScarlet {
            model_name: "gemma-4-31b-it-scarlet",
            constructor_name: gemma_4_31b_it_scarlet,
            display_name: "Gemma 4 31B Scarlet",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm4Long {
            model_name: "glm-4-long",
            constructor_name: glm_4_long,
            display_name: "GLM-4 Long",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleDiffusiongemma {
            model_name: "google/diffusiongemma",
            constructor_name: google_diffusiongemma,
            display_name: "DiffusionGemma",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini3FlashPreview {
            model_name: "google/gemini-3-flash-preview",
            constructor_name: google_gemini_3_flash_preview,
            display_name: "Gemini 3 Flash (Preview)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini3FlashPreviewThinking {
            model_name: "google/gemini-3-flash-preview-thinking",
            constructor_name: google_gemini_3_flash_preview_thinking,
            display_name: "Gemini 3 Flash Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        GoogleGemini31FlashLite {
            model_name: "google/gemini-3.1-flash-lite",
            constructor_name: google_gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31ProPreview {
            model_name: "google/gemini-3.1-pro-preview",
            constructor_name: google_gemini_3_1_pro_preview,
            display_name: "Gemini 3.1 Pro (Preview)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31ProPreviewCustomtools {
            model_name: "google/gemini-3.1-pro-preview-customtools",
            constructor_name: google_gemini_3_1_pro_preview_customtools,
            display_name: "Gemini 3.1 Pro (Preview Custom Tools)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31ProPreviewHigh {
            model_name: "google/gemini-3.1-pro-preview-high",
            constructor_name: google_gemini_3_1_pro_preview_high,
            display_name: "Gemini 3.1 Pro (Preview High)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31ProPreviewLow {
            model_name: "google/gemini-3.1-pro-preview-low",
            constructor_name: google_gemini_3_1_pro_preview_low,
            display_name: "Gemini 3.1 Pro (Preview Low)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini35Flash {
            model_name: "google/gemini-3.5-flash",
            constructor_name: google_gemini_3_5_flash,
            display_name: "Gemini 3.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini35FlashLite {
            model_name: "google/gemini-3.5-flash-lite",
            constructor_name: google_gemini_3_5_flash_lite,
            display_name: "Gemini 3.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini35FlashThinking {
            model_name: "google/gemini-3.5-flash-thinking",
            constructor_name: google_gemini_3_5_flash_thinking,
            display_name: "Gemini 3.5 Flash Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        GoogleGemini36Flash {
            model_name: "google/gemini-3.6-flash",
            constructor_name: google_gemini_3_6_flash,
            display_name: "Gemini 3.6 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini37Flash {
            model_name: "google/gemini-3.7-flash",
            constructor_name: google_gemini_3_7_flash,
            display_name: "Gemini 3.7 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini38Flash {
            model_name: "google/gemini-3.8-flash",
            constructor_name: google_gemini_3_8_flash,
            display_name: "Gemini 3.8 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini4Argon {
            model_name: "google/gemini-4-argon",
            constructor_name: google_gemini_4_argon,
            display_name: "Gemini 4 Argon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini4Carbon {
            model_name: "google/gemini-4-carbon",
            constructor_name: google_gemini_4_carbon,
            display_name: "Gemini 4 Carbon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGeminiFlashLatest {
            model_name: "google/gemini-flash-latest",
            constructor_name: google_gemini_flash_latest,
            display_name: "Gemini Flash Latest",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGeminiFlashLiteLatest {
            model_name: "google/gemini-flash-lite-latest",
            constructor_name: google_gemini_flash_lite_latest,
            display_name: "Gemini Flash Lite Latest",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGeminiProLatest {
            model_name: "google/gemini-pro-latest",
            constructor_name: google_gemini_pro_latest,
            display_name: "Gemini Pro Latest",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemma426bA4bIt {
            model_name: "google/gemma-4-26b-a4b-it",
            constructor_name: google_gemma_4_26b_a4b_it,
            display_name: "Gemma 4 26B A4B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemma426bA4bItCybersecurity {
            model_name: "google/gemma-4-26b-a4b-it-cybersecurity",
            constructor_name: google_gemma_4_26b_a4b_it_cybersecurity,
            display_name: "Gemma 4 26B A4B Cybersecurity",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma426bA4bItThinking {
            model_name: "google/gemma-4-26b-a4b-it:thinking",
            constructor_name: google_gemma_4_26b_a4b_it_thinking,
            display_name: "Gemma 4 26B A4B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31b-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma 4 31B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemma431bItThinking {
            model_name: "google/gemma-4-31b-it:thinking",
            constructor_name: google_gemma_4_31b_it_thinking,
            display_name: "Gemma 4 31B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemma431bSplituntied {
            model_name: "google/gemma4-31b-splituntied",
            constructor_name: google_gemma4_31b_splituntied,
            display_name: "Gemma 4 31B Split-Untied",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HeabsyCyberheabsy {
            model_name: "heabsy/cyberheabsy",
            constructor_name: heabsy_cyberheabsy,
            display_name: "CyberHeabsy",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HeabsyQwen3827bUncensoredFast {
            model_name: "heabsy/qwen3.8-27b-uncensored-fast",
            constructor_name: heabsy_qwen3_8_27b_uncensored_fast,
            display_name: "Qwen 3.8 27B Uncensored Fast by Heabsy",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HermesHigh {
            model_name: "hermes-high",
            constructor_name: hermes_high,
            display_name: "Hermes High",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HermesLow {
            model_name: "hermes-low",
            constructor_name: hermes_low,
            display_name: "Hermes Low",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HermesMedium {
            model_name: "hermes-medium",
            constructor_name: hermes_medium,
            display_name: "Hermes Medium",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Holo335bA3b {
            model_name: "holo3-35b-a3b",
            constructor_name: holo3_35b_a3b,
            display_name: "Holo3-35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Holo335bA3bThinking {
            model_name: "holo3-35b-a3b:thinking",
            constructor_name: holo3_35b_a3b_thinking,
            display_name: "Holo3-35B-A3B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HuihuiAiDeepseekR1DistillLlama70bAbliterated {
            model_name: "huihui-ai/DeepSeek-R1-Distill-Llama-70B-abliterated",
            constructor_name: huihui_ai_deepseek_r1_distill_llama_70b_abliterated,
            display_name: "DeepSeek R1 Llama 70B Abliterated",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        HuihuiAiDeepseekR1DistillQwen32bAbliterated {
            model_name: "huihui-ai/DeepSeek-R1-Distill-Qwen-32B-abliterated",
            constructor_name: huihui_ai_deepseek_r1_distill_qwen_32b_abliterated,
            display_name: "DeepSeek R1 Qwen Abliterated",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        HuihuiAiLlama3370bInstructAbliterated {
            model_name: "huihui-ai/Llama-3.3-70B-Instruct-abliterated",
            constructor_name: huihui_ai_llama_3_3_70b_instruct_abliterated,
            display_name: "Llama 3.3 70B Instruct abliterated",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        HuihuiAiQwen2532bInstructAbliterated {
            model_name: "huihui-ai/Qwen2.5-32B-Instruct-abliterated",
            constructor_name: huihui_ai_qwen2_5_32b_instruct_abliterated,
            display_name: "Qwen 2.5 32B Abliterated",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        IbmGraniteGranite428b {
            model_name: "ibm-granite/granite-4.2-8b",
            constructor_name: ibm_granite_granite_4_2_8b,
            display_name: "Granite 4.2 8B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InceptionMercury25Preview {
            model_name: "inception/mercury-2.5-preview",
            constructor_name: inception_mercury_2_5_preview,
            display_name: "Mercury 2.5 Preview",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InceptionMercury3 {
            model_name: "inception/mercury-3",
            constructor_name: inception_mercury_3,
            display_name: "Mercury 3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30Flash {
            model_name: "inclusionai/ling-3.0-flash",
            constructor_name: inclusionai_ling_3_0_flash,
            display_name: "Ling 3.0 Flash",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30FlashVl {
            model_name: "inclusionai/ling-3.0-flash-vl",
            constructor_name: inclusionai_ling_3_0_flash_vl,
            display_name: "Ling 3.0 Flash VL",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        InclusionaiLing30FlashThinking {
            model_name: "inclusionai/ling-3.0-flash:thinking",
            constructor_name: inclusionai_ling_3_0_flash_thinking,
            display_name: "Ling 3.0 Flash Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30Tiny {
            model_name: "inclusionai/ling-3.0-tiny",
            constructor_name: inclusionai_ling_3_0_tiny,
            display_name: "Ling 3.0 Tiny",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30TinyThinking {
            model_name: "inclusionai/ling-3.0-tiny:thinking",
            constructor_name: inclusionai_ling_3_0_tiny_thinking,
            display_name: "Ling 3.0 Tiny Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing31Flash {
            model_name: "inclusionai/ling-3.1-flash",
            constructor_name: inclusionai_ling_3_1_flash,
            display_name: "Ling 3.1 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InferenceNetSchematronV2Small {
            model_name: "inference-net/schematron-v2-small",
            constructor_name: inference_net_schematron_v2_small,
            display_name: "Schematron V2 Small",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        InferenceNetSchematronV2Turbo {
            model_name: "inference-net/schematron-v2-turbo",
            constructor_name: inference_net_schematron_v2_turbo,
            display_name: "Schematron V2 Turbo",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        InflatebotMn12bMagMellR1 {
            model_name: "inflatebot/MN-12B-Mag-Mell-R1",
            constructor_name: inflatebot_mn_12b_mag_mell_r1,
            display_name: "Mag Mell R1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        LightonaiLightonocr21b {
            model_name: "lightonai/LightOnOCR-2-1B",
            constructor_name: lightonai_lightonocr_2_1b,
            display_name: "LightOnOCR 2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LiquidLfm2526b {
            model_name: "liquid/lfm-2.5-2.6b",
            constructor_name: liquid_lfm_2_5_2_6b,
            display_name: "LFM2.5 2.6B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LiquidLfm2524bA2b {
            model_name: "liquid/lfm-2.5-24b-a2b",
            constructor_name: liquid_lfm_2_5_24b_a2b,
            display_name: "LFM2.5 24B A2B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LiquidLfm317b {
            model_name: "liquid/lfm-3-1.7b",
            constructor_name: liquid_lfm_3_1_7b,
            display_name: "LFM3 1.7B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Longcat20 {
            model_name: "longcat-2.0",
            constructor_name: longcat_2_0,
            display_name: "LongCat 2.0",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Longcat20Thinking {
            model_name: "longcat-2.0:thinking",
            constructor_name: longcat_2_0_thinking,
            display_name: "LongCat 2.0 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MeganovaAiMantaFlash10 {
            model_name: "meganova-ai/manta-flash-1.0",
            constructor_name: meganova_ai_manta_flash_1_0,
            display_name: "Manta Flash 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MeganovaAiMantaMini10 {
            model_name: "meganova-ai/manta-mini-1.0",
            constructor_name: meganova_ai_manta_mini_1_0,
            display_name: "Manta Mini 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MeganovaAiMantaPro10 {
            model_name: "meganova-ai/manta-pro-1.0",
            constructor_name: meganova_ai_manta_pro_1_0,
            display_name: "Manta Pro 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MeituanLongcat25Preview {
            model_name: "meituan/longcat-2.5-preview",
            constructor_name: meituan_longcat_2_5_preview,
            display_name: "LongCat 2.5 Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Mercury2 {
            model_name: "mercury-2",
            constructor_name: mercury_2,
            display_name: "Mercury 2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MercuryCoderSmall {
            model_name: "mercury-coder-small",
            constructor_name: mercury_coder_small,
            display_name: "Mercury Coder Small",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama318bInstruct {
            model_name: "meta-llama/llama-3.1-8b-instruct",
            constructor_name: meta_llama_llama_3_1_8b_instruct,
            display_name: "Llama 3.1 8b Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama323bInstruct {
            model_name: "meta-llama/llama-3.2-3b-instruct",
            constructor_name: meta_llama_llama_3_2_3b_instruct,
            display_name: "Llama 3.2 3b Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama3370bInstruct {
            model_name: "meta-llama/llama-3.3-70b-instruct",
            constructor_name: meta_llama_llama_3_3_70b_instruct,
            display_name: "Llama 3.3 70b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama4Maverick {
            model_name: "meta-llama/llama-4-maverick",
            constructor_name: meta_llama_llama_4_maverick,
            display_name: "Llama 4 Maverick",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama4Scout {
            model_name: "meta-llama/llama-4-scout",
            constructor_name: meta_llama_llama_4_scout,
            display_name: "Llama 4 Scout",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseEmber {
            model_name: "meta/muse-ember",
            constructor_name: meta_muse_ember,
            display_name: "Muse Ember",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseGlimmer30b {
            model_name: "meta/muse-glimmer-30b",
            constructor_name: meta_muse_glimmer_30b,
            display_name: "Muse Glimmer 30B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseSpark11 {
            model_name: "meta/muse-spark-1.1",
            constructor_name: meta_muse_spark_1_1,
            display_name: "Muse Spark 1.1",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        MetaMuseSpark12 {
            model_name: "meta/muse-spark-1.2",
            constructor_name: meta_muse_spark_1_2,
            display_name: "Muse Spark 1.2",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSpark12Contributor {
            model_name: "meta/muse-spark-1.2-contributor",
            constructor_name: meta_muse_spark_1_2_contributor,
            display_name: "Muse Spark 1.2 Contributor (Data Used for Training)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSpark13 {
            model_name: "meta/muse-spark-1.3",
            constructor_name: meta_muse_spark_1_3,
            display_name: "Muse Spark 1.3",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSpark13Contributor {
            model_name: "meta/muse-spark-1.3-contributor",
            constructor_name: meta_muse_spark_1_3_contributor,
            display_name: "Muse Spark 1.3 Contributor",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSparkOpen {
            model_name: "meta/muse-spark-open",
            constructor_name: meta_muse_spark_open,
            display_name: "Muse Spark Open",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseWonder {
            model_name: "meta/muse-wonder",
            constructor_name: meta_muse_wonder,
            display_name: "Muse Wonder",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MicrosoftWizardlm28x22b {
            model_name: "microsoft/wizardlm-2-8x22b",
            constructor_name: microsoft_wizardlm_2_8x22b,
            display_name: "WizardLM-2 8x22B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MinimaxMinimax01 {
            model_name: "minimax/minimax-01",
            constructor_name: minimax_minimax_01,
            display_name: "MiniMax 01",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MinimaxMinimaxLatest {
            model_name: "minimax/minimax-latest",
            constructor_name: minimax_minimax_latest,
            display_name: "MiniMax Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM2 {
            model_name: "minimax/minimax-m2",
            constructor_name: minimax_minimax_m2,
            display_name: "MiniMax M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        MinimaxMinimaxM21 {
            model_name: "minimax/minimax-m2.1",
            constructor_name: minimax_minimax_m2_1,
            display_name: "Minimax M2.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM25 {
            model_name: "minimax/minimax-m2.5",
            constructor_name: minimax_minimax_m2_5,
            display_name: "MiniMax M2.5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM25Official {
            model_name: "minimax/minimax-m2.5-official",
            constructor_name: minimax_minimax_m2_5_official,
            display_name: "MiniMax M2.5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM27 {
            model_name: "minimax/minimax-m2.7",
            constructor_name: minimax_minimax_m2_7,
            display_name: "MiniMax M2.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM27Turbo {
            model_name: "minimax/minimax-m2.7-turbo",
            constructor_name: minimax_minimax_m2_7_turbo,
            display_name: "MiniMax M2.7 Turbo",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM3 {
            model_name: "minimax/minimax-m3",
            constructor_name: minimax_minimax_m3,
            display_name: "MiniMax M3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM31Flash {
            model_name: "minimax/minimax-m3.1-flash",
            constructor_name: minimax_minimax_m3_1_flash,
            display_name: "MiniMax M3.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MinimaxMinimaxM3Thinking {
            model_name: "minimax/minimax-m3:thinking",
            constructor_name: minimax_minimax_m3_thinking,
            display_name: "MiniMax M3 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralCodeAgentLatest {
            model_name: "mistral-code-agent-latest",
            constructor_name: mistral_code_agent_latest,
            display_name: "Mistral Code Agent Latest",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralCodeLatest {
            model_name: "mistral-code-latest",
            constructor_name: mistral_code_latest,
            display_name: "Mistral Code Latest",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiCodestral2508 {
            model_name: "mistralai/codestral-2508",
            constructor_name: mistralai_codestral_2508,
            display_name: "Codestral 2508",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiDevstral2123bInstruct2512 {
            model_name: "mistralai/devstral-2-123b-instruct-2512",
            constructor_name: mistralai_devstral_2_123b_instruct_2512,
            display_name: "Devstral 2 123b Instruct 2512",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstralSmall2505 {
            model_name: "mistralai/devstral-small-2505",
            constructor_name: mistralai_devstral_small_2505,
            display_name: "Mistral Devstral Small 2505",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMinistral14b2512 {
            model_name: "mistralai/ministral-14b-2512",
            constructor_name: mistralai_ministral_14b_2512,
            display_name: "Ministral 14B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMinistral14bInstruct2512 {
            model_name: "mistralai/ministral-14b-instruct-2512",
            constructor_name: mistralai_ministral_14b_instruct_2512,
            display_name: "Ministral 14b Instruct 2512",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMinistral3b2512 {
            model_name: "mistralai/ministral-3b-2512",
            constructor_name: mistralai_ministral_3b_2512,
            display_name: "Ministral 3B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMinistral8b2512 {
            model_name: "mistralai/ministral-8b-2512",
            constructor_name: mistralai_ministral_8b_2512,
            display_name: "Ministral 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralLarge {
            model_name: "mistralai/mistral-large",
            constructor_name: mistralai_mistral_large,
            display_name: "Mistral Large 2411",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralLarge3675bInstruct2512 {
            model_name: "mistralai/mistral-large-3-675b-instruct-2512",
            constructor_name: mistralai_mistral_large_3_675b_instruct_2512,
            display_name: "Mistral Large 3 675b Instruct 2512",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralLarge4 {
            model_name: "mistralai/mistral-large-4",
            constructor_name: mistralai_mistral_large_4,
            display_name: "Mistral Large 4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralMedium3 {
            model_name: "mistralai/mistral-medium-3",
            constructor_name: mistralai_mistral_medium_3,
            display_name: "Mistral Medium 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralMedium31 {
            model_name: "mistralai/mistral-medium-3.1",
            constructor_name: mistralai_mistral_medium_3_1,
            display_name: "Mistral Medium 3.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralMedium35 {
            model_name: "mistralai/mistral-medium-3.5",
            constructor_name: mistralai_mistral_medium_3_5,
            display_name: "Mistral Medium 3.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralMedium35Thinking {
            model_name: "mistralai/mistral-medium-3.5:thinking",
            constructor_name: mistralai_mistral_medium_3_5_thinking,
            display_name: "Mistral Medium 3.5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralNemoInstruct2407 {
            model_name: "mistralai/mistral-nemo-instruct-2407",
            constructor_name: mistralai_mistral_nemo_instruct_2407,
            display_name: "Mistral Nemo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralSaba {
            model_name: "mistralai/mistral-saba",
            constructor_name: mistralai_mistral_saba,
            display_name: "Mistral Saba",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralSmall24bInstruct2501 {
            model_name: "mistralai/mistral-small-24b-instruct-2501",
            constructor_name: mistralai_mistral_small_24b_instruct_2501,
            display_name: "Mistral Small 24B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall3124bInstruct {
            model_name: "mistralai/mistral-small-3.1-24b-instruct",
            constructor_name: mistralai_mistral_small_3_1_24b_instruct,
            display_name: "Mistral Small 3.1 24B (2503)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralSmall3224bInstruct {
            model_name: "mistralai/mistral-small-3.2-24b-instruct",
            constructor_name: mistralai_mistral_small_3_2_24b_instruct,
            display_name: "Mistral Small 3.2 24B (2506)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralSmall4119b2603 {
            model_name: "mistralai/mistral-small-4-119b-2603",
            constructor_name: mistralai_mistral_small_4_119b_2603,
            display_name: "Mistral Small 4 119B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall4119b2603Thinking {
            model_name: "mistralai/mistral-small-4-119b-2603:thinking",
            constructor_name: mistralai_mistral_small_4_119b_2603_thinking,
            display_name: "Mistral Small 4 119B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMixtral8x22bInstructV01 {
            model_name: "mistralai/mixtral-8x22b-instruct-v0.1",
            constructor_name: mistralai_mixtral_8x22b_instruct_v0_1,
            display_name: "Mixtral 8x22B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MlabonneNeuraldaredevil8bAbliterated {
            model_name: "mlabonne/NeuralDaredevil-8B-abliterated",
            constructor_name: mlabonne_neuraldaredevil_8b_abliterated,
            display_name: "Neural Daredevil 8B abliterated",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MoondreamMoondream3Preview {
            model_name: "moondream/moondream3-preview",
            constructor_name: moondream_moondream3_preview,
            display_name: "Moondream 3 Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        MoondreamMoondream319bA2b {
            model_name: "moondream/moondream3.1-9b-a2b",
            constructor_name: moondream_moondream3_1_9b_a2b,
            display_name: "Moondream 3.1 9B A2B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        MoonshotaiKimiK2Instruct0905 {
            model_name: "moonshotai/Kimi-K2-Instruct-0905",
            constructor_name: moonshotai_kimi_k2_instruct_0905,
            display_name: "Kimi K2 0905",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct {
            model_name: "moonshotai/kimi-k2-instruct",
            constructor_name: moonshotai_kimi_k2_instruct,
            display_name: "Kimi K2 Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct0711 {
            model_name: "moonshotai/kimi-k2-instruct-0711",
            constructor_name: moonshotai_kimi_k2_instruct_0711,
            display_name: "Kimi K2 0711",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/kimi-k2-thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/kimi-k2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25Thinking {
            model_name: "moonshotai/kimi-k2.5-thinking",
            constructor_name: moonshotai_kimi_k2_5_thinking,
            display_name: "Kimi K2.5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26 {
            model_name: "moonshotai/kimi-k2.6",
            constructor_name: moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26Thinking {
            model_name: "moonshotai/kimi-k2.6:thinking",
            constructor_name: moonshotai_kimi_k2_6_thinking,
            display_name: "Kimi K2.6 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK27Code {
            model_name: "moonshotai/kimi-k2.7-code",
            constructor_name: moonshotai_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK27CodeHighspeed {
            model_name: "moonshotai/kimi-k2.7-code-highspeed",
            constructor_name: moonshotai_kimi_k2_7_code_highspeed,
            display_name: "Kimi K2.7 Code High-Speed",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK3 {
            model_name: "moonshotai/kimi-k3",
            constructor_name: moonshotai_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiK31Pro {
            model_name: "moonshotai/kimi-k3.1-pro",
            constructor_name: moonshotai_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiK31ProHighspeed {
            model_name: "moonshotai/kimi-k3.1-pro-highspeed",
            constructor_name: moonshotai_kimi_k3_1_pro_highspeed,
            display_name: "Kimi K3.1 Pro Highspeed",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiLatest {
            model_name: "moonshotai/kimi-latest",
            constructor_name: moonshotai_kimi_latest,
            display_name: "Kimi Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NanoGptHelp {
            model_name: "nano-gpt-help",
            constructor_name: nano_gpt_help,
            display_name: "NanoGPT Help",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NanogptCodingRouter {
            model_name: "nanogpt/coding-router",
            constructor_name: nanogpt_coding_router,
            display_name: "Coding Router",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NanogptCodingRouterHigh {
            model_name: "nanogpt/coding-router:high",
            constructor_name: nanogpt_coding_router_high,
            display_name: "Coding Router High",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NanogptCodingRouterLow {
            model_name: "nanogpt/coding-router:low",
            constructor_name: nanogpt_coding_router_low,
            display_name: "Coding Router Low",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NanogptCodingRouterMax {
            model_name: "nanogpt/coding-router:max",
            constructor_name: nanogpt_coding_router_max,
            display_name: "Coding Router Max",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NanogptCodingRouterMedium {
            model_name: "nanogpt/coding-router:medium",
            constructor_name: nanogpt_coding_router_medium,
            display_name: "Coding Router Medium",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NexAgiNexN25Mini {
            model_name: "nex-agi/nex-n2.5-mini",
            constructor_name: nex_agi_nex_n2_5_mini,
            display_name: "Nex N2.5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        NexAgiNexN25Pro {
            model_name: "nex-agi/nex-n2.5-pro",
            constructor_name: nex_agi_nex_n2_5_pro,
            display_name: "Nex N2.5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NexAgiNexN26 {
            model_name: "nex-agi/nex-n2.6",
            constructor_name: nex_agi_nex_n2_6,
            display_name: "Nex N2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NothingiisrealL3170bCelesteV01Bf16 {
            model_name: "nothingiisreal/L3.1-70B-Celeste-V0.1-BF16",
            constructor_name: nothingiisreal_l3_1_70b_celeste_v0_1_bf16,
            display_name: "Llama 3.1 70B Celeste v0.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NousresearchHermes4405bThinking {
            model_name: "nousresearch/hermes-4-405b:thinking",
            constructor_name: nousresearch_hermes_4_405b_thinking,
            display_name: "Hermes 4 405b Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaLlama31Nemotron70bInstructHf {
            model_name: "nvidia/Llama-3.1-Nemotron-70B-Instruct-HF",
            constructor_name: nvidia_llama_3_1_nemotron_70b_instruct_hf,
            display_name: "Nvidia Nemotron 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama33NemotronSuper49bV1 {
            model_name: "nvidia/Llama-3.3-Nemotron-Super-49B-v1",
            constructor_name: nvidia_llama_3_3_nemotron_super_49b_v1,
            display_name: "Nvidia Nemotron Super 49B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama33NemotronSuper49bV15 {
            model_name: "nvidia/llama-3_3-nemotron-super-49b-v1_5",
            constructor_name: nvidia_llama_3_3_nemotron_super_49b_v1_5,
            display_name: "Llama 3 3 Nemotron Super 49B V1 5",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Nano30bA3b {
            model_name: "nvidia/nemotron-3-nano-30b-a3b",
            constructor_name: nvidia_nemotron_3_nano_30b_a3b,
            display_name: "Nvidia Nemotron 3 Nano 30B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Super120bA12b {
            model_name: "nvidia/nemotron-3-super-120b-a12b",
            constructor_name: nvidia_nemotron_3_super_120b_a12b,
            display_name: "Nvidia Nemotron 3 Super 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Super120bA12bThinking {
            model_name: "nvidia/nemotron-3-super-120b-a12b:thinking",
            constructor_name: nvidia_nemotron_3_super_120b_a12b_thinking,
            display_name: "Nvidia Nemotron 3 Super 120B Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Ultra550bA55b {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b,
            display_name: "Nvidia Nemotron 3 Ultra 550B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Ultra550bA55bThinking {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b:thinking",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b_thinking,
            display_name: "Nvidia Nemotron 3 Ultra 550B Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35ContentSafety {
            model_name: "nvidia/nemotron-3.5-content-safety",
            constructor_name: nvidia_nemotron_3_5_content_safety,
            display_name: "Nemotron 3.5 Content Safety",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron35Lightning {
            model_name: "nvidia/nemotron-3.5-lightning",
            constructor_name: nvidia_nemotron_3_5_lightning,
            display_name: "Nvidia Nemotron 3.5 Lightning",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35LightningThinking {
            model_name: "nvidia/nemotron-3.5-lightning:thinking",
            constructor_name: nvidia_nemotron_3_5_lightning_thinking,
            display_name: "Nvidia Nemotron 3.5 Lightning Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35SuperVl {
            model_name: "nvidia/nemotron-3.5-super-vl",
            constructor_name: nvidia_nemotron_3_5_super_vl,
            display_name: "Nvidia Nemotron 3.5 Super VL",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        OpenaiGpt41 {
            model_name: "openai/gpt-4.1",
            constructor_name: openai_gpt_4_1,
            display_name: "GPT 4.1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Mini {
            model_name: "openai/gpt-4.1-mini",
            constructor_name: openai_gpt_4_1_mini,
            display_name: "GPT 4.1 Mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt41Nano {
            model_name: "openai/gpt-4.1-nano",
            constructor_name: openai_gpt_4_1_nano,
            display_name: "GPT 4.1 Nano",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt4o {
            model_name: "openai/gpt-4o",
            constructor_name: openai_gpt_4o,
            display_name: "GPT-4o",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4o20240806 {
            model_name: "openai/gpt-4o-2024-08-06",
            constructor_name: openai_gpt_4o_2024_08_06,
            display_name: "GPT-4o (2024-08-06)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4o20241120 {
            model_name: "openai/gpt-4o-2024-11-20",
            constructor_name: openai_gpt_4o_2024_11_20,
            display_name: "GPT-4o (2024-11-20)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMini {
            model_name: "openai/gpt-4o-mini",
            constructor_name: openai_gpt_4o_mini,
            display_name: "GPT-4o mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt5 {
            model_name: "openai/gpt-5",
            constructor_name: openai_gpt_5,
            display_name: "GPT 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Mini {
            model_name: "openai/gpt-5-mini",
            constructor_name: openai_gpt_5_mini,
            display_name: "GPT 5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt5Nano {
            model_name: "openai/gpt-5-nano",
            constructor_name: openai_gpt_5_nano,
            display_name: "GPT 5 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt5Pro {
            model_name: "openai/gpt-5-pro",
            constructor_name: openai_gpt_5_pro,
            display_name: "GPT 5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt51 {
            model_name: "openai/gpt-5.1",
            constructor_name: openai_gpt_5_1,
            display_name: "GPT 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5120251113 {
            model_name: "openai/gpt-5.1-2025-11-13",
            constructor_name: openai_gpt_5_1_2025_11_13,
            display_name: "GPT-5.1 (2025-11-13)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Codex {
            model_name: "openai/gpt-5.1-codex",
            constructor_name: openai_gpt_5_1_codex,
            display_name: "GPT 5.1 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt51CodexMax {
            model_name: "openai/gpt-5.1-codex-max",
            constructor_name: openai_gpt_5_1_codex_max,
            display_name: "GPT 5.1 Codex Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51CodexMini {
            model_name: "openai/gpt-5.1-codex-mini",
            constructor_name: openai_gpt_5_1_codex_mini,
            display_name: "GPT 5.1 Codex Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt52 {
            model_name: "openai/gpt-5.2",
            constructor_name: openai_gpt_5_2,
            display_name: "GPT 5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Codex {
            model_name: "openai/gpt-5.2-codex",
            constructor_name: openai_gpt_5_2_codex,
            display_name: "GPT 5.2 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt53Codex {
            model_name: "openai/gpt-5.3-codex",
            constructor_name: openai_gpt_5_3_codex,
            display_name: "GPT 5.3 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54 {
            model_name: "openai/gpt-5.4",
            constructor_name: openai_gpt_5_4,
            display_name: "GPT 5.4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54Mini {
            model_name: "openai/gpt-5.4-mini",
            constructor_name: openai_gpt_5_4_mini,
            display_name: "GPT 5.4 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54Nano {
            model_name: "openai/gpt-5.4-nano",
            constructor_name: openai_gpt_5_4_nano,
            display_name: "GPT 5.4 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt55 {
            model_name: "openai/gpt-5.5",
            constructor_name: openai_gpt_5_5,
            display_name: "GPT 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Luna {
            model_name: "openai/gpt-5.6-luna",
            constructor_name: openai_gpt_5_6_luna,
            display_name: "GPT 5.6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56LunaPro {
            model_name: "openai/gpt-5.6-luna-pro",
            constructor_name: openai_gpt_5_6_luna_pro,
            display_name: "GPT 5.6 Luna Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Sol {
            model_name: "openai/gpt-5.6-sol",
            constructor_name: openai_gpt_5_6_sol,
            display_name: "GPT 5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56SolPro {
            model_name: "openai/gpt-5.6-sol-pro",
            constructor_name: openai_gpt_5_6_sol_pro,
            display_name: "GPT 5.6 Sol Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Terra {
            model_name: "openai/gpt-5.6-terra",
            constructor_name: openai_gpt_5_6_terra,
            display_name: "GPT 5.6 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56TerraPro {
            model_name: "openai/gpt-5.6-terra-pro",
            constructor_name: openai_gpt_5_6_terra_pro,
            display_name: "GPT 5.6 Terra Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Astra {
            model_name: "openai/gpt-6-astra",
            constructor_name: openai_gpt_6_astra,
            display_name: "GPT 6 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6AstraPro {
            model_name: "openai/gpt-6-astra-pro",
            constructor_name: openai_gpt_6_astra_pro,
            display_name: "GPT 6 Astra Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Luna {
            model_name: "openai/gpt-6-luna",
            constructor_name: openai_gpt_6_luna,
            display_name: "GPT 6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6LunaPro {
            model_name: "openai/gpt-6-luna-pro",
            constructor_name: openai_gpt_6_luna_pro,
            display_name: "GPT 6 Luna Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Sol {
            model_name: "openai/gpt-6-sol",
            constructor_name: openai_gpt_6_sol,
            display_name: "GPT 6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6SolPro {
            model_name: "openai/gpt-6-sol-pro",
            constructor_name: openai_gpt_6_sol_pro,
            display_name: "GPT 6 Sol Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61Sol {
            model_name: "openai/gpt-6.1-sol",
            constructor_name: openai_gpt_6_1_sol,
            display_name: "GPT 6.1 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61SolPro {
            model_name: "openai/gpt-6.1-sol-pro",
            constructor_name: openai_gpt_6_1_sol_pro,
            display_name: "GPT 6.1 Sol Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61Terra {
            model_name: "openai/gpt-6.1-terra",
            constructor_name: openai_gpt_6_1_terra,
            display_name: "GPT 6.1 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61TerraPro {
            model_name: "openai/gpt-6.1-terra-pro",
            constructor_name: openai_gpt_6_1_terra_pro,
            display_name: "GPT 6.1 Terra Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptAstraLatest {
            model_name: "openai/gpt-astra-latest",
            constructor_name: openai_gpt_astra_latest,
            display_name: "GPT Astra Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptChatLatest {
            model_name: "openai/gpt-chat-latest",
            constructor_name: openai_gpt_chat_latest,
            display_name: "GPT Chat Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptLatest {
            model_name: "openai/gpt-latest",
            constructor_name: openai_gpt_latest,
            display_name: "GPT Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptLunaLatest {
            model_name: "openai/gpt-luna-latest",
            constructor_name: openai_gpt_luna_latest,
            display_name: "GPT Luna Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT Oss 120b",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOssSafeguard20b {
            model_name: "openai/gpt-oss-safeguard-20b",
            constructor_name: openai_gpt_oss_safeguard_20b,
            display_name: "GPT OSS Safeguard 20B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptSolLatest {
            model_name: "openai/gpt-sol-latest",
            constructor_name: openai_gpt_sol_latest,
            display_name: "GPT Sol Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptTerraLatest {
            model_name: "openai/gpt-terra-latest",
            constructor_name: openai_gpt_terra_latest,
            display_name: "GPT Terra Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO1 {
            model_name: "openai/o1",
            constructor_name: openai_o1,
            display_name: "OpenAI o1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiO1Pro {
            model_name: "openai/o1-pro",
            constructor_name: openai_o1_pro,
            display_name: "OpenAI o1 Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiO3 {
            model_name: "openai/o3",
            constructor_name: openai_o3,
            display_name: "OpenAI o3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiO3MiniLow {
            model_name: "openai/o3-mini-low",
            constructor_name: openai_o3_mini_low,
            display_name: "OpenAI o3-mini (Low)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3Pro20250610 {
            model_name: "openai/o3-pro-2025-06-10",
            constructor_name: openai_o3_pro_2025_06_10,
            display_name: "OpenAI o3-pro (2025-06-10)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OrcaOrcacyberFlash10 {
            model_name: "orca/orcacyber-flash-1.0",
            constructor_name: orca_orcacyber_flash_1_0,
            display_name: "OrcaCyber DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OrcaOrcacyberFlash10Thinking {
            model_name: "orca/orcacyber-flash-1.0:thinking",
            constructor_name: orca_orcacyber_flash_1_0_thinking,
            display_name: "OrcaCyber DeepSeek V4.1 Flash Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OrcaOrcacyberGlm53Flash {
            model_name: "orca/orcacyber-glm-5.3-flash",
            constructor_name: orca_orcacyber_glm_5_3_flash,
            display_name: "OrcaCyber GLM 5.3 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OrcaOrcacyberK31Pro {
            model_name: "orca/orcacyber-k3.1-pro",
            constructor_name: orca_orcacyber_k3_1_pro,
            display_name: "OrcaCyber K3.1 Pro Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OrcaOrcacyberMimoFlash {
            model_name: "orca/orcacyber-mimo-flash",
            constructor_name: orca_orcacyber_mimo_flash,
            display_name: "OrcaCyber MiMo V2.6 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OrcaOrcacyberMinimaxM31Flash {
            model_name: "orca/orcacyber-minimax-m3.1-flash",
            constructor_name: orca_orcacyber_minimax_m3_1_flash,
            display_name: "OrcaCyber MiniMax M3.1 Flash Uncensored",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OrcaOrcacyberQwen38FlashNext {
            model_name: "orca/orcacyber-qwen3.8-flash-next",
            constructor_name: orca_orcacyber_qwen3_8_flash_next,
            display_name: "OrcaCyber Qwen3.8 Flash Next Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OrcaOrcacyberZero15 {
            model_name: "orca/orcacyber-zero-1.5",
            constructor_name: orca_orcacyber_zero_1_5,
            display_name: "OrcaCyber Zero 1.5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OrnithAiOrnith15122b {
            model_name: "ornith-ai/ornith-1.5-122b",
            constructor_name: ornith_ai_ornith_1_5_122b,
            display_name: "Ornith 1.5 122B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OrnithAiOrnith1535bA3b {
            model_name: "ornith-ai/ornith-1.5-35b-a3b",
            constructor_name: ornith_ai_ornith_1_5_35b_a3b,
            display_name: "Ornith 1.5 35B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OrnithAiOrnith1535bA3bThinking {
            model_name: "ornith-ai/ornith-1.5-35b-a3b:thinking",
            constructor_name: ornith_ai_ornith_1_5_35b_a3b_thinking,
            display_name: "Ornith 1.5 35B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PamanseauOpenreasoningNemotron32b {
            model_name: "pamanseau/OpenReasoning-Nemotron-32B",
            constructor_name: pamanseau_openreasoning_nemotron_32b,
            display_name: "OpenReasoning Nemotron 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ParticleAiParticle1 {
            model_name: "particle-ai/particle-1",
            constructor_name: particle_ai_particle_1,
            display_name: "Particle-1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PerceptronPerceptronMk1 {
            model_name: "perceptron/perceptron-mk1",
            constructor_name: perceptron_perceptron_mk1,
            display_name: "Perceptron Mk1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        PerceptronPerceptronMk15 {
            model_name: "perceptron/perceptron-mk1.5",
            constructor_name: perceptron_perceptron_mk1_5,
            display_name: "Perceptron Mk1.5",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Phi4MiniInstruct {
            model_name: "phi-4-mini-instruct",
            constructor_name: phi_4_mini_instruct,
            display_name: "Phi 4 Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Phi4MultimodalInstruct {
            model_name: "phi-4-multimodal-instruct",
            constructor_name: phi_4_multimodal_instruct,
            display_name: "Phi 4 Multimodal",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        PokeeIsaac {
            model_name: "pokee-isaac",
            constructor_name: pokee_isaac,
            display_name: "Pokee-Isaac 28B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PqhazDeepseekV41Flash1017Heretic {
            model_name: "pqhaz/DeepSeek-V4.1-Flash-1017-heretic",
            constructor_name: pqhaz_deepseek_v4_1_flash_1017_heretic,
            display_name: "DeepSeek V4.1 Flash Heretic",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PqhazDeepseekV41Flash1017HereticThinking {
            model_name: "pqhaz/DeepSeek-V4.1-Flash-1017-heretic:thinking",
            constructor_name: pqhaz_deepseek_v4_1_flash_1017_heretic_thinking,
            display_name: "DeepSeek V4.1 Flash Heretic Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PrismMlTernaryBonsai227b {
            model_name: "prism-ml/ternary-bonsai-2-27b",
            constructor_name: prism_ml_ternary_bonsai_2_27b,
            display_name: "Ternary Bonsai 2 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PrismMlTernaryBonsai29b {
            model_name: "prism-ml/ternary-bonsai-2-9b",
            constructor_name: prism_ml_ternary_bonsai_2_9b,
            display_name: "Ternary Bonsai2 9B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen2572bInstruct {
            model_name: "qwen/qwen-2.5-72b-instruct",
            constructor_name: qwen_qwen_2_5_72b_instruct,
            display_name: "Qwen2.5 72B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwenMax {
            model_name: "qwen/qwen-max",
            constructor_name: qwen_qwen_max,
            display_name: "Qwen 2.5 Max",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwenPlus {
            model_name: "qwen/qwen-plus",
            constructor_name: qwen_qwen_plus,
            display_name: "Qwen Plus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwenTurbo {
            model_name: "qwen/qwen-turbo",
            constructor_name: qwen_qwen_turbo,
            display_name: "Qwen Turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen25Coder32bInstruct {
            model_name: "qwen/qwen2.5-coder-32b-instruct",
            constructor_name: qwen_qwen2_5_coder_32b_instruct,
            display_name: "Qwen 2.5 Coder 32b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen25Vl72bInstruct {
            model_name: "qwen/qwen2.5-vl-72b-instruct",
            constructor_name: qwen_qwen2_5_vl_72b_instruct,
            display_name: "Qwen25 VL 72b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen314b {
            model_name: "qwen/qwen3-14b",
            constructor_name: qwen_qwen3_14b,
            display_name: "Qwen 3 14b",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22b {
            model_name: "qwen/qwen3-235b-a22b",
            constructor_name: qwen_qwen3_235b_a22b,
            display_name: "Qwen 3 235b A22B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bInstruct2507 {
            model_name: "qwen/qwen3-235b-a22b-instruct-2507",
            constructor_name: qwen_qwen3_235b_a22b_instruct_2507,
            display_name: "Qwen 3 235b A22B 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bThinking2507 {
            model_name: "qwen/qwen3-235b-a22b-thinking-2507",
            constructor_name: qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3 235B A22B Thinking 2507",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3b {
            model_name: "qwen/qwen3-30b-a3b",
            constructor_name: qwen_qwen3_30b_a3b,
            display_name: "Qwen3 30B A3B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen330bA3bInstruct2507 {
            model_name: "qwen/qwen3-30b-a3b-instruct-2507",
            constructor_name: qwen_qwen3_30b_a3b_instruct_2507,
            display_name: "Qwen3 30B A3B Instruct 2507",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen332b {
            model_name: "qwen/qwen3-32b",
            constructor_name: qwen_qwen3_32b,
            display_name: "Qwen 3 32b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen38b {
            model_name: "qwen/qwen3-8b",
            constructor_name: qwen_qwen3_8b,
            display_name: "Qwen 3 8B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder {
            model_name: "qwen/qwen3-coder",
            constructor_name: qwen_qwen3_coder,
            display_name: "Qwen3 Coder",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder30bA3bInstruct {
            model_name: "qwen/qwen3-coder-30b-a3b-instruct",
            constructor_name: qwen_qwen3_coder_30b_a3b_instruct,
            display_name: "Qwen3 Coder 30B A3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderFlash {
            model_name: "qwen/qwen3-coder-flash",
            constructor_name: qwen_qwen3_coder_flash,
            display_name: "Qwen3 Coder Flash",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3CoderNext {
            model_name: "qwen/qwen3-coder-next",
            constructor_name: qwen_qwen3_coder_next,
            display_name: "Qwen3 Coder Next",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bInstruct {
            model_name: "qwen/qwen3-next-80b-a3b-instruct",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3 Next 80B A3B (Instruct)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bThinking {
            model_name: "qwen/qwen3-next-80b-a3b-thinking",
            constructor_name: qwen_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3 Next 80B A3B (Thinking)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Vl235bA22bInstruct {
            model_name: "qwen/qwen3-vl-235b-a22b-instruct",
            constructor_name: qwen_qwen3_vl_235b_a22b_instruct,
            display_name: "Qwen3 VL 235B A22B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Vl235bA22bThinking {
            model_name: "qwen/qwen3-vl-235b-a22b-thinking",
            constructor_name: qwen_qwen3_vl_235b_a22b_thinking,
            display_name: "Qwen3 VL 235B A22B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen35122bA10b {
            model_name: "qwen/qwen3.5-122b-a10b",
            constructor_name: qwen_qwen3_5_122b_a10b,
            display_name: "Qwen3.5 122B A10B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen35122bA10bThinking {
            model_name: "qwen/qwen3.5-122b-a10b:thinking",
            constructor_name: qwen_qwen3_5_122b_a10b_thinking,
            display_name: "Qwen3.5 122B A10B Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3527b {
            model_name: "qwen/qwen3.5-27b",
            constructor_name: qwen_qwen3_5_27b,
            display_name: "Qwen3.5 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3527bThinking {
            model_name: "qwen/qwen3.5-27b:thinking",
            constructor_name: qwen_qwen3_5_27b_thinking,
            display_name: "Qwen3.5 27B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3535bA3b {
            model_name: "qwen/qwen3.5-35b-a3b",
            constructor_name: qwen_qwen3_5_35b_a3b,
            display_name: "Qwen3.5 35B A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3535bA3bThinking {
            model_name: "qwen/qwen3.5-35b-a3b:thinking",
            constructor_name: qwen_qwen3_5_35b_a3b_thinking,
            display_name: "Qwen3.5 35B A3B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen35397bA17b {
            model_name: "qwen/qwen3.5-397b-a17b",
            constructor_name: qwen_qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B A17B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen35397bA17bThinking {
            model_name: "qwen/qwen3.5-397b-a17b:thinking",
            constructor_name: qwen_qwen3_5_397b_a17b_thinking,
            display_name: "Qwen3.5 397B A17B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen359b {
            model_name: "qwen/qwen3.5-9b",
            constructor_name: qwen_qwen3_5_9b,
            display_name: "Qwen3.5 9B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen35Flash {
            model_name: "qwen/qwen3.5-flash",
            constructor_name: qwen_qwen3_5_flash,
            display_name: "Qwen3.5 Flash",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen35FlashThinking {
            model_name: "qwen/qwen3.5-flash:thinking",
            constructor_name: qwen_qwen3_5_flash_thinking,
            display_name: "Qwen3.5 Flash Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen35OmniFlash {
            model_name: "qwen/qwen3.5-omni-flash",
            constructor_name: qwen_qwen3_5_omni_flash,
            display_name: "Qwen3.5 Omni Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen35OmniPlus {
            model_name: "qwen/qwen3.5-omni-plus",
            constructor_name: qwen_qwen3_5_omni_plus,
            display_name: "Qwen3.5 Omni Plus",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen35Plus {
            model_name: "qwen/qwen3.5-plus",
            constructor_name: qwen_qwen3_5_plus,
            display_name: "Qwen3.5 Plus",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen35PlusThinking {
            model_name: "qwen/qwen3.5-plus:thinking",
            constructor_name: qwen_qwen3_5_plus_thinking,
            display_name: "Qwen3.5 Plus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen3627b {
            model_name: "qwen/qwen3.6-27b",
            constructor_name: qwen_qwen3_6_27b,
            display_name: "Qwen3.6 27B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3627bThinking {
            model_name: "qwen/qwen3.6-27b:thinking",
            constructor_name: qwen_qwen3_6_27b_thinking,
            display_name: "Qwen3.6 27B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3635bA3b {
            model_name: "qwen/qwen3.6-35b-a3b",
            constructor_name: qwen_qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B A3B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3635bA3bThinking {
            model_name: "qwen/qwen3.6-35b-a3b:thinking",
            constructor_name: qwen_qwen3_6_35b_a3b_thinking,
            display_name: "Qwen3.6 35B A3B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen36Flash {
            model_name: "qwen/qwen3.6-flash",
            constructor_name: qwen_qwen3_6_flash,
            display_name: "Qwen3.6 Flash",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen36MaxPreview {
            model_name: "qwen/qwen3.6-max-preview",
            constructor_name: qwen_qwen3_6_max_preview,
            display_name: "Qwen3.6 Max Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen36Plus {
            model_name: "qwen/qwen3.6-plus",
            constructor_name: qwen_qwen3_6_plus,
            display_name: "Qwen 3.6 Plus",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen37Flash {
            model_name: "qwen/qwen3.7-flash",
            constructor_name: qwen_qwen3_7_flash,
            display_name: "Qwen3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen37FlashThinking {
            model_name: "qwen/qwen3.7-flash:thinking",
            constructor_name: qwen_qwen3_7_flash_thinking,
            display_name: "Qwen3.7 Flash Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen37Max {
            model_name: "qwen/qwen3.7-max",
            constructor_name: qwen_qwen3_7_max,
            display_name: "Qwen3.7 Max",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen37MaxThinking {
            model_name: "qwen/qwen3.7-max:thinking",
            constructor_name: qwen_qwen3_7_max_thinking,
            display_name: "Qwen3.7 Max Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen37Plus {
            model_name: "qwen/qwen3.7-plus",
            constructor_name: qwen_qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen37PlusThinking {
            model_name: "qwen/qwen3.7-plus:thinking",
            constructor_name: qwen_qwen3_7_plus_thinking,
            display_name: "Qwen3.7 Plus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3827b {
            model_name: "qwen/qwen3.8-27b",
            constructor_name: qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3827bAntislop {
            model_name: "qwen/qwen3.8-27b-antislop",
            constructor_name: qwen_qwen3_8_27b_antislop,
            display_name: "Qwen 3.8 27B Antislop",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bCybersecurity {
            model_name: "qwen/qwen3.8-27b-cybersecurity",
            constructor_name: qwen_qwen3_8_27b_cybersecurity,
            display_name: "Qwen 3.8 27B Cybersecurity",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bDarkidol {
            model_name: "qwen/qwen3.8-27b-darkidol",
            constructor_name: qwen_qwen3_8_27b_darkidol,
            display_name: "Qwen 3.8 27B DarkIdol",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bFable {
            model_name: "qwen/qwen3.8-27b-fable",
            constructor_name: qwen_qwen3_8_27b_fable,
            display_name: "Qwen 3.8 27B Fable",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bFimi {
            model_name: "qwen/qwen3.8-27b-fimi",
            constructor_name: qwen_qwen3_8_27b_fimi,
            display_name: "Qwen 3.8 27B Fimi",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bHemmingway {
            model_name: "qwen/qwen3.8-27b-hemmingway",
            constructor_name: qwen_qwen3_8_27b_hemmingway,
            display_name: "Qwen 3.8 27B Hemingway",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bKimiko {
            model_name: "qwen/qwen3.8-27b-kimiko",
            constructor_name: qwen_qwen3_8_27b_kimiko,
            display_name: "Qwen 3.8 27B Kimiko",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bKiwen {
            model_name: "qwen/qwen3.8-27b-kiwen",
            constructor_name: qwen_qwen3_8_27b_kiwen,
            display_name: "Qwen 3.8 27B Kiwen",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bNovelist {
            model_name: "qwen/qwen3.8-27b-novelist",
            constructor_name: qwen_qwen3_8_27b_novelist,
            display_name: "Qwen 3.8 27B Novelist",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bQueen {
            model_name: "qwen/qwen3.8-27b-queen",
            constructor_name: qwen_qwen3_8_27b_queen,
            display_name: "Qwen 3.8 27B Queen",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bSwift {
            model_name: "qwen/qwen3.8-27b-swift",
            constructor_name: qwen_qwen3_8_27b_swift,
            display_name: "Qwen 3.8 27B Swift",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bSynthia {
            model_name: "qwen/qwen3.8-27b-synthia",
            constructor_name: qwen_qwen3_8_27b_synthia,
            display_name: "Qwen 3.8 27B Synthia",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3827bUncensored {
            model_name: "qwen/qwen3.8-27b-uncensored",
            constructor_name: qwen_qwen3_8_27b_uncensored,
            display_name: "Qwen 3.8 27B Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bUncensoredThinking {
            model_name: "qwen/qwen3.8-27b-uncensored:thinking",
            constructor_name: qwen_qwen3_8_27b_uncensored_thinking,
            display_name: "Qwen 3.8 27B Uncensored Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bThinking {
            model_name: "qwen/qwen3.8-27b:thinking",
            constructor_name: qwen_qwen3_8_27b_thinking,
            display_name: "Qwen3.8 27B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen38Flash {
            model_name: "qwen/qwen3.8-flash",
            constructor_name: qwen_qwen3_8_flash,
            display_name: "Qwen3.8 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen38Max {
            model_name: "qwen/qwen3.8-max",
            constructor_name: qwen_qwen3_8_max,
            display_name: "Qwen3.8 Max",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen38Max0902 {
            model_name: "qwen/qwen3.8-max-0902",
            constructor_name: qwen_qwen3_8_max_0902,
            display_name: "Qwen3.8 Max 0902",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen38MaxPrime {
            model_name: "qwen/qwen3.8-max-prime",
            constructor_name: qwen_qwen3_8_max_prime,
            display_name: "Qwen3.8 Max Prime",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen38MaxThinking {
            model_name: "qwen/qwen3.8-max:thinking",
            constructor_name: qwen_qwen3_8_max_thinking,
            display_name: "Qwen3.8 Max Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen38OmniFlash {
            model_name: "qwen/qwen3.8-omni-flash",
            constructor_name: qwen_qwen3_8_omni_flash,
            display_name: "Qwen3.8 Omni Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3Vl235bA22bInstructOriginal {
            model_name: "qwen3-vl-235b-a22b-instruct-original",
            constructor_name: qwen3_vl_235b_a22b_instruct_original,
            display_name: "Qwen3 VL 235B A22B Instruct Original",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ReflectionBeam10 {
            model_name: "reflection/beam-1.0",
            constructor_name: reflection_beam_1_0,
            display_name: "Beam 1.0 501B A23B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RekaRekaEdge2603 {
            model_name: "reka/reka-edge-2603",
            constructor_name: reka_reka_edge_2603,
            display_name: "Reka Edge 2603",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RekaRekaFlash3 {
            model_name: "reka/reka-flash-3",
            constructor_name: reka_reka_flash_3,
            display_name: "Reka Flash 3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        RicursiveaiRipple65bPreview {
            model_name: "ricursiveai/ripple-65b-preview",
            constructor_name: ricursiveai_ripple_65b_preview,
            display_name: "Ripple 65B Preview",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SakanaFuguMax {
            model_name: "sakana/fugu-max",
            constructor_name: sakana_fugu_max,
            display_name: "Fugu Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SakanaFuguUltra {
            model_name: "sakana/fugu-ultra",
            constructor_name: sakana_fugu_ultra,
            display_name: "Fugu Ultra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SakanaFuguUltraV11 {
            model_name: "sakana/fugu-ultra-v1.1",
            constructor_name: sakana_fugu_ultra_v1_1,
            display_name: "Fugu Ultra v1.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Sarvam105b {
            model_name: "sarvam-105b",
            constructor_name: sarvam_105b,
            display_name: "Sarvam 105B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ShisaAiShisaV2Llama3370b {
            model_name: "shisa-ai/shisa-v2-llama3.3-70b",
            constructor_name: shisa_ai_shisa_v2_llama3_3_70b,
            display_name: "Shisa V2 Llama 3.3 70B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ShisaAiShisaV21Llama3370b {
            model_name: "shisa-ai/shisa-v2.1-llama3.3-70b",
            constructor_name: shisa_ai_shisa_v2_1_llama3_3_70b,
            display_name: "Shisa V2.1 Llama 3.3 70B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SicariussicariistuffAssistantPepe32b {
            model_name: "sicariussicariistuff/assistant-pepe-32b",
            constructor_name: sicariussicariistuff_assistant_pepe_32b,
            display_name: "Assistant Pepe 32B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Soob3123GraylineQwen38b {
            model_name: "soob3123/GrayLine-Qwen3-8B",
            constructor_name: soob3123_grayline_qwen3_8b,
            display_name: "Grayline Qwen3 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Soob3123VeiledCalla12b {
            model_name: "soob3123/Veiled-Calla-12B",
            constructor_name: soob3123_veiled_calla_12b,
            display_name: "Veiled Calla 12B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Soob3123AmoralGemma327bV2 {
            model_name: "soob3123/amoral-gemma3-27B-v2",
            constructor_name: soob3123_amoral_gemma3_27b_v2,
            display_name: "Amoral Gemma3 27B v2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SpeakleashBielik11bV30Instruct {
            model_name: "speakleash/bielik-11b-v3.0-instruct",
            constructor_name: speakleash_bielik_11b_v3_0_instruct,
            display_name: "Bielik 11B v3.0",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        StepfunAiStep35Flash {
            model_name: "stepfun-ai/step-3.5-flash",
            constructor_name: stepfun_ai_step_3_5_flash,
            display_name: "Step 3.5 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        StepfunAiStep35Flash2603 {
            model_name: "stepfun-ai/step-3.5-flash-2603",
            constructor_name: stepfun_ai_step_3_5_flash_2603,
            display_name: "Step 3.5 Flash 2603",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        StepfunStep37FlashThinking {
            model_name: "stepfun/step-3.7-flash:thinking",
            constructor_name: stepfun_step_3_7_flash_thinking,
            display_name: "Step 3.7 Flash Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        StepfunStep5Preview {
            model_name: "stepfun/step-5-preview",
            constructor_name: stepfun_step_5_preview,
            display_name: "Step 5 Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Synth25Flash {
            model_name: "synth-2.5-flash",
            constructor_name: synth_2_5_flash,
            display_name: "Synth 2.5 Flash Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Synth25Pro {
            model_name: "synth-2.5-pro",
            constructor_name: synth_2_5_pro,
            display_name: "Synth 2.5 Pro Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TencentHy3 {
            model_name: "tencent/hy3",
            constructor_name: tencent_hy3,
            display_name: "Tencent Hy3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHy4 {
            model_name: "tencent/hy4",
            constructor_name: tencent_hy4,
            display_name: "Tencent Hy4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHy4Preview {
            model_name: "tencent/hy4-preview",
            constructor_name: tencent_hy4_preview,
            display_name: "Tencent Hy4 Preview",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInklingSmall {
            model_name: "thinkingmachines/Inkling-Small",
            constructor_name: thinkingmachines_inkling_small,
            display_name: "Inkling Small",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInklingSmallThinking {
            model_name: "thinkingmachines/Inkling-Small:thinking",
            constructor_name: thinkingmachines_inkling_small_thinking,
            display_name: "Inkling Small Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInkling {
            model_name: "thinkingmachines/inkling",
            constructor_name: thinkingmachines_inkling,
            display_name: "Inkling",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInklingThinking {
            model_name: "thinkingmachines/inkling:thinking",
            constructor_name: thinkingmachines_inkling_thinking,
            display_name: "Inkling Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UnbiasedPareto2610 {
            model_name: "unbiased/pareto-26.10",
            constructor_name: unbiased_pareto_26_10,
            display_name: "Pareto 26.10",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UnbiasedPareto269 {
            model_name: "unbiased/pareto-26.9",
            constructor_name: unbiased_pareto_26_9,
            display_name: "Pareto 26.9",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Undi95RemmSlerpL213b {
            model_name: "undi95/remm-slerp-l2-13b",
            constructor_name: undi95_remm_slerp_l2_13b,
            display_name: "ReMM SLERP 13B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        UniversalSummarizer {
            model_name: "universal-summarizer",
            constructor_name: universal_summarizer,
            display_name: "Universal Summarizer",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        UnslothGemma312bIt {
            model_name: "unsloth/gemma-3-12b-it",
            constructor_name: unsloth_gemma_3_12b_it,
            display_name: "Gemma 3 12B IT",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        UnslothGemma327bIt {
            model_name: "unsloth/gemma-3-27b-it",
            constructor_name: unsloth_gemma_3_27b_it,
            display_name: "Gemma 3 27B IT",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        UnslothGemma34bIt {
            model_name: "unsloth/gemma-3-4b-it",
            constructor_name: unsloth_gemma_3_4b_it,
            display_name: "Gemma 3 4B IT",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        UpstageSolarMini4 {
            model_name: "upstage/solar-mini4",
            constructor_name: upstage_solar_mini4,
            display_name: "Solar Mini 4",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UpstageSolarMini4Thinking {
            model_name: "upstage/solar-mini4:thinking",
            constructor_name: upstage_solar_mini4_thinking,
            display_name: "Solar Mini 4 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UpstageSolarPro3 {
            model_name: "upstage/solar-pro-3",
            constructor_name: upstage_solar_pro_3,
            display_name: "Solar Pro 3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        UpstageSolarPro4 {
            model_name: "upstage/solar-pro4",
            constructor_name: upstage_solar_pro4,
            display_name: "Solar Pro 4",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UpstageSolarPro4Thinking {
            model_name: "upstage/solar-pro4:thinking",
            constructor_name: upstage_solar_pro4_thinking,
            display_name: "Solar Pro 4 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        VeniceUncensored {
            model_name: "venice-uncensored",
            constructor_name: venice_uncensored,
            display_name: "Venice Uncensored",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        XAiGrok420 {
            model_name: "x-ai/grok-4.20",
            constructor_name: x_ai_grok_4_20,
            display_name: "Grok 4.20",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok420MultiAgent {
            model_name: "x-ai/grok-4.20-multi-agent",
            constructor_name: x_ai_grok_4_20_multi_agent,
            display_name: "Grok 4.20 Multi-Agent",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok43 {
            model_name: "x-ai/grok-4.3",
            constructor_name: x_ai_grok_4_3,
            display_name: "Grok 4.3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok45 {
            model_name: "x-ai/grok-4.5",
            constructor_name: x_ai_grok_4_5,
            display_name: "Grok 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok46 {
            model_name: "x-ai/grok-4.6",
            constructor_name: x_ai_grok_4_6,
            display_name: "Grok 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok47 {
            model_name: "x-ai/grok-4.7",
            constructor_name: x_ai_grok_4_7,
            display_name: "Grok 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrokBuild01 {
            model_name: "x-ai/grok-build-0.1",
            constructor_name: x_ai_grok_build_0_1,
            display_name: "Grok Build 0.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrokLatest {
            model_name: "x-ai/grok-latest",
            constructor_name: x_ai_grok_latest,
            display_name: "Grok Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV25 {
            model_name: "xiaomi/mimo-v2.5",
            constructor_name: xiaomi_mimo_v2_5,
            display_name: "MiMo V2.5",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XiaomiMimoV25Pro {
            model_name: "xiaomi/mimo-v2.5-pro",
            constructor_name: xiaomi_mimo_v2_5_pro,
            display_name: "MiMo V2.5 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV25ProThinking {
            model_name: "xiaomi/mimo-v2.5-pro:thinking",
            constructor_name: xiaomi_mimo_v2_5_pro_thinking,
            display_name: "MiMo V2.5 Pro Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV25Thinking {
            model_name: "xiaomi/mimo-v2.5:thinking",
            constructor_name: xiaomi_mimo_v2_5_thinking,
            display_name: "MiMo V2.5 Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XiaomiMimoV26Flash {
            model_name: "xiaomi/mimo-v2.6-flash",
            constructor_name: xiaomi_mimo_v2_6_flash,
            display_name: "MiMo V2.6 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XiaomiMimoV26FlashAbliterated {
            model_name: "xiaomi/mimo-v2.6-flash-abliterated",
            constructor_name: xiaomi_mimo_v2_6_flash_abliterated,
            display_name: "MiMo V2.6 Flash Abliterated",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV26FlashUncensored {
            model_name: "xiaomi/mimo-v2.6-flash-uncensored",
            constructor_name: xiaomi_mimo_v2_6_flash_uncensored,
            display_name: "MiMo V2.6 Flash Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV26FlashUncensoredThinking {
            model_name: "xiaomi/mimo-v2.6-flash-uncensored:thinking",
            constructor_name: xiaomi_mimo_v2_6_flash_uncensored_thinking,
            display_name: "MiMo V2.6 Flash Uncensored Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV26Pro {
            model_name: "xiaomi/mimo-v2.6-pro",
            constructor_name: xiaomi_mimo_v2_6_pro,
            display_name: "MiMo V2.6 Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XiaomiMimoV26ProUltraspeed {
            model_name: "xiaomi/mimo-v2.6-pro-ultraspeed",
            constructor_name: xiaomi_mimo_v2_6_pro_ultraspeed,
            display_name: "MiMo V2.6 Pro UltraSpeed",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XingchenAgiXing4029bA4b {
            model_name: "xingchen-agi/xing4.0-29b-a4b",
            constructor_name: xingchen_agi_xing4_0_29b_a4b,
            display_name: "Xing 4.0 29B A4B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XtoolsAtDeepseekV4ProQwen359bUncensoredHeretic {
            model_name: "xtools-at/deepseek-v4-pro-qwen3.5-9b-uncensored-heretic",
            constructor_name: xtools_at_deepseek_v4_pro_qwen3_5_9b_uncensored_heretic,
            display_name: "DeepSeek V4 Pro Qwen3.5 9B Uncensored Heretic",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ZAiGlm45Air {
            model_name: "z-ai/GLM-4.5-Air",
            constructor_name: z_ai_glm_4_5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm45AirThinking {
            model_name: "z-ai/GLM-4.5-Air:thinking",
            constructor_name: z_ai_glm_4_5_air_thinking,
            display_name: "GLM 4.5 Air (Thinking)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm45Thinking {
            model_name: "z-ai/GLM-4.5:thinking",
            constructor_name: z_ai_glm_4_5_thinking,
            display_name: "GLM 4.5 (Thinking)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZAiGlm46Turbo {
            model_name: "z-ai/GLM-4.6-turbo",
            constructor_name: z_ai_glm_4_6_turbo,
            display_name: "GLM 4.6 Turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ZAiGlm46TurboThinking {
            model_name: "z-ai/GLM-4.6-turbo:thinking",
            constructor_name: z_ai_glm_4_6_turbo_thinking,
            display_name: "GLM 4.6 Turbo (Thinking)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZAiGlm45 {
            model_name: "z-ai/glm-4.5",
            constructor_name: z_ai_glm_4_5,
            display_name: "GLM 4.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ZAiGlm45v {
            model_name: "z-ai/glm-4.5v",
            constructor_name: z_ai_glm_4_5v,
            display_name: "GLM 4.5V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZAiGlm45vThinking {
            model_name: "z-ai/glm-4.5v:thinking",
            constructor_name: z_ai_glm_4_5v_thinking,
            display_name: "GLM 4.5V Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZAiGlm46 {
            model_name: "z-ai/glm-4.6",
            constructor_name: z_ai_glm_4_6,
            display_name: "GLM 4.6",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm46Original {
            model_name: "z-ai/glm-4.6-original",
            constructor_name: z_ai_glm_4_6_original,
            display_name: "GLM 4.6 Original",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm46Thinking {
            model_name: "z-ai/glm-4.6:thinking",
            constructor_name: z_ai_glm_4_6_thinking,
            display_name: "GLM 4.6 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm46v {
            model_name: "z-ai/glm-4.6v",
            constructor_name: z_ai_glm_4_6v,
            display_name: "GLM 4.6V",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        ZAiGlm46vOriginal {
            model_name: "z-ai/glm-4.6v-original",
            constructor_name: z_ai_glm_4_6v_original,
            display_name: "GLM 4.6V Original",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ZAiGlm47 {
            model_name: "z-ai/glm-4.7",
            constructor_name: z_ai_glm_4_7,
            display_name: "GLM 4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47Flash {
            model_name: "z-ai/glm-4.7-flash",
            constructor_name: z_ai_glm_4_7_flash,
            display_name: "GLM 4.7 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47FlashThinking {
            model_name: "z-ai/glm-4.7-flash:thinking",
            constructor_name: z_ai_glm_4_7_flash_thinking,
            display_name: "GLM 4.7 Flash Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47Original {
            model_name: "z-ai/glm-4.7-original",
            constructor_name: z_ai_glm_4_7_original,
            display_name: "GLM 4.7 Original",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47OriginalThinking {
            model_name: "z-ai/glm-4.7-original:thinking",
            constructor_name: z_ai_glm_4_7_original_thinking,
            display_name: "GLM 4.7 Original Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47Thinking {
            model_name: "z-ai/glm-4.7:thinking",
            constructor_name: z_ai_glm_4_7_thinking,
            display_name: "GLM 4.7 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5 {
            model_name: "z-ai/glm-5",
            constructor_name: z_ai_glm_5,
            display_name: "GLM 5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5Original {
            model_name: "z-ai/glm-5-original",
            constructor_name: z_ai_glm_5_original,
            display_name: "GLM 5 Original",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5OriginalThinking {
            model_name: "z-ai/glm-5-original:thinking",
            constructor_name: z_ai_glm_5_original_thinking,
            display_name: "GLM 5 Original Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5Turbo {
            model_name: "z-ai/glm-5-turbo",
            constructor_name: z_ai_glm_5_turbo,
            display_name: "GLM 5 Turbo",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm51 {
            model_name: "z-ai/glm-5.1",
            constructor_name: z_ai_glm_5_1,
            display_name: "GLM 5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm51Thinking {
            model_name: "z-ai/glm-5.1:thinking",
            constructor_name: z_ai_glm_5_1_thinking,
            display_name: "GLM 5.1 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm52 {
            model_name: "z-ai/glm-5.2",
            constructor_name: z_ai_glm_5_2,
            display_name: "GLM 5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm52Thinking {
            model_name: "z-ai/glm-5.2:thinking",
            constructor_name: z_ai_glm_5_2_thinking,
            display_name: "GLM 5.2 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53 {
            model_name: "z-ai/glm-5.3",
            constructor_name: z_ai_glm_5_3,
            display_name: "GLM 5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53Flash {
            model_name: "z-ai/glm-5.3-flash",
            constructor_name: z_ai_glm_5_3_flash,
            display_name: "GLM 5.3 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZAiGlm53FlashApex {
            model_name: "z-ai/glm-5.3-flash-apex",
            constructor_name: z_ai_glm_5_3_flash_apex,
            display_name: "Apex Flash 1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53FlashApexAbliterated {
            model_name: "z-ai/glm-5.3-flash-apex-abliterated",
            constructor_name: z_ai_glm_5_3_flash_apex_abliterated,
            display_name: "Apex Flash 1 Abliterated",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53FlashCybersecurity {
            model_name: "z-ai/glm-5.3-flash-cybersecurity",
            constructor_name: z_ai_glm_5_3_flash_cybersecurity,
            display_name: "GLM 5.3 Flash Cybersecurity",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53FlashUncensored {
            model_name: "z-ai/glm-5.3-flash-uncensored",
            constructor_name: z_ai_glm_5_3_flash_uncensored,
            display_name: "GLM 5.3 Flash Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53FlashxLegal {
            model_name: "z-ai/glm-5.3-flashx-legal",
            constructor_name: z_ai_glm_5_3_flashx_legal,
            display_name: "GLM 5.3 FlashX Legal",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53Uncensored {
            model_name: "z-ai/glm-5.3-uncensored",
            constructor_name: z_ai_glm_5_3_uncensored,
            display_name: "GLM 5.3 Uncensored",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53Thinking {
            model_name: "z-ai/glm-5.3:thinking",
            constructor_name: z_ai_glm_5_3_thinking,
            display_name: "GLM 5.3 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5Thinking {
            model_name: "z-ai/glm-5:thinking",
            constructor_name: z_ai_glm_5_thinking,
            display_name: "GLM 5 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5vTurbo {
            model_name: "z-ai/glm-5v-turbo",
            constructor_name: z_ai_glm_5v_turbo,
            display_name: "GLM 5V Turbo",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZAiGlm5vTurboThinking {
            model_name: "z-ai/glm-5v-turbo:thinking",
            constructor_name: z_ai_glm_5v_turbo_thinking,
            display_name: "GLM 5V Turbo Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZAiGlmLatest {
            model_name: "z-ai/glm-latest",
            constructor_name: z_ai_glm_latest,
            display_name: "GLM Latest",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45Air {
            model_name: "zai-org/glm-4.5-air",
            constructor_name: zai_org_glm_4_5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45AirThinking {
            model_name: "zai-org/glm-4.5-air:thinking",
            constructor_name: zai_org_glm_4_5_air_thinking,
            display_name: "GLM 4.5 Air Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46 {
            model_name: "zai-org/glm-4.6",
            constructor_name: zai_org_glm_4_6,
            display_name: "GLM 4.6",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46Thinking {
            model_name: "zai-org/glm-4.6:thinking",
            constructor_name: zai_org_glm_4_6_thinking,
            display_name: "GLM 4.6 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47 {
            model_name: "zai-org/glm-4.7",
            constructor_name: zai_org_glm_4_7,
            display_name: "GLM 4.7",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Thinking {
            model_name: "zai-org/glm-4.7:thinking",
            constructor_name: zai_org_glm_4_7_thinking,
            display_name: "GLM 4.7 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5 {
            model_name: "zai-org/glm-5",
            constructor_name: zai_org_glm_5,
            display_name: "GLM 5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5Original {
            model_name: "zai-org/glm-5-original",
            constructor_name: zai_org_glm_5_original,
            display_name: "GLM 5 Original",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5OriginalThinking {
            model_name: "zai-org/glm-5-original:thinking",
            constructor_name: zai_org_glm_5_original_thinking,
            display_name: "GLM 5 Original Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5Thinking {
            model_name: "zai-org/glm-5:thinking",
            constructor_name: zai_org_glm_5_thinking,
            display_name: "GLM 5 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
