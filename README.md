# AISDK

[![Build Status](https://github.com/lazy-hq/aisdk/actions/workflows/ci.yml/badge.svg)](https://github.com/lazy-hq/aisdk/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Issues](https://img.shields.io/github/issues/lazy-hq/aisdk)](https://github.com/lazy-hq/aisdk/issues)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/lazy-hq/aisdk/pulls)

An open-source Rust library for building AI-powered applications, inspired by the Vercel AI SDK. It provides a type-safe interface for interacting with Large Language Models (LLMs).

> **⚠️ Early Stage Warning**: This project is in very early development and not ready for production use. APIs may change significantly, and features are limited. Use at your own risk.

## Installation

```bash
cargo add aisdk
```

## Usage

Enable Providers such as OpenAI, Anthropic, Google and [more](https://aisdk.rs/docs#model-providers)

### Example with OpenAI provider

```bash
cargo add aisdk --features openai
```

### Basic Text Generation

```rust
use aisdk::{
    core::{LanguageModelRequest},
    providers::openai::OpenAI,
};

async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let openai = OpenAI::new("gpt-5");

    let result = LanguageModelRequest::builder()
        .model(openai)
        .prompt("What is the meaning of life?")
        .build()
        .generate_text() // or stream_text() for streaming
        .await?;

    println!("{}", result.text().unwrap());
    Ok(())
}
```

### Agents / Tools

```rust
use aisdk::{
    core::{LanguageModelRequest},
    utils::step_count_is,
    providers::openai::OpenAI,
};
use schemars::JsonSchema; // used to convert tool function to json schema

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    #[tool]
    /// Get the weather information given a location
    pub fn get_weather(location: String) {
        let weather = match location.as_str() {
            "New York" => 75,
            "Tokyo" => 80,
            _ => 70,
        };
        Ok(weather.to_string())
    }

    let result = LanguageModelRequest::builder()
        .model(OpenAI::new("gpt-4o"))
        .system("You are a helpful assistant with access to tools. Use them to answer questions accurately.")
        .prompt("What is the weather in New York?")
        .with_tool(get_weather())
        .stop_when(step_count_is(3)) // Limit agent loop to 3 steps
        .build()
        .generate_text()
        .await?;

    println!("{}", result.text().unwrap());
    Ok(())
}
```

### Prompts

The AISDK prompt feature provides a powerful, file-based template system for managing AI prompts using the Tera template engine. It allows you to create reusable prompt templates with variable substitution, conditionals, loops, and template inclusion. See [Examples](https://aisdk.rs/docs/concepts/prompt) for more template examples. Enable with ```cargo add aisdk --features prompt```


### Roadmap

- [x] Agents
- [x] Tool Execution
- [x] Prompt Templating
- [x] Stractured Output (JSON Schema)
- [x] Language Model Request Support (Text Generation, Streaming)
- [ ] Embedding Model Request Support
- [ ] Image Model Request Support
- [ ] Voice Model Request Support
- [ ] Additional Providers

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

Licensed under the MIT License. See [LICENSE](./LICENSE) for details.
