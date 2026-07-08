//! Client implementation for the Google provider.
use crate::core::client::{EmbeddingClient, LanguageModelClient, merge_body, merge_headers};
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
    #[serde(skip)]
    #[builder(default)]
    pub(crate) extra_body: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(skip)]
    #[builder(default)]
    pub(crate) extra_headers: Option<std::collections::HashMap<String, String>>,
}

impl GoogleOptions {
    pub(crate) fn builder() -> GoogleOptionsBuilder {
        GoogleOptionsBuilder::default()
    }
}

#[derive(Builder, Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GoogleEmbeddingOptions {
    pub(crate) model: String,
    pub(crate) requests: Vec<types::EmbedContentRequest>,
    #[serde(skip)]
    #[builder(default)]
    pub(crate) extra_body: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(skip)]
    #[builder(default)]
    pub(crate) extra_headers: Option<std::collections::HashMap<String, String>>,
}

impl<M: ModelName> LanguageModelClient for Google<M> {
    type Response = types::GenerateContentResponse;
    type StreamEvent = types::GoogleStreamEvent;

    fn path(&self) -> String {
        if let Some(ref path) = self.settings.path {
            return path.clone();
        }
        if self.lm_options.streaming {
            return format!(
                "/v1beta/models/{}:streamGenerateContent",
                self.lm_options.model
            );
        };
        format!("/v1beta/models/{}:generateContent", self.lm_options.model)
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn headers(&self) -> Result<reqwest::header::HeaderMap> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert("x-goog-api-key", self.settings.api_key.parse().unwrap());
        merge_headers(
            headers,
            self.settings.headers.as_ref(),
            self.lm_options.extra_headers.as_ref(),
        )
    }

    fn query_params(&self) -> Vec<(&str, &str)> {
        if self.lm_options.streaming {
            return vec![("alt", "sse")];
        }
        Vec::new()
    }

    fn body(&self) -> Result<reqwest::Body> {
        if let Some(request) = &self.lm_options.request {
            return merge_body(
                request,
                self.settings.body.as_ref(),
                self.lm_options.extra_body.as_ref(),
            );
        }

        merge_body(
            &serde_json::Value::Object(serde_json::Map::new()),
            self.settings.body.as_ref(),
            self.lm_options.extra_body.as_ref(),
        )
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
                            details: format!("Invalid JSON in SSE data: {e}"),
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

impl<M: ModelName> EmbeddingClient for Google<M> {
    type Response = types::BatchEmbedContentsResponse;

    fn path(&self) -> String {
        format!(
            "/v1beta/models/{}:batchEmbedContents",
            self.embedding_options.model
        )
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn headers(&self) -> Result<reqwest::header::HeaderMap> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert("x-goog-api-key", self.settings.api_key.parse().unwrap());
        merge_headers(
            headers,
            self.settings.headers.as_ref(),
            self.embedding_options.extra_headers.as_ref(),
        )
    }

    fn query_params(&self) -> Vec<(&str, &str)> {
        Vec::new()
    }

    fn body(&self) -> Result<reqwest::Body> {
        let request = types::BatchEmbedContentsRequest {
            requests: self.embedding_options.requests.clone(),
        };
        merge_body(
            &request,
            self.settings.body.as_ref(),
            self.embedding_options.extra_body.as_ref(),
        )
    }
}
