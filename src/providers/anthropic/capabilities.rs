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
        ClaudeHaiku35 {
            model_name: "claude-haiku-3.5",
            constructor_name: claude_haiku_3_5,
            display_name: "Claude Haiku 3.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku35Latest {
            model_name: "claude-haiku-3.5-(latest)",
            constructor_name: claude_haiku_3_5_latest,
            display_name: "Claude Haiku 3.5 (latest)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet35 {
            model_name: "claude-sonnet-3.5",
            constructor_name: claude_sonnet_3_5,
            display_name: "Claude Sonnet 3.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet35V2 {
            model_name: "claude-sonnet-3.5-v2",
            constructor_name: claude_sonnet_3_5_v2,
            display_name: "Claude Sonnet 3.5 v2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet37 {
            model_name: "claude-sonnet-3.7",
            constructor_name: claude_sonnet_3_7,
            display_name: "Claude Sonnet 3.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet37Latest {
            model_name: "claude-sonnet-3.7-(latest)",
            constructor_name: claude_sonnet_3_7_latest,
            display_name: "Claude Sonnet 3.7 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku3 {
            model_name: "claude-haiku-3",
            constructor_name: claude_haiku_3,
            display_name: "Claude Haiku 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus3 {
            model_name: "claude-opus-3",
            constructor_name: claude_opus_3,
            display_name: "Claude Opus 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet3 {
            model_name: "claude-sonnet-3",
            constructor_name: claude_sonnet_3,
            display_name: "Claude Sonnet 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku45Latest {
            model_name: "claude-haiku-4.5-(latest)",
            constructor_name: claude_haiku_4_5_latest,
            display_name: "Claude Haiku 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku45 {
            model_name: "claude-haiku-4.5",
            constructor_name: claude_haiku_4_5,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4Latest {
            model_name: "claude-opus-4-(latest)",
            constructor_name: claude_opus_4_latest,
            display_name: "Claude Opus 4 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus41Latest {
            model_name: "claude-opus-4.1-(latest)",
            constructor_name: claude_opus_4_1_latest,
            display_name: "Claude Opus 4.1 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus41 {
            model_name: "claude-opus-4.1",
            constructor_name: claude_opus_4_1,
            display_name: "Claude Opus 4.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4 {
            model_name: "claude-opus-4",
            constructor_name: claude_opus_4,
            display_name: "Claude Opus 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus45Latest {
            model_name: "claude-opus-4.5-(latest)",
            constructor_name: claude_opus_4_5_latest,
            display_name: "Claude Opus 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus45 {
            model_name: "claude-opus-4.5",
            constructor_name: claude_opus_4_5,
            display_name: "Claude Opus 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4Latest {
            model_name: "claude-sonnet-4-(latest)",
            constructor_name: claude_sonnet_4_latest,
            display_name: "Claude Sonnet 4 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4 {
            model_name: "claude-sonnet-4",
            constructor_name: claude_sonnet_4,
            display_name: "Claude Sonnet 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet45Latest {
            model_name: "claude-sonnet-4.5-(latest)",
            constructor_name: claude_sonnet_4_5_latest,
            display_name: "Claude Sonnet 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet45 {
            model_name: "claude-sonnet-4.5",
            constructor_name: claude_sonnet_4_5,
            display_name: "Claude Sonnet 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
