use crate::core::{
    AssistantMessage, LanguageModelStreamChunkType, Message, ToolCallInfo, ToolResultInfo,
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
    /// Generates Streaming text using a specified language model.
    ///
    /// Generate a text and call tools for a given prompt using a language model.
    /// This function streams the output. If you do not want to stream the output, use `GenerateText` instead.
    ///
    /// Returns an `Error` if the underlying model fails to generate a response.
    pub async fn stream_text(&mut self) -> Result<StreamTextResponse> {
        let (system_prompt, messages) = resolve_message(&self.options, &self.prompt);

        let options = Arc::new(Mutex::new(LanguageModelOptions {
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
        }));

        let (tx, stream) = LanguageModelStream::new();
        let _ = tx.send(LanguageModelStreamChunkType::Start).await;

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
                        let _ = tx
                            .send(LanguageModelStreamChunkType::Failed(format!(
                                "Model streaming failed: {}",
                                e
                            )))
                            .await;
                        return Err(e);
                    }
                };

                while let Some(ref chunk) = response.next().await {
                    println!("chunk: {:?}", chunk);
                    match chunk {
                        Ok(chunk) => {
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
                                            LanguageModelResponseContentType::Reasoning(
                                                ref reason,
                                            ) => {
                                                options.messages.push(TaggedMessage::new(
                                                current_step_id,
                                                Message::Assistant(AssistantMessage {
                                                    content:
                                                        LanguageModelResponseContentType::Reasoning(
                                                            reason.clone(),
                                                        ),
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
                                            let _ = tx
                                                .send(LanguageModelStreamChunkType::Incomplete(
                                                    "Stopped by hook".to_string(),
                                                ))
                                                .await;
                                            options.stop_reason = Some(StopReason::Hook);
                                            break;
                                        }

                                        let _ = tx
                                            .send(LanguageModelStreamChunkType::End(
                                                final_msg.clone(),
                                            ))
                                            .await;
                                    }
                                    LanguageModelStreamChunk::Delta(other) => {
                                        let _ = tx.send(other.clone()).await; // propagate chunks
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let _ = tx
                                .send(LanguageModelStreamChunkType::Failed(e.to_string()))
                                .await;
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

// Response from a stream call on `StreamText`.
pub struct StreamTextResponse {
    /// A stream of responses from the language model.
    pub stream: LanguageModelStream,
    // The reason the model stopped generating text.
    options: Arc<Mutex<LanguageModelOptions>>,
}

impl StreamTextResponse {
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
    pub async fn messages(&self) -> Vec<Message> {
        self.options.lock().await.messages()
    }

    pub async fn step(&self, index: usize) -> Option<Step> {
        self.options.lock().await.step(index)
    }

    pub async fn last_step(&self) -> Option<Step> {
        self.options.lock().await.last_step()
    }

    pub async fn steps(&self) -> Vec<Step> {
        self.options.lock().await.steps()
    }

    pub async fn usage(&self) -> Usage {
        self.options.lock().await.usage()
    }

    pub async fn content(&self) -> Option<LanguageModelResponseContentType> {
        self.options.lock().await.content().cloned()
    }

    pub async fn text(&self) -> Option<String> {
        self.options.lock().await.text()
    }

    pub async fn tool_results(&self) -> Option<Vec<ToolResultInfo>> {
        self.options.lock().await.tool_results()
    }

    pub async fn tool_calls(&self) -> Option<Vec<ToolCallInfo>> {
        self.options.lock().await.tool_calls()
    }
    pub async fn stop_reason(&self) -> Option<StopReason> {
        self.options.lock().await.stop_reason()
    }
}
