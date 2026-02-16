//! Capabilities for kuae_cloud_coding_plan models.
//!
//! This module defines model types and their capabilities for kuae_cloud_coding_plan providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::kuae_cloud_coding_plan::KuaeCloudCodingPlan;

model_capabilities! {
    provider: KuaeCloudCodingPlan,
    models: {
        Glm47 {
            model_name: "GLM-4.7",
            constructor_name: glm_4_7,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
