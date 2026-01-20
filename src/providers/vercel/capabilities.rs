//! Capabilities for vercel models.
//!
//! This module defines model types and their capabilities for vercel providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::vercel::Vercel;

model_capabilities! {
    provider: Vercel,
    models: {
        AlibabaQwen314b {
            model_name: "alibaba/qwen-3-14b",
            constructor_name: alibaba_qwen_3_14b,
            display_name: "Qwen3-14B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3235b {
            model_name: "alibaba/qwen-3-235b",
            constructor_name: alibaba_qwen_3_235b,
            display_name: "Qwen3 235B A22B Instruct 2507",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3235bA22bThinking {
            model_name: "alibaba/qwen3-235b-a22b-thinking",
            constructor_name: alibaba_qwen3_235b_a22b_thinking,
            display_name: "Qwen3 235B A22B Thinking 2507",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen330b {
            model_name: "alibaba/qwen-3-30b",
            constructor_name: alibaba_qwen_3_30b,
            display_name: "Qwen3-30B-A3B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen332b {
            model_name: "alibaba/qwen-3-32b",
            constructor_name: alibaba_qwen_3_32b,
            display_name: "Qwen 3.32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Coder {
            model_name: "alibaba/qwen3-coder",
            constructor_name: alibaba_qwen3_coder,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Coder30bA3b {
            model_name: "alibaba/qwen3-coder-30b-a3b",
            constructor_name: alibaba_qwen3_coder_30b_a3b,
            display_name: "Qwen 3 Coder 30B A3B Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3CoderPlus {
            model_name: "alibaba/qwen3-coder-plus",
            constructor_name: alibaba_qwen3_coder_plus,
            display_name: "Qwen3 Coder Plus",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Embedding06b {
            model_name: "alibaba/qwen3-embedding-0.6b",
            constructor_name: alibaba_qwen3_embedding_0_6b,
            display_name: "Qwen3 Embedding 0.6B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AlibabaQwen3Embedding4b {
            model_name: "alibaba/qwen3-embedding-4b",
            constructor_name: alibaba_qwen3_embedding_4b,
            display_name: "Qwen3 Embedding 4B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AlibabaQwen3Embedding8b {
            model_name: "alibaba/qwen3-embedding-8b",
            constructor_name: alibaba_qwen3_embedding_8b,
            display_name: "Qwen3 Embedding 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AlibabaQwen3Max {
            model_name: "alibaba/qwen3-max",
            constructor_name: alibaba_qwen3_max,
            display_name: "Qwen3 Max",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3MaxPreview {
            model_name: "alibaba/qwen3-max-preview",
            constructor_name: alibaba_qwen3_max_preview,
            display_name: "Qwen3 Max Preview",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Next80bA3bInstruct {
            model_name: "alibaba/qwen3-next-80b-a3b-instruct",
            constructor_name: alibaba_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3 Next 80B A3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Next80bA3bThinking {
            model_name: "alibaba/qwen3-next-80b-a3b-thinking",
            constructor_name: alibaba_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3 Next 80B A3B Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3VlInstruct {
            model_name: "alibaba/qwen3-vl-instruct",
            constructor_name: alibaba_qwen3_vl_instruct,
            display_name: "Qwen3 VL Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3VlThinking {
            model_name: "alibaba/qwen3-vl-thinking",
            constructor_name: alibaba_qwen3_vl_thinking,
            display_name: "Qwen3 VL Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AmazonNova2Lite {
            model_name: "amazon/nova-2-lite",
            constructor_name: amazon_nova_2_lite,
            display_name: "Nova 2 Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        AmazonNovaLite {
            model_name: "amazon/nova-lite",
            constructor_name: amazon_nova_lite,
            display_name: "Nova Lite",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonNovaMicro {
            model_name: "amazon/nova-micro",
            constructor_name: amazon_nova_micro,
            display_name: "Nova Micro",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AmazonNovaPro {
            model_name: "amazon/nova-pro",
            constructor_name: amazon_nova_pro,
            display_name: "Nova Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonTitanEmbedTextV2 {
            model_name: "amazon/titan-embed-text-v2",
            constructor_name: amazon_titan_embed_text_v2,
            display_name: "Titan Text Embeddings V2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AnthropicClaude35Haiku {
            model_name: "anthropic/claude-3.5-haiku",
            constructor_name: anthropic_claude_3_5_haiku,
            display_name: "Claude Haiku 3.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude35Sonnet {
            model_name: "anthropic/claude-3.5-sonnet",
            constructor_name: anthropic_claude_3_5_sonnet,
            display_name: "Claude Sonnet 3.5 v2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude35Sonnet20240620 {
            model_name: "anthropic/claude-3.5-sonnet-20240620",
            constructor_name: anthropic_claude_3_5_sonnet_20240620,
            display_name: "Claude 3.5 Sonnet (2024-06-20)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude37Sonnet {
            model_name: "anthropic/claude-3.7-sonnet",
            constructor_name: anthropic_claude_3_7_sonnet,
            display_name: "Claude Sonnet 3.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude3Haiku {
            model_name: "anthropic/claude-3-haiku",
            constructor_name: anthropic_claude_3_haiku,
            display_name: "Claude Haiku 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaude3Opus {
            model_name: "anthropic/claude-3-opus",
            constructor_name: anthropic_claude_3_opus,
            display_name: "Claude Opus 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku45 {
            model_name: "anthropic/claude-haiku-4.5",
            constructor_name: anthropic_claude_haiku_4_5,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus4 {
            model_name: "anthropic/claude-opus-4",
            constructor_name: anthropic_claude_opus_4,
            display_name: "Claude Opus 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus41 {
            model_name: "anthropic/claude-opus-4.1",
            constructor_name: anthropic_claude_opus_4_1,
            display_name: "Claude Opus 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus45 {
            model_name: "anthropic/claude-opus-4.5",
            constructor_name: anthropic_claude_opus_4_5,
            display_name: "Claude Opus 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet4 {
            model_name: "anthropic/claude-sonnet-4",
            constructor_name: anthropic_claude_sonnet_4,
            display_name: "Claude Sonnet 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet45 {
            model_name: "anthropic/claude-sonnet-4.5",
            constructor_name: anthropic_claude_sonnet_4_5,
            display_name: "Claude Sonnet 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ArceeAiTrinityMini {
            model_name: "arcee-ai/trinity-mini",
            constructor_name: arcee_ai_trinity_mini,
            display_name: "Trinity Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BflFluxKontextMax {
            model_name: "bfl/flux-kontext-max",
            constructor_name: bfl_flux_kontext_max,
            display_name: "FLUX.1 Kontext Max",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFluxKontextPro {
            model_name: "bfl/flux-kontext-pro",
            constructor_name: bfl_flux_kontext_pro,
            display_name: "FLUX.1 Kontext Pro",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFluxPro10Fill {
            model_name: "bfl/flux-pro-1.0-fill",
            constructor_name: bfl_flux_pro_1_0_fill,
            display_name: "FLUX.1 Fill \\[pro\\]",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFluxPro11 {
            model_name: "bfl/flux-pro-1.1",
            constructor_name: bfl_flux_pro_1_1,
            display_name: "FLUX1.1 \\[pro\\]",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFluxPro11Ultra {
            model_name: "bfl/flux-pro-1.1-ultra",
            constructor_name: bfl_flux_pro_1_1_ultra,
            display_name: "FLUX1.1 \\[pro\\] Ultra",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BytedanceSeed16 {
            model_name: "bytedance/seed-1.6",
            constructor_name: bytedance_seed_1_6,
            display_name: "Seed 1.6",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BytedanceSeed18 {
            model_name: "bytedance/seed-1.8",
            constructor_name: bytedance_seed_1_8,
            display_name: "Seed 1.8",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereCommandA {
            model_name: "cohere/command-a",
            constructor_name: cohere_command_a,
            display_name: "Command A",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereEmbedV40 {
            model_name: "cohere/embed-v4.0",
            constructor_name: cohere_embed_v4_0,
            display_name: "Embed v4.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekR1 {
            model_name: "deepseek/deepseek-r1",
            constructor_name: deepseek_deepseek_r1,
            display_name: "DeepSeek-R1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV3 {
            model_name: "deepseek/deepseek-v3",
            constructor_name: deepseek_deepseek_v3,
            display_name: "DeepSeek V3 0324",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV31 {
            model_name: "deepseek/deepseek-v3.1",
            constructor_name: deepseek_deepseek_v3_1,
            display_name: "DeepSeek-V3.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV31Terminus {
            model_name: "deepseek/deepseek-v3.1-terminus",
            constructor_name: deepseek_deepseek_v3_1_terminus,
            display_name: "DeepSeek V3.1 Terminus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32 {
            model_name: "deepseek/deepseek-v3.2",
            constructor_name: deepseek_deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekV32Exp {
            model_name: "deepseek/deepseek-v3.2-exp",
            constructor_name: deepseek_deepseek_v3_2_exp,
            display_name: "DeepSeek V3.2 Exp",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32Thinking {
            model_name: "deepseek/deepseek-v3.2-thinking",
            constructor_name: deepseek_deepseek_v3_2_thinking,
            display_name: "DeepSeek V3.2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini20Flash {
            model_name: "google/gemini-2.0-flash",
            constructor_name: google_gemini_2_0_flash,
            display_name: "Gemini 2.0 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini20FlashLite {
            model_name: "google/gemini-2.0-flash-lite",
            constructor_name: google_gemini_2_0_flash_lite,
            display_name: "Gemini 2.0 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25Flash {
            model_name: "google/gemini-2.5-flash",
            constructor_name: google_gemini_2_5_flash,
            display_name: "Gemini 2.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25FlashImage {
            model_name: "google/gemini-2.5-flash-image",
            constructor_name: google_gemini_2_5_flash_image,
            display_name: "Nano Banana (Gemini 2.5 Flash Image)",
            capabilities: [ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini25FlashImagePreview {
            model_name: "google/gemini-2.5-flash-image-preview",
            constructor_name: google_gemini_2_5_flash_image_preview,
            display_name: "Nano Banana Preview (Gemini 2.5 Flash Image Preview)",
            capabilities: [ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini25FlashLite {
            model_name: "google/gemini-2.5-flash-lite",
            constructor_name: google_gemini_2_5_flash_lite,
            display_name: "Gemini 2.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25FlashLitePreview092025 {
            model_name: "google/gemini-2.5-flash-lite-preview-09-2025",
            constructor_name: google_gemini_2_5_flash_lite_preview_09_2025,
            display_name: "Gemini 2.5 Flash Lite Preview 09-25",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25FlashPreview092025 {
            model_name: "google/gemini-2.5-flash-preview-09-2025",
            constructor_name: google_gemini_2_5_flash_preview_09_2025,
            display_name: "Gemini 2.5 Flash Preview 09-25",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25Pro {
            model_name: "google/gemini-2.5-pro",
            constructor_name: google_gemini_2_5_pro,
            display_name: "Gemini 2.5 Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini3Flash {
            model_name: "google/gemini-3-flash",
            constructor_name: google_gemini_3_flash,
            display_name: "Gemini 3 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini3ProImage {
            model_name: "google/gemini-3-pro-image",
            constructor_name: google_gemini_3_pro_image,
            display_name: "Nano Banana Pro (Gemini 3 Pro Image)",
            capabilities: [ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini3ProPreview {
            model_name: "google/gemini-3-pro-preview",
            constructor_name: google_gemini_3_pro_preview,
            display_name: "Gemini 3 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGeminiEmbedding001 {
            model_name: "google/gemini-embedding-001",
            constructor_name: google_gemini_embedding_001,
            display_name: "Gemini Embedding 001",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleImagen40FastGenerate001 {
            model_name: "google/imagen-4.0-fast-generate-001",
            constructor_name: google_imagen_4_0_fast_generate_001,
            display_name: "Imagen 4 Fast",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        GoogleImagen40Generate001 {
            model_name: "google/imagen-4.0-generate-001",
            constructor_name: google_imagen_4_0_generate_001,
            display_name: "Imagen 4",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        GoogleImagen40UltraGenerate001 {
            model_name: "google/imagen-4.0-ultra-generate-001",
            constructor_name: google_imagen_4_0_ultra_generate_001,
            display_name: "Imagen 4 Ultra",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        GoogleTextEmbedding005 {
            model_name: "google/text-embedding-005",
            constructor_name: google_text_embedding_005,
            display_name: "Text Embedding 005",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleTextMultilingualEmbedding002 {
            model_name: "google/text-multilingual-embedding-002",
            constructor_name: google_text_multilingual_embedding_002,
            display_name: "Text Multilingual Embedding 002",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        InceptionMercuryCoderSmall {
            model_name: "inception/mercury-coder-small",
            constructor_name: inception_mercury_coder_small,
            display_name: "Mercury Coder Small Beta",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KwaipilotKatCoderProV1 {
            model_name: "kwaipilot/kat-coder-pro-v1",
            constructor_name: kwaipilot_kat_coder_pro_v1,
            display_name: "KAT-Coder-Pro V1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        MeituanLongcatFlashChat {
            model_name: "meituan/longcat-flash-chat",
            constructor_name: meituan_longcat_flash_chat,
            display_name: "LongCat Flash Chat",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MeituanLongcatFlashThinking {
            model_name: "meituan/longcat-flash-thinking",
            constructor_name: meituan_longcat_flash_thinking,
            display_name: "LongCat Flash Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3170b {
            model_name: "meta/llama-3.1-70b",
            constructor_name: meta_llama_3_1_70b,
            display_name: "Llama 3.1 70B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama318b {
            model_name: "meta/llama-3.1-8b",
            constructor_name: meta_llama_3_1_8b,
            display_name: "Llama 3.1 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3211b {
            model_name: "meta/llama-3.2-11b",
            constructor_name: meta_llama_3_2_11b,
            display_name: "Llama 3.2 11B Vision Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama321b {
            model_name: "meta/llama-3.2-1b",
            constructor_name: meta_llama_3_2_1b,
            display_name: "Llama 3.2 1B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama323b {
            model_name: "meta/llama-3.2-3b",
            constructor_name: meta_llama_3_2_3b,
            display_name: "Llama 3.2 3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama3290b {
            model_name: "meta/llama-3.2-90b",
            constructor_name: meta_llama_3_2_90b,
            display_name: "Llama 3.2 90B Vision Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3370b {
            model_name: "meta/llama-3.3-70b",
            constructor_name: meta_llama_3_3_70b,
            display_name: "Llama-3.3-70B-Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama4Maverick {
            model_name: "meta/llama-4-maverick",
            constructor_name: meta_llama_4_maverick,
            display_name: "Llama-4-Maverick-17B-128E-Instruct-FP8",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama4Scout {
            model_name: "meta/llama-4-scout",
            constructor_name: meta_llama_4_scout,
            display_name: "Llama-4-Scout-17B-16E-Instruct-FP8",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM2 {
            model_name: "minimax/minimax-m2",
            constructor_name: minimax_minimax_m2,
            display_name: "MiniMax M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM21 {
            model_name: "minimax/minimax-m2.1",
            constructor_name: minimax_minimax_m2_1,
            display_name: "MiniMax M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM21Lightning {
            model_name: "minimax/minimax-m2.1-lightning",
            constructor_name: minimax_minimax_m2_1_lightning,
            display_name: "MiniMax M2.1 Lightning",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralCodestral {
            model_name: "mistral/codestral",
            constructor_name: mistral_codestral,
            display_name: "Codestral",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralCodestralEmbed {
            model_name: "mistral/codestral-embed",
            constructor_name: mistral_codestral_embed,
            display_name: "Codestral Embed",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralDevstral2 {
            model_name: "mistral/devstral-2",
            constructor_name: mistral_devstral_2,
            display_name: "Devstral 2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralDevstralSmall {
            model_name: "mistral/devstral-small",
            constructor_name: mistral_devstral_small,
            display_name: "Devstral Small 1.1",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralDevstralSmall2 {
            model_name: "mistral/devstral-small-2",
            constructor_name: mistral_devstral_small_2,
            display_name: "Devstral Small 2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMagistralMedium {
            model_name: "mistral/magistral-medium",
            constructor_name: mistral_magistral_medium,
            display_name: "Magistral Medium",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMagistralSmall {
            model_name: "mistral/magistral-small",
            constructor_name: mistral_magistral_small,
            display_name: "Magistral Small",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMinistral14b {
            model_name: "mistral/ministral-14b",
            constructor_name: mistral_ministral_14b,
            display_name: "Ministral 14B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralMinistral3b {
            model_name: "mistral/ministral-3b",
            constructor_name: mistral_ministral_3b,
            display_name: "Ministral 3B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMinistral8b {
            model_name: "mistral/ministral-8b",
            constructor_name: mistral_ministral_8b,
            display_name: "Ministral 8B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralEmbed {
            model_name: "mistral/mistral-embed",
            constructor_name: mistral_mistral_embed,
            display_name: "Mistral Embed",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralMistralLarge3 {
            model_name: "mistral/mistral-large-3",
            constructor_name: mistral_mistral_large_3,
            display_name: "Mistral Large 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralMistralMedium {
            model_name: "mistral/mistral-medium",
            constructor_name: mistral_mistral_medium,
            display_name: "Mistral Medium 3.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralNemo {
            model_name: "mistral/mistral-nemo",
            constructor_name: mistral_mistral_nemo,
            display_name: "Mistral Nemo",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralSmall {
            model_name: "mistral/mistral-small",
            constructor_name: mistral_mistral_small,
            display_name: "Mistral Small",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMixtral8x22bInstruct {
            model_name: "mistral/mixtral-8x22b-instruct",
            constructor_name: mistral_mixtral_8x22b_instruct,
            display_name: "Mixtral 8x22B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralPixtral12b {
            model_name: "mistral/pixtral-12b",
            constructor_name: mistral_pixtral_12b,
            display_name: "Pixtral 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralPixtralLarge {
            model_name: "mistral/pixtral-large",
            constructor_name: mistral_pixtral_large,
            display_name: "Pixtral Large",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK20905 {
            model_name: "moonshotai/kimi-k2-0905",
            constructor_name: moonshotai_kimi_k2_0905,
            display_name: "Kimi K2 0905",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/kimi-k2-thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2ThinkingTurbo {
            model_name: "moonshotai/kimi-k2-thinking-turbo",
            constructor_name: moonshotai_kimi_k2_thinking_turbo,
            display_name: "Kimi K2 Thinking Turbo",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Turbo {
            model_name: "moonshotai/kimi-k2-turbo",
            constructor_name: moonshotai_kimi_k2_turbo,
            display_name: "Kimi K2 Turbo",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MorphMorphV3Fast {
            model_name: "morph/morph-v3-fast",
            constructor_name: morph_morph_v3_fast,
            display_name: "Morph v3 Fast",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MorphMorphV3Large {
            model_name: "morph/morph-v3-large",
            constructor_name: morph_morph_v3_large,
            display_name: "Morph v3 Large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Nano30bA3b {
            model_name: "nvidia/nemotron-3-nano-30b-a3b",
            constructor_name: nvidia_nemotron_3_nano_30b_a3b,
            display_name: "Nemotron 3 Nano 30B A3B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotronNano12bV2Vl {
            model_name: "nvidia/nemotron-nano-12b-v2-vl",
            constructor_name: nvidia_nemotron_nano_12b_v2_vl,
            display_name: "Nvidia Nemotron Nano 12B V2 VL",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNano9bV2 {
            model_name: "nvidia/nemotron-nano-9b-v2",
            constructor_name: nvidia_nemotron_nano_9b_v2,
            display_name: "Nvidia Nemotron Nano 9B V2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiCodexMini {
            model_name: "openai/codex-mini",
            constructor_name: openai_codex_mini,
            display_name: "Codex Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt35Turbo {
            model_name: "openai/gpt-3.5-turbo",
            constructor_name: openai_gpt_3_5_turbo,
            display_name: "GPT-3.5 Turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt35TurboInstruct {
            model_name: "openai/gpt-3.5-turbo-instruct",
            constructor_name: openai_gpt_3_5_turbo_instruct,
            display_name: "GPT-3.5 Turbo Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt41 {
            model_name: "openai/gpt-4.1",
            constructor_name: openai_gpt_4_1,
            display_name: "GPT-4.1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Mini {
            model_name: "openai/gpt-4.1-mini",
            constructor_name: openai_gpt_4_1_mini,
            display_name: "GPT-4.1 mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Nano {
            model_name: "openai/gpt-4.1-nano",
            constructor_name: openai_gpt_4_1_nano,
            display_name: "GPT-4.1 nano",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4Turbo {
            model_name: "openai/gpt-4-turbo",
            constructor_name: openai_gpt_4_turbo,
            display_name: "GPT-4 Turbo",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4o {
            model_name: "openai/gpt-4o",
            constructor_name: openai_gpt_4o,
            display_name: "GPT-4o",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMini {
            model_name: "openai/gpt-4o-mini",
            constructor_name: openai_gpt_4o_mini,
            display_name: "GPT-4o mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5 {
            model_name: "openai/gpt-5",
            constructor_name: openai_gpt_5,
            display_name: "GPT-5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Codex {
            model_name: "openai/gpt-5.1-codex",
            constructor_name: openai_gpt_5_1_codex,
            display_name: "GPT-5.1-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51CodexMax {
            model_name: "openai/gpt-5.1-codex-max",
            constructor_name: openai_gpt_5_1_codex_max,
            display_name: "GPT 5.1 Codex Max",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51CodexMini {
            model_name: "openai/gpt-5.1-codex-mini",
            constructor_name: openai_gpt_5_1_codex_mini,
            display_name: "GPT-5.1 Codex mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Instant {
            model_name: "openai/gpt-5.1-instant",
            constructor_name: openai_gpt_5_1_instant,
            display_name: "GPT-5.1 Instant",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Thinking {
            model_name: "openai/gpt-5.1-thinking",
            constructor_name: openai_gpt_5_1_thinking,
            display_name: "GPT 5.1 Thinking",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52 {
            model_name: "openai/gpt-5.2",
            constructor_name: openai_gpt_5_2,
            display_name: "GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Chat {
            model_name: "openai/gpt-5.2-chat",
            constructor_name: openai_gpt_5_2_chat,
            display_name: "GPT-5.2 Chat",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Codex {
            model_name: "openai/gpt-5.2-codex",
            constructor_name: openai_gpt_5_2_codex,
            display_name: "GPT-5.2-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Pro {
            model_name: "openai/gpt-5.2-pro",
            constructor_name: openai_gpt_5_2_pro,
            display_name: "GPT 5.2 ",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Chat {
            model_name: "openai/gpt-5-chat",
            constructor_name: openai_gpt_5_chat,
            display_name: "GPT-5 Chat",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Codex {
            model_name: "openai/gpt-5-codex",
            constructor_name: openai_gpt_5_codex,
            display_name: "GPT-5-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Mini {
            model_name: "openai/gpt-5-mini",
            constructor_name: openai_gpt_5_mini,
            display_name: "GPT-5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Nano {
            model_name: "openai/gpt-5-nano",
            constructor_name: openai_gpt_5_nano,
            display_name: "GPT-5 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Pro {
            model_name: "openai/gpt-5-pro",
            constructor_name: openai_gpt_5_pro,
            display_name: "GPT-5 pro",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOssSafeguard20b {
            model_name: "openai/gpt-oss-safeguard-20b",
            constructor_name: openai_gpt_oss_safeguard_20b,
            display_name: "gpt-oss-safeguard-20b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO1 {
            model_name: "openai/o1",
            constructor_name: openai_o1,
            display_name: "o1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3 {
            model_name: "openai/o3",
            constructor_name: openai_o3,
            display_name: "o3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3DeepResearch {
            model_name: "openai/o3-deep-research",
            constructor_name: openai_o3_deep_research,
            display_name: "o3-deep-research",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3Mini {
            model_name: "openai/o3-mini",
            constructor_name: openai_o3_mini,
            display_name: "o3-mini",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3Pro {
            model_name: "openai/o3-pro",
            constructor_name: openai_o3_pro,
            display_name: "o3 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO4Mini {
            model_name: "openai/o4-mini",
            constructor_name: openai_o4_mini,
            display_name: "o4-mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiTextEmbedding3Large {
            model_name: "openai/text-embedding-3-large",
            constructor_name: openai_text_embedding_3_large,
            display_name: "text-embedding-3-large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiTextEmbedding3Small {
            model_name: "openai/text-embedding-3-small",
            constructor_name: openai_text_embedding_3_small,
            display_name: "text-embedding-3-small",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiTextEmbeddingAda002 {
            model_name: "openai/text-embedding-ada-002",
            constructor_name: openai_text_embedding_ada_002,
            display_name: "text-embedding-ada-002",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        PerplexitySonar {
            model_name: "perplexity/sonar",
            constructor_name: perplexity_sonar,
            display_name: "Sonar",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PerplexitySonarPro {
            model_name: "perplexity/sonar-pro",
            constructor_name: perplexity_sonar_pro,
            display_name: "Sonar Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PerplexitySonarReasoning {
            model_name: "perplexity/sonar-reasoning",
            constructor_name: perplexity_sonar_reasoning,
            display_name: "Sonar Reasoning",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        PerplexitySonarReasoningPro {
            model_name: "perplexity/sonar-reasoning-pro",
            constructor_name: perplexity_sonar_reasoning_pro,
            display_name: "Sonar Reasoning Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        PrimeIntellectIntellect3 {
            model_name: "prime-intellect/intellect-3",
            constructor_name: prime_intellect_intellect_3,
            display_name: "INTELLECT 3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RecraftRecraftV2 {
            model_name: "recraft/recraft-v2",
            constructor_name: recraft_recraft_v2,
            display_name: "Recraft V2",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV3 {
            model_name: "recraft/recraft-v3",
            constructor_name: recraft_recraft_v3,
            display_name: "Recraft V3",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        VercelV010Md {
            model_name: "vercel/v0-1.0-md",
            constructor_name: vercel_v0_1_0_md,
            display_name: "v0-1.0-md",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        VercelV015Md {
            model_name: "vercel/v0-1.5-md",
            constructor_name: vercel_v0_1_5_md,
            display_name: "v0-1.5-md",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        VoyageVoyage35 {
            model_name: "voyage/voyage-3.5",
            constructor_name: voyage_voyage_3_5,
            display_name: "voyage-3.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyage35Lite {
            model_name: "voyage/voyage-3.5-lite",
            constructor_name: voyage_voyage_3_5_lite,
            display_name: "voyage-3.5-lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyage3Large {
            model_name: "voyage/voyage-3-large",
            constructor_name: voyage_voyage_3_large,
            display_name: "voyage-3-large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyageCode2 {
            model_name: "voyage/voyage-code-2",
            constructor_name: voyage_voyage_code_2,
            display_name: "voyage-code-2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyageCode3 {
            model_name: "voyage/voyage-code-3",
            constructor_name: voyage_voyage_code_3,
            display_name: "voyage-code-3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyageFinance2 {
            model_name: "voyage/voyage-finance-2",
            constructor_name: voyage_voyage_finance_2,
            display_name: "voyage-finance-2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyageLaw2 {
            model_name: "voyage/voyage-law-2",
            constructor_name: voyage_voyage_law_2,
            display_name: "voyage-law-2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        XaiGrok2Vision {
            model_name: "xai/grok-2-vision",
            constructor_name: xai_grok_2_vision,
            display_name: "Grok 2 Vision",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok3 {
            model_name: "xai/grok-3",
            constructor_name: xai_grok_3,
            display_name: "Grok 3",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok3Fast {
            model_name: "xai/grok-3-fast",
            constructor_name: xai_grok_3_fast,
            display_name: "Grok 3 Fast",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok3Mini {
            model_name: "xai/grok-3-mini",
            constructor_name: xai_grok_3_mini,
            display_name: "Grok 3 Mini",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok3MiniFast {
            model_name: "xai/grok-3-mini-fast",
            constructor_name: xai_grok_3_mini_fast,
            display_name: "Grok 3 Mini Fast",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok4 {
            model_name: "xai/grok-4",
            constructor_name: xai_grok_4,
            display_name: "Grok 4",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok41FastNonReasoning {
            model_name: "xai/grok-4.1-fast-non-reasoning",
            constructor_name: xai_grok_4_1_fast_non_reasoning,
            display_name: "Grok 4.1 Fast Non-Reasoning",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok41FastReasoning {
            model_name: "xai/grok-4.1-fast-reasoning",
            constructor_name: xai_grok_4_1_fast_reasoning,
            display_name: "Grok 4.1 Fast Reasoning",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok4FastNonReasoning {
            model_name: "xai/grok-4-fast-non-reasoning",
            constructor_name: xai_grok_4_fast_non_reasoning,
            display_name: "Grok 4 Fast (Non-Reasoning)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok4FastReasoning {
            model_name: "xai/grok-4-fast-reasoning",
            constructor_name: xai_grok_4_fast_reasoning,
            display_name: "Grok 4 Fast Reasoning",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrokCodeFast1 {
            model_name: "xai/grok-code-fast-1",
            constructor_name: xai_grok_code_fast_1,
            display_name: "Grok Code Fast 1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV2Flash {
            model_name: "xiaomi/mimo-v2-flash",
            constructor_name: xiaomi_mimo_v2_flash,
            display_name: "MiMo V2 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm45 {
            model_name: "zai/glm-4.5",
            constructor_name: zai_glm_4_5,
            display_name: "GLM 4.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm45Air {
            model_name: "zai/glm-4.5-air",
            constructor_name: zai_glm_4_5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm45v {
            model_name: "zai/glm-4.5v",
            constructor_name: zai_glm_4_5v,
            display_name: "GLM 4.5V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm46 {
            model_name: "zai/glm-4.6",
            constructor_name: zai_glm_4_6,
            display_name: "GLM 4.6",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm46v {
            model_name: "zai/glm-4.6v",
            constructor_name: zai_glm_4_6v,
            display_name: "GLM-4.6V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm46vFlash {
            model_name: "zai/glm-4.6v-flash",
            constructor_name: zai_glm_4_6v_flash,
            display_name: "GLM-4.6V-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm47 {
            model_name: "zai/glm-4.7",
            constructor_name: zai_glm_4_7,
            display_name: "GLM 4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
