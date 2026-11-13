//! Capabilities for google models.
//!
//! This module defines model types and their capabilities for google providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::google::Google;

model_capabilities! {
    provider: Google,
    models: {
        DeepResearchMaxPreview042026 {
            model_name: "deep-research-max-preview-04-2026",
            constructor_name: deep_research_max_preview_04_2026,
            display_name: "Deep Research Max Preview (Apr-21-2026)",
            capabilities: [AudioInputSupport, ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DeepResearchPreview042026 {
            model_name: "deep-research-preview-04-2026",
            constructor_name: deep_research_preview_04_2026,
            display_name: "Deep Research Preview (Apr-21-2026)",
            capabilities: [AudioInputSupport, ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini15Flash {
            model_name: "gemini-1.5-flash",
            constructor_name: gemini_1_5_flash,
            display_name: "Gemini 1.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini15Flash8b {
            model_name: "gemini-1.5-flash-8b",
            constructor_name: gemini_1_5_flash_8b,
            display_name: "Gemini 1.5 Flash-8B",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini15Pro {
            model_name: "gemini-1.5-pro",
            constructor_name: gemini_1_5_pro,
            display_name: "Gemini 1.5 Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini20Flash {
            model_name: "gemini-2.0-flash",
            constructor_name: gemini_2_0_flash,
            display_name: "Gemini 2.0 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini20FlashLite {
            model_name: "gemini-2.0-flash-lite",
            constructor_name: gemini_2_0_flash_lite,
            display_name: "Gemini 2.0 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25ComputerUsePreview102025 {
            model_name: "gemini-2.5-computer-use-preview-10-2025",
            constructor_name: gemini_2_5_computer_use_preview_10_2025,
            display_name: "Gemini 2.5 Computer Use Preview 10-2025",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25Flash {
            model_name: "gemini-2.5-flash",
            constructor_name: gemini_2_5_flash,
            display_name: "Gemini 2.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashImage {
            model_name: "gemini-2.5-flash-image",
            constructor_name: gemini_2_5_flash_image,
            display_name: "Gemini 2.5 Flash Image",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashImagePreview {
            model_name: "gemini-2.5-flash-image-preview",
            constructor_name: gemini_2_5_flash_image_preview,
            display_name: "Gemini 2.5 Flash Image (Preview)",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashLite {
            model_name: "gemini-2.5-flash-lite",
            constructor_name: gemini_2_5_flash_lite,
            display_name: "Gemini 2.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashLitePreview0617 {
            model_name: "gemini-2.5-flash-lite-preview-06-17",
            constructor_name: gemini_2_5_flash_lite_preview_06_17,
            display_name: "Gemini 2.5 Flash Lite Preview 06-17",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashLitePreview092025 {
            model_name: "gemini-2.5-flash-lite-preview-09-2025",
            constructor_name: gemini_2_5_flash_lite_preview_09_2025,
            display_name: "Gemini 2.5 Flash Lite Preview 09-25",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashPreview0417 {
            model_name: "gemini-2.5-flash-preview-04-17",
            constructor_name: gemini_2_5_flash_preview_04_17,
            display_name: "Gemini 2.5 Flash Preview 04-17",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashPreview0520 {
            model_name: "gemini-2.5-flash-preview-05-20",
            constructor_name: gemini_2_5_flash_preview_05_20,
            display_name: "Gemini 2.5 Flash Preview 05-20",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashPreview092025 {
            model_name: "gemini-2.5-flash-preview-09-2025",
            constructor_name: gemini_2_5_flash_preview_09_2025,
            display_name: "Gemini 2.5 Flash Preview 09-25",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashPreviewTts {
            model_name: "gemini-2.5-flash-preview-tts",
            constructor_name: gemini_2_5_flash_preview_tts,
            display_name: "Gemini 2.5 Flash Preview TTS",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        Gemini25Pro {
            model_name: "gemini-2.5-pro",
            constructor_name: gemini_2_5_pro,
            display_name: "Gemini 2.5 Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25ProPreview0506 {
            model_name: "gemini-2.5-pro-preview-05-06",
            constructor_name: gemini_2_5_pro_preview_05_06,
            display_name: "Gemini 2.5 Pro Preview 05-06",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25ProPreview0605 {
            model_name: "gemini-2.5-pro-preview-06-05",
            constructor_name: gemini_2_5_pro_preview_06_05,
            display_name: "Gemini 2.5 Pro Preview 06-05",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25ProPreviewTts {
            model_name: "gemini-2.5-pro-preview-tts",
            constructor_name: gemini_2_5_pro_preview_tts,
            display_name: "Gemini 2.5 Pro Preview TTS",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        Gemini3FlashPreview {
            model_name: "gemini-3-flash-preview",
            constructor_name: gemini_3_flash_preview,
            display_name: "Gemini 3 Flash Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini3ProImage {
            model_name: "gemini-3-pro-image",
            constructor_name: gemini_3_pro_image,
            display_name: "Nano Banana Pro",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini3ProPreview {
            model_name: "gemini-3-pro-preview",
            constructor_name: gemini_3_pro_preview,
            display_name: "Gemini 3 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini31FlashImage {
            model_name: "gemini-3.1-flash-image",
            constructor_name: gemini_3_1_flash_image,
            display_name: "Nano Banana 2",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Gemini31FlashLite {
            model_name: "gemini-3.1-flash-lite",
            constructor_name: gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini31FlashLiteImage {
            model_name: "gemini-3.1-flash-lite-image",
            constructor_name: gemini_3_1_flash_lite_image,
            display_name: "Nano Banana 2 Lite",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini31FlashLivePreview {
            model_name: "gemini-3.1-flash-live-preview",
            constructor_name: gemini_3_1_flash_live_preview,
            display_name: "Gemini 3.1 Flash Live Preview",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini31FlashTtsPreview {
            model_name: "gemini-3.1-flash-tts-preview",
            constructor_name: gemini_3_1_flash_tts_preview,
            display_name: "Gemini 3.1 Flash TTS Preview",
            capabilities: [AudioOutputSupport, ReasoningSupport, TextInputSupport]
        },
        Gemini31ProPreview {
            model_name: "gemini-3.1-pro-preview",
            constructor_name: gemini_3_1_pro_preview,
            display_name: "Gemini 3.1 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini31ProPreviewCustomtools {
            model_name: "gemini-3.1-pro-preview-customtools",
            constructor_name: gemini_3_1_pro_preview_customtools,
            display_name: "Gemini 3.1 Pro Preview Custom Tools",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini35Flash {
            model_name: "gemini-3.5-flash",
            constructor_name: gemini_3_5_flash,
            display_name: "Gemini 3.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini35FlashLite {
            model_name: "gemini-3.5-flash-lite",
            constructor_name: gemini_3_5_flash_lite,
            display_name: "Gemini 3.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini35LiveTranslatePreview {
            model_name: "gemini-3.5-live-translate-preview",
            constructor_name: gemini_3_5_live_translate_preview,
            display_name: "Gemini 3.5 Live Translate Preview",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextOutputSupport]
        },
        Gemini36Flash {
            model_name: "gemini-3.6-flash",
            constructor_name: gemini_3_6_flash,
            display_name: "Gemini 3.6 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini37Flash {
            model_name: "gemini-3.7-flash",
            constructor_name: gemini_3_7_flash,
            display_name: "Gemini 3.7 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini38Flash {
            model_name: "gemini-3.8-flash",
            constructor_name: gemini_3_8_flash,
            display_name: "Gemini 3.8 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini4Argon {
            model_name: "gemini-4-argon",
            constructor_name: gemini_4_argon,
            display_name: "Gemini 4 Argon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini4Carbon {
            model_name: "gemini-4-carbon",
            constructor_name: gemini_4_carbon,
            display_name: "Gemini 4 Carbon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini4Hydrogen {
            model_name: "gemini-4-hydrogen",
            constructor_name: gemini_4_hydrogen,
            display_name: "Gemini 4 Hydrogen",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GeminiEmbedding001 {
            model_name: "gemini-embedding-001",
            constructor_name: gemini_embedding_001,
            display_name: "Gemini Embedding 001",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GeminiEmbedding2 {
            model_name: "gemini-embedding-2",
            constructor_name: gemini_embedding_2,
            display_name: "Gemini Embedding 2",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        GeminiFlashLatest {
            model_name: "gemini-flash-latest",
            constructor_name: gemini_flash_latest,
            display_name: "Gemini Flash Latest",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GeminiFlashLiteLatest {
            model_name: "gemini-flash-lite-latest",
            constructor_name: gemini_flash_lite_latest,
            display_name: "Gemini Flash-Lite Latest",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GeminiLive25Flash {
            model_name: "gemini-live-2.5-flash",
            constructor_name: gemini_live_2_5_flash,
            display_name: "Gemini Live 2.5 Flash",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GeminiLive25FlashPreviewNativeAudio {
            model_name: "gemini-live-2.5-flash-preview-native-audio",
            constructor_name: gemini_live_2_5_flash_preview_native_audio,
            display_name: "Gemini Live 2.5 Flash Preview Native Audio",
            capabilities: [AudioInputSupport, AudioOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GeminiOmniFlashPreview {
            model_name: "gemini-omni-flash-preview",
            constructor_name: gemini_omni_flash_preview,
            display_name: "Gemini Omni Flash Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        GeminiProLatest {
            model_name: "gemini-pro-latest",
            constructor_name: gemini_pro_latest,
            display_name: "Gemini Pro Latest",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemma426bA4bIt {
            model_name: "gemma-4-26b-a4b-it",
            constructor_name: gemma_4_26b_a4b_it,
            display_name: "Gemma 4 26B A4B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bIt {
            model_name: "gemma-4-31b-it",
            constructor_name: gemma_4_31b_it,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Lyria3ClipPreview {
            model_name: "lyria-3-clip-preview",
            constructor_name: lyria_3_clip_preview,
            display_name: "Lyria 3 Clip Preview",
            capabilities: [AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Lyria3ProPreview {
            model_name: "lyria-3-pro-preview",
            constructor_name: lyria_3_pro_preview,
            display_name: "Lyria 3 Pro Preview",
            capabilities: [AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
    }
}
