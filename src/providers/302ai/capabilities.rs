//! Capabilities for ai_302 models.
//!
//! This module defines model types and their capabilities for ai_302 providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::ai_302::Ai302;

model_capabilities! {
    provider: Ai302,
    models: {
        MinimaxM1 {
            model_name: "MiniMax-M1",
            constructor_name: minimax_m1,
            display_name: "MiniMax-M1",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM2 {
            model_name: "MiniMax-M2",
            constructor_name: minimax_m2,
            display_name: "MiniMax-M2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM21 {
            model_name: "MiniMax-M2.1",
            constructor_name: minimax_m2_1,
            display_name: "MiniMax-M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM25 {
            model_name: "MiniMax-M2.5",
            constructor_name: minimax_m2_5,
            display_name: "MiniMax-M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM27 {
            model_name: "MiniMax-M2.7",
            constructor_name: minimax_m2_7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM3 {
            model_name: "MiniMax-M3",
            constructor_name: minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Chatgpt4oLatest {
            model_name: "chatgpt-4o-latest",
            constructor_name: chatgpt_4o_latest,
            display_name: "chatgpt-4o-latest",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ClaudeFable5 {
            model_name: "claude-fable-5",
            constructor_name: claude_fable_5,
            display_name: "Claude Fable 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeFable51 {
            model_name: "claude-fable-5-1",
            constructor_name: claude_fable_5_1,
            display_name: "Claude Fable 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku45 {
            model_name: "claude-haiku-4-5",
            constructor_name: claude_haiku_4_5,
            display_name: "claude-haiku-4-5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku4520251001 {
            model_name: "claude-haiku-4-5-20251001",
            constructor_name: claude_haiku_4_5_20251001,
            display_name: "claude-haiku-4-5-20251001",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku55 {
            model_name: "claude-haiku-5-5",
            constructor_name: claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4120250805 {
            model_name: "claude-opus-4-1-20250805",
            constructor_name: claude_opus_4_1_20250805,
            display_name: "claude-opus-4-1-20250805",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4120250805Thinking {
            model_name: "claude-opus-4-1-20250805-thinking",
            constructor_name: claude_opus_4_1_20250805_thinking,
            display_name: "claude-opus-4-1-20250805-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4520251101 {
            model_name: "claude-opus-4-5-20251101",
            constructor_name: claude_opus_4_5_20251101,
            display_name: "claude-opus-4-5-20251101",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4520251101Thinking {
            model_name: "claude-opus-4-5-20251101-thinking",
            constructor_name: claude_opus_4_5_20251101_thinking,
            display_name: "claude-opus-4-5-20251101-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus47 {
            model_name: "claude-opus-4-7",
            constructor_name: claude_opus_4_7,
            display_name: "claude-opus-4-7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus47Thinking {
            model_name: "claude-opus-4-7-thinking",
            constructor_name: claude_opus_4_7_thinking,
            display_name: "claude-opus-4-7-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus48 {
            model_name: "claude-opus-4-8",
            constructor_name: claude_opus_4_8,
            display_name: "Claude Opus 4.8",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus5 {
            model_name: "claude-opus-5",
            constructor_name: claude_opus_5,
            display_name: "Claude Opus 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus55 {
            model_name: "claude-opus-5-5",
            constructor_name: claude_opus_5_5,
            display_name: "Claude Opus 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus5Thinking {
            model_name: "claude-opus-5-thinking",
            constructor_name: claude_opus_5_thinking,
            display_name: "claude-opus-5-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4520250929 {
            model_name: "claude-sonnet-4-5-20250929",
            constructor_name: claude_sonnet_4_5_20250929,
            display_name: "claude-sonnet-4-5-20250929",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4520250929Thinking {
            model_name: "claude-sonnet-4-5-20250929-thinking",
            constructor_name: claude_sonnet_4_5_20250929_thinking,
            display_name: "claude-sonnet-4-5-20250929-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet46 {
            model_name: "claude-sonnet-4-6",
            constructor_name: claude_sonnet_4_6,
            display_name: "claude-sonnet-4-6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet46Thinking {
            model_name: "claude-sonnet-4-6-thinking",
            constructor_name: claude_sonnet_4_6_thinking,
            display_name: "claude-sonnet-4-6-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet5 {
            model_name: "claude-sonnet-5",
            constructor_name: claude_sonnet_5,
            display_name: "Claude Sonnet 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet55 {
            model_name: "claude-sonnet-5-5",
            constructor_name: claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekChat {
            model_name: "deepseek-chat",
            constructor_name: deepseek_chat,
            display_name: "Deepseek-Chat",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekFlash {
            model_name: "deepseek-flash",
            constructor_name: deepseek_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekReasoner {
            model_name: "deepseek-reasoner",
            constructor_name: deepseek_reasoner,
            display_name: "Deepseek-Reasoner",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV32 {
            model_name: "deepseek-v3.2",
            constructor_name: deepseek_v3_2,
            display_name: "deepseek-v3.2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV32Thinking {
            model_name: "deepseek-v3.2-thinking",
            constructor_name: deepseek_v3_2_thinking,
            display_name: "DeepSeek-V3.2-Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DoubaoSeed16Thinking250715 {
            model_name: "doubao-seed-1-6-thinking-250715",
            constructor_name: doubao_seed_1_6_thinking_250715,
            display_name: "doubao-seed-1-6-thinking-250715",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DoubaoSeed16Vision250815 {
            model_name: "doubao-seed-1-6-vision-250815",
            constructor_name: doubao_seed_1_6_vision_250815,
            display_name: "doubao-seed-1-6-vision-250815",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DoubaoSeed18251215 {
            model_name: "doubao-seed-1-8-251215",
            constructor_name: doubao_seed_1_8_251215,
            display_name: "doubao-seed-1-8-251215",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DoubaoSeedCodePreview251028 {
            model_name: "doubao-seed-code-preview-251028",
            constructor_name: doubao_seed_code_preview_251028,
            display_name: "doubao-seed-code-preview-251028",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini20FlashLite {
            model_name: "gemini-2.0-flash-lite",
            constructor_name: gemini_2_0_flash_lite,
            display_name: "gemini-2.0-flash-lite",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25Flash {
            model_name: "gemini-2.5-flash",
            constructor_name: gemini_2_5_flash,
            display_name: "gemini-2.5-flash",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashImage {
            model_name: "gemini-2.5-flash-image",
            constructor_name: gemini_2_5_flash_image,
            display_name: "gemini-2.5-flash-image",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashLitePreview092025 {
            model_name: "gemini-2.5-flash-lite-preview-09-2025",
            constructor_name: gemini_2_5_flash_lite_preview_09_2025,
            display_name: "gemini-2.5-flash-lite-preview-09-2025",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashNothink {
            model_name: "gemini-2.5-flash-nothink",
            constructor_name: gemini_2_5_flash_nothink,
            display_name: "gemini-2.5-flash-nothink",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashPreview092025 {
            model_name: "gemini-2.5-flash-preview-09-2025",
            constructor_name: gemini_2_5_flash_preview_09_2025,
            display_name: "gemini-2.5-flash-preview-09-2025",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25Pro {
            model_name: "gemini-2.5-pro",
            constructor_name: gemini_2_5_pro,
            display_name: "gemini-2.5-pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini3FlashPreview {
            model_name: "gemini-3-flash-preview",
            constructor_name: gemini_3_flash_preview,
            display_name: "gemini-3-flash-preview",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini3ProImagePreview {
            model_name: "gemini-3-pro-image-preview",
            constructor_name: gemini_3_pro_image_preview,
            display_name: "gemini-3-pro-image-preview",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini3ProPreview {
            model_name: "gemini-3-pro-preview",
            constructor_name: gemini_3_pro_preview,
            display_name: "gemini-3-pro-preview",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini31FlashImagePreview {
            model_name: "gemini-3.1-flash-image-preview",
            constructor_name: gemini_3_1_flash_image_preview,
            display_name: "gemini-3.1-flash-image-preview",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini31FlashLite {
            model_name: "gemini-3.1-flash-lite",
            constructor_name: gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini31FlashLitePreview {
            model_name: "gemini-3.1-flash-lite-preview",
            constructor_name: gemini_3_1_flash_lite_preview,
            display_name: "Gemini 3.1 Flash Lite Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini31ProPreview {
            model_name: "gemini-3.1-pro-preview",
            constructor_name: gemini_3_1_pro_preview,
            display_name: "Gemini 3.1 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini35Flash {
            model_name: "gemini-3.5-flash",
            constructor_name: gemini_3_5_flash,
            display_name: "Gemini 3.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini35FlashLite {
            model_name: "gemini-3.5-flash-lite",
            constructor_name: gemini_3_5_flash_lite,
            display_name: "Gemini 3.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini35FlashThinking {
            model_name: "gemini-3.5-flash-thinking",
            constructor_name: gemini_3_5_flash_thinking,
            display_name: "gemini-3.5-flash-thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini36Flash {
            model_name: "gemini-3.6-flash",
            constructor_name: gemini_3_6_flash,
            display_name: "Gemini 3.6 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini37Flash {
            model_name: "gemini-3.7-flash",
            constructor_name: gemini_3_7_flash,
            display_name: "Gemini 3.7 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini38Flash {
            model_name: "gemini-3.8-flash",
            constructor_name: gemini_3_8_flash,
            display_name: "Gemini 3.8 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini4Argon {
            model_name: "gemini-4-argon",
            constructor_name: gemini_4_argon,
            display_name: "Gemini 4 Argon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm45 {
            model_name: "glm-4.5",
            constructor_name: glm_4_5,
            display_name: "GLM-4.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm45v {
            model_name: "glm-4.5v",
            constructor_name: glm_4_5v,
            display_name: "GLM-4.5V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm46 {
            model_name: "glm-4.6",
            constructor_name: glm_4_6,
            display_name: "glm-4.6",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm46v {
            model_name: "glm-4.6v",
            constructor_name: glm_4_6v,
            display_name: "GLM-4.6V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm47 {
            model_name: "glm-4.7",
            constructor_name: glm_4_7,
            display_name: "glm-4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm5 {
            model_name: "glm-5",
            constructor_name: glm_5,
            display_name: "glm-5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm5Turbo {
            model_name: "glm-5-turbo",
            constructor_name: glm_5_turbo,
            display_name: "glm-5-turbo",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm51 {
            model_name: "glm-5.1",
            constructor_name: glm_5_1,
            display_name: "GLM-5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52 {
            model_name: "glm-5.2",
            constructor_name: glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53 {
            model_name: "glm-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash {
            model_name: "glm-5.3-flash",
            constructor_name: glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm5vTurbo {
            model_name: "glm-5v-turbo",
            constructor_name: glm_5v_turbo,
            display_name: "GLM-5V-Turbo",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gpt41 {
            model_name: "gpt-4.1",
            constructor_name: gpt_4_1,
            display_name: "gpt-4.1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Mini {
            model_name: "gpt-4.1-mini",
            constructor_name: gpt_4_1_mini,
            display_name: "gpt-4.1-mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Nano {
            model_name: "gpt-4.1-nano",
            constructor_name: gpt_4_1_nano,
            display_name: "gpt-4.1-nano",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4o {
            model_name: "gpt-4o",
            constructor_name: gpt_4o,
            display_name: "gpt-4o",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5 {
            model_name: "gpt-5",
            constructor_name: gpt_5,
            display_name: "gpt-5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Mini {
            model_name: "gpt-5-mini",
            constructor_name: gpt_5_mini,
            display_name: "gpt-5-mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Pro {
            model_name: "gpt-5-pro",
            constructor_name: gpt_5_pro,
            display_name: "gpt-5-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Thinking {
            model_name: "gpt-5-thinking",
            constructor_name: gpt_5_thinking,
            display_name: "gpt-5-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51 {
            model_name: "gpt-5.1",
            constructor_name: gpt_5_1,
            display_name: "gpt-5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51ChatLatest {
            model_name: "gpt-5.1-chat-latest",
            constructor_name: gpt_5_1_chat_latest,
            display_name: "gpt-5.1-chat-latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52 {
            model_name: "gpt-5.2",
            constructor_name: gpt_5_2,
            display_name: "gpt-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52ChatLatest {
            model_name: "gpt-5.2-chat-latest",
            constructor_name: gpt_5_2_chat_latest,
            display_name: "gpt-5.2-chat-latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt53ChatLatest {
            model_name: "gpt-5.3-chat-latest",
            constructor_name: gpt_5_3_chat_latest,
            display_name: "GPT-5.3 Chat (latest)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54 {
            model_name: "gpt-5.4",
            constructor_name: gpt_5_4,
            display_name: "gpt-5.4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Mini {
            model_name: "gpt-5.4-mini",
            constructor_name: gpt_5_4_mini,
            display_name: "gpt-5.4-mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Nano {
            model_name: "gpt-5.4-nano",
            constructor_name: gpt_5_4_nano,
            display_name: "gpt-5.4-nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt55 {
            model_name: "gpt-5.5",
            constructor_name: gpt_5_5,
            display_name: "GPT-5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Luna {
            model_name: "gpt-5.6-luna",
            constructor_name: gpt_5_6_luna,
            display_name: "GPT-5.6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56LunaPro {
            model_name: "gpt-5.6-luna-pro",
            constructor_name: gpt_5_6_luna_pro,
            display_name: "gpt-5.6-luna-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Sol {
            model_name: "gpt-5.6-sol",
            constructor_name: gpt_5_6_sol,
            display_name: "GPT-5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56SolPro {
            model_name: "gpt-5.6-sol-pro",
            constructor_name: gpt_5_6_sol_pro,
            display_name: "gpt-5.6-sol-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Terra {
            model_name: "gpt-5.6-terra",
            constructor_name: gpt_5_6_terra,
            display_name: "GPT-5.6 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56TerraPro {
            model_name: "gpt-5.6-terra-pro",
            constructor_name: gpt_5_6_terra_pro,
            display_name: "gpt-5.6-terra-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Astra {
            model_name: "gpt-6-astra",
            constructor_name: gpt_6_astra,
            display_name: "GPT-6 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Luna {
            model_name: "gpt-6-luna",
            constructor_name: gpt_6_luna,
            display_name: "GPT-6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Sol {
            model_name: "gpt-6-sol",
            constructor_name: gpt_6_sol,
            display_name: "GPT-6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61Sol {
            model_name: "gpt-6.1-sol",
            constructor_name: gpt_6_1_sol,
            display_name: "GPT-6.1 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41FastNonReasoning {
            model_name: "grok-4-1-fast-non-reasoning",
            constructor_name: grok_4_1_fast_non_reasoning,
            display_name: "grok-4-1-fast-non-reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41FastReasoning {
            model_name: "grok-4-1-fast-reasoning",
            constructor_name: grok_4_1_fast_reasoning,
            display_name: "grok-4-1-fast-reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4FastNonReasoning {
            model_name: "grok-4-fast-non-reasoning",
            constructor_name: grok_4_fast_non_reasoning,
            display_name: "grok-4-fast-non-reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4FastReasoning {
            model_name: "grok-4-fast-reasoning",
            constructor_name: grok_4_fast_reasoning,
            display_name: "grok-4-fast-reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41 {
            model_name: "grok-4.1",
            constructor_name: grok_4_1,
            display_name: "grok-4.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok420Beta0309Reasoning {
            model_name: "grok-4.20-beta-0309-reasoning",
            constructor_name: grok_4_20_beta_0309_reasoning,
            display_name: "grok-4.20-beta-0309-reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok43 {
            model_name: "grok-4.3",
            constructor_name: grok_4_3,
            display_name: "Grok 4.3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok45 {
            model_name: "grok-4.5",
            constructor_name: grok_4_5,
            display_name: "Grok 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok46 {
            model_name: "grok-4.6",
            constructor_name: grok_4_6,
            display_name: "Grok 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok47 {
            model_name: "grok-4.7",
            constructor_name: grok_4_7,
            display_name: "Grok 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK20905Preview {
            model_name: "kimi-k2-0905-preview",
            constructor_name: kimi_k2_0905_preview,
            display_name: "kimi-k2-0905-preview",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2Thinking {
            model_name: "kimi-k2-thinking",
            constructor_name: kimi_k2_thinking,
            display_name: "kimi-k2-thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2ThinkingTurbo {
            model_name: "kimi-k2-thinking-turbo",
            constructor_name: kimi_k2_thinking_turbo,
            display_name: "kimi-k2-thinking-turbo",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK25 {
            model_name: "kimi-k2.5",
            constructor_name: kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK26 {
            model_name: "kimi-k2.6",
            constructor_name: kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK27Code {
            model_name: "kimi-k2.7-code",
            constructor_name: kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK3 {
            model_name: "kimi-k3",
            constructor_name: kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK31Pro {
            model_name: "kimi-k3.1-pro",
            constructor_name: kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Ministral14b2512 {
            model_name: "ministral-14b-2512",
            constructor_name: ministral_14b_2512,
            display_name: "ministral-14b-2512",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralLarge2512 {
            model_name: "mistral-large-2512",
            constructor_name: mistral_large_2512,
            display_name: "mistral-large-2512",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O3 {
            model_name: "o3",
            constructor_name: o3,
            display_name: "o3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenFlash {
            model_name: "qwen-flash",
            constructor_name: qwen_flash,
            display_name: "Qwen-Flash",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenMaxLatest {
            model_name: "qwen-max-latest",
            constructor_name: qwen_max_latest,
            display_name: "Qwen-Max-Latest",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenPlus {
            model_name: "qwen-plus",
            constructor_name: qwen_plus,
            display_name: "Qwen-Plus",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3235bA22b {
            model_name: "qwen3-235b-a22b",
            constructor_name: qwen3_235b_a22b,
            display_name: "Qwen3-235B-A22B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3235bA22bInstruct2507 {
            model_name: "qwen3-235b-a22b-instruct-2507",
            constructor_name: qwen3_235b_a22b_instruct_2507,
            display_name: "qwen3-235b-a22b-instruct-2507",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen330bA3b {
            model_name: "qwen3-30b-a3b",
            constructor_name: qwen3_30b_a3b,
            display_name: "Qwen3-30B-A3B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder480bA35bInstruct {
            model_name: "qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen3_coder_480b_a35b_instruct,
            display_name: "qwen3-coder-480b-a35b-instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Max20250923 {
            model_name: "qwen3-max-2025-09-23",
            constructor_name: qwen3_max_2025_09_23,
            display_name: "qwen3-max-2025-09-23",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3535bA3b {
            model_name: "qwen3.5-35b-a3b",
            constructor_name: qwen3_5_35b_a3b,
            display_name: "Qwen3.5 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen35Plus {
            model_name: "qwen3.5-plus",
            constructor_name: qwen3_5_plus,
            display_name: "Qwen3.5 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3635bA3b {
            model_name: "qwen3.6-35b-a3b",
            constructor_name: qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen36Flash {
            model_name: "qwen3.6-flash",
            constructor_name: qwen3_6_flash,
            display_name: "Qwen3.6 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen36Plus {
            model_name: "qwen3.6-plus",
            constructor_name: qwen3_6_plus,
            display_name: "Qwen3.6 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen37Max {
            model_name: "qwen3.7-max",
            constructor_name: qwen3_7_max,
            display_name: "Qwen3.7 Max",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen37Max20260608 {
            model_name: "qwen3.7-max-2026-06-08",
            constructor_name: qwen3_7_max_2026_06_08,
            display_name: "qwen3.7-max-2026-06-08",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen37Plus {
            model_name: "qwen3.7-plus",
            constructor_name: qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38Flash {
            model_name: "qwen3.8-flash",
            constructor_name: qwen3_8_flash,
            display_name: "Qwen3.8 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38Max {
            model_name: "qwen3.8-max",
            constructor_name: qwen3_8_max,
            display_name: "Qwen3.8 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
