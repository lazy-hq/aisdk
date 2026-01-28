# AISDK

[![Docs](https://img.shields.io/badge/docs-latest-blue)](https://aisdk.rs)
[![Build Status](https://github.com/lazy-hq/aisdk/actions/workflows/ci.yml/badge.svg)](https://github.com/lazy-hq/aisdk/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Issues](https://img.shields.io/github/issues/lazy-hq/aisdk)](https://github.com/lazy-hq/aisdk/issues)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/lazy-hq/aisdk/pulls)

An open-source, provider-agnostic Rust library for building AI-powered applications, inspired by the Vercel AI SDK. It provides a type-safe interface for interacting with Large Language Models (LLMs) and offers seamless support for Rust backend frameworks as well as popular UI frameworks like React, Solid, Vue, Svelte, and more.

To learn more about how to use the AI SDK, check out our [Documentation](https://aisdk.rs) and [API Reference](https://docs.rs/aisdk/latest).

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

### Tool Approval (Human-in-the-Loop)

Require user confirmation before executing sensitive tools:

```rust
use std::sync::Arc;
use aisdk::core::{LanguageModelRequest, Tool, NeedsApproval, Message};
use aisdk::providers::OpenAI;

// Tool that always requires approval
let delete_tool = Tool::builder()
    .name("delete_file")
    .description("Delete a file from the system")
    .needs_approval(NeedsApproval::Always)
    // ...
    .build()?;

// Tool with dynamic approval based on input
let payment_tool = Tool::builder()
    .name("send_payment")
    .description("Send a payment")
    .needs_approval(NeedsApproval::Dynamic(Arc::new(|input, _ctx| {
        // Require approval for payments over $1000
        input["amount"].as_f64().unwrap_or(0.0) > 1000.0
    })))
    // ...
    .build()?;
```

When a tool requires approval, the response will contain pending approval requests:

```rust
let response = request.generate_text().await?;

if response.has_pending_approvals() {
    for approval in response.pending_tool_approvals() {
        println!("Tool '{}' wants to run with input: {:?}",
            approval.tool_call.tool.name,
            approval.tool_call.input);

        // Get user confirmation, then continue with approval response
        let approved = get_user_confirmation(); // your UI logic

        // Continue the conversation with the approval decision
        let continued = LanguageModelRequest::builder()
            .model(openai)
            .messages(response.messages())
            .message(Message::tool_approval(approval.approval_id, approved))
            .build()
            .generate_text()
            .await?;
    }
}
```

### Structured Output

Define your target output format.

```rust
use serde::Deserialize;
use schemars::JsonSchema;

#[derive(JsonSchema, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: Option<String>,
}
```

Use the `schema` attribute to infer the structure of the output.

```rust
use aisdk::core::LanguageModelRequest;
use aisdk::providers::OpenAI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let user: User = LanguageModelRequest::builder()
        .model(OpenAI::gpt_5())
        .prompt("Generate a random user")
        .schema::<User>()
        .build()
        .generate_text()
        .await?
        .into_schema()?;

    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("Email: {}", user.email.unwrap_or_default());
    Ok(())
}
```

### Prompts

The AISDK prompt feature provides a powerful, file-based template system for managing AI prompts using the Tera template engine. It allows you to create reusable prompt templates with variable substitution, conditionals, loops, and template inclusion. See [Examples](https://aisdk.rs/docs/concepts/prompt) for more template examples. Enable with ```cargo add aisdk --features prompt```


### Roadmap

- [x] Agents
- [x] Tool Execution
- [x] Prompt Templating
- [x] Structured Output (JSON Schema)
- [x] Language Model Request Support (Text Generation, Streaming)
- [x] Compatible with [Vercel AI SDK UI](https://ai-sdk.dev/docs/ai-sdk-ui/overview) (React, Solid, Vue, Svelte, …)
- [ ] Embedding Model Request Support
- [ ] Image Model Request Support
- [ ] Voice Model Request Support
- [ ] Additional Providers
    - [x] Anthropic
    - [x] Amazon Bedrock
    - [x] DeepSeek
    - [x] Google
    - [x] Groq
    - [x] OpenAI
    - [x] OpenRouter
    - [x] TogetherAI
    - [x] Vercel
    - [x] xAI (Grok)
    - [ ] more to come...

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

Licensed under the MIT License. See [LICENSE](./LICENSE) for details.
