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
        MercuryCoder {
            model_name: "mercury-coder",
            constructor_name: mercury_coder,
            display_name: "Mercury Coder",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Mercury2 {
            model_name: "mercury-2",
            constructor_name: mercury_2,
            display_name: "Mercury 2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport, StructuredOutputSupport, ReasoningSupport]
        },
        MercuryEdit {
            model_name: "mercury-edit",
            constructor_name: mercury_edit,
            display_name: "Mercury Edit",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
    }
}
