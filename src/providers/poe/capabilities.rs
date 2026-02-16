//! Capabilities for poe models.
//!
//! This module defines model types and their capabilities for poe providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::poe::Poe;

model_capabilities! {
    provider: Poe,
    models: {
        AnthropicClaudeHaiku3 {
            model_name: "anthropic/claude-haiku-3",
            constructor_name: anthropic_claude_haiku_3,
            display_name: "Claude-Haiku-3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku35 {
            model_name: "anthropic/claude-haiku-3.5",
            constructor_name: anthropic_claude_haiku_3_5,
            display_name: "Claude-Haiku-3.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku45 {
            model_name: "anthropic/claude-haiku-4.5",
            constructor_name: anthropic_claude_haiku_4_5,
            display_name: "Claude-Haiku-4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus4 {
            model_name: "anthropic/claude-opus-4",
            constructor_name: anthropic_claude_opus_4,
            display_name: "Claude-Opus-4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus41 {
            model_name: "anthropic/claude-opus-4.1",
            constructor_name: anthropic_claude_opus_4_1,
            display_name: "Claude-Opus-4.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus45 {
            model_name: "anthropic/claude-opus-4.5",
            constructor_name: anthropic_claude_opus_4_5,
            display_name: "Claude-Opus-4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46 {
            model_name: "anthropic/claude-opus-4.6",
            constructor_name: anthropic_claude_opus_4_6,
            display_name: "Claude-Opus-4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet35 {
            model_name: "anthropic/claude-sonnet-3.5",
            constructor_name: anthropic_claude_sonnet_3_5,
            display_name: "Claude-Sonnet-3.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet35June {
            model_name: "anthropic/claude-sonnet-3.5-june",
            constructor_name: anthropic_claude_sonnet_3_5_june,
            display_name: "Claude-Sonnet-3.5-June",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet37 {
            model_name: "anthropic/claude-sonnet-3.7",
            constructor_name: anthropic_claude_sonnet_3_7,
            display_name: "Claude-Sonnet-3.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet4 {
            model_name: "anthropic/claude-sonnet-4",
            constructor_name: anthropic_claude_sonnet_4,
            display_name: "Claude-Sonnet-4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet45 {
            model_name: "anthropic/claude-sonnet-4.5",
            constructor_name: anthropic_claude_sonnet_4_5,
            display_name: "Claude-Sonnet-4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CerebrasGptOss120bCs {
            model_name: "cerebras/gpt-oss-120b-cs",
            constructor_name: cerebras_gpt_oss_120b_cs,
            display_name: "gpt-oss-120b-cs",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CerebrasLlama318bCs {
            model_name: "cerebras/llama-3.1-8b-cs",
            constructor_name: cerebras_llama_3_1_8b_cs,
            display_name: "llama-3.1-8b-cs",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CerebrasLlama3370bCs {
            model_name: "cerebras/llama-3.3-70b-cs",
            constructor_name: cerebras_llama_3_3_70b_cs,
            display_name: "llama-3.3-70b-cs",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        CerebrasQwen3235b2507Cs {
            model_name: "cerebras/qwen3-235b-2507-cs",
            constructor_name: cerebras_qwen3_235b_2507_cs,
            display_name: "qwen3-235b-2507-cs",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CerebrasQwen332bCs {
            model_name: "cerebras/qwen3-32b-cs",
            constructor_name: cerebras_qwen3_32b_cs,
            display_name: "qwen3-32b-cs",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ElevenlabsElevenlabsMusic {
            model_name: "elevenlabs/elevenlabs-music",
            constructor_name: elevenlabs_elevenlabs_music,
            display_name: "ElevenLabs-Music",
            capabilities: [AudioOutputSupport, ImageInputSupport, TextInputSupport, ToolCallSupport]
        },
        ElevenlabsElevenlabsV25Turbo {
            model_name: "elevenlabs/elevenlabs-v2.5-turbo",
            constructor_name: elevenlabs_elevenlabs_v2_5_turbo,
            display_name: "ElevenLabs-v2.5-Turbo",
            capabilities: [AudioOutputSupport, ImageInputSupport, TextInputSupport, ToolCallSupport]
        },
        ElevenlabsElevenlabsV3 {
            model_name: "elevenlabs/elevenlabs-v3",
            constructor_name: elevenlabs_elevenlabs_v3,
            display_name: "ElevenLabs-v3",
            capabilities: [AudioOutputSupport, ImageInputSupport, TextInputSupport, ToolCallSupport]
        },
        GoogleGemini20Flash {
            model_name: "google/gemini-2.0-flash",
            constructor_name: google_gemini_2_0_flash,
            display_name: "Gemini-2.0-Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini20FlashLite {
            model_name: "google/gemini-2.0-flash-lite",
            constructor_name: google_gemini_2_0_flash_lite,
            display_name: "Gemini-2.0-Flash-Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25Flash {
            model_name: "google/gemini-2.5-flash",
            constructor_name: google_gemini_2_5_flash,
            display_name: "Gemini-2.5-Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25FlashLite {
            model_name: "google/gemini-2.5-flash-lite",
            constructor_name: google_gemini_2_5_flash_lite,
            display_name: "Gemini-2.5-Flash-Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25Pro {
            model_name: "google/gemini-2.5-pro",
            constructor_name: google_gemini_2_5_pro,
            display_name: "Gemini-2.5-Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini3Flash {
            model_name: "google/gemini-3-flash",
            constructor_name: google_gemini_3_flash,
            display_name: "Gemini-3-Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini3Pro {
            model_name: "google/gemini-3-pro",
            constructor_name: google_gemini_3_pro,
            display_name: "Gemini-3-Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGeminiDeepResearch {
            model_name: "google/gemini-deep-research",
            constructor_name: google_gemini_deep_research,
            display_name: "gemini-deep-research",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleImagen3 {
            model_name: "google/imagen-3",
            constructor_name: google_imagen_3,
            display_name: "Imagen-3",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        GoogleImagen3Fast {
            model_name: "google/imagen-3-fast",
            constructor_name: google_imagen_3_fast,
            display_name: "Imagen-3-Fast",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        GoogleImagen4 {
            model_name: "google/imagen-4",
            constructor_name: google_imagen_4,
            display_name: "Imagen-4",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        GoogleImagen4Fast {
            model_name: "google/imagen-4-fast",
            constructor_name: google_imagen_4_fast,
            display_name: "Imagen-4-Fast",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        GoogleImagen4Ultra {
            model_name: "google/imagen-4-ultra",
            constructor_name: google_imagen_4_ultra,
            display_name: "Imagen-4-Ultra",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        GoogleLyria {
            model_name: "google/lyria",
            constructor_name: google_lyria,
            display_name: "Lyria",
            capabilities: [AudioOutputSupport, ImageInputSupport, TextInputSupport, ToolCallSupport]
        },
        GoogleNanoBanana {
            model_name: "google/nano-banana",
            constructor_name: google_nano_banana,
            display_name: "Nano-Banana",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleNanoBananaPro {
            model_name: "google/nano-banana-pro",
            constructor_name: google_nano_banana_pro,
            display_name: "Nano-Banana-Pro",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        GoogleVeo2 {
            model_name: "google/veo-2",
            constructor_name: google_veo_2,
            display_name: "Veo-2",
            capabilities: [ImageInputSupport, TextInputSupport, ToolCallSupport, VideoOutputSupport]
        },
        GoogleVeo3 {
            model_name: "google/veo-3",
            constructor_name: google_veo_3,
            display_name: "Veo-3",
            capabilities: [ImageInputSupport, TextInputSupport, ToolCallSupport, VideoOutputSupport]
        },
        GoogleVeo3Fast {
            model_name: "google/veo-3-fast",
            constructor_name: google_veo_3_fast,
            display_name: "Veo-3-Fast",
            capabilities: [ImageInputSupport, TextInputSupport, ToolCallSupport, VideoOutputSupport]
        },
        GoogleVeo31 {
            model_name: "google/veo-3.1",
            constructor_name: google_veo_3_1,
            display_name: "Veo-3.1",
            capabilities: [ImageInputSupport, TextInputSupport, ToolCallSupport, VideoOutputSupport]
        },
        GoogleVeo31Fast {
            model_name: "google/veo-3.1-fast",
            constructor_name: google_veo_3_1_fast,
            display_name: "Veo-3.1-Fast",
            capabilities: [ImageInputSupport, TextInputSupport, ToolCallSupport, VideoOutputSupport]
        },
        IdeogramaiIdeogram {
            model_name: "ideogramai/ideogram",
            constructor_name: ideogramai_ideogram,
            display_name: "Ideogram",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        IdeogramaiIdeogramV2 {
            model_name: "ideogramai/ideogram-v2",
            constructor_name: ideogramai_ideogram_v2,
            display_name: "Ideogram-v2",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        IdeogramaiIdeogramV2a {
            model_name: "ideogramai/ideogram-v2a",
            constructor_name: ideogramai_ideogram_v2a,
            display_name: "Ideogram-v2a",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        IdeogramaiIdeogramV2aTurbo {
            model_name: "ideogramai/ideogram-v2a-turbo",
            constructor_name: ideogramai_ideogram_v2a_turbo,
            display_name: "Ideogram-v2a-Turbo",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        LumalabsRay2 {
            model_name: "lumalabs/ray2",
            constructor_name: lumalabs_ray2,
            display_name: "Ray2",
            capabilities: [ImageInputSupport, TextInputSupport, ToolCallSupport, VideoOutputSupport]
        },
        NovitaGlm46 {
            model_name: "novita/glm-4.6",
            constructor_name: novita_glm_4_6,
            display_name: "GLM-4.6",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NovitaGlm46v {
            model_name: "novita/glm-4.6v",
            constructor_name: novita_glm_4_6v,
            display_name: "glm-4.6v",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NovitaGlm47 {
            model_name: "novita/glm-4.7",
            constructor_name: novita_glm_4_7,
            display_name: "glm-4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NovitaGlm47Flash {
            model_name: "novita/glm-4.7-flash",
            constructor_name: novita_glm_4_7_flash,
            display_name: "glm-4.7-flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NovitaGlm47N {
            model_name: "novita/glm-4.7-n",
            constructor_name: novita_glm_4_7_n,
            display_name: "glm-4.7-n",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NovitaKimiK2Thinking {
            model_name: "novita/kimi-k2-thinking",
            constructor_name: novita_kimi_k2_thinking,
            display_name: "kimi-k2-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NovitaKimiK25 {
            model_name: "novita/kimi-k2.5",
            constructor_name: novita_kimi_k2_5,
            display_name: "kimi-k2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NovitaMinimaxM21 {
            model_name: "novita/minimax-m2.1",
            constructor_name: novita_minimax_m2_1,
            display_name: "minimax-m2.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiChatgpt4oLatest {
            model_name: "openai/chatgpt-4o-latest",
            constructor_name: openai_chatgpt_4o_latest,
            display_name: "ChatGPT-4o-Latest",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiDallE3 {
            model_name: "openai/dall-e-3",
            constructor_name: openai_dall_e_3,
            display_name: "DALL-E-3",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        OpenaiGpt35Turbo {
            model_name: "openai/gpt-3.5-turbo",
            constructor_name: openai_gpt_3_5_turbo,
            display_name: "GPT-3.5-Turbo",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt35TurboInstruct {
            model_name: "openai/gpt-3.5-turbo-instruct",
            constructor_name: openai_gpt_3_5_turbo_instruct,
            display_name: "GPT-3.5-Turbo-Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt35TurboRaw {
            model_name: "openai/gpt-3.5-turbo-raw",
            constructor_name: openai_gpt_3_5_turbo_raw,
            display_name: "GPT-3.5-Turbo-Raw",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4Classic {
            model_name: "openai/gpt-4-classic",
            constructor_name: openai_gpt_4_classic,
            display_name: "GPT-4-Classic",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4Classic0314 {
            model_name: "openai/gpt-4-classic-0314",
            constructor_name: openai_gpt_4_classic_0314,
            display_name: "GPT-4-Classic-0314",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4Turbo {
            model_name: "openai/gpt-4-turbo",
            constructor_name: openai_gpt_4_turbo,
            display_name: "GPT-4-Turbo",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41 {
            model_name: "openai/gpt-4.1",
            constructor_name: openai_gpt_4_1,
            display_name: "GPT-4.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Mini {
            model_name: "openai/gpt-4.1-mini",
            constructor_name: openai_gpt_4_1_mini,
            display_name: "GPT-4.1-mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Nano {
            model_name: "openai/gpt-4.1-nano",
            constructor_name: openai_gpt_4_1_nano,
            display_name: "GPT-4.1-nano",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4o {
            model_name: "openai/gpt-4o",
            constructor_name: openai_gpt_4o,
            display_name: "GPT-4o",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oAug {
            model_name: "openai/gpt-4o-aug",
            constructor_name: openai_gpt_4o_aug,
            display_name: "GPT-4o-Aug",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMini {
            model_name: "openai/gpt-4o-mini",
            constructor_name: openai_gpt_4o_mini,
            display_name: "GPT-4o-mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMiniSearch {
            model_name: "openai/gpt-4o-mini-search",
            constructor_name: openai_gpt_4o_mini_search,
            display_name: "GPT-4o-mini-Search",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oSearch {
            model_name: "openai/gpt-4o-search",
            constructor_name: openai_gpt_4o_search,
            display_name: "GPT-4o-Search",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5 {
            model_name: "openai/gpt-5",
            constructor_name: openai_gpt_5,
            display_name: "GPT-5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Chat {
            model_name: "openai/gpt-5-chat",
            constructor_name: openai_gpt_5_chat,
            display_name: "GPT-5-Chat",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Codex {
            model_name: "openai/gpt-5-codex",
            constructor_name: openai_gpt_5_codex,
            display_name: "GPT-5-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Mini {
            model_name: "openai/gpt-5-mini",
            constructor_name: openai_gpt_5_mini,
            display_name: "GPT-5-mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Nano {
            model_name: "openai/gpt-5-nano",
            constructor_name: openai_gpt_5_nano,
            display_name: "GPT-5-nano",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Pro {
            model_name: "openai/gpt-5-pro",
            constructor_name: openai_gpt_5_pro,
            display_name: "GPT-5-Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51 {
            model_name: "openai/gpt-5.1",
            constructor_name: openai_gpt_5_1,
            display_name: "GPT-5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            display_name: "GPT-5.1-Codex-Max",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51CodexMini {
            model_name: "openai/gpt-5.1-codex-mini",
            constructor_name: openai_gpt_5_1_codex_mini,
            display_name: "GPT-5.1-Codex-Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Instant {
            model_name: "openai/gpt-5.1-instant",
            constructor_name: openai_gpt_5_1_instant,
            display_name: "GPT-5.1-Instant",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52 {
            model_name: "openai/gpt-5.2",
            constructor_name: openai_gpt_5_2,
            display_name: "GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Codex {
            model_name: "openai/gpt-5.2-codex",
            constructor_name: openai_gpt_5_2_codex,
            display_name: "GPT-5.2-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Instant {
            model_name: "openai/gpt-5.2-instant",
            constructor_name: openai_gpt_5_2_instant,
            display_name: "GPT-5.2-Instant",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Pro {
            model_name: "openai/gpt-5.2-pro",
            constructor_name: openai_gpt_5_2_pro,
            display_name: "GPT-5.2-Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptImage1 {
            model_name: "openai/gpt-image-1",
            constructor_name: openai_gpt_image_1,
            display_name: "GPT-Image-1",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        OpenaiGptImage1Mini {
            model_name: "openai/gpt-image-1-mini",
            constructor_name: openai_gpt_image_1_mini,
            display_name: "GPT-Image-1-Mini",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        OpenaiGptImage15 {
            model_name: "openai/gpt-image-1.5",
            constructor_name: openai_gpt_image_1_5,
            display_name: "gpt-image-1.5",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        OpenaiO1 {
            model_name: "openai/o1",
            constructor_name: openai_o1,
            display_name: "o1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO1Pro {
            model_name: "openai/o1-pro",
            constructor_name: openai_o1_pro,
            display_name: "o1-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3 {
            model_name: "openai/o3",
            constructor_name: openai_o3,
            display_name: "o3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3MiniHigh {
            model_name: "openai/o3-mini-high",
            constructor_name: openai_o3_mini_high,
            display_name: "o3-mini-high",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3Pro {
            model_name: "openai/o3-pro",
            constructor_name: openai_o3_pro,
            display_name: "o3-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO4Mini {
            model_name: "openai/o4-mini",
            constructor_name: openai_o4_mini,
            display_name: "o4-mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO4MiniDeepResearch {
            model_name: "openai/o4-mini-deep-research",
            constructor_name: openai_o4_mini_deep_research,
            display_name: "o4-mini-deep-research",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiSora2 {
            model_name: "openai/sora-2",
            constructor_name: openai_sora_2,
            display_name: "Sora-2",
            capabilities: [ImageInputSupport, TextInputSupport, ToolCallSupport, VideoOutputSupport]
        },
        OpenaiSora2Pro {
            model_name: "openai/sora-2-pro",
            constructor_name: openai_sora_2_pro,
            display_name: "Sora-2-Pro",
            capabilities: [ImageInputSupport, TextInputSupport, ToolCallSupport, VideoOutputSupport]
        },
        PoetoolsClaudeCode {
            model_name: "poetools/claude-code",
            constructor_name: poetools_claude_code,
            display_name: "claude-code",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RunwaymlRunway {
            model_name: "runwayml/runway",
            constructor_name: runwayml_runway,
            display_name: "Runway",
            capabilities: [ImageInputSupport, TextInputSupport, ToolCallSupport, VideoOutputSupport]
        },
        RunwaymlRunwayGen4Turbo {
            model_name: "runwayml/runway-gen-4-turbo",
            constructor_name: runwayml_runway_gen_4_turbo,
            display_name: "Runway-Gen-4-Turbo",
            capabilities: [ImageInputSupport, TextInputSupport, ToolCallSupport, VideoOutputSupport]
        },
        StabilityaiStablediffusionxl {
            model_name: "stabilityai/stablediffusionxl",
            constructor_name: stabilityai_stablediffusionxl,
            display_name: "StableDiffusionXL",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        TopazlabsCoTopazlabs {
            model_name: "topazlabs-co/topazlabs",
            constructor_name: topazlabs_co_topazlabs,
            display_name: "TopazLabs",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, ToolCallSupport]
        },
        TrytakoTako {
            model_name: "trytako/tako",
            constructor_name: trytako_tako,
            display_name: "Tako",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok3 {
            model_name: "xai/grok-3",
            constructor_name: xai_grok_3,
            display_name: "Grok 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok3Mini {
            model_name: "xai/grok-3-mini",
            constructor_name: xai_grok_3_mini,
            display_name: "Grok 3 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok4 {
            model_name: "xai/grok-4",
            constructor_name: xai_grok_4,
            display_name: "Grok 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok4FastNonReasoning {
            model_name: "xai/grok-4-fast-non-reasoning",
            constructor_name: xai_grok_4_fast_non_reasoning,
            display_name: "Grok-4-Fast-Non-Reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok4FastReasoning {
            model_name: "xai/grok-4-fast-reasoning",
            constructor_name: xai_grok_4_fast_reasoning,
            display_name: "Grok 4 Fast Reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok41FastNonReasoning {
            model_name: "xai/grok-4.1-fast-non-reasoning",
            constructor_name: xai_grok_4_1_fast_non_reasoning,
            display_name: "Grok-4.1-Fast-Non-Reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok41FastReasoning {
            model_name: "xai/grok-4.1-fast-reasoning",
            constructor_name: xai_grok_4_1_fast_reasoning,
            display_name: "Grok-4.1-Fast-Reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrokCodeFast1 {
            model_name: "xai/grok-code-fast-1",
            constructor_name: xai_grok_code_fast_1,
            display_name: "Grok Code Fast 1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
