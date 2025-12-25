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
//! use aisdk::core::tools::{Tool, ToolExecute};
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
//! };
//!
//! assert_eq!(tool.name, "sum");
//! assert_eq!(tool.description, "Adds two numbers together.");
//! ```
//!

use crate::error::{Error, Result};
use crate::extensions::Extensions;
use derive_builder::Builder;
use schemars::Schema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

/// A function that will be called when the tool is executed.
pub type ToolFn = Box<dyn Fn(Value) -> std::result::Result<String, String> + Send + Sync>;

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
/// use aisdk::core::tools::{Tool, ToolExecute};
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
}

impl Debug for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tool")
            .field("name", &self.name)
            .field("description", &self.description)
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
}

#[derive(Default, Debug, Clone, PartialEq)]
/// Describes a tool
pub struct ToolDetails {
    /// The name of the tool, usually a function name.
    pub name: String,
    /// Uniquely identifies a tool, usually provided by the provider.
    pub id: String,
}

/// Contains information necessary to call a tool
#[derive(Default, Debug, Clone)]
pub struct ToolCallInfo {
    /// The details of the tool to be called.
    pub tool: ToolDetails,
    /// The input parameters for the tool.
    pub input: serde_json::Value,
    /// Provider-specific extensions.
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
