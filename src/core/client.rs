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
    fn streaming_body(&self) -> reqwest::Body;

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
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Self::StreamEvent>> + Send>>> {
        let client = reqwest::Client::new();
        let base_url = base_url.join(self.path()).expect("Invalid base URL");
        let resp = client
            .request(self.method(), base_url)
            .headers(self.headers())
            .query(&self.query_params())
            .body(self.streaming_body())
            .send()
            .await
            .and_then(|response| response.error_for_status())
            .map_err(|e| Error::ApiError(e.to_string()))?;

        let stream = resp.bytes_stream().map(|result| {
            result
                .map_err(|e| Error::ApiError(e.to_string()))
                .and_then(|bytes| {
                    let text = String::from_utf8(bytes.to_vec())
                        .map_err(|e| Error::ApiError(format!("UTF-8 error: {}", e)))?;
                    let mut events = vec![];
                    for message in text.split("\n\n") {
                        if let Some(data_line) =
                            message.lines().find(|line| line.starts_with("data: "))
                        {
                            let json_str = &data_line[6..];
                            if json_str.trim().is_empty() || json_str.trim() == "[DONE]" {
                                continue;
                            }
                            let event: Self::StreamEvent = serde_json::from_str(json_str)
                                .map_err(|e| Error::ApiError(format!("JSON parse error: {}", e)))?;
                            events.push(event);
                        }
                    }
                    if events.is_empty() {
                        Err(Error::ApiError("No events parsed".to_string()))
                    } else {
                        Ok(events.into_iter().next().unwrap())
                    }
                })
        });

        Ok(Box::pin(stream))
    }
}
