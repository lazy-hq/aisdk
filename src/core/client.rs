//! This module provides the client for interacting with the AI providers.
//! It is a thin wrapper around the `reqwest` crate.

use crate::error::{Error, Result};
use futures::Stream;
use futures::StreamExt;
use reqwest;
use reqwest::Url;
use reqwest_eventsource::{Event, RequestBuilderExt};
use serde::de::DeserializeOwned;
use std::pin::Pin;

#[allow(dead_code)]
pub(crate) trait Client {
    type Response: DeserializeOwned + std::fmt::Debug + Clone;
    type StreamEvent: DeserializeOwned + From<NotSupportedEvent>;

    fn path(&self) -> &str;
    fn method(&self) -> reqwest::Method;
    fn query_params(&self) -> Vec<(&str, &str)>;
    fn body(&self) -> reqwest::Body;

    /// Sets the default headers for the request
    fn headers(&self) -> reqwest::header::HeaderMap;

    async fn send(&self, base_url: Url) -> Result<Self::Response> {
        let client = reqwest::Client::new();
        let base_url = base_url.join(self.path()).expect("Invalid base URL");
        let resp = client
            .request(self.method(), base_url)
            .headers(self.headers())
            .query(&self.query_params())
            .body(self.body())
            .send()
            .await
            .and_then(|response| response.error_for_status())
            .map_err(|e| Error::ApiError(e.to_string()));

        resp?
            .json::<Self::Response>()
            .await
            .map_err(|e| Error::ApiError(e.to_string()))
    }

    async fn send_and_stream(
        &self,
        base_url: Url,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Self::StreamEvent>> + Send>>>
    where
        Self::StreamEvent: Send + 'static,
    {
        let client = reqwest::Client::new();
        let base_url = base_url.join(self.path()).expect("Invalid base URL");

        let events_stream = client
            .request(self.method(), base_url)
            .headers(self.headers())
            .query(&self.query_params())
            .body(self.body())
            .eventsource()
            .map_err(|e| Error::ApiError(format!("SSE stream error: {}", e)))?;

        // Map events to deserialized StreamEvent with generic fallback
        let stream = events_stream.map(|event_result| match event_result {
            Ok(event) => match event {
                Event::Open => Ok(Self::StreamEvent::not_supported("{}".to_string())),
                Event::Message(msg) => {
                    // Parse msg.data as JSON Value
                    let value: serde_json::Value = serde_json::from_str(&msg.data)
                        .map_err(|e| Error::ApiError(format!("Invalid JSON in SSE data: {}", e)))?;

                    Ok(serde_json::from_value::<Self::StreamEvent>(value)
                        .unwrap_or_else(|_| Self::StreamEvent::not_supported(msg.data)))
                }
            },
            Err(e) => Err(Error::ApiError(format!("SSE event error: {}", e))),
        });

        Ok(Box::pin(stream))
    }
}

/// A common trait for stream events
pub trait StreamEventExt {
    fn not_supported(json: String) -> Self;
}

/// Common fallback for unknown stream events.
#[derive(Debug, Clone)]
pub struct NotSupportedEvent {
    pub json: String,
}

// Blanket implementation for types that can be created from NotSupportedEvent.
impl<T> StreamEventExt for T
where
    T: From<NotSupportedEvent>,
{
    fn not_supported(json: String) -> Self {
        NotSupportedEvent { json }.into()
    }
}
