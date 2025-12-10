//! This module provides the client for interacting with the AI providers.
//! It is a thin wrapper around the `reqwest` crate.

use crate::error::{Error, Result};
use futures::Stream;
use futures::StreamExt;
use reqwest;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::pin::Pin;

#[allow(dead_code)]
pub(crate) trait Client {
    type Response: DeserializeOwned + std::fmt::Debug + Clone;
    type StreamEvent: DeserializeOwned;

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
        let resp = client
            .request(self.method(), base_url)
            .headers(self.headers())
            .query(&self.query_params())
            .body(self.body())
            .send()
            .await
            .and_then(|response| response.error_for_status())
            .map_err(|e| Error::ApiError(e.to_string()))?;

        let stream = resp
            .bytes_stream()
            .scan(String::new(), |buffer, result| {
                let chunk = match result {
                    Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                    Err(e) => {
                        return futures::future::ready(Some(vec![Err(Error::ApiError(
                            e.to_string(),
                        ))]));
                    }
                };

                buffer.push_str(&chunk);

                let mut events = Vec::new();

                while let Some(pos) = buffer.find("\n\n") {
                    let message = buffer[..pos].to_string();
                    *buffer = buffer[pos + 2..].to_string();

                    for line in message.lines() {
                        let line = line.trim();
                        if let Some(event) = Self::parse_sse_stream(line) {
                            events.push(event);
                        }
                    }
                }

                futures::future::ready(if events.is_empty() {
                    None
                } else {
                    Some(events)
                })
            })
            .map(futures::stream::iter)
            .flatten();

        Ok(Box::pin(stream))
    }

    fn parse_sse_stream(text: &str) -> Option<Result<Self::StreamEvent>>;
}
