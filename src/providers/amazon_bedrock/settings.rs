//! Defines the settings for the Amazon Bedrock provider.

use derive_builder::Builder;

/// Settings for the Amazon Bedrock provider (delegates to OpenAI).
#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
pub struct AmazonBedrockProviderSettings {
    /// The name of the provider. Defaults to "AmazonBedrock".
    pub provider_name: String,

    /// The base URL for the Amazon Bedrock Runtime API.
    /// Format: https://bedrock-runtime.{region}.amazonaws.com/openai/
    pub base_url: String,

    /// The API key for Amazon Bedrock (AWS Bearer Token).
    pub api_key: String,
}

impl Default for AmazonBedrockProviderSettings {
    /// Returns the default settings for the Amazon Bedrock provider.
    fn default() -> Self {
        Self {
            provider_name: "AmazonBedrock".to_string(),
            base_url: "https://bedrock-runtime.us-east-1.amazonaws.com/openai/".to_string(),
            api_key: std::env::var("BEDROCK_API_KEY").unwrap_or_default(),
        }
    }
}

impl AmazonBedrockProviderSettings {
    /// Creates a new builder for `AmazonBedrockProviderSettings`.
    pub fn builder() -> AmazonBedrockProviderSettingsBuilder {
        AmazonBedrockProviderSettingsBuilder::default()
    }
}
