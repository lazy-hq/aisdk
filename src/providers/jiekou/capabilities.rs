//! Capabilities for jiekou models.
//!
//! This module defines model types and their capabilities for jiekou providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::jiekou::Jiekou;

model_capabilities! {
    provider: Jiekou,
    models: {
        BaiduErnie45300bA47bPaddle {
            model_name: "baidu/ernie-4.5-300b-a47b-paddle",
            constructor_name: baidu_ernie_4_5_300b_a47b_paddle,
            display_name: "ERNIE 4.5 300B A47B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BaiduErnie45Vl424bA47b {
            model_name: "baidu/ernie-4.5-vl-424b-a47b",
            constructor_name: baidu_ernie_4_5_vl_424b_a47b,
            display_name: "ERNIE 4.5 VL 424B A47B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku4520251001 {
            model_name: "claude-haiku-4-5-20251001",
            constructor_name: claude_haiku_4_5_20251001,
            display_name: "claude-haiku-4-5-20251001",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4120250805 {
            model_name: "claude-opus-4-1-20250805",
            constructor_name: claude_opus_4_1_20250805,
            display_name: "claude-opus-4-1-20250805",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus420250514 {
            model_name: "claude-opus-4-20250514",
            constructor_name: claude_opus_4_20250514,
            display_name: "claude-opus-4-20250514",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4520251101 {
            model_name: "claude-opus-4-5-20251101",
            constructor_name: claude_opus_4_5_20251101,
            display_name: "claude-opus-4-5-20251101",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus46 {
            model_name: "claude-opus-4-6",
            constructor_name: claude_opus_4_6,
            display_name: "claude-opus-4-6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet420250514 {
            model_name: "claude-sonnet-4-20250514",
            constructor_name: claude_sonnet_4_20250514,
            display_name: "claude-sonnet-4-20250514",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4520250929 {
            model_name: "claude-sonnet-4-5-20250929",
            constructor_name: claude_sonnet_4_5_20250929,
            display_name: "claude-sonnet-4-5-20250929",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekR10528 {
            model_name: "deepseek/deepseek-r1-0528",
            constructor_name: deepseek_deepseek_r1_0528,
            display_name: "DeepSeek R1 0528",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV30324 {
            model_name: "deepseek/deepseek-v3-0324",
            constructor_name: deepseek_deepseek_v3_0324,
            display_name: "DeepSeek V3 0324",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV31 {
            model_name: "deepseek/deepseek-v3.1",
            constructor_name: deepseek_deepseek_v3_1,
            display_name: "DeepSeek V3.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25Flash {
            model_name: "gemini-2.5-flash",
            constructor_name: gemini_2_5_flash,
            display_name: "gemini-2.5-flash",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashLite {
            model_name: "gemini-2.5-flash-lite",
            constructor_name: gemini_2_5_flash_lite,
            display_name: "gemini-2.5-flash-lite",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashLitePreview0617 {
            model_name: "gemini-2.5-flash-lite-preview-06-17",
            constructor_name: gemini_2_5_flash_lite_preview_06_17,
            display_name: "gemini-2.5-flash-lite-preview-06-17",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashLitePreview092025 {
            model_name: "gemini-2.5-flash-lite-preview-09-2025",
            constructor_name: gemini_2_5_flash_lite_preview_09_2025,
            display_name: "gemini-2.5-flash-lite-preview-09-2025",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashPreview0520 {
            model_name: "gemini-2.5-flash-preview-05-20",
            constructor_name: gemini_2_5_flash_preview_05_20,
            display_name: "gemini-2.5-flash-preview-05-20",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25Pro {
            model_name: "gemini-2.5-pro",
            constructor_name: gemini_2_5_pro,
            display_name: "gemini-2.5-pro",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25ProPreview0605 {
            model_name: "gemini-2.5-pro-preview-06-05",
            constructor_name: gemini_2_5_pro_preview_06_05,
            display_name: "gemini-2.5-pro-preview-06-05",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini3FlashPreview {
            model_name: "gemini-3-flash-preview",
            constructor_name: gemini_3_flash_preview,
            display_name: "gemini-3-flash-preview",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini3ProPreview {
            model_name: "gemini-3-pro-preview",
            constructor_name: gemini_3_pro_preview,
            display_name: "gemini-3-pro-preview",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gpt5ChatLatest {
            model_name: "gpt-5-chat-latest",
            constructor_name: gpt_5_chat_latest,
            display_name: "gpt-5-chat-latest",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Codex {
            model_name: "gpt-5-codex",
            constructor_name: gpt_5_codex,
            display_name: "gpt-5-codex",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Mini {
            model_name: "gpt-5-mini",
            constructor_name: gpt_5_mini,
            display_name: "gpt-5-mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Nano {
            model_name: "gpt-5-nano",
            constructor_name: gpt_5_nano,
            display_name: "gpt-5-nano",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Pro {
            model_name: "gpt-5-pro",
            constructor_name: gpt_5_pro,
            display_name: "gpt-5-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51 {
            model_name: "gpt-5.1",
            constructor_name: gpt_5_1,
            display_name: "gpt-5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51Codex {
            model_name: "gpt-5.1-codex",
            constructor_name: gpt_5_1_codex,
            display_name: "gpt-5.1-codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51CodexMax {
            model_name: "gpt-5.1-codex-max",
            constructor_name: gpt_5_1_codex_max,
            display_name: "gpt-5.1-codex-max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51CodexMini {
            model_name: "gpt-5.1-codex-mini",
            constructor_name: gpt_5_1_codex_mini,
            display_name: "gpt-5.1-codex-mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52 {
            model_name: "gpt-5.2",
            constructor_name: gpt_5_2,
            display_name: "gpt-5.2",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52Codex {
            model_name: "gpt-5.2-codex",
            constructor_name: gpt_5_2_codex,
            display_name: "gpt-5.2-codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52Pro {
            model_name: "gpt-5.2-pro",
            constructor_name: gpt_5_2_pro,
            display_name: "gpt-5.2-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok40709 {
            model_name: "grok-4-0709",
            constructor_name: grok_4_0709,
            display_name: "grok-4-0709",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41FastNonReasoning {
            model_name: "grok-4-1-fast-non-reasoning",
            constructor_name: grok_4_1_fast_non_reasoning,
            display_name: "grok-4-1-fast-non-reasoning",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41FastReasoning {
            model_name: "grok-4-1-fast-reasoning",
            constructor_name: grok_4_1_fast_reasoning,
            display_name: "grok-4-1-fast-reasoning",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4FastNonReasoning {
            model_name: "grok-4-fast-non-reasoning",
            constructor_name: grok_4_fast_non_reasoning,
            display_name: "grok-4-fast-non-reasoning",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4FastReasoning {
            model_name: "grok-4-fast-reasoning",
            constructor_name: grok_4_fast_reasoning,
            display_name: "grok-4-fast-reasoning",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GrokCodeFast1 {
            model_name: "grok-code-fast-1",
            constructor_name: grok_code_fast_1,
            display_name: "grok-code-fast-1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM21 {
            model_name: "minimax/minimax-m2.1",
            constructor_name: minimax_minimax_m2_1,
            display_name: "Minimax M2.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxaiMinimaxM180k {
            model_name: "minimaxai/minimax-m1-80k",
            constructor_name: minimaxai_minimax_m1_80k,
            display_name: "MiniMax M1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK20905 {
            model_name: "moonshotai/kimi-k2-0905",
            constructor_name: moonshotai_kimi_k2_0905,
            display_name: "Kimi K2 0905",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct {
            model_name: "moonshotai/kimi-k2-instruct",
            constructor_name: moonshotai_kimi_k2_instruct,
            display_name: "Kimi K2 Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/kimi-k2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        O3 {
            model_name: "o3",
            constructor_name: o3,
            display_name: "o3",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O3Mini {
            model_name: "o3-mini",
            constructor_name: o3_mini,
            display_name: "o3-mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O4Mini {
            model_name: "o4-mini",
            constructor_name: o4_mini,
            display_name: "o4-mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bFp8 {
            model_name: "qwen/qwen3-235b-a22b-fp8",
            constructor_name: qwen_qwen3_235b_a22b_fp8,
            display_name: "Qwen3 235B A22B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen3235bA22bInstruct2507 {
            model_name: "qwen/qwen3-235b-a22b-instruct-2507",
            constructor_name: qwen_qwen3_235b_a22b_instruct_2507,
            display_name: "Qwen3 235B A22B Instruct 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bThinking2507 {
            model_name: "qwen/qwen3-235b-a22b-thinking-2507",
            constructor_name: qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3 235B A22b Thinking 2507",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3bFp8 {
            model_name: "qwen/qwen3-30b-a3b-fp8",
            constructor_name: qwen_qwen3_30b_a3b_fp8,
            display_name: "Qwen3 30B A3B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen332bFp8 {
            model_name: "qwen/qwen3-32b-fp8",
            constructor_name: qwen_qwen3_32b_fp8,
            display_name: "Qwen3 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Coder480bA35bInstruct {
            model_name: "qwen/qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen_qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderNext {
            model_name: "qwen/qwen3-coder-next",
            constructor_name: qwen_qwen3_coder_next,
            display_name: "qwen/qwen3-coder-next",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bInstruct {
            model_name: "qwen/qwen3-next-80b-a3b-instruct",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3 Next 80B A3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bThinking {
            model_name: "qwen/qwen3-next-80b-a3b-thinking",
            constructor_name: qwen_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3 Next 80B A3B Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomimimoMimoV2Flash {
            model_name: "xiaomimimo/mimo-v2-flash",
            constructor_name: xiaomimimo_mimo_v2_flash,
            display_name: "XiaomiMiMo/MiMo-V2-Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45 {
            model_name: "zai-org/glm-4.5",
            constructor_name: zai_org_glm_4_5,
            display_name: "GLM-4.5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45v {
            model_name: "zai-org/glm-4.5v",
            constructor_name: zai_org_glm_4_5v,
            display_name: "GLM 4.5V",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZaiOrgGlm47 {
            model_name: "zai-org/glm-4.7",
            constructor_name: zai_org_glm_4_7,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Flash {
            model_name: "zai-org/glm-4.7-flash",
            constructor_name: zai_org_glm_4_7_flash,
            display_name: "GLM-4.7-Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
