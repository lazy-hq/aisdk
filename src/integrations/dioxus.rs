//! Integration with Dioxus. WIP.

/// Types for the Dioxus integration.
pub mod types {
    use crate::integrations::vercel_aisdk_ui::VercelUIMessage;
    use dioxus::{prelude::Callback, signals::ReadSignal};

    /// Config options for the `use_chat` hook.
    pub struct DioxusUseChatOptions {
        /// Server path to use, defaults to "/api/chat"
        pub api: String,
    }

    impl Default for DioxusUseChatOptions {
        fn default() -> Self {
            Self {
                api: String::from("/api/chat"),
            }
        }
    }

    /// Current state of the chat
    pub enum DioxusChatStatus {
        /// The request has been sent, awaiting a response
        Submitted,
        /// The first response has been received, processing following stream
        Streaming,
        /// The stream has been fully processed, ready for new requests
        Ready,
        /// An error has occurred, ready for new request or regeneration
        Error, // Add body for details
    }

    // The reactive chat state managed by the `use_chat` hook.
    // pub struct _DioxusChatSignal {
    //     /// Chat messages
    //     pub messages: Vec<VercelUIMessage>,
    //     /// Chat state
    //     pub status: DioxusChatStatus,
    // }

    /// The value returned by the `use_chat` hook.
    pub struct DioxusChatSignal {
        /// Chat messages
        pub messages: ReadSignal<Vec<VercelUIMessage>>,
        /// Chat state
        pub status: ReadSignal<DioxusChatStatus>,
        /// Send a message string. Handles appending the user message,
        /// posting to the server, and updating state through the full lifecycle.
        pub send_message: Callback<String>,
    }
}

/// Dioxus hooks
pub mod hooks {
    use super::types::*;
    use crate::integrations::vercel_aisdk_ui::{
        VercelUIMessage, VercelUIMessagePart, VercelUIRequest, VercelUIStream,
    };
    use dioxus::prelude::{ReadableExt, WritableExt, spawn, use_callback, use_signal};
    use futures::StreamExt;
    use reqwest_eventsource::{Event, RequestBuilderExt};

    /// A hook that manages the full lifecycle of a chat session.
    ///
    /// Returns a [`UseChatReturn`] containing:
    /// - `chat`: a reactive [`Signal`] with the current messages and status.
    /// - `send_message`: a [`Callback`] that accepts a [`String`] and handles sending
    ///   the message to the server, streaming the response, and updating state.
    ///
    /// # Example
    /// ```rust,ignore
    /// let UseChatReturn { chat, send_message } = use_chat(DioxusUseChatOptions::default());
    ///
    /// // Read state
    /// let messages = chat.read().messages;
    /// let status = chat.read().status;
    ///
    /// // Send a message
    /// send_message.call("Hello!".to_string());
    /// ```
    pub fn use_chat(options: DioxusUseChatOptions) -> DioxusChatSignal {
        // let mut chat = use_signal(|| _DioxusChatSignal {
        //     messages: vec![],
        //     status: DioxusChatStatus::Ready,
        // });

        let api = options.api.clone();

        let mut messages = use_signal(Vec::new);
        let mut status = use_signal(|| DioxusChatStatus::Ready);

        let send_message = use_callback(move |message: String| {
            // Guard: only allow sending when ready
            if !matches!(*status.read(), DioxusChatStatus::Ready) {
                return;
            }

            // Append the user message and transition to Submitted
            {
                // let mut write_msg = messages.write();
                // let mut write_status = status.write();
                messages.write().push(VercelUIMessage {
                    id: uuid::Uuid::new_v4().simple().to_string(),
                    role: "user".to_string(),
                    parts: vec![VercelUIMessagePart {
                        text: message,
                        part_type: "text".to_string(),
                    }],
                });
                *status.write() = DioxusChatStatus::Submitted;
            }

            let api = api.clone();

            spawn(async move {
                let request = VercelUIRequest {
                    id: uuid::Uuid::new_v4().simple().to_string(),
                    messages: messages(),
                    trigger: "submit-message".to_string(),
                };

                let body = match serde_json::to_string(&request) {
                    Ok(b) => b,
                    Err(_) => {
                        *status.write() = DioxusChatStatus::Error;
                        return;
                    }
                };

                let client = reqwest::Client::new();
                let mut event_source = match client
                    .post(&api)
                    .header("Content-Type", "application/json")
                    .body(body)
                    .eventsource()
                {
                    Ok(es) => es,
                    Err(_) => {
                        *status.write() = DioxusChatStatus::Error;
                        return;
                    }
                };

                // Index of the assistant message being built, set on first text delta
                let mut assistant_idx: Option<usize> = None;

                while let Some(event) = event_source.next().await {
                    match event {
                        Ok(Event::Open) => {
                            // Connection established — push an empty assistant message and start streaming
                            *status.write() = DioxusChatStatus::Streaming;
                            messages.write().push(VercelUIMessage {
                                // TODO: use a generator for id
                                id: uuid::Uuid::new_v4().simple().to_string(),
                                role: "assistant".to_string(),
                                parts: vec![VercelUIMessagePart {
                                    text: String::new(),
                                    part_type: "text".to_string(),
                                }],
                            });
                            assistant_idx = Some(messages.read().len() - 1);
                        }
                        Ok(Event::Message(msg)) => {
                            let chunk = match serde_json::from_str::<VercelUIStream>(&msg.data) {
                                Ok(c) => c,
                                Err(_) => continue,
                            };

                            match chunk {
                                VercelUIStream::TextDelta { delta, .. } => {
                                    if let Some(idx) = assistant_idx
                                        && let Some(part) = messages
                                            .write()
                                            .get_mut(idx)
                                            .and_then(|m| m.parts.get_mut(0))
                                    {
                                        part.text.push_str(&delta);
                                    } // TODO: handle if assistant_idx is not set by Event::Open
                                }
                                VercelUIStream::Error { .. }
                                | VercelUIStream::NotSupported { .. } => {
                                    *status.write() = DioxusChatStatus::Error;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        Err(_) => {
                            // Stream closed or network error
                            if !matches!(*status.read(), DioxusChatStatus::Error) {
                                *status.write() = DioxusChatStatus::Ready;
                            }
                            break;
                        }
                    }
                }

                // Stream exhausted normally
                if matches!(*status.read(), DioxusChatStatus::Streaming) {
                    *status.write() = DioxusChatStatus::Ready;
                }
            });
        });

        DioxusChatSignal {
            messages: messages.into(),
            status: status.into(),
            send_message,
        }
    }
}
