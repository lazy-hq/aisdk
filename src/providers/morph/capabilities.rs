//! Capabilities for morph models.
//!
//! This module defines model types and their capabilities for morph providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::morph::Morph;

model_capabilities! {
    provider: Morph,
    models: {
        Auto {
            model_name: "auto",
            constructor_name: auto,
            display_name: "Auto",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MorphDsv41flash {
            model_name: "morph-dsv41flash",
            constructor_name: morph_dsv41flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MorphDsv41flash1017 {
            model_name: "morph-dsv41flash-1017",
            constructor_name: morph_dsv41flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MorphDsv4flash {
            model_name: "morph-dsv4flash",
            constructor_name: morph_dsv4flash,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MorphGlm52744b {
            model_name: "morph-glm52-744b",
            constructor_name: morph_glm52_744b,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MorphGlm53744b {
            model_name: "morph-glm53-744b",
            constructor_name: morph_glm53_744b,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MorphGlm53flash {
            model_name: "morph-glm53flash",
            constructor_name: morph_glm53flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MorphKimik3 {
            model_name: "morph-kimik3",
            constructor_name: morph_kimik3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MorphKimik3Fast {
            model_name: "morph-kimik3-fast",
            constructor_name: morph_kimik3_fast,
            display_name: "Kimi K3 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MorphV3Fast {
            model_name: "morph-v3-fast",
            constructor_name: morph_v3_fast,
            display_name: "Morph v3 Fast",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MorphV3Large {
            model_name: "morph-v3-large",
            constructor_name: morph_v3_large,
            display_name: "Morph v3 Large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
    }
}
