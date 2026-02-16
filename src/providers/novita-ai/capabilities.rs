//! Capabilities for novita_ai models.
//!
//! This module defines model types and their capabilities for novita_ai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::novita_ai::NovitaAi;

model_capabilities! {
    provider: NovitaAi,
    models: {
        BaichuanBaichuanM232b {
            model_name: "baichuan/baichuan-m2-32b",
            constructor_name: baichuan_baichuan_m2_32b,
            display_name: "baichuan-m2-32b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BaiduErnie4521bA3b {
            model_name: "baidu/ernie-4.5-21B-a3b",
            constructor_name: baidu_ernie_4_5_21b_a3b,
            display_name: "ERNIE 4.5 21B A3B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BaiduErnie4521bA3bThinking {
            model_name: "baidu/ernie-4.5-21B-a3b-thinking",
            constructor_name: baidu_ernie_4_5_21b_a3b_thinking,
            display_name: "ERNIE-4.5-21B-A3B-Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        BaiduErnie45300bA47bPaddle {
            model_name: "baidu/ernie-4.5-300b-a47b-paddle",
            constructor_name: baidu_ernie_4_5_300b_a47b_paddle,
            display_name: "ERNIE 4.5 300B A47B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        BaiduErnie45Vl28bA3b {
            model_name: "baidu/ernie-4.5-vl-28b-a3b",
            constructor_name: baidu_ernie_4_5_vl_28b_a3b,
            display_name: "ERNIE 4.5 VL 28B A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BaiduErnie45Vl28bA3bThinking {
            model_name: "baidu/ernie-4.5-vl-28b-a3b-thinking",
            constructor_name: baidu_ernie_4_5_vl_28b_a3b_thinking,
            display_name: "ERNIE-4.5-VL-28B-A3B-Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        BaiduErnie45Vl424bA47b {
            model_name: "baidu/ernie-4.5-vl-424b-a47b",
            constructor_name: baidu_ernie_4_5_vl_424b_a47b,
            display_name: "ERNIE 4.5 VL 424B A47B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekOcr {
            model_name: "deepseek/deepseek-ocr",
            constructor_name: deepseek_deepseek_ocr,
            display_name: "DeepSeek-OCR",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekOcr2 {
            model_name: "deepseek/deepseek-ocr-2",
            constructor_name: deepseek_deepseek_ocr_2,
            display_name: "deepseek/deepseek-ocr-2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekProverV2671b {
            model_name: "deepseek/deepseek-prover-v2-671b",
            constructor_name: deepseek_deepseek_prover_v2_671b,
            display_name: "Deepseek Prover V2 671B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekR10528 {
            model_name: "deepseek/deepseek-r1-0528",
            constructor_name: deepseek_deepseek_r1_0528,
            display_name: "DeepSeek R1 0528",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekR10528Qwen38b {
            model_name: "deepseek/deepseek-r1-0528-qwen3-8b",
            constructor_name: deepseek_deepseek_r1_0528_qwen3_8b,
            display_name: "DeepSeek R1 0528 Qwen3 8B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekR1DistillLlama70b {
            model_name: "deepseek/deepseek-r1-distill-llama-70b",
            constructor_name: deepseek_deepseek_r1_distill_llama_70b,
            display_name: "DeepSeek R1 Distill LLama 70B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekR1Turbo {
            model_name: "deepseek/deepseek-r1-turbo",
            constructor_name: deepseek_deepseek_r1_turbo,
            display_name: "DeepSeek R1 (Turbo)	",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV30324 {
            model_name: "deepseek/deepseek-v3-0324",
            constructor_name: deepseek_deepseek_v3_0324,
            display_name: "DeepSeek V3 0324",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV3Turbo {
            model_name: "deepseek/deepseek-v3-turbo",
            constructor_name: deepseek_deepseek_v3_turbo,
            display_name: "DeepSeek V3 (Turbo)	",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV31 {
            model_name: "deepseek/deepseek-v3.1",
            constructor_name: deepseek_deepseek_v3_1,
            display_name: "DeepSeek V3.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV31Terminus {
            model_name: "deepseek/deepseek-v3.1-terminus",
            constructor_name: deepseek_deepseek_v3_1_terminus,
            display_name: "Deepseek V3.1 Terminus",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32 {
            model_name: "deepseek/deepseek-v3.2",
            constructor_name: deepseek_deepseek_v3_2,
            display_name: "Deepseek V3.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32Exp {
            model_name: "deepseek/deepseek-v3.2-exp",
            constructor_name: deepseek_deepseek_v3_2_exp,
            display_name: "Deepseek V3.2 Exp",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma327bIt {
            model_name: "google/gemma-3-27b-it",
            constructor_name: google_gemma_3_27b_it,
            display_name: "Gemma 3 27B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GrypheMythomaxL213b {
            model_name: "gryphe/mythomax-l2-13b",
            constructor_name: gryphe_mythomax_l2_13b,
            display_name: "Mythomax L2 13B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        KwaipilotKatCoder {
            model_name: "kwaipilot/kat-coder",
            constructor_name: kwaipilot_kat_coder,
            display_name: "KAT-Coder-Pro V1(Free)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KwaipilotKatCoderPro {
            model_name: "kwaipilot/kat-coder-pro",
            constructor_name: kwaipilot_kat_coder_pro,
            display_name: "Kat Coder Pro",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama370bInstruct {
            model_name: "meta-llama/llama-3-70b-instruct",
            constructor_name: meta_llama_llama_3_70b_instruct,
            display_name: "Llama3 70B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama38bInstruct {
            model_name: "meta-llama/llama-3-8b-instruct",
            constructor_name: meta_llama_llama_3_8b_instruct,
            display_name: "Llama 3 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama318bInstruct {
            model_name: "meta-llama/llama-3.1-8b-instruct",
            constructor_name: meta_llama_llama_3_1_8b_instruct,
            display_name: "Llama 3.1 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama3370bInstruct {
            model_name: "meta-llama/llama-3.3-70b-instruct",
            constructor_name: meta_llama_llama_3_3_70b_instruct,
            display_name: "Llama 3.3 70B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama4Maverick17b128eInstructFp8 {
            model_name: "meta-llama/llama-4-maverick-17b-128e-instruct-fp8",
            constructor_name: meta_llama_llama_4_maverick_17b_128e_instruct_fp8,
            display_name: "Llama 4 Maverick Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama4Scout17b16eInstruct {
            model_name: "meta-llama/llama-4-scout-17b-16e-instruct",
            constructor_name: meta_llama_llama_4_scout_17b_16e_instruct,
            display_name: "Llama 4 Scout Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MicrosoftWizardlm28x22b {
            model_name: "microsoft/wizardlm-2-8x22b",
            constructor_name: microsoft_wizardlm_2_8x22b,
            display_name: "Wizardlm 2 8x22B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MinimaxMinimaxM2 {
            model_name: "minimax/minimax-m2",
            constructor_name: minimax_minimax_m2,
            display_name: "MiniMax-M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        MinimaxaiMinimaxM180k {
            model_name: "minimaxai/minimax-m1-80k",
            constructor_name: minimaxai_minimax_m1_80k,
            display_name: "MiniMax M1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralNemo {
            model_name: "mistralai/mistral-nemo",
            constructor_name: mistralai_mistral_nemo,
            display_name: "Mistral Nemo",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        MoonshotaiKimiK20905 {
            model_name: "moonshotai/kimi-k2-0905",
            constructor_name: moonshotai_kimi_k2_0905,
            display_name: "Kimi K2 0905",
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NousresearchHermes2ProLlama38b {
            model_name: "nousresearch/hermes-2-pro-llama-3-8b",
            constructor_name: nousresearch_hermes_2_pro_llama_3_8b,
            display_name: "Hermes 2 Pro Llama 3 8B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "OpenAI GPT OSS 120B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "OpenAI: GPT OSS 20B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        PaddlepaddlePaddleocrVl {
            model_name: "paddlepaddle/paddleocr-vl",
            constructor_name: paddlepaddle_paddleocr_vl,
            display_name: "PaddleOCR-VL",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen2572bInstruct {
            model_name: "qwen/qwen-2.5-72b-instruct",
            constructor_name: qwen_qwen_2_5_72b_instruct,
            display_name: "Qwen 2.5 72B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwenMtPlus {
            model_name: "qwen/qwen-mt-plus",
            constructor_name: qwen_qwen_mt_plus,
            display_name: "Qwen MT Plus",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen257bInstruct {
            model_name: "qwen/qwen2.5-7b-instruct",
            constructor_name: qwen_qwen2_5_7b_instruct,
            display_name: "Qwen2.5 7B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen25Vl72bInstruct {
            model_name: "qwen/qwen2.5-vl-72b-instruct",
            constructor_name: qwen_qwen2_5_vl_72b_instruct,
            display_name: "Qwen2.5 VL 72B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen3235bA22bFp8 {
            model_name: "qwen/qwen3-235b-a22b-fp8",
            constructor_name: qwen_qwen3_235b_a22b_fp8,
            display_name: "Qwen3 235B A22B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen3235bA22bInstruct2507 {
            model_name: "qwen/qwen3-235b-a22b-instruct-2507",
            constructor_name: qwen_qwen3_235b_a22b_instruct_2507,
            display_name: "Qwen3 235B A22B Instruct 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bThinking2507 {
            model_name: "qwen/qwen3-235b-a22b-thinking-2507",
            constructor_name: qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3 235B A22b Thinking 2507",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3bFp8 {
            model_name: "qwen/qwen3-30b-a3b-fp8",
            constructor_name: qwen_qwen3_30b_a3b_fp8,
            display_name: "Qwen3 30B A3B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen332bFp8 {
            model_name: "qwen/qwen3-32b-fp8",
            constructor_name: qwen_qwen3_32b_fp8,
            display_name: "Qwen3 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen34bFp8 {
            model_name: "qwen/qwen3-4b-fp8",
            constructor_name: qwen_qwen3_4b_fp8,
            display_name: "Qwen3 4B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen38bFp8 {
            model_name: "qwen/qwen3-8b-fp8",
            constructor_name: qwen_qwen3_8b_fp8,
            display_name: "Qwen3 8B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Coder30bA3bInstruct {
            model_name: "qwen/qwen3-coder-30b-a3b-instruct",
            constructor_name: qwen_qwen3_coder_30b_a3b_instruct,
            display_name: "Qwen3 Coder 30b A3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder480bA35bInstruct {
            model_name: "qwen/qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen_qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderNext {
            model_name: "qwen/qwen3-coder-next",
            constructor_name: qwen_qwen3_coder_next,
            display_name: "Qwen3 Coder Next",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Max {
            model_name: "qwen/qwen3-max",
            constructor_name: qwen_qwen3_max,
            display_name: "Qwen3 Max",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bInstruct {
            model_name: "qwen/qwen3-next-80b-a3b-instruct",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3 Next 80B A3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bThinking {
            model_name: "qwen/qwen3-next-80b-a3b-thinking",
            constructor_name: qwen_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3 Next 80B A3B Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Omni30bA3bInstruct {
            model_name: "qwen/qwen3-omni-30b-a3b-instruct",
            constructor_name: qwen_qwen3_omni_30b_a3b_instruct,
            display_name: "Qwen3 Omni 30B A3B Instruct",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3Omni30bA3bThinking {
            model_name: "qwen/qwen3-omni-30b-a3b-thinking",
            constructor_name: qwen_qwen3_omni_30b_a3b_thinking,
            display_name: "Qwen3 Omni 30B A3B Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3Vl235bA22bInstruct {
            model_name: "qwen/qwen3-vl-235b-a22b-instruct",
            constructor_name: qwen_qwen3_vl_235b_a22b_instruct,
            display_name: "Qwen3 VL 235B A22B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3Vl235bA22bThinking {
            model_name: "qwen/qwen3-vl-235b-a22b-thinking",
            constructor_name: qwen_qwen3_vl_235b_a22b_thinking,
            display_name: "Qwen3 VL 235B A22B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen3Vl30bA3bInstruct {
            model_name: "qwen/qwen3-vl-30b-a3b-instruct",
            constructor_name: qwen_qwen3_vl_30b_a3b_instruct,
            display_name: "qwen/qwen3-vl-30b-a3b-instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3Vl30bA3bThinking {
            model_name: "qwen/qwen3-vl-30b-a3b-thinking",
            constructor_name: qwen_qwen3_vl_30b_a3b_thinking,
            display_name: "qwen/qwen3-vl-30b-a3b-thinking",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3Vl8bInstruct {
            model_name: "qwen/qwen3-vl-8b-instruct",
            constructor_name: qwen_qwen3_vl_8b_instruct,
            display_name: "qwen/qwen3-vl-8b-instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Sao10kL38bSthenoV32 {
            model_name: "sao10k/L3-8B-Stheno-v3.2",
            constructor_name: sao10k_l3_8b_stheno_v3_2,
            display_name: "L3 8B Stheno V3.2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Sao10kL370bEuryaleV21 {
            model_name: "sao10k/l3-70b-euryale-v2.1",
            constructor_name: sao10k_l3_70b_euryale_v2_1,
            display_name: "L3 70B Euryale V2.1	",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Sao10kL38bLunaris {
            model_name: "sao10k/l3-8b-lunaris",
            constructor_name: sao10k_l3_8b_lunaris,
            display_name: "Sao10k L3 8B Lunaris	",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Sao10kL3170bEuryaleV22 {
            model_name: "sao10k/l31-70b-euryale-v2.2",
            constructor_name: sao10k_l31_70b_euryale_v2_2,
            display_name: "L31 70B Euryale V2.2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SkyworkR1v4Lite {
            model_name: "skywork/r1v4-lite",
            constructor_name: skywork_r1v4_lite,
            display_name: "Skywork R1V4-Lite",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        XiaomimimoMimoV2Flash {
            model_name: "xiaomimimo/mimo-v2-flash",
            constructor_name: xiaomimimo_mimo_v2_flash,
            display_name: "XiaomiMiMo/MiMo-V2-Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgAutoglmPhone9bMultilingual {
            model_name: "zai-org/autoglm-phone-9b-multilingual",
            constructor_name: zai_org_autoglm_phone_9b_multilingual,
            display_name: "AutoGLM-Phone-9B-Multilingual",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm45 {
            model_name: "zai-org/glm-4.5",
            constructor_name: zai_org_glm_4_5,
            display_name: "GLM-4.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45Air {
            model_name: "zai-org/glm-4.5-air",
            constructor_name: zai_org_glm_4_5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45v {
            model_name: "zai-org/glm-4.5v",
            constructor_name: zai_org_glm_4_5v,
            display_name: "GLM 4.5V",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZaiOrgGlm46 {
            model_name: "zai-org/glm-4.6",
            constructor_name: zai_org_glm_4_6,
            display_name: "GLM 4.6",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm46v {
            model_name: "zai-org/glm-4.6v",
            constructor_name: zai_org_glm_4_6v,
            display_name: "GLM 4.6V",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZaiOrgGlm47 {
            model_name: "zai-org/glm-4.7",
            constructor_name: zai_org_glm_4_7,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Flash {
            model_name: "zai-org/glm-4.7-flash",
            constructor_name: zai_org_glm_4_7_flash,
            display_name: "GLM-4.7-Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5 {
            model_name: "zai-org/glm-5",
            constructor_name: zai_org_glm_5,
            display_name: "GLM-5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
