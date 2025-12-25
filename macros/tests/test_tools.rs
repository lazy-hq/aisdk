// tests
#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use aisdk::core::tools::Tool;
    use aisdk::macros::tool;
    use serde_json::Value;
    use std::collections::HashMap;

    #[tool]
    /// This is The Description of an example tool.
    pub fn my_example_tool(a: u8, b: Option<u8>) -> Tool {
        Ok(format!("{}{}", a, b.unwrap_or(0)))
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
                .call(Value::Object(
                    HashMap::from([
                        ("a".to_string(), 1.into()),
                        ("b".to_string(), Option::<u32>::None.into())
                    ])
                    .into_iter()
                    .collect()
                ))
                .unwrap(),
            "10".to_string()
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
}
