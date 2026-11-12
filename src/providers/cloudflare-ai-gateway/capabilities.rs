//! Capabilities for cloudflare_ai_gateway models.
//!
//! This module defines model types and their capabilities for cloudflare_ai_gateway providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::cloudflare_ai_gateway::CloudflareAiGateway;

model_capabilities! {
    provider: CloudflareAiGateway,
    models: {
        AlibabaQwen3Max {
            model_name: "alibaba/qwen3-max",
            constructor_name: alibaba_qwen3_max,
            display_name: "Qwen3 Max",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen35397bA17b {
            model_name: "alibaba/qwen3.5-397b-a17b",
            constructor_name: alibaba_qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B-A17B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AlibabaQwen37Max {
            model_name: "alibaba/qwen3.7-max",
            constructor_name: alibaba_qwen3_7_max,
            display_name: "Qwen3.7 Max",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen37Plus {
            model_name: "alibaba/qwen3.7-plus",
            constructor_name: alibaba_qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AlibabaQwen38Max {
            model_name: "alibaba/qwen3.8-max",
            constructor_name: alibaba_qwen3_8_max,
            display_name: "Qwen3.8 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AnthropicClaude35Haiku {
            model_name: "anthropic/claude-3-5-haiku",
            constructor_name: anthropic_claude_3_5_haiku,
            display_name: "Claude Haiku 3.5 (latest)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude3Haiku {
            model_name: "anthropic/claude-3-haiku",
            constructor_name: anthropic_claude_3_haiku,
            display_name: "Claude Haiku 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude3Opus {
            model_name: "anthropic/claude-3-opus",
            constructor_name: anthropic_claude_3_opus,
            display_name: "Claude Opus 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude3Sonnet {
            model_name: "anthropic/claude-3-sonnet",
            constructor_name: anthropic_claude_3_sonnet,
            display_name: "Claude Sonnet 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude35Sonnet {
            model_name: "anthropic/claude-3.5-sonnet",
            constructor_name: anthropic_claude_3_5_sonnet,
            display_name: "Claude Sonnet 3.5 v2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku45 {
            model_name: "anthropic/claude-haiku-4-5",
            constructor_name: anthropic_claude_haiku_4_5,
            display_name: "Claude Haiku 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            display_name: "Claude Opus 4 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus41 {
            model_name: "anthropic/claude-opus-4-1",
            constructor_name: anthropic_claude_opus_4_1,
            display_name: "Claude Opus 4.1 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus45 {
            model_name: "anthropic/claude-opus-4-5",
            constructor_name: anthropic_claude_opus_4_5,
            display_name: "Claude Opus 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46 {
            model_name: "anthropic/claude-opus-4-6",
            constructor_name: anthropic_claude_opus_4_6,
            display_name: "Claude Opus 4.6 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            display_name: "Claude Sonnet 4 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet45 {
            model_name: "anthropic/claude-sonnet-4-5",
            constructor_name: anthropic_claude_sonnet_4_5,
            display_name: "Claude Sonnet 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        DeepseekDeepseekV4Pro {
            model_name: "deepseek/deepseek-v4-pro",
            constructor_name: deepseek_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        FireworksEmber1 {
            model_name: "fireworks/ember-1",
            constructor_name: fireworks_ember_1,
            display_name: "Ember-1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini25Flash {
            model_name: "google/gemini-2.5-flash",
            constructor_name: google_gemini_2_5_flash,
            display_name: "Gemini 2.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25FlashLite {
            model_name: "google/gemini-2.5-flash-lite",
            constructor_name: google_gemini_2_5_flash_lite,
            display_name: "Gemini 2.5 Flash-Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25Pro {
            model_name: "google/gemini-2.5-pro",
            constructor_name: google_gemini_2_5_pro,
            display_name: "Gemini 2.5 Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini3Flash {
            model_name: "google/gemini-3-flash",
            constructor_name: google_gemini_3_flash,
            display_name: "Gemini 3 Flash Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31FlashLite {
            model_name: "google/gemini-3.1-flash-lite",
            constructor_name: google_gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31Pro {
            model_name: "google/gemini-3.1-pro",
            constructor_name: google_gemini_3_1_pro,
            display_name: "Gemini 3.1 Pro Preview",
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
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseEmber10 {
            model_name: "meta/muse-ember-1.0",
            constructor_name: meta_muse_ember_1_0,
            display_name: "Muse Ember",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSpark11 {
            model_name: "meta/muse-spark-1.1",
            constructor_name: meta_muse_spark_1_1,
            display_name: "Muse Spark 1.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSpark12 {
            model_name: "meta/muse-spark-1.2",
            constructor_name: meta_muse_spark_1_2,
            display_name: "Muse Spark 1.2",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSpark13 {
            model_name: "meta/muse-spark-1.3",
            constructor_name: meta_muse_spark_1_3,
            display_name: "Muse Spark 1.3",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseSparkOpen {
            model_name: "meta/muse-spark-open",
            constructor_name: meta_muse_spark_open,
            display_name: "Muse Spark Open",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaMuseWonder10 {
            model_name: "meta/muse-wonder-1.0",
            constructor_name: meta_muse_wonder_1_0,
            display_name: "Muse Wonder",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MinimaxM31Flash {
            model_name: "minimax/m3.1-flash",
            constructor_name: minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiK3 {
            model_name: "moonshotai/kimi-k3",
            constructor_name: moonshotai_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        OpenaiGpt35Turbo {
            model_name: "openai/gpt-3.5-turbo",
            constructor_name: openai_gpt_3_5_turbo,
            display_name: "GPT-3.5-turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt4 {
            model_name: "openai/gpt-4",
            constructor_name: openai_gpt_4,
            display_name: "GPT-4",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4Turbo {
            model_name: "openai/gpt-4-turbo",
            constructor_name: openai_gpt_4_turbo,
            display_name: "GPT-4 Turbo",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            display_name: "GPT-4.1 mini",
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
        OpenaiGpt4oMini {
            model_name: "openai/gpt-4o-mini",
            constructor_name: openai_gpt_4o_mini,
            display_name: "GPT-4o mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5 {
            model_name: "openai/gpt-5",
            constructor_name: openai_gpt_5,
            display_name: "GPT-5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        OpenaiGpt51 {
            model_name: "openai/gpt-5.1",
            constructor_name: openai_gpt_5_1,
            display_name: "GPT-5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Codex {
            model_name: "openai/gpt-5.1-codex",
            constructor_name: openai_gpt_5_1_codex,
            display_name: "GPT-5.1 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52 {
            model_name: "openai/gpt-5.2",
            constructor_name: openai_gpt_5_2,
            display_name: "GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54 {
            model_name: "openai/gpt-5.4",
            constructor_name: openai_gpt_5_4,
            display_name: "GPT-5.4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        OpenaiGpt56Sol {
            model_name: "openai/gpt-5.6-sol",
            constructor_name: openai_gpt_5_6_sol,
            display_name: "GPT-5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Terra {
            model_name: "openai/gpt-5.6-terra",
            constructor_name: openai_gpt_5_6_terra,
            display_name: "GPT-5.6 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Astra {
            model_name: "openai/gpt-6-astra",
            constructor_name: openai_gpt_6_astra,
            display_name: "GPT-6 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Luna {
            model_name: "openai/gpt-6-luna",
            constructor_name: openai_gpt_6_luna,
            display_name: "GPT-6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Sol {
            model_name: "openai/gpt-6-sol",
            constructor_name: openai_gpt_6_sol,
            display_name: "GPT-6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61Sol {
            model_name: "openai/gpt-6.1-sol",
            constructor_name: openai_gpt_6_1_sol,
            display_name: "GPT-6.1 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61Terra {
            model_name: "openai/gpt-6.1-terra",
            constructor_name: openai_gpt_6_1_terra,
            display_name: "GPT-6.1 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO1 {
            model_name: "openai/o1",
            constructor_name: openai_o1,
            display_name: "o1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3 {
            model_name: "openai/o3",
            constructor_name: openai_o3,
            display_name: "o3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3Mini {
            model_name: "openai/o3-mini",
            constructor_name: openai_o3_mini,
            display_name: "o3-mini",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            display_name: "o4-mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ReflectionAiBeam10501bA23b {
            model_name: "reflection-ai/beam-1.0-501b-a23b",
            constructor_name: reflection_ai_beam_1_0_501b_a23b,
            display_name: "Beam 1.0 501B A23B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UnbiasedPareto {
            model_name: "unbiased/pareto",
            constructor_name: unbiased_pareto,
            display_name: "Pareto",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        WorkersAiCfAi4bharatIndictrans2EnIndic1b {
            model_name: "workers-ai/@cf/ai4bharat/indictrans2-en-indic-1B",
            constructor_name: workers_ai_cf_ai4bharat_indictrans2_en_indic_1b,
            display_name: "IndicTrans2 EN-Indic 1B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfAisingaporeGemmaSeaLionV427bIt {
            model_name: "workers-ai/@cf/aisingapore/gemma-sea-lion-v4-27b-it",
            constructor_name: workers_ai_cf_aisingapore_gemma_sea_lion_v4_27b_it,
            display_name: "Gemma SEA-LION v4 27B IT",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfBaaiBgeBaseEnV15 {
            model_name: "workers-ai/@cf/baai/bge-base-en-v1.5",
            constructor_name: workers_ai_cf_baai_bge_base_en_v1_5,
            display_name: "BGE Base EN v1.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfBaaiBgeLargeEnV15 {
            model_name: "workers-ai/@cf/baai/bge-large-en-v1.5",
            constructor_name: workers_ai_cf_baai_bge_large_en_v1_5,
            display_name: "BGE Large EN v1.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfBaaiBgeM3 {
            model_name: "workers-ai/@cf/baai/bge-m3",
            constructor_name: workers_ai_cf_baai_bge_m3,
            display_name: "BGE M3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfBaaiBgeRerankerBase {
            model_name: "workers-ai/@cf/baai/bge-reranker-base",
            constructor_name: workers_ai_cf_baai_bge_reranker_base,
            display_name: "BGE Reranker Base",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfBaaiBgeSmallEnV15 {
            model_name: "workers-ai/@cf/baai/bge-small-en-v1.5",
            constructor_name: workers_ai_cf_baai_bge_small_en_v1_5,
            display_name: "BGE Small EN v1.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfDeepgramAura2En {
            model_name: "workers-ai/@cf/deepgram/aura-2-en",
            constructor_name: workers_ai_cf_deepgram_aura_2_en,
            display_name: "Deepgram Aura 2 (EN)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfDeepgramAura2Es {
            model_name: "workers-ai/@cf/deepgram/aura-2-es",
            constructor_name: workers_ai_cf_deepgram_aura_2_es,
            display_name: "Deepgram Aura 2 (ES)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfDeepgramNova3 {
            model_name: "workers-ai/@cf/deepgram/nova-3",
            constructor_name: workers_ai_cf_deepgram_nova_3,
            display_name: "Deepgram Nova 3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfDeepseekAiDeepseekR1DistillQwen32b {
            model_name: "workers-ai/@cf/deepseek-ai/deepseek-r1-distill-qwen-32b",
            constructor_name: workers_ai_cf_deepseek_ai_deepseek_r1_distill_qwen_32b,
            display_name: "DeepSeek R1 Distill Qwen 32B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfFacebookBartLargeCnn {
            model_name: "workers-ai/@cf/facebook/bart-large-cnn",
            constructor_name: workers_ai_cf_facebook_bart_large_cnn,
            display_name: "BART Large CNN",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfGoogleGemma312bIt {
            model_name: "workers-ai/@cf/google/gemma-3-12b-it",
            constructor_name: workers_ai_cf_google_gemma_3_12b_it,
            display_name: "Gemma 3 12B IT",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfHuggingfaceDistilbertSst2Int8 {
            model_name: "workers-ai/@cf/huggingface/distilbert-sst-2-int8",
            constructor_name: workers_ai_cf_huggingface_distilbert_sst_2_int8,
            display_name: "DistilBERT SST-2 INT8",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfIbmGraniteGranite40HMicro {
            model_name: "workers-ai/@cf/ibm-granite/granite-4.0-h-micro",
            constructor_name: workers_ai_cf_ibm_granite_granite_4_0_h_micro,
            display_name: "IBM Granite 4.0 H Micro",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlama27bChatFp16 {
            model_name: "workers-ai/@cf/meta/llama-2-7b-chat-fp16",
            constructor_name: workers_ai_cf_meta_llama_2_7b_chat_fp16,
            display_name: "Llama 2 7B Chat FP16",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlama38bInstruct {
            model_name: "workers-ai/@cf/meta/llama-3-8b-instruct",
            constructor_name: workers_ai_cf_meta_llama_3_8b_instruct,
            display_name: "Llama 3 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlama38bInstructAwq {
            model_name: "workers-ai/@cf/meta/llama-3-8b-instruct-awq",
            constructor_name: workers_ai_cf_meta_llama_3_8b_instruct_awq,
            display_name: "Llama 3 8B Instruct AWQ",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlama318bInstruct {
            model_name: "workers-ai/@cf/meta/llama-3.1-8b-instruct",
            constructor_name: workers_ai_cf_meta_llama_3_1_8b_instruct,
            display_name: "Llama 3.1 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlama318bInstructAwq {
            model_name: "workers-ai/@cf/meta/llama-3.1-8b-instruct-awq",
            constructor_name: workers_ai_cf_meta_llama_3_1_8b_instruct_awq,
            display_name: "Llama 3.1 8B Instruct AWQ",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlama318bInstructFp8 {
            model_name: "workers-ai/@cf/meta/llama-3.1-8b-instruct-fp8",
            constructor_name: workers_ai_cf_meta_llama_3_1_8b_instruct_fp8,
            display_name: "Llama 3.1 8B Instruct FP8",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlama3211bVisionInstruct {
            model_name: "workers-ai/@cf/meta/llama-3.2-11b-vision-instruct",
            constructor_name: workers_ai_cf_meta_llama_3_2_11b_vision_instruct,
            display_name: "Llama 3.2 11B Vision Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlama321bInstruct {
            model_name: "workers-ai/@cf/meta/llama-3.2-1b-instruct",
            constructor_name: workers_ai_cf_meta_llama_3_2_1b_instruct,
            display_name: "Llama 3.2 1B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlama323bInstruct {
            model_name: "workers-ai/@cf/meta/llama-3.2-3b-instruct",
            constructor_name: workers_ai_cf_meta_llama_3_2_3b_instruct,
            display_name: "Llama 3.2 3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlama3370bInstructFp8Fast {
            model_name: "workers-ai/@cf/meta/llama-3.3-70b-instruct-fp8-fast",
            constructor_name: workers_ai_cf_meta_llama_3_3_70b_instruct_fp8_fast,
            display_name: "Llama 3.3 70B Instruct FP8 Fast",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlama4Scout17b16eInstruct {
            model_name: "workers-ai/@cf/meta/llama-4-scout-17b-16e-instruct",
            constructor_name: workers_ai_cf_meta_llama_4_scout_17b_16e_instruct,
            display_name: "Llama 4 Scout 17B 16E Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaLlamaGuard38b {
            model_name: "workers-ai/@cf/meta/llama-guard-3-8b",
            constructor_name: workers_ai_cf_meta_llama_guard_3_8b,
            display_name: "Llama Guard 3 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMetaM2m10012b {
            model_name: "workers-ai/@cf/meta/m2m100-1.2b",
            constructor_name: workers_ai_cf_meta_m2m100_1_2b,
            display_name: "M2M100 1.2B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMistralMistral7bInstructV01 {
            model_name: "workers-ai/@cf/mistral/mistral-7b-instruct-v0.1",
            constructor_name: workers_ai_cf_mistral_mistral_7b_instruct_v0_1,
            display_name: "Mistral 7B Instruct v0.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMistralaiMistralSmall3124bInstruct {
            model_name: "workers-ai/@cf/mistralai/mistral-small-3.1-24b-instruct",
            constructor_name: workers_ai_cf_mistralai_mistral_small_3_1_24b_instruct,
            display_name: "Mistral Small 3.1 24B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfMyshellAiMelotts {
            model_name: "workers-ai/@cf/myshell-ai/melotts",
            constructor_name: workers_ai_cf_myshell_ai_melotts,
            display_name: "MyShell MeloTTS",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfOpenaiGptOss120b {
            model_name: "workers-ai/@cf/openai/gpt-oss-120b",
            constructor_name: workers_ai_cf_openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfOpenaiGptOss20b {
            model_name: "workers-ai/@cf/openai/gpt-oss-20b",
            constructor_name: workers_ai_cf_openai_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfPfnetPlamoEmbedding1b {
            model_name: "workers-ai/@cf/pfnet/plamo-embedding-1b",
            constructor_name: workers_ai_cf_pfnet_plamo_embedding_1b,
            display_name: "PLaMo Embedding 1B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfPipecatAiSmartTurnV2 {
            model_name: "workers-ai/@cf/pipecat-ai/smart-turn-v2",
            constructor_name: workers_ai_cf_pipecat_ai_smart_turn_v2,
            display_name: "Pipecat Smart Turn v2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfQwenQwen25Coder32bInstruct {
            model_name: "workers-ai/@cf/qwen/qwen2.5-coder-32b-instruct",
            constructor_name: workers_ai_cf_qwen_qwen2_5_coder_32b_instruct,
            display_name: "Qwen 2.5 Coder 32B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfQwenQwen330bA3bFp8 {
            model_name: "workers-ai/@cf/qwen/qwen3-30b-a3b-fp8",
            constructor_name: workers_ai_cf_qwen_qwen3_30b_a3b_fp8,
            display_name: "Qwen3 30B A3B FP8",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfQwenQwen3Embedding06b {
            model_name: "workers-ai/@cf/qwen/qwen3-embedding-0.6b",
            constructor_name: workers_ai_cf_qwen_qwen3_embedding_0_6b,
            display_name: "Qwen3 Embedding 0.6B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        WorkersAiCfQwenQwq32b {
            model_name: "workers-ai/@cf/qwen/qwq-32b",
            constructor_name: workers_ai_cf_qwen_qwq_32b,
            display_name: "QwQ 32B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        XaiGrok4200309NonReasoning {
            model_name: "xai/grok-4.20-0309-non-reasoning",
            constructor_name: xai_grok_4_20_0309_non_reasoning,
            display_name: "Grok 4.20 (Non-Reasoning)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok4200309Reasoning {
            model_name: "xai/grok-4.20-0309-reasoning",
            constructor_name: xai_grok_4_20_0309_reasoning,
            display_name: "Grok 4.20 (Reasoning)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok43 {
            model_name: "xai/grok-4.3",
            constructor_name: xai_grok_4_3,
            display_name: "Grok 4.3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok45 {
            model_name: "xai/grok-4.5",
            constructor_name: xai_grok_4_5,
            display_name: "Grok 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok46 {
            model_name: "xai/grok-4.6",
            constructor_name: xai_grok_4_6,
            display_name: "Grok 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok47 {
            model_name: "xai/grok-4.7",
            constructor_name: xai_grok_4_7,
            display_name: "Grok 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
