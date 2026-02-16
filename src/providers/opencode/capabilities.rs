//! Capabilities for opencode models.
//!
//! This module defines model types and their capabilities for opencode providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::opencode::Opencode;

model_capabilities! {
    provider: Opencode,
    models: {
        BigPickle {
            model_name: "big-pickle",
            constructor_name: big_pickle,
            display_name: "Big Pickle",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude35Haiku {
            model_name: "claude-3-5-haiku",
            constructor_name: claude_3_5_haiku,
            display_name: "Claude Haiku 3.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku45 {
            model_name: "claude-haiku-4-5",
            constructor_name: claude_haiku_4_5,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus41 {
            model_name: "claude-opus-4-1",
            constructor_name: claude_opus_4_1,
            display_name: "Claude Opus 4.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus45 {
            model_name: "claude-opus-4-5",
            constructor_name: claude_opus_4_5,
            display_name: "Claude Opus 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus46 {
            model_name: "claude-opus-4-6",
            constructor_name: claude_opus_4_6,
            display_name: "Claude Opus 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4 {
            model_name: "claude-sonnet-4",
            constructor_name: claude_sonnet_4,
            display_name: "Claude Sonnet 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet45 {
            model_name: "claude-sonnet-4-5",
            constructor_name: claude_sonnet_4_5,
            display_name: "Claude Sonnet 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini3Flash {
            model_name: "gemini-3-flash",
            constructor_name: gemini_3_flash,
            display_name: "Gemini 3 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini3Pro {
            model_name: "gemini-3-pro",
            constructor_name: gemini_3_pro,
            display_name: "Gemini 3 Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm46 {
            model_name: "glm-4.6",
            constructor_name: glm_4_6,
            display_name: "GLM-4.6",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm47 {
            model_name: "glm-4.7",
            constructor_name: glm_4_7,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm5 {
            model_name: "glm-5",
            constructor_name: glm_5,
            display_name: "GLM-5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5 {
            model_name: "gpt-5",
            constructor_name: gpt_5,
            display_name: "GPT-5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Codex {
            model_name: "gpt-5-codex",
            constructor_name: gpt_5_codex,
            display_name: "GPT-5 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Nano {
            model_name: "gpt-5-nano",
            constructor_name: gpt_5_nano,
            display_name: "GPT-5 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51 {
            model_name: "gpt-5.1",
            constructor_name: gpt_5_1,
            display_name: "GPT-5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51Codex {
            model_name: "gpt-5.1-codex",
            constructor_name: gpt_5_1_codex,
            display_name: "GPT-5.1 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51CodexMax {
            model_name: "gpt-5.1-codex-max",
            constructor_name: gpt_5_1_codex_max,
            display_name: "GPT-5.1 Codex Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51CodexMini {
            model_name: "gpt-5.1-codex-mini",
            constructor_name: gpt_5_1_codex_mini,
            display_name: "GPT-5.1 Codex Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52 {
            model_name: "gpt-5.2",
            constructor_name: gpt_5_2,
            display_name: "GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52Codex {
            model_name: "gpt-5.2-codex",
            constructor_name: gpt_5_2_codex,
            display_name: "GPT-5.2 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2 {
            model_name: "kimi-k2",
            constructor_name: kimi_k2,
            display_name: "Kimi K2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2Thinking {
            model_name: "kimi-k2-thinking",
            constructor_name: kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK25 {
            model_name: "kimi-k2.5",
            constructor_name: kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK25Free {
            model_name: "kimi-k2.5-free",
            constructor_name: kimi_k2_5_free,
            display_name: "Kimi K2.5 Free",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MinimaxM21 {
            model_name: "minimax-m2.1",
            constructor_name: minimax_m2_1,
            display_name: "MiniMax M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM25 {
            model_name: "minimax-m2.5",
            constructor_name: minimax_m2_5,
            display_name: "MiniMax M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM25Free {
            model_name: "minimax-m2.5-free",
            constructor_name: minimax_m2_5_free,
            display_name: "MiniMax M2.5 Free",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
