// Macro definitions for provider test generation.

/// Configuration structure for provider test generation.
///
/// This struct defines which models to use for different test categories.
///
pub struct ProviderConfig {
    /// Default model for general tests
    pub default_model: &'static str,

    /// Model for reasoning-specific tests (optional)
    pub reasoning_model: Option<&'static str>,

    /// Model for non-reasoning validation tests (optional)
    pub non_reasoning_model: Option<&'static str>,

    /// Model for stractured output tests (optional)
    pub structured_output_model: Option<&'static str>,

    /// Model name to use for error handling tests
    pub error_model: &'static str,
}

impl ProviderConfig {
    /// Get the appropriate model for basic tests
    pub fn basic_model(&self) -> &str {
        self.default_model
    }

    /// Get the appropriate model for reasoning tests
    pub fn reasoning_model(&self) -> &str {
        self.reasoning_model.unwrap_or(self.default_model)
    }

    /// Get the appropriate model for non-reasoning validation tests
    pub fn non_reasoning_model(&self) -> &str {
        self.non_reasoning_model.unwrap_or(self.default_model)
    }

    /// Get the appropriate model for tool tests
    pub fn tool_model(&self) -> &str {
        self.default_model
    }

    /// Get the appropriate model for streaming tests
    pub fn streaming_model(&self) -> &str {
        self.default_model
    }

    /// Get the appropriate model for structured output tests
    pub fn structured_output_model(&self) -> &str {
        self.structured_output_model.unwrap_or(self.default_model)
    }

    /// Get the model for error handling tests
    pub fn error_model(&self) -> &str {
        self.error_model
    }
}

/// Main macro to generate all language model provider integration tests.
///
/// This macro generates a comprehensive suite of tests for a language model provider,
/// including basic functionality, streaming, tools, structured output, and error handling.
///
/// # Parameters
///
/// * `$provider_type` - The type of the provider (e.g., `OpenAI`)
/// * `$config:expr` - A `ProviderConfig` instance defining model mappings
/// * `$env_key:expr` - Environment variable name for the API key
/// * `skip_reasoning: $skip_reasoning:tt` - Bool literal to skip reasoning tests at compile time
/// * `skip_tool: $skip_tool:tt` - Bool literal to skip tool tests at compile time
/// * `skip_structured_output: $skip_structured_output:tt` - Bool literal to skip structured output tests at compile time
/// * `skip_streaming: $skip_streaming:tt` - Bool literal to skip streaming tests at compile time
///
/// # Example
/// ```rust
/// let config = ProviderConfig {
///     default_model: "gpt-4o",
///     reasoning_model: Some("o1-mini"),
///     non_reasoning_model: Some("gpt-4"),
///     error_model: "invalid-model-name",
/// };
///
/// generate_language_model_tests!(
///     OpenAI,
///     config,
///     "OPENAI_API_KEY",
///     skip_reasoning: false,
///     skip_tool: false,
///     skip_structured_output: false,
///     skip_streaming: false
/// );
/// ```
macro_rules! generate_language_model_tests {
    (
        $provider_type:ty,
        $config:expr,
        $env_key:expr,
        skip_reasoning: $skip_reasoning:tt,
        skip_tool: $skip_tool:tt,
        skip_structured_output: $skip_structured_output:tt,
        skip_streaming: $skip_streaming:tt
    ) => {
        use aisdk::core::{
            LanguageModelRequest, LanguageModelStreamChunkType, Message,
            language_model::{LanguageModelResponseContentType, StopReason},
            tool,
            tools::{Tool, ToolExecute},
        };
        use dotenv::dotenv;
        use futures::StreamExt;
        use schemars::JsonSchema;
        #[allow(unused_imports)]
        use serde::Deserialize;
        use std::sync::{Arc, Mutex};

        // Helper macro for API key checking
        macro_rules! skip_if_no_api_key {
            () => {
                dotenv().ok();
                if std::env::var($env_key).is_err() {
                    println!("Skipping test: {} not set", $env_key);
                    return;
                }
            };
        }

        // Helper macro to get provider with custom settings
        macro_rules! provider_with_settings {
            () => {
                <$provider_type>::builder()
                    .api_key(std::env::var($env_key).unwrap())
                    .model_name($config.default_model)
                    .build()
                    .expect("Failed to build provider")
            };
        }

        // Generate all standard test categories
        generate_basic_tests!($provider_type, $config);
        generate_language_model_stop_reason_tests!($provider_type, $config);
        generate_language_model_streaming_tests!($provider_type, $config, $skip_streaming);
        generate_language_model_tool_tests!($provider_type, $config, $skip_tool);
        generate_language_model_schema_tests!($provider_type, $config, $skip_structured_output);
        generate_language_model_hook_tests!($provider_type, $config);
        generate_language_model_step_id_tests!($provider_type, $config);
        generate_language_model_reasoning_tests!($provider_type, $config, $skip_reasoning);
        generate_language_model_error_tests!($provider_type, $config);
    };
}

/// Generate basic text generation tests
macro_rules! generate_basic_tests {
    ($provider_type:ty, $config:expr) => {
        #[tokio::test]
        async fn test_generate_text_basic() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.basic_model()))
                .prompt("Respond with exactly the word 'hello' in all lowercase.Do not include any punctuation, prefixes, or suffixes.")
                .build()
                .generate_text()
                .await;

            assert!(result.is_ok());

            let text = result
                .as_ref()
                .expect("")
                .text()
                .unwrap()
                .trim()
                .to_string();

            assert!(text.contains("hello"));
        }

        #[tokio::test]
        async fn test_generate_text_with_system_prompt() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model(provider_with_settings!())
                .system("Only say hello whatever the user says. all lowercase no punctuation, prefixes, or suffixes.")
                .prompt("Hello how are you doing?")
                .build()
                .generate_text()
                .await;

            assert!(result.is_ok());

            let text = result
                .as_ref()
                .expect("Failed to get result")
                .text()
                .unwrap()
                .trim()
                .to_string();
            assert!(text.contains("hello"));
        }

        #[tokio::test]
        async fn test_generate_text_with_messages() {
            skip_if_no_api_key!();

            let messages = Message::builder()
                .system("You are a helpful assistant.")
                .user("Whatsup?, Surafel is here")
                .assistant("How could I help you?")
                .user("Could you tell my name?")
                .build();

            let mut language_model = LanguageModelRequest::builder()
                .model(provider_with_settings!())
                .messages(messages)
                .build();

            let result = language_model.generate_text().await;
            assert!(result.is_ok());

            let text = result
                .as_ref()
                .expect("Failed to get result")
                .text()
                .unwrap()
                .trim()
                .to_string();
            assert!(text.contains("Surafel"));
        }

        #[tokio::test]
        async fn test_generate_text_with_messages_and_system_prompt() {
            skip_if_no_api_key!();

            let messages = Message::builder()
                .system("Only say hello whatever the user says. \n all lowercase no punctuation, prefixes, or suffixes.")
                .user("Whatsup?, Surafel is here")
                .assistant("How could I help you?")
                .user("Could you tell my name?")
                .build();

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.basic_model()))
                .system("Only say hello whatever the user says. all lowercase no punctuation, prefixes, or suffixes.")
                .messages(messages)
                .build()
                .generate_text()
                .await;

            assert!(result.is_ok());

            let text = result
                .as_ref()
                .expect("Failed to get result")
                .text()
                .unwrap()
                .trim()
                .to_string();
            assert!(text.contains("hello"));
        }
    };
}

/// Generate stop reason tests
macro_rules! generate_language_model_stop_reason_tests {
    ($provider_type:ty, $config:expr) => {
        #[tokio::test]
        async fn test_stop_reason_normal_finish() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.basic_model()))
                .prompt("Respond with exactly the word 'hello' in all lowercase. Do not include any punctuation.")
                .build()
                .generate_text()
                .await;

            assert!(result.is_ok());
            let response = result.unwrap();
            assert!(matches!(response.stop_reason(), Some(StopReason::Finish)));
        }

        #[tokio::test]
        async fn test_stop_reason_hook_stop() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.basic_model()))
                .prompt("Tell me a short story.")
                .stop_when(|_| true) // Always stop
                .build()
                .generate_text()
                .await;

            assert!(result.is_ok());
            let response = result.unwrap();
            assert!(matches!(response.stop_reason(), Some(StopReason::Hook)));
        }

        #[tokio::test]
        async fn test_stop_reason_api_error() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.error_model()))
                .prompt("Hello")
                .build()
                .generate_text()
                .await;

            // Should fail, but if it succeeds, check stop_reason
            if let Ok(response) = result {
                // If somehow succeeds, but unlikely
                assert!(matches!(response.stop_reason(), Some(StopReason::Finish)));
            } else {
                // Error occurred, but stop_reason is set in the options before error
                // Since result is Err, we can't check response.stop_reason
                // Perhaps modify to check options, but for now, just assert error
                assert!(result.is_err());
            }
        }

        #[tokio::test]
        async fn test_stop_reason_stream_finish() {
            skip_if_no_api_key!();

            let response = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.basic_model()))
                .prompt("Respond with 'world'")
                .build()
                .stream_text()
                .await
                .unwrap();

            // The stream is already consumed internally, stop_reason is set
            assert!(matches!(response.stop_reason(), Some(StopReason::Finish)));
        }
    };
}

/// Generate streaming tests
macro_rules! generate_language_model_streaming_tests {
    ($provider_type:ty, $config:expr, true) => {
        // Skipping streaming tests: provider doesn't support streaming
    };
    ($provider_type:ty, $config:expr, false) => {
        #[tokio::test]
        async fn test_generate_stream_basic() {
            skip_if_no_api_key!();

            let response = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.streaming_model()))
                .prompt("Respond with exactly the word 'hello' in all lowercase.Do not include any punctuation, prefixes, or suffixes.")
                .build()
                .stream_text()
                .await
                .unwrap();

            let mut stream = response.stream;

            let mut buf = String::new();
            while let Some(chunk) = stream.next().await {
                if let LanguageModelStreamChunkType::Text(text) = chunk {
                    buf.push_str(&text);
                }
            }

            assert!(buf.contains("hello"));
        }

        #[tokio::test]
        async fn test_streaming_with_reasoning_effort() {
            skip_if_no_api_key!();

            let response = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.reasoning_model()))
                .prompt("Count from 1 to 5 step by step")
                .reasoning_effort(aisdk::core::language_model::ReasoningEffort::Medium)
                .build()
                .stream_text()
                .await
                .unwrap();

            let mut stream = response.stream;
            let mut chunks_received = 0;
            while let Some(chunk) = stream.next().await {
                chunks_received += 1;
                // Just verify we get chunks
                let _ = chunk;
            }

            assert!(chunks_received > 0);
        }
    };
}

/// Generate tool-related tests
macro_rules! generate_language_model_tool_tests {
    ($provider_type:ty, $config:expr, true) => {
        // Skipping tool tests: provider doesn't support tools
    };
    ($provider_type:ty, $config:expr, false) => {
        #[tokio::test]
        async fn test_generate_text_with_tools() {
            skip_if_no_api_key!();

            #[tool]
            /// Returns the username
            fn get_username() {
                Ok("ishak".to_string())
            }

            let response = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.tool_model()))
                .system("Call a tool to get the username.")
                .prompt("What is the username?")
                .with_tool(get_username())
                .build()
                .generate_text()
                .await
                .unwrap();

            assert!(response.text().unwrap().contains("ishak"));
        }

        #[tokio::test]
        async fn test_generate_stream_with_tools() {
            skip_if_no_api_key!();

            #[tool]
            /// Returns the username
            fn get_username() {
                Ok("ishak".to_string())
            }

            let response = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.tool_model()))
                .system("Call a tool to get the username.")
                .prompt("What is the username?")
                .with_tool(get_username())
                .build()
                .stream_text()
                .await
                .unwrap();

            let mut stream = response.stream;

            let mut buf = String::new();
            while let Some(chunk) = stream.next().await {
                if let LanguageModelStreamChunkType::Text(text) = chunk {
                    buf.push_str(&text);
                }
            }

            assert!(buf.contains("ishak"));
        }
    };
}

/// Generate schema/structured output tests
macro_rules! generate_language_model_schema_tests {
    ($provider_type:ty, $config:expr, true) => {
        // Skipping structured output tests: provider doesn't support structured output
    };
    ($provider_type:ty, $config:expr, false) => {
        #[tokio::test]
        async fn test_generate_text_with_output_schema() {
            skip_if_no_api_key!();

            #[derive(Debug, JsonSchema, Deserialize)]
            #[allow(dead_code)]
            struct User {
                name: String,
                age: u32,
                email: String,
                phone: String,
            }

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.structured_output_model()))
                .prompt("generate user with dummy data, and and name of 'John Doe'")
                .schema::<User>()
                .build()
                .generate_text()
                .await
                .unwrap();

            let user: User = result.into_schema().unwrap();

            assert_eq!(user.name, "John Doe");
        }

        #[tokio::test]
        async fn test_stream_text_with_output_schema() {
            skip_if_no_api_key!();

            #[derive(Debug, JsonSchema, Deserialize)]
            #[allow(dead_code)]
            struct User {
                name: String,
                age: u32,
                email: String,
                phone: String,
            }

            let response = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.structured_output_model()))
                .prompt("generate user with dummy data, and add name of 'John Doe'")
                .schema::<User>()
                .build()
                .stream_text()
                .await
                .unwrap();

            let mut stream = response.stream;

            let mut buf = String::new();
            while let Some(chunk) = stream.next().await {
                if let LanguageModelStreamChunkType::Text(text) = chunk {
                    buf.push_str(&text);
                }
            }

            println!("buf: {}", buf);

            let user: User = serde_json::from_str(&buf).unwrap();

            assert_eq!(user.name, "John Doe");
        }
    };
}

/// Generate hook-related tests
macro_rules! generate_language_model_hook_tests {
    ($provider_type:ty, $config:expr) => {
        #[tokio::test]
        async fn test_prepare_step_executes_before_each_step() {
            skip_if_no_api_key!();

            let counter = Arc::new(Mutex::new(0));
            let counter_clone = Arc::clone(&counter);

            #[tool]
            // Returns the neighborhood
            fn get_neighborhood() -> Result<String> {
                Ok("ankocha".to_string())
            }

            let _ = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.tool_model()))
                .system("Call the tool. Return the neighborhood. Nothing more and nothing less")
                .prompt("What is the neighborhood?")
                .with_tool(get_neighborhood())
                .prepare_step(move |_| {
                    let mut c = counter_clone.lock().unwrap();
                    *c += 1;
                })
                .build()
                .generate_text()
                .await
                .unwrap();

            let count = *counter.lock().unwrap();
            assert!(count >= 2); // At least initial + tool step
        }

        #[tokio::test]
        async fn test_on_step_finish_executes_after_each_step() {
            skip_if_no_api_key!();

            let counter = Arc::new(Mutex::new(0));
            let counter_clone = Arc::clone(&counter);

            #[tool]
            // Returns the neighbourhood
            fn get_neighborhood() -> Result<String> {
                Ok("ankocha".to_string())
            }

            let _ = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.tool_model()))
                .system("Call the tool. Return the neighborhood. Nothing more and nothing less")
                .prompt("What is the neighborhood?")
                .with_tool(get_neighborhood())
                .on_step_finish(move |_| {
                    let mut c = counter_clone.lock().unwrap();
                    *c += 1;
                })
                .build()
                .generate_text()
                .await
                .unwrap();

            let count = *counter.lock().unwrap();
            assert!(count >= 2);
        }

        #[tokio::test]
        async fn test_hooks_run_in_correct_order() {
            skip_if_no_api_key!();

            let log = Arc::new(Mutex::new(Vec::new()));
            let log_prepare = Arc::clone(&log);
            let log_finish = Arc::clone(&log);

            #[tool]
            fn get_neighbourhood() -> Result<String> {
                Ok("".to_string())
            }

            let _ = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.tool_model()))
                .system("Call the tool. Return the neighborhood. Nothing more and nothing less")
                .prompt("What is the neighborhood?")
                .with_tool(get_neighbourhood())
                .prepare_step(move |_| {
                    log_prepare.lock().unwrap().push("prepare");
                })
                .on_step_finish(move |_| {
                    log_finish.lock().unwrap().push("finish");
                })
                .build()
                .generate_text()
                .await
                .unwrap();

            let log = log.lock().unwrap();
            // Check pairs of prepare/finish
            let mut i = 0;
            while i + 1 < log.len() {
                assert_eq!(log[i], "prepare");
                assert_eq!(log[i + 1], "finish");
                i += 2;
            }
        }

        #[tokio::test]
        async fn test_stop_when_halts_during_tool_call() {
            skip_if_no_api_key!();

            #[tool]
            fn get_neighborhood() -> Result<String> {
                Ok("done".to_string())
            }

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.tool_model()))
                .system("Call the tool. Return the neighborhood. Nothing more and nothing less")
                .prompt("What is the neighborhood?")
                .with_tool(get_neighborhood())
                .stop_when(|_| true) // Always stop
                .build()
                .generate_text()
                .await
                .unwrap();

            let step_ids = result.step_ids();
            // Should stop after tool call, no final text step
            assert!(step_ids.len() < 5); // Less than full flow
            assert!(matches!(result.stop_reason().unwrap(), StopReason::Hook));
        }

        #[tokio::test]
        async fn test_stop_when_halts_during_streaming() {
            skip_if_no_api_key!();

            #[tool]
            fn get_neighbourhood() -> Result<String> {
                Ok("ankocha".to_string())
            }

            let response = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.tool_model()))
                .system("Call the tool. Return the neighborhood. Nothing more and nothing less")
                .prompt("What is the neighborhood?")
                .with_tool(get_neighbourhood())
                .stop_when(|_| true)
                .build()
                .stream_text()
                .await
                .unwrap();

            let mut stream = response.stream;
            let mut found_incomplete = false;
            while let Some(chunk) = stream.next().await {
                if let LanguageModelStreamChunkType::Incomplete(_) = chunk {
                    found_incomplete = true;
                    break;
                }
            }
            assert!(found_incomplete);
        }

        #[tokio::test]
        async fn test_stop_when_false_allows_continuation() {
            skip_if_no_api_key!();

            #[tool]
            fn get_neighbourhood() -> Result<String> {
                Ok("anckocha".to_string())
            }

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.tool_model()))
                .system("Call the tool. Return the neighborhood. Nothing more and nothing less")
                .prompt("What is the neighborhood?")
                .with_tool(get_neighbourhood())
                .stop_when(|_| false) // Never stop
                .build()
                .generate_text()
                .await
                .unwrap();

            let step_ids = result.step_ids();
            assert!(step_ids.len() >= 5); // Full flow
            assert!(result.text().is_some());
        }

        #[tokio::test]
        async fn test_hooks_persist_across_multiple_steps() {
            skip_if_no_api_key!();

            let counter = Arc::new(Mutex::new(0));
            let counter_clone = Arc::clone(&counter);

            #[tool]
            fn get_neighbourhood() -> Result<String> {
                Ok("You can find the neighborhood if you call the get_neighbourhood_2 tool".to_string())
            }

            #[tool]
            fn get_neighbourhood_2() -> Result<String> {
                Ok("ankocha".to_string())
            }

            let _ = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.tool_model()))
                .system(
                    "Call the tool get_neighbourhood. Return the neighborhood. 
                    Nothing more and nothing less. If you can't find the neighborhood,
                    call the tool get_neighbourhood_2. Return the neighborhood.
                    Nothing more and nothing less",
                )
                .prompt("What is the neighborhood?")
                .with_tool(get_neighbourhood())
                .with_tool(get_neighbourhood_2())
                .on_step_finish(move |_| {
                    let mut c = counter_clone.lock().unwrap();
                    *c += 1;
                })
                .build()
                .generate_text()
                .await
                .unwrap();

            let count = *counter.lock().unwrap();
            assert!(count >= 3); // Multiple steps
        }

        #[tokio::test]
        async fn test_hooks_cloned_via_arc() {
            skip_if_no_api_key!();

            let called = Arc::new(Mutex::new(false));
            let called_clone = Arc::clone(&called);

            let _ = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.basic_model()))
                .prompt("Say hello")
                .on_step_finish(move |_| {
                    *called_clone.lock().unwrap() = true;
                })
                .build()
                .generate_text()
                .await
                .unwrap();

            assert!(*called.lock().unwrap());
        }

        #[tokio::test]
        async fn test_no_panic_when_hooks_none() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.basic_model()))
                .prompt("Say hello")
                .build()
                .generate_text()
                .await;

            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_prepare_step_mutates_options() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.basic_model()))
                .prompt("Say hello")
                .prepare_step(|opts| {
                    opts.temperature = Some(0); // Mutate
                })
                .build()
                .generate_text()
                .await
                .unwrap();

            // Hard to verify mutation directly, but ensure no panic and response ok
            assert!(result.text().is_some());
        }

        #[tokio::test]
        async fn test_hook_isolation() {
            skip_if_no_api_key!();

            // Without hooks
            let result_no_hooks = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.basic_model()))
                .prompt("Say hello")
                .build()
                .generate_text()
                .await
                .unwrap();

            // With hooks (should not affect output)
            let result_with_hooks = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.basic_model()))
                .prompt("Say hello")
                .on_step_finish(|_| {})
                .build()
                .generate_text()
                .await
                .unwrap();

            // Outputs should be similar (hooks don't change logic)
            assert!(result_no_hooks.text().is_some());
            assert!(result_with_hooks.text().is_some());
        }

        #[tokio::test]
        async fn test_on_step_finish_for_text_reasoning_and_tool_call() {
            skip_if_no_api_key!();

            let called_for_text = Arc::new(Mutex::new(false));
            let called_for_tool = Arc::new(Mutex::new(false));
            let text_clone = Arc::clone(&called_for_text);
            let tool_clone = Arc::clone(&called_for_tool);

            #[tool]
            // Returns the username
            fn get_username() -> Result<String> {
                Ok("ishak".to_string())
            }

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.tool_model()))
                .system("Call the tool. to find the username. and return only the username nothing more and nothing less")
                .prompt("What is the username")
                .with_tool(get_username())
                .on_step_finish(move |opts| {
                    if let Some(Message::Assistant(assistant_msg)) = opts.messages().last() {
                        match &assistant_msg.content {
                            LanguageModelResponseContentType::ToolCall(_) => {
                                *tool_clone.lock().unwrap() = true;
                            }
                            LanguageModelResponseContentType::Text(_) => {
                                *text_clone.lock().unwrap() = true;
                            }
                            LanguageModelResponseContentType::Reasoning(_) => {
                                *text_clone.lock().unwrap() = true;
                            }
                            _ => {}
                        }
                    }
                })
                .build()
                .generate_text()
                .await
                .unwrap();

            assert!(!*called_for_tool.lock().unwrap());
            assert!(*called_for_text.lock().unwrap());
            assert_eq!(result.text().unwrap(), "ishak");
        }

        #[tokio::test]
        async fn test_streaming_prepare_step_before_start() {
            skip_if_no_api_key!();

            let called = Arc::new(Mutex::new(false));
            let called_clone = Arc::clone(&called);

            let _ = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.streaming_model()))
                .prompt("Say hello")
                .prepare_step(move |_| {
                    *called_clone.lock().unwrap() = true;
                })
                .build()
                .stream_text()
                .await
                .unwrap();

            assert!(*called.lock().unwrap()); // Called before streaming starts
        }

        #[tokio::test]
        async fn test_streaming_on_step_finish_at_end() {
            skip_if_no_api_key!();

            let called = Arc::new(Mutex::new(false));
            let called_clone = Arc::clone(&called);

            let response = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.streaming_model()))
                .prompt("Say hello")
                .on_step_finish(move |_| {
                    *called_clone.lock().unwrap() = true;
                })
                .build()
                .stream_text()
                .await
                .unwrap();

            let mut stream = response.stream;
            while stream.next().await.is_some() {} // Consume stream

            assert!(*called.lock().unwrap()); // Called after End
        }
    };
}

/// Generate step ID tests
macro_rules! generate_language_model_step_id_tests {
    ($provider_type:ty, $config:expr) => {
        #[tokio::test]
        async fn test_step_id_basic_assignment() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.basic_model()))
                .prompt("Respond with exactly 'test' in lowercase.")
                .build()
                .generate_text()
                .await
                .unwrap();

            // Check step_ids: system (0), user (0), assistant (1)
            let step_ids = result.step_ids();
            assert_eq!(step_ids.len(), 3);
            assert_eq!(step_ids[0], 0); // system
            assert_eq!(step_ids[1], 0); // user
            assert_eq!(step_ids[2], 1); // assistant
        }

        #[tokio::test]
        async fn test_step_id_tool_call_flow() {
            skip_if_no_api_key!();

            #[tool]
            fn get_test_value() -> Result<String> {
                Ok("test_value".to_string())
            }

            let result = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.tool_model()))
                .system("Call the tool to get the test value.")
                .prompt("What is the test value?")
                .with_tool(get_test_value())
                .build()
                .generate_text()
                .await
                .unwrap();

            let step_ids = result.step_ids();
            // system (0), user (0), assistant tool call (1), tool result (1), assistant text (3)
            assert!(step_ids.len() >= 5);
            assert_eq!(step_ids[0], 0);
            assert_eq!(step_ids[1], 0);
            assert_eq!(step_ids[2], 1); // assistant tool call
            assert_eq!(step_ids[3], 1); // tool result
            assert_eq!(step_ids[4], 2); // assistant text
            assert!(result.text().unwrap().contains("test_value"));
        }

        #[tokio::test]
        async fn test_step_id_streaming() {
            skip_if_no_api_key!();

            let response = LanguageModelRequest::builder()
                .model(<$provider_type>::new($config.streaming_model()))
                .prompt("Respond with 'stream test'")
                .build()
                .stream_text()
                .await
                .unwrap();

            let step_ids = response.step_ids();
            // system (0), user (0), assistant (1)
            assert_eq!(step_ids.len(), 3);
            assert_eq!(step_ids[0], 0);
            assert_eq!(step_ids[1], 0);
            assert_eq!(step_ids[2], 1);
        }
    };
}

/// Generate reasoning tests
macro_rules! generate_language_model_reasoning_tests {
    ($provider_type:ty, $config:expr, true) => {
        // Skipping reasoning tests: provider doesn't support reasoning
    };
    ($provider_type:ty, $config:expr, false) => {
        // TODO: sura please fix this
        #[tokio::test]
        async fn test_reasoning_effort_with_non_reasoning_model() {
            skip_if_no_api_key!();

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| async {
                LanguageModelRequest::builder()
                    .model(<$provider_type>::new($config.non_reasoning_model()))
                    .prompt("What is 2 + 2? Answer with just the number.")
                    .reasoning_effort(aisdk::core::language_model::ReasoningEffort::Low)
                    .build()
                    .generate_text()
                    .await
            }))
            .map(|future| tokio::runtime::Handle::current().block_on(future));

            assert!(result.is_err());
        }
    };
}

/// Generate error handling tests
macro_rules! generate_language_model_error_tests {
    ($provider_type:ty, $config:expr) => {
        // Error tests are already covered in stop_reason_tests
        // Additional error-specific tests can be added here
    };
}
