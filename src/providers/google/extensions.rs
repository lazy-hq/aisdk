//! Google tool-specific metadata for extensions.

/// Metadata specific to Google provider tool functionality.
#[derive(Debug, Clone, Default)]
pub(crate) struct GoogleToolMetadata {
    /// The thought signature returned by Gemini 3 models.
    ///
    /// This must be preserved and sent back in subsequent turns
    /// for tool calls to work correctly.
    pub thought_signature: Option<String>,
}
