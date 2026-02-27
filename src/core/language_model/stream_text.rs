//! Text Streaming impl for the `LanguageModelRequest` trait.

use crate::core::{
    AssistantMessage, LanguageModelStreamChunkType, Message, Messages, ToolCallInfo,
    ToolResultInfo,
    language_model::{
        LanguageModel, LanguageModelOptions, LanguageModelResponseContentType, LanguageModelStream,
        LanguageModelStreamChunk, Step, StopReason, Usage, request::LanguageModelRequest,
    },
    messages::TaggedMessage,
    utils::resolve_message,
};
use crate::error::Result;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

impl<M: LanguageModel> LanguageModelRequest<M> {
    /// Streams text generation and tool execution using the language model.
    ///
    /// This method performs streaming text generation, providing real-time access to response chunks
    /// as they are produced. It supports tool calling and execution in multiple steps, streaming
    /// intermediate results and handling tool interactions dynamically.
    ///
    /// For non-streaming responses, use [`generate_text`](Self::generate_text) instead.
    ///
    /// # Returns
    ///
    /// A [`StreamTextResponse`] containing the stream of chunks and final conversation state.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if the underlying language model fails to generate a response
    /// or if tool execution encounters an error.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    ///# #[cfg(feature = "openai")]
    ///# {
    ///    use aisdk::{
    ///        core::{LanguageModelRequest, LanguageModelStreamChunkType},
    ///        providers::OpenAI,
    ///    };
    ///    use futures::StreamExt;
    ///
    ///    async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///
    ///        let openai = OpenAI::gpt_5();
    ///
    ///        let mut stream = LanguageModelRequest::builder()
    ///            .model(openai)
    ///            .prompt("What is the meaning of life?")
    ///            .build()
    ///            .stream_text()
    ///            .await?
    ///            .stream;
    ///
    ///         while let Some(chunk) = stream.next().await {
    ///             if let LanguageModelStreamChunkType::Text(text) = chunk {
    ///                 println!("{}", text);
    ///             }
    ///         }
    ///        
    ///        Ok(())
    ///    }
    ///# }
    /// ```
    pub async fn stream_text(&mut self) -> Result<StreamTextResponse> {
        let (system_prompt, messages) = resolve_message(&self.options, &self.prompt);

        let options = Arc::new(Mutex::new(LanguageModelOptions {
            system: (!system_prompt.is_empty()).then_some(system_prompt),
            messages,
            schema: self.options.schema.to_owned(),
            stop_sequences: self.options.stop_sequences.to_owned(),
            tools: self.options.tools.to_owned(),
            stop_when: self.options.stop_when.clone(),
            on_step_start: self.options.on_step_start.clone(),
            on_step_finish: self.options.on_step_finish.clone(),
            stop_reason: None,
            ..self.options
        }));

        let (tx, stream) = LanguageModelStream::new();
        let _ = tx.send(LanguageModelStreamChunkType::Start);

        let mut model = self.model.clone();

        let thread_options = options.clone();
        tokio::spawn(async move {
            loop {
                let mut options = thread_options.lock().await;
                // Update the current step
                options.current_step_id += 1;
                let current_step_id = options.current_step_id;

                // Prepare the next step
                if let Some(hook) = options.on_step_start.clone() {
                    hook(&mut options);
                }

                let response_result = model.stream_text(options.clone()).await;
                let mut response = match response_result {
                    Ok(r) => r,
                    Err(e) => {
                        options.stop_reason = Some(StopReason::Error(e.clone()));
                        let _ = tx.send(LanguageModelStreamChunkType::Failed(format!(
                            "Model streaming failed: {e}"
                        )));
                        return Err(e);
                    }
                };

                while let Some(ref chunk) = response.next().await {
                    match chunk {
                        Ok(chunk) => {
                            let mut had_tool_call = false;
                            for output in chunk {
                                match output {
                                    LanguageModelStreamChunk::Done(final_msg) => {
                                        match final_msg.content {
                                            LanguageModelResponseContentType::Text(_) => {
                                                let assistant_msg =
                                                    Message::Assistant(AssistantMessage {
                                                        content: final_msg.content.clone(),
                                                        usage: final_msg.usage.clone(),
                                                    });
                                                options.messages.push(TaggedMessage::new(
                                                    current_step_id,
                                                    assistant_msg,
                                                ));
                                                options.stop_reason = Some(StopReason::Finish);
                                            }
                                            LanguageModelResponseContentType::Reasoning {
                                                ref content,
                                                ref extensions,
                                            } => {
                                                options.messages.push(TaggedMessage::new(
                                                current_step_id,
                                                Message::Assistant(AssistantMessage {
                                                    content:
                                                        LanguageModelResponseContentType::Reasoning {
                                                            content: content.clone(),
                                                            extensions: extensions.clone(),
                                                        },
                                                    usage: final_msg.usage.clone(),
                                                    }),
                                                ));
                                                options.stop_reason = Some(StopReason::Finish);
                                            }
                                            LanguageModelResponseContentType::ToolCall(
                                                ref tool_info,
                                            ) => {
                                                // add tool message
                                                let usage = final_msg.usage.clone();
                                                let _ = &options.messages.push(TaggedMessage::new(
                                                    current_step_id.to_owned(),
                                                    Message::Assistant(AssistantMessage::new(
                                                        LanguageModelResponseContentType::ToolCall(
                                                            tool_info.clone(),
                                                        ),
                                                        usage,
                                                    )),
                                                ));
                                                // Emit tool call info BEFORE execution
                                                let _ = tx.send(
                                                    LanguageModelStreamChunkType::ToolCallStart(
                                                        tool_info.clone(),
                                                    ),
                                                );
                                                options.handle_tool_call(tool_info).await;
                                                // Emit tool result AFTER execution (last message is the result)
                                                if let Some(TaggedMessage {
                                                    message: Message::Tool(result_info),
                                                    ..
                                                }) = options.messages.last()
                                                {
                                                    let _ = tx.send(
                                                        LanguageModelStreamChunkType::ToolResult(
                                                            result_info.clone(),
                                                        ),
                                                    );
                                                }
                                                had_tool_call = true;
                                            }
                                            _ => {}
                                        }

                                        // Finish the step
                                        if let Some(ref hook) = options.on_step_finish {
                                            hook(&options);
                                        }

                                        // Stop If
                                        if let Some(hook) = &options.stop_when.clone()
                                            && hook(&options)
                                        {
                                            let _ =
                                                tx.send(LanguageModelStreamChunkType::Incomplete(
                                                    "Stopped by hook".to_string(),
                                                ));
                                            options.stop_reason = Some(StopReason::Hook);
                                            break;
                                        }
                                    }
                                    LanguageModelStreamChunk::Delta(other) => match other {
                                        // Propagate text and reasoning chunks
                                        LanguageModelStreamChunkType::Text(_)
                                        | LanguageModelStreamChunkType::Reasoning(_) => {
                                            let _ = tx.send(other.clone());
                                        }
                                        _ => {}
                                    },
                                }
                            }
                            // When both text and tool_use blocks arrive in a
                            // single MessageStop event, Done(Text) sets
                            // stop_reason = Finish before Done(ToolCall) is
                            // processed. Clear stop_reason so the agentic loop
                            // continues and makes the follow-up API call with
                            // the tool result.
                            if had_tool_call {
                                options.stop_reason = None;
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(LanguageModelStreamChunkType::Failed(e.to_string()));
                            options.stop_reason = Some(StopReason::Error(e.clone()));
                            break;
                        }
                    }

                    match options.stop_reason {
                        None => {}
                        _ => break,
                    };
                }

                match options.stop_reason {
                    None => {}
                    _ => break,
                };
            }

            drop(tx);

            Ok(())
        });

        let result = StreamTextResponse { stream, options };

        Ok(result)
    }
}

// ============================================================================
// Section: response types
// ============================================================================

/// Response from a streaming text generation call.
///
/// This struct contains the streaming response from a language model,
/// including the stream of chunks and the final options state.
pub struct StreamTextResponse {
    /// The stream of response chunks from the language model.
    pub stream: LanguageModelStream,
    // The reason the model stopped generating text.
    options: Arc<Mutex<LanguageModelOptions>>,
}

impl StreamTextResponse {
    /// Returns the step IDs of all messages in the conversation.
    ///
    /// This is primarily used for testing and debugging purposes.
    #[cfg(any(test, feature = "test-access"))]
    pub async fn step_ids(&self) -> Vec<usize> {
        self.options
            .lock()
            .await
            .messages
            .iter()
            .map(|t| t.step_id)
            .collect()
    }
}

impl StreamTextResponse {
    /// Returns all messages from the conversation.
    ///
    /// This includes system prompts, user inputs, assistant responses,
    /// and any tool-related messages that occurred during streaming.
    ///
    /// # Returns
    ///
    /// A vector of all [`Message`] instances in the conversation.
    pub async fn messages(&self) -> Messages {
        self.options.lock().await.messages()
    }

    /// Returns the conversation step with the specified index.
    ///
    /// A step represents all messages exchanged during one cycle of model interaction,
    /// including user input, assistant responses, and tool calls/results.
    ///
    /// # Parameters
    ///
    /// * `index` - The step ID to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<Step>` containing the step if it exists.
    pub async fn step(&self, index: usize) -> Option<Step> {
        self.options.lock().await.step(index)
    }

    /// Returns the most recent conversation step.
    ///
    /// This is equivalent to calling `step()` with the highest step ID.
    ///
    /// # Returns
    ///
    /// An `Option<Step>` containing the last step if any steps exist.
    pub async fn last_step(&self) -> Option<Step> {
        self.options.lock().await.last_step()
    }

    /// Returns all conversation steps in chronological order.
    ///
    /// Each step contains all messages exchanged during that cycle of interaction.
    ///
    /// # Returns
    ///
    /// A vector of all [`Step`] instances in order.
    pub async fn steps(&self) -> Vec<Step> {
        self.options.lock().await.steps()
    }

    /// Calculates the total token usage across all conversation steps.
    ///
    /// This aggregates input, output, reasoning, and cached token counts
    /// from all assistant messages in the conversation.
    ///
    /// # Returns
    ///
    /// A [`Usage`] struct containing the aggregated token statistics.
    pub async fn usage(&self) -> Usage {
        self.options.lock().await.usage()
    }

    /// Returns the content of the last assistant message, excluding reasoning.
    ///
    /// This provides access to the final output content from the language model,
    /// filtering out any reasoning content that may be present.
    ///
    /// # Returns
    ///
    /// An `Option<LanguageModelResponseContentType>` containing the content if available.
    pub async fn content(&self) -> Option<LanguageModelResponseContentType> {
        self.options.lock().await.content().cloned()
    }

    /// Returns the text content of the last assistant message.
    ///
    /// This extracts the plain text from the final assistant response,
    /// if the content type is text.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the text if the last message is text content.
    pub async fn text(&self) -> Option<String> {
        self.options.lock().await.text()
    }

    /// Extracts all tool execution results from the conversation.
    ///
    /// This collects all tool result messages that were generated during
    /// the streaming process, including results from tool calls.
    ///
    /// # Returns
    ///
    /// An `Option<Vec<ToolResultInfo>>` containing all tool results if any exist.
    pub async fn tool_results(&self) -> Option<Vec<ToolResultInfo>> {
        self.options.lock().await.tool_results()
    }

    /// Extracts all tool calls from the conversation.
    ///
    /// This collects all tool call requests that were made by the assistant
    /// during the streaming process.
    ///
    /// # Returns
    ///
    /// An `Option<Vec<ToolCallInfo>>` containing all tool calls if any exist.
    pub async fn tool_calls(&self) -> Option<Vec<ToolCallInfo>> {
        self.options.lock().await.tool_calls()
    }
    /// Returns the reason why text generation stopped.
    ///
    /// This indicates how and why the streaming process terminated,
    /// such as completion, error, or user-defined stop conditions.
    ///
    /// # Returns
    ///
    /// An `Option<StopReason>` indicating the termination reason if available.
    pub async fn stop_reason(&self) -> Option<StopReason> {
        self.options.lock().await.stop_reason()
    }
}
