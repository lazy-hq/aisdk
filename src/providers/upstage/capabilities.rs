//! Capabilities for upstage models.
//!
//! This module defines model types and their capabilities for upstage providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::upstage::Upstage;

model_capabilities! {
    provider: Upstage,
    models: {
        SolarMini {
            model_name: "solar-mini",
            constructor_name: solar_mini,
            display_name: "solar-mini",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SolarMini4 {
            model_name: "solar-mini4",
            constructor_name: solar_mini4,
            display_name: "Solar Mini 4",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SolarPro2 {
            model_name: "solar-pro2",
            constructor_name: solar_pro2,
            display_name: "solar-pro2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SolarPro3 {
            model_name: "solar-pro3",
            constructor_name: solar_pro3,
            display_name: "solar-pro3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SolarPro4 {
            model_name: "solar-pro4",
            constructor_name: solar_pro4,
            display_name: "Solar Pro 4",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
