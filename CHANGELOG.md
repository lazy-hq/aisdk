# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Changelog entries are grouped by type, with the following types:
- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added
- Added `Vercel AI Geatway` provider
- Added `OpenRouter` provider
- Added `DeepSeek` provider
- Added `Amazon Bedrock` provider
- Added `TogetherAI` provider

### Changed
- Changed Groq provider to use OpenAI's ChatCompletions API

## [0.3.0] - 2025-12-29

### Added
- Added `Messages` type alias for `Vec<Message>`.
- Added seamless integration to work with vercel's ai-sdk-ui.
- Added seamless integration to work with axum + vercel's ai-sdk ui.
- Added Google provider
- Added `Extensions` struct for attaching provider-specific metadata to core SDK structures
- Async accessor methods to `StreamTextResponse` for thread-safe data retrieval (`messages()`, `steps()`, `usage()`, etc.)
- Added `builder()` method for `Tool` for easier construction
- `tool` macro is now re-exported from the main `aisdk` crate for easier access
- Automatic retry logic with exponential backoff for rate limit errors (429)

### Changed
- Model capabilities are now enforced at compile time via marker traits, preventing invalid feature usage (e.g., tool calls on unsupported models).
- Updated tool macro to require a return type of `Tool`
- User does not need to import `aisdk::core::tools::ToolExecute` anymore to work with the tool macro
- `LanguageModel` trait now requires `Clone + 'static` bounds (all providers must implement `Clone`)
- Streaming implementation now uses `tokio::sync::mpsc` instead of `std::sync::mpsc`
- Import path for `tool` macro has changed from `use aisdk_macros::tool` to `use aisdk::macros::tool`
- `Error::ApiError` now uses a struct with `status_code: Option<StatusCode>` and `details: String` fields
- `LanguageModelResponseContentType::Reasoning` now includes an `extensions` field for provider-specific metadata


### Removed
- Removed the tool macro re-export from `src/core/mod.rs`. User should use `aisdk_macros::tool` directly
- Removed async-openai dependency
- `Clone` implementation from `LanguageModelRequest<M>`
- `Deref` implementation from `StreamTextResponse` (direct field access no longer works)

### Fixed
- `aisdk-macros` `#[tool]` function unused variable warning even though it is used
- Anthropic provider API endpoint path corrected from `/messages` to `/v1/messages`
- Anthropic provider serialization/deserialization issues with tool calls
- Stream reliability issues that caused premature termination

## [0.2.1] - 2025-12-02

### Added
- OpenAI provider add verbosity option to default value

## [0.2.0] - 2025-12-02

### Added
- Tooll Call Support
- Hooks (StopWhen, OnStepStart, OnStepFinish) for Language Model Requests
- Reasoning message and configuration options
- Anthropic provider
- Groq provider

### Changed
- Changed standalone (generate_text,stream_text) functions to methods on LanguageModelRequest struct
- Rebranded to aisdk

<!-- next-url -->
[Unreleased]: https://github.com/lazy-hq/aisdk/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/lazy-hq/aisdk/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/lazy-hq/aisdk/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/lazy-hq/aisdk/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/lazy-hq/aisdk/releases/tag/v0.1.0
