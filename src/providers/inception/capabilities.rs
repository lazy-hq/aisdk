//! Capabilities for inception models.
//!
//! This module defines model types and their capabilities for inception providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::inception::Inception;

model_capabilities! {
    provider: Inception,
    models: {
        Mercury {
            model_name: "mercury",
            constructor_name: mercury,
            display_name: "Mercury",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Mercury2 {
            model_name: "mercury-2",
            constructor_name: mercury_2,
            display_name: "Mercury 2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Mercury25 {
            model_name: "mercury-2.5",
            constructor_name: mercury_2_5,
            display_name: "Mercury 2.5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Mercury3 {
            model_name: "mercury-3",
            constructor_name: mercury_3,
            display_name: "Mercury 3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MercuryCoder {
            model_name: "mercury-coder",
            constructor_name: mercury_coder,
            display_name: "Mercury Coder",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MercuryEdit2 {
            model_name: "mercury-edit-2",
            constructor_name: mercury_edit_2,
            display_name: "Mercury Edit 2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
    }
}
