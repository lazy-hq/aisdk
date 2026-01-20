//! Capabilities for amazonbedrock models.
//!
//! This module defines model types and their capabilities for amazonbedrock providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::amazon_bedrock::AmazonBedrock;

model_capabilities! {
    provider: AmazonBedrock,
    models: {
        Ai21Jamba15LargeV10 {
            model_name: "ai21.jamba-1-5-large-v1:0",
            constructor_name: ai21_jamba_1_5_large_v1_0,
            display_name: "Jamba 1.5 Large",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ai21Jamba15MiniV10 {
            model_name: "ai21.jamba-1-5-mini-v1:0",
            constructor_name: ai21_jamba_1_5_mini_v1_0,
            display_name: "Jamba 1.5 Mini",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AmazonNova2LiteV10 {
            model_name: "amazon.nova-2-lite-v1:0",
            constructor_name: amazon_nova_2_lite_v1_0,
            display_name: "Nova 2 Lite",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonNovaLiteV10 {
            model_name: "amazon.nova-lite-v1:0",
            constructor_name: amazon_nova_lite_v1_0,
            display_name: "Nova Lite",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonNovaMicroV10 {
            model_name: "amazon.nova-micro-v1:0",
            constructor_name: amazon_nova_micro_v1_0,
            display_name: "Nova Micro",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AmazonNovaPremierV10 {
            model_name: "amazon.nova-premier-v1:0",
            constructor_name: amazon_nova_premier_v1_0,
            display_name: "Nova Premier",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonNovaProV10 {
            model_name: "amazon.nova-pro-v1:0",
            constructor_name: amazon_nova_pro_v1_0,
            display_name: "Nova Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonTitanTextExpressV1 {
            model_name: "amazon.titan-text-express-v1",
            constructor_name: amazon_titan_text_express_v1,
            display_name: "Titan Text G1 - Express",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AmazonTitanTextExpressV108k {
            model_name: "amazon.titan-text-express-v1:0:8k",
            constructor_name: amazon_titan_text_express_v1_0_8k,
            display_name: "Titan Text G1 - Express",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude35Haiku20241022V10 {
            model_name: "anthropic.claude-3-5-haiku-20241022-v1:0",
            constructor_name: anthropic_claude_3_5_haiku_20241022_v1_0,
            display_name: "Claude Haiku 3.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude35Sonnet20240620V10 {
            model_name: "anthropic.claude-3-5-sonnet-20240620-v1:0",
            constructor_name: anthropic_claude_3_5_sonnet_20240620_v1_0,
            display_name: "Claude Sonnet 3.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude35Sonnet20241022V20 {
            model_name: "anthropic.claude-3-5-sonnet-20241022-v2:0",
            constructor_name: anthropic_claude_3_5_sonnet_20241022_v2_0,
            display_name: "Claude Sonnet 3.5 v2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude37Sonnet20250219V10 {
            model_name: "anthropic.claude-3-7-sonnet-20250219-v1:0",
            constructor_name: anthropic_claude_3_7_sonnet_20250219_v1_0,
            display_name: "Claude Sonnet 3.7",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude3Haiku20240307V10 {
            model_name: "anthropic.claude-3-haiku-20240307-v1:0",
            constructor_name: anthropic_claude_3_haiku_20240307_v1_0,
            display_name: "Claude Haiku 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude3Opus20240229V10 {
            model_name: "anthropic.claude-3-opus-20240229-v1:0",
            constructor_name: anthropic_claude_3_opus_20240229_v1_0,
            display_name: "Claude Opus 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude3Sonnet20240229V10 {
            model_name: "anthropic.claude-3-sonnet-20240229-v1:0",
            constructor_name: anthropic_claude_3_sonnet_20240229_v1_0,
            display_name: "Claude Sonnet 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku4520251001V10 {
            model_name: "anthropic.claude-haiku-4-5-20251001-v1:0",
            constructor_name: anthropic_claude_haiku_4_5_20251001_v1_0,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeInstantV1 {
            model_name: "anthropic.claude-instant-v1",
            constructor_name: anthropic_claude_instant_v1,
            display_name: "Claude Instant",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AnthropicClaudeOpus4120250805V10 {
            model_name: "anthropic.claude-opus-4-1-20250805-v1:0",
            constructor_name: anthropic_claude_opus_4_1_20250805_v1_0,
            display_name: "Claude Opus 4.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus420250514V10 {
            model_name: "anthropic.claude-opus-4-20250514-v1:0",
            constructor_name: anthropic_claude_opus_4_20250514_v1_0,
            display_name: "Claude Opus 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus4520251101V10 {
            model_name: "anthropic.claude-opus-4-5-20251101-v1:0",
            constructor_name: anthropic_claude_opus_4_5_20251101_v1_0,
            display_name: "Claude Opus 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet420250514V10 {
            model_name: "anthropic.claude-sonnet-4-20250514-v1:0",
            constructor_name: anthropic_claude_sonnet_4_20250514_v1_0,
            display_name: "Claude Sonnet 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet4520250929V10 {
            model_name: "anthropic.claude-sonnet-4-5-20250929-v1:0",
            constructor_name: anthropic_claude_sonnet_4_5_20250929_v1_0,
            display_name: "Claude Sonnet 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeV2 {
            model_name: "anthropic.claude-v2",
            constructor_name: anthropic_claude_v2,
            display_name: "Claude 2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AnthropicClaudeV21 {
            model_name: "anthropic.claude-v2:1",
            constructor_name: anthropic_claude_v2_1,
            display_name: "Claude 2.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereCommandLightTextV14 {
            model_name: "cohere.command-light-text-v14",
            constructor_name: cohere_command_light_text_v14,
            display_name: "Command Light",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereCommandRPlusV10 {
            model_name: "cohere.command-r-plus-v1:0",
            constructor_name: cohere_command_r_plus_v1_0,
            display_name: "Command R+",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereCommandRV10 {
            model_name: "cohere.command-r-v1:0",
            constructor_name: cohere_command_r_v1_0,
            display_name: "Command R",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereCommandTextV14 {
            model_name: "cohere.command-text-v14",
            constructor_name: cohere_command_text_v14,
            display_name: "Command",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekR1V10 {
            model_name: "deepseek.r1-v1:0",
            constructor_name: deepseek_r1_v1_0,
            display_name: "DeepSeek-R1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV3V10 {
            model_name: "deepseek.v3-v1:0",
            constructor_name: deepseek_v3_v1_0,
            display_name: "DeepSeek-V3.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GlobalAnthropicClaudeOpus4520251101V10 {
            model_name: "global.anthropic.claude-opus-4-5-20251101-v1:0",
            constructor_name: global_anthropic_claude_opus_4_5_20251101_v1_0,
            display_name: "Claude Opus 4.5 (Global)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma312bIt {
            model_name: "google.gemma-3-12b-it",
            constructor_name: google_gemma_3_12b_it,
            display_name: "Google Gemma 3 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma327bIt {
            model_name: "google.gemma-3-27b-it",
            constructor_name: google_gemma_3_27b_it,
            display_name: "Google Gemma 3 27B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma34bIt {
            model_name: "google.gemma-3-4b-it",
            constructor_name: google_gemma_3_4b_it,
            display_name: "Gemma 3 4B IT",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3170bInstructV10 {
            model_name: "meta.llama3-1-70b-instruct-v1:0",
            constructor_name: meta_llama3_1_70b_instruct_v1_0,
            display_name: "Llama 3.1 70B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama318bInstructV10 {
            model_name: "meta.llama3-1-8b-instruct-v1:0",
            constructor_name: meta_llama3_1_8b_instruct_v1_0,
            display_name: "Llama 3.1 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3211bInstructV10 {
            model_name: "meta.llama3-2-11b-instruct-v1:0",
            constructor_name: meta_llama3_2_11b_instruct_v1_0,
            display_name: "Llama 3.2 11B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama321bInstructV10 {
            model_name: "meta.llama3-2-1b-instruct-v1:0",
            constructor_name: meta_llama3_2_1b_instruct_v1_0,
            display_name: "Llama 3.2 1B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama323bInstructV10 {
            model_name: "meta.llama3-2-3b-instruct-v1:0",
            constructor_name: meta_llama3_2_3b_instruct_v1_0,
            display_name: "Llama 3.2 3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3290bInstructV10 {
            model_name: "meta.llama3-2-90b-instruct-v1:0",
            constructor_name: meta_llama3_2_90b_instruct_v1_0,
            display_name: "Llama 3.2 90B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3370bInstructV10 {
            model_name: "meta.llama3-3-70b-instruct-v1:0",
            constructor_name: meta_llama3_3_70b_instruct_v1_0,
            display_name: "Llama 3.3 70B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama370bInstructV10 {
            model_name: "meta.llama3-70b-instruct-v1:0",
            constructor_name: meta_llama3_70b_instruct_v1_0,
            display_name: "Llama 3 70B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama38bInstructV10 {
            model_name: "meta.llama3-8b-instruct-v1:0",
            constructor_name: meta_llama3_8b_instruct_v1_0,
            display_name: "Llama 3 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama4Maverick17bInstructV10 {
            model_name: "meta.llama4-maverick-17b-instruct-v1:0",
            constructor_name: meta_llama4_maverick_17b_instruct_v1_0,
            display_name: "Llama 4 Maverick 17B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama4Scout17bInstructV10 {
            model_name: "meta.llama4-scout-17b-instruct-v1:0",
            constructor_name: meta_llama4_scout_17b_instruct_v1_0,
            display_name: "Llama 4 Scout 17B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM2 {
            model_name: "minimax.minimax-m2",
            constructor_name: minimax_minimax_m2,
            display_name: "MiniMax M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMinistral314bInstruct {
            model_name: "mistral.ministral-3-14b-instruct",
            constructor_name: mistral_ministral_3_14b_instruct,
            display_name: "Ministral 14B 3.0",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMinistral38bInstruct {
            model_name: "mistral.ministral-3-8b-instruct",
            constructor_name: mistral_ministral_3_8b_instruct,
            display_name: "Ministral 3 8B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistral7bInstructV02 {
            model_name: "mistral.mistral-7b-instruct-v0:2",
            constructor_name: mistral_mistral_7b_instruct_v0_2,
            display_name: "Mistral-7B-Instruct-v0.3",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralLarge2402V10 {
            model_name: "mistral.mistral-large-2402-v1:0",
            constructor_name: mistral_mistral_large_2402_v1_0,
            display_name: "Mistral Large (24.02)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMixtral8x7bInstructV01 {
            model_name: "mistral.mixtral-8x7b-instruct-v0:1",
            constructor_name: mistral_mixtral_8x7b_instruct_v0_1,
            display_name: "Mixtral-8x7B-Instruct-v0.1",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralVoxtralMini3b2507 {
            model_name: "mistral.voxtral-mini-3b-2507",
            constructor_name: mistral_voxtral_mini_3b_2507,
            display_name: "Voxtral Mini 3B 2507",
            capabilities: [AudioInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralVoxtralSmall24b2507 {
            model_name: "mistral.voxtral-small-24b-2507",
            constructor_name: mistral_voxtral_small_24b_2507,
            display_name: "Voxtral Small 24B 2507",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotKimiK2Thinking {
            model_name: "moonshot.kimi-k2-thinking",
            constructor_name: moonshot_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNano12bV2 {
            model_name: "nvidia.nemotron-nano-12b-v2",
            constructor_name: nvidia_nemotron_nano_12b_v2,
            display_name: "NVIDIA Nemotron Nano 12B v2 VL BF16",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNano9bV2 {
            model_name: "nvidia.nemotron-nano-9b-v2",
            constructor_name: nvidia_nemotron_nano_9b_v2,
            display_name: "NVIDIA Nemotron Nano 9B v2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b10 {
            model_name: "openai.gpt-oss-120b-1:0",
            constructor_name: openai_gpt_oss_120b_1_0,
            display_name: "gpt-oss-120b",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b10 {
            model_name: "openai.gpt-oss-20b-1:0",
            constructor_name: openai_gpt_oss_20b_1_0,
            display_name: "gpt-oss-20b",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOssSafeguard120b {
            model_name: "openai.gpt-oss-safeguard-120b",
            constructor_name: openai_gpt_oss_safeguard_120b,
            display_name: "GPT OSS Safeguard 120B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOssSafeguard20b {
            model_name: "openai.gpt-oss-safeguard-20b",
            constructor_name: openai_gpt_oss_safeguard_20b,
            display_name: "GPT OSS Safeguard 20B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22b2507V10 {
            model_name: "qwen.qwen3-235b-a22b-2507-v1:0",
            constructor_name: qwen_qwen3_235b_a22b_2507_v1_0,
            display_name: "Qwen3 235B A22B 2507",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen332bV10 {
            model_name: "qwen.qwen3-32b-v1:0",
            constructor_name: qwen_qwen3_32b_v1_0,
            display_name: "Qwen3 32B (dense)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder30bA3bV10 {
            model_name: "qwen.qwen3-coder-30b-a3b-v1:0",
            constructor_name: qwen_qwen3_coder_30b_a3b_v1_0,
            display_name: "Qwen3 Coder 30B A3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder480bA35bV10 {
            model_name: "qwen.qwen3-coder-480b-a35b-v1:0",
            constructor_name: qwen_qwen3_coder_480b_a35b_v1_0,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3b {
            model_name: "qwen.qwen3-next-80b-a3b",
            constructor_name: qwen_qwen3_next_80b_a3b,
            display_name: "Qwen/Qwen3-Next-80B-A3B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl235bA22b {
            model_name: "qwen.qwen3-vl-235b-a22b",
            constructor_name: qwen_qwen3_vl_235b_a22b,
            display_name: "Qwen/Qwen3-VL-235B-A22B-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
