//! Capabilities for nvidia models.
//!
//! This module defines model types and their capabilities for nvidia providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::nvidia::Nvidia;

model_capabilities! {
    provider: Nvidia,
    models: {
        BlackForestLabsFlux1Dev {
            model_name: "black-forest-labs/flux.1-dev",
            constructor_name: black_forest_labs_flux_1_dev,
            display_name: "FLUX.1-dev",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BlackForestLabsFlux1KontextDev {
            model_name: "black-forest-labs/flux_1-kontext-dev",
            constructor_name: black_forest_labs_flux_1_kontext_dev,
            display_name: "FLUX.1-Kontext-dev",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BlackForestLabsFlux1Schnell {
            model_name: "black-forest-labs/flux_1-schnell",
            constructor_name: black_forest_labs_flux_1_schnell,
            display_name: "FLUX.1-schnell",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BlackForestLabsFlux2Klein4b {
            model_name: "black-forest-labs/flux_2-klein-4b",
            constructor_name: black_forest_labs_flux_2_klein_4b,
            display_name: "FLUX.2 Klein 4B",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        DeepseekAiDeepseekCoder67bInstruct {
            model_name: "deepseek-ai/deepseek-coder-6.7b-instruct",
            constructor_name: deepseek_ai_deepseek_coder_6_7b_instruct,
            display_name: "Deepseek Coder 6.7b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR1 {
            model_name: "deepseek-ai/deepseek-r1",
            constructor_name: deepseek_ai_deepseek_r1,
            display_name: "Deepseek R1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekR10528 {
            model_name: "deepseek-ai/deepseek-r1-0528",
            constructor_name: deepseek_ai_deepseek_r1_0528,
            display_name: "Deepseek R1 0528",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31 {
            model_name: "deepseek-ai/deepseek-v3.1",
            constructor_name: deepseek_ai_deepseek_v3_1,
            display_name: "DeepSeek V3.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31Terminus {
            model_name: "deepseek-ai/deepseek-v3.1-terminus",
            constructor_name: deepseek_ai_deepseek_v3_1_terminus,
            display_name: "DeepSeek V3.1 Terminus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV32 {
            model_name: "deepseek-ai/deepseek-v3.2",
            constructor_name: deepseek_ai_deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV41Flash {
            model_name: "deepseek-ai/deepseek-v4.1-flash",
            constructor_name: deepseek_ai_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleCodegemma117b {
            model_name: "google/codegemma-1.1-7b",
            constructor_name: google_codegemma_1_1_7b,
            display_name: "Codegemma 1.1 7b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleCodegemma7b {
            model_name: "google/codegemma-7b",
            constructor_name: google_codegemma_7b,
            display_name: "Codegemma 7b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleDiffusiongemma26bA4bIt {
            model_name: "google/diffusiongemma-26b-a4b-it",
            constructor_name: google_diffusiongemma_26b_a4b_it,
            display_name: "DiffusionGemma 26B A4B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemma227bIt {
            model_name: "google/gemma-2-27b-it",
            constructor_name: google_gemma_2_27b_it,
            display_name: "Gemma 2 27b It",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma22bIt {
            model_name: "google/gemma-2-2b-it",
            constructor_name: google_gemma_2_2b_it,
            display_name: "Gemma 2 2b It",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma312bIt {
            model_name: "google/gemma-3-12b-it",
            constructor_name: google_gemma_3_12b_it,
            display_name: "Gemma 3 12b It",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma31bIt {
            model_name: "google/gemma-3-1b-it",
            constructor_name: google_gemma_3_1b_it,
            display_name: "Gemma 3 1b It",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma327bIt {
            model_name: "google/gemma-3-27b-it",
            constructor_name: google_gemma_3_27b_it,
            display_name: "Gemma-3-27B-IT",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma3nE2bIt {
            model_name: "google/gemma-3n-e2b-it",
            constructor_name: google_gemma_3n_e2b_it,
            display_name: "Gemma 3n E2b It",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma3nE4bIt {
            model_name: "google/gemma-3n-e4b-it",
            constructor_name: google_gemma_3n_e4b_it,
            display_name: "Gemma 3n E4b It",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31b-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma-4-31B-IT",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGooglePaligemma {
            model_name: "google/google-paligemma",
            constructor_name: google_google_paligemma,
            display_name: "paligemma",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaCodellama70b {
            model_name: "meta/codellama-70b",
            constructor_name: meta_codellama_70b,
            display_name: "Codellama 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama31405bInstruct {
            model_name: "meta/llama-3.1-405b-instruct",
            constructor_name: meta_llama_3_1_405b_instruct,
            display_name: "Llama 3.1 405b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3170bInstruct {
            model_name: "meta/llama-3.1-70b-instruct",
            constructor_name: meta_llama_3_1_70b_instruct,
            display_name: "Llama 3.1 70b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3211bVisionInstruct {
            model_name: "meta/llama-3.2-11b-vision-instruct",
            constructor_name: meta_llama_3_2_11b_vision_instruct,
            display_name: "Llama 3.2 11b Vision Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama321bInstruct {
            model_name: "meta/llama-3.2-1b-instruct",
            constructor_name: meta_llama_3_2_1b_instruct,
            display_name: "Llama 3.2 1b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3290bVisionInstruct {
            model_name: "meta/llama-3.2-90b-vision-instruct",
            constructor_name: meta_llama_3_2_90b_vision_instruct,
            display_name: "Llama-3.2-90B-Vision-Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3370bInstruct {
            model_name: "meta/llama-3.3-70b-instruct",
            constructor_name: meta_llama_3_3_70b_instruct,
            display_name: "Llama 3.3 70b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama4Maverick17b128eInstruct {
            model_name: "meta/llama-4-maverick-17b-128e-instruct",
            constructor_name: meta_llama_4_maverick_17b_128e_instruct,
            display_name: "Llama 4 Maverick 17b 128e Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama4Scout17b16eInstruct {
            model_name: "meta/llama-4-scout-17b-16e-instruct",
            constructor_name: meta_llama_4_scout_17b_16e_instruct,
            display_name: "Llama 4 Scout 17b 16e Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaGuard412b {
            model_name: "meta/llama-guard-4-12b",
            constructor_name: meta_llama_guard_4_12b,
            display_name: "Llama Guard 4 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlama370bInstruct {
            model_name: "meta/llama3-70b-instruct",
            constructor_name: meta_llama3_70b_instruct,
            display_name: "Llama3 70b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama38bInstruct {
            model_name: "meta/llama3-8b-instruct",
            constructor_name: meta_llama3_8b_instruct,
            display_name: "Llama3 8b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseGlimmer30b {
            model_name: "meta/muse-glimmer-30b",
            constructor_name: meta_muse_glimmer_30b,
            display_name: "Muse Glimmer 30B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftPhi3Medium128kInstruct {
            model_name: "microsoft/phi-3-medium-128k-instruct",
            constructor_name: microsoft_phi_3_medium_128k_instruct,
            display_name: "Phi 3 Medium 128k Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftPhi3Medium4kInstruct {
            model_name: "microsoft/phi-3-medium-4k-instruct",
            constructor_name: microsoft_phi_3_medium_4k_instruct,
            display_name: "Phi 3 Medium 4k Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftPhi3Small128kInstruct {
            model_name: "microsoft/phi-3-small-128k-instruct",
            constructor_name: microsoft_phi_3_small_128k_instruct,
            display_name: "Phi 3 Small 128k Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftPhi3Small8kInstruct {
            model_name: "microsoft/phi-3-small-8k-instruct",
            constructor_name: microsoft_phi_3_small_8k_instruct,
            display_name: "Phi 3 Small 8k Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftPhi3Vision128kInstruct {
            model_name: "microsoft/phi-3-vision-128k-instruct",
            constructor_name: microsoft_phi_3_vision_128k_instruct,
            display_name: "Phi 3 Vision 128k Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftPhi35MoeInstruct {
            model_name: "microsoft/phi-3.5-moe-instruct",
            constructor_name: microsoft_phi_3_5_moe_instruct,
            display_name: "Phi 3.5 Moe Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftPhi35VisionInstruct {
            model_name: "microsoft/phi-3.5-vision-instruct",
            constructor_name: microsoft_phi_3_5_vision_instruct,
            display_name: "Phi 3.5 Vision Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftPhi4MiniInstruct {
            model_name: "microsoft/phi-4-mini-instruct",
            constructor_name: microsoft_phi_4_mini_instruct,
            display_name: "Phi-4-Mini",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxaiMinimaxM2 {
            model_name: "minimaxai/minimax-m2",
            constructor_name: minimaxai_minimax_m2,
            display_name: "MiniMax-M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxaiMinimaxM21 {
            model_name: "minimaxai/minimax-m2.1",
            constructor_name: minimaxai_minimax_m2_1,
            display_name: "MiniMax-M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxaiMinimaxM31Flash {
            model_name: "minimaxai/minimax-m3.1-flash",
            constructor_name: minimaxai_minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MistralaiCodestral22bInstructV01 {
            model_name: "mistralai/codestral-22b-instruct-v0.1",
            constructor_name: mistralai_codestral_22b_instruct_v0_1,
            display_name: "Codestral 22b Instruct V0.1",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstral2123bInstruct2512 {
            model_name: "mistralai/devstral-2-123b-instruct-2512",
            constructor_name: mistralai_devstral_2_123b_instruct_2512,
            display_name: "Devstral-2-123B-Instruct-2512",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMambaCodestral7bV01 {
            model_name: "mistralai/mamba-codestral-7b-v0.1",
            constructor_name: mistralai_mamba_codestral_7b_v0_1,
            display_name: "Mamba Codestral 7b V0.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMinistral14bInstruct2512 {
            model_name: "mistralai/ministral-14b-instruct-2512",
            constructor_name: mistralai_ministral_14b_instruct_2512,
            display_name: "Ministral 3 14B Instruct 2512",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralLarge2Instruct {
            model_name: "mistralai/mistral-large-2-instruct",
            constructor_name: mistralai_mistral_large_2_instruct,
            display_name: "Mistral Large 2 Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralLarge3675bInstruct2512 {
            model_name: "mistralai/mistral-large-3-675b-instruct-2512",
            constructor_name: mistralai_mistral_large_3_675b_instruct_2512,
            display_name: "Mistral Large 3 675B Instruct 2512",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall3124bInstruct2503 {
            model_name: "mistralai/mistral-small-3.1-24b-instruct-2503",
            constructor_name: mistralai_mistral_small_3_1_24b_instruct_2503,
            display_name: "Mistral Small 3.1 24b Instruct 2503",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct {
            model_name: "moonshotai/kimi-k2-instruct",
            constructor_name: moonshotai_kimi_k2_instruct,
            display_name: "Kimi K2 Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct0905 {
            model_name: "moonshotai/kimi-k2-instruct-0905",
            constructor_name: moonshotai_kimi_k2_instruct_0905,
            display_name: "Kimi K2 0905",
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
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaActiveSpeakerDetection {
            model_name: "nvidia/active-speaker-detection",
            constructor_name: nvidia_active_speaker_detection,
            display_name: "Active Speaker Detection",
            capabilities: [ImageInputSupport, TextOutputSupport, VideoInputSupport]
        },
        NvidiaBevformer {
            model_name: "nvidia/bevformer",
            constructor_name: nvidia_bevformer,
            display_name: "bevformer",
            capabilities: [ImageInputSupport, TextOutputSupport, VideoInputSupport]
        },
        NvidiaCosmosNemotron34b {
            model_name: "nvidia/cosmos-nemotron-34b",
            constructor_name: nvidia_cosmos_nemotron_34b,
            display_name: "Cosmos Nemotron 34B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        NvidiaCosmosTransfer252b {
            model_name: "nvidia/cosmos-transfer2_5-2b",
            constructor_name: nvidia_cosmos_transfer2_5_2b,
            display_name: "cosmos-transfer2.5-2b",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        NvidiaLlama31Nemotron51bInstruct {
            model_name: "nvidia/llama-3.1-nemotron-51b-instruct",
            constructor_name: nvidia_llama_3_1_nemotron_51b_instruct,
            display_name: "Llama 3.1 Nemotron 51b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaLlama31Nemotron70bInstruct {
            model_name: "nvidia/llama-3.1-nemotron-70b-instruct",
            constructor_name: nvidia_llama_3_1_nemotron_70b_instruct,
            display_name: "Llama 3.1 Nemotron 70b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaLlama31NemotronSafetyGuard8bV3 {
            model_name: "nvidia/llama-3.1-nemotron-safety-guard-8b-v3",
            constructor_name: nvidia_llama_3_1_nemotron_safety_guard_8b_v3,
            display_name: "llama-3.1-nemotron-safety-guard-8b-v3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama31NemotronUltra253bV1 {
            model_name: "nvidia/llama-3.1-nemotron-ultra-253b-v1",
            constructor_name: nvidia_llama_3_1_nemotron_ultra_253b_v1,
            display_name: "Llama-3.1-Nemotron-Ultra-253B-v1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaLlama33NemotronSuper49bV1 {
            model_name: "nvidia/llama-3.3-nemotron-super-49b-v1",
            constructor_name: nvidia_llama_3_3_nemotron_super_49b_v1,
            display_name: "Llama 3.3 Nemotron Super 49b V1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama33NemotronSuper49bV15 {
            model_name: "nvidia/llama-3.3-nemotron-super-49b-v1.5",
            constructor_name: nvidia_llama_3_3_nemotron_super_49b_v1_5,
            display_name: "Llama 3.3 Nemotron Super 49b V1.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlamaEmbedNemotron8b {
            model_name: "nvidia/llama-embed-nemotron-8b",
            constructor_name: nvidia_llama_embed_nemotron_8b,
            display_name: "Llama Embed Nemotron 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlamaNemotronEmbedVl1bV2 {
            model_name: "nvidia/llama-nemotron-embed-vl-1b-v2",
            constructor_name: nvidia_llama_nemotron_embed_vl_1b_v2,
            display_name: "llama-nemotron-embed-vl-1b-v2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaLlamaNemotronRerankVl1bV2 {
            model_name: "nvidia/llama-nemotron-rerank-vl-1b-v2",
            constructor_name: nvidia_llama_nemotron_rerank_vl_1b_v2,
            display_name: "llama-nemotron-rerank-vl-1b-v2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama3Chatqa1570b {
            model_name: "nvidia/llama3-chatqa-1.5-70b",
            constructor_name: nvidia_llama3_chatqa_1_5_70b,
            display_name: "Llama3 Chatqa 1.5 70b",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaMagpieTtsZeroshot {
            model_name: "nvidia/magpie-tts-zeroshot",
            constructor_name: nvidia_magpie_tts_zeroshot,
            display_name: "magpie-tts-zeroshot",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport]
        },
        NvidiaNemoretrieverOcrV1 {
            model_name: "nvidia/nemoretriever-ocr-v1",
            constructor_name: nvidia_nemoretriever_ocr_v1,
            display_name: "NeMo Retriever OCR v1",
            capabilities: [ImageInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Nano30bA3b {
            model_name: "nvidia/nemotron-3-nano-30b-a3b",
            constructor_name: nvidia_nemotron_3_nano_30b_a3b,
            display_name: "nemotron-3-nano-30b-a3b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3NanoOmni30bA3bReasoning {
            model_name: "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning",
            constructor_name: nvidia_nemotron_3_nano_omni_30b_a3b_reasoning,
            display_name: "Nemotron 3 Nano Omni",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaNemotron3Ultra550bA55b {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b,
            display_name: "Nemotron 3 Ultra 550B A55B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35ContentSafety {
            model_name: "nvidia/nemotron-3.5-content-safety",
            constructor_name: nvidia_nemotron_3_5_content_safety,
            display_name: "Nemotron 3.5 Content Safety",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron35Lightning30bA3b {
            model_name: "nvidia/nemotron-3.5-lightning-30b-a3b",
            constructor_name: nvidia_nemotron_3_5_lightning_30b_a3b,
            display_name: "Nemotron 3.5 Lightning 30B A3B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35SuperVl120bA12b {
            model_name: "nvidia/nemotron-3.5-super-vl-120b-a12b",
            constructor_name: nvidia_nemotron_3_5_super_vl_120b_a12b,
            display_name: "Nemotron 3.5 Super VL 120B A12B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaNemotron4340bInstruct {
            model_name: "nvidia/nemotron-4-340b-instruct",
            constructor_name: nvidia_nemotron_4_340b_instruct,
            display_name: "Nemotron 4 340b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronVoicechat {
            model_name: "nvidia/nemotron-voicechat",
            constructor_name: nvidia_nemotron_voicechat,
            display_name: "nemotron-voicechat",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNvidiaNemotronNano9bV2 {
            model_name: "nvidia/nvidia-nemotron-nano-9b-v2",
            constructor_name: nvidia_nvidia_nemotron_nano_9b_v2,
            display_name: "nvidia-nemotron-nano-9b-v2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaParakeetTdt06bV2 {
            model_name: "nvidia/parakeet-tdt-0.6b-v2",
            constructor_name: nvidia_parakeet_tdt_0_6b_v2,
            display_name: "Parakeet TDT 0.6B v2",
            capabilities: [AudioInputSupport, TextOutputSupport]
        },
        NvidiaSparsedrive {
            model_name: "nvidia/sparsedrive",
            constructor_name: nvidia_sparsedrive,
            display_name: "sparsedrive",
            capabilities: [ImageInputSupport, TextOutputSupport, VideoInputSupport]
        },
        NvidiaStreampetr {
            model_name: "nvidia/streampetr",
            constructor_name: nvidia_streampetr,
            display_name: "streampetr",
            capabilities: [ImageInputSupport, TextOutputSupport, VideoInputSupport]
        },
        NvidiaStudiovoice {
            model_name: "nvidia/studiovoice",
            constructor_name: nvidia_studiovoice,
            display_name: "studiovoice",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport]
        },
        NvidiaSyntheticVideoDetector {
            model_name: "nvidia/synthetic-video-detector",
            constructor_name: nvidia_synthetic_video_detector,
            display_name: "synthetic-video-detector",
            capabilities: [ImageInputSupport, TextOutputSupport, VideoInputSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT-OSS-120B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiWhisperLargeV3 {
            model_name: "openai/whisper-large-v3",
            constructor_name: openai_whisper_large_v3,
            display_name: "Whisper Large v3",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        PoolsideLagunaXs21 {
            model_name: "poolside/laguna-xs-2.1",
            constructor_name: poolside_laguna_xs_2_1,
            display_name: "Laguna XS 2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwenImage {
            model_name: "qwen/qwen-image",
            constructor_name: qwen_qwen_image,
            display_name: "Qwen Image",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        QwenQwenImageEdit {
            model_name: "qwen/qwen-image-edit",
            constructor_name: qwen_qwen_image_edit,
            display_name: "Qwen Image Edit",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        QwenQwen25Coder32bInstruct {
            model_name: "qwen/qwen2.5-coder-32b-instruct",
            constructor_name: qwen_qwen2_5_coder_32b_instruct,
            display_name: "Qwen2.5 Coder 32b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen25Coder7bInstruct {
            model_name: "qwen/qwen2.5-coder-7b-instruct",
            constructor_name: qwen_qwen2_5_coder_7b_instruct,
            display_name: "Qwen2.5 Coder 7b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22b {
            model_name: "qwen/qwen3-235b-a22b",
            constructor_name: qwen_qwen3_235b_a22b,
            display_name: "Qwen3-235B-A22B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder480bA35bInstruct {
            model_name: "qwen/qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen_qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bInstruct {
            model_name: "qwen/qwen3-next-80b-a3b-instruct",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3-Next-80B-A3B-Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bThinking {
            model_name: "qwen/qwen3-next-80b-a3b-thinking",
            constructor_name: qwen_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3-Next-80B-A3B-Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwq32b {
            model_name: "qwen/qwq-32b",
            constructor_name: qwen_qwq_32b,
            display_name: "Qwq 32b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        StepfunAiStep5Preview {
            model_name: "stepfun-ai/step-5-preview",
            constructor_name: stepfun_ai_step_5_preview,
            display_name: "Step 5 Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53Flash {
            model_name: "z-ai/glm-5.3-flash",
            constructor_name: z_ai_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47 {
            model_name: "z-ai/glm4.7",
            constructor_name: z_ai_glm4_7,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5 {
            model_name: "z-ai/glm5",
            constructor_name: z_ai_glm5,
            display_name: "GLM5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
