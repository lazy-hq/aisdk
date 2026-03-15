//! Language model implementation for the Anthropic provider.

use crate::core::capabilities::ModelName;
use crate::core::client::LanguageModelClient;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, ProviderStream,
};
use crate::core::messages::AssistantMessage;
use crate::core::tools::ToolDetails;
use crate::core::{LanguageModelStreamChunkType, ToolCallInfo};
use crate::extensions::Extensions;
use crate::providers::anthropic::Anthropic;
use crate::providers::anthropic::client::{
    AnthropicContentBlock, AnthropicDelta, AnthropicMessageDeltaUsage, AnthropicOptions,
    AnthropicStreamEvent,
};
use crate::providers::anthropic::extensions;
use crate::{core::language_model::LanguageModel, error::Result};
use async_trait::async_trait;
use futures::StreamExt;
use std::collections::HashMap;

#[async_trait]
impl<M: ModelName> LanguageModel for Anthropic<M> {
    /// Returns the name of the model.
    fn name(&self) -> String {
        self.options.model.clone()
    }

    /// Generates text using the Anthropic provider.
    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let additional_headers = options.headers.clone();
        let mut options: AnthropicOptions = options.into();
        options.model = self.options.model.clone();
        self.options = options;

        let response = self
            .send(self.settings.base_url.clone(), additional_headers)
            .await?;

        let mut collected: Vec<LanguageModelResponseContentType> = Vec::new();

        for out in response.content {
            match out {
                AnthropicContentBlock::Text { text, .. } => {
                    collected.push(LanguageModelResponseContentType::new(text));
                }
                AnthropicContentBlock::Thinking {
                    signature,
                    thinking,
                } => {
                    let extensions = Extensions::default();
                    extensions
                        .get_mut::<extensions::AnthropicThinkingMetadata>()
                        .signature = Some(signature);
                    collected.push(LanguageModelResponseContentType::Reasoning {
                        content: thinking,
                        extensions,
                    });
                }
                AnthropicContentBlock::RedactedThinking { data } => {
                    collected.push(LanguageModelResponseContentType::Reasoning {
                        content: data,
                        extensions: Extensions::default(),
                    });
                }
                AnthropicContentBlock::ToolUse { id, input, name } => {
                    collected.push(LanguageModelResponseContentType::ToolCall(ToolCallInfo {
                        input,
                        tool: ToolDetails {
                            id: id.to_string(),
                            name: name.to_string(),
                        },
                        extensions: Extensions::default(),
                    }));
                }
            }
        }

        Ok(LanguageModelResponse {
            contents: collected,
            usage: Some(response.usage.into()),
        })
    }

    /// Streams text using the Anthropic provider.
    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        let additional_headers = options.headers.clone();
        let mut options: AnthropicOptions = options.into();
        options.stream = Some(true);
        options.model = self.options.model.clone();
        self.options = options;

        // Retry logic for rate limiting
        let max_retries = 5;
        let mut retry_count = 0;
        let mut wait_time = std::time::Duration::from_secs(1);

        let response = loop {
            match self
                .send_and_stream(self.settings.base_url.clone(), additional_headers.clone())
                .await
            {
                Ok(stream) => break stream,
                Err(crate::error::Error::ApiError {
                    status_code: Some(status),
                    ..
                }) if status == reqwest::StatusCode::TOO_MANY_REQUESTS
                    && retry_count < max_retries =>
                {
                    retry_count += 1;
                    tokio::time::sleep(wait_time).await;
                    wait_time *= 2; // Exponential backoff
                    continue;
                }
                Err(e) => return Err(e),
            }
        };

        #[derive(Default)]
        struct StreamState {
            content_blocks: HashMap<usize, AccumulatedBlock>,
            usage: Option<AnthropicMessageDeltaUsage>,
        }

        #[derive(Debug)]
        enum AccumulatedBlock {
            Text(String),
            Thinking {
                thinking: String,
                signature: Option<String>,
            },
            RedactedThinking(String),
            ToolUse {
                id: String,
                name: String,
                accumulated_json: String,
            },
        }

        let stream = response.scan::<_, Result<Vec<LanguageModelStreamChunk>>, _, _>(
            StreamState::default(),
            |state, evt_res| {
                let unsupported =  |event: &str| {
                    vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::NotSupported(format!("AnthropicStreamEvent::{event}")),
                    )]
                };
                futures::future::ready({
                    match evt_res {
                    Ok(event) => match event {
                        AnthropicStreamEvent::MessageStart { .. } => {
                            Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                LanguageModelStreamChunkType::Start,
                            )]))
                        }
                        AnthropicStreamEvent::ContentBlockStart {
                            index,
                            content_block,
                        } => match content_block {
                            AnthropicContentBlock::Text { .. } => {
                                state
                                    .content_blocks
                                    .insert(index, AccumulatedBlock::Text(String::new()));
                                Some(Ok(unsupported("ContentBlockStart::Text")))
                            }
                            AnthropicContentBlock::Thinking { .. } => {
                                state
                                    .content_blocks
                                    .insert(index, AccumulatedBlock::Thinking {
                                        thinking: String::new(),
                                        signature: None,
                                    });
                                Some(Ok(unsupported("ContentBlockStart::Thinking")))
                            }
                            AnthropicContentBlock::RedactedThinking { data } => {
                                state.content_blocks.insert(
                                    index,
                                    AccumulatedBlock::RedactedThinking(data.clone()),
                                );
                                Some(Ok(unsupported("ContentBlockStart::RedactedThinking")))
                            }
                            AnthropicContentBlock::ToolUse { id, name, .. } => {
                                state.content_blocks.insert(
                                    index,
                                    AccumulatedBlock::ToolUse {
                                        id,
                                        name,
                                        accumulated_json: String::new(),
                                    },
                                );
                                Some(Ok(unsupported("ContentBlockStart::ToolUse")))
                            }
                        },
                        AnthropicStreamEvent::ContentBlockDelta { index, delta } => {
                            if let Some(block) = state.content_blocks.get_mut(&index) {
                                match (block, delta) {
                                    (
                                        AccumulatedBlock::Text(text),
                                        AnthropicDelta::TextDelta { text: delta_text },
                                    ) => {
                                        text.push_str(&delta_text);
                                        Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                            LanguageModelStreamChunkType::Text(delta_text),
                                        )]))
                                    }
                                    (
                                        AccumulatedBlock::Thinking { thinking, .. },
                                        AnthropicDelta::ThinkingDelta { thinking: delta_thinking },
                                    ) => {
                                        thinking.push_str(&delta_thinking);
                                        Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                            LanguageModelStreamChunkType::Text(delta_thinking),
                                        )]))
                                    }
                                    (
                                        AccumulatedBlock::Thinking { signature, .. },
                                        AnthropicDelta::SignatureDelta { signature: delta_signature },
                                    ) => {
                                        *signature = Some(delta_signature.clone());
                                        Some(Ok(unsupported("SignatureDelta")))
                                    }
                                    (
                                        AccumulatedBlock::ToolUse {
                                            accumulated_json, ..
                                        },
                                        AnthropicDelta::ToolUseDelta { partial_json },
                                    ) => {
                                        accumulated_json.push_str(&partial_json);
                                        Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                            LanguageModelStreamChunkType::ToolCall(partial_json),
                                        )]))
                                    }
                                    _ => Some(Ok(unsupported("ContentBlockDelta"))),
                                }
                            } else {
                                unreachable!("Anthropic accumulator must be initialized on AnthropicStreamEvent::ContentBlockStart")
                            }
                        }
                        AnthropicStreamEvent::ContentBlockStop { .. } => {
                            Some(Ok(unsupported("ContentBlockStop")))
                        }
                        AnthropicStreamEvent::MessageDelta { usage, .. } => {
                            state.usage = Some(usage);
                            Some(Ok(unsupported("MessageDelta")))
                        }
                        AnthropicStreamEvent::MessageStop => {
                            let mut collected = vec![];
                            for block in state.content_blocks.values() {
                                match block {
                                    AccumulatedBlock::Text(text) => collected
                                        .push(LanguageModelResponseContentType::new(text.clone())),
                                    AccumulatedBlock::Thinking { thinking, signature } => {
                                        let extensions = Extensions::default();
                                        if let Some(sig) = signature {
                                            extensions
                                                .get_mut::<extensions::AnthropicThinkingMetadata>()
                                                .signature = Some(sig.clone());
                                        }
                                        collected.push(LanguageModelResponseContentType::Reasoning {
                                            content: thinking.clone(),
                                            extensions,
                                        })
                                    }
                                    AccumulatedBlock::RedactedThinking(data) => collected.push(
                                        LanguageModelResponseContentType::Reasoning {
                                            content: data.clone(),
                                            extensions: Extensions::default(),
                                        },
                                    ),
                                    AccumulatedBlock::ToolUse {
                                        id,
                                        name,
                                        accumulated_json,
                                    } => {
                                        let json_str = if accumulated_json.trim().is_empty() {
                                            "{}"
                                        } else {
                                            accumulated_json
                                        };
                                        if let Ok(input) = serde_json::from_str(json_str) {
                                            collected.push(
                                                LanguageModelResponseContentType::ToolCall(
                                                    ToolCallInfo {
                                                        input,
                                                        tool: ToolDetails {
                                                            id: id.clone(),
                                                            name: name.clone(),
                                                        },
                                                        extensions: Extensions::default(),
                                                    },
                                                ),
                                            );
                                        } else {
                                            collected.push(
                                                LanguageModelResponseContentType::NotSupported(
                                                    format!(
                                                        "Invalid tool json: {accumulated_json}"
                                                    ),
                                                ),
                                            );
                                        }
                                    }
                                }
                            }
                            Some(Ok(collected
                                .into_iter()
                                .map(|ref c| {
                                    LanguageModelStreamChunk::Done(AssistantMessage {
                                        content: c.clone(),
                                        usage: state.usage.clone().map(|usage| usage.into()),
                                    })
                                })
                                .collect()))
                        }
                        AnthropicStreamEvent::Error { error } => {
                            let reason = format!("{}: {}", error.type_, error.message);

                            Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                LanguageModelStreamChunkType::Failed(reason),
                            )]))
                        }
                        AnthropicStreamEvent::NotSupported(txt) => {
                            Some(Ok(vec![LanguageModelStreamChunk::Delta(
                                LanguageModelStreamChunkType::NotSupported(txt),
                            )]))
                        }
                    },
                    Err(e) => Some(Err(e)),
                }})
            },
        );

        Ok(Box::pin(stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::language_model::ReasoningEffort;
    use crate::core::tools::{Tool, ToolExecute};
    use crate::core::{DynamicModel, LanguageModelRequest, Message};
    use crate::providers::anthropic::ANTHROPIC_API_VERSION;
    use futures::StreamExt;
    use schemars::{JsonSchema, schema_for};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::collections::HashMap;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use wiremock::matchers::{body_partial_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct SumInput {
        a: i32,
        b: i32,
    }

    fn test_model(base_url: String) -> Anthropic<DynamicModel> {
        let mut model = Anthropic::<DynamicModel>::model_name("claude-sonnet-4-0");
        model.settings.base_url = base_url;
        model.settings.api_key = "test-key".to_string();
        model
    }

    fn anthropic_message_response(content: &str) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(json!({
            "id": "msg_1",
            "type": "message",
            "role": "assistant",
            "model": "claude-sonnet-4-0",
            "content": [
                {
                    "type": "text",
                    "text": content,
                    "citations": []
                }
            ],
            "stop_reason": "end_turn",
            "stop_sequences": [],
            "usage": {
                "cache_creation": {
                    "ephemeral_1h_input_tokens": 0,
                    "ephemeral_5m_input_tokens": 0
                },
                "cache_creation_input_tokens": 0,
                "cache_read_input_tokens": 0,
                "input_tokens": 1,
                "output_tokens": 1,
                "server_tool_use": {
                    "web_search_requests": 0
                },
                "service_tier": "standard"
            }
        }))
    }

    fn content_length(request: &str) -> usize {
        request
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length").then(|| {
                    value
                        .trim()
                        .parse::<usize>()
                        .expect("content-length should parse")
                })
            })
            .unwrap_or_default()
    }

    fn header_value<'a>(request: &'a str, name: &str) -> Option<&'a str> {
        request.lines().find_map(|line| {
            let (header_name, value) = line.split_once(':')?;
            header_name
                .eq_ignore_ascii_case(name)
                .then_some(value.trim())
        })
    }

    fn body_json(request: &str) -> serde_json::Value {
        let (_, body) = request
            .split_once("\r\n\r\n")
            .expect("request should contain headers and body");

        serde_json::from_str(body).expect("request body should be valid json")
    }

    fn sum_tool() -> Tool {
        Tool::builder()
            .name("sum")
            .description("Adds two numbers")
            .input_schema(schema_for!(SumInput))
            .execute(ToolExecute::new(Box::new(|_| Ok("3".to_string()))))
            .build()
            .expect("tool should build")
    }

    async fn spawn_sse_server() -> (String, tokio::task::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("listener should bind");
        let address = listener.local_addr().expect("listener should have address");

        let handle = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("server should accept");
            let mut buffer = Vec::new();
            let mut chunk = [0u8; 1024];

            loop {
                let read = socket
                    .read(&mut chunk)
                    .await
                    .expect("request should be readable");
                if read == 0 {
                    break;
                }

                buffer.extend_from_slice(&chunk[..read]);

                if let Some(headers_end) =
                    buffer.windows(4).position(|window| window == b"\r\n\r\n")
                {
                    let headers_end = headers_end + 4;
                    let request = String::from_utf8_lossy(&buffer).to_string();
                    let body_length = content_length(&request);
                    if buffer.len() >= headers_end + body_length {
                        break;
                    }
                }
            }

            let request = String::from_utf8(buffer).expect("request should be valid utf-8");
            let response_body = concat!(
                "data: {",
                "\"type\":\"content_block_start\",",
                "\"index\":0,",
                "\"content_block\":{",
                "\"type\":\"text\",",
                "\"text\":\"\",",
                "\"citations\":[]",
                "}",
                "}\n\n",
                "data: {",
                "\"type\":\"content_block_delta\",",
                "\"index\":0,",
                "\"delta\":{",
                "\"type\":\"text_delta\",",
                "\"text\":\"Hello\"",
                "}",
                "}\n\n",
                "data: {",
                "\"type\":\"message_delta\",",
                "\"delta\":{",
                "\"stop_reason\":null,",
                "\"stop_sequence\":null",
                "},",
                "\"usage\":{",
                "\"output_tokens\":1",
                "}",
                "}\n\n",
                "data: {",
                "\"type\":\"message_stop\"",
                "}\n\n"
            );
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: text/event-stream\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );

            socket
                .write_all(response.as_bytes())
                .await
                .expect("response should be writable");

            request
        });

        (format!("http://{}", address), handle)
    }

    #[tokio::test]
    async fn test_generate_text_sends_request_body_without_custom_headers() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/messages"))
            .and(header("x-api-key", "test-key"))
            .and(header("anthropic-version", ANTHROPIC_API_VERSION))
            .and(header("content-type", "application/json"))
            .and(body_partial_json(json!({
                "model": "claude-sonnet-4-0",
                "system": "You are helpful",
                "messages": [
                    {
                        "role": "user",
                        "content": "Hello"
                    }
                ],
                "thinking": {
                    "type": "enable",
                    "budget_tokens": 5000
                },
                "tools": [
                    {
                        "name": "sum",
                        "description": "Adds two numbers",
                        "input_schema": {
                            "type": "object",
                            "properties": {
                                "a": {
                                    "type": "integer",
                                    "format": "int32"
                                },
                                "b": {
                                    "type": "integer",
                                    "format": "int32"
                                }
                            }
                        }
                    }
                ]
            })))
            .respond_with(anthropic_message_response("ok"))
            .expect(1)
            .mount(&server)
            .await;

        let mut request = LanguageModelRequest::builder()
            .model(test_model(server.uri()))
            .system("You are helpful")
            .messages(vec![Message::User("Hello".to_string().into())])
            .reasoning_effort(ReasoningEffort::Medium)
            .with_tool(sum_tool())
            .build();

        let response = request
            .generate_text()
            .await
            .expect("request should succeed");

        assert_eq!(response.text().as_deref(), Some("ok"));
    }

    #[tokio::test]
    async fn test_generate_text_merges_custom_headers_into_request() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/messages"))
            .and(header("x-api-key", "test-key"))
            .and(header("anthropic-version", ANTHROPIC_API_VERSION))
            .and(header("x-trace-id", "trace-123"))
            .and(body_partial_json(json!({
                "model": "claude-sonnet-4-0",
                "messages": [
                    {
                        "role": "user",
                        "content": "Hello"
                    }
                ]
            })))
            .respond_with(anthropic_message_response("ok"))
            .expect(1)
            .mount(&server)
            .await;

        let response = LanguageModelRequest::builder()
            .model(test_model(server.uri()))
            .messages(vec![Message::User("Hello".to_string().into())])
            .headers(HashMap::from([(
                "x-trace-id".to_string(),
                "trace-123".to_string(),
            )]))
            .build()
            .generate_text()
            .await
            .expect("request should succeed");

        assert_eq!(response.text().as_deref(), Some("ok"));
    }

    #[tokio::test]
    async fn test_stream_text_sends_streaming_request_without_custom_headers() {
        let (base_url, request_handle) = spawn_sse_server().await;

        let mut request = LanguageModelRequest::builder()
            .model(test_model(base_url))
            .system("You are helpful")
            .messages(vec![Message::User("Hello".to_string().into())])
            .reasoning_effort(ReasoningEffort::Medium)
            .with_tool(sum_tool())
            .build();

        let mut stream = request
            .stream_text()
            .await
            .expect("stream request should succeed");

        let mut first_item = tokio::time::timeout(Duration::from_secs(1), stream.stream.next())
            .await
            .expect("stream should yield an event")
            .expect("stream should not end immediately");

        if matches!(
            first_item,
            crate::core::language_model::LanguageModelStreamChunkType::Start
        ) {
            first_item = tokio::time::timeout(Duration::from_secs(1), stream.stream.next())
                .await
                .expect("stream should yield an event")
                .expect("stream should not end immediately");
        }

        match first_item {
            crate::core::language_model::LanguageModelStreamChunkType::Text(text) => {
                assert!(!text.is_empty())
            }
            _ => panic!("Expected Text chunk"),
        }

        let request = request_handle
            .await
            .expect("request capture should succeed");
        assert!(request.starts_with("POST /messages HTTP/1.1"));
        assert_eq!(header_value(&request, "x-api-key"), Some("test-key"));
        assert_eq!(
            header_value(&request, "anthropic-version"),
            Some(ANTHROPIC_API_VERSION)
        );
        assert_eq!(
            header_value(&request, "content-type"),
            Some("application/json")
        );

        let body = body_json(&request);
        assert_eq!(body["model"], json!("claude-sonnet-4-0"));
        assert_eq!(body["stream"], json!(true));
        assert_eq!(body["system"], json!("You are helpful"));
        assert_eq!(body["thinking"]["type"], json!("enable"));
        assert_eq!(body["thinking"]["budget_tokens"], json!(5000));

        let messages = body["messages"]
            .as_array()
            .expect("messages should be an array");
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["role"], json!("user"));
        assert_eq!(messages[0]["content"], json!("Hello"));

        let tools = body["tools"].as_array().expect("tools should be an array");
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], json!("sum"));
        assert_eq!(tools[0]["description"], json!("Adds two numbers"));
        assert_eq!(tools[0]["input_schema"]["type"], json!("object"));
        assert!(tools[0]["input_schema"]["properties"].get("a").is_some());
        assert!(tools[0]["input_schema"]["properties"].get("b").is_some());
    }

    #[tokio::test]
    async fn test_stream_text_merges_custom_headers_into_request() {
        let (base_url, request_handle) = spawn_sse_server().await;

        let mut stream = LanguageModelRequest::builder()
            .model(test_model(base_url))
            .messages(vec![Message::User("Hello".to_string().into())])
            .headers(HashMap::from([(
                "x-trace-id".to_string(),
                "stream-123".to_string(),
            )]))
            .build()
            .stream_text()
            .await
            .expect("stream request should succeed");

        let mut first_item = tokio::time::timeout(Duration::from_secs(1), stream.stream.next())
            .await
            .expect("stream should yield an event")
            .expect("stream should not end immediately");

        if matches!(
            first_item,
            crate::core::language_model::LanguageModelStreamChunkType::Start
        ) {
            first_item = tokio::time::timeout(Duration::from_secs(1), stream.stream.next())
                .await
                .expect("stream should yield an event")
                .expect("stream should not end immediately");
        }

        match first_item {
            crate::core::language_model::LanguageModelStreamChunkType::Text(text) => {
                assert!(!text.is_empty())
            }
            _ => panic!("Expected Text chunk"),
        }

        let request = request_handle
            .await
            .expect("request capture should succeed");
        assert!(request.starts_with("POST /messages HTTP/1.1"));
        assert_eq!(header_value(&request, "x-api-key"), Some("test-key"));
        assert_eq!(header_value(&request, "x-trace-id"), Some("stream-123"));
        assert!(request.contains("\"stream\":true"));
    }
}
