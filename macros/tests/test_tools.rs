// tests
#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use aisdk::__private::schemars::{self, schema_for};
    use aisdk::core::language_model::{
        LanguageModelOptions, LanguageModelStream, LanguageModelStreamChunkType,
    };
    use aisdk::core::tools::{Tool, ToolContext, ToolExecute};
    use aisdk::macros::tool;
    use futures::StreamExt;
    use serde_json::Value;
    use std::collections::HashMap;

    #[tool]
    /// This is The Description of an example tool.
    pub fn my_example_tool(a: u8, b: Option<u8>) -> Tool {
        Ok(format!("{}{}", a, b.unwrap_or(0)))
    }

    #[tool]
    /// Tool that have async body
    pub async fn example_async_tool() -> Tool {
        //sleep for 1 second
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        Ok("Hello World".to_string())
    }

    #[tool]
    /// Tool that reads runtime context.
    pub fn context_tool(ctx: ToolContext, value: String) -> Tool {
        let system = ctx.options().system.as_deref().unwrap_or_default();
        Ok(format!("{system}:{value}"))
    }

    #[tool]
    /// Tool that emits stream chunks through runtime context.
    pub async fn context_stream_tool(ctx: ToolContext, value: String) -> Tool {
        let _ = ctx.emit(LanguageModelStreamChunkType::TextDelta(format!(
            "tool:{value}"
        )));
        Ok(value)
    }

    #[tool]
    /// Tool that accepts runtime context in a non-first position.
    pub fn trailing_context_tool(value: String, ctx: ToolContext) -> Tool {
        let system = ctx.options().system.as_deref().unwrap_or_default();
        Ok(format!("{value}:{system}"))
    }

    #[tokio::test]
    async fn test_tool_macro_with_no_args() {
        let tool = my_example_tool();

        assert_eq!(tool.name, "my_example_tool");
        assert_eq!(
            tool.description,
            " This is The Description of an example tool."
        );
        let schema_properties = tool
            .input_schema
            .as_object()
            .unwrap()
            .get("properties")
            .unwrap();
        assert_eq!(
            schema_properties.get("a").unwrap().get("format").unwrap(),
            &serde_json::Value::String("uint8".to_string())
        );
        assert_eq!(
            schema_properties.get("b").unwrap().get("format").unwrap(),
            &serde_json::Value::String("uint8".to_string())
        );
        assert_eq!(
            schema_properties.get("b").unwrap().get("type").unwrap(),
            &serde_json::Value::Array(vec![
                serde_json::Value::String("integer".to_string()),
                serde_json::Value::String("null".to_string())
            ])
        );
        assert_eq!(
            tool.execute
                .call(
                    ToolContext::default(),
                    Value::Object(
                        HashMap::from([
                            ("a".to_string(), 1.into()),
                            ("b".to_string(), Option::<u32>::None.into())
                        ])
                        .into_iter()
                        .collect()
                    )
                )
                .await
                .unwrap(),
            "10".to_string()
        );
    }

    #[tokio::test]
    async fn test_tool_macro_with_async_body() {
        let tool = example_async_tool();

        assert_eq!(tool.name, "example_async_tool");
        assert_eq!(tool.description, " Tool that have async body");
        assert_eq!(
            tool.execute
                .call(
                    ToolContext::default(),
                    Value::Object(serde_json::Map::new())
                )
                .await
                .unwrap(),
            "Hello World".to_string()
        );
    }

    #[tokio::test]
    async fn test_tool_macro_with_context_excludes_context_from_schema() {
        let tool = context_tool();
        let schema_properties = tool
            .input_schema
            .as_object()
            .unwrap()
            .get("properties")
            .unwrap()
            .as_object()
            .unwrap();

        assert!(schema_properties.contains_key("value"));
        assert!(!schema_properties.contains_key("ctx"));

        let mut options = LanguageModelOptions::default();
        options.system = Some("system prompt".to_string());
        let context = ToolContext::new(options);

        assert_eq!(
            tool.execute
                .call(
                    context,
                    Value::Object(
                        HashMap::from([("value".to_string(), "payload".into())])
                            .into_iter()
                            .collect()
                    )
                )
                .await
                .unwrap(),
            "system prompt:payload"
        );
    }

    #[tokio::test]
    async fn test_tool_macro_with_context_can_emit_stream_chunks() {
        let tool = context_stream_tool();
        let (tx, mut stream) = LanguageModelStream::new();
        let context = ToolContext::new(LanguageModelOptions::default()).with_stream_tx(tx);

        assert_eq!(
            tool.execute
                .call(
                    context,
                    Value::Object(
                        HashMap::from([("value".to_string(), "payload".into())])
                            .into_iter()
                            .collect()
                    )
                )
                .await
                .unwrap(),
            "payload"
        );

        match stream.next().await {
            Some(LanguageModelStreamChunkType::TextDelta(text)) => assert_eq!(text, "tool:payload"),
            other => panic!("expected tool-emitted text chunk, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_tool_macro_with_trailing_context_excludes_context_from_schema() {
        let tool = trailing_context_tool();
        let schema_properties = tool
            .input_schema
            .as_object()
            .unwrap()
            .get("properties")
            .unwrap()
            .as_object()
            .unwrap();

        assert!(schema_properties.contains_key("value"));
        assert!(!schema_properties.contains_key("ctx"));

        let mut options = LanguageModelOptions::default();
        options.system = Some("system prompt".to_string());
        let context = ToolContext::new(options);

        assert_eq!(
            tool.execute
                .call(
                    context,
                    Value::Object(
                        HashMap::from([("value".to_string(), "payload".into())])
                            .into_iter()
                            .collect()
                    )
                )
                .await
                .unwrap(),
            "payload:system prompt"
        );
    }

    #[tool(name = "the-name-for-this-tool")]
    pub fn my_example_tool_with_name(_name: String, a: u8, b: Option<u8>) -> Tool {
        Ok(format!("{}{}", a, b.unwrap_or(0)))
    }

    #[test]
    fn test_tool_macro_with_name() {
        let tool = my_example_tool_with_name();
        assert!(tool.name != "my-example-tool-with-name");
        assert_eq!(tool.name, "the-name-for-this-tool");
    }

    #[tool(desc = "the-description-for-this-tool")]
    /// This is The Description of an example tool.
    pub fn my_example_tool_with_description(_name: String, a: u8, b: Option<u8>) -> Tool {
        Ok(format!("{}{}", a, b.unwrap_or(0)))
    }

    #[test]
    /// This is The Description of an example tool.
    fn test_tool_macro_with_description() {
        let tool = my_example_tool_with_description();
        assert!(tool.description != " This is The Description of an example tool.");
        assert_eq!(tool.description, "the-description-for-this-tool");
    }

    #[tool(
        name = "the-name-for-this-tool",
        desc = "the-description-for-this-tool"
    )]
    /// This is The Description of an example tool.
    pub fn my_example_tool_with_name_and_description(_name: String, a: u8, b: Option<u8>) -> Tool {
        Ok(format!("{}{}", a, b.unwrap_or(0)))
    }

    #[test]
    fn test_tool_macro_with_name_and_description() {
        let tool = my_example_tool_with_name_and_description();
        assert!(tool.name != "my-example-tool-with-name-and-description");
        assert_eq!(tool.name, "the-name-for-this-tool");
        assert!(tool.description != " This is The Description of an example tool.");
        assert_eq!(tool.description, "the-description-for-this-tool");
    }

    #[tool(
        desc = "the-description-for-this-tool",
        name = "the-name-for-this-tool"
    )]
    /// This is The Description of an example tool.
    pub fn my_example_tool_with_description_and_name(_name: String, a: u8, b: Option<u8>) -> Tool {
        Ok(format!("{}{}", a, b.unwrap_or(0)))
    }

    #[test]
    fn test_tool_macro_with_description_and_name() {
        let tool = my_example_tool_with_description_and_name();
        assert!(tool.name != "my-example-tool-with-description-and-name");
        assert_eq!(tool.name, "the-name-for-this-tool");
        assert!(tool.description != " This is The Description of an example tool.");
        assert_eq!(tool.description, "the-description-for-this-tool");
    }

    #[test]
    fn test_argument_json_schema() {}

    #[tokio::test]
    async fn test_tool_builder_with_sync_executor() {
        #[derive(schemars::JsonSchema)]
        struct ToolInput {
            a: u8,
            b: Option<u8>,
        }

        let tool = Tool::builder()
            .name("builder-sync-tool")
            .description("sync builder test")
            .input_schema(schema_for!(ToolInput))
            .execute(ToolExecute::from_sync(|_ctx, params: Value| {
                let a = params["a"].as_u64().unwrap();
                let b = params["b"].as_u64().unwrap_or_default();
                Ok(format!("{}", a + b))
            }))
            .build()
            .unwrap();

        assert_eq!(
            tool.execute
                .call(
                    ToolContext::default(),
                    Value::Object(
                        HashMap::from([("a".to_string(), 2.into()), ("b".to_string(), 3.into())])
                            .into_iter()
                            .collect()
                    )
                )
                .await
                .unwrap(),
            "5".to_string()
        );
    }

    #[tokio::test]
    async fn test_tool_builder_with_async_executor() {
        #[derive(schemars::JsonSchema)]
        struct NoArgs {}

        let tool = Tool::builder()
            .name("builder-async-tool")
            .description("async builder test")
            .input_schema(schema_for!(NoArgs))
            .execute(ToolExecute::from_async(|_ctx, _params: Value| async move {
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                Ok("builder async".to_string())
            }))
            .build()
            .unwrap();

        assert_eq!(
            tool.execute
                .call(
                    ToolContext::default(),
                    Value::Object(serde_json::Map::new())
                )
                .await
                .unwrap(),
            "builder async".to_string()
        );
    }
}
