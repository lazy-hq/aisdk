//! Language model implementation for the OpenAI provider.

use crate::core::capabilities::ModelName;
use crate::core::client::LanguageModelClient;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, LanguageModelStreamChunkType, ProviderStream, Usage,
};
use crate::core::messages::AssistantMessage;
use crate::providers::openai::client::{OpenAILanguageModelOptions, types};
use crate::providers::openai::{OpenAI, client};
use crate::{
    core::{language_model::LanguageModel, tools::ToolCallInfo},
    error::Result,
};
use async_trait::async_trait;
use futures::StreamExt;

#[async_trait]
impl<M: ModelName> LanguageModel for OpenAI<M> {
    /// Returns the name of the model.
    fn name(&self) -> String {
        self.lm_options.model.clone()
    }

    /// Generates text using the OpenAI provider.
    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let additional_headers = options.headers.clone();
        let mut options: OpenAILanguageModelOptions = options.into();

        options.model = self.lm_options.model.clone();

        self.lm_options = options;

        let response: client::OpenAIResponse = self
            .send(&self.settings.base_url, additional_headers)
            .await?;

        let mut collected: Vec<LanguageModelResponseContentType> = Vec::new();

        for out in response.output.unwrap_or_default() {
            match out {
                types::MessageItem::OutputMessage { content, .. } => {
                    for c in content {
                        if let types::OutputContent::OutputText { text, .. } = c {
                            collected.push(LanguageModelResponseContentType::new(text))
                        }
                    }
                }
                types::MessageItem::FunctionCall {
                    arguments,
                    name,
                    call_id,
                    ..
                } => {
                    let mut tool_info = ToolCallInfo::new(name);
                    tool_info.id(call_id);
                    tool_info.input(serde_json::from_str(&arguments).unwrap_or_default());
                    collected.push(LanguageModelResponseContentType::ToolCall(tool_info));
                }
                _ => (),
            }
        }

        Ok(LanguageModelResponse {
            contents: collected,
            usage: response.usage.map(|usage| usage.into()),
        })
    }

    /// Streams text using the OpenAI provider.
    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        let additional_headers = options.headers.clone();
        let mut options: OpenAILanguageModelOptions = options.into();

        options.model = self.lm_options.model.to_string();
        options.stream = Some(true);

        self.lm_options = options;

        // Retry logic for rate limiting
        let max_retries = 5;
        let mut retry_count = 0;
        let mut wait_time = std::time::Duration::from_secs(1);

        let openai_stream = loop {
            match self
                .send_and_stream(&self.settings.base_url, additional_headers.clone())
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

        let stream = openai_stream.map(|evt_res| match evt_res {
            Ok(client::OpenAiStreamEvent::ResponseOutputTextDelta { delta, .. }) => {
                Ok(vec![LanguageModelStreamChunk::Delta(
                    LanguageModelStreamChunkType::Text(delta),
                )])
            }
            Ok(client::OpenAiStreamEvent::ResponseReasoningSummaryTextDelta { delta, .. }) => {
                Ok(vec![LanguageModelStreamChunk::Delta(
                    LanguageModelStreamChunkType::Reasoning(delta),
                )])
            }
            Ok(client::OpenAiStreamEvent::ResponseCompleted { response, .. }) => {
                let mut result: Vec<LanguageModelStreamChunk> = Vec::new();

                let usage: Usage = response.usage.unwrap_or_default().into();
                let output = response.output.unwrap_or_default();

                for msg in output {
                    match &msg {
                        // ---- Final OutputMessage ----
                        types::MessageItem::OutputMessage { content, .. } => {
                            if let Some(types::OutputContent::OutputText { text, .. }) =
                                content.first()
                            {
                                result.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: LanguageModelResponseContentType::new(text.clone()),
                                    usage: Some(usage.clone()),
                                }));
                            }
                        }

                        // ---- Reasoning ----
                        types::MessageItem::Reasoning { summary, .. } => {
                            if let Some(types::ReasoningSummary { text, .. }) = summary.first() {
                                result.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: LanguageModelResponseContentType::Reasoning {
                                        content: text.to_owned(),
                                        extensions: crate::extensions::Extensions::default(),
                                    },
                                    usage: Some(usage.clone()),
                                }));
                            }
                        }

                        // ---- FunctionCall ----
                        types::MessageItem::FunctionCall {
                            call_id,
                            name,
                            arguments,
                            ..
                        } => {
                            let mut tool_info = ToolCallInfo::new(name.clone());
                            tool_info.id(call_id.clone());
                            tool_info.input(serde_json::from_str(arguments).unwrap_or_default());

                            result.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                content: LanguageModelResponseContentType::ToolCall(tool_info),
                                usage: Some(usage.clone()),
                            }));
                        }

                        _ => {}
                    }
                }

                Ok(result)
            }
            Ok(client::OpenAiStreamEvent::ResponseIncomplete { response, .. }) => {
                Ok(vec![LanguageModelStreamChunk::Delta(
                    LanguageModelStreamChunkType::Incomplete(
                        response
                            .incomplete_details
                            .map(|d| d.reason)
                            .unwrap_or("Unknown".to_string()),
                    ),
                )])
            }
            Ok(client::OpenAiStreamEvent::ResponseError { code, message, .. }) => {
                let reason = format!("{}: {}", code.unwrap_or("unknown".to_string()), message);
                Ok(vec![LanguageModelStreamChunk::Delta(
                    LanguageModelStreamChunkType::Failed(reason),
                )])
            }
            Ok(evt) => Ok(vec![LanguageModelStreamChunk::Delta(
                LanguageModelStreamChunkType::NotSupported(format!("{evt:?}")),
            )]),
            Err(e) => Err(e),
        });

        Ok(Box::pin(stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::language_model::ReasoningEffort;
    use crate::core::tools::{Tool, ToolExecute};
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

    fn test_model(base_url: String) -> OpenAI<DynamicModel> {
        let mut model = OpenAI::<DynamicModel>::model_name("gpt-4o-mini");
        model.settings.base_url = base_url;
        model.settings.api_key = "test-key".to_string();
        model
    }

    fn responses_api_response(content: &str) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(json!({
            "id": "resp_1",
            "model": "gpt-4o-mini",
            "output": [
                {
                    "type": "message",
                    "id": "msg_1",
                    "status": "completed",
                    "role": "assistant",
                    "content": [
                        {
                            "type": "output_text",
                            "text": content,
                            "annotations": [],
                            "logprobs": []
                        }
                    ]
                }
            ]
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
                "\"type\":\"response.output_text.delta\",",
                "\"sequence_number\":1,",
                "\"item_id\":\"item_1\",",
                "\"output_index\":0,",
                "\"content_index\":0,",
                "\"delta\":\"Hello\"",
                "}\n\n",
                "data: {",
                "\"type\":\"response.completed\",",
                "\"sequence_number\":2,",
                "\"response\":{",
                "\"id\":\"resp_stream_1\",",
                "\"model\":\"gpt-4o-mini\",",
                "\"output\":[{",
                "\"type\":\"message\",",
                "\"id\":\"msg_stream_1\",",
                "\"status\":\"completed\",",
                "\"role\":\"assistant\",",
                "\"content\":[{",
                "\"type\":\"output_text\",",
                "\"text\":\"Hello\",",
                "\"annotations\":[],",
                "\"logprobs\":[]",
                "}]",
                "}]",
                "}",
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
            .and(path("/v1/responses"))
            .and(header("authorization", "Bearer test-key"))
            .and(header("content-type", "application/json"))
            .and(body_partial_json(json!({
                "model": "gpt-4o-mini",
                "input": [
                    {
                        "role": "system",
                        "type": "message",
                        "content": [
                            {
                                "type": "input_text",
                                "text": "You are helpful"
                            }
                        ]
                    },
                    {
                        "role": "user",
                        "type": "message",
                        "content": [
                            {
                                "type": "input_text",
                                "text": "Hello"
                            }
                        ]
                    }
                ],
                "text": {
                    "format": {
                        "type": "json_schema",
                        "name": "StructuredOutput",
                        "schema": {
                            "type": "object",
                            "properties": {
                                "answer": {
                                    "type": "string"
                                }
                            }
                        }
                    }
                },
                "reasoning": {
                    "effort": "high",
                    "summary": "auto"
                },
                "temperature": 0.7,
                "top_p": 0.8,
                "stream": false,
                "tools": [
                    {
                        "type": "function",
                        "name": "sum",
                        "description": "Adds two numbers",
                        "strict": true,
                        "parameters": {
                            "type": "object",
                            "additionalProperties": false,
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
            .respond_with(responses_api_response("ok"))
            .expect(1)
            .mount(&server)
            .await;

        let mut request = LanguageModelRequest::builder()
            .model(test_model(server.uri()))
            .messages(vec![
                Message::System("You are helpful".to_string().into()),
                Message::User("Hello".to_string().into()),
            ])
            .schema::<StructuredOutput>()
            .with_tool(sum_tool())
            .temperature(70u32)
            .top_p(80u32)
            .reasoning_effort(ReasoningEffort::High)
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
            .and(path("/v1/responses"))
            .and(header("authorization", "Bearer test-key"))
            .and(header("x-trace-id", "trace-123"))
            .and(body_partial_json(json!({
                "model": "gpt-4o-mini",
                "input": [
                    {
                        "role": "user",
                        "type": "message",
                        "content": [
                            {
                                "type": "input_text",
                                "text": "Hello"
                            }
                        ]
                    }
                ],
                "temperature": 0.6
            })))
            .respond_with(responses_api_response("ok"))
            .expect(1)
            .mount(&server)
            .await;

        let response = LanguageModelRequest::builder()
            .model(test_model(server.uri()))
            .messages(vec![Message::User("Hello".to_string().into())])
            .temperature(60u32)
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
            .messages(vec![
                Message::System("You are helpful".to_string().into()),
                Message::User("Hello".to_string().into()),
            ])
            .schema::<StructuredOutput>()
            .with_tool(sum_tool())
            .temperature(70u32)
            .top_p(80u32)
            .reasoning_effort(ReasoningEffort::High)
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
        assert!(request.starts_with("POST /v1/responses HTTP/1.1"));
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
        assert_eq!(body["temperature"], json!(0.7));
        assert_eq!(body["top_p"], json!(0.8));
        assert_eq!(body["reasoning"]["effort"], json!("high"));
        assert_eq!(body["reasoning"]["summary"], json!("auto"));

        let input = body["input"].as_array().expect("input should be an array");
        assert_eq!(input.len(), 2);
        assert_eq!(input[0]["role"], json!("system"));
        assert_eq!(input[0]["type"], json!("message"));
        assert_eq!(input[0]["content"][0]["type"], json!("input_text"));
        assert_eq!(input[0]["content"][0]["text"], json!("You are helpful"));
        assert_eq!(input[1]["role"], json!("user"));
        assert_eq!(input[1]["content"][0]["text"], json!("Hello"));

        assert_eq!(body["text"]["format"]["type"], json!("json_schema"));
        assert_eq!(body["text"]["format"]["name"], json!("StructuredOutput"));
        assert_eq!(body["text"]["format"]["strict"], json!(false));
        assert_eq!(body["text"]["format"]["schema"]["type"], json!("object"));
        assert_eq!(
            body["text"]["format"]["schema"]["properties"]["answer"]["type"],
            json!("string")
        );

        let tools = body["tools"].as_array().expect("tools should be an array");
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["type"], json!("function"));
        assert_eq!(tools[0]["name"], json!("sum"));
        assert_eq!(tools[0]["description"], json!("Adds two numbers"));
        assert_eq!(tools[0]["strict"], json!(true));
        assert_eq!(tools[0]["parameters"]["type"], json!("object"));
        assert_eq!(tools[0]["parameters"]["additionalProperties"], json!(false));
        assert!(tools[0]["parameters"]["properties"].get("a").is_some());
        assert!(tools[0]["parameters"]["properties"].get("b").is_some());
    }

    #[tokio::test]
    async fn test_stream_text_merges_custom_headers_into_request() {
        let (base_url, request_handle) = spawn_sse_server().await;

        let mut stream = LanguageModelRequest::builder()
            .model(test_model(base_url))
            .messages(vec![Message::User("Hello".to_string().into())])
            .temperature(60u32)
            .top_p(95u32)
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
        assert!(request.starts_with("POST /v1/responses HTTP/1.1"));
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
