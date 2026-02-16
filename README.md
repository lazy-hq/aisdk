# AISDK

[![Docs](https://img.shields.io/badge/docs-latest-blue)](https://aisdk.rs)
[![Build Status](https://github.com/lazy-hq/aisdk/actions/workflows/ci.yml/badge.svg)](https://github.com/lazy-hq/aisdk/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Issues](https://img.shields.io/github/issues/lazy-hq/aisdk)](https://github.com/lazy-hq/aisdk/issues)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/lazy-hq/aisdk/pulls)


An open-source, **provider-agnostic** Rust library for building AI-powered applications, inspired by the Vercel AI SDK. Type-safe, framework-friendly, and ready to connect with **70+ AI providers**.

To learn more about how to use the AI SDK, check out our [Documentation](https://aisdk.rs) and [API Reference](https://docs.rs/aisdk/latest).

## Features
* Agents & Tool Execution
* Prompt Templating
* Text Generation & Streaming
* Structured Output (JSON Schema)
* Embedding Model Support
* Compatible with [Vercel AI SDK UI](https://ai-sdk.dev/docs/ai-sdk-ui/overview) (React, Solid, Vue, Svelte, …)
* Supports 73+ providers, including Anthropic, Google, OpenAI, OpenRouter, xAI

## Installation

```bash
cargo add aisdk
```

## Usage

Enable Providers of your choice such as OpenAI, Anthropic, Google, and [more](https://aisdk.rs/docs#model-providers)

### Example with OpenAI provider

```bash
cargo add aisdk --features openai
```

### Basic Text Generation

```rust
use aisdk::core::LanguageModelRequest;
use aisdk::providers::OpenAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let openai = OpenAI::gpt_5();

    let result = LanguageModelRequest::builder()
        .model(openai)
        .prompt("What is the meaning of life?")
        .build()
        .generate_text() // or stream_text() for streaming
        .await?;

    println!("Response: {:?}", result.text());
    Ok(())
}
```

## Agents

### Defining a Tool

Use the `#[tool]` macro to expose a Rust function as a callable tool.

```rust
use aisdk::core::Tool;
use aisdk::macros::tool;

#[tool]
/// Get the weather information given a location
pub fn get_weather(location: String) -> Tool {
    let weather = match location.as_str() {
        "New York" => 75,
        "Tokyo" => 80,
        _ => 70,
    };
    Ok(weather.to_string())
}
```

### Using Tools in an Agent

Register tools with an agent so the model can call them during its reasoning loop.

```rust
use aisdk::core::{LanguageModelRequest, utils::step_count_is};
use aisdk::providers::OpenAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let result = LanguageModelRequest::builder()
        .model(OpenAI::gpt_4o())
        .system("You are a helpful assistant.")
        .prompt("What is the weather in New York?")
        .with_tool(get_weather())
        .stop_when(step_count_is(3)) // Limit agent loop to 3 steps
        .build()
        .generate_text()
        .await?;

    println!("Response: {:?}", result.text());
    Ok(())
}
```

### Prompts

The AISDK prompt feature provides, file-based template system for managing AI prompts using the Tera template engine. It allows you to create reusable prompt templates with variable substitution, conditionals, loops, and template inclusion. See [Examples](https://aisdk.rs/docs/concepts/prompt) for more template examples. Enable with `cargo add aisdk --features prompt`

### Roadmap

- [ ] Image Model Request Support
- [ ] Voice Model Request Support
- [ ] Observability & OpenTelemetry (OTel) Support

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

Licensed under the MIT License. See [LICENSE](./LICENSE) for details.
