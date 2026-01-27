//! This module provides the client for interacting with the AI providers.
//! It is a thin wrapper around the `reqwest` crate.

use crate::error::{Error, Result};
use futures::Stream;
use futures::StreamExt;
use reqwest;
use reqwest::IntoUrl;
use reqwest::Url;
use reqwest_eventsource::{Event, RequestBuilderExt};
use serde::de::DeserializeOwned;
use std::pin::Pin;

/// Joins a base URL with a path, handling trailing/leading slashes automatically.
///
/// This function normalizes the URL components to ensure proper joining:
/// - Strips trailing slashes from base_url
/// - Strips leading slashes from path
/// - Joins them with a single slash
///
/// # Examples
///
/// ```ignore
/// // All of these produce "https://api.example.com/v1/chat/completions"
/// join_url("https://api.example.com/v1", "chat/completions");
/// join_url("https://api.example.com/v1/", "chat/completions");
/// join_url("https://api.example.com/v1", "/chat/completions");
/// join_url("https://api.example.com/v1/", "/chat/completions");
/// ```
pub(crate) fn join_url(base_url: impl IntoUrl, path: &str) -> Result<Url> {
    let base_url = base_url
        .into_url()
        .map_err(|_| Error::InvalidInput("Invalid base URL".into()))?;

    // Normalize: strip trailing slashes from base, strip leading slashes from path
    let base_str = base_url.as_str().trim_end_matches('/');
    let path_str = path.trim_start_matches('/');

    // Join with a single slash
    let full_url = format!("{}/{}", base_str, path_str);

    Url::parse(&full_url)
        .map_err(|_| Error::InvalidInput("Failed to join base URL and path".into()))
}

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

        let url = join_url(base_url, &self.path())?;

        let max_retries = 5;
        let mut retry_count = 0;
        let mut wait_time = std::time::Duration::from_secs(1);

        loop {
            let resp = client
                .request(self.method(), url.clone())
                .headers(self.headers())
                .query(&self.query_params())
                .body(self.body())
                .send()
                .await
                .map_err(|e| Error::ApiError {
                    status_code: e.status(),
                    details: e.to_string(),
                })?;

            let status = resp.status();
            let resp_text = resp.text().await.map_err(|e| Error::ApiError {
                status_code: e.status(),
                details: format!("Failed to read response: {}", e),
            })?;

            if status.is_success() {
                return serde_json::from_str(&resp_text).map_err(|e| Error::ApiError {
                    status_code: Some(status),
                    details: format!("Failed to parse response: {}", e),
                });
            }

            // Check for 429 rate limit error and retry
            if status == reqwest::StatusCode::TOO_MANY_REQUESTS && retry_count < max_retries {
                retry_count += 1;
                tokio::time::sleep(wait_time).await;
                wait_time *= 2; // Exponential backoff
                continue;
            }

            return Err(Error::ApiError {
                status_code: Some(status),
                details: resp_text,
            });
        }
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

        let url = join_url(base_url, &self.path())?;

        // Establish the event source stream directly
        // Note: Status code errors (including 429) will be surfaced as stream events
        // and should be handled by retry logic in the provider's stream_text() method
        let events_stream = client
            .request(self.method(), url.clone())
            .headers(self.headers())
            .query(&self.query_params())
            .body(self.body())
            .eventsource()
            .map_err(|e| Error::ApiError {
                status_code: None,
                details: format!("SSE stream error: {}", e),
            })?;

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
