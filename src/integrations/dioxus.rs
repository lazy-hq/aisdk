//! Integration with Dioxus. WIP.

/// Types for the Dioxus integration.
pub mod types {
    use crate::integrations::vercel_aisdk_ui::VercelUIMessage;
    use dioxus::{prelude::Callback, signals::ReadSignal};
    use std::collections::HashMap;

    // ── DioxusTransportOptions ────────────────────────────────────────────────

    /// Transport-level options that control how HTTP requests are made to the
    /// chat API endpoint.
    ///
    /// Construct with [`DioxusTransportOptions::new`] (or [`Default::default`])
    /// and configure via the builder methods.
    ///
    /// # Example
    /// ```rust,ignore
    /// let transport = DioxusTransportOptions::new()
    ///     .header("Authorization", "Bearer my-token")
    ///     .body(serde_json::json!({ "model": "gpt-4o" }));
    /// ```
    #[derive(Clone, Debug)]
    pub struct DioxusTransportOptions {
        /// Extra HTTP headers sent with every request.
        pub(crate) headers: HashMap<String, String>,

        /// Extra fields merged into the top-level JSON request body.
        pub(crate) body: Option<serde_json::Value>,
    }

    impl DioxusTransportOptions {
        /// Create a new [`DioxusTransportOptions`] with no headers and no extra body.
        pub fn new() -> Self {
            Self {
                headers: HashMap::new(),
                body: None,
            }
        }

        /// Set all extra headers at once, replacing any previously set headers.
        ///
        /// Headers are applied *after* the built-in `Content-Type: application/json`,
        /// so they can override it if necessary.
        ///
        /// # Example
        /// ```rust,ignore
        /// use std::collections::HashMap;
        /// let transport = DioxusTransportOptions::new()
        ///     .headers([("Authorization", "Bearer token"), ("X-Org-Id", "123")]
        ///         .into_iter()
        ///         .map(|(k, v)| (k.to_string(), v.to_string()))
        ///         .collect());
        /// ```
        pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
            self.headers = headers;
            self
        }

        /// Insert a single extra header, in addition to any already set.
        ///
        /// Calling this multiple times accumulates headers. A later call with the
        /// same key overwrites the earlier value.
        pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.headers.insert(key.into(), value.into());
            self
        }

        /// Set extra fields to be merged into the top-level JSON request body.
        ///
        /// The provided value is serialized immediately. Key/value pairs from
        /// this object are merged into the [`VercelUIRequest`] before sending;
        /// existing request keys (`id`, `messages`, `trigger`) take precedence
        /// and cannot be overridden.
        ///
        /// # Example
        /// ```rust,ignore
        /// let transport = DioxusTransportOptions::new()
        ///     .body(serde_json::json!({ "model": "gpt-4o", "temperature": 0.7 }));
        /// ```
        pub fn body<T: serde::Serialize>(mut self, body: T) -> Self {
            self.body = serde_json::to_value(body).ok();
            self
        }
    }

    impl Default for DioxusTransportOptions {
        fn default() -> Self {
            Self::new()
        }
    }

    // ── DioxusUseChatOptions ──────────────────────────────────────────────────

    /// Configuration options for the [`use_chat`](super::hooks::use_chat) hook.
    ///
    /// Construct with [`DioxusUseChatOptions::new`] (or [`Default::default`])
    /// and configure via the builder methods.
    ///
    /// # Example
    /// ```rust,ignore
    /// let options = DioxusUseChatOptions::new()
    ///     .api("/api/my-chat")
    ///     .transport(
    ///         DioxusTransportOptions::new()
    ///             .header("Authorization", "Bearer my-token"),
    ///     );
    /// ```
    pub struct DioxusUseChatOptions {
        /// Server path to POST messages to.
        pub(crate) api: String,

        /// Transport-level options controlling headers and extra body fields.
        pub(crate) transport: DioxusTransportOptions,
    }

    impl DioxusUseChatOptions {
        /// Create a new [`DioxusUseChatOptions`] with defaults:
        /// - `api`: `"/api/chat"`
        /// - `transport`: [`DioxusTransportOptions::default`]
        pub fn new() -> Self {
            Self {
                api: String::from("/api/chat"),
                transport: DioxusTransportOptions::new(),
            }
        }

        /// Set the server endpoint to POST chat messages to.
        pub fn api(mut self, api: impl Into<String>) -> Self {
            self.api = api.into();
            self
        }

        /// Set the transport options (headers, extra body fields).
        pub fn transport(mut self, transport: DioxusTransportOptions) -> Self {
            self.transport = transport;
            self
        }
    }

    impl Default for DioxusUseChatOptions {
        fn default() -> Self {
            Self::new()
        }
    }

    // ── DioxusChatStatus ──────────────────────────────────────────────────────

    /// Current state of the chat session managed by [`use_chat`](super::hooks::use_chat).
    pub enum DioxusChatStatus {
        /// The request has been sent, awaiting a response.
        Submitted,
        /// The first response has been received; the stream is being processed.
        Streaming,
        /// The stream has been fully processed; ready for new requests.
        Ready,
        /// An error has occurred. The inner string describes the failure.
        /// Ready for a new request or regeneration.
        Error(String),
    }

    // ── DioxusChatSignal ──────────────────────────────────────────────────────

    /// The reactive chat state returned by [`use_chat`](super::hooks::use_chat).
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
        let api = options.api.clone();
        let transport = options.transport.clone();

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
            let transport = transport.clone();

            spawn(async move {
                let request = VercelUIRequest {
                    id: uuid::Uuid::new_v4().simple().to_string(),
                    messages: messages(),
                    trigger: "submit-message".to_string(),
                };

                // Serialize the request, then merge any extra body fields from transport
                let body = match serde_json::to_value(&request) {
                    Ok(mut req_value) => {
                        if let Some(extra) = &transport.body
                            && let (Some(req_obj), Some(extra_obj)) =
                                (req_value.as_object_mut(), extra.as_object())
                        {
                            // if let (Some(req_obj), Some(extra_obj)) =
                            // (req_value.as_object_mut(), extra.as_object())

                            for (k, v) in extra_obj {
                                // Extra fields do not override existing request keys
                                req_obj.entry(k.clone()).or_insert_with(|| v.clone());
                            }
                        }
                        match serde_json::to_string(&req_value) {
                            Ok(s) => s,
                            Err(e) => {
                                log::error!("Failed to serialize request: {}", e);
                                *status.write() = DioxusChatStatus::Error(String::from(
                                    "Failed to serialize request",
                                ));
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to serialize request: {}", e);
                        *status.write() =
                            DioxusChatStatus::Error(String::from("Failed to serialize request"));
                        return;
                    }
                };

                let client = reqwest::Client::new();
                let mut request_builder =
                    client.post(&api).header("Content-Type", "application/json");

                // Apply extra headers from transport options
                for (key, value) in &transport.headers {
                    request_builder = request_builder.header(key, value);
                }

                let mut event_source = match request_builder.body(body).eventsource() {
                    Ok(es) => es,
                    Err(e) => {
                        log::error!("Failed to open stream: {}", e);
                        *status.write() =
                            DioxusChatStatus::Error(String::from("Failed to open stream"));
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
                                VercelUIStream::Error { error_text } => {
                                    *status.write() = DioxusChatStatus::Error(error_text);
                                    break;
                                }
                                VercelUIStream::NotSupported { .. } => {
                                    *status.write() = DioxusChatStatus::Error(String::from(
                                        "Stream chunk not supported",
                                    ));
                                    break;
                                }
                                _ => {}
                            }
                        }
                        Err(e) => {
                            // A stream error before we ever received Event::Open means
                            // the connection itself failed — treat as Error.
                            // An error after streaming started is a normal close.
                            let mut s = status.write();
                            match *s {
                                DioxusChatStatus::Streaming => *s = DioxusChatStatus::Ready,
                                DioxusChatStatus::Error(_) => { /* already error, leave it */ }
                                _ => {
                                    *s = DioxusChatStatus::Error(format!(
                                        "Error opening stream, {}",
                                        e
                                    ))
                                }
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
