//! Capabilities for nano_gpt models.
//!
//! This module defines model types and their capabilities for nano_gpt providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::nano_gpt::NanoGpt;

model_capabilities! {
    provider: NanoGpt,
    models: {
        DeepseekDeepseekR1 {
            model_name: "deepseek/deepseek-r1",
            constructor_name: deepseek_deepseek_r1,
            display_name: "Deepseek R1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32Thinking {
            model_name: "deepseek/deepseek-v3.2:thinking",
            constructor_name: deepseek_deepseek_v3_2_thinking,
            display_name: "Deepseek V3.2 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3370bInstruct {
            model_name: "meta-llama/llama-3.3-70b-instruct",
            constructor_name: meta_llama_llama_3_3_70b_instruct,
            display_name: "Llama 3.3 70b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama4Maverick {
            model_name: "meta-llama/llama-4-maverick",
            constructor_name: meta_llama_llama_4_maverick,
            display_name: "Llama 4 Maverick",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM21 {
            model_name: "minimax/minimax-m2.1",
            constructor_name: minimax_minimax_m2_1,
            display_name: "Minimax M2.1",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM25 {
            model_name: "minimax/minimax-m2.5",
            constructor_name: minimax_minimax_m2_5,
            display_name: "MiniMax M2.5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM25Official {
            model_name: "minimax/minimax-m2.5-official",
            constructor_name: minimax_minimax_m2_5_official,
            display_name: "MiniMax M2.5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstral2123bInstruct2512 {
            model_name: "mistralai/devstral-2-123b-instruct-2512",
            constructor_name: mistralai_devstral_2_123b_instruct_2512,
            display_name: "Devstral 2 123b Instruct 2512",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMinistral14bInstruct2512 {
            model_name: "mistralai/ministral-14b-instruct-2512",
            constructor_name: mistralai_ministral_14b_instruct_2512,
            display_name: "Ministral 14b Instruct 2512",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralLarge3675bInstruct2512 {
            model_name: "mistralai/mistral-large-3-675b-instruct-2512",
            constructor_name: mistralai_mistral_large_3_675b_instruct_2512,
            display_name: "Mistral Large 3 675b Instruct 2512",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct {
            model_name: "moonshotai/kimi-k2-instruct",
            constructor_name: moonshotai_kimi_k2_instruct,
            display_name: "Kimi K2 Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/kimi-k2-thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/kimi-k2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25Thinking {
            model_name: "moonshotai/kimi-k2.5-thinking",
            constructor_name: moonshotai_kimi_k2_5_thinking,
            display_name: "Kimi K2.5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes4405bThinking {
            model_name: "nousresearch/hermes-4-405b:thinking",
            constructor_name: nousresearch_hermes_4_405b_thinking,
            display_name: "Hermes 4 405b Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaLlama33NemotronSuper49bV15 {
            model_name: "nvidia/llama-3_3-nemotron-super-49b-v1_5",
            constructor_name: nvidia_llama_3_3_nemotron_super_49b_v1_5,
            display_name: "Llama 3 3 Nemotron Super 49B V1 5",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT Oss 120b",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bThinking2507 {
            model_name: "qwen/qwen3-235b-a22b-thinking-2507",
            constructor_name: qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3 235B A22B Thinking 2507",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder {
            model_name: "qwen/qwen3-coder",
            constructor_name: qwen_qwen3_coder,
            display_name: "Qwen3 Coder",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45Air {
            model_name: "zai-org/glm-4.5-air",
            constructor_name: zai_org_glm_4_5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45AirThinking {
            model_name: "zai-org/glm-4.5-air:thinking",
            constructor_name: zai_org_glm_4_5_air_thinking,
            display_name: "GLM 4.5 Air Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46 {
            model_name: "zai-org/glm-4.6",
            constructor_name: zai_org_glm_4_6,
            display_name: "GLM 4.6",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46Thinking {
            model_name: "zai-org/glm-4.6:thinking",
            constructor_name: zai_org_glm_4_6_thinking,
            display_name: "GLM 4.6 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47 {
            model_name: "zai-org/glm-4.7",
            constructor_name: zai_org_glm_4_7,
            display_name: "GLM 4.7",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Thinking {
            model_name: "zai-org/glm-4.7:thinking",
            constructor_name: zai_org_glm_4_7_thinking,
            display_name: "GLM 4.7 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5 {
            model_name: "zai-org/glm-5",
            constructor_name: zai_org_glm_5,
            display_name: "GLM 5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5Original {
            model_name: "zai-org/glm-5-original",
            constructor_name: zai_org_glm_5_original,
            display_name: "GLM 5 Original",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5OriginalThinking {
            model_name: "zai-org/glm-5-original:thinking",
            constructor_name: zai_org_glm_5_original_thinking,
            display_name: "GLM 5 Original Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5Thinking {
            model_name: "zai-org/glm-5:thinking",
            constructor_name: zai_org_glm_5_thinking,
            display_name: "GLM 5 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
