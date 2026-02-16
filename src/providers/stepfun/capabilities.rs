//! Capabilities for stepfun models.
//!
//! This module defines model types and their capabilities for stepfun providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::stepfun::Stepfun;

model_capabilities! {
    provider: Stepfun,
    models: {
        Step132k {
            model_name: "step-1-32k",
            constructor_name: step_1_32k,
            display_name: "Step 1 (32K)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Step216k {
            model_name: "step-2-16k",
            constructor_name: step_2_16k,
            display_name: "Step 2 (16K)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Step35Flash {
            model_name: "step-3.5-flash",
            constructor_name: step_3_5_flash,
            display_name: "Step 3.5 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
