//! Capabilities for openai models.
//!
//! This module defines model types and their capabilities for openai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::openai::OpenAI;

model_capabilities! {
    provider: OpenAI,
    models: {
        CodexMini {
            model_name: "codex-mini",
            constructor_name: codex_mini,
            display_name: "Codex Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt35Turbo {
            model_name: "gpt-3.5-turbo",
            constructor_name: gpt_3_5_turbo,
            display_name: "GPT-3.5-turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gpt4 {
            model_name: "gpt-4",
            constructor_name: gpt_4,
            display_name: "GPT-4",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41 {
            model_name: "gpt-4.1",
            constructor_name: gpt_4_1,
            display_name: "GPT-4.1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Mini {
            model_name: "gpt-4.1-mini",
            constructor_name: gpt_4_1_mini,
            display_name: "GPT-4.1 mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Nano {
            model_name: "gpt-4.1-nano",
            constructor_name: gpt_4_1_nano,
            display_name: "GPT-4.1 nano",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4Turbo {
            model_name: "gpt-4-turbo",
            constructor_name: gpt_4_turbo,
            display_name: "GPT-4 Turbo",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4o {
            model_name: "gpt-4o",
            constructor_name: gpt_4o,
            display_name: "GPT-4o",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4o20240513 {
            model_name: "gpt-4o-(2024-05-13)",
            constructor_name: gpt_4o_2024_05_13,
            display_name: "GPT-4o (2024-05-13)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4o20240806 {
            model_name: "gpt-4o-(2024-08-06)",
            constructor_name: gpt_4o_2024_08_06,
            display_name: "GPT-4o (2024-08-06)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4o20241120 {
            model_name: "gpt-4o-(2024-11-20)",
            constructor_name: gpt_4o_2024_11_20,
            display_name: "GPT-4o (2024-11-20)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4oMini {
            model_name: "gpt-4o-mini",
            constructor_name: gpt_4o_mini,
            display_name: "GPT-4o mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5 {
            model_name: "gpt-5",
            constructor_name: gpt_5,
            display_name: "GPT-5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51 {
            model_name: "gpt-5.1",
            constructor_name: gpt_5_1,
            display_name: "GPT-5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51Chat {
            model_name: "gpt-5.1-chat",
            constructor_name: gpt_5_1_chat,
            display_name: "GPT-5.1 Chat",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51Codex {
            model_name: "gpt-5.1-codex",
            constructor_name: gpt_5_1_codex,
            display_name: "GPT-5.1 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51CodexMax {
            model_name: "gpt-5.1-codex-max",
            constructor_name: gpt_5_1_codex_max,
            display_name: "GPT-5.1 Codex Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51CodexMini {
            model_name: "gpt-5.1-codex-mini",
            constructor_name: gpt_5_1_codex_mini,
            display_name: "GPT-5.1 Codex mini",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52 {
            model_name: "gpt-5.2",
            constructor_name: gpt_5_2,
            display_name: "GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52Chat {
            model_name: "gpt-5.2-chat",
            constructor_name: gpt_5_2_chat,
            display_name: "GPT-5.2 Chat",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52Pro {
            model_name: "gpt-5.2-pro",
            constructor_name: gpt_5_2_pro,
            display_name: "GPT-5.2 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5ChatLatest {
            model_name: "gpt-5-chat-(latest)",
            constructor_name: gpt_5_chat_latest,
            display_name: "GPT-5 Chat (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Gpt5Codex {
            model_name: "gpt-5-codex",
            constructor_name: gpt_5_codex,
            display_name: "GPT-5-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Mini {
            model_name: "gpt-5-mini",
            constructor_name: gpt_5_mini,
            display_name: "GPT-5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Nano {
            model_name: "gpt-5-nano",
            constructor_name: gpt_5_nano,
            display_name: "GPT-5 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Pro {
            model_name: "gpt-5-pro",
            constructor_name: gpt_5_pro,
            display_name: "GPT-5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O1 {
            model_name: "o1",
            constructor_name: o1,
            display_name: "o1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O1Mini {
            model_name: "o1-mini",
            constructor_name: o1_mini,
            display_name: "o1-mini",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        O1Preview {
            model_name: "o1-preview",
            constructor_name: o1_preview,
            display_name: "o1-preview",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        O1Pro {
            model_name: "o1-pro",
            constructor_name: o1_pro,
            display_name: "o1-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O3 {
            model_name: "o3",
            constructor_name: o3,
            display_name: "o3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O3DeepResearch {
            model_name: "o3-deep-research",
            constructor_name: o3_deep_research,
            display_name: "o3-deep-research",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O3Mini {
            model_name: "o3-mini",
            constructor_name: o3_mini,
            display_name: "o3-mini",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O3Pro {
            model_name: "o3-pro",
            constructor_name: o3_pro,
            display_name: "o3-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O4Mini {
            model_name: "o4-mini",
            constructor_name: o4_mini,
            display_name: "o4-mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O4MiniDeepResearch {
            model_name: "o4-mini-deep-research",
            constructor_name: o4_mini_deep_research,
            display_name: "o4-mini-deep-research",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TextEmbedding3Large {
            model_name: "text-embedding-3-large",
            constructor_name: text_embedding_3_large,
            display_name: "text-embedding-3-large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TextEmbedding3Small {
            model_name: "text-embedding-3-small",
            constructor_name: text_embedding_3_small,
            display_name: "text-embedding-3-small",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TextEmbeddingAda002 {
            model_name: "text-embedding-ada-002",
            constructor_name: text_embedding_ada_002,
            display_name: "text-embedding-ada-002",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
    }
}
