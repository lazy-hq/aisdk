//! Capabilities for bailing models.
//!
//! This module defines model types and their capabilities for bailing providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::bailing::Bailing;

model_capabilities! {
    provider: Bailing,
    models: {
        Ling1t {
            model_name: "Ling-1T",
            constructor_name: ling_1t,
            display_name: "Ling-1T",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ling30Flash {
            model_name: "Ling-3.0-flash",
            constructor_name: ling_3_0_flash,
            display_name: "Ling 3.0 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ling30FlashVl {
            model_name: "Ling-3.0-flash-VL",
            constructor_name: ling_3_0_flash_vl,
            display_name: "Ling 3.0 Flash VL",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Ling30Tiny {
            model_name: "Ling-3.0-tiny",
            constructor_name: ling_3_0_tiny,
            display_name: "Ling 3.0 Tiny",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ling31Flash {
            model_name: "Ling-3.1-flash",
            constructor_name: ling_3_1_flash,
            display_name: "Ling 3.1 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ling32Flash {
            model_name: "Ling-3.2-flash",
            constructor_name: ling_3_2_flash,
            display_name: "Ling 3.2 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Ring1t {
            model_name: "Ring-1T",
            constructor_name: ring_1t,
            display_name: "Ring-1T",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
    }
}
