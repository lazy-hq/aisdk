//! Language model implementation for the Google provider.
use crate::core::capabilities::ModelName;
use crate::core::client::LanguageModelClient;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, LanguageModelStreamChunkType, ProviderStream, Usage,
};
use crate::core::messages::AssistantMessage;
use crate::core::tools::ToolDetails;
use crate::providers::google::{Google, client, client::types, extensions};
use crate::{
    core::{language_model::LanguageModel, tools::ToolCallInfo},
    error::Result,
};
use async_trait::async_trait;
use futures::StreamExt;

#[async_trait]
impl<M: ModelName> LanguageModel for Google<M> {
    fn name(&self) -> String {
        self.lm_options.model.clone()
    }

    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let mut options: client::GoogleOptions = options.into();
        options.model = self.lm_options.model.clone();
        options.streaming = false;
        self.lm_options = options;

        let response: types::GenerateContentResponse = self.send(&self.settings.base_url).await?;

        let mut collected = Vec::new();
        let usage = response.usage_metadata.map(|u| u.into());

        for candidate in response.candidates {
            for part in candidate.content.parts {
                if let Some(t) = part.text {
                    collected.push(LanguageModelResponseContentType::Text(t));
                }
                if let Some(fc) = part.function_call {
                    let mut tool_info = ToolCallInfo::new(fc.name);
                    tool_info.input(fc.args);
                    if let Some(sig) = part.thought_signature {
                        tool_info
                            .extensions
                            .get_mut::<extensions::GoogleToolMetadata>()
                            .thought_signature = Some(sig);
                    }
                    collected.push(LanguageModelResponseContentType::ToolCall(tool_info));
                }
            }
        }

        Ok(LanguageModelResponse {
            contents: collected,
            usage,
        })
    }

    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        let mut options: client::GoogleOptions = options.into();
        options.model = self.lm_options.model.clone();
        options.streaming = true;
        self.lm_options = options;

        // Retry logic for rate limiting
        let max_retries = 5;
        let mut retry_count = 0;
        let mut wait_time = std::time::Duration::from_secs(1);

        let google_stream = loop {
            match self.send_and_stream(&self.settings.base_url).await {
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
            accumulated_text: String,
            accumulated_tool_call: Option<ToolCallInfo>,
            usage: Option<Usage>,
            // Block-open tracking for symmetric Start/End events
            text_open: bool,
            tool_call_open: bool,
        }

        let stream = google_stream.scan(StreamState::default(), |state, evt_res| {
            futures::future::ready(match evt_res {
                Ok(types::GoogleStreamEvent::Response(response)) => {
                    let mut chunks = Vec::new();

                    if let Some(usage) = response.usage_metadata.clone().map(Usage::from) {
                        state.usage = Some(usage);
                    }

                    for candidate in &response.candidates {
                        for part in &candidate.content.parts {
                            if let Some(t) = &part.text {
                                state.accumulated_text.push_str(t);
                                if !state.text_open {
                                    state.text_open = true;
                                    chunks.push(LanguageModelStreamChunk::Delta(
                                        LanguageModelStreamChunkType::TextStart,
                                    ));
                                }
                                chunks.push(LanguageModelStreamChunk::Delta(
                                    LanguageModelStreamChunkType::TextDelta(t.clone()),
                                ));
                            }

                            if let Some(fc) = &part.function_call {
                                let mut tool_info = ToolCallInfo::new(fc.name.clone());
                                tool_info.id(fc.name.clone()); // Google uses name as id
                                tool_info.input(fc.args.clone());

                                if let Some(sig) = &part.thought_signature {
                                    tool_info
                                        .extensions
                                        .get_mut::<extensions::GoogleToolMetadata>()
                                        .thought_signature = Some(sig.clone());
                                }

                                if !state.tool_call_open {
                                    state.tool_call_open = true;
                                    chunks.push(LanguageModelStreamChunk::Delta(
                                        LanguageModelStreamChunkType::ToolCallStart(ToolDetails {
                                            id: fc.name.clone(),
                                            name: fc.name.clone(),
                                        }),
                                    ));
                                }

                                state.accumulated_tool_call = Some(tool_info);

                                chunks.push(LanguageModelStreamChunk::Delta(
                                    LanguageModelStreamChunkType::ToolCallDelta {
                                        id: fc.name.clone(),
                                        delta: serde_json::to_string(&fc.args).unwrap_or_default(),
                                    },
                                ));
                            }
                        }

                        if let Some(ref finish_reason) = candidate.finish_reason {
                            use types::FinishReason;

                            let (content, end_chunk) =
                                if let Some(tc) = state.accumulated_tool_call.take() {
                                    state.tool_call_open = false;
                                    // Also close any text block that may have been open
                                    if state.text_open {
                                        state.text_open = false;
                                        chunks.push(LanguageModelStreamChunk::Delta(
                                            LanguageModelStreamChunkType::TextEnd,
                                        ));
                                    }

                                    (LanguageModelResponseContentType::ToolCall(tc), None)
                                } else {
                                    let text = std::mem::take(&mut state.accumulated_text);
                                    if state.text_open {
                                        state.text_open = false;
                                        chunks.push(LanguageModelStreamChunk::Delta(
                                            LanguageModelStreamChunkType::TextEnd,
                                        ));
                                    }
                                    // Map non-normal finish reasons to Failed / Incomplete
                                    let extra = match finish_reason {
                                        FinishReason::MaxTokens => {
                                            Some(LanguageModelStreamChunk::Delta(
                                                LanguageModelStreamChunkType::Incomplete(
                                                    "max_tokens".to_string(),
                                                ),
                                            ))
                                        }
                                        FinishReason::Safety
                                        | FinishReason::Recitation
                                        | FinishReason::Blocklist
                                        | FinishReason::ProhibitedContent
                                        | FinishReason::Spii
                                        | FinishReason::MalformedFunctionCall => {
                                            Some(LanguageModelStreamChunk::Delta(
                                                LanguageModelStreamChunkType::Failed(format!(
                                                    "{finish_reason:?}"
                                                )),
                                            ))
                                        }
                                        _ => None,
                                    };
                                    (LanguageModelResponseContentType::Text(text), extra)
                                };

                            if let Some(extra) = end_chunk {
                                chunks.push(extra);
                            }

                            chunks.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                content,
                                usage: state.usage.clone(),
                            }));
                        }
                    }
                    Some(Ok(chunks))
                }
                Ok(types::GoogleStreamEvent::NotSupported(msg)) => {
                    Some(Ok(vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::NotSupported(msg),
                    )]))
                }
                Err(e) => Some(Err(e)),
            })
        });

        Ok(Box::pin(stream))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

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

    fn test_model(base_url: String) -> Google<DynamicModel> {
        let mut model = Google::<DynamicModel>::model_name("gemini-1.5-flash");
        model.settings.base_url = base_url;
        model.settings.api_key = "test-key".to_string();
        model
    }

    fn generate_content_response(content: &str) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(json!({
            "candidates": [
                {
                    "content": {
                        "role": "model",
                        "parts": [
                            {
                                "text": content
                            }
                        ]
                    },
                    "finishReason": "STOP"
                }
            ],
            "usageMetadata": {
                "promptTokenCount": 1,
                "candidatesTokenCount": 1,
                "totalTokenCount": 2
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
            .execute(ToolExecute::from_sync(|_, _| Ok("3".to_string())))
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
                "\"candidates\":[{",
                "\"content\":{\"role\":\"model\",\"parts\":[{\"text\":\"Hello\"}]},",
                "\"finishReason\":null",
                "}]}\n\n",
                "data: {",
                "\"candidates\":[{",
                "\"content\":{\"role\":\"model\",\"parts\":[]},",
                "\"finishReason\":\"STOP\"",
                "}]}\n\n",
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
            .and(path("/v1beta/models/gemini-1.5-flash:generateContent"))
            .and(header("x-goog-api-key", "test-key"))
            .and(header("content-type", "application/json"))
            .and(body_partial_json(json!({
                "contents": [
                    {
                        "role": "user",
                        "parts": [
                            {
                                "text": "Hello"
                            }
                        ]
                    }
                ],
                "systemInstruction": {
                    "role": "user",
                    "parts": [
                        {
                            "text": "You are helpful"
                        }
                    ]
                },
                "generationConfig": {
                    "temperature": 0.7,
                    "topP": 0.8,
                    "stopSequences": ["END"],
                    "frequencyPenalty": 0.5,
                    "responseMimeType": "application/json",
                    "responseSchema": {
                        "type": "object",
                        "properties": {
                            "answer": {
                                "type": "string"
                            }
                        }
                    }
                },
                "tools": [{
                    "functionDeclarations": [{
                        "name": "sum",
                        "description": "Adds two numbers",
                        "parameters": {
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
                    }]
                }]
            })))
            .respond_with(generate_content_response("ok"))
            .expect(1)
            .mount(&server)
            .await;

        let response = LanguageModelRequest::builder()
            .model(test_model(server.uri()))
            .system("You are helpful")
            .messages(vec![Message::User("Hello".to_string().into())])
            .schema::<StructuredOutput>()
            .with_tool(sum_tool())
            .temperature(70u32)
            .top_p(80u32)
            .stop_sequences(vec!["END".to_string()])
            .frequency_penalty(0.5)
            .build()
            .generate_text()
            .await
            .expect("request should succeed");

        assert_eq!(response.text().as_deref(), Some("ok"));
    }

    #[tokio::test]
    async fn test_generate_text_merges_custom_headers_into_request() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1beta/models/gemini-1.5-flash:generateContent"))
            .and(header("x-goog-api-key", "test-key"))
            .and(header("x-trace-id", "trace-123"))
            .and(body_partial_json(json!({
                "contents": [
                    {
                        "role": "user",
                        "parts": [{"text": "Hello"}]
                    }
                ],
                "generationConfig": {
                    "frequencyPenalty": 0.5
                }
            })))
            .respond_with(generate_content_response("ok"))
            .expect(1)
            .mount(&server)
            .await;

        let response = LanguageModelRequest::builder()
            .model(test_model(server.uri()))
            .messages(vec![Message::User("Hello".to_string().into())])
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
    async fn test_generate_text_merges_provider_and_request_body_overrides() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1beta/models/gemini-1.5-flash:generateContent"))
            .and(header("x-goog-api-key", "test-key"))
            .and(body_partial_json(json!({
                "contents": [
                    {
                        "role": "user",
                        "parts": [{"text": "Hello"}]
                    }
                ],
                "systemInstruction": {
                    "role": "user",
                    "parts": [{"text": "Use override"}]
                },
                "cachedContent": "cachedContents/test-cache"
            })))
            .respond_with(generate_content_response("ok"))
            .expect(1)
            .mount(&server)
            .await;

        let mut model = test_model(server.uri());
        model.settings.body = Some(
            json!({
                "cachedContent": "cachedContents/test-cache"
            })
            .as_object()
            .expect("provider body should be an object")
            .clone(),
        );

        let response = LanguageModelRequest::builder()
            .model(model)
            .system("You are helpful")
            .messages(vec![Message::User("Hello".to_string().into())])
            .body(json!({
                "systemInstruction": {
                    "role": "user",
                    "parts": [{"text": "Use override"}]
                }
            }))
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
            .schema::<StructuredOutput>()
            .with_tool(sum_tool())
            .temperature(70u32)
            .top_p(80u32)
            .stop_sequences(vec!["END".to_string()])
            .frequency_penalty(0.5)
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
            crate::core::language_model::LanguageModelStreamChunkType::TextStart
        ) {
            first_item = tokio::time::timeout(Duration::from_secs(1), stream.stream.next())
                .await
                .expect("stream should yield an event")
                .expect("stream should not end immediately");
        }

        match first_item {
            crate::core::language_model::LanguageModelStreamChunkType::TextDelta(text) => {
                assert!(!text.is_empty())
            }
            _ => panic!("Expected TextDelta chunk"),
        } // Hello will be returned in chunk

        let request = request_handle
            .await
            .expect("request capture should succeed");
        assert!(request.starts_with(
            "POST /v1beta/models/gemini-1.5-flash:streamGenerateContent?alt=sse HTTP/1.1"
        ));
        assert_eq!(header_value(&request, "x-goog-api-key"), Some("test-key"));
        assert_eq!(
            header_value(&request, "content-type"),
            Some("application/json")
        );

        let body = body_json(&request);
        let contents = body["contents"].as_array().unwrap();
        assert_eq!(contents.len(), 1);
        assert_eq!(contents[0]["role"], json!("user"));
        assert_eq!(contents[0]["parts"][0]["text"], json!("Hello"));

        assert_eq!(body["systemInstruction"]["role"], json!("user"));
        assert_eq!(
            body["systemInstruction"]["parts"][0]["text"],
            json!("You are helpful")
        );

        let gen_config = &body["generationConfig"];
        assert_eq!(gen_config["temperature"], json!(0.7));
        assert_eq!(gen_config["topP"], json!(0.8));
        assert_eq!(gen_config["stopSequences"][0], json!("END"));
        assert_eq!(gen_config["frequencyPenalty"], json!(0.5));

        assert_eq!(gen_config["responseMimeType"], json!("application/json"));
        assert_eq!(gen_config["responseSchema"]["type"], json!("object"));
        assert_eq!(
            gen_config["responseSchema"]["properties"]["answer"]["type"],
            json!("string")
        );

        let tools = body["tools"].as_array().expect("tools should be an array");
        assert_eq!(tools.len(), 1);
        let func_decl = &tools[0]["functionDeclarations"][0];
        assert_eq!(func_decl["name"], json!("sum"));
        assert_eq!(func_decl["description"], json!("Adds two numbers"));
        assert_eq!(func_decl["parameters"]["type"], json!("object"));
        assert!(func_decl["parameters"]["properties"].get("a").is_some());
        assert!(func_decl["parameters"]["properties"].get("b").is_some());
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
            crate::core::language_model::LanguageModelStreamChunkType::TextStart
        ) {
            first_item = tokio::time::timeout(Duration::from_secs(1), stream.stream.next())
                .await
                .expect("stream should yield an event")
                .expect("stream should not end immediately");
        }

        match first_item {
            crate::core::language_model::LanguageModelStreamChunkType::TextDelta(text) => {
                assert!(!text.is_empty())
            }
            _ => panic!("Expected TextDelta chunk"),
        }

        let request = request_handle
            .await
            .expect("request capture should succeed");
        assert!(request.starts_with(
            "POST /v1beta/models/gemini-1.5-flash:streamGenerateContent?alt=sse HTTP/1.1"
        ));
        assert_eq!(header_value(&request, "x-goog-api-key"), Some("test-key"));
        assert_eq!(header_value(&request, "x-trace-id"), Some("stream-123"));
        assert!(request.contains("\"temperature\":0.6"));
        assert!(request.contains("\"topP\":0.95"));
    }
}
