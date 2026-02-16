//! Capabilities for friendli models.
//!
//! This module defines model types and their capabilities for friendli providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::friendli::Friendli;

model_capabilities! {
    provider: Friendli,
    models: {
        LgaiExaoneExaone40132b {
            model_name: "LGAI-EXAONE/EXAONE-4.0.1-32B",
            constructor_name: lgai_exaone_exaone_4_0_1_32b,
            display_name: "EXAONE 4.0.1 32B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LgaiExaoneKExaone236bA23b {
            model_name: "LGAI-EXAONE/K-EXAONE-236B-A23B",
            constructor_name: lgai_exaone_k_exaone_236b_a23b,
            display_name: "K EXAONE 236B A23B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxaiMinimaxM21 {
            model_name: "MiniMaxAI/MiniMax-M2.1",
            constructor_name: minimaxai_minimax_m2_1,
            display_name: "MiniMax M2.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bInstruct2507 {
            model_name: "Qwen/Qwen3-235B-A22B-Instruct-2507",
            constructor_name: qwen_qwen3_235b_a22b_instruct_2507,
            display_name: "Qwen3 235B A22B Instruct 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama318bInstruct {
            model_name: "meta-llama/Llama-3.1-8B-Instruct",
            constructor_name: meta_llama_llama_3_1_8b_instruct,
            display_name: "Llama 3.1 8B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3370bInstruct {
            model_name: "meta-llama/Llama-3.3-70B-Instruct",
            constructor_name: meta_llama_llama_3_3_70b_instruct,
            display_name: "Llama 3.3 70B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47 {
            model_name: "zai-org/GLM-4.7",
            constructor_name: zai_org_glm_4_7,
            display_name: "GLM 4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5 {
            model_name: "zai-org/GLM-5",
            constructor_name: zai_org_glm_5,
            display_name: "GLM 5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
