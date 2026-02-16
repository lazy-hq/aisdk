//! Capabilities for siliconflow_cn models.
//!
//! This module defines model types and their capabilities for siliconflow_cn providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::siliconflow_cn::SiliconflowCn;

model_capabilities! {
    provider: SiliconflowCn,
    models: {
        BytedanceSeedSeedOss36bInstruct {
            model_name: "ByteDance-Seed/Seed-OSS-36B-Instruct",
            constructor_name: bytedance_seed_seed_oss_36b_instruct,
            display_name: "ByteDance-Seed/Seed-OSS-36B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KwaipilotKatDev {
            model_name: "Kwaipilot/KAT-Dev",
            constructor_name: kwaipilot_kat_dev,
            display_name: "Kwaipilot/KAT-Dev",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxaiMinimaxM21 {
            model_name: "Pro/MiniMaxAI/MiniMax-M2.1",
            constructor_name: minimaxai_minimax_m2_1,
            display_name: "Pro/MiniMaxAI/MiniMax-M2.1",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR1 {
            model_name: "Pro/deepseek-ai/DeepSeek-R1",
            constructor_name: deepseek_ai_deepseek_r1,
            display_name: "Pro/deepseek-ai/DeepSeek-R1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV3 {
            model_name: "Pro/deepseek-ai/DeepSeek-V3",
            constructor_name: deepseek_ai_deepseek_v3,
            display_name: "Pro/deepseek-ai/DeepSeek-V3",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31Terminus {
            model_name: "Pro/deepseek-ai/DeepSeek-V3.1-Terminus",
            constructor_name: deepseek_ai_deepseek_v3_1_terminus,
            display_name: "Pro/deepseek-ai/DeepSeek-V3.1-Terminus",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV32 {
            model_name: "Pro/deepseek-ai/DeepSeek-V3.2",
            constructor_name: deepseek_ai_deepseek_v3_2,
            display_name: "Pro/deepseek-ai/DeepSeek-V3.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct0905 {
            model_name: "Pro/moonshotai/Kimi-K2-Instruct-0905",
            constructor_name: moonshotai_kimi_k2_instruct_0905,
            display_name: "Pro/moonshotai/Kimi-K2-Instruct-0905",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "Pro/moonshotai/Kimi-K2-Thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Pro/moonshotai/Kimi-K2-Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "Pro/moonshotai/Kimi-K2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Pro/moonshotai/Kimi-K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47 {
            model_name: "Pro/zai-org/GLM-4.7",
            constructor_name: zai_org_glm_4_7,
            display_name: "Pro/zai-org/GLM-4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5 {
            model_name: "Pro/zai-org/GLM-5",
            constructor_name: zai_org_glm_5,
            display_name: "Pro/zai-org/GLM-5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwq32b {
            model_name: "Qwen/QwQ-32B",
            constructor_name: qwen_qwq_32b,
            display_name: "Qwen/QwQ-32B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen2514bInstruct {
            model_name: "Qwen/Qwen2.5-14B-Instruct",
            constructor_name: qwen_qwen2_5_14b_instruct,
            display_name: "Qwen/Qwen2.5-14B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen2532bInstruct {
            model_name: "Qwen/Qwen2.5-32B-Instruct",
            constructor_name: qwen_qwen2_5_32b_instruct,
            display_name: "Qwen/Qwen2.5-32B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen2572bInstruct {
            model_name: "Qwen/Qwen2.5-72B-Instruct",
            constructor_name: qwen_qwen2_5_72b_instruct,
            display_name: "Qwen/Qwen2.5-72B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen2572bInstruct128k {
            model_name: "Qwen/Qwen2.5-72B-Instruct-128K",
            constructor_name: qwen_qwen2_5_72b_instruct_128k,
            display_name: "Qwen/Qwen2.5-72B-Instruct-128K",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen257bInstruct {
            model_name: "Qwen/Qwen2.5-7B-Instruct",
            constructor_name: qwen_qwen2_5_7b_instruct,
            display_name: "Qwen/Qwen2.5-7B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen25Coder32bInstruct {
            model_name: "Qwen/Qwen2.5-Coder-32B-Instruct",
            constructor_name: qwen_qwen2_5_coder_32b_instruct,
            display_name: "Qwen/Qwen2.5-Coder-32B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen25Vl32bInstruct {
            model_name: "Qwen/Qwen2.5-VL-32B-Instruct",
            constructor_name: qwen_qwen2_5_vl_32b_instruct,
            display_name: "Qwen/Qwen2.5-VL-32B-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen25Vl72bInstruct {
            model_name: "Qwen/Qwen2.5-VL-72B-Instruct",
            constructor_name: qwen_qwen2_5_vl_72b_instruct,
            display_name: "Qwen/Qwen2.5-VL-72B-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen314b {
            model_name: "Qwen/Qwen3-14B",
            constructor_name: qwen_qwen3_14b,
            display_name: "Qwen/Qwen3-14B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bInstruct2507 {
            model_name: "Qwen/Qwen3-235B-A22B-Instruct-2507",
            constructor_name: qwen_qwen3_235b_a22b_instruct_2507,
            display_name: "Qwen/Qwen3-235B-A22B-Instruct-2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bThinking2507 {
            model_name: "Qwen/Qwen3-235B-A22B-Thinking-2507",
            constructor_name: qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen/Qwen3-235B-A22B-Thinking-2507",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3bInstruct2507 {
            model_name: "Qwen/Qwen3-30B-A3B-Instruct-2507",
            constructor_name: qwen_qwen3_30b_a3b_instruct_2507,
            display_name: "Qwen/Qwen3-30B-A3B-Instruct-2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3bThinking2507 {
            model_name: "Qwen/Qwen3-30B-A3B-Thinking-2507",
            constructor_name: qwen_qwen3_30b_a3b_thinking_2507,
            display_name: "Qwen/Qwen3-30B-A3B-Thinking-2507",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen332b {
            model_name: "Qwen/Qwen3-32B",
            constructor_name: qwen_qwen3_32b,
            display_name: "Qwen/Qwen3-32B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen38b {
            model_name: "Qwen/Qwen3-8B",
            constructor_name: qwen_qwen3_8b,
            display_name: "Qwen/Qwen3-8B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder30bA3bInstruct {
            model_name: "Qwen/Qwen3-Coder-30B-A3B-Instruct",
            constructor_name: qwen_qwen3_coder_30b_a3b_instruct,
            display_name: "Qwen/Qwen3-Coder-30B-A3B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder480bA35bInstruct {
            model_name: "Qwen/Qwen3-Coder-480B-A35B-Instruct",
            constructor_name: qwen_qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen/Qwen3-Coder-480B-A35B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bInstruct {
            model_name: "Qwen/Qwen3-Next-80B-A3B-Instruct",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen/Qwen3-Next-80B-A3B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bThinking {
            model_name: "Qwen/Qwen3-Next-80B-A3B-Thinking",
            constructor_name: qwen_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen/Qwen3-Next-80B-A3B-Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Omni30bA3bCaptioner {
            model_name: "Qwen/Qwen3-Omni-30B-A3B-Captioner",
            constructor_name: qwen_qwen3_omni_30b_a3b_captioner,
            display_name: "Qwen/Qwen3-Omni-30B-A3B-Captioner",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Omni30bA3bInstruct {
            model_name: "Qwen/Qwen3-Omni-30B-A3B-Instruct",
            constructor_name: qwen_qwen3_omni_30b_a3b_instruct,
            display_name: "Qwen/Qwen3-Omni-30B-A3B-Instruct",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Omni30bA3bThinking {
            model_name: "Qwen/Qwen3-Omni-30B-A3B-Thinking",
            constructor_name: qwen_qwen3_omni_30b_a3b_thinking,
            display_name: "Qwen/Qwen3-Omni-30B-A3B-Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl235bA22bInstruct {
            model_name: "Qwen/Qwen3-VL-235B-A22B-Instruct",
            constructor_name: qwen_qwen3_vl_235b_a22b_instruct,
            display_name: "Qwen/Qwen3-VL-235B-A22B-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl235bA22bThinking {
            model_name: "Qwen/Qwen3-VL-235B-A22B-Thinking",
            constructor_name: qwen_qwen3_vl_235b_a22b_thinking,
            display_name: "Qwen/Qwen3-VL-235B-A22B-Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl30bA3bInstruct {
            model_name: "Qwen/Qwen3-VL-30B-A3B-Instruct",
            constructor_name: qwen_qwen3_vl_30b_a3b_instruct,
            display_name: "Qwen/Qwen3-VL-30B-A3B-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl30bA3bThinking {
            model_name: "Qwen/Qwen3-VL-30B-A3B-Thinking",
            constructor_name: qwen_qwen3_vl_30b_a3b_thinking,
            display_name: "Qwen/Qwen3-VL-30B-A3B-Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl32bInstruct {
            model_name: "Qwen/Qwen3-VL-32B-Instruct",
            constructor_name: qwen_qwen3_vl_32b_instruct,
            display_name: "Qwen/Qwen3-VL-32B-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl32bThinking {
            model_name: "Qwen/Qwen3-VL-32B-Thinking",
            constructor_name: qwen_qwen3_vl_32b_thinking,
            display_name: "Qwen/Qwen3-VL-32B-Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl8bInstruct {
            model_name: "Qwen/Qwen3-VL-8B-Instruct",
            constructor_name: qwen_qwen3_vl_8b_instruct,
            display_name: "Qwen/Qwen3-VL-8B-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl8bThinking {
            model_name: "Qwen/Qwen3-VL-8B-Thinking",
            constructor_name: qwen_qwen3_vl_8b_thinking,
            display_name: "Qwen/Qwen3-VL-8B-Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThudmGlm432b0414 {
            model_name: "THUDM/GLM-4-32B-0414",
            constructor_name: thudm_glm_4_32b_0414,
            display_name: "THUDM/GLM-4-32B-0414",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThudmGlm49b0414 {
            model_name: "THUDM/GLM-4-9B-0414",
            constructor_name: thudm_glm_4_9b_0414,
            display_name: "THUDM/GLM-4-9B-0414",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThudmGlmZ132b0414 {
            model_name: "THUDM/GLM-Z1-32B-0414",
            constructor_name: thudm_glm_z1_32b_0414,
            display_name: "THUDM/GLM-Z1-32B-0414",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThudmGlmZ19b0414 {
            model_name: "THUDM/GLM-Z1-9B-0414",
            constructor_name: thudm_glm_z1_9b_0414,
            display_name: "THUDM/GLM-Z1-9B-0414",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AscendTribePanguProMoe {
            model_name: "ascend-tribe/pangu-pro-moe",
            constructor_name: ascend_tribe_pangu_pro_moe,
            display_name: "ascend-tribe/pangu-pro-moe",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        BaiduErnie45300bA47b {
            model_name: "baidu/ERNIE-4.5-300B-A47B",
            constructor_name: baidu_ernie_4_5_300b_a47b,
            display_name: "baidu/ERNIE-4.5-300B-A47B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR1 {
            model_name: "deepseek-ai/DeepSeek-R1",
            constructor_name: deepseek_ai_deepseek_r1,
            display_name: "deepseek-ai/DeepSeek-R1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR1DistillQwen14b {
            model_name: "deepseek-ai/DeepSeek-R1-Distill-Qwen-14B",
            constructor_name: deepseek_ai_deepseek_r1_distill_qwen_14b,
            display_name: "deepseek-ai/DeepSeek-R1-Distill-Qwen-14B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR1DistillQwen32b {
            model_name: "deepseek-ai/DeepSeek-R1-Distill-Qwen-32B",
            constructor_name: deepseek_ai_deepseek_r1_distill_qwen_32b,
            display_name: "deepseek-ai/DeepSeek-R1-Distill-Qwen-32B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV3 {
            model_name: "deepseek-ai/DeepSeek-V3",
            constructor_name: deepseek_ai_deepseek_v3,
            display_name: "deepseek-ai/DeepSeek-V3",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31Terminus {
            model_name: "deepseek-ai/DeepSeek-V3.1-Terminus",
            constructor_name: deepseek_ai_deepseek_v3_1_terminus,
            display_name: "deepseek-ai/DeepSeek-V3.1-Terminus",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV32 {
            model_name: "deepseek-ai/DeepSeek-V3.2",
            constructor_name: deepseek_ai_deepseek_v3_2,
            display_name: "deepseek-ai/DeepSeek-V3.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekVl2 {
            model_name: "deepseek-ai/deepseek-vl2",
            constructor_name: deepseek_ai_deepseek_vl2,
            display_name: "deepseek-ai/deepseek-vl2",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLingFlash20 {
            model_name: "inclusionAI/Ling-flash-2.0",
            constructor_name: inclusionai_ling_flash_2_0,
            display_name: "inclusionAI/Ling-flash-2.0",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLingMini20 {
            model_name: "inclusionAI/Ling-mini-2.0",
            constructor_name: inclusionai_ling_mini_2_0,
            display_name: "inclusionAI/Ling-mini-2.0",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiRingFlash20 {
            model_name: "inclusionAI/Ring-flash-2.0",
            constructor_name: inclusionai_ring_flash_2_0,
            display_name: "inclusionAI/Ring-flash-2.0",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct0905 {
            model_name: "moonshotai/Kimi-K2-Instruct-0905",
            constructor_name: moonshotai_kimi_k2_instruct_0905,
            display_name: "moonshotai/Kimi-K2-Instruct-0905",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/Kimi-K2-Thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "moonshotai/Kimi-K2-Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        StepfunAiStep35Flash {
            model_name: "stepfun-ai/Step-3.5-Flash",
            constructor_name: stepfun_ai_step_3_5_flash,
            display_name: "stepfun-ai/Step-3.5-Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHunyuanA13bInstruct {
            model_name: "tencent/Hunyuan-A13B-Instruct",
            constructor_name: tencent_hunyuan_a13b_instruct,
            display_name: "tencent/Hunyuan-A13B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHunyuanMt7b {
            model_name: "tencent/Hunyuan-MT-7B",
            constructor_name: tencent_hunyuan_mt_7b,
            display_name: "tencent/Hunyuan-MT-7B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45Air {
            model_name: "zai-org/GLM-4.5-Air",
            constructor_name: zai_org_glm_4_5_air,
            display_name: "zai-org/GLM-4.5-Air",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45v {
            model_name: "zai-org/GLM-4.5V",
            constructor_name: zai_org_glm_4_5v,
            display_name: "zai-org/GLM-4.5V",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46 {
            model_name: "zai-org/GLM-4.6",
            constructor_name: zai_org_glm_4_6,
            display_name: "zai-org/GLM-4.6",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46v {
            model_name: "zai-org/GLM-4.6V",
            constructor_name: zai_org_glm_4_6v,
            display_name: "zai-org/GLM-4.6V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
