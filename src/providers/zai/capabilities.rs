//! Capabilities for zai models.
//!
//! This module defines model types and their capabilities for zai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::zai::Zai;

model_capabilities! {
    provider: Zai,
    models: {
        Glm45 {
            model_name: "glm-4.5",
            constructor_name: glm_4_5,
            display_name: "GLM-4.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm45Air {
            model_name: "glm-4.5-air",
            constructor_name: glm_4_5_air,
            display_name: "GLM-4.5-Air",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm45Flash {
            model_name: "glm-4.5-flash",
            constructor_name: glm_4_5_flash,
            display_name: "GLM-4.5-Flash",
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
            display_name: "GLM-4.6",
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
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm47Flash {
            model_name: "glm-4.7-flash",
            constructor_name: glm_4_7_flash,
            display_name: "GLM-4.7-Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm5 {
            model_name: "glm-5",
            constructor_name: glm_5,
            display_name: "GLM-5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
