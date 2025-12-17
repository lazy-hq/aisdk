//! Capabilities for anthropic models.
//!
//! This module defines model types and their capabilities for anthropic providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::anthropic::Anthropic;

model_capabilities! {
    provider: Anthropic,
    models: {
        Claude35Haiku20241022 {
            model_name: "claude-3-5-haiku-20241022",
            constructor_name: claude_3_5_haiku_20241022,
            display_name: "Claude Haiku 3.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude35HaikuLatest {
            model_name: "claude-3-5-haiku-latest",
            constructor_name: claude_3_5_haiku_latest,
            display_name: "Claude Haiku 3.5 (latest)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude35Sonnet20240620 {
            model_name: "claude-3-5-sonnet-20240620",
            constructor_name: claude_3_5_sonnet_20240620,
            display_name: "Claude Sonnet 3.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude35Sonnet20241022 {
            model_name: "claude-3-5-sonnet-20241022",
            constructor_name: claude_3_5_sonnet_20241022,
            display_name: "Claude Sonnet 3.5 v2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude37Sonnet20250219 {
            model_name: "claude-3-7-sonnet-20250219",
            constructor_name: claude_3_7_sonnet_20250219,
            display_name: "Claude Sonnet 3.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude37SonnetLatest {
            model_name: "claude-3-7-sonnet-latest",
            constructor_name: claude_3_7_sonnet_latest,
            display_name: "Claude Sonnet 3.7 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude3Haiku20240307 {
            model_name: "claude-3-haiku-20240307",
            constructor_name: claude_3_haiku_20240307,
            display_name: "Claude Haiku 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude3Opus20240229 {
            model_name: "claude-3-opus-20240229",
            constructor_name: claude_3_opus_20240229,
            display_name: "Claude Opus 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude3Sonnet20240229 {
            model_name: "claude-3-sonnet-20240229",
            constructor_name: claude_3_sonnet_20240229,
            display_name: "Claude Sonnet 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku45 {
            model_name: "claude-haiku-4-5",
            constructor_name: claude_haiku_4_5,
            display_name: "Claude Haiku 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku4520251001 {
            model_name: "claude-haiku-4-5-20251001",
            constructor_name: claude_haiku_4_5_20251001,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus40 {
            model_name: "claude-opus-4-0",
            constructor_name: claude_opus_4_0,
            display_name: "Claude Opus 4 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus41 {
            model_name: "claude-opus-4-1",
            constructor_name: claude_opus_4_1,
            display_name: "Claude Opus 4.1 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4120250805 {
            model_name: "claude-opus-4-1-20250805",
            constructor_name: claude_opus_4_1_20250805,
            display_name: "Claude Opus 4.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus420250514 {
            model_name: "claude-opus-4-20250514",
            constructor_name: claude_opus_4_20250514,
            display_name: "Claude Opus 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus45 {
            model_name: "claude-opus-4-5",
            constructor_name: claude_opus_4_5,
            display_name: "Claude Opus 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4520251101 {
            model_name: "claude-opus-4-5-20251101",
            constructor_name: claude_opus_4_5_20251101,
            display_name: "Claude Opus 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet40 {
            model_name: "claude-sonnet-4-0",
            constructor_name: claude_sonnet_4_0,
            display_name: "Claude Sonnet 4 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet420250514 {
            model_name: "claude-sonnet-4-20250514",
            constructor_name: claude_sonnet_4_20250514,
            display_name: "Claude Sonnet 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet45 {
            model_name: "claude-sonnet-4-5",
            constructor_name: claude_sonnet_4_5,
            display_name: "Claude Sonnet 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4520250929 {
            model_name: "claude-sonnet-4-5-20250929",
            constructor_name: claude_sonnet_4_5_20250929,
            display_name: "Claude Sonnet 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
