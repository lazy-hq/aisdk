//! Language model implementation for the OpenAI Chat Completions provider.

use crate::core::capabilities::ModelName;
use crate::core::client::LanguageModelClient;
use crate::core::language_model::{
    LanguageModel, LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, LanguageModelStreamChunkType, ProviderStream,
};
use crate::core::messages::AssistantMessage;
use crate::core::tools::ToolCallInfo;
use crate::error::Result;
use crate::providers::openai_chat_completions::OpenAIChatCompletions;
use crate::providers::openai_chat_completions::client::{self, types};
use async_trait::async_trait;
use futures::StreamExt;

#[async_trait]
impl<M: ModelName> LanguageModel for OpenAIChatCompletions<M> {
    fn name(&self) -> String {
        self.options.model.clone()
    }

    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let additional_headers = options.headers.clone();
        let mut options: client::ChatCompletionsOptions = options.into();
        options.model = self.options.model.clone();
        self.options = options;

        let response: types::ChatCompletionsResponse = self
            .send(&self.settings.base_url, additional_headers)
            .await?;

        // Convert choices to LanguageModelResponse
        let mut contents = Vec::new();

        for choice in response.choices {
            // Handle text content
            if let Some(text) = choice.message.content
                && !text.is_empty()
            {
                contents.push(LanguageModelResponseContentType::Text(text));
            }

            // Handle tool calls
            if let Some(tool_calls) = choice.message.tool_calls {
                for tool_call in tool_calls {
                    let mut tool_info = ToolCallInfo::new(tool_call.function.name);
                    tool_info.id(tool_call.id);
                    tool_info.input(
                        serde_json::from_str(&tool_call.function.arguments)
                            .unwrap_or_else(|_| serde_json::Value::Object(serde_json::Map::new())),
                    );
                    contents.push(LanguageModelResponseContentType::ToolCall(tool_info));
                }
            }
        }

        Ok(LanguageModelResponse {
            contents,
            usage: response.usage.map(|u| u.into()),
        })
    }

    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        let additional_headers = options.headers.clone();
        let mut options: client::ChatCompletionsOptions = options.into();
        options.model = self.options.model.clone();
        options.stream = Some(true);
        // Note: stream_options is not sent to maintain compatibility with
        // OpenAI-compatible providers that don't support this field (e.g., Z.ai)
        // TODO: There should be a correct way to override options for different
        // open ai compatible providers
        self.options = options;

        let stream = self
            .send_and_stream(&self.settings.base_url, additional_headers)
            .await?;

        // State for accumulating tool calls across chunks
        use std::collections::HashMap;
        let mut accumulated_tool_calls: HashMap<u32, (String, String, String)> = HashMap::new();

        // Map stream events to SDK stream chunks
        let stream = stream.map(move |evt_res| match evt_res {
            Ok(types::ChatCompletionsStreamEvent::Chunk(chunk)) => {
                let mut results = Vec::new();

                for choice in chunk.choices {
                    // Reasoning delta (for reasoning models like o1, DeepSeek R1)
                    if let Some(reasoning) = choice.delta.reasoning_content
                        && !reasoning.is_empty()
                    {
                        results.push(LanguageModelStreamChunk::Delta(
                            LanguageModelStreamChunkType::Reasoning(reasoning),
                        ));
                    }

                    // Text delta
                    if let Some(content) = choice.delta.content
                        && !content.is_empty()
                    {
                        results.push(LanguageModelStreamChunk::Delta(
                            LanguageModelStreamChunkType::Text(content),
                        ));
                    }

                    // Accumulate tool call deltas
                    if let Some(tool_calls) = choice.delta.tool_calls {
                        for tool_call in tool_calls {
                            let entry = accumulated_tool_calls.entry(tool_call.index).or_insert((
                                String::new(),
                                String::new(),
                                String::new(),
                            ));

                            // Accumulate ID
                            if let Some(id) = tool_call.id {
                                entry.0 = id;
                            }

                            // Accumulate name and arguments
                            if let Some(function) = tool_call.function {
                                if let Some(name) = function.name {
                                    entry.1 = name;
                                }
                                if let Some(args) = function.arguments {
                                    entry.2.push_str(&args);
                                    results.push(LanguageModelStreamChunk::Delta(
                                        LanguageModelStreamChunkType::ToolCall(args),
                                    ));
                                }
                            }
                        }
                    }

                    if let Some(finish_reason) = choice.finish_reason {
                        let usage = chunk.usage.clone().map(|u| u.into());

                        match finish_reason.as_str() {
                            "stop" | "length" => {
                                results.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: LanguageModelResponseContentType::Text(String::new()),
                                    usage,
                                }));
                            }
                            "tool_calls" | "function_call" => {
                                // Send accumulated tool calls
                                for (id, name, args) in accumulated_tool_calls.values() {
                                    let mut tool_info = ToolCallInfo::new(name.clone());
                                    tool_info.id(id.clone());
                                    tool_info.input(serde_json::from_str(args).unwrap_or_else(
                                        |_| serde_json::Value::Object(serde_json::Map::new()),
                                    ));
                                    results.push(LanguageModelStreamChunk::Done(
                                        AssistantMessage {
                                            content: LanguageModelResponseContentType::ToolCall(
                                                tool_info,
                                            ),
                                            usage: usage.clone(),
                                        },
                                    ));
                                }
                            }
                            "content_filter" => {
                                results.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: LanguageModelResponseContentType::Text(String::new()),
                                    usage,
                                }));
                                results.push(LanguageModelStreamChunk::Delta(
                                    LanguageModelStreamChunkType::Failed(
                                        "Content filtered".to_string(),
                                    ),
                                ));
                            }
                            // For any unknown finish reason, treat as normal completion
                            _ => {
                                results.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: LanguageModelResponseContentType::Text(String::new()),
                                    usage,
                                }));
                            }
                        }
                    }
                }

                Ok(results)
            }
            Ok(types::ChatCompletionsStreamEvent::Open) => Ok(vec![]),
            Ok(types::ChatCompletionsStreamEvent::Done) => Ok(vec![]),
            Ok(types::ChatCompletionsStreamEvent::Error(e)) => {
                Ok(vec![LanguageModelStreamChunk::Delta(
                    LanguageModelStreamChunkType::Failed(e),
                )])
            }
            Err(e) => Err(e),
        });

        Ok(Box::pin(stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::language_model::{LanguageModelOptions, ReasoningEffort};
    use crate::core::tools::{Tool, ToolExecute, ToolList};
    use crate::core::{DynamicModel, LanguageModelRequest, Message};
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
    struct StructuredOutput {
        answer: String,
    }

    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    struct SumInput {
        a: i32,
        b: i32,
    }

    fn test_model(base_url: String) -> OpenAIChatCompletions<DynamicModel> {
        let mut model = OpenAIChatCompletions::<DynamicModel>::model_name("gpt-4o-mini");
        model.settings.base_url = base_url;
        model.settings.api_key = "test-key".to_string();
        model
    }

    fn chat_completion_response(content: &str) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(json!({
            "id": "chatcmpl-1",
            "object": "chat.completion",
            "created": 0,
            "model": "gpt-4o-mini",
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": content,
                        "name": null,
                        "tool_calls": null,
                        "tool_call_id": null
                    },
                    "finish_reason": "stop"
                }
            ],
            "usage": {
                "prompt_tokens": 1,
                "completion_tokens": 1,
                "total_tokens": 2
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
                "\"id\":\"chatcmpl-1\",",
                "\"object\":\"chat.completion.chunk\",",
                "\"created\":0,",
                "\"model\":\"gpt-4o-mini\",",
                "\"choices\":[{",
                "\"index\":0,",
                "\"delta\":{\"content\":\"Hello\"},",
                "\"finish_reason\":null",
                "}]",
                "}\n\n",
                "data: {",
                "\"id\":\"chatcmpl-1\",",
                "\"object\":\"chat.completion.chunk\",",
                "\"created\":0,",
                "\"model\":\"gpt-4o-mini\",",
                "\"choices\":[{",
                "\"index\":0,",
                "\"delta\":{},",
                "\"finish_reason\":\"stop\"",
                "}]",
                "}\n\n",
                "data: [DONE]\n\n"
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
            .and(path("/chat/completions"))
            .and(header("authorization", "Bearer test-key"))
            .and(header("content-type", "application/json"))
            .and(body_partial_json(json!({
                "model": "gpt-4o-mini",
                "messages": [
                    { "role": "system", "content": "You are helpful" },
                    { "role": "user", "content": "Hello" }
                ],
                "seed": 42,
                "temperature": 0.7,
                "top_p": 0.8,
                "max_completion_tokens": 256,
                "stop": "END",
                "presence_penalty": 0.2,
                "frequency_penalty": 0.5,
                "reasoning_effort": "high",
                "response_format": {
                    "type": "json_schema",
                    "json_schema": {
                        "name": "StructuredOutput",
                        "strict": true,
                        "schema": {
                            "additionalProperties": false,
                            "properties": {
                                "answer": {}
                            }
                        }
                    }
                },
                "tools": [{
                    "type": "function",
                    "function": {
                        "name": "sum",
                        "description": "Adds two numbers",
                        "strict": true,
                        "parameters": {
                            "type": "object",
                            "additionalProperties": false,
                            "properties": {
                                "a": {},
                                "b": {}
                            }
                        }
                    }
                }],
                "tool_choice": "auto",
                "parallel_tool_calls": true
            })))
            .respond_with(chat_completion_response("ok"))
            .expect(1)
            .mount(&server)
            .await;

        let mut request = LanguageModelRequest::builder()
            .model(test_model(server.uri()))
            .messages(vec![Message::User("Hello".to_string().into())])
            .seed(42u32)
            .temperature(70u32)
            .top_p(80u32)
            .stop_sequences(vec!["END".to_string()])
            .frequency_penalty(0.5)
            .build();
        request.system = Some("You are helpful".to_string());
        request.schema = Some(schema_for!(StructuredOutput));
        request.tools = Some(ToolList::new(vec![sum_tool()]));
        request.reasoning_effort = Some(ReasoningEffort::High);
        request.max_output_tokens = Some(256);
        request.presence_penalty = Some(0.2);

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
            .and(path("/chat/completions"))
            .and(header("authorization", "Bearer test-key"))
            .and(header("x-trace-id", "trace-123"))
            .and(body_partial_json(json!({
                "model": "gpt-4o-mini",
                "messages": [
                    { "role": "user", "content": "Hello" }
                ],
                "seed": 42,
                "frequency_penalty": 0.5
            })))
            .respond_with(chat_completion_response("ok"))
            .expect(1)
            .mount(&server)
            .await;

        let response = LanguageModelRequest::builder()
            .model(test_model(server.uri()))
            .messages(vec![Message::User("Hello".to_string().into())])
            .seed(42u32)
            .frequency_penalty(0.5)
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

        let mut model = test_model(base_url);
        let mut stream = model
            .stream_text(LanguageModelOptions {
                system: Some("You are helpful".to_string()),
                messages: vec![Message::User("Hello".to_string().into()).into()],
                schema: Some(schema_for!(StructuredOutput)),
                seed: Some(42),
                temperature: Some(70),
                top_p: Some(80),
                max_output_tokens: Some(256),
                stop_sequences: Some(vec!["END".to_string()]),
                presence_penalty: Some(0.2),
                frequency_penalty: Some(0.5),
                reasoning_effort: Some(ReasoningEffort::High),
                tools: Some(ToolList::new(vec![sum_tool()])),
                ..Default::default()
            })
            .await
            .expect("stream request should succeed");

        let first_item = tokio::time::timeout(Duration::from_secs(1), stream.next())
            .await
            .expect("stream should yield an event")
            .expect("stream should not end immediately")
            .expect("stream event should parse");

        assert!(first_item.is_empty());

        let request = request_handle
            .await
            .expect("request capture should succeed");
        assert!(request.starts_with("POST /chat/completions HTTP/1.1"));
        assert_eq!(
            header_value(&request, "authorization"),
            Some("Bearer test-key")
        );
        assert_eq!(
            header_value(&request, "content-type"),
            Some("application/json")
        );

        let body = body_json(&request);
        assert_eq!(body["model"], json!("gpt-4o-mini"));
        assert_eq!(body["stream"], json!(true));
        assert_eq!(body["seed"], json!(42));
        assert_eq!(body["temperature"], json!(0.7));
        assert_eq!(body["top_p"], json!(0.8));
        assert_eq!(body["max_completion_tokens"], json!(256));
        assert_eq!(body["stop"], json!("END"));
        assert_eq!(body["presence_penalty"], json!(0.2));
        assert_eq!(body["frequency_penalty"], json!(0.5));
        assert_eq!(body["reasoning_effort"], json!("high"));

        let messages = body["messages"]
            .as_array()
            .expect("messages should be an array");
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0]["role"], json!("system"));
        assert_eq!(messages[0]["content"], json!("You are helpful"));
        assert_eq!(messages[1]["role"], json!("user"));
        assert_eq!(messages[1]["content"], json!("Hello"));

        assert_eq!(body["response_format"]["type"], json!("json_schema"));
        assert_eq!(
            body["response_format"]["json_schema"]["name"],
            json!("StructuredOutput")
        );
        assert_eq!(
            body["response_format"]["json_schema"]["strict"],
            json!(true)
        );
        assert_eq!(
            body["response_format"]["json_schema"]["schema"]["additionalProperties"],
            json!(false)
        );
        assert!(
            body["response_format"]["json_schema"]["schema"]["properties"]
                .get("answer")
                .is_some()
        );

        let tools = body["tools"].as_array().expect("tools should be an array");
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["type"], json!("function"));
        assert_eq!(tools[0]["function"]["name"], json!("sum"));
        assert_eq!(
            tools[0]["function"]["description"],
            json!("Adds two numbers")
        );
        assert_eq!(tools[0]["function"]["strict"], json!(true));
        assert_eq!(tools[0]["function"]["parameters"]["type"], json!("object"));
        assert_eq!(
            tools[0]["function"]["parameters"]["additionalProperties"],
            json!(false)
        );
        assert!(
            tools[0]["function"]["parameters"]["properties"]
                .get("a")
                .is_some()
        );
        assert!(
            tools[0]["function"]["parameters"]["properties"]
                .get("b")
                .is_some()
        );
        assert_eq!(body["tool_choice"], json!("auto"));
        assert_eq!(body["parallel_tool_calls"], json!(true));
    }

    #[tokio::test]
    async fn test_stream_text_merges_custom_headers_into_request() {
        let (base_url, request_handle) = spawn_sse_server().await;

        let mut model = test_model(base_url);
        let mut stream = model
            .stream_text(LanguageModelOptions {
                messages: vec![Message::User("Hello".to_string().into()).into()],
                headers: Some(HashMap::from([(
                    "x-trace-id".to_string(),
                    "stream-123".to_string(),
                )])),
                temperature: Some(60),
                top_p: Some(95),
                ..Default::default()
            })
            .await
            .expect("stream request should succeed");

        let first_item = tokio::time::timeout(Duration::from_secs(1), stream.next())
            .await
            .expect("stream should yield an event")
            .expect("stream should not end immediately")
            .expect("stream event should parse");

        assert!(first_item.is_empty());

        let request = request_handle
            .await
            .expect("request capture should succeed");
        assert!(request.starts_with("POST /chat/completions HTTP/1.1"));
        assert_eq!(
            header_value(&request, "authorization"),
            Some("Bearer test-key")
        );
        assert_eq!(header_value(&request, "x-trace-id"), Some("stream-123"));
        assert!(request.contains("\"stream\":true"));
        assert!(request.contains("\"temperature\":0.6"));
        assert!(request.contains("\"top_p\":0.95"));
    }
}
