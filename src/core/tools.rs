//! Tools are a way to extend the capabilities of a language model. aisdk provides a
//! macro to simplify the process of defining and registering tools. This module provides
//! The necessary types and functions for defining and using tools both by the macro and
//! by the user.
//!
//! The Tool struct is the core component of a tool. It contains the `name`, `description`,
//! and `input_schema` of the tool as well as the logic to execute. The `execute`
//! method is the main entry point for executing the tool. The language model is responsible
//! for calling this method using `input_schema` to generate the arguments for the tool.
//!
//!
//! The tool macro generates the necessary code for registering the tool with the SDK.
//! It infers the necessary fields for the Tool struct from a valid rust function.
//!
//! # Example
//! ```
//! use aisdk::core::tools::{Tool, ToolExecute};
//! use aisdk::macros::tool;
//!
//! #[tool]
//! /// Adds two numbers together.
//! pub fn sum(a: u8, b: u8) -> Tool {
//!     Ok(format!("{}", a + b))
//! }
//!
//! let tool: Tool = sum();
//!
//! assert_eq!(tool.name, "sum");
//! assert_eq!(tool.description, " Adds two numbers together.");
//!
//!
//! ```
//!
//! # Example with struct
//!
//! ```rust
//! use aisdk::core::tools::{Tool, ToolExecute, NeedsApproval};
//! use schemars::schema_for;
//! use serde::{Deserialize, Serialize};
//! use serde_json::Value;
//!
//! #[derive(Serialize, Deserialize, schemars::JsonSchema)]
//! struct SumInput {
//!     a: u8,
//!     b: u8,
//! }
//!
//! let tool: Tool = Tool {
//!     name: "sum".to_string(),
//!     description: "Adds two numbers together.".to_string(),
//!     input_schema: schema_for!(SumInput),
//!     execute:
//!         ToolExecute::new(Box::new(|params: Value| {
//!             let a = params["a"].as_u64().unwrap();
//!             let b = params["b"].as_u64().unwrap();
//!             Ok(format!("{}", a + b))
//!         })),
//!     needs_approval: NeedsApproval::Never,
//! };
//!
//! assert_eq!(tool.name, "sum");
//! assert_eq!(tool.description, "Adds two numbers together.");
//! ```
//!

use crate::core::Message;
use crate::error::{Error, Result};
use crate::extensions::Extensions;
use derive_builder::Builder;
use schemars::Schema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use uuid::Uuid;

/// A function that will be called when the tool is executed.
pub type ToolFn = Box<dyn Fn(Value) -> std::result::Result<String, String> + Send + Sync>;

// ============================================================================
// Section: Tool Approval Types
// ============================================================================

/// Function type for dynamic approval decisions.
///
/// This function receives the tool input and context, and returns `true` if
/// approval is required for this specific invocation.
pub type NeedsApprovalFn =
    Arc<dyn Fn(&serde_json::Value, &NeedsApprovalContext) -> bool + Send + Sync>;

/// Context provided to dynamic approval functions.
///
/// This struct contains information about the current tool call and conversation
/// state that can be used to make approval decisions.
#[derive(Debug, Clone)]
pub struct NeedsApprovalContext {
    /// The unique ID of the tool call being evaluated.
    pub tool_call_id: String,
    /// The current conversation messages.
    pub messages: Vec<Message>,
}

/// Determines when a tool requires user approval before execution.
///
/// This enum allows tools to specify their approval requirements, ranging from
/// always requiring approval to dynamic decisions based on input parameters.
///
/// # Examples
///
/// ```rust
/// use aisdk::core::tools::NeedsApproval;
/// use std::sync::Arc;
///
/// // Always require approval
/// let always = NeedsApproval::Always;
///
/// // Never require approval (default)
/// let never = NeedsApproval::Never;
///
/// // Dynamic approval based on input
/// let dynamic = NeedsApproval::Dynamic(Arc::new(|input, _ctx| {
///     input["amount"].as_f64().unwrap_or(0.0) > 1000.0
/// }));
/// ```
#[derive(Clone, Default)]
pub enum NeedsApproval {
    /// Always requires user approval before execution.
    Always,
    /// Never requires user approval (default behavior).
    #[default]
    Never,
    /// Dynamically determines if approval is needed based on input and context.
    Dynamic(NeedsApprovalFn),
}

impl Debug for NeedsApproval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NeedsApproval::Always => write!(f, "NeedsApproval::Always"),
            NeedsApproval::Never => write!(f, "NeedsApproval::Never"),
            NeedsApproval::Dynamic(_) => write!(f, "NeedsApproval::Dynamic(<fn>)"),
        }
    }
}

/// A request for user approval before executing a tool.
///
/// When a tool requires approval, this struct is returned in the response content
/// instead of immediately executing the tool. The user should respond with a
/// [`ToolApprovalResponse`] in their next message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolApprovalRequest {
    /// Unique identifier for this approval request.
    pub approval_id: String,
    /// Information about the tool call that requires approval.
    pub tool_call: ToolCallInfo,
}

impl ToolApprovalRequest {
    /// Creates a new approval request for the given tool call.
    ///
    /// Generates a unique approval ID using UUID v4.
    pub fn new(tool_call: ToolCallInfo) -> Self {
        Self {
            approval_id: Uuid::new_v4().to_string(),
            tool_call,
        }
    }

    /// Creates a new approval request with a specific approval ID.
    ///
    /// This is useful for testing or when you need deterministic IDs.
    pub fn with_id(approval_id: impl Into<String>, tool_call: ToolCallInfo) -> Self {
        Self {
            approval_id: approval_id.into(),
            tool_call,
        }
    }
}

/// User's response to a tool approval request.
///
/// This struct should be included in the user's message to indicate whether
/// the tool execution was approved or denied.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolApprovalResponse {
    /// The approval ID from the corresponding [`ToolApprovalRequest`].
    pub approval_id: String,
    /// Whether the tool execution was approved.
    pub approved: bool,
    /// Optional reason for the decision (especially useful for denials).
    pub reason: Option<String>,
}

impl ToolApprovalResponse {
    /// Creates a new approval response.
    pub fn new(approval_id: impl Into<String>, approved: bool) -> Self {
        Self {
            approval_id: approval_id.into(),
            approved,
            reason: None,
        }
    }

    /// Creates an approval response with a reason.
    pub fn with_reason(
        approval_id: impl Into<String>,
        approved: bool,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            approval_id: approval_id.into(),
            approved,
            reason: Some(reason.into()),
        }
    }

    /// Creates an approved response.
    pub fn approve(approval_id: impl Into<String>) -> Self {
        Self::new(approval_id, true)
    }

    /// Creates a denied response with a reason.
    pub fn deny(approval_id: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::with_reason(approval_id, false, reason)
    }
}

/// Holds the function that will be called when the tool is executed. the function
/// should take a single argument of type `Value` and returns a
/// `Result<String, String>`.
#[derive(Clone)]
pub struct ToolExecute {
    inner: Arc<ToolFn>,
}

impl ToolExecute {
    /// Calls the tool with the given input.
    pub fn call(&self, map: Value) -> Result<String> {
        (*self.inner)(map).map_err(Error::ToolCallError)
    }

    /// Creates a new `ToolExecute` instance with the given function.
    /// The function should take a single argument of type `Value` and return a
    /// `Result<String, String>`.
    pub fn new(f: ToolFn) -> Self {
        Self { inner: Arc::new(f) }
    }
}

impl Default for ToolExecute {
    fn default() -> Self {
        Self::new(Box::new(|_| Ok("".to_string())))
    }
}

impl Serialize for ToolExecute {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("ToolExecuteCall")
    }
}

impl<'de> Deserialize<'de> for ToolExecute {
    fn deserialize<D>(_: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::default())
    }
}

/// The `Tool` struct represents a tool that can be executed by a language model.
/// It contains the name, description, input schema, and execution logic of the tool.
/// The `execute` method is the main entry point for executing the tool and is called.
/// by the language model.
///
/// `name` and `description` help the model identify and understand the tool. `input_schema`
/// defines the structure of the input data that the tool expects. `Schema` is a type from
/// the [`schemars`](https://docs.rs/schemars/latest/schemars/) crate that can be used to
/// define the input schema.
///
/// The execute method is responsible for executing the tool and returning the result to
/// the language model. It takes a single argument of type `Value` and returns a
/// `Result<String, String>`.
///
/// # Example
/// ```
/// use aisdk::core::tools::{Tool, ToolExecute, NeedsApproval};
/// use schemars::schema_for;
/// use serde::{Deserialize, Serialize};
/// use serde_json::Value;
///
/// #[derive(Serialize, Deserialize, schemars::JsonSchema)]
/// struct SumInput {
///     a: u8,
///     b: u8,
/// }
///
/// let tool: Tool = Tool {
///     name: "sum".to_string(),
///     description: "Adds two numbers together.".to_string(),
///     input_schema: schema_for!(SumInput),
///     execute:
///         ToolExecute::new(Box::new(|params: Value| {
///             let a = params["a"].as_u64().unwrap();
///             let b = params["b"].as_u64().unwrap();
///             Ok(format!("{}", a + b))
///         })),
///     needs_approval: NeedsApproval::Never,
/// };
///
/// assert_eq!(tool.name, "sum");
/// assert_eq!(tool.description, "Adds two numbers together.");
/// ```
#[derive(Builder, Clone, Default)]
#[builder(pattern = "owned", setter(into), build_fn(error = "Error"))]
pub struct Tool {
    /// The name of the tool
    pub name: String,
    /// AI friendly description
    pub description: String,
    /// The input schema of the tool as json schema
    pub input_schema: Schema,
    /// The output schema of the tool. AI will use this to generate outputs.
    pub execute: ToolExecute,
    /// Whether this tool requires user approval before execution.
    #[builder(default)]
    pub needs_approval: NeedsApproval,
}

impl Debug for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tool")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("needs_approval", &self.needs_approval)
            .finish()
    }
}

impl Tool {
    /// Get builder to construct a new tool.
    pub fn builder() -> ToolBuilder {
        ToolBuilder::default()
    }
}

#[derive(Debug, Clone, Default)]
/// A list of tools.
pub struct ToolList {
    /// The list of tools.
    pub tools: Arc<Mutex<Vec<Tool>>>,
}

impl ToolList {
    /// Creates a new `ToolList` instance with the given list of tools.
    pub fn new(tools: Vec<Tool>) -> Self {
        Self {
            tools: Arc::new(Mutex::new(tools)),
        }
    }

    /// Adds a tool to the list.
    pub fn add_tool(&mut self, tool: Tool) {
        self.tools
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(tool);
    }

    /// Executes a tool.
    pub async fn execute(&self, tool_info: ToolCallInfo) -> JoinHandle<Result<String>> {
        let tools = self.tools.clone();
        tokio::spawn(async move {
            let tools = tools
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let tool = tools.iter().find(|tool| tool.name == tool_info.tool.name);

            match tool {
                Some(tool) => tool.execute.call(tool_info.input),
                None => Err(crate::error::Error::ToolCallError(
                    "Tool not found".to_string(),
                )),
            }
        })
    }

    /// Checks if a tool requires approval for the given call.
    ///
    /// Returns `true` if approval is needed, `false` otherwise.
    pub fn needs_approval(&self, tool_call: &ToolCallInfo, messages: &[Message]) -> bool {
        let tools = self
            .tools
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let tool = tools.iter().find(|t| t.name == tool_call.tool.name);

        match tool {
            Some(tool) => match &tool.needs_approval {
                NeedsApproval::Never => false,
                NeedsApproval::Always => true,
                NeedsApproval::Dynamic(f) => {
                    let ctx = NeedsApprovalContext {
                        tool_call_id: tool_call.tool.id.clone(),
                        messages: messages.to_vec(),
                    };
                    f(&tool_call.input, &ctx)
                }
            },
            None => false,
        }
    }

    /// Gets a tool by name.
    pub fn get_tool(&self, name: &str) -> Option<Tool> {
        let tools = self
            .tools
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        tools.iter().find(|t| t.name == name).cloned()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Describes a tool
pub struct ToolDetails {
    /// The name of the tool, usually a function name.
    pub name: String,
    /// Uniquely identifies a tool, usually provided by the provider.
    pub id: String,
}

/// Contains information necessary to call a tool
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallInfo {
    /// The details of the tool to be called.
    pub tool: ToolDetails,
    /// The input parameters for the tool.
    pub input: serde_json::Value,
    /// Provider-specific extensions.
    #[serde(skip)]
    pub extensions: Extensions,
}

impl PartialEq for ToolCallInfo {
    fn eq(&self, other: &Self) -> bool {
        self.tool == other.tool && self.input == other.input
    }
}

impl ToolCallInfo {
    /// Creates a new `ToolCallInfo` instance with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            tool: ToolDetails {
                name: name.into(),
                ..Default::default()
            },
            extensions: Extensions::default(),
            ..Default::default()
        }
    }

    /// Sets the name of the tool.
    pub fn name(&mut self, name: impl Into<String>) {
        self.tool.name = name.into();
    }

    /// Sets the id of the tool.
    pub fn id(&mut self, id: impl Into<String>) {
        self.tool.id = id.into();
    }

    /// Sets the input of the tool.
    pub fn input(&mut self, inp: serde_json::Value) {
        self.input = inp;
    }
}

/// Contains information from a tool
#[derive(Debug, Clone)]
pub struct ToolResultInfo {
    /// The details of the tool.
    pub tool: ToolDetails,

    /// The output of the tool.
    pub output: Result<serde_json::Value>,
}

impl Default for ToolResultInfo {
    fn default() -> Self {
        Self {
            tool: ToolDetails::default(),
            output: Ok(serde_json::Value::Null),
        }
    }
}

impl ToolResultInfo {
    /// Creates a new `ToolResultInfo` instance with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            tool: ToolDetails {
                name: name.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Sets the name of the tool.
    pub fn name(&mut self, name: impl Into<String>) {
        self.tool.name = name.into();
    }

    /// Sets the id of the tool.
    pub fn id(&mut self, id: impl Into<String>) {
        self.tool.id = id.into();
    }

    /// Sets the output of the tool.
    pub fn output(&mut self, inp: serde_json::Value) {
        self.output = Ok(inp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use schemars::schema_for;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, schemars::JsonSchema)]
    struct TestInput {
        value: u32,
    }

    fn create_test_tool(name: &str, needs_approval: NeedsApproval) -> Tool {
        Tool {
            name: name.to_string(),
            description: "Test tool".to_string(),
            input_schema: schema_for!(TestInput),
            execute: ToolExecute::default(),
            needs_approval,
        }
    }

    fn create_test_tool_call(name: &str, input: serde_json::Value) -> ToolCallInfo {
        let mut call = ToolCallInfo::new(name);
        call.input = input;
        call.id(format!("{}_id", name));
        call
    }

    #[test]
    fn test_needs_approval_default() {
        let approval = NeedsApproval::default();
        matches!(approval, NeedsApproval::Never);
    }

    #[test]
    fn test_tool_list_needs_approval_never() {
        let tool = create_test_tool("test_tool", NeedsApproval::Never);
        let tool_list = ToolList::new(vec![tool]);
        let tool_call = create_test_tool_call("test_tool", serde_json::json!({"value": 10}));

        assert!(!tool_list.needs_approval(&tool_call, &[]));
    }

    #[test]
    fn test_tool_list_needs_approval_always() {
        let tool = create_test_tool("test_tool", NeedsApproval::Always);
        let tool_list = ToolList::new(vec![tool]);
        let tool_call = create_test_tool_call("test_tool", serde_json::json!({"value": 10}));

        assert!(tool_list.needs_approval(&tool_call, &[]));
    }

    #[test]
    fn test_tool_list_needs_approval_dynamic_true() {
        let tool = create_test_tool(
            "test_tool",
            NeedsApproval::Dynamic(Arc::new(|input, _ctx| {
                input["value"].as_u64().unwrap_or(0) > 100
            })),
        );
        let tool_list = ToolList::new(vec![tool]);
        let tool_call = create_test_tool_call("test_tool", serde_json::json!({"value": 500}));

        assert!(tool_list.needs_approval(&tool_call, &[]));
    }

    #[test]
    fn test_tool_list_needs_approval_dynamic_false() {
        let tool = create_test_tool(
            "test_tool",
            NeedsApproval::Dynamic(Arc::new(|input, _ctx| {
                input["value"].as_u64().unwrap_or(0) > 100
            })),
        );
        let tool_list = ToolList::new(vec![tool]);
        let tool_call = create_test_tool_call("test_tool", serde_json::json!({"value": 50}));

        assert!(!tool_list.needs_approval(&tool_call, &[]));
    }

    #[test]
    fn test_tool_list_needs_approval_tool_not_found() {
        let tool = create_test_tool("test_tool", NeedsApproval::Always);
        let tool_list = ToolList::new(vec![tool]);
        let tool_call = create_test_tool_call("nonexistent_tool", serde_json::json!({"value": 10}));

        // When tool is not found, should return false (can't execute anyway)
        assert!(!tool_list.needs_approval(&tool_call, &[]));
    }

    #[test]
    fn test_tool_approval_request_new() {
        let tool_call = create_test_tool_call("test_tool", serde_json::json!({"value": 10}));
        let request = ToolApprovalRequest::new(tool_call.clone());

        assert!(!request.approval_id.is_empty());
        assert_eq!(request.tool_call.tool.name, "test_tool");
    }

    #[test]
    fn test_tool_approval_request_with_id() {
        let tool_call = create_test_tool_call("test_tool", serde_json::json!({"value": 10}));
        let request = ToolApprovalRequest::with_id("custom-id", tool_call);

        assert_eq!(request.approval_id, "custom-id");
    }

    #[test]
    fn test_tool_approval_response_new() {
        let response = ToolApprovalResponse::new("approval-1", true);

        assert_eq!(response.approval_id, "approval-1");
        assert!(response.approved);
        assert!(response.reason.is_none());
    }

    #[test]
    fn test_tool_approval_response_with_reason() {
        let response = ToolApprovalResponse::with_reason("approval-1", false, "Too dangerous");

        assert_eq!(response.approval_id, "approval-1");
        assert!(!response.approved);
        assert_eq!(response.reason, Some("Too dangerous".to_string()));
    }

    #[test]
    fn test_tool_approval_response_approve() {
        let response = ToolApprovalResponse::approve("approval-1");

        assert_eq!(response.approval_id, "approval-1");
        assert!(response.approved);
    }

    #[test]
    fn test_tool_approval_response_deny() {
        let response = ToolApprovalResponse::deny("approval-1", "Not allowed");

        assert_eq!(response.approval_id, "approval-1");
        assert!(!response.approved);
        assert_eq!(response.reason, Some("Not allowed".to_string()));
    }

    #[test]
    fn test_tool_call_info_serialization() {
        let mut tool_call = ToolCallInfo::new("test_tool");
        tool_call.id("tool-123");
        tool_call.input = serde_json::json!({"value": 42});

        let serialized = serde_json::to_string(&tool_call).unwrap();
        let deserialized: ToolCallInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.tool.name, "test_tool");
        assert_eq!(deserialized.tool.id, "tool-123");
        assert_eq!(deserialized.input, serde_json::json!({"value": 42}));
    }

    #[test]
    fn test_tool_approval_request_serialization() {
        let tool_call = create_test_tool_call("test_tool", serde_json::json!({"value": 10}));
        let request = ToolApprovalRequest::with_id("approval-1", tool_call);

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: ToolApprovalRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.approval_id, "approval-1");
        assert_eq!(deserialized.tool_call.tool.name, "test_tool");
    }

    #[test]
    fn test_tool_approval_response_serialization() {
        let response = ToolApprovalResponse::with_reason("approval-1", false, "Not allowed");

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: ToolApprovalResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.approval_id, "approval-1");
        assert!(!deserialized.approved);
        assert_eq!(deserialized.reason, Some("Not allowed".to_string()));
    }

    #[test]
    fn test_tool_builder_with_needs_approval() {
        let tool = Tool::builder()
            .name("test_tool")
            .description("A test tool")
            .input_schema(schema_for!(TestInput))
            .execute(ToolExecute::default())
            .needs_approval(NeedsApproval::Always)
            .build()
            .unwrap();

        assert_eq!(tool.name, "test_tool");
        matches!(tool.needs_approval, NeedsApproval::Always);
    }

    #[test]
    fn test_tool_builder_default_needs_approval() {
        let tool = Tool::builder()
            .name("test_tool")
            .description("A test tool")
            .input_schema(schema_for!(TestInput))
            .execute(ToolExecute::default())
            .build()
            .unwrap();

        // Default should be Never
        matches!(tool.needs_approval, NeedsApproval::Never);
    }
}
