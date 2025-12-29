//! Client implementation for the Google provider.
use crate::core::client::Client;
use crate::error::{Error, Result};
use crate::providers::google::{Google, ModelName};
use derive_builder::Builder;
use reqwest::header::CONTENT_TYPE;
use reqwest_eventsource::Event;
use serde::{Deserialize, Serialize};

pub(crate) mod types;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Builder)]
#[builder(setter(into), build_fn(error = "Error"))]
pub(crate) struct GoogleOptions {
    pub(crate) model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) request: Option<types::GenerateContentRequest>,
    #[serde(skip)]
    #[builder(default)]
    pub(crate) streaming: bool,
}

impl GoogleOptions {
    pub(crate) fn builder() -> GoogleOptionsBuilder {
        GoogleOptionsBuilder::default()
    }
}

impl<M: ModelName> Client for Google<M> {
    type Response = types::GenerateContentResponse;
    type StreamEvent = types::GoogleStreamEvent;

    fn path(&self) -> String {
        if self.options.streaming {
            return format!(
                "/v1beta/models/{}:streamGenerateContent",
                self.options.model
            );
        };
        format!("/v1beta/models/{}:generateContent", self.options.model)
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert("x-goog-api-key", self.settings.api_key.parse().unwrap());
        headers
    }

    fn query_params(&self) -> Vec<(&str, &str)> {
        if self.options.streaming {
            return vec![("alt", "sse")];
        }
        Vec::new()
    }

    fn body(&self) -> reqwest::Body {
        if let Some(request) = &self.options.request {
            let body = serde_json::to_string(request).unwrap();
            return reqwest::Body::from(body);
        };
        reqwest::Body::from("{}")
    }

    fn parse_stream_sse(
        event: std::result::Result<Event, reqwest_eventsource::Error>,
    ) -> Result<Self::StreamEvent> {
        match event {
            Ok(event) => match event {
                Event::Open => Ok(types::GoogleStreamEvent::NotSupported("{}".to_string())),
                Event::Message(msg) => {
                    let value: serde_json::Value =
                        serde_json::from_str(&msg.data).map_err(|e| Error::ApiError {
                            status_code: None,
                            details: format!("Invalid JSON in SSE data: {}", e),
                        })?;

                    Ok(
                        serde_json::from_value::<types::GenerateContentResponse>(value)
                            .map(types::GoogleStreamEvent::Response)
                            .unwrap_or(types::GoogleStreamEvent::NotSupported(msg.data)),
                    )
                }
            },
            Err(e) => {
                // Extract status code if it's an InvalidStatusCode error
                let status_code = match &e {
                    reqwest_eventsource::Error::InvalidStatusCode(status, _) => Some(*status),
                    _ => None,
                };
                Err(Error::ApiError {
                    status_code,
                    details: e.to_string(),
                })
            }
        }
    }

    fn end_stream(event: &Self::StreamEvent) -> bool {
        match event {
            types::GoogleStreamEvent::Response(resp) => {
                resp.candidates.iter().any(|c| c.finish_reason.is_some())
            }
            _ => false,
        }
    }
}
