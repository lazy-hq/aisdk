//! Capabilities for helicone models.
//!
//! This module defines model types and their capabilities for helicone providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::helicone::Helicone;

model_capabilities! {
    provider: Helicone,
    models: {
        Chatgpt4oLatest {
            model_name: "chatgpt-4o-latest",
            constructor_name: chatgpt_4o_latest,
            display_name: "OpenAI ChatGPT-4o",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude3Haiku20240307 {
            model_name: "claude-3-haiku-20240307",
            constructor_name: claude_3_haiku_20240307,
            display_name: "Anthropic: Claude 3 Haiku",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude35Haiku {
            model_name: "claude-3.5-haiku",
            constructor_name: claude_3_5_haiku,
            display_name: "Anthropic: Claude 3.5 Haiku",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude35SonnetV2 {
            model_name: "claude-3.5-sonnet-v2",
            constructor_name: claude_3_5_sonnet_v2,
            display_name: "Anthropic: Claude 3.5 Sonnet v2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude37Sonnet {
            model_name: "claude-3.7-sonnet",
            constructor_name: claude_3_7_sonnet,
            display_name: "Anthropic: Claude 3.7 Sonnet",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude45Haiku {
            model_name: "claude-4.5-haiku",
            constructor_name: claude_4_5_haiku,
            display_name: "Anthropic: Claude 4.5 Haiku",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude45Opus {
            model_name: "claude-4.5-opus",
            constructor_name: claude_4_5_opus,
            display_name: "Anthropic: Claude Opus 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude45Sonnet {
            model_name: "claude-4.5-sonnet",
            constructor_name: claude_4_5_sonnet,
            display_name: "Anthropic: Claude Sonnet 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku4520251001 {
            model_name: "claude-haiku-4-5-20251001",
            constructor_name: claude_haiku_4_5_20251001,
            display_name: "Anthropic: Claude 4.5 Haiku (20251001)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4 {
            model_name: "claude-opus-4",
            constructor_name: claude_opus_4,
            display_name: "Anthropic: Claude Opus 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus41 {
            model_name: "claude-opus-4-1",
            constructor_name: claude_opus_4_1,
            display_name: "Anthropic: Claude Opus 4.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4120250805 {
            model_name: "claude-opus-4-1-20250805",
            constructor_name: claude_opus_4_1_20250805,
            display_name: "Anthropic: Claude Opus 4.1 (20250805)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4 {
            model_name: "claude-sonnet-4",
            constructor_name: claude_sonnet_4,
            display_name: "Anthropic: Claude Sonnet 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4520250929 {
            model_name: "claude-sonnet-4-5-20250929",
            constructor_name: claude_sonnet_4_5_20250929,
            display_name: "Anthropic: Claude Sonnet 4.5 (20250929)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CodexMiniLatest {
            model_name: "codex-mini-latest",
            constructor_name: codex_mini_latest,
            display_name: "OpenAI Codex Mini Latest",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR1DistillLlama70b {
            model_name: "deepseek-r1-distill-llama-70b",
            constructor_name: deepseek_r1_distill_llama_70b,
            display_name: "DeepSeek R1 Distill Llama 70B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekReasoner {
            model_name: "deepseek-reasoner",
            constructor_name: deepseek_reasoner,
            display_name: "DeepSeek Reasoner",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekTngR1t2Chimera {
            model_name: "deepseek-tng-r1t2-chimera",
            constructor_name: deepseek_tng_r1t2_chimera,
            display_name: "DeepSeek TNG R1T2 Chimera",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV3 {
            model_name: "deepseek-v3",
            constructor_name: deepseek_v3,
            display_name: "DeepSeek V3",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV31Terminus {
            model_name: "deepseek-v3.1-terminus",
            constructor_name: deepseek_v3_1_terminus,
            display_name: "DeepSeek V3.1 Terminus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV32 {
            model_name: "deepseek-v3.2",
            constructor_name: deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ernie4521bA3bThinking {
            model_name: "ernie-4.5-21b-a3b-thinking",
            constructor_name: ernie_4_5_21b_a3b_thinking,
            display_name: "Baidu Ernie 4.5 21B A3B Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25Flash {
            model_name: "gemini-2.5-flash",
            constructor_name: gemini_2_5_flash,
            display_name: "Google Gemini 2.5 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashLite {
            model_name: "gemini-2.5-flash-lite",
            constructor_name: gemini_2_5_flash_lite,
            display_name: "Google Gemini 2.5 Flash Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25Pro {
            model_name: "gemini-2.5-pro",
            constructor_name: gemini_2_5_pro,
            display_name: "Google Gemini 2.5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini3ProPreview {
            model_name: "gemini-3-pro-preview",
            constructor_name: gemini_3_pro_preview,
            display_name: "Google Gemini 3 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemma312bIt {
            model_name: "gemma-3-12b-it",
            constructor_name: gemma_3_12b_it,
            display_name: "Google Gemma 3 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemma29bIt {
            model_name: "gemma2-9b-it",
            constructor_name: gemma2_9b_it,
            display_name: "Google Gemma 2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm46 {
            model_name: "glm-4.6",
            constructor_name: glm_4_6,
            display_name: "Zai GLM-4.6",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41 {
            model_name: "gpt-4.1",
            constructor_name: gpt_4_1,
            display_name: "OpenAI GPT-4.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Mini {
            model_name: "gpt-4.1-mini",
            constructor_name: gpt_4_1_mini,
            display_name: "OpenAI GPT-4.1 Mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Mini20250414 {
            model_name: "gpt-4.1-mini-2025-04-14",
            constructor_name: gpt_4_1_mini_2025_04_14,
            display_name: "OpenAI GPT-4.1 Mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Nano {
            model_name: "gpt-4.1-nano",
            constructor_name: gpt_4_1_nano,
            display_name: "OpenAI GPT-4.1 Nano",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4o {
            model_name: "gpt-4o",
            constructor_name: gpt_4o,
            display_name: "OpenAI GPT-4o",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4oMini {
            model_name: "gpt-4o-mini",
            constructor_name: gpt_4o_mini,
            display_name: "OpenAI GPT-4o-mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5 {
            model_name: "gpt-5",
            constructor_name: gpt_5,
            display_name: "OpenAI GPT-5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5ChatLatest {
            model_name: "gpt-5-chat-latest",
            constructor_name: gpt_5_chat_latest,
            display_name: "OpenAI GPT-5 Chat Latest",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Codex {
            model_name: "gpt-5-codex",
            constructor_name: gpt_5_codex,
            display_name: "OpenAI: GPT-5 Codex",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Mini {
            model_name: "gpt-5-mini",
            constructor_name: gpt_5_mini,
            display_name: "OpenAI GPT-5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Nano {
            model_name: "gpt-5-nano",
            constructor_name: gpt_5_nano,
            display_name: "OpenAI GPT-5 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Pro {
            model_name: "gpt-5-pro",
            constructor_name: gpt_5_pro,
            display_name: "OpenAI: GPT-5 Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51 {
            model_name: "gpt-5.1",
            constructor_name: gpt_5_1,
            display_name: "OpenAI GPT-5.1",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51ChatLatest {
            model_name: "gpt-5.1-chat-latest",
            constructor_name: gpt_5_1_chat_latest,
            display_name: "OpenAI GPT-5.1 Chat",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51Codex {
            model_name: "gpt-5.1-codex",
            constructor_name: gpt_5_1_codex,
            display_name: "OpenAI: GPT-5.1 Codex",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51CodexMini {
            model_name: "gpt-5.1-codex-mini",
            constructor_name: gpt_5_1_codex_mini,
            display_name: "OpenAI: GPT-5.1 Codex Mini",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "OpenAI GPT-OSS 120b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss20b {
            model_name: "gpt-oss-20b",
            constructor_name: gpt_oss_20b,
            display_name: "OpenAI GPT-OSS 20b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok3 {
            model_name: "grok-3",
            constructor_name: grok_3,
            display_name: "xAI Grok 3",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok3Mini {
            model_name: "grok-3-mini",
            constructor_name: grok_3_mini,
            display_name: "xAI Grok 3 Mini",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4 {
            model_name: "grok-4",
            constructor_name: grok_4,
            display_name: "xAI Grok 4",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41FastNonReasoning {
            model_name: "grok-4-1-fast-non-reasoning",
            constructor_name: grok_4_1_fast_non_reasoning,
            display_name: "xAI Grok 4.1 Fast Non-Reasoning",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41FastReasoning {
            model_name: "grok-4-1-fast-reasoning",
            constructor_name: grok_4_1_fast_reasoning,
            display_name: "xAI Grok 4.1 Fast Reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4FastNonReasoning {
            model_name: "grok-4-fast-non-reasoning",
            constructor_name: grok_4_fast_non_reasoning,
            display_name: "xAI Grok 4 Fast Non-Reasoning",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4FastReasoning {
            model_name: "grok-4-fast-reasoning",
            constructor_name: grok_4_fast_reasoning,
            display_name: "xAI: Grok 4 Fast Reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GrokCodeFast1 {
            model_name: "grok-code-fast-1",
            constructor_name: grok_code_fast_1,
            display_name: "xAI Grok Code Fast 1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hermes2ProLlama38b {
            model_name: "hermes-2-pro-llama-3-8b",
            constructor_name: hermes_2_pro_llama_3_8b,
            display_name: "Hermes 2 Pro Llama 3 8B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK20711 {
            model_name: "kimi-k2-0711",
            constructor_name: kimi_k2_0711,
            display_name: "Kimi K2 (07/11)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK20905 {
            model_name: "kimi-k2-0905",
            constructor_name: kimi_k2_0905,
            display_name: "Kimi K2 (09/05)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2Thinking {
            model_name: "kimi-k2-thinking",
            constructor_name: kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama318bInstant {
            model_name: "llama-3.1-8b-instant",
            constructor_name: llama_3_1_8b_instant,
            display_name: "Meta Llama 3.1 8B Instant",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama318bInstruct {
            model_name: "llama-3.1-8b-instruct",
            constructor_name: llama_3_1_8b_instruct,
            display_name: "Meta Llama 3.1 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama318bInstructTurbo {
            model_name: "llama-3.1-8b-instruct-turbo",
            constructor_name: llama_3_1_8b_instruct_turbo,
            display_name: "Meta Llama 3.1 8B Instruct Turbo",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama3370bInstruct {
            model_name: "llama-3.3-70b-instruct",
            constructor_name: llama_3_3_70b_instruct,
            display_name: "Meta Llama 3.3 70B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama3370bVersatile {
            model_name: "llama-3.3-70b-versatile",
            constructor_name: llama_3_3_70b_versatile,
            display_name: "Meta Llama 3.3 70B Versatile",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama4Maverick {
            model_name: "llama-4-maverick",
            constructor_name: llama_4_maverick,
            display_name: "Meta Llama 4 Maverick 17B 128E",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama4Scout {
            model_name: "llama-4-scout",
            constructor_name: llama_4_scout,
            display_name: "Meta Llama 4 Scout 17B 16E",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LlamaGuard4 {
            model_name: "llama-guard-4",
            constructor_name: llama_guard_4,
            display_name: "Meta Llama Guard 4 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        LlamaPromptGuard222m {
            model_name: "llama-prompt-guard-2-22m",
            constructor_name: llama_prompt_guard_2_22m,
            display_name: "Meta Llama Prompt Guard 2 22M",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        LlamaPromptGuard286m {
            model_name: "llama-prompt-guard-2-86m",
            constructor_name: llama_prompt_guard_2_86m,
            display_name: "Meta Llama Prompt Guard 2 86M",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralLarge2411 {
            model_name: "mistral-large-2411",
            constructor_name: mistral_large_2411,
            display_name: "Mistral-Large",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralNemo {
            model_name: "mistral-nemo",
            constructor_name: mistral_nemo,
            display_name: "Mistral Nemo",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralSmall {
            model_name: "mistral-small",
            constructor_name: mistral_small,
            display_name: "Mistral Small",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O1 {
            model_name: "o1",
            constructor_name: o1,
            display_name: "OpenAI: o1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        O1Mini {
            model_name: "o1-mini",
            constructor_name: o1_mini,
            display_name: "OpenAI: o1-mini",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        O3 {
            model_name: "o3",
            constructor_name: o3,
            display_name: "OpenAI o3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O3Mini {
            model_name: "o3-mini",
            constructor_name: o3_mini,
            display_name: "OpenAI o3 Mini",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O3Pro {
            model_name: "o3-pro",
            constructor_name: o3_pro,
            display_name: "OpenAI o3 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O4Mini {
            model_name: "o4-mini",
            constructor_name: o4_mini,
            display_name: "OpenAI o4 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen25Coder7bFast {
            model_name: "qwen2.5-coder-7b-fast",
            constructor_name: qwen2_5_coder_7b_fast,
            display_name: "Qwen2.5 Coder 7B fast",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen3235bA22bThinking {
            model_name: "qwen3-235b-a22b-thinking",
            constructor_name: qwen3_235b_a22b_thinking,
            display_name: "Qwen3 235B A22B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen330bA3b {
            model_name: "qwen3-30b-a3b",
            constructor_name: qwen3_30b_a3b,
            display_name: "Qwen3 30B A3B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen332b {
            model_name: "qwen3-32b",
            constructor_name: qwen3_32b,
            display_name: "Qwen3 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder {
            model_name: "qwen3-coder",
            constructor_name: qwen3_coder,
            display_name: "Qwen3 Coder 480B A35B Instruct Turbo",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3Coder30bA3bInstruct {
            model_name: "qwen3-coder-30b-a3b-instruct",
            constructor_name: qwen3_coder_30b_a3b_instruct,
            display_name: "Qwen3 Coder 30B A3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Next80bA3bInstruct {
            model_name: "qwen3-next-80b-a3b-instruct",
            constructor_name: qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3 Next 80B A3B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3Vl235bA22bInstruct {
            model_name: "qwen3-vl-235b-a22b-instruct",
            constructor_name: qwen3_vl_235b_a22b_instruct,
            display_name: "Qwen3 VL 235B A22B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Sonar {
            model_name: "sonar",
            constructor_name: sonar,
            display_name: "Perplexity Sonar",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SonarDeepResearch {
            model_name: "sonar-deep-research",
            constructor_name: sonar_deep_research,
            display_name: "Perplexity Sonar Deep Research",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        SonarPro {
            model_name: "sonar-pro",
            constructor_name: sonar_pro,
            display_name: "Perplexity Sonar Pro",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SonarReasoning {
            model_name: "sonar-reasoning",
            constructor_name: sonar_reasoning,
            display_name: "Perplexity Sonar Reasoning",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        SonarReasoningPro {
            model_name: "sonar-reasoning-pro",
            constructor_name: sonar_reasoning_pro,
            display_name: "Perplexity Sonar Reasoning Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
    }
}
