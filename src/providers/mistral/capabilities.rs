//! Capabilities for mistral models.
//!
//! This module defines model types and their capabilities for mistral providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::mistral::Mistral;

model_capabilities! {
    provider: Mistral,
    models: {
        Codestral2508 {
            model_name: "codestral-2508",
            constructor_name: codestral_2508,
            display_name: "Codestral 25.08",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CodestralLatest {
            model_name: "codestral-latest",
            constructor_name: codestral_latest,
            display_name: "Codestral",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash1017 {
            model_name: "deepseek-v4-1-flash-1017",
            constructor_name: deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Devstral2512 {
            model_name: "devstral-2512",
            constructor_name: devstral_2512,
            display_name: "Devstral 2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DevstralMedium2507 {
            model_name: "devstral-medium-2507",
            constructor_name: devstral_medium_2507,
            display_name: "Devstral Medium",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DevstralMediumLatest {
            model_name: "devstral-medium-latest",
            constructor_name: devstral_medium_latest,
            display_name: "Devstral 2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DevstralSmall2505 {
            model_name: "devstral-small-2505",
            constructor_name: devstral_small_2505,
            display_name: "Devstral Small 2505",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DevstralSmall2507 {
            model_name: "devstral-small-2507",
            constructor_name: devstral_small_2507,
            display_name: "Devstral Small",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LabsDevstralSmall2512 {
            model_name: "labs-devstral-small-2512",
            constructor_name: labs_devstral_small_2512,
            display_name: "Devstral Small 2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MagistralMediumLatest {
            model_name: "magistral-medium-latest",
            constructor_name: magistral_medium_latest,
            display_name: "Magistral Medium",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MagistralSmall {
            model_name: "magistral-small",
            constructor_name: magistral_small,
            display_name: "Magistral Small",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral14b2512 {
            model_name: "ministral-14b-2512",
            constructor_name: ministral_14b_2512,
            display_name: "Ministral 3 14B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral3b2512 {
            model_name: "ministral-3b-2512",
            constructor_name: ministral_3b_2512,
            display_name: "Ministral 3 3B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral3bLatest {
            model_name: "ministral-3b-latest",
            constructor_name: ministral_3b_latest,
            display_name: "Ministral 3B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral8b2512 {
            model_name: "ministral-8b-2512",
            constructor_name: ministral_8b_2512,
            display_name: "Ministral 3 8B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral8bLatest {
            model_name: "ministral-8b-latest",
            constructor_name: ministral_8b_latest,
            display_name: "Ministral 8B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralEmbed {
            model_name: "mistral-embed",
            constructor_name: mistral_embed,
            display_name: "Mistral Embed",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralLarge2411 {
            model_name: "mistral-large-2411",
            constructor_name: mistral_large_2411,
            display_name: "Mistral Large 2.1",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralLarge2512 {
            model_name: "mistral-large-2512",
            constructor_name: mistral_large_2512,
            display_name: "Mistral Large 3",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralLarge4 {
            model_name: "mistral-large-4",
            constructor_name: mistral_large_4,
            display_name: "Mistral Large 4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralLarge408 {
            model_name: "mistral-large-4-0-8",
            constructor_name: mistral_large_4_0_8,
            display_name: "Mistral Large 4.0.8",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralLargeLatest {
            model_name: "mistral-large-latest",
            constructor_name: mistral_large_latest,
            display_name: "Mistral Large",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMedium2505 {
            model_name: "mistral-medium-2505",
            constructor_name: mistral_medium_2505,
            display_name: "Mistral Medium 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMedium2508 {
            model_name: "mistral-medium-2508",
            constructor_name: mistral_medium_2508,
            display_name: "Mistral Medium 3.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMedium2604 {
            model_name: "mistral-medium-2604",
            constructor_name: mistral_medium_2604,
            display_name: "Mistral Medium 3.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMediumLatest {
            model_name: "mistral-medium-latest",
            constructor_name: mistral_medium_latest,
            display_name: "Mistral Medium",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralNemo {
            model_name: "mistral-nemo",
            constructor_name: mistral_nemo,
            display_name: "Mistral Nemo",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmall2506 {
            model_name: "mistral-small-2506",
            constructor_name: mistral_small_2506,
            display_name: "Mistral Small 3.2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmall2603 {
            model_name: "mistral-small-2603",
            constructor_name: mistral_small_2603,
            display_name: "Mistral Small 4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmallLatest {
            model_name: "mistral-small-latest",
            constructor_name: mistral_small_latest,
            display_name: "Mistral Small",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenMistral7b {
            model_name: "open-mistral-7b",
            constructor_name: open_mistral_7b,
            display_name: "Mistral 7B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenMixtral8x22b {
            model_name: "open-mixtral-8x22b",
            constructor_name: open_mixtral_8x22b,
            display_name: "Mixtral 8x22B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenMixtral8x7b {
            model_name: "open-mixtral-8x7b",
            constructor_name: open_mixtral_8x7b,
            display_name: "Mixtral 8x7B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Pixtral12b {
            model_name: "pixtral-12b",
            constructor_name: pixtral_12b,
            display_name: "Pixtral 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PixtralLargeLatest {
            model_name: "pixtral-large-latest",
            constructor_name: pixtral_large_latest,
            display_name: "Pixtral Large",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        VoxtralMiniLatest {
            model_name: "voxtral-mini-latest",
            constructor_name: voxtral_mini_latest,
            display_name: "Voxtral Mini (latest)",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        VoxtralMiniTtsLatest {
            model_name: "voxtral-mini-tts-latest",
            constructor_name: voxtral_mini_tts_latest,
            display_name: "Voxtral Mini TTS (latest)",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        VoxtralSmall2507 {
            model_name: "voxtral-small-2507",
            constructor_name: voxtral_small_2507,
            display_name: "Voxtral Small",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        VoxtralSmallLatest {
            model_name: "voxtral-small-latest",
            constructor_name: voxtral_small_latest,
            display_name: "Voxtral Small (latest)",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm53 {
            model_name: "zai-glm-5-3",
            constructor_name: zai_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm53Flash {
            model_name: "zai-glm-5-3-flash",
            constructor_name: zai_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
