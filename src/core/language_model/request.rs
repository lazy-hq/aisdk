//! Defines the `LanguageModelRequest` struct and its builder for configuring text generation requests.
//!
//! This module provides the `LanguageModelRequest` type, which encapsulates a language model
//! and options for generating text or streaming responses. It includes a type-state builder
//! pattern to ensure requests are constructed correctly and safely.

use crate::Error;
use crate::core::Messages;
use crate::core::language_model::{LanguageModel, LanguageModelOptions};
use crate::core::tools::{Tool, ToolExecute};
use crate::core::{ToolContext, capabilities::*};
use futures::StreamExt;
use schemars::{JsonSchema, schema_for};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// Options for text generation requests such as `generate_text` and `stream_text`.
#[derive(Debug)]
pub struct LanguageModelRequest<M: LanguageModel> {
    /// The language model to use for text generation.
    pub model: M,

    /// An optional simple text prompt for the request.
    ///
    /// This should not be set if `messages` are provided in the options.
    pub prompt: Option<String>,

    /// Configuration options for the language model request.
    pub(crate) options: LanguageModelOptions,
}

impl<M: LanguageModel> LanguageModelRequest<M> {
    /// Creates a new builder for constructing a `LanguageModelRequest`.
    ///
    /// This method initiates the type-state builder pattern, starting with the
    /// [`ModelStage`] where you must specify the language model.
    pub fn builder() -> LanguageModelRequestBuilder<M> {
        LanguageModelRequestBuilder::default()
    }
}

impl<M: LanguageModel> Deref for LanguageModelRequest<M> {
    type Target = LanguageModelOptions;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl<M: LanguageModel> DerefMut for LanguageModelRequest<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.options
    }
}

/// Type-state markers for the `LanguageModelRequestBuilder`.
///
/// These zero-sized types ensure the builder is used in the correct order,
/// preventing invalid request configurations at compile time.
///
/// The initial builder state where the language model must be set.
///
/// Transitions to [`SystemStage`] after calling [`model`](LanguageModelRequestBuilder::model).
pub struct ModelStage {}

/// The state after setting the model, where a system prompt can be optionally added.
///
/// Transitions to [`ConversationStage`] after calling [`system`](LanguageModelRequestBuilder::system),
/// or directly to [`OptionsStage`] after calling [`prompt`](LanguageModelRequestBuilder::prompt) or [`messages`](LanguageModelRequestBuilder::messages).
pub struct SystemStage {}

/// The state after optionally setting a system prompt, where conversation input must be provided.
///
/// Transitions to [`OptionsStage`] after calling [`prompt`](LanguageModelRequestBuilder::prompt) or [`messages`](LanguageModelRequestBuilder::messages).
pub struct ConversationStage {}

/// The state after setting a subagent, requiring a system prompt before options can be set.
///
/// Transitions to [`OptionsStage`] after calling [`system`](LanguageModelRequestBuilder::system).
pub struct SubAgentStage {}

/// The final state where additional options can be configured before building.
///
/// Transitions to the completed `LanguageModelRequest` after calling [`build`](LanguageModelRequestBuilder::build).
pub struct OptionsStage {}

/// The default request-building mode for normal text generation requests.
pub struct RequestMode {}

/// The subagent-building mode for producing a subagent tool.
pub struct SubagentMode {}

/// A type-state builder for constructing `LanguageModelRequest` instances.
///
/// This builder uses phantom types to enforce a specific construction order,
/// ensuring that required fields (like the model) are set before optional ones.
///
/// # Type Parameters
///
/// * `M` - The language model type.
/// * `State` - The current builder state, determining available methods.
/// * `Mode` - Whether the builder is producing a normal request or a subagent tool.
pub struct LanguageModelRequestBuilder<M: LanguageModel, State = ModelStage, Mode = RequestMode> {
    model: Option<M>,
    prompt: Option<String>,
    options: LanguageModelOptions,
    subagent_name: Option<String>,
    subagent_description: Option<String>,
    state: std::marker::PhantomData<State>,
    mode: std::marker::PhantomData<Mode>,
}

impl<M: LanguageModel, State, Mode> Deref for LanguageModelRequestBuilder<M, State, Mode> {
    type Target = LanguageModelOptions;

    /// Dereferences to the underlying `LanguageModelOptions`.
    ///
    /// This allows direct access to the options fields during building.
    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl<M: LanguageModel, State, Mode> DerefMut for LanguageModelRequestBuilder<M, State, Mode> {
    /// Mutably dereferences to the underlying `LanguageModelOptions`.
    ///
    /// This allows direct mutation of the options fields during building.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.options
    }
}

impl<M: LanguageModel> LanguageModelRequestBuilder<M> {
    fn default() -> Self {
        LanguageModelRequestBuilder {
            model: None,
            prompt: None,
            options: LanguageModelOptions::default(),
            subagent_name: None,
            subagent_description: None,
            state: std::marker::PhantomData,
            mode: std::marker::PhantomData,
        }
    }
}

/// Methods available in the [`ModelStage`] state.
impl<M: LanguageModel> LanguageModelRequestBuilder<M, ModelStage, RequestMode> {
    /// Sets the language model for the request.
    ///
    /// This is the first required step in building a request.
    ///
    /// # Parameters
    ///
    /// * `model` - The language model instance to use.
    ///
    /// # Returns
    ///
    /// The builder in the [`SystemStage`] state.
    pub fn model(self, model: M) -> LanguageModelRequestBuilder<M, SystemStage, RequestMode> {
        LanguageModelRequestBuilder {
            model: Some(model),
            prompt: self.prompt,
            options: self.options,
            subagent_name: self.subagent_name,
            subagent_description: self.subagent_description,
            state: std::marker::PhantomData,
            mode: std::marker::PhantomData,
        }
    }
}

/// Methods available in the [`SystemStage`] state.
impl<M: LanguageModel> LanguageModelRequestBuilder<M, SystemStage, RequestMode> {
    /// Sets an optional system prompt for the request.
    ///
    /// The system prompt provides context or instructions to the model.
    ///
    /// # Parameters
    ///
    /// * `system` - The system prompt text.
    ///
    /// # Returns
    ///
    /// The builder in the [`ConversationStage`] state.
    pub fn system(
        self,
        system: impl Into<String>,
    ) -> LanguageModelRequestBuilder<M, ConversationStage, RequestMode> {
        LanguageModelRequestBuilder {
            model: self.model,
            prompt: self.prompt,
            options: LanguageModelOptions {
                system: Some(system.into()),
                ..self.options
            },
            subagent_name: self.subagent_name,
            subagent_description: self.subagent_description,
            state: std::marker::PhantomData,
            mode: std::marker::PhantomData,
        }
    }

    /// Switches the builder into **subagent mode**.
    ///
    /// A subagent is an agent that can be invoked by a parent agent as a tool.
    /// The parent delegates a task, and the subagent executes it autonomously,
    /// returning the result once complete.
    ///
    /// Subagents does not accept a direct prompt or messages at construction time.
    /// Instead, it receives its task dynamically from the parent agent when invoked.
    ///
    /// # Parameters
    ///
    /// * `name` - A unique identifier for the subagent.
    /// * `description` - A short description of the subagent’s purpose, used by the
    ///   parent agent to decide when to invoke it.
    ///
    /// # Example
    /// * `name` - "explore-agent"
    /// * `description` - "Explore the codebase to have a deeper understanding of the project."
    ///
    /// # Returns
    ///
    /// The builder in [`SubAgentStage`], where only `.system()` is available next.
    ///
    pub fn subagent(
        self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> LanguageModelRequestBuilder<M, SubAgentStage, SubagentMode> {
        LanguageModelRequestBuilder {
            model: self.model,
            prompt: None,
            options: self.options,
            subagent_name: Some(name.into()),
            subagent_description: Some(description.into()),
            state: std::marker::PhantomData,
            mode: std::marker::PhantomData,
        }
    }

    /// Sets a simple text prompt for the request.
    ///
    /// This skips the system prompt and goes directly to options.
    ///
    /// # Parameters
    ///
    /// * `prompt` - The user prompt text.
    ///
    /// # Returns
    ///
    /// The builder in the [`OptionsStage`] state.
    pub fn prompt(
        self,
        prompt: impl Into<String>,
    ) -> LanguageModelRequestBuilder<M, OptionsStage, RequestMode> {
        LanguageModelRequestBuilder {
            model: self.model,
            prompt: Some(prompt.into()),
            options: self.options,
            subagent_name: self.subagent_name,
            subagent_description: self.subagent_description,
            state: std::marker::PhantomData,
            mode: std::marker::PhantomData,
        }
    }

    /// Sets conversation messages for the request.
    ///
    /// This allows for multi-turn conversations with the model.
    ///
    /// # Parameters
    ///
    /// * `messages` - `Messages` instances representing the conversation.
    ///
    /// # Returns
    ///
    /// The builder in the [`OptionsStage`] state.
    pub fn messages(
        self,
        messages: Messages,
    ) -> LanguageModelRequestBuilder<M, OptionsStage, RequestMode> {
        LanguageModelRequestBuilder {
            model: self.model,
            prompt: self.prompt,
            options: LanguageModelOptions {
                messages: messages.into_iter().map(|msg| msg.into()).collect(),
                ..self.options
            },
            subagent_name: self.subagent_name,
            subagent_description: self.subagent_description,
            state: std::marker::PhantomData,
            mode: std::marker::PhantomData,
        }
    }
}

/// Methods available in the [`ConversationStage`] state.
impl<M: LanguageModel> LanguageModelRequestBuilder<M, ConversationStage, RequestMode> {
    /// Sets a simple text prompt for the request.
    ///
    /// This method allows setting a user prompt.
    /// The prompt represents the user's input to the language model.
    ///
    /// # Parameters
    ///
    /// * `prompt` - The user prompt text.
    ///
    /// # Returns
    ///
    /// The builder in the [`OptionsStage`] state.
    pub fn prompt(
        self,
        prompt: impl Into<String>,
    ) -> LanguageModelRequestBuilder<M, OptionsStage, RequestMode>
    where
        M: TextInputSupport,
    {
        LanguageModelRequestBuilder {
            model: self.model,
            prompt: Some(prompt.into()),
            options: self.options,
            subagent_name: self.subagent_name,
            subagent_description: self.subagent_description,
            state: std::marker::PhantomData,
            mode: std::marker::PhantomData,
        }
    }

    /// Sets conversation messages for the request.
    ///
    /// This method allows providing a full conversation history as a vector of messages,
    /// enabling multi-turn conversations with the language model.
    ///
    /// # Parameters
    ///
    /// * `messages` - `Messages` instances representing the conversation.
    ///
    /// # Returns
    ///
    /// The builder in the [`OptionsStage`] state.
    pub fn messages(
        self,
        messages: Messages,
    ) -> LanguageModelRequestBuilder<M, OptionsStage, RequestMode>
    where
        M: TextInputSupport,
    {
        LanguageModelRequestBuilder {
            model: self.model,
            prompt: self.prompt,
            options: LanguageModelOptions {
                messages: messages.into_iter().map(|msg| msg.into()).collect(),
                ..self.options
            },
            subagent_name: self.subagent_name,
            subagent_description: self.subagent_description,
            state: std::marker::PhantomData,
            mode: std::marker::PhantomData,
        }
    }
}

/// Methods available in the subagent system stage.
impl<M: LanguageModel> LanguageModelRequestBuilder<M, SubAgentStage, SubagentMode> {
    /// Sets the system prompt for the delegated subagent request,
    /// defining how the subagent should behave.
    ///
    /// Subagents require a system prompt before they can be configured further or built.
    pub fn system(
        self,
        system: impl Into<String>,
    ) -> LanguageModelRequestBuilder<M, OptionsStage, SubagentMode> {
        LanguageModelRequestBuilder {
            model: self.model,
            prompt: None,
            options: LanguageModelOptions {
                system: Some(system.into()),
                ..self.options
            },
            subagent_name: self.subagent_name,
            subagent_description: self.subagent_description,
            state: std::marker::PhantomData,
            mode: std::marker::PhantomData,
        }
    }
}

/// Methods available in the [`OptionsStage`] state.
impl<M: LanguageModel, Mode> LanguageModelRequestBuilder<M, OptionsStage, Mode> {
    /// Sets the output schema for structured generation.
    ///
    /// This method configures the language model to generate output that conforms
    /// to the provided JSON schema. The schema is derived from the given type `T`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type that implements [`JsonSchema`], used to generate the output schema.
    ///
    /// # Returns
    ///
    /// The builder with the schema configured.
    pub fn schema<T: JsonSchema>(mut self) -> Self
    where
        M: StructuredOutputSupport,
    {
        self.schema = Some(schema_for!(T));
        self
    }

    /// Sets a seed for deterministic generation.
    ///
    /// # Parameters
    ///
    /// * `seed` - The random seed value.
    ///
    /// # Returns
    ///
    /// The builder with the seed set.
    pub fn seed(mut self, seed: impl Into<u32>) -> Self {
        self.seed = Some(seed.into());
        self
    }

    /// Sets the temperature for generation randomness (0-100, scaled to 0.0-1.0).
    ///
    /// Higher values increase creativity, lower values increase determinism.
    ///
    /// # Parameters
    ///
    /// * `temperature` - The temperature value (0-100).
    ///
    /// # Returns
    ///
    /// The builder with the temperature set.
    pub fn temperature(mut self, temperature: impl Into<u32>) -> Self {
        self.temperature = Some(temperature.into());
        self
    }

    /// Sets the top-p (nucleus) sampling parameter (0-100, scaled to 0.0-1.0).
    ///
    /// # Parameters
    ///
    /// * `top_p` - The top-p value (0-100).
    ///
    /// # Returns
    ///
    /// The builder with top-p set.
    pub fn top_p(mut self, top_p: impl Into<u32>) -> Self {
        self.top_p = Some(top_p.into());
        self
    }

    /// Sets the top-k sampling parameter.
    ///
    /// # Parameters
    ///
    /// * `top_k` - The top-k value.
    ///
    /// # Returns
    ///
    /// The builder with top-k set.
    pub fn top_k(mut self, top_k: impl Into<u32>) -> Self {
        self.top_k = Some(top_k.into());
        self
    }

    /// Sets stop sequences that halt generation.
    ///
    /// # Parameters
    ///
    /// * `stop_sequences` - A list of strings that stop generation when encountered.
    ///
    /// # Returns
    ///
    /// The builder with stop sequences set.
    pub fn stop_sequences(mut self, stop_sequences: impl Into<Vec<String>>) -> Self {
        self.stop_sequences = Some(stop_sequences.into());
        self
    }

    /// Sets the maximum number of retries for failed requests.
    ///
    /// # Parameters
    ///
    /// * `max_retries` - The maximum retry count.
    ///
    /// # Returns
    ///
    /// The builder with max retries set.
    pub fn max_retries(mut self, max_retries: impl Into<u32>) -> Self {
        self.max_retries = Some(max_retries.into());
        self
    }

    /// Sets the frequency penalty to reduce repetition.
    ///
    /// # Parameters
    ///
    /// * `frequency_penalty` - The penalty value.
    ///
    /// # Returns
    ///
    /// The builder with frequency penalty set.
    pub fn frequency_penalty(mut self, frequency_penalty: impl Into<f32>) -> Self {
        self.frequency_penalty = Some(frequency_penalty.into());
        self
    }

    /// Adds a tool to the request.
    ///
    /// # Arguments
    ///
    /// * `tool` - The tool to add.
    ///
    /// # Returns
    ///
    /// The builder with the tool added.
    pub fn with_tool(mut self, tool: Tool) -> Self
    where
        M: ToolCallSupport,
    {
        self.tools.get_or_insert_default().add_tool(tool);
        self
    }

    /// Sets a condition to stop the generation loop.
    ///
    /// # Parameters
    ///
    /// * `hook` - A function that returns `true` when generation should stop.
    ///
    /// # Returns
    ///
    /// The builder with the stop condition set.
    pub fn stop_when<F>(mut self, hook: F) -> Self
    where
        F: Fn(&LanguageModelOptions) -> bool + Send + Sync + 'static,
    {
        self.stop_when = Some(Arc::new(hook));
        self
    }

    /// Sets a hook to run at the start of each generation step.
    ///
    /// # Parameters
    ///
    /// * `hook` - A function called before each step.
    ///
    /// # Returns
    ///
    /// The builder with the hook set.
    pub fn on_step_start<F>(mut self, hook: F) -> Self
    where
        F: Fn(&mut LanguageModelOptions) + Send + Sync + 'static,
    {
        self.on_step_start = Some(Arc::new(hook));
        self
    }

    /// Sets a hook to run at the end of each generation step.
    ///
    /// # Parameters
    ///
    /// * `hook` - A function called after each step.
    ///
    /// # Returns
    ///
    /// The builder with the hook set.
    pub fn on_step_finish<F>(mut self, hook: F) -> Self
    where
        F: Fn(&LanguageModelOptions) + Send + Sync + 'static,
    {
        self.on_step_finish = Some(Arc::new(hook));
        self
    }

    /// Sets the reasoning effort level.
    ///
    /// # Parameters
    ///
    /// * `reasoning_effort` - The effort level.
    ///
    /// # Returns
    ///
    /// The builder with reasoning effort set.
    pub fn reasoning_effort(
        mut self,
        reasoning_effort: impl Into<crate::core::language_model::ReasoningEffort>,
    ) -> Self
    where
        M: ReasoningSupport,
    {
        self.reasoning_effort = Some(reasoning_effort.into());
        self
    }

    /// Sets custom HTTP headers for the request.
    ///
    /// These headers will be merged with the provider's default headers.
    /// If a header key conflicts with a default header, the custom value takes precedence.
    ///
    /// # Parameters
    ///
    /// * `headers` - A map of header names to values.
    ///
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.options.headers = Some(headers);
        self
    }

    /// Sets extra fields to merge into the provider's request body.
    ///
    /// These fields are merged at the top level of the JSON body,
    /// allowing provider-specific options without modifying the SDK.
    ///
    /// # Example
    ///
    /// ```ignore
    /// .body(serde_json::json!({
    ///     "store": false
    /// }))
    /// ```
    pub fn body(mut self, body: serde_json::Value) -> Self {
        if let serde_json::Value::Object(map) = body {
            self.options.body = Some(map);
        }
        self
    }
}

impl<M: LanguageModel> LanguageModelRequestBuilder<M, OptionsStage, RequestMode> {
    /// Builds a standard `LanguageModelRequest`.
    ///
    /// This method consumes the builder and returns the configured request.
    ///
    /// # Returns
    ///
    /// The constructed `LanguageModelRequest`.
    pub fn build(self) -> LanguageModelRequest<M> {
        let model = self
            .model
            .unwrap_or_else(|| unreachable!("Model must be set"));

        LanguageModelRequest {
            model,
            prompt: self.prompt,
            options: self.options,
        }
    }
}

// --------------------------------------------------------------------------------
// SubAgent
// --------------------------------------------------------------------------------

#[derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
/// Input schema for subagent tools.
pub struct SubAgentInput {
    /// The task prompt delegated to the subagent.
    pub prompt: String,
}

impl<M: LanguageModel> LanguageModelRequestBuilder<M, OptionsStage, SubagentMode> {
    /// Builds the subagent as a tool that can be provided to a parent agent.
    pub fn build(self) -> Tool {
        let name = self
            .subagent_name
            .unwrap_or_else(|| unreachable!("Subagent name is guaranteed to be set"));
        let description = self
            .subagent_description
            .unwrap_or_else(|| unreachable!("Subagent description is guaranteed to be set"));
        let model = self
            .model
            .unwrap_or_else(|| unreachable!("Model is guaranteed to be set"));
        let options = self.options;

        Tool::builder()
            .name(name)
            .description(description)
            .input_schema(schema_for!(SubAgentInput))
            .execute(ToolExecute::from_async(move |ctx: ToolContext, input| {
                let prompt = input["prompt"]
                    .as_str()
                    .unwrap_or("The agent didn’t give the task as a prompt.");
                let mut request = LanguageModelRequest::<M> {
                    model: model.clone(),
                    prompt: Some(prompt.into()),
                    options: options.clone(),
                };

                async move {
                    if let Some(_tx) = ctx.stream_tx() {
                        let mut response = request.stream_text().await?;
                        // consume the stream and send chunks to main agent
                        {
                            let stream = &mut response.stream;
                            while let Some(chunk) = stream.next().await {
                                ctx.emit(chunk).map_err(|err| {
                                    Error::ToolCallError(format!(
                                        "failed to emit subagent stream chunk: {err}"
                                    ))
                                })?;
                            }
                        }

                        // get the final text
                        let text = response.text().await;
                        return Ok(text.unwrap_or_default());
                    }

                    let response = request.generate_text().await?;
                    let text = response.text().unwrap_or_default();
                    Ok(text)
                }
            }))
            .build()
            .unwrap()
    }
}
