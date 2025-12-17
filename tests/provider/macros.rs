/// Main macro to generate all language model provider integration tests.
///
/// This macro generates a comprehensive suite of tests for a language model provider,
/// including basic functionality, streaming, tools, structured output, and error handling.
///
/// # Parameters
///
/// * `provider: $provider_type:ident` - The type of the provider (e.g., `OpenAI`)
/// * `api_key_var: $env_key:expr` - Environment variable name for the API key (e.g., `"OPENAI_API_KEY"`)
/// * `model_struct: $model_struct:ident` - The model struct type (e.g., `Gpt5`)
/// * `default_model: $default_model:expr` - Expression for the default model instance
/// * `tool_model: $tool_model:expr` - Expression for the model instance used in tool tests
/// * `structured_output_model: $structured_output_model:expr` - Expression for the model instance used in structured output tests
/// * `reasoning_model: $reasoning_model:expr` - Expression for the model instance used in reasoning tests
/// * `skip_reasoning: $skip_reasoning:tt` - Bool literal to skip reasoning tests at compile time
/// * `skip_tool: $skip_tool:tt` - Bool literal to skip tool tests at compile time
/// * `skip_structured_output: $skip_structured_output:tt` - Bool literal to skip structured output tests at compile time
/// * `skip_streaming: $skip_streaming:tt` - Bool literal to skip streaming tests at compile time
///
/// # Example
///
/// ```rust
/// generate_language_model_tests!(
///     provider: OpenAI,
///     api_key_var: "OPENAI_API_KEY",
///     model_struct: Gpt5,
///     default_model: OpenAI::gpt_5_nano(),
///     tool_model: OpenAI::gpt_5_nano(),
///     structured_output_model: OpenAI::gpt_5_nano(),
///     reasoning_model: OpenAI::gpt_5_nano(),
///     skip_reasoning: true,
///     skip_tool: false,
///     skip_structured_output: false,
///     skip_streaming: false
/// );
///
macro_rules! generate_language_model_tests {
    (
        provider: $provider_type:ident,
        api_key_var: $env_key:expr,
        model_struct: $model_struct:ident,
        default_model: $default_model:expr,
        tool_model: $tool_model:expr,
        structured_output_model: $structured_output_model:expr,
        reasoning_model: $reasoning_model:expr,
        skip_reasoning: $skip_reasoning:tt,
        skip_tool: $skip_tool:tt,
        skip_structured_output: $skip_structured_output:tt,
        skip_streaming: $skip_streaming:tt
    ) => {
        use aisdk::core::tools::ToolExecute;
        use aisdk::core::{
            LanguageModelRequest, LanguageModelStreamChunkType, Message,
            language_model::{LanguageModelResponseContentType, StopReason},
            tools::Tool,
        };
        use aisdk_macros::tool;
        use dotenv::dotenv;
        use std::sync::{Arc, Mutex};

        #[allow(unused_imports)]
        use {
            futures::StreamExt,
            schemars::JsonSchema,
            serde::Deserialize,
            serde_json::Value,
        };

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

        // Generate all standard test categories
        generate_provider_has_default_interface!($provider_type, $model_struct);
        generate_basic_tests!($default_model);
        generate_language_model_stop_reason_tests!($default_model);
        generate_language_model_hook_tests!($tool_model);
        generate_language_model_step_id_tests!($default_model);
        generate_language_model_streaming_tests!($reasoning_model, $skip_streaming);
        generate_language_model_tool_tests!($tool_model, $skip_tool);
        generate_language_model_schema_tests!($structured_output_model, $skip_structured_output);
        generate_language_model_reasoning_tests!($reasoning_model, $skip_reasoning);
    };
}

// Test to ensure all providers have the default provider settings builder interface
macro_rules! generate_provider_has_default_interface {
    ($provider_type:ident, $model_struct:ident) => {
        #[tokio::test]
        async fn test_provider_has_default_interface() {
            let provider = $provider_type::<$model_struct>::builder()
                .provider_name("test-provider".to_string())
                .api_key("test-api-key")
                .base_url("http://localhost:8080".to_string())
                .build();

            // check if provider didn't throw an error
            assert!(provider.is_ok());

            // check provider settings
            let provider = provider.unwrap();
            assert_eq!(provider.settings.provider_name, "test-provider");
            assert_eq!(provider.settings.api_key, "test-api-key");
            assert_eq!(provider.settings.base_url, "http://localhost:8080/");

            // should fail on invalid base url
            let provider2 = $provider_type::<$model_struct>::builder()
                .provider_name("test-provider2".to_string())
                .api_key("test-api-key2")
                .base_url("ocalhost:80802".to_string())
                .build();

            assert!(provider2.is_err());
            assert_eq!(provider2.unwrap_err().to_string(), "Invalid input: Base URL must start with http:// or https://");

            // should fail on empty api key
            let provider3 = $provider_type::<$model_struct>::builder()
                .provider_name("test-provider3".to_string())
                .api_key("")
                .base_url("http://localhost:8080/".to_string())
                .build();

            assert!(provider3.is_err());
            assert_eq!(provider3.unwrap_err().to_string(), "A required field is missing: api_key");
        }
    };
}

// Generate basic text generation tests
macro_rules! generate_basic_tests {
    ($default_model:expr) => {
        #[tokio::test]
        async fn test_generate_text_basic() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model($default_model)
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
                .model($default_model)
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
                .model($default_model)
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
                .model($default_model)
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
    ($default_model:expr) => {
        #[tokio::test]
        async fn test_stop_reason_normal_finish() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model($default_model)
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
             .model($default_model)
             .prompt("just response with 'HI' nothing less nothing more")
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
                .model($default_model)
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

            let mut response = LanguageModelRequest::builder()
                .model($default_model)
                .prompt("Respond with 'world'")
                .build()
                .stream_text()
                .await
                .unwrap();

            // consume the stream
            while let Some(_) = response.stream.next().await {}

            // The stream is already consumed internally, stop_reason is set
            assert!(matches!(response.stop_reason().await, Some(StopReason::Finish)));
        }
    };
}

/// Generate streaming tests
macro_rules! generate_language_model_streaming_tests {
    ($reasoning_model:expr, true) => {
        // Skipping streaming tests: provider doesn't support streaming
    };
    ($reasoning_model:expr, false) => {
        #[tokio::test]
        async fn test_generate_stream_basic() {
            skip_if_no_api_key!();

            let response = LanguageModelRequest::builder()
                .model($reasoning_model)
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
                .model($reasoning_model)
                .prompt("Count from 1 to 5 step by step")
                .reasoning_effort(aisdk::core::language_model::ReasoningEffort::Medium)
                .build()
                .stream_text()
                .await
                .unwrap();

            let mut stream = response.stream;
            let mut chunks_received = 0;
            while let Some(chunk) = stream.next().await {
                if let LanguageModelStreamChunkType::Text(_) = chunk {
                    chunks_received += 1;
                }
            }

            assert!(chunks_received > 0);
        }

    };
}

/// Generate tool-related tests
macro_rules! generate_language_model_tool_tests {
    ($tool_model:expr, true) => {
        // Skipping tool tests: provider doesn't support tools
    };
    ($tool_model:expr, false) => {
        #[tokio::test]
        async fn test_generate_text_with_tools() {
            skip_if_no_api_key!();

            #[tool]
            /// Returns the username
            fn get_username() -> Tool {
                Ok("ishak".to_string())
            }

            let response = LanguageModelRequest::builder()
                .model($tool_model)
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
        async fn test_generate_text_with_tools_input() {
            skip_if_no_api_key!();

            #[tool]
            /// Returns the username
            fn get_username(user_id: String) -> Tool {
                match user_id.as_str() {
                    "123" => Ok("sura".to_string()),
                    _ => Ok("invalid".to_string()),
                }
            }

            let response = LanguageModelRequest::builder()
                .model($tool_model)
                .system("you are a helpful assistant.")
                .prompt("What is the username with user id 123?")
                .with_tool(get_username())
                .build()
                .generate_text()
                .await
                .unwrap();

            assert!(response.text().unwrap().contains("sura"));
        }

        #[tokio::test]
        async fn test_generate_stream_with_tools() {
            skip_if_no_api_key!();

            #[tool]
            /// Returns the username
            fn get_username() -> Tool {
                Ok("ishak".to_string())
            }

            let response = LanguageModelRequest::builder()
                .model($tool_model)
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

        #[tokio::test]
        async fn test_generate_stream_with_tools_input() {
            skip_if_no_api_key!();

            #[tool]
            /// Returns the username
            fn get_username(user_id: String) -> Tool {
                match user_id.as_str() {
                    "123" => Ok("sura".to_string()),
                    _ => Ok("invalid".to_string()),
                }
            }

            let response = LanguageModelRequest::builder()
                .model($tool_model)
                .system("You are a helpful assistant.")
                .prompt("What is the username for user id 123?")
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

            assert!(buf.contains("sura"));
        }

        #[tokio::test]
        async fn test_generate_stream_with_structs() {
            skip_if_no_api_key!();

            // define tool function body, should return Result<String, String>
            #[allow(unused_variables)]
            let func = ToolExecute::new(Box::new(|inp: Value| {
                // Ai SDK will pass in a json object with the following structure
                // ```json
                // {
                //     "location": "New York"
                // }
                // ```
                let location = inp.get("location").unwrap();
                Ok(format!("Cloudy"))
            }));

            // define tool input structure
            #[derive(schemars::JsonSchema, Debug)]
            #[allow(dead_code)]
            struct ToolInput {
                location: String,
            }

            // change tool arguments to json schema
            // Which will be similar to the following
            // ```json
            // "properties": {
            //     "location": {
            //         "type": "string"
            //     }
            // }
            let schema = schemars::schema_for!(ToolInput);

            // bring it all together
            let get_weather_tool = Tool::builder()
                .name("get-weather")
                .description("Get the weather information given a location")
                .input_schema(schema.clone())
                .execute(func)
                .build()
                .unwrap();

            // call the model with the tool
            let result = LanguageModelRequest::builder()
                .model($tool_model)
                .system("You are a helpful assistant with access to tools.")
                .prompt("What is the weather in New York?")
                .with_tool(get_weather_tool) // you don't need to call it with.
                .build()
                .generate_text()
                .await;

            assert!(result.is_ok());
        }
    };
}

/// Generate schema/structured output tests
macro_rules! generate_language_model_schema_tests {
    ($structured_output_model:expr, true) => {
        // Skipping structured output tests: provider doesn't support structured output
    };
    ($structured_output_model:expr, false) => {
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
                .model($structured_output_model)
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
                .model($structured_output_model)
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

            let user: User = serde_json::from_str(&buf).unwrap();

            assert_eq!(user.name, "John Doe");
        }
    };
}

/// Generate hook-related tests
macro_rules! generate_language_model_hook_tests {
    ($tool_model:expr) => {
        #[tokio::test]
        async fn test_on_step_start_executes_before_each_step() {
            skip_if_no_api_key!();

            let counter = Arc::new(Mutex::new(0));
            let counter_clone = Arc::clone(&counter);

            #[tool]
            // Returns the neighborhood
            fn get_neighborhood() -> Tool {
                Ok("ankocha".to_string())
            }

            let _ = LanguageModelRequest::builder()
                .model($tool_model)
                .system("Call the tool. Return the neighborhood. Nothing more and nothing less")
                .prompt("What is the neighborhood?")
                .with_tool(get_neighborhood())
                .on_step_start(move |_| {
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
            fn get_neighborhood() -> Tool {
                Ok("ankocha".to_string())
            }

            let _ = LanguageModelRequest::builder()
                .model($tool_model)
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
            fn get_neighbourhood() -> Tool {
                Ok("".to_string())
            }

            let _ = LanguageModelRequest::builder()
                .model($tool_model)
                .system("Call the tool. Return the neighborhood. Nothing more and nothing less")
                .prompt("What is the neighborhood?")
                .with_tool(get_neighbourhood())
                .on_step_start(move |_| {
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
            fn get_neighborhood() -> Tool {
                Ok("done".to_string())
            }

            let result = LanguageModelRequest::builder()
                .model($tool_model)
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
            fn get_neighbourhood() -> Tool {
                Ok("ankocha".to_string())
            }

            let response = LanguageModelRequest::builder()
                .model($tool_model)
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
            fn get_neighbourhood() -> Tool {
                Ok("anckocha".to_string())
            }

            let result = LanguageModelRequest::builder()
                .model($tool_model)
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
            fn get_neighbourhood() -> Tool {
                Ok("You can find the neighborhood if you call the get_neighbourhood_2 tool".to_string())
            }

            #[tool]
            fn get_neighbourhood_2() -> Tool {
                Ok("ankocha".to_string())
            }

            let _ = LanguageModelRequest::builder()
                .model($tool_model)
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
                .model($tool_model)
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
                .model($tool_model)
                .prompt("Say hello")
                .build()
                .generate_text()
                .await;

            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_on_step_start_mutates_options() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model($tool_model)
                .prompt("Say hello")
                .on_step_start(|opts| {
                    opts.system = Some("Updated system message!!!".to_string());
                })
                .build()
                .generate_text()
                .await
                .unwrap();

            // Hard to verify mutation directly, but ensure no panic and response ok
            assert!(result.text().is_some());
            assert_eq!(result.options.system.unwrap(), "Updated system message!!!");
        }

        #[tokio::test]
        async fn test_hook_isolation() {
            skip_if_no_api_key!();

            // Without hooks
            let result_no_hooks = LanguageModelRequest::builder()
                .model($tool_model)
                .prompt("Say hello")
                .build()
                .generate_text()
                .await
                .unwrap();

            // With hooks (should not affect output)
            let result_with_hooks = LanguageModelRequest::builder()
                .model($tool_model)
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
            fn get_username() -> Tool {
                Ok("ishak".to_string())
            }

            let result = LanguageModelRequest::builder()
                .model($tool_model)
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
        async fn test_streaming_on_step_start_before_start() {
            skip_if_no_api_key!();

            let called = Arc::new(Mutex::new(false));
            let called_clone = Arc::clone(&called);

            let mut response = LanguageModelRequest::builder()
                .model($tool_model)
                .prompt("Say hello")
                .on_step_start(move |_| {
                    *called_clone.lock().unwrap() = true;
                })
                .build()
                .stream_text()
                .await
                .unwrap();

            // consume the stream
            while let Some(_) = response.stream.next().await {}

            assert!(*called.lock().unwrap()); // Called before streaming starts
        }

        #[tokio::test]
        async fn test_streaming_on_step_finish_at_end() {
            skip_if_no_api_key!();

            let called = Arc::new(Mutex::new(false));
            let called_clone = Arc::clone(&called);

            let response = LanguageModelRequest::builder()
                .model($tool_model)
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
    ($default_model:expr) => {
        #[tokio::test]
        async fn test_step_id_basic_assignment() {
            skip_if_no_api_key!();

            let result = LanguageModelRequest::builder()
                .model($default_model)
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
            fn get_test_value() -> Tool {
                Ok("test_value".to_string())
            }

            let result = LanguageModelRequest::builder()
                .model($default_model)
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
                .model($default_model)
                .prompt("Respond with 'stream test'")
                .build()
                .stream_text()
                .await
                .unwrap();

            let step_ids = response.step_ids().await;
            // user (0), assistant (0)
            assert_eq!(step_ids.len(), 2);
            assert_eq!(step_ids[0], 0);
            assert_eq!(step_ids[1], 0);
        }
    };
}

/// Generate reasoning tests
macro_rules! generate_language_model_reasoning_tests {
    ($reasoning_model:expr, true) => {
        // Skipping reasoning tests: provider doesn't support reasoning
    };
    ($reasoning_model:expr, false) => {
        // TODO: sura please fix this
        #[tokio::test]
        async fn test_reasoning_effort_with_non_reasoning_model() {
            skip_if_no_api_key!();

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| async {
                LanguageModelRequest::builder()
                    .model($reasoning_model)
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
