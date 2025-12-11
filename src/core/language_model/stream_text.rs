use crate::core::{
    AssistantMessage, LanguageModelStreamChunkType, Message,
    language_model::{
        LanguageModel, LanguageModelOptions, LanguageModelResponseContentType, LanguageModelStream,
        LanguageModelStreamChunk, StopReason, request::LanguageModelRequest,
    },
    messages::TaggedMessage,
    utils::resolve_message,
};
use crate::error::Result;
use futures::StreamExt;
use std::sync::{Mutex, OnceLock};
use tokio::runtime::Handle;
use tokio::task::JoinHandle;

impl<M: LanguageModel + 'static> LanguageModelRequest<M> {
    /// Generates Streaming text using a specified language model.
    ///
    /// Generate a text and call tools for a given prompt using a language model.
    /// This function streams the output. If you do not want to stream the output, use `GenerateText` instead.
    ///
    /// Returns an `Error` if the underlying model fails to generate a response.
    pub async fn stream_text(&mut self) -> Result<StreamTextResponse> {
        let (system_prompt, messages) = resolve_message(&self.options, &self.prompt);

        let mut options = LanguageModelOptions {
            system: Some(system_prompt),
            messages,
            schema: self.options.schema.to_owned(),
            stop_sequences: self.options.stop_sequences.to_owned(),
            tools: self.options.tools.to_owned(),
            stop_when: self.options.stop_when.clone(),
            on_step_start: self.options.on_step_start.clone(),
            on_step_finish: self.options.on_step_finish.clone(),
            stop_reason: None,
            ..self.options
        };

        let (tx, stream) = LanguageModelStream::new();
        let _ = tx.send(LanguageModelStreamChunkType::Start);

        let mut this = self.clone();
        let handle = tokio::spawn(async move {
            println!("Handling stream");
            loop {
                // Update the current step
                options.current_step_id += 1;

                // Prepare the next step
                if let Some(hook) = options.on_step_start.clone() {
                    hook(&mut options);
                }

                // let response_result = &mut thread_self.model.stream_text(options.clone()).await;
                let response_result = this.model.stream_text(options.clone()).await;
                let mut response = match response_result {
                    Ok(stream) => stream,
                    Err(e) => {
                        options.stop_reason = Some(StopReason::Error(e.clone()));
                        let _ = tx.send(LanguageModelStreamChunkType::Failed(e.to_string()));
                        // Since this is the first call in the loop, break to avoid proceeding with an invalid stream
                        break;
                    }
                };

                while let Some(ref chunk) = response.next().await {
                    match chunk {
                        Ok(chunk) => {
                            for output in chunk {
                                // TODO: handle Reasoning delta event (streaming reasoning)
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
                                                    options.current_step_id,
                                                    assistant_msg,
                                                ));
                                                options.stop_reason = Some(StopReason::Finish);
                                            }
                                            LanguageModelResponseContentType::Reasoning(
                                                ref reason,
                                            ) => options.messages.push(TaggedMessage::new(
                                                options.current_step_id,
                                                Message::Assistant(AssistantMessage {
                                                    content:
                                                        LanguageModelResponseContentType::Reasoning(
                                                            reason.clone(),
                                                        ),
                                                    usage: final_msg.usage.clone(),
                                                }),
                                            )),
                                            LanguageModelResponseContentType::ToolCall(
                                                ref tool_info,
                                            ) => {
                                                // add tool message
                                                let usage = final_msg.usage.clone();
                                                options.messages.push(TaggedMessage::new(
                                                    options.current_step_id.to_owned(),
                                                    Message::Assistant(AssistantMessage::new(
                                                        LanguageModelResponseContentType::ToolCall(
                                                            tool_info.clone(),
                                                        ),
                                                        usage,
                                                    )),
                                                ));
                                                options.handle_tool_call(tool_info).await;
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

                                        let _ = tx.send(LanguageModelStreamChunkType::End(
                                            final_msg.clone(),
                                        ));
                                    }
                                    LanguageModelStreamChunk::Delta(other) => {
                                        let _ = tx.send(other.clone()); // propagate chunks
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(LanguageModelStreamChunkType::Failed(e.to_string()));
                            options.stop_reason = Some(StopReason::Error(e.clone()));
                            break;
                        }
                    }

                    match &options.stop_reason {
                        None => {}
                        _ => break,
                    };
                }

                match &options.stop_reason {
                    None => {}
                    _ => break,
                };
            }

            drop(tx);
            options
        });

        println!("Spawned stream");

        let result = StreamTextResponse {
            stream,
            options_handle: Mutex::new(Some(handle)),
            cached_options: OnceLock::new(),
        };

        Ok(result)
    }
}

// ============================================================================
// Section: response types
// ============================================================================

// Response from a stream call on `StreamText`.
pub struct StreamTextResponse {
    /// A stream of responses from the language model.
    pub stream: LanguageModelStream,
    // Wrapped in Mutex<Option> for interior mutability (to take ownership in an immutable method).
    options_handle: Mutex<Option<JoinHandle<LanguageModelOptions>>>,
    // Cached result after joining (initialized lazily).
    cached_options: OnceLock<LanguageModelOptions>,
}

// impl Debug for StreamTextResponse {}

impl StreamTextResponse {
    // Constructor example (adjust as needed; assumes you have the handle ready).
    pub fn new(
        stream: LanguageModelStream,
        options_handle: JoinHandle<LanguageModelOptions>,
    ) -> Self {
        Self {
            stream,
            options_handle: Mutex::new(Some(options_handle)),
            cached_options: OnceLock::new(),
        }
    }

    #[cfg(any(test, feature = "test-access"))]
    pub fn step_ids(&self) -> Vec<usize> {
        // Use deref to access the cached options.
        self.messages.iter().map(|t| t.step_id).collect()
    }
}

impl std::ops::Deref for StreamTextResponse {
    type Target = LanguageModelOptions;

    fn deref(&self) -> &Self::Target {
        self.cached_options.get_or_init(|| {
            let mut guard = self.options_handle.lock().unwrap();
            // Take the handle (leaving None); this runs only once due to OnceLock.
            if let Some(handle) = guard.take() {
                // Block synchronously until the task completes.
                Handle::current()
                    .block_on(handle)
                    .expect("Failed to join LanguageModelOptions task") // Or handle JoinError properly
            } else {
                // This branch shouldn't be hit unless called concurrently in a race (rare), but panic or fallback.
                unreachable!("JoinHandle already consumed");
            }
        })
    }
}
