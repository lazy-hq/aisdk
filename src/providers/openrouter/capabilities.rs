//! Capabilities for openrouter models.
//!
//! This module defines model types and their capabilities for openrouter providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::openrouter::Openrouter;

model_capabilities! {
    provider: Openrouter,
    models: {
        AllenaiMolmo28bFree {
            model_name: "allenai/molmo-2-8b:free",
            constructor_name: allenai_molmo_2_8b_free,
            display_name: "Molmo2 8B (free)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AnthropicClaude35Haiku {
            model_name: "anthropic/claude-3.5-haiku",
            constructor_name: anthropic_claude_3_5_haiku,
            display_name: "Claude Haiku 3.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude37Sonnet {
            model_name: "anthropic/claude-3.7-sonnet",
            constructor_name: anthropic_claude_3_7_sonnet,
            display_name: "Claude Sonnet 3.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku45 {
            model_name: "anthropic/claude-haiku-4.5",
            constructor_name: anthropic_claude_haiku_4_5,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus4 {
            model_name: "anthropic/claude-opus-4",
            constructor_name: anthropic_claude_opus_4,
            display_name: "Claude Opus 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus41 {
            model_name: "anthropic/claude-opus-4.1",
            constructor_name: anthropic_claude_opus_4_1,
            display_name: "Claude Opus 4.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus45 {
            model_name: "anthropic/claude-opus-4.5",
            constructor_name: anthropic_claude_opus_4_5,
            display_name: "Claude Opus 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet4 {
            model_name: "anthropic/claude-sonnet-4",
            constructor_name: anthropic_claude_sonnet_4,
            display_name: "Claude Sonnet 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet45 {
            model_name: "anthropic/claude-sonnet-4.5",
            constructor_name: anthropic_claude_sonnet_4_5,
            display_name: "Claude Sonnet 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ArceeAiTrinityLargePreviewFree {
            model_name: "arcee-ai/trinity-large-preview:free",
            constructor_name: arcee_ai_trinity_large_preview_free,
            display_name: "Trinity Large Preview",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ArceeAiTrinityMiniFree {
            model_name: "arcee-ai/trinity-mini:free",
            constructor_name: arcee_ai_trinity_mini_free,
            display_name: "Trinity Mini",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BlackForestLabsFlux2Flex {
            model_name: "black-forest-labs/flux.2-flex",
            constructor_name: black_forest_labs_flux_2_flex,
            display_name: "FLUX.2 Flex",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        BlackForestLabsFlux2Klein4b {
            model_name: "black-forest-labs/flux.2-klein-4b",
            constructor_name: black_forest_labs_flux_2_klein_4b,
            display_name: "FLUX.2 Klein 4B",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        BlackForestLabsFlux2Max {
            model_name: "black-forest-labs/flux.2-max",
            constructor_name: black_forest_labs_flux_2_max,
            display_name: "FLUX.2 Max",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        BlackForestLabsFlux2Pro {
            model_name: "black-forest-labs/flux.2-pro",
            constructor_name: black_forest_labs_flux_2_pro,
            display_name: "FLUX.2 Pro",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        BytedanceSeedSeedream45 {
            model_name: "bytedance-seed/seedream-4.5",
            constructor_name: bytedance_seed_seedream_4_5,
            display_name: "Seedream 4.5",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        CognitivecomputationsDolphinMistral24bVeniceEditionFree {
            model_name: "cognitivecomputations/dolphin-mistral-24b-venice-edition:free",
            constructor_name: cognitivecomputations_dolphin_mistral_24b_venice_edition_free,
            display_name: "Uncensored (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CognitivecomputationsDolphin30Mistral24b {
            model_name: "cognitivecomputations/dolphin3.0-mistral-24b",
            constructor_name: cognitivecomputations_dolphin3_0_mistral_24b,
            display_name: "Dolphin3.0 Mistral 24B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CognitivecomputationsDolphin30R1Mistral24b {
            model_name: "cognitivecomputations/dolphin3.0-r1-mistral-24b",
            constructor_name: cognitivecomputations_dolphin3_0_r1_mistral_24b,
            display_name: "Dolphin3.0 R1 Mistral 24B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekChatV30324 {
            model_name: "deepseek/deepseek-chat-v3-0324",
            constructor_name: deepseek_deepseek_chat_v3_0324,
            display_name: "DeepSeek V3 0324",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekChatV31 {
            model_name: "deepseek/deepseek-chat-v3.1",
            constructor_name: deepseek_deepseek_chat_v3_1,
            display_name: "DeepSeek-V3.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekR10528Qwen38bFree {
            model_name: "deepseek/deepseek-r1-0528-qwen3-8b:free",
            constructor_name: deepseek_deepseek_r1_0528_qwen3_8b_free,
            display_name: "Deepseek R1 0528 Qwen3 8B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekR10528Free {
            model_name: "deepseek/deepseek-r1-0528:free",
            constructor_name: deepseek_deepseek_r1_0528_free,
            display_name: "R1 0528 (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekR1DistillLlama70b {
            model_name: "deepseek/deepseek-r1-distill-llama-70b",
            constructor_name: deepseek_deepseek_r1_distill_llama_70b,
            display_name: "DeepSeek R1 Distill Llama 70B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekR1DistillQwen14b {
            model_name: "deepseek/deepseek-r1-distill-qwen-14b",
            constructor_name: deepseek_deepseek_r1_distill_qwen_14b,
            display_name: "DeepSeek R1 Distill Qwen 14B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekR1Free {
            model_name: "deepseek/deepseek-r1:free",
            constructor_name: deepseek_deepseek_r1_free,
            display_name: "R1 (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV3BaseFree {
            model_name: "deepseek/deepseek-v3-base:free",
            constructor_name: deepseek_deepseek_v3_base_free,
            display_name: "DeepSeek V3 Base (free)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekV31Terminus {
            model_name: "deepseek/deepseek-v3.1-terminus",
            constructor_name: deepseek_deepseek_v3_1_terminus,
            display_name: "DeepSeek V3.1 Terminus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV31TerminusExacto {
            model_name: "deepseek/deepseek-v3.1-terminus:exacto",
            constructor_name: deepseek_deepseek_v3_1_terminus_exacto,
            display_name: "DeepSeek V3.1 Terminus (exacto)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32 {
            model_name: "deepseek/deepseek-v3.2",
            constructor_name: deepseek_deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32Speciale {
            model_name: "deepseek/deepseek-v3.2-speciale",
            constructor_name: deepseek_deepseek_v3_2_speciale,
            display_name: "DeepSeek V3.2 Speciale",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        FeatherlessQwerky72b {
            model_name: "featherless/qwerky-72b",
            constructor_name: featherless_qwerky_72b,
            display_name: "Qwerky 72B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleGemini20Flash001 {
            model_name: "google/gemini-2.0-flash-001",
            constructor_name: google_gemini_2_0_flash_001,
            display_name: "Gemini 2.0 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini20FlashExpFree {
            model_name: "google/gemini-2.0-flash-exp:free",
            constructor_name: google_gemini_2_0_flash_exp_free,
            display_name: "Gemini 2.0 Flash Experimental (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini25Flash {
            model_name: "google/gemini-2.5-flash",
            constructor_name: google_gemini_2_5_flash,
            display_name: "Gemini 2.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25FlashLite {
            model_name: "google/gemini-2.5-flash-lite",
            constructor_name: google_gemini_2_5_flash_lite,
            display_name: "Gemini 2.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25FlashLitePreview092025 {
            model_name: "google/gemini-2.5-flash-lite-preview-09-2025",
            constructor_name: google_gemini_2_5_flash_lite_preview_09_2025,
            display_name: "Gemini 2.5 Flash Lite Preview 09-25",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25FlashPreview092025 {
            model_name: "google/gemini-2.5-flash-preview-09-2025",
            constructor_name: google_gemini_2_5_flash_preview_09_2025,
            display_name: "Gemini 2.5 Flash Preview 09-25",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25Pro {
            model_name: "google/gemini-2.5-pro",
            constructor_name: google_gemini_2_5_pro,
            display_name: "Gemini 2.5 Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25ProPreview0506 {
            model_name: "google/gemini-2.5-pro-preview-05-06",
            constructor_name: google_gemini_2_5_pro_preview_05_06,
            display_name: "Gemini 2.5 Pro Preview 05-06",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25ProPreview0605 {
            model_name: "google/gemini-2.5-pro-preview-06-05",
            constructor_name: google_gemini_2_5_pro_preview_06_05,
            display_name: "Gemini 2.5 Pro Preview 06-05",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini3FlashPreview {
            model_name: "google/gemini-3-flash-preview",
            constructor_name: google_gemini_3_flash_preview,
            display_name: "Gemini 3 Flash Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini3ProPreview {
            model_name: "google/gemini-3-pro-preview",
            constructor_name: google_gemini_3_pro_preview,
            display_name: "Gemini 3 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemma29bIt {
            model_name: "google/gemma-2-9b-it",
            constructor_name: google_gemma_2_9b_it,
            display_name: "Gemma 2 9B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleGemma312bIt {
            model_name: "google/gemma-3-12b-it",
            constructor_name: google_gemma_3_12b_it,
            display_name: "Gemma 3 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma312bItFree {
            model_name: "google/gemma-3-12b-it:free",
            constructor_name: google_gemma_3_12b_it_free,
            display_name: "Gemma 3 12B (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma327bIt {
            model_name: "google/gemma-3-27b-it",
            constructor_name: google_gemma_3_27b_it,
            display_name: "Gemma 3 27B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma327bItFree {
            model_name: "google/gemma-3-27b-it:free",
            constructor_name: google_gemma_3_27b_it_free,
            display_name: "Gemma 3 27B (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma34bIt {
            model_name: "google/gemma-3-4b-it",
            constructor_name: google_gemma_3_4b_it,
            display_name: "Gemma 3 4B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma34bItFree {
            model_name: "google/gemma-3-4b-it:free",
            constructor_name: google_gemma_3_4b_it_free,
            display_name: "Gemma 3 4B (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma3nE2bItFree {
            model_name: "google/gemma-3n-e2b-it:free",
            constructor_name: google_gemma_3n_e2b_it_free,
            display_name: "Gemma 3n 2B (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma3nE4bIt {
            model_name: "google/gemma-3n-e4b-it",
            constructor_name: google_gemma_3n_e4b_it,
            display_name: "Gemma 3n 4B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma3nE4bItFree {
            model_name: "google/gemma-3n-e4b-it:free",
            constructor_name: google_gemma_3n_e4b_it_free,
            display_name: "Gemma 3n 4B (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        KwaipilotKatCoderProFree {
            model_name: "kwaipilot/kat-coder-pro:free",
            constructor_name: kwaipilot_kat_coder_pro_free,
            display_name: "Kat Coder Pro (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LiquidLfm2512bInstructFree {
            model_name: "liquid/lfm-2.5-1.2b-instruct:free",
            constructor_name: liquid_lfm_2_5_1_2b_instruct_free,
            display_name: "LFM2.5-1.2B-Instruct (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LiquidLfm2512bThinkingFree {
            model_name: "liquid/lfm-2.5-1.2b-thinking:free",
            constructor_name: liquid_lfm_2_5_1_2b_thinking_free,
            display_name: "LFM2.5-1.2B-Thinking (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama31405bInstructFree {
            model_name: "meta-llama/llama-3.1-405b-instruct:free",
            constructor_name: meta_llama_llama_3_1_405b_instruct_free,
            display_name: "Llama 3.1 405B Instruct (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3211bVisionInstruct {
            model_name: "meta-llama/llama-3.2-11b-vision-instruct",
            constructor_name: meta_llama_llama_3_2_11b_vision_instruct,
            display_name: "Llama 3.2 11B Vision Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama323bInstructFree {
            model_name: "meta-llama/llama-3.2-3b-instruct:free",
            constructor_name: meta_llama_llama_3_2_3b_instruct_free,
            display_name: "Llama 3.2 3B Instruct (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3370bInstructFree {
            model_name: "meta-llama/llama-3.3-70b-instruct:free",
            constructor_name: meta_llama_llama_3_3_70b_instruct_free,
            display_name: "Llama 3.3 70B Instruct (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama4ScoutFree {
            model_name: "meta-llama/llama-4-scout:free",
            constructor_name: meta_llama_llama_4_scout_free,
            display_name: "Llama 4 Scout (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftMaiDsR1Free {
            model_name: "microsoft/mai-ds-r1:free",
            constructor_name: microsoft_mai_ds_r1_free,
            display_name: "MAI DS R1 (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimax01 {
            model_name: "minimax/minimax-01",
            constructor_name: minimax_minimax_01,
            display_name: "MiniMax-01",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM1 {
            model_name: "minimax/minimax-m1",
            constructor_name: minimax_minimax_m1,
            display_name: "MiniMax M1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM2 {
            model_name: "minimax/minimax-m2",
            constructor_name: minimax_minimax_m2,
            display_name: "MiniMax M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM21 {
            model_name: "minimax/minimax-m2.1",
            constructor_name: minimax_minimax_m2_1,
            display_name: "MiniMax M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiCodestral2508 {
            model_name: "mistralai/codestral-2508",
            constructor_name: mistralai_codestral_2508,
            display_name: "Codestral 2508",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstral2512 {
            model_name: "mistralai/devstral-2512",
            constructor_name: mistralai_devstral_2512,
            display_name: "Devstral 2 2512",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstral2512Free {
            model_name: "mistralai/devstral-2512:free",
            constructor_name: mistralai_devstral_2512_free,
            display_name: "Devstral 2 2512 (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstralMedium2507 {
            model_name: "mistralai/devstral-medium-2507",
            constructor_name: mistralai_devstral_medium_2507,
            display_name: "Devstral Medium",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstralSmall2505 {
            model_name: "mistralai/devstral-small-2505",
            constructor_name: mistralai_devstral_small_2505,
            display_name: "Devstral Small",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstralSmall2505Free {
            model_name: "mistralai/devstral-small-2505:free",
            constructor_name: mistralai_devstral_small_2505_free,
            display_name: "Devstral Small 2505 (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstralSmall2507 {
            model_name: "mistralai/devstral-small-2507",
            constructor_name: mistralai_devstral_small_2507,
            display_name: "Devstral Small 1.1",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistral7bInstructFree {
            model_name: "mistralai/mistral-7b-instruct:free",
            constructor_name: mistralai_mistral_7b_instruct_free,
            display_name: "Mistral 7B Instruct (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralMedium3 {
            model_name: "mistralai/mistral-medium-3",
            constructor_name: mistralai_mistral_medium_3,
            display_name: "Mistral Medium 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralMedium31 {
            model_name: "mistralai/mistral-medium-3.1",
            constructor_name: mistralai_mistral_medium_3_1,
            display_name: "Mistral Medium 3.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralNemoFree {
            model_name: "mistralai/mistral-nemo:free",
            constructor_name: mistralai_mistral_nemo_free,
            display_name: "Mistral Nemo (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall3124bInstruct {
            model_name: "mistralai/mistral-small-3.1-24b-instruct",
            constructor_name: mistralai_mistral_small_3_1_24b_instruct,
            display_name: "Mistral Small 3.1 24B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall3224bInstruct {
            model_name: "mistralai/mistral-small-3.2-24b-instruct",
            constructor_name: mistralai_mistral_small_3_2_24b_instruct,
            display_name: "Mistral Small 3.2 24B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall3224bInstructFree {
            model_name: "mistralai/mistral-small-3.2-24b-instruct:free",
            constructor_name: mistralai_mistral_small_3_2_24b_instruct_free,
            display_name: "Mistral Small 3.2 24B (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiDev72bFree {
            model_name: "moonshotai/kimi-dev-72b:free",
            constructor_name: moonshotai_kimi_dev_72b_free,
            display_name: "Kimi Dev 72b (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2 {
            model_name: "moonshotai/kimi-k2",
            constructor_name: moonshotai_kimi_k2,
            display_name: "Kimi K2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK20905 {
            model_name: "moonshotai/kimi-k2-0905",
            constructor_name: moonshotai_kimi_k2_0905,
            display_name: "Kimi K2 Instruct 0905",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK20905Exacto {
            model_name: "moonshotai/kimi-k2-0905:exacto",
            constructor_name: moonshotai_kimi_k2_0905_exacto,
            display_name: "Kimi K2 Instruct 0905 (exacto)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/kimi-k2-thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/kimi-k2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiK2Free {
            model_name: "moonshotai/kimi-k2:free",
            constructor_name: moonshotai_kimi_k2_free,
            display_name: "Kimi K2 (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchDeephermes3Llama38bPreview {
            model_name: "nousresearch/deephermes-3-llama-3-8b-preview",
            constructor_name: nousresearch_deephermes_3_llama_3_8b_preview,
            display_name: "DeepHermes 3 Llama 3 8B Preview",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes3Llama31405bFree {
            model_name: "nousresearch/hermes-3-llama-3.1-405b:free",
            constructor_name: nousresearch_hermes_3_llama_3_1_405b_free,
            display_name: "Hermes 3 405B Instruct (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes4405b {
            model_name: "nousresearch/hermes-4-405b",
            constructor_name: nousresearch_hermes_4_405b,
            display_name: "Hermes 4 405B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes470b {
            model_name: "nousresearch/hermes-4-70b",
            constructor_name: nousresearch_hermes_4_70b,
            display_name: "Hermes 4 70B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Nano30bA3bFree {
            model_name: "nvidia/nemotron-3-nano-30b-a3b:free",
            constructor_name: nvidia_nemotron_3_nano_30b_a3b_free,
            display_name: "Nemotron 3 Nano 30B A3B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNano12bV2VlFree {
            model_name: "nvidia/nemotron-nano-12b-v2-vl:free",
            constructor_name: nvidia_nemotron_nano_12b_v2_vl_free,
            display_name: "Nemotron Nano 12B 2 VL (free)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNano9bV2 {
            model_name: "nvidia/nemotron-nano-9b-v2",
            constructor_name: nvidia_nemotron_nano_9b_v2,
            display_name: "nvidia-nemotron-nano-9b-v2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNano9bV2Free {
            model_name: "nvidia/nemotron-nano-9b-v2:free",
            constructor_name: nvidia_nemotron_nano_9b_v2_free,
            display_name: "Nemotron Nano 9B V2 (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41 {
            model_name: "openai/gpt-4.1",
            constructor_name: openai_gpt_4_1,
            display_name: "GPT-4.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Mini {
            model_name: "openai/gpt-4.1-mini",
            constructor_name: openai_gpt_4_1_mini,
            display_name: "GPT-4.1 Mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMini {
            model_name: "openai/gpt-4o-mini",
            constructor_name: openai_gpt_4o_mini,
            display_name: "GPT-4o-mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5 {
            model_name: "openai/gpt-5",
            constructor_name: openai_gpt_5,
            display_name: "GPT-5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Chat {
            model_name: "openai/gpt-5-chat",
            constructor_name: openai_gpt_5_chat,
            display_name: "GPT-5 Chat (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt5Codex {
            model_name: "openai/gpt-5-codex",
            constructor_name: openai_gpt_5_codex,
            display_name: "GPT-5 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Image {
            model_name: "openai/gpt-5-image",
            constructor_name: openai_gpt_5_image,
            display_name: "GPT-5 Image",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Mini {
            model_name: "openai/gpt-5-mini",
            constructor_name: openai_gpt_5_mini,
            display_name: "GPT-5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Nano {
            model_name: "openai/gpt-5-nano",
            constructor_name: openai_gpt_5_nano,
            display_name: "GPT-5 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Pro {
            model_name: "openai/gpt-5-pro",
            constructor_name: openai_gpt_5_pro,
            display_name: "GPT-5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51 {
            model_name: "openai/gpt-5.1",
            constructor_name: openai_gpt_5_1,
            display_name: "GPT-5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Chat {
            model_name: "openai/gpt-5.1-chat",
            constructor_name: openai_gpt_5_1_chat,
            display_name: "GPT-5.1 Chat",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Codex {
            model_name: "openai/gpt-5.1-codex",
            constructor_name: openai_gpt_5_1_codex,
            display_name: "GPT-5.1-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51CodexMax {
            model_name: "openai/gpt-5.1-codex-max",
            constructor_name: openai_gpt_5_1_codex_max,
            display_name: "GPT-5.1-Codex-Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51CodexMini {
            model_name: "openai/gpt-5.1-codex-mini",
            constructor_name: openai_gpt_5_1_codex_mini,
            display_name: "GPT-5.1-Codex-Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52 {
            model_name: "openai/gpt-5.2",
            constructor_name: openai_gpt_5_2,
            display_name: "GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Chat {
            model_name: "openai/gpt-5.2-chat",
            constructor_name: openai_gpt_5_2_chat,
            display_name: "GPT-5.2 Chat",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Codex {
            model_name: "openai/gpt-5.2-codex",
            constructor_name: openai_gpt_5_2_codex,
            display_name: "GPT-5.2-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Pro {
            model_name: "openai/gpt-5.2-pro",
            constructor_name: openai_gpt_5_2_pro,
            display_name: "GPT-5.2 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120bExacto {
            model_name: "openai/gpt-oss-120b:exacto",
            constructor_name: openai_gpt_oss_120b_exacto,
            display_name: "GPT OSS 120B (exacto)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120bFree {
            model_name: "openai/gpt-oss-120b:free",
            constructor_name: openai_gpt_oss_120b_free,
            display_name: "gpt-oss-120b (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20bFree {
            model_name: "openai/gpt-oss-20b:free",
            constructor_name: openai_gpt_oss_20b_free,
            display_name: "gpt-oss-20b (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOssSafeguard20b {
            model_name: "openai/gpt-oss-safeguard-20b",
            constructor_name: openai_gpt_oss_safeguard_20b,
            display_name: "GPT OSS Safeguard 20B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO4Mini {
            model_name: "openai/o4-mini",
            constructor_name: openai_o4_mini,
            display_name: "o4 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenrouterSherlockDashAlpha {
            model_name: "openrouter/sherlock-dash-alpha",
            constructor_name: openrouter_sherlock_dash_alpha,
            display_name: "Sherlock Dash Alpha",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenrouterSherlockThinkAlpha {
            model_name: "openrouter/sherlock-think-alpha",
            constructor_name: openrouter_sherlock_think_alpha,
            display_name: "Sherlock Think Alpha",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen25Coder32bInstruct {
            model_name: "qwen/qwen-2.5-coder-32b-instruct",
            constructor_name: qwen_qwen_2_5_coder_32b_instruct,
            display_name: "Qwen2.5 Coder 32B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen25Vl7bInstructFree {
            model_name: "qwen/qwen-2.5-vl-7b-instruct:free",
            constructor_name: qwen_qwen_2_5_vl_7b_instruct_free,
            display_name: "Qwen2.5-VL 7B Instruct (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen25Vl32bInstructFree {
            model_name: "qwen/qwen2.5-vl-32b-instruct:free",
            constructor_name: qwen_qwen2_5_vl_32b_instruct_free,
            display_name: "Qwen2.5 VL 32B Instruct (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen25Vl72bInstruct {
            model_name: "qwen/qwen2.5-vl-72b-instruct",
            constructor_name: qwen_qwen2_5_vl_72b_instruct,
            display_name: "Qwen2.5 VL 72B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen25Vl72bInstructFree {
            model_name: "qwen/qwen2.5-vl-72b-instruct:free",
            constructor_name: qwen_qwen2_5_vl_72b_instruct_free,
            display_name: "Qwen2.5 VL 72B Instruct (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen314bFree {
            model_name: "qwen/qwen3-14b:free",
            constructor_name: qwen_qwen3_14b_free,
            display_name: "Qwen3 14B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22b0725 {
            model_name: "qwen/qwen3-235b-a22b-07-25",
            constructor_name: qwen_qwen3_235b_a22b_07_25,
            display_name: "Qwen3 235B A22B Instruct 2507",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22b0725Free {
            model_name: "qwen/qwen3-235b-a22b-07-25:free",
            constructor_name: qwen_qwen3_235b_a22b_07_25_free,
            display_name: "Qwen3 235B A22B Instruct 2507 (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bThinking2507 {
            model_name: "qwen/qwen3-235b-a22b-thinking-2507",
            constructor_name: qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3 235B A22B Thinking 2507",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bFree {
            model_name: "qwen/qwen3-235b-a22b:free",
            constructor_name: qwen_qwen3_235b_a22b_free,
            display_name: "Qwen3 235B A22B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3bInstruct2507 {
            model_name: "qwen/qwen3-30b-a3b-instruct-2507",
            constructor_name: qwen_qwen3_30b_a3b_instruct_2507,
            display_name: "Qwen3 30B A3B Instruct 2507",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3bThinking2507 {
            model_name: "qwen/qwen3-30b-a3b-thinking-2507",
            constructor_name: qwen_qwen3_30b_a3b_thinking_2507,
            display_name: "Qwen3 30B A3B Thinking 2507",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3bFree {
            model_name: "qwen/qwen3-30b-a3b:free",
            constructor_name: qwen_qwen3_30b_a3b_free,
            display_name: "Qwen3 30B A3B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen332bFree {
            model_name: "qwen/qwen3-32b:free",
            constructor_name: qwen_qwen3_32b_free,
            display_name: "Qwen3 32B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen34bFree {
            model_name: "qwen/qwen3-4b:free",
            constructor_name: qwen_qwen3_4b_free,
            display_name: "Qwen3 4B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen38bFree {
            model_name: "qwen/qwen3-8b:free",
            constructor_name: qwen_qwen3_8b_free,
            display_name: "Qwen3 8B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder {
            model_name: "qwen/qwen3-coder",
            constructor_name: qwen_qwen3_coder,
            display_name: "Qwen3 Coder",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder30bA3bInstruct {
            model_name: "qwen/qwen3-coder-30b-a3b-instruct",
            constructor_name: qwen_qwen3_coder_30b_a3b_instruct,
            display_name: "Qwen3 Coder 30B A3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderFlash {
            model_name: "qwen/qwen3-coder-flash",
            constructor_name: qwen_qwen3_coder_flash,
            display_name: "Qwen3 Coder Flash",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderExacto {
            model_name: "qwen/qwen3-coder:exacto",
            constructor_name: qwen_qwen3_coder_exacto,
            display_name: "Qwen3 Coder (exacto)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderFree {
            model_name: "qwen/qwen3-coder:free",
            constructor_name: qwen_qwen3_coder_free,
            display_name: "Qwen3 Coder 480B A35B Instruct (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Max {
            model_name: "qwen/qwen3-max",
            constructor_name: qwen_qwen3_max,
            display_name: "Qwen3 Max",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bInstruct {
            model_name: "qwen/qwen3-next-80b-a3b-instruct",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3 Next 80B A3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bInstructFree {
            model_name: "qwen/qwen3-next-80b-a3b-instruct:free",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct_free,
            display_name: "Qwen3 Next 80B A3B Instruct (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bThinking {
            model_name: "qwen/qwen3-next-80b-a3b-thinking",
            constructor_name: qwen_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3 Next 80B A3B Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwq32bFree {
            model_name: "qwen/qwq-32b:free",
            constructor_name: qwen_qwq_32b_free,
            display_name: "QwQ 32B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RekaaiRekaFlash3 {
            model_name: "rekaai/reka-flash-3",
            constructor_name: rekaai_reka_flash_3,
            display_name: "Reka Flash 3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SarvamaiSarvamMFree {
            model_name: "sarvamai/sarvam-m:free",
            constructor_name: sarvamai_sarvam_m_free,
            display_name: "Sarvam-M (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SourcefulRiverflowV2FastPreview {
            model_name: "sourceful/riverflow-v2-fast-preview",
            constructor_name: sourceful_riverflow_v2_fast_preview,
            display_name: "Riverflow V2 Fast Preview",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        SourcefulRiverflowV2MaxPreview {
            model_name: "sourceful/riverflow-v2-max-preview",
            constructor_name: sourceful_riverflow_v2_max_preview,
            display_name: "Riverflow V2 Max Preview",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        SourcefulRiverflowV2StandardPreview {
            model_name: "sourceful/riverflow-v2-standard-preview",
            constructor_name: sourceful_riverflow_v2_standard_preview,
            display_name: "Riverflow V2 Standard Preview",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        ThudmGlmZ132bFree {
            model_name: "thudm/glm-z1-32b:free",
            constructor_name: thudm_glm_z1_32b_free,
            display_name: "GLM Z1 32B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TngtechDeepseekR1t2ChimeraFree {
            model_name: "tngtech/deepseek-r1t2-chimera:free",
            constructor_name: tngtech_deepseek_r1t2_chimera_free,
            display_name: "DeepSeek R1T2 Chimera (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        TngtechTngR1tChimeraFree {
            model_name: "tngtech/tng-r1t-chimera:free",
            constructor_name: tngtech_tng_r1t_chimera_free,
            display_name: "R1T Chimera (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        XAiGrok3 {
            model_name: "x-ai/grok-3",
            constructor_name: x_ai_grok_3,
            display_name: "Grok 3",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok3Beta {
            model_name: "x-ai/grok-3-beta",
            constructor_name: x_ai_grok_3_beta,
            display_name: "Grok 3 Beta",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok3Mini {
            model_name: "x-ai/grok-3-mini",
            constructor_name: x_ai_grok_3_mini,
            display_name: "Grok 3 Mini",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok3MiniBeta {
            model_name: "x-ai/grok-3-mini-beta",
            constructor_name: x_ai_grok_3_mini_beta,
            display_name: "Grok 3 Mini Beta",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok4 {
            model_name: "x-ai/grok-4",
            constructor_name: x_ai_grok_4,
            display_name: "Grok 4",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok4Fast {
            model_name: "x-ai/grok-4-fast",
            constructor_name: x_ai_grok_4_fast,
            display_name: "Grok 4 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok41Fast {
            model_name: "x-ai/grok-4.1-fast",
            constructor_name: x_ai_grok_4_1_fast,
            display_name: "Grok 4.1 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrokCodeFast1 {
            model_name: "x-ai/grok-code-fast-1",
            constructor_name: x_ai_grok_code_fast_1,
            display_name: "Grok Code Fast 1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm45 {
            model_name: "z-ai/glm-4.5",
            constructor_name: z_ai_glm_4_5,
            display_name: "GLM 4.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm45Air {
            model_name: "z-ai/glm-4.5-air",
            constructor_name: z_ai_glm_4_5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm45AirFree {
            model_name: "z-ai/glm-4.5-air:free",
            constructor_name: z_ai_glm_4_5_air_free,
            display_name: "GLM 4.5 Air (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZAiGlm45v {
            model_name: "z-ai/glm-4.5v",
            constructor_name: z_ai_glm_4_5v,
            display_name: "GLM 4.5V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZAiGlm46 {
            model_name: "z-ai/glm-4.6",
            constructor_name: z_ai_glm_4_6,
            display_name: "GLM 4.6",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm46Exacto {
            model_name: "z-ai/glm-4.6:exacto",
            constructor_name: z_ai_glm_4_6_exacto,
            display_name: "GLM 4.6 (exacto)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47 {
            model_name: "z-ai/glm-4.7",
            constructor_name: z_ai_glm_4_7,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47Flash {
            model_name: "z-ai/glm-4.7-flash",
            constructor_name: z_ai_glm_4_7_flash,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
