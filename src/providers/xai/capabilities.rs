//! Capabilities for xai models.
//!
//! This module defines model types and their capabilities for xai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::xai::XAI;

model_capabilities! {
    provider: XAI,
    models: {
        Grok2 {
            model_name: "grok-2",
            constructor_name: grok_2,
            display_name: "Grok 2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok21212 {
            model_name: "grok-2-1212",
            constructor_name: grok_2_1212,
            display_name: "Grok 2 (1212)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok2Latest {
            model_name: "grok-2-latest",
            constructor_name: grok_2_latest,
            display_name: "Grok 2 Latest",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok2Vision {
            model_name: "grok-2-vision",
            constructor_name: grok_2_vision,
            display_name: "Grok 2 Vision",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok2Vision1212 {
            model_name: "grok-2-vision-1212",
            constructor_name: grok_2_vision_1212,
            display_name: "Grok 2 Vision (1212)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok2VisionLatest {
            model_name: "grok-2-vision-latest",
            constructor_name: grok_2_vision_latest,
            display_name: "Grok 2 Vision Latest",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok3 {
            model_name: "grok-3",
            constructor_name: grok_3,
            display_name: "Grok 3",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok3Fast {
            model_name: "grok-3-fast",
            constructor_name: grok_3_fast,
            display_name: "Grok 3 Fast",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok3FastLatest {
            model_name: "grok-3-fast-latest",
            constructor_name: grok_3_fast_latest,
            display_name: "Grok 3 Fast Latest",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok3Latest {
            model_name: "grok-3-latest",
            constructor_name: grok_3_latest,
            display_name: "Grok 3 Latest",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok3Mini {
            model_name: "grok-3-mini",
            constructor_name: grok_3_mini,
            display_name: "Grok 3 Mini",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok3MiniFast {
            model_name: "grok-3-mini-fast",
            constructor_name: grok_3_mini_fast,
            display_name: "Grok 3 Mini Fast",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok3MiniFastLatest {
            model_name: "grok-3-mini-fast-latest",
            constructor_name: grok_3_mini_fast_latest,
            display_name: "Grok 3 Mini Fast Latest",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok3MiniLatest {
            model_name: "grok-3-mini-latest",
            constructor_name: grok_3_mini_latest,
            display_name: "Grok 3 Mini Latest",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4 {
            model_name: "grok-4",
            constructor_name: grok_4,
            display_name: "Grok 4",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41Fast {
            model_name: "grok-4-1-fast",
            constructor_name: grok_4_1_fast,
            display_name: "Grok 4.1 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41FastNonReasoning {
            model_name: "grok-4-1-fast-non-reasoning",
            constructor_name: grok_4_1_fast_non_reasoning,
            display_name: "Grok 4.1 Fast (Non-Reasoning)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4Fast {
            model_name: "grok-4-fast",
            constructor_name: grok_4_fast,
            display_name: "Grok 4 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4FastNonReasoning {
            model_name: "grok-4-fast-non-reasoning",
            constructor_name: grok_4_fast_non_reasoning,
            display_name: "Grok 4 Fast (Non-Reasoning)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GrokBeta {
            model_name: "grok-beta",
            constructor_name: grok_beta,
            display_name: "Grok Beta",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GrokCodeFast1 {
            model_name: "grok-code-fast-1",
            constructor_name: grok_code_fast_1,
            display_name: "Grok Code Fast 1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GrokVisionBeta {
            model_name: "grok-vision-beta",
            constructor_name: grok_vision_beta,
            display_name: "Grok Vision Beta",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
