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
        AionLabsAion20 {
            model_name: "aion-labs/aion-2.0",
            constructor_name: aion_labs_aion_2_0,
            display_name: "Aion-2.0",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AionLabsAion30 {
            model_name: "aion-labs/aion-3.0",
            constructor_name: aion_labs_aion_3_0,
            display_name: "Aion-3.0",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AionLabsAion30Mini {
            model_name: "aion-labs/aion-3.0-mini",
            constructor_name: aion_labs_aion_3_0_mini,
            display_name: "Aion-3.0-Mini",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AionLabsAion35 {
            model_name: "aion-labs/aion-3.5",
            constructor_name: aion_labs_aion_3_5,
            display_name: "Aion 3.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AionLabsAion35Mini {
            model_name: "aion-labs/aion-3.5-mini",
            constructor_name: aion_labs_aion_3_5_mini,
            display_name: "Aion 3.5 Mini",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AionLabsAionRpLlama318b {
            model_name: "aion-labs/aion-rp-llama-3.1-8b",
            constructor_name: aion_labs_aion_rp_llama_3_1_8b,
            display_name: "Aion-RP 1.0 (8B)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AllenaiMolmo28bFree {
            model_name: "allenai/molmo-2-8b:free",
            constructor_name: allenai_molmo_2_8b_free,
            display_name: "Molmo2 8B (free)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonNova2LiteV1 {
            model_name: "amazon/nova-2-lite-v1",
            constructor_name: amazon_nova_2_lite_v1,
            display_name: "Nova 2 Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonNovaLiteV1 {
            model_name: "amazon/nova-lite-v1",
            constructor_name: amazon_nova_lite_v1,
            display_name: "Nova Lite 1.0",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AmazonNovaMicroV1 {
            model_name: "amazon/nova-micro-v1",
            constructor_name: amazon_nova_micro_v1,
            display_name: "Nova Micro 1.0",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AmazonNovaProV1 {
            model_name: "amazon/nova-pro-v1",
            constructor_name: amazon_nova_pro_v1,
            display_name: "Nova Pro 1.0",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthraciteOrgMagnumV472b {
            model_name: "anthracite-org/magnum-v4-72b",
            constructor_name: anthracite_org_magnum_v4_72b,
            display_name: "Magnum v4 72B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
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
        AnthropicClaudeHaiku45 {
            model_name: "anthropic/claude-haiku-4.5",
            constructor_name: anthropic_claude_haiku_4_5,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku55 {
            model_name: "anthropic/claude-haiku-5.5",
            constructor_name: anthropic_claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46 {
            model_name: "anthropic/claude-opus-4.6",
            constructor_name: anthropic_claude_opus_4_6,
            display_name: "Claude Opus 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus47 {
            model_name: "anthropic/claude-opus-4.7",
            constructor_name: anthropic_claude_opus_4_7,
            display_name: "Claude Opus 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus48 {
            model_name: "anthropic/claude-opus-4.8",
            constructor_name: anthropic_claude_opus_4_8,
            display_name: "Claude Opus 4.8",
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet46 {
            model_name: "anthropic/claude-sonnet-4.6",
            constructor_name: anthropic_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet5 {
            model_name: "anthropic/claude-sonnet-5",
            constructor_name: anthropic_claude_sonnet_5,
            display_name: "Claude Sonnet 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet55 {
            model_name: "anthropic/claude-sonnet-5.5",
            constructor_name: anthropic_claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        BaiduErnie55FlashPreviewFree {
            model_name: "baidu/ernie-5.5-flash-preview:free",
            constructor_name: baidu_ernie_5_5_flash_preview_free,
            display_name: "ERNIE-5.5-Flash-Preview (free)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BaiduErnieLightDt10Intl {
            model_name: "baidu/ernie-light-dt-1.0-intl",
            constructor_name: baidu_ernie_light_dt_1_0_intl,
            display_name: "ERNIE-Light-DT-1.0",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        BytedanceSeedSeed21Turbo {
            model_name: "bytedance-seed/seed-2-1-turbo",
            constructor_name: bytedance_seed_seed_2_1_turbo,
            display_name: "Seed 2.1 Turbo",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        BytedanceSeedSeed20Lite {
            model_name: "bytedance-seed/seed-2.0-lite",
            constructor_name: bytedance_seed_seed_2_0_lite,
            display_name: "Seed 2.0 Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        BytedanceSeedSeed20Mini {
            model_name: "bytedance-seed/seed-2.0-mini",
            constructor_name: bytedance_seed_seed_2_0_mini,
            display_name: "Seed 2.0 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        BytedanceSeedSeedream45 {
            model_name: "bytedance-seed/seedream-4.5",
            constructor_name: bytedance_seed_seedream_4_5,
            display_name: "Seedream 4.5",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        BytedanceUiTars157b {
            model_name: "bytedance/ui-tars-1.5-7b",
            constructor_name: bytedance_ui_tars_1_5_7b,
            display_name: "UI-TARS 7B ",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        CognitivecomputationsDolphinMistral24bVeniceEdition {
            model_name: "cognitivecomputations/dolphin-mistral-24b-venice-edition",
            constructor_name: cognitivecomputations_dolphin_mistral_24b_venice_edition,
            display_name: "Uncensored",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        CohereCommandA {
            model_name: "cohere/command-a",
            constructor_name: cohere_command_a,
            display_name: "Command A",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereCommandAPlus {
            model_name: "cohere/command-a-plus",
            constructor_name: cohere_command_a_plus,
            display_name: "Command A+",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereCommandR082024 {
            model_name: "cohere/command-r-08-2024",
            constructor_name: cohere_command_r_08_2024,
            display_name: "Command R",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereCommandRPlus082024 {
            model_name: "cohere/command-r-plus-08-2024",
            constructor_name: cohere_command_r_plus_08_2024,
            display_name: "Command R+",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereCommandR7b122024 {
            model_name: "cohere/command-r7b-12-2024",
            constructor_name: cohere_command_r7b_12_2024,
            display_name: "Command R7B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereNorthMiniCodeFree {
            model_name: "cohere/north-mini-code:free",
            constructor_name: cohere_north_mini_code_free,
            display_name: "North Mini Code (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekChat {
            model_name: "deepseek/deepseek-chat",
            constructor_name: deepseek_deepseek_chat,
            display_name: "DeepSeek Chat",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekChatV30324 {
            model_name: "deepseek/deepseek-chat-v3-0324",
            constructor_name: deepseek_deepseek_chat_v3_0324,
            display_name: "DeepSeek V3 0324",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekChatV31 {
            model_name: "deepseek/deepseek-chat-v3.1",
            constructor_name: deepseek_deepseek_chat_v3_1,
            display_name: "DeepSeek-V3.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekR10528 {
            model_name: "deepseek/deepseek-r1-0528",
            constructor_name: deepseek_deepseek_r1_0528,
            display_name: "DeepSeek R1 0528",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32Exp {
            model_name: "deepseek/deepseek-v3.2-exp",
            constructor_name: deepseek_deepseek_v3_2_exp,
            display_name: "DeepSeek V3.2 Exp",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32Speciale {
            model_name: "deepseek/deepseek-v3.2-speciale",
            constructor_name: deepseek_deepseek_v3_2_speciale,
            display_name: "DeepSeek V3.2 Speciale",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash {
            model_name: "deepseek/deepseek-v4-flash",
            constructor_name: deepseek_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash0731 {
            model_name: "deepseek/deepseek-v4-flash-0731",
            constructor_name: deepseek_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4FlashVisionExp {
            model_name: "deepseek/deepseek-v4-flash-vision-exp",
            constructor_name: deepseek_deepseek_v4_flash_vision_exp,
            display_name: "DeepSeek V4 Flash Vision Exp",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        DeepseekDeepseekV41Flash {
            model_name: "deepseek/deepseek-v4.1-flash",
            constructor_name: deepseek_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash1017 {
            model_name: "deepseek/deepseek-v4.1-flash-1017",
            constructor_name: deepseek_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ElorianGlimpse {
            model_name: "elorian/glimpse",
            constructor_name: elorian_glimpse,
            display_name: "Glimpse",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        FeatherlessQwerky72b {
            model_name: "featherless/qwerky-72b",
            constructor_name: featherless_qwerky_72b,
            display_name: "Qwerky 72B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        FireworksEmber1 {
            model_name: "fireworks/ember-1",
            constructor_name: fireworks_ember_1,
            display_name: "Ember-1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25FlashImage {
            model_name: "google/gemini-2.5-flash-image",
            constructor_name: google_gemini_2_5_flash_image,
            display_name: "Nano Banana",
            capabilities: [ImageInputSupport, ImageOutputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini25FlashLite {
            model_name: "google/gemini-2.5-flash-lite",
            constructor_name: google_gemini_2_5_flash_lite,
            display_name: "Gemini 2.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
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
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25ProPreview {
            model_name: "google/gemini-2.5-pro-preview",
            constructor_name: google_gemini_2_5_pro_preview,
            display_name: "Gemini 2.5 Pro Preview 06-05",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini3ProImage {
            model_name: "google/gemini-3-pro-image",
            constructor_name: google_gemini_3_pro_image,
            display_name: "Nano Banana Pro",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini3ProImagePreview {
            model_name: "google/gemini-3-pro-image-preview",
            constructor_name: google_gemini_3_pro_image_preview,
            display_name: "Nano Banana Pro Preview",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini3ProPreview {
            model_name: "google/gemini-3-pro-preview",
            constructor_name: google_gemini_3_pro_preview,
            display_name: "Gemini 3 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31FlashImage {
            model_name: "google/gemini-3.1-flash-image",
            constructor_name: google_gemini_3_1_flash_image,
            display_name: "Nano Banana 2",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini31FlashImagePreview {
            model_name: "google/gemini-3.1-flash-image-preview",
            constructor_name: google_gemini_3_1_flash_image_preview,
            display_name: "Nano Banana 2 Preview",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini31FlashLite {
            model_name: "google/gemini-3.1-flash-lite",
            constructor_name: google_gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31FlashLiteImage {
            model_name: "google/gemini-3.1-flash-lite-image",
            constructor_name: google_gemini_3_1_flash_lite_image,
            display_name: "Nano Banana 2 Lite",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini31FlashLitePreview {
            model_name: "google/gemini-3.1-flash-lite-preview",
            constructor_name: google_gemini_3_1_flash_lite_preview,
            display_name: "Gemini 3.1 Flash Lite Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31ProPreview {
            model_name: "google/gemini-3.1-pro-preview",
            constructor_name: google_gemini_3_1_pro_preview,
            display_name: "Gemini 3.1 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31ProPreviewCustomtools {
            model_name: "google/gemini-3.1-pro-preview-customtools",
            constructor_name: google_gemini_3_1_pro_preview_customtools,
            display_name: "Gemini 3.1 Pro Preview Custom Tools",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
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
        GoogleGemini4Hydrogen {
            model_name: "google/gemini-4-hydrogen",
            constructor_name: google_gemini_4_hydrogen,
            display_name: "Gemini 4 Hydrogen",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGeminiNanoBanana21 {
            model_name: "google/gemini-nano-banana-2.1",
            constructor_name: google_gemini_nano_banana_2_1,
            display_name: "Nano Banana 2.1",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGeminiOmni11Flash {
            model_name: "google/gemini-omni-1.1-flash",
            constructor_name: google_gemini_omni_1_1_flash,
            display_name: "Gemini Omni 1.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport, VideoOutputSupport]
        },
        GoogleGeminiOmni12Flash {
            model_name: "google/gemini-omni-1.2-flash",
            constructor_name: google_gemini_omni_1_2_flash,
            display_name: "Gemini Omni 1.2 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport, VideoOutputSupport]
        },
        GoogleGemma227bIt {
            model_name: "google/gemma-2-27b-it",
            constructor_name: google_gemma_2_27b_it,
            display_name: "Gemma 2 27B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
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
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
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
        GoogleGemma426bA4bIt {
            model_name: "google/gemma-4-26b-a4b-it",
            constructor_name: google_gemma_4_26b_a4b_it,
            display_name: "Gemma 4 26B A4B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemma426bA4bItFree {
            model_name: "google/gemma-4-26b-a4b-it:free",
            constructor_name: google_gemma_4_26b_a4b_it_free,
            display_name: "Gemma 4 26B A4B  (free)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31b-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemma431bItFree {
            model_name: "google/gemma-4-31b-it:free",
            constructor_name: google_gemma_4_31b_it_free,
            display_name: "Gemma 4 31B (free)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleLyria3ClipPreview {
            model_name: "google/lyria-3-clip-preview",
            constructor_name: google_lyria_3_clip_preview,
            display_name: "Lyria 3 Clip Preview",
            capabilities: [AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleLyria3ProPreview {
            model_name: "google/lyria-3-pro-preview",
            constructor_name: google_lyria_3_pro_preview,
            display_name: "Lyria 3 Pro Preview",
            capabilities: [AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GrypheMythomaxL213b {
            model_name: "gryphe/mythomax-l2-13b",
            constructor_name: gryphe_mythomax_l2_13b,
            display_name: "MythoMax L2 13B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        IbmGraniteGranite428b {
            model_name: "ibm-granite/granite-4.2-8b",
            constructor_name: ibm_granite_granite_4_2_8b,
            display_name: "Granite 4.2 8B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InceptionMercury2 {
            model_name: "inception/mercury-2",
            constructor_name: inception_mercury_2,
            display_name: "Mercury 2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InceptionMercury25 {
            model_name: "inception/mercury-2.5",
            constructor_name: inception_mercury_2_5,
            display_name: "Mercury 2.5",
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
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30FlashVl {
            model_name: "inclusionai/ling-3.0-flash-vl",
            constructor_name: inclusionai_ling_3_0_flash_vl,
            display_name: "Ling 3.0 Flash VL",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        InclusionaiLing30Tiny {
            model_name: "inclusionai/ling-3.0-tiny",
            constructor_name: inclusionai_ling_3_0_tiny,
            display_name: "Ling 3.0 Tiny",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        KtMidmK30Pro {
            model_name: "kt/midm-k-3.0-pro",
            constructor_name: kt_midm_k_3_0_pro,
            display_name: "Mi:dm K 3.0 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        LiquidLfm2526b {
            model_name: "liquid/lfm-2.5-2.6b",
            constructor_name: liquid_lfm_2_5_2_6b,
            display_name: "LFM2.5-2.6B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LiquidLfm2526bFree {
            model_name: "liquid/lfm-2.5-2.6b:free",
            constructor_name: liquid_lfm_2_5_2_6b_free,
            display_name: "LFM2.5-2.6B (free)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LiquidLfm2524bA2b {
            model_name: "liquid/lfm-2.5-24b-a2b",
            constructor_name: liquid_lfm_2_5_24b_a2b,
            display_name: "LFM2.5-24B-A2B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LiquidLfm317b {
            model_name: "liquid/lfm-3-1.7b",
            constructor_name: liquid_lfm_3_1_7b,
            display_name: "LFM3-1.7B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LiquidLfm317bFree {
            model_name: "liquid/lfm-3-1.7b:free",
            constructor_name: liquid_lfm_3_1_7b_free,
            display_name: "LFM3-1.7B (free)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MancerWeaver {
            model_name: "mancer/weaver",
            constructor_name: mancer_weaver,
            display_name: "Weaver (alpha)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MeituanLongcat20 {
            model_name: "meituan/longcat-2.0",
            constructor_name: meituan_longcat_2_0,
            display_name: "LongCat-2.0",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama31405bInstructFree {
            model_name: "meta-llama/llama-3.1-405b-instruct:free",
            constructor_name: meta_llama_llama_3_1_405b_instruct_free,
            display_name: "Llama 3.1 405B Instruct (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3170bInstruct {
            model_name: "meta-llama/llama-3.1-70b-instruct",
            constructor_name: meta_llama_llama_3_1_70b_instruct,
            display_name: "Llama-3.1-70B-Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama318bInstruct {
            model_name: "meta-llama/llama-3.1-8b-instruct",
            constructor_name: meta_llama_llama_3_1_8b_instruct,
            display_name: "Llama-3.1-8B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3211bVisionInstruct {
            model_name: "meta-llama/llama-3.2-11b-vision-instruct",
            constructor_name: meta_llama_llama_3_2_11b_vision_instruct,
            display_name: "Llama 3.2 11B Vision Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama321bInstruct {
            model_name: "meta-llama/llama-3.2-1b-instruct",
            constructor_name: meta_llama_llama_3_2_1b_instruct,
            display_name: "Llama 3.2 1B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama323bInstruct {
            model_name: "meta-llama/llama-3.2-3b-instruct",
            constructor_name: meta_llama_llama_3_2_3b_instruct,
            display_name: "Llama 3.2 3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama323bInstructFree {
            model_name: "meta-llama/llama-3.2-3b-instruct:free",
            constructor_name: meta_llama_llama_3_2_3b_instruct_free,
            display_name: "Llama 3.2 3B Instruct (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3370bInstruct {
            model_name: "meta-llama/llama-3.3-70b-instruct",
            constructor_name: meta_llama_llama_3_3_70b_instruct,
            display_name: "Llama-3.3-70B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3370bInstructFree {
            model_name: "meta-llama/llama-3.3-70b-instruct:free",
            constructor_name: meta_llama_llama_3_3_70b_instruct_free,
            display_name: "Llama 3.3 70B Instruct (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        MetaLlamaLlama4ScoutFree {
            model_name: "meta-llama/llama-4-scout:free",
            constructor_name: meta_llama_llama_4_scout_free,
            display_name: "Llama 4 Scout (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlamaGuard412b {
            model_name: "meta-llama/llama-guard-4-12b",
            constructor_name: meta_llama_llama_guard_4_12b,
            display_name: "Llama Guard 4 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSpark12 {
            model_name: "meta/muse-spark-1.2",
            constructor_name: meta_muse_spark_1_2,
            display_name: "Muse Spark 1.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSpark12Contributor {
            model_name: "meta/muse-spark-1.2-contributor",
            constructor_name: meta_muse_spark_1_2_contributor,
            display_name: "Muse Spark 1.2 Contributor",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSpark13 {
            model_name: "meta/muse-spark-1.3",
            constructor_name: meta_muse_spark_1_3,
            display_name: "Muse Spark 1.3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSpark13Contributor {
            model_name: "meta/muse-spark-1.3-contributor",
            constructor_name: meta_muse_spark_1_3_contributor,
            display_name: "Muse Spark 1.3 Contributor",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSparkOpen {
            model_name: "meta/muse-spark-open",
            constructor_name: meta_muse_spark_open,
            display_name: "Muse Spark Open",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseWonder {
            model_name: "meta/muse-wonder",
            constructor_name: meta_muse_wonder,
            display_name: "Muse Wonder",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MicrosoftMaiDsR1Free {
            model_name: "microsoft/mai-ds-r1:free",
            constructor_name: microsoft_mai_ds_r1_free,
            display_name: "MAI DS R1 (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftPhi4 {
            model_name: "microsoft/phi-4",
            constructor_name: microsoft_phi_4,
            display_name: "Phi 4",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
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
        MinimaxMinimaxM25 {
            model_name: "minimax/minimax-m2.5",
            constructor_name: minimax_minimax_m2_5,
            display_name: "MiniMax-M2.5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM27 {
            model_name: "minimax/minimax-m2.7",
            constructor_name: minimax_minimax_m2_7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM3 {
            model_name: "minimax/minimax-m3",
            constructor_name: minimax_minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MinimaxMinimaxM31Flash {
            model_name: "minimax/minimax-m3.1-flash",
            constructor_name: minimax_minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MistralaiCodestral2508 {
            model_name: "mistralai/codestral-2508",
            constructor_name: mistralai_codestral_2508,
            display_name: "Codestral 2508",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstral2512 {
            model_name: "mistralai/devstral-2512",
            constructor_name: mistralai_devstral_2512,
            display_name: "Devstral 2 2512",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        MistralaiMinistral14b2512 {
            model_name: "mistralai/ministral-14b-2512",
            constructor_name: mistralai_ministral_14b_2512,
            display_name: "Ministral 3 14B 2512",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMinistral3b2512 {
            model_name: "mistralai/ministral-3b-2512",
            constructor_name: mistralai_ministral_3b_2512,
            display_name: "Ministral 3 3B 2512",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMinistral8b2512 {
            model_name: "mistralai/ministral-8b-2512",
            constructor_name: mistralai_ministral_8b_2512,
            display_name: "Ministral 3 8B 2512",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistral7bInstructFree {
            model_name: "mistralai/mistral-7b-instruct:free",
            constructor_name: mistralai_mistral_7b_instruct_free,
            display_name: "Mistral 7B Instruct (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralLarge {
            model_name: "mistralai/mistral-large",
            constructor_name: mistralai_mistral_large,
            display_name: "Mistral Large",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralLarge2407 {
            model_name: "mistralai/mistral-large-2407",
            constructor_name: mistralai_mistral_large_2407,
            display_name: "Mistral Large 2407",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralLarge2512 {
            model_name: "mistralai/mistral-large-2512",
            constructor_name: mistralai_mistral_large_2512,
            display_name: "Mistral Large 3",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralLarge40 {
            model_name: "mistralai/mistral-large-4-0",
            constructor_name: mistralai_mistral_large_4_0,
            display_name: "Mistral Large 4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralMedium3 {
            model_name: "mistralai/mistral-medium-3",
            constructor_name: mistralai_mistral_medium_3,
            display_name: "Mistral Medium 3",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralMedium35 {
            model_name: "mistralai/mistral-medium-3-5",
            constructor_name: mistralai_mistral_medium_3_5,
            display_name: "Mistral Medium 3.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralMedium31 {
            model_name: "mistralai/mistral-medium-3.1",
            constructor_name: mistralai_mistral_medium_3_1,
            display_name: "Mistral Medium 3.1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralNemo {
            model_name: "mistralai/mistral-nemo",
            constructor_name: mistralai_mistral_nemo,
            display_name: "Mistral Nemo",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralNemoFree {
            model_name: "mistralai/mistral-nemo:free",
            constructor_name: mistralai_mistral_nemo_free,
            display_name: "Mistral Nemo (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSaba {
            model_name: "mistralai/mistral-saba",
            constructor_name: mistralai_mistral_saba,
            display_name: "Saba",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall24bInstruct2501 {
            model_name: "mistralai/mistral-small-24b-instruct-2501",
            constructor_name: mistralai_mistral_small_24b_instruct_2501,
            display_name: "Mistral Small 3",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralSmall2603 {
            model_name: "mistralai/mistral-small-2603",
            constructor_name: mistralai_mistral_small_2603,
            display_name: "Mistral Small 4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall3124bInstruct {
            model_name: "mistralai/mistral-small-3.1-24b-instruct",
            constructor_name: mistralai_mistral_small_3_1_24b_instruct,
            display_name: "Mistral Small 3.1 24B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall3224bInstruct {
            model_name: "mistralai/mistral-small-3.2-24b-instruct",
            constructor_name: mistralai_mistral_small_3_2_24b_instruct,
            display_name: "Mistral Small 3.2 24B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall3224bInstructFree {
            model_name: "mistralai/mistral-small-3.2-24b-instruct:free",
            constructor_name: mistralai_mistral_small_3_2_24b_instruct_free,
            display_name: "Mistral Small 3.2 24B (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMixtral8x22bInstruct {
            model_name: "mistralai/mixtral-8x22b-instruct",
            constructor_name: mistralai_mixtral_8x22b_instruct,
            display_name: "Mixtral 8x22B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiVoxtralSmall24b2507 {
            model_name: "mistralai/voxtral-small-24b-2507",
            constructor_name: mistralai_voxtral_small_24b_2507,
            display_name: "Voxtral Small 24B 2507",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            display_name: "Moondream 3.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
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
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/kimi-k2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiK26 {
            model_name: "moonshotai/kimi-k2.6",
            constructor_name: moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK27Code {
            model_name: "moonshotai/kimi-k2.7-code",
            constructor_name: moonshotai_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Free {
            model_name: "moonshotai/kimi-k2:free",
            constructor_name: moonshotai_kimi_k2_free,
            display_name: "Kimi K2 (free)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        MorphMorphV3Fast {
            model_name: "morph/morph-v3-fast",
            constructor_name: morph_morph_v3_fast,
            display_name: "Morph V3 Fast",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        MorphMorphV3Large {
            model_name: "morph/morph-v3-large",
            constructor_name: morph_morph_v3_large,
            display_name: "Morph V3 Large",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        MotifTechnologiesMotif3 {
            model_name: "motif-technologies/motif-3",
            constructor_name: motif_technologies_motif_3,
            display_name: "Motif 3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NexAgiNexN25Mini {
            model_name: "nex-agi/nex-n2.5-mini",
            constructor_name: nex_agi_nex_n2_5_mini,
            display_name: "Nex-N2.5-Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        NexAgiNexN25Pro {
            model_name: "nex-agi/nex-n2.5-pro",
            constructor_name: nex_agi_nex_n2_5_pro,
            display_name: "Nex-N2.5-Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NexAgiNexN26 {
            model_name: "nex-agi/nex-n2.6",
            constructor_name: nex_agi_nex_n2_6,
            display_name: "Nex-N2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes470b {
            model_name: "nousresearch/hermes-4-70b",
            constructor_name: nousresearch_hermes_4_70b,
            display_name: "Hermes 4 70B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Nano30bA3b {
            model_name: "nvidia/nemotron-3-nano-30b-a3b",
            constructor_name: nvidia_nemotron_3_nano_30b_a3b,
            display_name: "Nemotron 3 Nano 30B A3B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Nano30bA3bFree {
            model_name: "nvidia/nemotron-3-nano-30b-a3b:free",
            constructor_name: nvidia_nemotron_3_nano_30b_a3b_free,
            display_name: "Nemotron 3 Nano 30B A3B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3NanoOmni30bA3bReasoningFree {
            model_name: "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free",
            constructor_name: nvidia_nemotron_3_nano_omni_30b_a3b_reasoning_free,
            display_name: "Nemotron 3 Nano Omni (free)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaNemotron3Super120bA12b {
            model_name: "nvidia/nemotron-3-super-120b-a12b",
            constructor_name: nvidia_nemotron_3_super_120b_a12b,
            display_name: "Nemotron 3 Super 120B A12B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Ultra550bA55b {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b,
            display_name: "Nemotron 3 Ultra 550B A55B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Ultra550bA55bFree {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b:free",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b_free,
            display_name: "Nemotron 3 Ultra (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35ContentSafety {
            model_name: "nvidia/nemotron-3.5-content-safety",
            constructor_name: nvidia_nemotron_3_5_content_safety,
            display_name: "Nemotron 3.5 Content Safety",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron35Lightning {
            model_name: "nvidia/nemotron-3.5-lightning",
            constructor_name: nvidia_nemotron_3_5_lightning,
            display_name: "Nemotron 3.5 Lightning 30B A3B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35LightningFree {
            model_name: "nvidia/nemotron-3.5-lightning:free",
            constructor_name: nvidia_nemotron_3_5_lightning_free,
            display_name: "Nemotron 3.5 Lightning (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35SuperVl {
            model_name: "nvidia/nemotron-3.5-super-vl",
            constructor_name: nvidia_nemotron_3_5_super_vl,
            display_name: "Nemotron 3.5 Super VL",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaNemotron35SuperVlFree {
            model_name: "nvidia/nemotron-3.5-super-vl:free",
            constructor_name: nvidia_nemotron_3_5_super_vl_free,
            display_name: "Nemotron 3.5 Super VL (free)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
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
        OpenaiGpt35Turbo0613 {
            model_name: "openai/gpt-3.5-turbo-0613",
            constructor_name: openai_gpt_3_5_turbo_0613,
            display_name: "GPT-3.5 Turbo (older v0613)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt35Turbo16k {
            model_name: "openai/gpt-3.5-turbo-16k",
            constructor_name: openai_gpt_3_5_turbo_16k,
            display_name: "GPT-3.5 Turbo 16k",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4 {
            model_name: "openai/gpt-4",
            constructor_name: openai_gpt_4,
            display_name: "GPT-4",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41 {
            model_name: "openai/gpt-4.1",
            constructor_name: openai_gpt_4_1,
            display_name: "GPT-4.1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Mini {
            model_name: "openai/gpt-4.1-mini",
            constructor_name: openai_gpt_4_1_mini,
            display_name: "GPT-4.1 Mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Nano {
            model_name: "openai/gpt-4.1-nano",
            constructor_name: openai_gpt_4_1_nano,
            display_name: "GPT-4.1 nano",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4o {
            model_name: "openai/gpt-4o",
            constructor_name: openai_gpt_4o,
            display_name: "GPT-4o",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4o20240513 {
            model_name: "openai/gpt-4o-2024-05-13",
            constructor_name: openai_gpt_4o_2024_05_13,
            display_name: "GPT-4o (2024-05-13)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4o20240806 {
            model_name: "openai/gpt-4o-2024-08-06",
            constructor_name: openai_gpt_4o_2024_08_06,
            display_name: "GPT-4o (2024-08-06)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4o20241120 {
            model_name: "openai/gpt-4o-2024-11-20",
            constructor_name: openai_gpt_4o_2024_11_20,
            display_name: "GPT-4o (2024-11-20)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMini {
            model_name: "openai/gpt-4o-mini",
            constructor_name: openai_gpt_4o_mini,
            display_name: "GPT-4o-mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMini20240718 {
            model_name: "openai/gpt-4o-mini-2024-07-18",
            constructor_name: openai_gpt_4o_mini_2024_07_18,
            display_name: "GPT-4o-mini (2024-07-18)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5 {
            model_name: "openai/gpt-5",
            constructor_name: openai_gpt_5,
            display_name: "GPT-5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5ImageMini {
            model_name: "openai/gpt-5-image-mini",
            constructor_name: openai_gpt_5_image_mini,
            display_name: "GPT-5 Image Mini",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt5Mini {
            model_name: "openai/gpt-5-mini",
            constructor_name: openai_gpt_5_mini,
            display_name: "GPT-5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Nano {
            model_name: "openai/gpt-5-nano",
            constructor_name: openai_gpt_5_nano,
            display_name: "GPT-5 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Pro {
            model_name: "openai/gpt-5-pro",
            constructor_name: openai_gpt_5_pro,
            display_name: "GPT-5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        OpenaiGpt53Codex {
            model_name: "openai/gpt-5.3-codex",
            constructor_name: openai_gpt_5_3_codex,
            display_name: "GPT-5.3 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54 {
            model_name: "openai/gpt-5.4",
            constructor_name: openai_gpt_5_4,
            display_name: "GPT-5.4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54Image2 {
            model_name: "openai/gpt-5.4-image-2",
            constructor_name: openai_gpt_5_4_image_2,
            display_name: "GPT-5.4 Image 2",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt54Mini {
            model_name: "openai/gpt-5.4-mini",
            constructor_name: openai_gpt_5_4_mini,
            display_name: "GPT-5.4 mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54Nano {
            model_name: "openai/gpt-5.4-nano",
            constructor_name: openai_gpt_5_4_nano,
            display_name: "GPT-5.4 nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54Pro {
            model_name: "openai/gpt-5.4-pro",
            constructor_name: openai_gpt_5_4_pro,
            display_name: "GPT-5.4 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt55 {
            model_name: "openai/gpt-5.5",
            constructor_name: openai_gpt_5_5,
            display_name: "GPT-5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt55Pro {
            model_name: "openai/gpt-5.5-pro",
            constructor_name: openai_gpt_5_5_pro,
            display_name: "GPT-5.5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Luna {
            model_name: "openai/gpt-5.6-luna",
            constructor_name: openai_gpt_5_6_luna,
            display_name: "GPT-5.6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56LunaPro {
            model_name: "openai/gpt-5.6-luna-pro",
            constructor_name: openai_gpt_5_6_luna_pro,
            display_name: "GPT-5.6 Luna Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Sol {
            model_name: "openai/gpt-5.6-sol",
            constructor_name: openai_gpt_5_6_sol,
            display_name: "GPT-5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56SolPro {
            model_name: "openai/gpt-5.6-sol-pro",
            constructor_name: openai_gpt_5_6_sol_pro,
            display_name: "GPT-5.6 Sol Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Terra {
            model_name: "openai/gpt-5.6-terra",
            constructor_name: openai_gpt_5_6_terra,
            display_name: "GPT-5.6 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56TerraPro {
            model_name: "openai/gpt-5.6-terra-pro",
            constructor_name: openai_gpt_5_6_terra_pro,
            display_name: "GPT-5.6 Terra Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Astra {
            model_name: "openai/gpt-6-astra",
            constructor_name: openai_gpt_6_astra,
            display_name: "GPT-6 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6AstraPro {
            model_name: "openai/gpt-6-astra-pro",
            constructor_name: openai_gpt_6_astra_pro,
            display_name: "GPT-6 Astra Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Luna {
            model_name: "openai/gpt-6-luna",
            constructor_name: openai_gpt_6_luna,
            display_name: "GPT-6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6LunaPro {
            model_name: "openai/gpt-6-luna-pro",
            constructor_name: openai_gpt_6_luna_pro,
            display_name: "GPT-6 Luna Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Sol {
            model_name: "openai/gpt-6-sol",
            constructor_name: openai_gpt_6_sol,
            display_name: "GPT-6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6SolPro {
            model_name: "openai/gpt-6-sol-pro",
            constructor_name: openai_gpt_6_sol_pro,
            display_name: "GPT-6 Sol Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61Sol {
            model_name: "openai/gpt-6.1-sol",
            constructor_name: openai_gpt_6_1_sol,
            display_name: "GPT-6.1 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61SolPro {
            model_name: "openai/gpt-6.1-sol-pro",
            constructor_name: openai_gpt_6_1_sol_pro,
            display_name: "GPT-6.1 Sol Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61Terra {
            model_name: "openai/gpt-6.1-terra",
            constructor_name: openai_gpt_6_1_terra,
            display_name: "GPT-6.1 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61TerraPro {
            model_name: "openai/gpt-6.1-terra-pro",
            constructor_name: openai_gpt_6_1_terra_pro,
            display_name: "GPT-6.1 Terra Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptAudio {
            model_name: "openai/gpt-audio",
            constructor_name: openai_gpt_audio,
            display_name: "GPT-Audio",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptAudioMini {
            model_name: "openai/gpt-audio-mini",
            constructor_name: openai_gpt_audio_mini,
            display_name: "GPT-Audio Mini",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptChatLatest {
            model_name: "openai/gpt-chat-latest",
            constructor_name: openai_gpt_chat_latest,
            display_name: "GPT Chat Latest",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3 {
            model_name: "openai/o3",
            constructor_name: openai_o3,
            display_name: "o3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3Pro {
            model_name: "openai/o3-pro",
            constructor_name: openai_o3_pro,
            display_name: "o3-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO4Mini {
            model_name: "openai/o4-mini",
            constructor_name: openai_o4_mini,
            display_name: "o4 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenrouterAuto {
            model_name: "openrouter/auto",
            constructor_name: openrouter_auto,
            display_name: "Auto Router",
            capabilities: [AudioInputSupport, ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        OpenrouterBodybuilder {
            model_name: "openrouter/bodybuilder",
            constructor_name: openrouter_bodybuilder,
            display_name: "Body Builder (beta)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenrouterFree {
            model_name: "openrouter/free",
            constructor_name: openrouter_free,
            display_name: "Free Models Router",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenrouterFusion {
            model_name: "openrouter/fusion",
            constructor_name: openrouter_fusion,
            display_name: "Fusion",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenrouterParetoCode {
            model_name: "openrouter/pareto-code",
            constructor_name: openrouter_pareto_code,
            display_name: "Pareto Code Router",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        ParticleAiParticle1 {
            model_name: "particle-ai/particle-1",
            constructor_name: particle_ai_particle_1,
            display_name: "Particle-1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ParticleAiParticle1Free {
            model_name: "particle-ai/particle-1:free",
            constructor_name: particle_ai_particle_1_free,
            display_name: "Particle-1 (free)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        PerplexitySonar {
            model_name: "perplexity/sonar",
            constructor_name: perplexity_sonar,
            display_name: "Sonar",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        PerplexitySonarDeepResearch {
            model_name: "perplexity/sonar-deep-research",
            constructor_name: perplexity_sonar_deep_research,
            display_name: "Sonar Deep Research",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        PerplexitySonarPro {
            model_name: "perplexity/sonar-pro",
            constructor_name: perplexity_sonar_pro,
            display_name: "Sonar Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        PerplexitySonarProSearch {
            model_name: "perplexity/sonar-pro-search",
            constructor_name: perplexity_sonar_pro_search,
            display_name: "Sonar Pro Search",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        PerplexitySonarReasoningPro {
            model_name: "perplexity/sonar-reasoning-pro",
            constructor_name: perplexity_sonar_reasoning_pro,
            display_name: "Sonar Reasoning Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
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
            display_name: "Ternary Bonsai 2 9B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen2572bInstruct {
            model_name: "qwen/qwen-2.5-72b-instruct",
            constructor_name: qwen_qwen_2_5_72b_instruct,
            display_name: "Qwen2.5 72B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen257bInstruct {
            model_name: "qwen/qwen-2.5-7b-instruct",
            constructor_name: qwen_qwen_2_5_7b_instruct,
            display_name: "Qwen2.5 7B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        QwenQwenPlus {
            model_name: "qwen/qwen-plus",
            constructor_name: qwen_qwen_plus,
            display_name: "Qwen Plus",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen25Vl72bInstructFree {
            model_name: "qwen/qwen2.5-vl-72b-instruct:free",
            constructor_name: qwen_qwen2_5_vl_72b_instruct_free,
            display_name: "Qwen2.5 VL 72B Instruct (free)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen314b {
            model_name: "qwen/qwen3-14b",
            constructor_name: qwen_qwen3_14b,
            display_name: "Qwen3 14B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        QwenQwen3235bA22b2507 {
            model_name: "qwen/qwen3-235b-a22b-2507",
            constructor_name: qwen_qwen3_235b_a22b_2507,
            display_name: "Qwen3 235B A22B Instruct 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        QwenQwen330bA3b {
            model_name: "qwen/qwen3-30b-a3b",
            constructor_name: qwen_qwen3_30b_a3b,
            display_name: "Qwen3 30B A3B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3bInstruct2507 {
            model_name: "qwen/qwen3-30b-a3b-instruct-2507",
            constructor_name: qwen_qwen3_30b_a3b_instruct_2507,
            display_name: "Qwen3 30B A3B Instruct 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        QwenQwen332b {
            model_name: "qwen/qwen3-32b",
            constructor_name: qwen_qwen3_32b,
            display_name: "Qwen3 32B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderNext {
            model_name: "qwen/qwen3-coder-next",
            constructor_name: qwen_qwen3_coder_next,
            display_name: "Qwen3 Coder Next",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        QwenQwen3Vl235bA22bInstruct {
            model_name: "qwen/qwen3-vl-235b-a22b-instruct",
            constructor_name: qwen_qwen3_vl_235b_a22b_instruct,
            display_name: "Qwen3 VL 235B A22B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl30bA3bInstruct {
            model_name: "qwen/qwen3-vl-30b-a3b-instruct",
            constructor_name: qwen_qwen3_vl_30b_a3b_instruct,
            display_name: "Qwen3 VL 30B A3B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl30bA3bThinking {
            model_name: "qwen/qwen3-vl-30b-a3b-thinking",
            constructor_name: qwen_qwen3_vl_30b_a3b_thinking,
            display_name: "Qwen3 VL 30B A3B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl8bInstruct {
            model_name: "qwen/qwen3-vl-8b-instruct",
            constructor_name: qwen_qwen3_vl_8b_instruct,
            display_name: "Qwen3 VL 8B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen35122bA10b {
            model_name: "qwen/qwen3.5-122b-a10b",
            constructor_name: qwen_qwen3_5_122b_a10b,
            display_name: "Qwen3.5 122B-A10B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3527b {
            model_name: "qwen/qwen3.5-27b",
            constructor_name: qwen_qwen3_5_27b,
            display_name: "Qwen3.5 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3535bA3b {
            model_name: "qwen/qwen3.5-35b-a3b",
            constructor_name: qwen_qwen3_5_35b_a3b,
            display_name: "Qwen3.5 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen35397bA17b {
            model_name: "qwen/qwen3.5-397b-a17b",
            constructor_name: qwen_qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B-A17B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen35Flash0223 {
            model_name: "qwen/qwen3.5-flash-02-23",
            constructor_name: qwen_qwen3_5_flash_02_23,
            display_name: "Qwen3.5-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen35Plus0215 {
            model_name: "qwen/qwen3.5-plus-02-15",
            constructor_name: qwen_qwen3_5_plus_02_15,
            display_name: "Qwen3.5 Plus 2026-02-15",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen35Plus20260420 {
            model_name: "qwen/qwen3.5-plus-20260420",
            constructor_name: qwen_qwen3_5_plus_20260420,
            display_name: "Qwen3.5 Plus 2026-04-20",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3627b {
            model_name: "qwen/qwen3.6-27b",
            constructor_name: qwen_qwen3_6_27b,
            display_name: "Qwen3.6 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3635bA3b {
            model_name: "qwen/qwen3.6-35b-a3b",
            constructor_name: qwen_qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen36Flash {
            model_name: "qwen/qwen3.6-flash",
            constructor_name: qwen_qwen3_6_flash,
            display_name: "Qwen3.6 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen36Plus {
            model_name: "qwen/qwen3.6-plus",
            constructor_name: qwen_qwen3_6_plus,
            display_name: "Qwen3.6 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen37Flash {
            model_name: "qwen/qwen3.7-flash",
            constructor_name: qwen_qwen3_7_flash,
            display_name: "Qwen3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen37Max {
            model_name: "qwen/qwen3.7-max",
            constructor_name: qwen_qwen3_7_max,
            display_name: "Qwen3.7 Max",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen37Plus {
            model_name: "qwen/qwen3.7-plus",
            constructor_name: qwen_qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3824tA95b {
            model_name: "qwen/qwen3.8-2.4t-a95b",
            constructor_name: qwen_qwen3_8_2_4t_a95b,
            display_name: "Qwen3.8 2.4T A95B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827b {
            model_name: "qwen/qwen3.8-27b",
            constructor_name: qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen38Flash {
            model_name: "qwen/qwen3.8-flash",
            constructor_name: qwen_qwen3_8_flash,
            display_name: "Qwen3.8 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
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
            display_name: "Qwen 3.8 Max Prime",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen38OmniFlash {
            model_name: "qwen/qwen3.8-omni-flash",
            constructor_name: qwen_qwen3_8_omni_flash,
            display_name: "Qwen3.8 Omni Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwq32bFree {
            model_name: "qwen/qwq-32b:free",
            constructor_name: qwen_qwq_32b_free,
            display_name: "QwQ 32B (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RecursiveFlintV10Free {
            model_name: "recursive/flint-v1.0:free",
            constructor_name: recursive_flint_v1_0_free,
            display_name: "Flint (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ReflectionBeam10501bA23b {
            model_name: "reflection/beam-1.0-501b-a23b",
            constructor_name: reflection_beam_1_0_501b_a23b,
            display_name: "Beam 1.0 501B A23B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RekaaiRekaEdge {
            model_name: "rekaai/reka-edge",
            constructor_name: rekaai_reka_edge,
            display_name: "Reka Edge",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        RekaaiRekaFlash3 {
            model_name: "rekaai/reka-flash-3",
            constructor_name: rekaai_reka_flash_3,
            display_name: "Reka Flash 3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RelaceRelaceApply3 {
            model_name: "relace/relace-apply-3",
            constructor_name: relace_relace_apply_3,
            display_name: "Relace Apply 3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        RelaceRelaceSearch {
            model_name: "relace/relace-search",
            constructor_name: relace_relace_search,
            display_name: "Relace Search",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        SakanaFuguUltraV2 {
            model_name: "sakana/fugu-ultra-v2",
            constructor_name: sakana_fugu_ultra_v2,
            display_name: "Fugu Ultra v2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SakanaSakanaNamazu {
            model_name: "sakana/sakana-namazu",
            constructor_name: sakana_sakana_namazu,
            display_name: "Sakana Namazu",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Sao10kL3Lunaris8b {
            model_name: "sao10k/l3-lunaris-8b",
            constructor_name: sao10k_l3_lunaris_8b,
            display_name: "Llama 3 8B Lunaris",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Sao10kL31Euryale70b {
            model_name: "sao10k/l3.1-euryale-70b",
            constructor_name: sao10k_l3_1_euryale_70b,
            display_name: "Llama 3.1 Euryale 70B v2.2",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        SarvamaiSarvamMFree {
            model_name: "sarvamai/sarvam-m:free",
            constructor_name: sarvamai_sarvam_m_free,
            display_name: "Sarvam-M (free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ScaledownExtract {
            model_name: "scaledown/extract",
            constructor_name: scaledown_extract,
            display_name: "Extract",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ScaledownSummarize {
            model_name: "scaledown/summarize",
            constructor_name: scaledown_summarize,
            display_name: "Summarize",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
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
        StealthCroutonAlpha {
            model_name: "stealth/crouton-alpha",
            constructor_name: stealth_crouton_alpha,
            display_name: "Crouton Alpha",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        StepfunStep35Flash {
            model_name: "stepfun/step-3.5-flash",
            constructor_name: stepfun_step_3_5_flash,
            display_name: "Step 3.5 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        StepfunStep37Flash {
            model_name: "stepfun/step-3.7-flash",
            constructor_name: stepfun_step_3_7_flash,
            display_name: "Step 3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        StepfunStep5Preview {
            model_name: "stepfun/step-5-preview",
            constructor_name: stepfun_step_5_preview,
            display_name: "Step 5 Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        TencentHunyuanA13bInstruct {
            model_name: "tencent/hunyuan-a13b-instruct",
            constructor_name: tencent_hunyuan_a13b_instruct,
            display_name: "Hunyuan A13B Instruct",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        TencentHyMt218b {
            model_name: "tencent/hy-mt2-1.8b",
            constructor_name: tencent_hy_mt2_1_8b,
            display_name: "Hy-MT2-1.8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TencentHyMt230bA3b {
            model_name: "tencent/hy-mt2-30b-a3b",
            constructor_name: tencent_hy_mt2_30b_a3b,
            display_name: "Hy-MT2-30B-A3B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        TencentHyMt27b {
            model_name: "tencent/hy-mt2-7b",
            constructor_name: tencent_hy_mt2_7b,
            display_name: "Hy-MT2-7B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        TencentHy3 {
            model_name: "tencent/hy3",
            constructor_name: tencent_hy3,
            display_name: "Hy3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHy3Preview {
            model_name: "tencent/hy3-preview",
            constructor_name: tencent_hy3_preview,
            display_name: "Hy3 preview",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHy4 {
            model_name: "tencent/hy4",
            constructor_name: tencent_hy4,
            display_name: "Hy4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHy4Preview {
            model_name: "tencent/hy4-preview",
            constructor_name: tencent_hy4_preview,
            display_name: "Hy4 preview",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThedrummerCydonia24bV41 {
            model_name: "thedrummer/cydonia-24b-v4.1",
            constructor_name: thedrummer_cydonia_24b_v4_1,
            display_name: "Cydonia 24B V4.1",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        ThedrummerSkyfall36bV2 {
            model_name: "thedrummer/skyfall-36b-v2",
            constructor_name: thedrummer_skyfall_36b_v2,
            display_name: "Skyfall 36B V2",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        ThedrummerUnslopnemo12b {
            model_name: "thedrummer/unslopnemo-12b",
            constructor_name: thedrummer_unslopnemo_12b,
            display_name: "UnslopNemo 12B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        ThinkingmachinesInkling {
            model_name: "thinkingmachines/inkling",
            constructor_name: thinkingmachines_inkling,
            display_name: "Inkling",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInkling2Small {
            model_name: "thinkingmachines/inkling-2-small",
            constructor_name: thinkingmachines_inkling_2_small,
            display_name: "Inkling 2 Small",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInkling2SmallFree {
            model_name: "thinkingmachines/inkling-2-small:free",
            constructor_name: thinkingmachines_inkling_2_small_free,
            display_name: "Inkling 2 Small (free)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInklingSmall {
            model_name: "thinkingmachines/inkling-small",
            constructor_name: thinkingmachines_inkling_small,
            display_name: "Inkling Small",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        UnbiasedPareto2610 {
            model_name: "unbiased/pareto-26.10",
            constructor_name: unbiased_pareto_26_10,
            display_name: "Pareto 26.10",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UnbiasedPareto2610Preview {
            model_name: "unbiased/pareto-26.10-preview",
            constructor_name: unbiased_pareto_26_10_preview,
            display_name: "Pareto 26.10 Preview",
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
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        UpstageSolarMini4 {
            model_name: "upstage/solar-mini4",
            constructor_name: upstage_solar_mini4,
            display_name: "Solar Mini 4",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UpstageSolarPro4 {
            model_name: "upstage/solar-pro4",
            constructor_name: upstage_solar_pro4,
            display_name: "Solar Pro 4",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        WriterPalmyraX5 {
            model_name: "writer/palmyra-x5",
            constructor_name: writer_palmyra_x5,
            display_name: "Palmyra X5",
            capabilities: [TextInputSupport, TextOutputSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
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
        XAiGrokCodeFast1 {
            model_name: "x-ai/grok-code-fast-1",
            constructor_name: x_ai_grok_code_fast_1,
            display_name: "Grok Code Fast 1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV26Flash {
            model_name: "xiaomi/mimo-v2.6-flash",
            constructor_name: xiaomi_mimo_v2_6_flash,
            display_name: "MiMo-V2.6-Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XiaomiMimoV26Pro {
            model_name: "xiaomi/mimo-v2.6-pro",
            constructor_name: xiaomi_mimo_v2_6_pro,
            display_name: "MiMo-V2.6-Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XiaomiMimoV26ProUltraspeed {
            model_name: "xiaomi/mimo-v2.6-pro-ultraspeed",
            constructor_name: xiaomi_mimo_v2_6_pro_ultraspeed,
            display_name: "MiMo-V2.6-Pro-UltraSpeed",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XingchenAgiXing4029bA4bFree {
            model_name: "xingchen-agi/xing4.0-29b-a4b:free",
            constructor_name: xingchen_agi_xing4_0_29b_a4b_free,
            display_name: "Xing4.0 29B A4B (free)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm46Exacto {
            model_name: "z-ai/glm-4.6:exacto",
            constructor_name: z_ai_glm_4_6_exacto,
            display_name: "GLM 4.6 (exacto)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm46v {
            model_name: "z-ai/glm-4.6v",
            constructor_name: z_ai_glm_4_6v,
            display_name: "GLM-4.6V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZAiGlm47 {
            model_name: "z-ai/glm-4.7",
            constructor_name: z_ai_glm_4_7,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47Flash {
            model_name: "z-ai/glm-4.7-flash",
            constructor_name: z_ai_glm_4_7_flash,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5 {
            model_name: "z-ai/glm-5",
            constructor_name: z_ai_glm_5,
            display_name: "GLM-5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5Turbo {
            model_name: "z-ai/glm-5-turbo",
            constructor_name: z_ai_glm_5_turbo,
            display_name: "GLM-5-Turbo",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm51 {
            model_name: "z-ai/glm-5.1",
            constructor_name: z_ai_glm_5_1,
            display_name: "GLM-5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm52 {
            model_name: "z-ai/glm-5.2",
            constructor_name: z_ai_glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53 {
            model_name: "z-ai/glm-5.3",
            constructor_name: z_ai_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53Flash {
            model_name: "z-ai/glm-5.3-flash",
            constructor_name: z_ai_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZAiGlm53Flashx {
            model_name: "z-ai/glm-5.3-flashx",
            constructor_name: z_ai_glm_5_3_flashx,
            display_name: "GLM 5.3 FlashX",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZAiGlm53FlashxLegal {
            model_name: "z-ai/glm-5.3-flashx-legal",
            constructor_name: z_ai_glm_5_3_flashx_legal,
            display_name: "GLM 5.3 FlashX Legal",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZAiGlm53Prime {
            model_name: "z-ai/glm-5.3-prime",
            constructor_name: z_ai_glm_5_3_prime,
            display_name: "GLM 5.3 Prime",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5vTurbo {
            model_name: "z-ai/glm-5v-turbo",
            constructor_name: z_ai_glm_5v_turbo,
            display_name: "GLM-5V-Turbo",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AnthropicClaudeFableLatest {
            model_name: "~anthropic/claude-fable-latest",
            constructor_name: anthropic_claude_fable_latest,
            display_name: "Claude Fable Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaikuLatest {
            model_name: "~anthropic/claude-haiku-latest",
            constructor_name: anthropic_claude_haiku_latest,
            display_name: "Claude Haiku Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpusLatest {
            model_name: "~anthropic/claude-opus-latest",
            constructor_name: anthropic_claude_opus_latest,
            display_name: "Claude Opus Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnetLatest {
            model_name: "~anthropic/claude-sonnet-latest",
            constructor_name: anthropic_claude_sonnet_latest,
            display_name: "Claude Sonnet Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekFlashLatest {
            model_name: "~deepseek/deepseek-flash-latest",
            constructor_name: deepseek_deepseek_flash_latest,
            display_name: "DeepSeek Flash Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekProLatest {
            model_name: "~deepseek/deepseek-pro-latest",
            constructor_name: deepseek_deepseek_pro_latest,
            display_name: "DeepSeek Pro Latest",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4FlashLatest {
            model_name: "~deepseek/deepseek-v4-flash-latest",
            constructor_name: deepseek_deepseek_v4_flash_latest,
            display_name: "DeepSeek V4 Flash Latest",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGeminiFlashLatest {
            model_name: "~google/gemini-flash-latest",
            constructor_name: google_gemini_flash_latest,
            display_name: "Gemini Flash Latest",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGeminiProLatest {
            model_name: "~google/gemini-pro-latest",
            constructor_name: google_gemini_pro_latest,
            display_name: "Gemini Pro Latest",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiLatest {
            model_name: "~moonshotai/kimi-latest",
            constructor_name: moonshotai_kimi_latest,
            display_name: "Kimi Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        OpenaiGptAstraLatest {
            model_name: "~openai/gpt-astra-latest",
            constructor_name: openai_gpt_astra_latest,
            display_name: "GPT Astra Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptLunaLatest {
            model_name: "~openai/gpt-luna-latest",
            constructor_name: openai_gpt_luna_latest,
            display_name: "GPT Luna Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptMiniLatest {
            model_name: "~openai/gpt-mini-latest",
            constructor_name: openai_gpt_mini_latest,
            display_name: "GPT Mini Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptSolLatest {
            model_name: "~openai/gpt-sol-latest",
            constructor_name: openai_gpt_sol_latest,
            display_name: "GPT Sol Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptTerraLatest {
            model_name: "~openai/gpt-terra-latest",
            constructor_name: openai_gpt_terra_latest,
            display_name: "GPT Terra Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UnbiasedParetoLatest {
            model_name: "~unbiased/pareto-latest",
            constructor_name: unbiased_pareto_latest,
            display_name: "Pareto Latest",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrokLatest {
            model_name: "~x-ai/grok-latest",
            constructor_name: x_ai_grok_latest,
            display_name: "Grok Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlmFlashLatest {
            model_name: "~z-ai/glm-flash-latest",
            constructor_name: z_ai_glm_flash_latest,
            display_name: "GLM Flash Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZAiGlmLatest {
            model_name: "~z-ai/glm-latest",
            constructor_name: z_ai_glm_latest,
            display_name: "GLM Latest",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
