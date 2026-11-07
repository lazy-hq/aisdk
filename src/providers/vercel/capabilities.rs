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
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        AlibabaQwen3235bA22bThinking {
            model_name: "alibaba/qwen3-235b-a22b-thinking",
            constructor_name: alibaba_qwen3_235b_a22b_thinking,
            display_name: "Qwen3 235B A22B Thinking 2507",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Coder {
            model_name: "alibaba/qwen3-coder",
            constructor_name: alibaba_qwen3_coder,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Coder30bA3b {
            model_name: "alibaba/qwen3-coder-30b-a3b",
            constructor_name: alibaba_qwen3_coder_30b_a3b,
            display_name: "Qwen 3 Coder 30B A3B Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3CoderNext {
            model_name: "alibaba/qwen3-coder-next",
            constructor_name: alibaba_qwen3_coder_next,
            display_name: "Qwen3 Coder Next",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        AlibabaQwen3MaxThinking {
            model_name: "alibaba/qwen3-max-thinking",
            constructor_name: alibaba_qwen3_max_thinking,
            display_name: "Qwen 3 Max Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        AlibabaQwen35Flash {
            model_name: "alibaba/qwen3.5-flash",
            constructor_name: alibaba_qwen3_5_flash,
            display_name: "Qwen 3.5 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen35Plus {
            model_name: "alibaba/qwen3.5-plus",
            constructor_name: alibaba_qwen3_5_plus,
            display_name: "Qwen 3.5 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3627b {
            model_name: "alibaba/qwen3.6-27b",
            constructor_name: alibaba_qwen3_6_27b,
            display_name: "Qwen 3.6 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen36Plus {
            model_name: "alibaba/qwen3.6-plus",
            constructor_name: alibaba_qwen3_6_plus,
            display_name: "Qwen 3.6 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen37Flash {
            model_name: "alibaba/qwen3.7-flash",
            constructor_name: alibaba_qwen3_7_flash,
            display_name: "Qwen 3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen37Max {
            model_name: "alibaba/qwen3.7-max",
            constructor_name: alibaba_qwen3_7_max,
            display_name: "Qwen 3.7 Max",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen37Plus {
            model_name: "alibaba/qwen3.7-plus",
            constructor_name: alibaba_qwen3_7_plus,
            display_name: "Qwen 3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3824tA95b {
            model_name: "alibaba/qwen3.8-2.4t-a95b",
            constructor_name: alibaba_qwen3_8_2_4t_a95b,
            display_name: "Qwen3.8 2.4T A95B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3827b {
            model_name: "alibaba/qwen3.8-27b",
            constructor_name: alibaba_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen38Flash {
            model_name: "alibaba/qwen3.8-flash",
            constructor_name: alibaba_qwen3_8_flash,
            display_name: "Qwen 3.8 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen38Max {
            model_name: "alibaba/qwen3.8-max",
            constructor_name: alibaba_qwen3_8_max,
            display_name: "Qwen 3.8 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen38Max0902 {
            model_name: "alibaba/qwen3.8-max-0902",
            constructor_name: alibaba_qwen3_8_max_0902,
            display_name: "Qwen3.8 Max 0902",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen38MaxPrime {
            model_name: "alibaba/qwen3.8-max-prime",
            constructor_name: alibaba_qwen3_8_max_prime,
            display_name: "Qwen 3.8 Max Prime",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen38OmniFlash {
            model_name: "alibaba/qwen3.8-omni-flash",
            constructor_name: alibaba_qwen3_8_omni_flash,
            display_name: "Qwen 3.8 Omni Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaWanV25T2vPreview {
            model_name: "alibaba/wan-v2.5-t2v-preview",
            constructor_name: alibaba_wan_v2_5_t2v_preview,
            display_name: "Wan v2.5 Text-to-Video Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV26I2v {
            model_name: "alibaba/wan-v2.6-i2v",
            constructor_name: alibaba_wan_v2_6_i2v,
            display_name: "Wan v2.6 Image-to-Video",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV26I2vFlash {
            model_name: "alibaba/wan-v2.6-i2v-flash",
            constructor_name: alibaba_wan_v2_6_i2v_flash,
            display_name: "Wan v2.6 Image-to-Video Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV26R2v {
            model_name: "alibaba/wan-v2.6-r2v",
            constructor_name: alibaba_wan_v2_6_r2v,
            display_name: "Wan v2.6 Reference-to-Video",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        AlibabaWanV26R2vFlash {
            model_name: "alibaba/wan-v2.6-r2v-flash",
            constructor_name: alibaba_wan_v2_6_r2v_flash,
            display_name: "Wan v2.6 Reference-to-Video Flash",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        AlibabaWanV26T2v {
            model_name: "alibaba/wan-v2.6-t2v",
            constructor_name: alibaba_wan_v2_6_t2v,
            display_name: "Wan v2.6 Text-to-Video",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV27R2v {
            model_name: "alibaba/wan-v2.7-r2v",
            constructor_name: alibaba_wan_v2_7_r2v,
            display_name: "Wan v2.7 Reference-to-Video",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        AlibabaWanV27T2v {
            model_name: "alibaba/wan-v2.7-t2v",
            constructor_name: alibaba_wan_v2_7_t2v,
            display_name: "Wan v2.7 Text-to-Video",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV30Video {
            model_name: "alibaba/wan-v3.0-video",
            constructor_name: alibaba_wan_v3_0_video,
            display_name: "Wan v3.0 Video",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        AlibabaWanV30VideoPrime {
            model_name: "alibaba/wan-v3.0-video-prime",
            constructor_name: alibaba_wan_v3_0_video_prime,
            display_name: "Wan v3.0 Video Prime",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
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
        AnthropicClaudeFable5 {
            model_name: "anthropic/claude-fable-5",
            constructor_name: anthropic_claude_fable_5,
            display_name: "Claude Fable 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeFable51 {
            model_name: "anthropic/claude-fable-5.1",
            constructor_name: anthropic_claude_fable_5_1,
            display_name: "Claude Fable 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku45 {
            model_name: "anthropic/claude-haiku-4.5",
            constructor_name: anthropic_claude_haiku_4_5,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku55 {
            model_name: "anthropic/claude-haiku-5.5",
            constructor_name: anthropic_claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        AnthropicClaudeOpus46 {
            model_name: "anthropic/claude-opus-4.6",
            constructor_name: anthropic_claude_opus_4_6,
            display_name: "Claude Opus 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus47 {
            model_name: "anthropic/claude-opus-4.7",
            constructor_name: anthropic_claude_opus_4_7,
            display_name: "Claude Opus 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus48 {
            model_name: "anthropic/claude-opus-4.8",
            constructor_name: anthropic_claude_opus_4_8,
            display_name: "Claude Opus 4.8",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus48Fast {
            model_name: "anthropic/claude-opus-4.8-fast",
            constructor_name: anthropic_claude_opus_4_8_fast,
            display_name: "Claude Opus 4.8 (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus5 {
            model_name: "anthropic/claude-opus-5",
            constructor_name: anthropic_claude_opus_5,
            display_name: "Claude Opus 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus5Fast {
            model_name: "anthropic/claude-opus-5-fast",
            constructor_name: anthropic_claude_opus_5_fast,
            display_name: "Claude Opus 5 (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus55 {
            model_name: "anthropic/claude-opus-5.5",
            constructor_name: anthropic_claude_opus_5_5,
            display_name: "Claude Opus 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus55Fast {
            model_name: "anthropic/claude-opus-5.5-fast",
            constructor_name: anthropic_claude_opus_5_5_fast,
            display_name: "Claude Opus 5.5 (Fast)",
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet46 {
            model_name: "anthropic/claude-sonnet-4.6",
            constructor_name: anthropic_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet5 {
            model_name: "anthropic/claude-sonnet-5",
            constructor_name: anthropic_claude_sonnet_5,
            display_name: "Claude Sonnet 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet55 {
            model_name: "anthropic/claude-sonnet-5.5",
            constructor_name: anthropic_claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ArceeAiTrinityMini {
            model_name: "arcee-ai/trinity-mini",
            constructor_name: arcee_ai_trinity_mini,
            display_name: "Trinity Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BflFlux2Flex {
            model_name: "bfl/flux-2-flex",
            constructor_name: bfl_flux_2_flex,
            display_name: "FLUX.2 [flex]",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BflFlux2Klein4b {
            model_name: "bfl/flux-2-klein-4b",
            constructor_name: bfl_flux_2_klein_4b,
            display_name: "FLUX.2 [klein] 4B",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BflFlux2Klein9b {
            model_name: "bfl/flux-2-klein-9b",
            constructor_name: bfl_flux_2_klein_9b,
            display_name: "FLUX.2 [klein] 9B",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BflFlux2Max {
            model_name: "bfl/flux-2-max",
            constructor_name: bfl_flux_2_max,
            display_name: "FLUX.2 [max]",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BflFlux2Pro {
            model_name: "bfl/flux-2-pro",
            constructor_name: bfl_flux_2_pro,
            display_name: "FLUX.2 [pro]",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BflFlux3Image {
            model_name: "bfl/flux-3-image",
            constructor_name: bfl_flux_3_image,
            display_name: "FLUX 3 Image",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BflFlux3ImageKlein {
            model_name: "bfl/flux-3-image-klein",
            constructor_name: bfl_flux_3_image_klein,
            display_name: "FLUX 3 Image Klein",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BflFlux3Video {
            model_name: "bfl/flux-3-video",
            constructor_name: bfl_flux_3_video,
            display_name: "Flux 3",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        BflFluxKontextMax {
            model_name: "bfl/flux-kontext-max",
            constructor_name: bfl_flux_kontext_max,
            display_name: "FLUX.1 Kontext Max",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BflFluxKontextPro {
            model_name: "bfl/flux-kontext-pro",
            constructor_name: bfl_flux_kontext_pro,
            display_name: "FLUX.1 Kontext Pro",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BflFluxPro10Fill {
            model_name: "bfl/flux-pro-1.0-fill",
            constructor_name: bfl_flux_pro_1_0_fill,
            display_name: "FLUX.1 Fill \\[pro\\]",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BytedanceSeed18 {
            model_name: "bytedance/seed-1.8",
            constructor_name: bytedance_seed_1_8,
            display_name: "Seed 1.8",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BytedanceSeed21Turbo {
            model_name: "bytedance/seed-2.1-turbo",
            constructor_name: bytedance_seed_2_1_turbo,
            display_name: "Seed 2.1 Turbo",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BytedanceSeedance20 {
            model_name: "bytedance/seedance-2.0",
            constructor_name: bytedance_seedance_2_0,
            display_name: "Seedance 2.0",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        BytedanceSeedance20Fast {
            model_name: "bytedance/seedance-2.0-fast",
            constructor_name: bytedance_seedance_2_0_fast,
            display_name: "Seedance 2.0 Fast",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        BytedanceSeedance20Mini {
            model_name: "bytedance/seedance-2.0-mini",
            constructor_name: bytedance_seedance_2_0_mini,
            display_name: "Seedance 2.0 Mini",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        BytedanceSeedance25 {
            model_name: "bytedance/seedance-2.5",
            constructor_name: bytedance_seedance_2_5,
            display_name: "Seedance 2.5",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        BytedanceSeedanceV10Pro {
            model_name: "bytedance/seedance-v1.0-pro",
            constructor_name: bytedance_seedance_v1_0_pro,
            display_name: "Seedance v1.0 Pro",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        BytedanceSeedanceV10ProFast {
            model_name: "bytedance/seedance-v1.0-pro-fast",
            constructor_name: bytedance_seedance_v1_0_pro_fast,
            display_name: "Seedance v1.0 Pro Fast",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        BytedanceSeedanceV15Pro {
            model_name: "bytedance/seedance-v1.5-pro",
            constructor_name: bytedance_seedance_v1_5_pro,
            display_name: "Seedance v1.5 Pro",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        BytedanceSeedream40 {
            model_name: "bytedance/seedream-4.0",
            constructor_name: bytedance_seedream_4_0,
            display_name: "Seedream 4.0",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BytedanceSeedream45 {
            model_name: "bytedance/seedream-4.5",
            constructor_name: bytedance_seedream_4_5,
            display_name: "Seedream 4.5",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BytedanceSeedream50Lite {
            model_name: "bytedance/seedream-5.0-lite",
            constructor_name: bytedance_seedream_5_0_lite,
            display_name: "Seedream 5.0 Lite",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BytedanceSeedream50Pro {
            model_name: "bytedance/seedream-5.0-pro",
            constructor_name: bytedance_seedream_5_0_pro,
            display_name: "Seedream 5.0 Pro",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        CallstackApex {
            model_name: "callstack/apex",
            constructor_name: callstack_apex,
            display_name: "Apex",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereCommandA {
            model_name: "cohere/command-a",
            constructor_name: cohere_command_a,
            display_name: "Command A",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereEmbedV40 {
            model_name: "cohere/embed-v4.0",
            constructor_name: cohere_embed_v4_0,
            display_name: "Embed v4.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereEmbedV50Fast {
            model_name: "cohere/embed-v5.0-fast",
            constructor_name: cohere_embed_v5_0_fast,
            display_name: "Embed 5 Fast",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereEmbedV50Pro {
            model_name: "cohere/embed-v5.0-pro",
            constructor_name: cohere_embed_v5_0_pro,
            display_name: "Embed 5 Pro",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereRerankV35 {
            model_name: "cohere/rerank-v3.5",
            constructor_name: cohere_rerank_v3_5,
            display_name: "Cohere Rerank 3.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereRerankV4Fast {
            model_name: "cohere/rerank-v4-fast",
            constructor_name: cohere_rerank_v4_fast,
            display_name: "Cohere Rerank 4 Fast",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereRerankV4Pro {
            model_name: "cohere/rerank-v4-pro",
            constructor_name: cohere_rerank_v4_pro,
            display_name: "Cohere Rerank 4 Pro",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereRerankV5Fast {
            model_name: "cohere/rerank-v5-fast",
            constructor_name: cohere_rerank_v5_fast,
            display_name: "Cohere Rerank 5 Fast",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereRerankV5Pro {
            model_name: "cohere/rerank-v5-pro",
            constructor_name: cohere_rerank_v5_pro,
            display_name: "Cohere Rerank 5 Pro",
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
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
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
        DeepseekDeepseekV4Flash {
            model_name: "deepseek/deepseek-v4-flash",
            constructor_name: deepseek_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash0731 {
            model_name: "deepseek/deepseek-v4-flash-0731",
            constructor_name: deepseek_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4FlashVisionExp {
            model_name: "deepseek/deepseek-v4-flash-vision-exp",
            constructor_name: deepseek_deepseek_v4_flash_vision_exp,
            display_name: "DeepSeek V4 Flash Vision Exp",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Pro {
            model_name: "deepseek/deepseek-v4-pro",
            constructor_name: deepseek_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Pro0813 {
            model_name: "deepseek/deepseek-v4-pro-0813",
            constructor_name: deepseek_deepseek_v4_pro_0813,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash {
            model_name: "deepseek/deepseek-v4.1-flash",
            constructor_name: deepseek_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash1017 {
            model_name: "deepseek/deepseek-v4.1-flash-1017",
            constructor_name: deepseek_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash1017Fast {
            model_name: "deepseek/deepseek-v4.1-flash-1017-fast",
            constructor_name: deepseek_deepseek_v4_1_flash_1017_fast,
            display_name: "DeepSeek V4.1 Flash 1017 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash1017Ultrafast {
            model_name: "deepseek/deepseek-v4.1-flash-1017-ultrafast",
            constructor_name: deepseek_deepseek_v4_1_flash_1017_ultrafast,
            display_name: "DeepSeek V4.1 Flash 1017 Ultrafast",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41FlashFast {
            model_name: "deepseek/deepseek-v4.1-flash-fast",
            constructor_name: deepseek_deepseek_v4_1_flash_fast,
            display_name: "DeepSeek V4.1 Flash Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ElorianGlimpse {
            model_name: "elorian/glimpse",
            constructor_name: elorian_glimpse,
            display_name: "Glimpse",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        FireworksEmber1 {
            model_name: "fireworks/ember-1",
            constructor_name: fireworks_ember_1,
            display_name: "Ember-1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        FishAudioDrama3 {
            model_name: "fish-audio/drama-3",
            constructor_name: fish_audio_drama_3,
            display_name: "Drama 3",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        FishAudioS1 {
            model_name: "fish-audio/s1",
            constructor_name: fish_audio_s1,
            display_name: "S1",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        FishAudioS2Pro {
            model_name: "fish-audio/s2-pro",
            constructor_name: fish_audio_s2_pro,
            display_name: "S2 Pro",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        FishAudioS21Pro {
            model_name: "fish-audio/s2.1-pro",
            constructor_name: fish_audio_s2_1_pro,
            display_name: "S2.1 Pro",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        FishAudioTranscribe1 {
            model_name: "fish-audio/transcribe-1",
            constructor_name: fish_audio_transcribe_1,
            display_name: "Transcribe-1",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
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
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
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
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
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
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini3ProPreview {
            model_name: "google/gemini-3-pro-preview",
            constructor_name: google_gemini_3_pro_preview,
            display_name: "Gemini 3 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31FlashImage {
            model_name: "google/gemini-3.1-flash-image",
            constructor_name: google_gemini_3_1_flash_image,
            display_name: "Nano Banana 2",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini31FlashLite {
            model_name: "google/gemini-3.1-flash-lite",
            constructor_name: google_gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini31FlashLiteImage {
            model_name: "google/gemini-3.1-flash-lite-image",
            constructor_name: google_gemini_3_1_flash_lite_image,
            display_name: "Gemini 3.1 Flash Lite Image (Nano Banana 2 Lite)",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini31ProPreview {
            model_name: "google/gemini-3.1-pro-preview",
            constructor_name: google_gemini_3_1_pro_preview,
            display_name: "Gemini 3.1 Pro Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini35Flash {
            model_name: "google/gemini-3.5-flash",
            constructor_name: google_gemini_3_5_flash,
            display_name: "Gemini 3.5 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini35FlashLite {
            model_name: "google/gemini-3.5-flash-lite",
            constructor_name: google_gemini_3_5_flash_lite,
            display_name: "Gemini 3.5 Flash Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini35Transcribe {
            model_name: "google/gemini-3.5-transcribe",
            constructor_name: google_gemini_3_5_transcribe,
            display_name: "Gemini 3.5 Transcribe",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        GoogleGemini35TranscribeLive {
            model_name: "google/gemini-3.5-transcribe-live",
            constructor_name: google_gemini_3_5_transcribe_live,
            display_name: "Gemini 3.5 Transcribe Live",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        GoogleGemini36Flash {
            model_name: "google/gemini-3.6-flash",
            constructor_name: google_gemini_3_6_flash,
            display_name: "Gemini 3.6 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini37Flash {
            model_name: "google/gemini-3.7-flash",
            constructor_name: google_gemini_3_7_flash,
            display_name: "Gemini 3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini38Flash {
            model_name: "google/gemini-3.8-flash",
            constructor_name: google_gemini_3_8_flash,
            display_name: "Gemini 3.8 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini38FlashLiteTts {
            model_name: "google/gemini-3.8-flash-lite-tts",
            constructor_name: google_gemini_3_8_flash_lite_tts,
            display_name: "Gemini 3.8 Flash-Lite TTS",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        GoogleGemini38FlashTts {
            model_name: "google/gemini-3.8-flash-tts",
            constructor_name: google_gemini_3_8_flash_tts,
            display_name: "Gemini 3.8 Flash TTS",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        GoogleGemini38Live {
            model_name: "google/gemini-3.8-live",
            constructor_name: google_gemini_3_8_live,
            display_name: "Gemini 3.8 Live",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini38LiveExtendedThinking {
            model_name: "google/gemini-3.8-live-extended-thinking",
            constructor_name: google_gemini_3_8_live_extended_thinking,
            display_name: "Gemini 3.8 Live Extended Thinking",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini4Argon {
            model_name: "google/gemini-4-argon",
            constructor_name: google_gemini_4_argon,
            display_name: "Gemini 4 Argon",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGeminiEmbedding001 {
            model_name: "google/gemini-embedding-001",
            constructor_name: google_gemini_embedding_001,
            display_name: "Gemini Embedding 001",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleGeminiEmbedding2 {
            model_name: "google/gemini-embedding-2",
            constructor_name: google_gemini_embedding_2,
            display_name: "Gemini Embedding 2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleGeminiNanoBanana21 {
            model_name: "google/gemini-nano-banana-2.1",
            constructor_name: google_gemini_nano_banana_2_1,
            display_name: "Gemini Nano Banana 2.1",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGeminiOmni12Flash {
            model_name: "google/gemini-omni-1.2-flash",
            constructor_name: google_gemini_omni_1_2_flash,
            display_name: "Gemini Omni 1.2 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, VideoOutputSupport]
        },
        GoogleGeminiOmniFlashPreview {
            model_name: "google/gemini-omni-flash-preview",
            constructor_name: google_gemini_omni_flash_preview,
            display_name: "Gemini Omni Flash Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, VideoOutputSupport]
        },
        GoogleGemma426bA4bIt {
            model_name: "google/gemma-4-26b-a4b-it",
            constructor_name: google_gemma_4_26b_a4b_it,
            display_name: "Gemma 4 26B A4B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31b-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        GoogleVeo31FastGenerate001 {
            model_name: "google/veo-3.1-fast-generate-001",
            constructor_name: google_veo_3_1_fast_generate_001,
            display_name: "Veo 3.1 Fast Generate",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        GoogleVeo31Generate001 {
            model_name: "google/veo-3.1-generate-001",
            constructor_name: google_veo_3_1_generate_001,
            display_name: "Veo 3.1",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        GoogleVeo31LiteGenerate001 {
            model_name: "google/veo-3.1-lite-generate-001",
            constructor_name: google_veo_3_1_lite_generate_001,
            display_name: "Veo 3.1 Lite Generate",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        HeygenHeygenVideo1 {
            model_name: "heygen/heygen-video-1",
            constructor_name: heygen_heygen_video_1,
            display_name: "HeyGen Video",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        HumansandAmpersandV01 {
            model_name: "humansand/ampersand-v0.1",
            constructor_name: humansand_ampersand_v0_1,
            display_name: "Ampersand v0.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InceptionMercury2 {
            model_name: "inception/mercury-2",
            constructor_name: inception_mercury_2,
            display_name: "Mercury 2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InceptionMercury25 {
            model_name: "inception/mercury-2.5",
            constructor_name: inception_mercury_2_5,
            display_name: "Mercury 2.5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InceptionMercury3 {
            model_name: "inception/mercury-3",
            constructor_name: inception_mercury_3,
            display_name: "Mercury 3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InceptionMercuryCoderSmall {
            model_name: "inception/mercury-coder-small",
            constructor_name: inception_mercury_coder_small,
            display_name: "Mercury Coder Small Beta",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30Flash {
            model_name: "inclusionai/ling-3.0-flash",
            constructor_name: inclusionai_ling_3_0_flash,
            display_name: "Ling 3.0 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30FlashFin {
            model_name: "inclusionai/ling-3.0-flash-fin",
            constructor_name: inclusionai_ling_3_0_flash_fin,
            display_name: "Ling 3.0 Flash Fin",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30FlashSante {
            model_name: "inclusionai/ling-3.0-flash-sante",
            constructor_name: inclusionai_ling_3_0_flash_sante,
            display_name: "Ling 3.0 Flash Sante",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30FlashVl {
            model_name: "inclusionai/ling-3.0-flash-vl",
            constructor_name: inclusionai_ling_3_0_flash_vl,
            display_name: "Ling 3.0 Flash VL",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing31Flash {
            model_name: "inclusionai/ling-3.1-flash",
            constructor_name: inclusionai_ling_3_1_flash,
            display_name: "Ling 3.1 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing31Flash1019 {
            model_name: "inclusionai/ling-3.1-flash-1019",
            constructor_name: inclusionai_ling_3_1_flash_1019,
            display_name: "Ling 3.1 Flash 1019",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InferenceNetSchematronV2Small {
            model_name: "inference-net/schematron-v2-small",
            constructor_name: inference_net_schematron_v2_small,
            display_name: "Schematron V2 Small",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        InferenceNetSchematronV2Turbo {
            model_name: "inference-net/schematron-v2-turbo",
            constructor_name: inference_net_schematron_v2_turbo,
            display_name: "Schematron V2 Turbo",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        InterfazeInterfazeBeta {
            model_name: "interfaze/interfaze-beta",
            constructor_name: interfaze_interfaze_beta,
            display_name: "Interfaze Beta",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        KlingaiKlingV25TurboI2v {
            model_name: "klingai/kling-v2.5-turbo-i2v",
            constructor_name: klingai_kling_v2_5_turbo_i2v,
            display_name: "Kling v2.5 Turbo Image-to-Video",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV25TurboT2v {
            model_name: "klingai/kling-v2.5-turbo-t2v",
            constructor_name: klingai_kling_v2_5_turbo_t2v,
            display_name: "Kling v2.5 Turbo Text-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV26I2v {
            model_name: "klingai/kling-v2.6-i2v",
            constructor_name: klingai_kling_v2_6_i2v,
            display_name: "Kling v2.6 Image-to-Video",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV26MotionControl {
            model_name: "klingai/kling-v2.6-motion-control",
            constructor_name: klingai_kling_v2_6_motion_control,
            display_name: "Kling v2.6 Motion Control",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV26T2v {
            model_name: "klingai/kling-v2.6-t2v",
            constructor_name: klingai_kling_v2_6_t2v,
            display_name: "Kling v2.6 Text-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV30I2v {
            model_name: "klingai/kling-v3.0-i2v",
            constructor_name: klingai_kling_v3_0_i2v,
            display_name: "Kling v3.0 Image-to-Video",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV30MotionControl {
            model_name: "klingai/kling-v3.0-motion-control",
            constructor_name: klingai_kling_v3_0_motion_control,
            display_name: "Kling v3.0 Motion Control",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV30T2v {
            model_name: "klingai/kling-v3.0-t2v",
            constructor_name: klingai_kling_v3_0_t2v,
            display_name: "Kling v3.0 Text-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        KwaipilotKatCoderProV1 {
            model_name: "kwaipilot/kat-coder-pro-v1",
            constructor_name: kwaipilot_kat_coder_pro_v1,
            display_name: "KAT-Coder-Pro V1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        LiquidLfm2524bA2b {
            model_name: "liquid/lfm2.5-24b-a2b",
            constructor_name: liquid_lfm2_5_24b_a2b,
            display_name: "LFM2.5-24B-A2B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MeituanLongcat25 {
            model_name: "meituan/longcat-2.5",
            constructor_name: meituan_longcat_2_5,
            display_name: "LongCat 2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        MetaMuseGlimmer30b {
            model_name: "meta/muse-glimmer-30b",
            constructor_name: meta_muse_glimmer_30b,
            display_name: "Muse Glimmer 30B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseImage10 {
            model_name: "meta/muse-image-1.0",
            constructor_name: meta_muse_image_1_0,
            display_name: "Muse Image 1.0",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        MetaMuseSpark11 {
            model_name: "meta/muse-spark-1.1",
            constructor_name: meta_muse_spark_1_1,
            display_name: "Muse Spark 1.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseSpark12 {
            model_name: "meta/muse-spark-1.2",
            constructor_name: meta_muse_spark_1_2,
            display_name: "Muse Spark 1.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseSpark12Contributor {
            model_name: "meta/muse-spark-1.2-contributor",
            constructor_name: meta_muse_spark_1_2_contributor,
            display_name: "Muse Spark 1.2 Contributor",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseSpark13 {
            model_name: "meta/muse-spark-1.3",
            constructor_name: meta_muse_spark_1_3,
            display_name: "Muse Spark 1.3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseSpark13Contributor {
            model_name: "meta/muse-spark-1.3-contributor",
            constructor_name: meta_muse_spark_1_3_contributor,
            display_name: "Muse Spark 1.3 Contributor",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseSparkOpen {
            model_name: "meta/muse-spark-open",
            constructor_name: meta_muse_spark_open,
            display_name: "Muse Spark Open",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseWonder10 {
            model_name: "meta/muse-wonder-1.0",
            constructor_name: meta_muse_wonder_1_0,
            display_name: "Muse Wonder",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftMaiImage26 {
            model_name: "microsoft/mai-image-2.6",
            constructor_name: microsoft_mai_image_2_6,
            display_name: "MAI-Image-2.6",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        MicrosoftMaiImage26Flash {
            model_name: "microsoft/mai-image-2.6-flash",
            constructor_name: microsoft_mai_image_2_6_flash,
            display_name: "MAI-Image-2.6-Flash",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        MicrosoftMaiTranscribe2 {
            model_name: "microsoft/mai-transcribe-2",
            constructor_name: microsoft_mai_transcribe_2,
            display_name: "MAI-Transcribe 2",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        MicrosoftMaiTranscribe2Streaming {
            model_name: "microsoft/mai-transcribe-2-streaming",
            constructor_name: microsoft_mai_transcribe_2_streaming,
            display_name: "MAI-Transcribe-2-Streaming",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        MicrosoftMaiVoice2 {
            model_name: "microsoft/mai-voice-2",
            constructor_name: microsoft_mai_voice_2,
            display_name: "MAI-Voice-2",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        MicrosoftMaiVoice2Flash {
            model_name: "microsoft/mai-voice-2-flash",
            constructor_name: microsoft_mai_voice_2_flash,
            display_name: "MAI-Voice-2-Flash",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        MicrosoftMaiVoice21 {
            model_name: "microsoft/mai-voice-2.1",
            constructor_name: microsoft_mai_voice_2_1,
            display_name: "MAI-Voice-2.1",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        MicrosoftMaiVoice21Flash {
            model_name: "microsoft/mai-voice-2.1-flash",
            constructor_name: microsoft_mai_voice_2_1_flash,
            display_name: "MAI-Voice-2.1-Flash",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        MinimaxMinimaxH3 {
            model_name: "minimax/minimax-h3",
            constructor_name: minimax_minimax_h3,
            display_name: "MiniMax H3",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        MinimaxMinimaxH3Max {
            model_name: "minimax/minimax-h3-max",
            constructor_name: minimax_minimax_h3_max,
            display_name: "MiniMax H3 Max",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
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
        MinimaxMinimaxM25 {
            model_name: "minimax/minimax-m2.5",
            constructor_name: minimax_minimax_m2_5,
            display_name: "MiniMax M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM25Highspeed {
            model_name: "minimax/minimax-m2.5-highspeed",
            constructor_name: minimax_minimax_m2_5_highspeed,
            display_name: "MiniMax M2.5 High Speed",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM27 {
            model_name: "minimax/minimax-m2.7",
            constructor_name: minimax_minimax_m2_7,
            display_name: "Minimax M2.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM27Highspeed {
            model_name: "minimax/minimax-m2.7-highspeed",
            constructor_name: minimax_minimax_m2_7_highspeed,
            display_name: "MiniMax M2.7 High Speed",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM3 {
            model_name: "minimax/minimax-m3",
            constructor_name: minimax_minimax_m3,
            display_name: "MiniMax M3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM31Flash {
            model_name: "minimax/minimax-m3.1-flash",
            constructor_name: minimax_minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMinistral8b {
            model_name: "mistral/ministral-8b",
            constructor_name: mistral_ministral_8b,
            display_name: "Ministral 8B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        MistralMistralLarge4 {
            model_name: "mistral/mistral-large-4",
            constructor_name: mistral_mistral_large_4,
            display_name: "Mistral Large 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralMedium {
            model_name: "mistral/mistral-medium",
            constructor_name: mistral_mistral_medium,
            display_name: "Mistral Medium 3.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralMedium35 {
            model_name: "mistral/mistral-medium-3.5",
            constructor_name: mistral_mistral_medium_3_5,
            display_name: "Mistral Medium Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        MistralVoxtralMiniTranscribe3 {
            model_name: "mistral/voxtral-mini-transcribe-3",
            constructor_name: mistral_voxtral_mini_transcribe_3,
            display_name: "Voxtral Mini Transcribe 3",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        MistralVoxtralMiniTts3 {
            model_name: "mistral/voxtral-mini-tts-3",
            constructor_name: mistral_voxtral_mini_tts_3,
            display_name: "Voxtral TTS 3",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        MistralVoxtralSmallTranscribe3 {
            model_name: "mistral/voxtral-small-transcribe-3",
            constructor_name: mistral_voxtral_small_transcribe_3,
            display_name: "Voxtral Small Transcribe 3",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        MixedbreadToast1 {
            model_name: "mixedbread/toast-1",
            constructor_name: mixedbread_toast_1,
            display_name: "Toast 1",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2 {
            model_name: "moonshotai/kimi-k2",
            constructor_name: moonshotai_kimi_k2,
            display_name: "Kimi K2 Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        MoonshotaiKimiK25 {
            model_name: "moonshotai/kimi-k2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26 {
            model_name: "moonshotai/kimi-k2.6",
            constructor_name: moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK27Code {
            model_name: "moonshotai/kimi-k2.7-code",
            constructor_name: moonshotai_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK27CodeHighspeed {
            model_name: "moonshotai/kimi-k2.7-code-highspeed",
            constructor_name: moonshotai_kimi_k2_7_code_highspeed,
            display_name: "Kimi K2.7 Code High Speed",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK3 {
            model_name: "moonshotai/kimi-k3",
            constructor_name: moonshotai_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK3Fast {
            model_name: "moonshotai/kimi-k3-fast",
            constructor_name: moonshotai_kimi_k3_fast,
            display_name: "Kimi K3 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK31Pro {
            model_name: "moonshotai/kimi-k3.1-pro",
            constructor_name: moonshotai_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK31ProHighspeed {
            model_name: "moonshotai/kimi-k3.1-pro-highspeed",
            constructor_name: moonshotai_kimi_k3_1_pro_highspeed,
            display_name: "Kimi K3.1 Pro Highspeed",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        NvidiaNemotron3Super120bA12b {
            model_name: "nvidia/nemotron-3-super-120b-a12b",
            constructor_name: nvidia_nemotron_3_super_120b_a12b,
            display_name: "NVIDIA Nemotron 3 Super 120B A12B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Ultra550bA55b {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b,
            display_name: "Nemotron 3 Ultra",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35Lightning {
            model_name: "nvidia/nemotron-3.5-lightning",
            constructor_name: nvidia_nemotron_3_5_lightning,
            display_name: "Nemotron 3.5 Lightning 30B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35LightningFree {
            model_name: "nvidia/nemotron-3.5-lightning-free",
            constructor_name: nvidia_nemotron_3_5_lightning_free,
            display_name: "Nemotron 3.5 Lightning Free",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35SuperVl {
            model_name: "nvidia/nemotron-3.5-super-vl",
            constructor_name: nvidia_nemotron_3_5_super_vl,
            display_name: "NVIDIA Nemotron 3.5 Super VL",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        OpenaiGpt4Turbo {
            model_name: "openai/gpt-4-turbo",
            constructor_name: openai_gpt_4_turbo,
            display_name: "GPT-4 Turbo",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41 {
            model_name: "openai/gpt-4.1",
            constructor_name: openai_gpt_4_1,
            display_name: "GPT-4.1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Fast {
            model_name: "openai/gpt-4.1-fast",
            constructor_name: openai_gpt_4_1_fast,
            display_name: "GPT-4.1 (Fast)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Mini {
            model_name: "openai/gpt-4.1-mini",
            constructor_name: openai_gpt_4_1_mini,
            display_name: "GPT-4.1 mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41MiniFast {
            model_name: "openai/gpt-4.1-mini-fast",
            constructor_name: openai_gpt_4_1_mini_fast,
            display_name: "GPT-4.1 mini (Fast)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Nano {
            model_name: "openai/gpt-4.1-nano",
            constructor_name: openai_gpt_4_1_nano,
            display_name: "GPT-4.1 nano",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4o {
            model_name: "openai/gpt-4o",
            constructor_name: openai_gpt_4o,
            display_name: "GPT-4o",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oFast {
            model_name: "openai/gpt-4o-fast",
            constructor_name: openai_gpt_4o_fast,
            display_name: "GPT-4o (Fast)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMini {
            model_name: "openai/gpt-4o-mini",
            constructor_name: openai_gpt_4o_mini,
            display_name: "GPT-4o mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMiniFast {
            model_name: "openai/gpt-4o-mini-fast",
            constructor_name: openai_gpt_4o_mini_fast,
            display_name: "GPT-4o mini (Fast)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMiniTranscribe {
            model_name: "openai/gpt-4o-mini-transcribe",
            constructor_name: openai_gpt_4o_mini_transcribe,
            display_name: "GPT-4o mini Transcribe",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        OpenaiGpt4oTranscribe {
            model_name: "openai/gpt-4o-transcribe",
            constructor_name: openai_gpt_4o_transcribe,
            display_name: "GPT-4o Transcribe",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        OpenaiGpt5 {
            model_name: "openai/gpt-5",
            constructor_name: openai_gpt_5,
            display_name: "GPT-5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        OpenaiGpt5Fast {
            model_name: "openai/gpt-5-fast",
            constructor_name: openai_gpt_5_fast,
            display_name: "GPT-5 (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Mini {
            model_name: "openai/gpt-5-mini",
            constructor_name: openai_gpt_5_mini,
            display_name: "GPT-5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5MiniFast {
            model_name: "openai/gpt-5-mini-fast",
            constructor_name: openai_gpt_5_mini_fast,
            display_name: "GPT-5 mini (Fast)",
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
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Codex {
            model_name: "openai/gpt-5.1-codex",
            constructor_name: openai_gpt_5_1_codex,
            display_name: "GPT-5.1-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51CodexMax {
            model_name: "openai/gpt-5.1-codex-max",
            constructor_name: openai_gpt_5_1_codex_max,
            display_name: "GPT 5.1 Codex Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51CodexMini {
            model_name: "openai/gpt-5.1-codex-mini",
            constructor_name: openai_gpt_5_1_codex_mini,
            display_name: "GPT-5.1 Codex mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        OpenaiGpt51ThinkingFast {
            model_name: "openai/gpt-5.1-thinking-fast",
            constructor_name: openai_gpt_5_1_thinking_fast,
            display_name: "GPT 5.1 Thinking (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52 {
            model_name: "openai/gpt-5.2",
            constructor_name: openai_gpt_5_2,
            display_name: "GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Fast {
            model_name: "openai/gpt-5.2-fast",
            constructor_name: openai_gpt_5_2_fast,
            display_name: "GPT 5.2 (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Pro {
            model_name: "openai/gpt-5.2-pro",
            constructor_name: openai_gpt_5_2_pro,
            display_name: "GPT 5.2 ",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt53Codex {
            model_name: "openai/gpt-5.3-codex",
            constructor_name: openai_gpt_5_3_codex,
            display_name: "GPT 5.3 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt53CodexFast {
            model_name: "openai/gpt-5.3-codex-fast",
            constructor_name: openai_gpt_5_3_codex_fast,
            display_name: "GPT 5.3 Codex (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54 {
            model_name: "openai/gpt-5.4",
            constructor_name: openai_gpt_5_4,
            display_name: "GPT 5.4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54Fast {
            model_name: "openai/gpt-5.4-fast",
            constructor_name: openai_gpt_5_4_fast,
            display_name: "GPT 5.4 (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54Mini {
            model_name: "openai/gpt-5.4-mini",
            constructor_name: openai_gpt_5_4_mini,
            display_name: "GPT 5.4 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54MiniFast {
            model_name: "openai/gpt-5.4-mini-fast",
            constructor_name: openai_gpt_5_4_mini_fast,
            display_name: "GPT 5.4 Mini (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54Nano {
            model_name: "openai/gpt-5.4-nano",
            constructor_name: openai_gpt_5_4_nano,
            display_name: "GPT 5.4 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54Pro {
            model_name: "openai/gpt-5.4-pro",
            constructor_name: openai_gpt_5_4_pro,
            display_name: "GPT 5.4 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt55 {
            model_name: "openai/gpt-5.5",
            constructor_name: openai_gpt_5_5,
            display_name: "GPT 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt55Fast {
            model_name: "openai/gpt-5.5-fast",
            constructor_name: openai_gpt_5_5_fast,
            display_name: "GPT 5.5 (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt55Pro {
            model_name: "openai/gpt-5.5-pro",
            constructor_name: openai_gpt_5_5_pro,
            display_name: "GPT 5.5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Luna {
            model_name: "openai/gpt-5.6-luna",
            constructor_name: openai_gpt_5_6_luna,
            display_name: "GPT 5.6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56LunaFast {
            model_name: "openai/gpt-5.6-luna-fast",
            constructor_name: openai_gpt_5_6_luna_fast,
            display_name: "GPT 5.6 Luna (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Sol {
            model_name: "openai/gpt-5.6-sol",
            constructor_name: openai_gpt_5_6_sol,
            display_name: "GPT 5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56SolFast {
            model_name: "openai/gpt-5.6-sol-fast",
            constructor_name: openai_gpt_5_6_sol_fast,
            display_name: "GPT 5.6 Sol (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Terra {
            model_name: "openai/gpt-5.6-terra",
            constructor_name: openai_gpt_5_6_terra,
            display_name: "GPT 5.6 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56TerraFast {
            model_name: "openai/gpt-5.6-terra-fast",
            constructor_name: openai_gpt_5_6_terra_fast,
            display_name: "GPT 5.6 Terra (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Astra {
            model_name: "openai/gpt-6-astra",
            constructor_name: openai_gpt_6_astra,
            display_name: "GPT-6 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6AstraFast {
            model_name: "openai/gpt-6-astra-fast",
            constructor_name: openai_gpt_6_astra_fast,
            display_name: "GPT-6 Astra (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Luna {
            model_name: "openai/gpt-6-luna",
            constructor_name: openai_gpt_6_luna,
            display_name: "GPT-6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6LunaFast {
            model_name: "openai/gpt-6-luna-fast",
            constructor_name: openai_gpt_6_luna_fast,
            display_name: "GPT-6 Luna (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Sol {
            model_name: "openai/gpt-6-sol",
            constructor_name: openai_gpt_6_sol,
            display_name: "GPT-6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6SolFast {
            model_name: "openai/gpt-6-sol-fast",
            constructor_name: openai_gpt_6_sol_fast,
            display_name: "GPT-6 Sol (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61Sol {
            model_name: "openai/gpt-6.1-sol",
            constructor_name: openai_gpt_6_1_sol,
            display_name: "GPT-6.1 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61SolFast {
            model_name: "openai/gpt-6.1-sol-fast",
            constructor_name: openai_gpt_6_1_sol_fast,
            display_name: "GPT-6.1 Sol (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61Terra {
            model_name: "openai/gpt-6.1-terra",
            constructor_name: openai_gpt_6_1_terra,
            display_name: "GPT-6.1 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61TerraFast {
            model_name: "openai/gpt-6.1-terra-fast",
            constructor_name: openai_gpt_6_1_terra_fast,
            display_name: "GPT-6.1 Terra (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptImage1Mini {
            model_name: "openai/gpt-image-1-mini",
            constructor_name: openai_gpt_image_1_mini,
            display_name: "GPT Image 1 Mini",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        OpenaiGptImage15 {
            model_name: "openai/gpt-image-1.5",
            constructor_name: openai_gpt_image_1_5,
            display_name: "GPT Image 1.5",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        OpenaiGptImage2 {
            model_name: "openai/gpt-image-2",
            constructor_name: openai_gpt_image_2,
            display_name: "GPT Image 2",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        OpenaiGptImage25Flare {
            model_name: "openai/gpt-image-2.5-flare",
            constructor_name: openai_gpt_image_2_5_flare,
            display_name: "GPT Image 2.5 Flare",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        OpenaiGptImage25Sunburst {
            model_name: "openai/gpt-image-2.5-sunburst",
            constructor_name: openai_gpt_image_2_5_sunburst,
            display_name: "GPT Image 2.5 Sunburst",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        OpenaiGptLive1 {
            model_name: "openai/gpt-live-1",
            constructor_name: openai_gpt_live_1,
            display_name: "GPT-Live 1",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOssSafeguard120b {
            model_name: "openai/gpt-oss-safeguard-120b",
            constructor_name: openai_gpt_oss_safeguard_120b,
            display_name: "GPT OSS Safeguard 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOssSafeguard20b {
            model_name: "openai/gpt-oss-safeguard-20b",
            constructor_name: openai_gpt_oss_safeguard_20b,
            display_name: "gpt-oss-safeguard-20b",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptRealtime15 {
            model_name: "openai/gpt-realtime-1.5",
            constructor_name: openai_gpt_realtime_1_5,
            display_name: "GPT-Realtime-1.5",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptRealtime2 {
            model_name: "openai/gpt-realtime-2",
            constructor_name: openai_gpt_realtime_2,
            display_name: "gpt-realtime-2",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptRealtime21 {
            model_name: "openai/gpt-realtime-2.1",
            constructor_name: openai_gpt_realtime_2_1,
            display_name: "gpt-realtime-2.1",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptRealtimeMini {
            model_name: "openai/gpt-realtime-mini",
            constructor_name: openai_gpt_realtime_mini,
            display_name: "GPT-Realtime mini",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptRealtimeWhisper {
            model_name: "openai/gpt-realtime-whisper",
            constructor_name: openai_gpt_realtime_whisper,
            display_name: "gpt-realtime-whisper",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
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
        OpenaiO3Fast {
            model_name: "openai/o3-fast",
            constructor_name: openai_o3_fast,
            display_name: "o3 (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        OpenaiTts1 {
            model_name: "openai/tts-1",
            constructor_name: openai_tts_1,
            display_name: "TTS-1",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        OpenaiTts1Hd {
            model_name: "openai/tts-1-hd",
            constructor_name: openai_tts_1_hd,
            display_name: "TTS-1 HD",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        OpenaiWhisper1 {
            model_name: "openai/whisper-1",
            constructor_name: openai_whisper_1,
            display_name: "Whisper",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        ParticleParticle1 {
            model_name: "particle/particle-1",
            constructor_name: particle_particle_1,
            display_name: "Particle-1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PerplexityPplxEmbedV106b {
            model_name: "perplexity/pplx-embed-v1-0.6b",
            constructor_name: perplexity_pplx_embed_v1_0_6b,
            display_name: "Embed v1 0.6b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        PerplexityPplxEmbedV14b {
            model_name: "perplexity/pplx-embed-v1-4b",
            constructor_name: perplexity_pplx_embed_v1_4b,
            display_name: "Embed v1 4b",
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
        PoolsideLagunaS21 {
            model_name: "poolside/laguna-s-2.1",
            constructor_name: poolside_laguna_s_2_1,
            display_name: "Laguna S 2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PrimeIntellectIntellect3 {
            model_name: "prime-intellect/intellect-3",
            constructor_name: prime_intellect_intellect_3,
            display_name: "INTELLECT 3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QuiveraiArrow2 {
            model_name: "quiverai/arrow-2",
            constructor_name: quiverai_arrow_2,
            display_name: "Arrow 2",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QuiveraiArrow2Telos {
            model_name: "quiverai/arrow-2-telos",
            constructor_name: quiverai_arrow_2_telos,
            display_name: "Arrow 2 Telos",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        RecraftRecraftV4 {
            model_name: "recraft/recraft-v4",
            constructor_name: recraft_recraft_v4,
            display_name: "Recraft V4",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV4Pro {
            model_name: "recraft/recraft-v4-pro",
            constructor_name: recraft_recraft_v4_pro,
            display_name: "Recraft V4 Pro",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV41 {
            model_name: "recraft/recraft-v4.1",
            constructor_name: recraft_recraft_v4_1,
            display_name: "Recraft V4.1",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV41Flash {
            model_name: "recraft/recraft-v4.1-flash",
            constructor_name: recraft_recraft_v4_1_flash,
            display_name: "Recraft V4.1 Flash",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV41Pro {
            model_name: "recraft/recraft-v4.1-pro",
            constructor_name: recraft_recraft_v4_1_pro,
            display_name: "Recraft V4.1 Pro",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV41Utility {
            model_name: "recraft/recraft-v4.1-utility",
            constructor_name: recraft_recraft_v4_1_utility,
            display_name: "Recraft V4.1 Utility",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV41UtilityPro {
            model_name: "recraft/recraft-v4.1-utility-pro",
            constructor_name: recraft_recraft_v4_1_utility_pro,
            display_name: "Recraft V4.1 Utility Pro",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        ReflectionBeam {
            model_name: "reflection/beam",
            constructor_name: reflection_beam,
            display_name: "Beam 1.0",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ReflectionBeamFree {
            model_name: "reflection/beam-free",
            constructor_name: reflection_beam_free,
            display_name: "Beam 1.0 (Free)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SakanaFuguMax {
            model_name: "sakana/fugu-max",
            constructor_name: sakana_fugu_max,
            display_name: "Fugu Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SakanaFuguUltra {
            model_name: "sakana/fugu-ultra",
            constructor_name: sakana_fugu_ultra,
            display_name: "Fugu Ultra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SakanaFuguUltraV2 {
            model_name: "sakana/fugu-ultra-v2",
            constructor_name: sakana_fugu_ultra_v2,
            display_name: "Fugu Ultra v2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SakanaNamazu {
            model_name: "sakana/namazu",
            constructor_name: sakana_namazu,
            display_name: "Sakana Namazu",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ScaledownScaledownExtract1 {
            model_name: "scaledown/scaledown-extract-1",
            constructor_name: scaledown_scaledown_extract_1,
            display_name: "Extract-1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ScaledownScaledownSummarize1 {
            model_name: "scaledown/scaledown-summarize-1",
            constructor_name: scaledown_scaledown_summarize_1,
            display_name: "Summarize-1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SpacexaiGrok420MultiAgent {
            model_name: "spacexai/grok-4.20-multi-agent",
            constructor_name: spacexai_grok_4_20_multi_agent,
            display_name: "Grok 4.20 Multi-Agent",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SpacexaiGrok420MultiAgentBeta {
            model_name: "spacexai/grok-4.20-multi-agent-beta",
            constructor_name: spacexai_grok_4_20_multi_agent_beta,
            display_name: "Grok 4.20 Multi Agent Beta",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SpacexaiGrok420NonReasoning {
            model_name: "spacexai/grok-4.20-non-reasoning",
            constructor_name: spacexai_grok_4_20_non_reasoning,
            display_name: "Grok 4.20 Non-Reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SpacexaiGrok420NonReasoningBeta {
            model_name: "spacexai/grok-4.20-non-reasoning-beta",
            constructor_name: spacexai_grok_4_20_non_reasoning_beta,
            display_name: "Grok 4.20 Beta Non-Reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SpacexaiGrok420Reasoning {
            model_name: "spacexai/grok-4.20-reasoning",
            constructor_name: spacexai_grok_4_20_reasoning,
            display_name: "Grok 4.20 Reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SpacexaiGrok420ReasoningBeta {
            model_name: "spacexai/grok-4.20-reasoning-beta",
            constructor_name: spacexai_grok_4_20_reasoning_beta,
            display_name: "Grok 4.20 Beta Reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SpacexaiGrok43 {
            model_name: "spacexai/grok-4.3",
            constructor_name: spacexai_grok_4_3,
            display_name: "Grok 4.3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SpacexaiGrok45 {
            model_name: "spacexai/grok-4.5",
            constructor_name: spacexai_grok_4_5,
            display_name: "Grok 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SpacexaiGrok46 {
            model_name: "spacexai/grok-4.6",
            constructor_name: spacexai_grok_4_6,
            display_name: "Grok 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SpacexaiGrok47 {
            model_name: "spacexai/grok-4.7",
            constructor_name: spacexai_grok_4_7,
            display_name: "Grok 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SpacexaiGrokBuild01 {
            model_name: "spacexai/grok-build-0.1",
            constructor_name: spacexai_grok_build_0_1,
            display_name: "Grok Build 0.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SpacexaiGrokImagineImage {
            model_name: "spacexai/grok-imagine-image",
            constructor_name: spacexai_grok_imagine_image,
            display_name: "Grok Imagine Image",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        SpacexaiGrokImagineImage20 {
            model_name: "spacexai/grok-imagine-image-2.0",
            constructor_name: spacexai_grok_imagine_image_2_0,
            display_name: "Grok Imagine Image 2.0",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        SpacexaiGrokImagineVideo {
            model_name: "spacexai/grok-imagine-video",
            constructor_name: spacexai_grok_imagine_video,
            display_name: "Grok Imagine",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        SpacexaiGrokImagineVideo15 {
            model_name: "spacexai/grok-imagine-video-1.5",
            constructor_name: spacexai_grok_imagine_video_1_5,
            display_name: "Grok Imagine Video 1.5",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        SpacexaiGrokImagineVideo15Lite {
            model_name: "spacexai/grok-imagine-video-1.5-lite",
            constructor_name: spacexai_grok_imagine_video_1_5_lite,
            display_name: "Grok Imagine Video 1.5 Lite",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        SpacexaiGrokStt {
            model_name: "spacexai/grok-stt",
            constructor_name: spacexai_grok_stt,
            display_name: "Grok STT",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        SpacexaiGrokTts {
            model_name: "spacexai/grok-tts",
            constructor_name: spacexai_grok_tts,
            display_name: "Grok TTS",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        StealthGlyphCluster {
            model_name: "stealth/glyph-cluster",
            constructor_name: stealth_glyph_cluster,
            display_name: "Glyph Cluster",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        StepfunStep37Flash {
            model_name: "stepfun/step-3.7-flash",
            constructor_name: stepfun_step_3_7_flash,
            display_name: "Step 3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        StepfunStep5Preview {
            model_name: "stepfun/step-5-preview",
            constructor_name: stepfun_step_5_preview,
            display_name: "Step 5 Preview",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        TencentHyMt2Lite {
            model_name: "tencent/hy-mt2-lite",
            constructor_name: tencent_hy_mt2_lite,
            display_name: "Tencent Hy-MT2-Lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TencentHyMt2Plus {
            model_name: "tencent/hy-mt2-plus",
            constructor_name: tencent_hy_mt2_plus,
            display_name: "Tencent Hy-MT2-Plus",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TencentHyMt2Pro {
            model_name: "tencent/hy-mt2-pro",
            constructor_name: tencent_hy_mt2_pro,
            display_name: "Tencent Hy-MT2-Pro",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TencentHy3 {
            model_name: "tencent/hy3",
            constructor_name: tencent_hy3,
            display_name: "Hy3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHy4 {
            model_name: "tencent/hy4",
            constructor_name: tencent_hy4,
            display_name: "Tencent Hy4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHy4Preview {
            model_name: "tencent/hy4-preview",
            constructor_name: tencent_hy4_preview,
            display_name: "Tencent Hy4 Preview",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInkling {
            model_name: "thinkingmachines/inkling",
            constructor_name: thinkingmachines_inkling,
            display_name: "Inkling",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInklingSmall {
            model_name: "thinkingmachines/inkling-small",
            constructor_name: thinkingmachines_inkling_small,
            display_name: "Inkling Small",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TopazProteus {
            model_name: "topaz/proteus",
            constructor_name: topaz_proteus,
            display_name: "Proteus",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        TopazStarlightPrecise26 {
            model_name: "topaz/starlight-precise-2.6",
            constructor_name: topaz_starlight_precise_2_6,
            display_name: "Starlight Precise 2.6",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        TopazWonder35 {
            model_name: "topaz/wonder-3.5",
            constructor_name: topaz_wonder_3_5,
            display_name: "Wonder 3.5",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
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
        VoyageRerank25 {
            model_name: "voyage/rerank-2.5",
            constructor_name: voyage_rerank_2_5,
            display_name: "Voyage Rerank 2.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageRerank25Lite {
            model_name: "voyage/rerank-2.5-lite",
            constructor_name: voyage_rerank_2_5_lite,
            display_name: "Voyage Rerank 2.5 Lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageRerank3 {
            model_name: "voyage/rerank-3",
            constructor_name: voyage_rerank_3,
            display_name: "Voyage Rerank 3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageRerank3Lite {
            model_name: "voyage/rerank-3-lite",
            constructor_name: voyage_rerank_3_lite,
            display_name: "Voyage Rerank 3 Lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyage3Large {
            model_name: "voyage/voyage-3-large",
            constructor_name: voyage_voyage_3_large,
            display_name: "voyage-3-large",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        VoyageVoyage4 {
            model_name: "voyage/voyage-4",
            constructor_name: voyage_voyage_4,
            display_name: "voyage-4",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyage4Large {
            model_name: "voyage/voyage-4-large",
            constructor_name: voyage_voyage_4_large,
            display_name: "voyage-4-large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyage4Lite {
            model_name: "voyage/voyage-4-lite",
            constructor_name: voyage_voyage_4_lite,
            display_name: "voyage-4-lite",
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
        XiaomiMimoV26Flash {
            model_name: "xiaomi/mimo-v2.6-flash",
            constructor_name: xiaomi_mimo_v2_6_flash,
            display_name: "MiMo V2.6 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV26Pro {
            model_name: "xiaomi/mimo-v2.6-pro",
            constructor_name: xiaomi_mimo_v2_6_pro,
            display_name: "MiMo V2.6 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV26ProUltraspeed {
            model_name: "xiaomi/mimo-v2.6-pro-ultraspeed",
            constructor_name: xiaomi_mimo_v2_6_pro_ultraspeed,
            display_name: "MiMo V2.6 Pro UltraSpeed",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        ZaiGlm47Flash {
            model_name: "zai/glm-4.7-flash",
            constructor_name: zai_glm_4_7_flash,
            display_name: "GLM 4.7 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm47Flashx {
            model_name: "zai/glm-4.7-flashx",
            constructor_name: zai_glm_4_7_flashx,
            display_name: "GLM 4.7 FlashX",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm5 {
            model_name: "zai/glm-5",
            constructor_name: zai_glm_5,
            display_name: "GLM-5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm5Turbo {
            model_name: "zai/glm-5-turbo",
            constructor_name: zai_glm_5_turbo,
            display_name: "GLM 5 Turbo",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm51 {
            model_name: "zai/glm-5.1",
            constructor_name: zai_glm_5_1,
            display_name: "GLM 5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm52 {
            model_name: "zai/glm-5.2",
            constructor_name: zai_glm_5_2,
            display_name: "GLM 5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm52Fast {
            model_name: "zai/glm-5.2-fast",
            constructor_name: zai_glm_5_2_fast,
            display_name: "GLM 5.2 Fast",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm53 {
            model_name: "zai/glm-5.3",
            constructor_name: zai_glm_5_3,
            display_name: "GLM 5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm53Fast {
            model_name: "zai/glm-5.3-fast",
            constructor_name: zai_glm_5_3_fast,
            display_name: "GLM 5.3 Fast",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm53Flash {
            model_name: "zai/glm-5.3-flash",
            constructor_name: zai_glm_5_3_flash,
            display_name: "GLM 5.3 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm53Flashx {
            model_name: "zai/glm-5.3-flashx",
            constructor_name: zai_glm_5_3_flashx,
            display_name: "GLM 5.3 FlashX",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm53FlashxLegal {
            model_name: "zai/glm-5.3-flashx-legal",
            constructor_name: zai_glm_5_3_flashx_legal,
            display_name: "GLM 5.3 FlashX Legal",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm5vTurbo {
            model_name: "zai/glm-5v-turbo",
            constructor_name: zai_glm_5v_turbo,
            display_name: "GLM 5V Turbo",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
