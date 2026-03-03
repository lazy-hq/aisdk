use aisdk::core::Tool;
#[cfg(any(feature = "openai", feature = "anthropic", feature = "google"))]
use aisdk::core::{LanguageModelRequest, LanguageModelStreamChunkType};
#[cfg(any(feature = "openai", feature = "anthropic", feature = "google"))]
use aisdk::integrations::vercel_aisdk_ui::VercelUIStreamOptions;
use aisdk::macros::tool;
#[cfg(feature = "anthropic")]
use aisdk::providers::anthropic::Anthropic;
#[cfg(feature = "google")]
use aisdk::providers::google::Google;
#[cfg(feature = "openai")]
use aisdk::providers::openai::OpenAI;
#[cfg(any(feature = "openai", feature = "anthropic", feature = "google"))]
use futures::StreamExt;

#[tool]
fn get_username(user_id: String) -> Tool {
    match user_id.as_str() {
        "123" => Ok("sura".to_string()),
        _ => Ok("invalid".to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let provider = args.get(1).map(String::as_str).unwrap_or("openai");
    #[cfg(any(feature = "openai", feature = "anthropic", feature = "google"))]
    let mode = args.get(2).map(String::as_str).unwrap_or("both");

    match provider {
        "openai" => {
            #[cfg(feature = "openai")]
            {
                return run_openai(mode).await;
            }
            #[cfg(not(feature = "openai"))]
            {
                return Err("openai feature is not enabled".into());
            }
        }
        "anthropic" => {
            #[cfg(feature = "anthropic")]
            {
                return run_anthropic(mode).await;
            }
            #[cfg(not(feature = "anthropic"))]
            {
                return Err("anthropic feature is not enabled".into());
            }
        }
        "google" => {
            #[cfg(feature = "google")]
            {
                return run_google(mode).await;
            }
            #[cfg(not(feature = "google"))]
            {
                return Err("google feature is not enabled".into());
            }
        }
        _ => Err("provider must be one of: openai | anthropic | google".into()),
    }
}

#[cfg(feature = "openai")]
async fn run_openai(mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    if mode == "core" || mode == "both" {
        println!("=== OPENAI core stream ===");
        let response = LanguageModelRequest::builder()
            .model(OpenAI::gpt_5_nano())
            .system("You must call tool get_username with user_id = '123'. After tool result, respond with only the username.")
            .prompt("What is the username for user id 123?")
            .with_tool(get_username())
            .build()
            .stream_text()
            .await?;

        let mut stream = response.stream;
        while let Some(chunk) = stream.next().await {
            match chunk {
                LanguageModelStreamChunkType::ToolCallDelta {
                    tool_call_id,
                    tool_name,
                    delta,
                } => {
                    println!("ToolCallDelta id={tool_call_id} name={tool_name} delta={delta}");
                }
                LanguageModelStreamChunkType::ToolCallStart(info) => {
                    println!(
                        "ToolCallStart id={} name={} input={}",
                        info.tool.id, info.tool.name, info.input
                    );
                }
                LanguageModelStreamChunkType::ToolResult(info) => {
                    println!(
                        "ToolResult id={} name={} output={:?}",
                        info.tool.id, info.tool.name, info.output
                    );
                }
                other => println!("{other:?}"),
            }
        }
    }

    if mode == "vercel" || mode == "both" {
        println!("=== OPENAI vercel ui stream ===");
        let response = LanguageModelRequest::builder()
            .model(OpenAI::gpt_5_nano())
            .system("You must call tool get_username with user_id = '123'. After tool result, respond with only the username.")
            .prompt("What is the username for user id 123?")
            .with_tool(get_username())
            .build()
            .stream_text()
            .await?;

        let mut ui = response.into_vercel_ui_stream(VercelUIStreamOptions {
            send_reasoning: true,
            send_start: true,
            send_finish: true,
            ..Default::default()
        });

        while let Some(item) = ui.next().await {
            println!("{:?}", item?);
        }
    }

    Ok(())
}

#[cfg(feature = "anthropic")]
async fn run_anthropic(mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    if mode == "core" || mode == "both" {
        println!("=== ANTHROPIC core stream ===");
        let response = LanguageModelRequest::builder()
            .model(Anthropic::claude_haiku_4_5())
            .system("You must call tool get_username with user_id = '123'. After tool result, respond with only the username.")
            .prompt("What is the username for user id 123?")
            .with_tool(get_username())
            .build()
            .stream_text()
            .await?;

        let mut stream = response.stream;
        while let Some(chunk) = stream.next().await {
            match chunk {
                LanguageModelStreamChunkType::ToolCallDelta {
                    tool_call_id,
                    tool_name,
                    delta,
                } => {
                    println!("ToolCallDelta id={tool_call_id} name={tool_name} delta={delta}");
                }
                LanguageModelStreamChunkType::ToolCallStart(info) => {
                    println!(
                        "ToolCallStart id={} name={} input={}",
                        info.tool.id, info.tool.name, info.input
                    );
                }
                LanguageModelStreamChunkType::ToolResult(info) => {
                    println!(
                        "ToolResult id={} name={} output={:?}",
                        info.tool.id, info.tool.name, info.output
                    );
                }
                other => println!("{other:?}"),
            }
        }
    }

    if mode == "vercel" || mode == "both" {
        println!("=== ANTHROPIC vercel ui stream ===");
        let response = LanguageModelRequest::builder()
            .model(Anthropic::claude_haiku_4_5())
            .system("You must call tool get_username with user_id = '123'. After tool result, respond with only the username.")
            .prompt("What is the username for user id 123?")
            .with_tool(get_username())
            .build()
            .stream_text()
            .await?;

        let mut ui = response.into_vercel_ui_stream(VercelUIStreamOptions {
            send_reasoning: true,
            send_start: true,
            send_finish: true,
            ..Default::default()
        });

        while let Some(item) = ui.next().await {
            println!("{:?}", item?);
        }
    }

    Ok(())
}

#[cfg(feature = "google")]
async fn run_google(mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    if mode == "core" || mode == "both" {
        println!("=== GOOGLE core stream ===");
        let response = LanguageModelRequest::builder()
            .model(Google::gemini_3_flash_preview())
            .system("You must call tool get_username with user_id = '123'. After tool result, respond with only the username.")
            .prompt("What is the username for user id 123?")
            .with_tool(get_username())
            .build()
            .stream_text()
            .await?;

        let mut stream = response.stream;
        while let Some(chunk) = stream.next().await {
            match chunk {
                LanguageModelStreamChunkType::ToolCallDelta {
                    tool_call_id,
                    tool_name,
                    delta,
                } => {
                    println!("ToolCallDelta id={tool_call_id} name={tool_name} delta={delta}");
                }
                LanguageModelStreamChunkType::ToolCallStart(info) => {
                    println!(
                        "ToolCallStart id={} name={} input={}",
                        info.tool.id, info.tool.name, info.input
                    );
                }
                LanguageModelStreamChunkType::ToolResult(info) => {
                    println!(
                        "ToolResult id={} name={} output={:?}",
                        info.tool.id, info.tool.name, info.output
                    );
                }
                other => println!("{other:?}"),
            }
        }
    }

    if mode == "vercel" || mode == "both" {
        println!("=== GOOGLE vercel ui stream ===");
        let response = LanguageModelRequest::builder()
            .model(Google::gemini_3_flash_preview())
            .system("You must call tool get_username with user_id = '123'. After tool result, respond with only the username.")
            .prompt("What is the username for user id 123?")
            .with_tool(get_username())
            .build()
            .stream_text()
            .await?;

        let mut ui = response.into_vercel_ui_stream(VercelUIStreamOptions {
            send_reasoning: true,
            send_start: true,
            send_finish: true,
            ..Default::default()
        });

        while let Some(item) = ui.next().await {
            println!("{:?}", item?);
        }
    }

    Ok(())
}
