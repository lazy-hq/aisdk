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

### Changed
- Updated tool macro to require a return type of `Tool`
- User does not need to import `aisdk::core::tools::ToolExecute` anymore to work with the tool macro

### Removed
- Removed the tool macro re-export from `src/core/mod.rs`. User should use `aisdk_macros::tool` directly

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
[Unreleased]: https://github.com/lazy-hq/aisdk/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/lazy-hq/aisdk/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/lazy-hq/aisdk/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/lazy-hq/aisdk/releases/tag/v0.1.0
