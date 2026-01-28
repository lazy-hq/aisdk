//! Settings for the OpenAI Chat Completions API compatible providers.

use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
/// Settings for OpenAI Chat Completions API compatible providers.
///
/// This can be used by Groq, OpenRouter, Together AI, and other providers
/// that implement the OpenAI Chat Completions API format.
pub struct OpenAIChatCompletionsSettings {
    /// The name of the provider (e.g., "groq", "openrouter", "together")
    pub provider_name: String,

    /// The API base URL
    pub base_url: String,

    /// The API key for authentication
    pub api_key: String,
}

impl Default for OpenAIChatCompletionsSettings {
    fn default() -> Self {
        Self {
            provider_name: "openai-chat".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
        }
    }
}
