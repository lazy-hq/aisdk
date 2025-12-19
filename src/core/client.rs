//! This module provides the client for interacting with the AI providers.
//! It is a thin wrapper around the `reqwest` crate.

use crate::error::{Error, Result};
use futures::Stream;
use futures::StreamExt;
use reqwest;
use reqwest::IntoUrl;
use reqwest_eventsource::{Event, RequestBuilderExt};
use serde::de::DeserializeOwned;
use std::pin::Pin;

#[allow(dead_code)]
pub(crate) trait Client {
    type Response: DeserializeOwned + std::fmt::Debug + Clone;
    type StreamEvent: DeserializeOwned + std::fmt::Debug + Clone;

    fn path(&self) -> String;
    fn method(&self) -> reqwest::Method;
    fn query_params(&self) -> Vec<(&str, &str)>;
    fn body(&self) -> reqwest::Body;
    fn headers(&self) -> reqwest::header::HeaderMap;

    async fn send(&self, base_url: impl IntoUrl) -> Result<Self::Response> {
        let client = reqwest::Client::new();

        let base_url = base_url
            .into_url()
            .map_err(|_| Error::InvalidInput("Invalid base URL".into()))?;

        let url = base_url
            .join(&self.path())
            .map_err(|_| Error::InvalidInput("Failed to join base URL and path".into()))?;

        let resp = client
            .request(self.method(), url)
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

    /// Parses an SSE event into a StreamEvent ( ProviderStreamEvent )
    fn parse_stream_sse(
        event: std::result::Result<Event, reqwest_eventsource::Error>,
    ) -> Result<Self::StreamEvent>;

    /// Returns true to mark the stream as ended
    fn end_stream(event: &Self::StreamEvent) -> bool;

    async fn send_and_stream(
        &self,
        base_url: impl IntoUrl,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Self::StreamEvent>> + Send>>>
    where
        Self::StreamEvent: Send + 'static,
        Self: Sync,
    {
        let client = reqwest::Client::new();

        let base_url = base_url
            .into_url()
            .map_err(|_| Error::InvalidInput("Invalid base URL".into()))?;

        let url = base_url
            .join(&self.path())
            .map_err(|_| Error::InvalidInput("Failed to join base URL and path".into()))?;

        let events_stream = client
            .request(self.method(), url)
            .headers(self.headers())
            .query(&self.query_params())
            .body(self.body())
            .eventsource()
            .map_err(|e| Error::ApiError(format!("SSE stream error: {}", e)))?;

        // Map events to deserialized StreamEvent ( ProviderStreamEvent )
        let mapped_stream = events_stream.map(|event_result| Self::parse_stream_sse(event_result));

        // State that indicates if the stream has ended
        let ended = std::sync::Arc::new(std::sync::Mutex::new(false));

        // Scan to end or mark the stream as ended
        let stream = mapped_stream.scan(ended, |ended, res| {
            let mut ended = ended.lock().unwrap();

            if *ended {
                return futures::future::ready(None); // Stop the stream after end event
            }

            *ended = res.as_ref().map_or(true, |evt| Self::end_stream(evt)); // Mark the stream as ended on api error or end event

            futures::future::ready(Some(res)) // Emit the event
        });

        Ok(Box::pin(stream))
    }
}
