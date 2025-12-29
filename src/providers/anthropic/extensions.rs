//! Anthropic-specific metadata for extensions.

/// Metadata specific to Anthropic provider thinking functionality.
#[derive(Debug, Clone, Default)]
pub(crate) struct AnthropicThinkingMetadata {
    /// The signature returned by Anthropic models for thinking blocks.
    ///
    /// This must be preserved and sent back in subsequent turns
    /// for extended thinking to work correctly.
    pub signature: Option<String>,
}
