//! This module provides the OpenAI provider, which implements the `LanguageModel`
//! and `Provider` traits for interacting with the OpenAI API.

pub mod client;
pub mod conversions;
pub mod settings;

use crate::core::client::Client;
use crate::core::language_model::{
    LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, LanguageModelStreamChunkType, ProviderStream,
};
use crate::core::messages::AssistantMessage;
use crate::providers::openai::client::{OpenAIOptions, types};
use crate::providers::openai::settings::{OpenAIProviderSettings, OpenAIProviderSettingsBuilder};
use crate::{
    core::{language_model::LanguageModel, provider::Provider, tools::ToolCallInfo},
    error::Result,
};
use async_trait::async_trait;
use futures::StreamExt;

/// The OpenAI provider.
#[derive(Debug, Clone)]
pub struct OpenAI {
    options: OpenAIOptions,
    settings: OpenAIProviderSettings,
}

impl OpenAI {
    /// Creates a new `OpenAI` provider with the given settings.
    pub fn new(model_name: impl Into<String>) -> Self {
        OpenAIProviderSettingsBuilder::default()
            .model_name(model_name.into())
            .build()
            .expect("Failed to build OpenAIProviderSettings")
    }

    /// OpenAI provider setting builder.
    pub fn builder() -> OpenAIProviderSettingsBuilder {
        OpenAIProviderSettings::builder()
    }
}

impl Provider for OpenAI {}

#[async_trait]
impl LanguageModel for OpenAI {
    fn name(&self) -> String {
        self.options.model.clone()
    }

    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let mut options: OpenAIOptions = options.into();
        options.model = self.options.model.clone();

        self.options = options;

        let response: client::OpenAiResponse = self.send(self.settings.base_url.clone()).await?;

        let mut collected: Vec<LanguageModelResponseContentType> = Vec::new();

        for out in response.output.unwrap_or_default() {
            match out {
                types::MessageItem::OutputMessage { content, .. } => {
                    for c in content {
                        if let types::OutputContent::OutputText { text, .. } = c {
                            collected.push(LanguageModelResponseContentType::new(text))
                        }
                    }
                }
                types::MessageItem::FunctionCall {
                    arguments,
                    name,
                    call_id,
                    ..
                } => {
                    let mut tool_info = ToolCallInfo::new(name);
                    tool_info.id(call_id);
                    tool_info.input(arguments);
                    collected.push(LanguageModelResponseContentType::ToolCall(tool_info));
                }
                _ => (),
            }
        }

        Ok(LanguageModelResponse {
            contents: collected,
            usage: response.usage.map(|usage| usage.into()),
        })
    }

    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        let mut options: OpenAIOptions = options.into();
        options.model = self.options.model.to_string();
        options.stream = Some(true);

        self.options = options;

        let openai_stream = self.send_and_stream(self.settings.base_url.clone()).await?;

        #[derive(Default)]
        struct StreamState {
            completed: bool,
            tool_call_args: std::collections::HashMap<String, String>, // Accumulate tool call args as strings
        }

        let state = std::sync::Arc::new(std::sync::Mutex::new(StreamState::default()));

        let stream = openai_stream.flat_map(move |evt_res| {
            let state = std::sync::Arc::clone(&state);
            let mut state = state.lock().unwrap();
            if state.completed {
                return futures::stream::empty().boxed();
            }
            match evt_res {
                Ok(client::OpenAiStreamEvent::ResponseOutputTextDelta { delta, .. }) => {
                    futures::stream::iter(vec![Ok(vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::Text(delta),
                    )])])
                    .boxed()
                }
                Ok(client::OpenAiStreamEvent::ResponseOutputTextDone { text, .. }) => {
                    state.completed = true;
                    futures::stream::iter(vec![Ok(vec![LanguageModelStreamChunk::Done(
                        AssistantMessage {
                            content: LanguageModelResponseContentType::new(text),
                            usage: None,
                        },
                    )])])
                    .boxed()
                }
                Ok(client::OpenAiStreamEvent::ResponseFunctionCallArgumentsDelta {
                    delta,
                    item_id,
                    ..
                }) => {
                    // Accumulate delta into tool_call_args map
                    let args = state
                        .tool_call_args
                        .entry(item_id.clone())
                        .or_insert_with(String::new);
                    args.push_str(&delta);
                    futures::stream::iter(vec![Ok(vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::ToolCall(delta),
                    )])])
                    .boxed()
                }
                Ok(client::OpenAiStreamEvent::ResponseFunctionCallArgumentsDone {
                    name,
                    item_id,
                    arguments: _,
                    ..
                }) => {
                    // Finalize tool call and emit Done
                    let accumulated = state.tool_call_args.remove(&item_id).unwrap_or_default();
                    let input = serde_json::from_str(&accumulated)
                        .unwrap_or(serde_json::Value::String(accumulated));
                    let mut tool_info = ToolCallInfo::new(name.unwrap_or("unknown".to_string()));
                    tool_info.input(input);
                    futures::stream::iter(vec![Ok(vec![LanguageModelStreamChunk::Done(
                        AssistantMessage {
                            content: LanguageModelResponseContentType::ToolCall(tool_info),
                            usage: None,
                        },
                    )])])
                    .boxed()
                }
                Ok(client::OpenAiStreamEvent::ResponseCompleted { response, .. }) => {
                    state.completed = true;
                    let mut chunks = Vec::new();
                    for out in response.output.unwrap_or_default() {
                        if let types::MessageItem::FunctionCall {
                            call_id,
                            arguments,
                            name,
                            ..
                        } = out
                        {
                            let input = arguments;
                            let mut tool_info = ToolCallInfo::new(name);
                            tool_info.id(call_id);
                            tool_info.input(input);
                            chunks.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                content: LanguageModelResponseContentType::ToolCall(tool_info),
                                usage: response.usage.as_ref().map(|u| u.clone().into()),
                            }));
                        }
                    }
                    futures::stream::iter(vec![Ok(chunks)]).boxed()
                }
                Ok(client::OpenAiStreamEvent::ResponseIncomplete { response, .. }) => {
                    futures::stream::iter(vec![Ok(vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::Incomplete(
                            response
                                .incomplete_details
                                .map(|d| d.reason)
                                .unwrap_or("Unknown".to_string()),
                        ),
                    )])])
                    .boxed()
                }
                Ok(client::OpenAiStreamEvent::ResponseError { code, message, .. }) => {
                    state.completed = true;
                    let reason = format!("{}: {}", code.unwrap_or("unknown".to_string()), message);
                    futures::stream::iter(vec![Ok(vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::Failed(reason),
                    )])])
                    .boxed()
                }
                Ok(evt) => futures::stream::iter(vec![Ok(vec![LanguageModelStreamChunk::Delta(
                    LanguageModelStreamChunkType::NotSupported(format!("{evt:?}")),
                )])])
                .boxed(),
                Err(e) => {
                    state.completed = true;
                    let reason = format!("Stream error: {}", e);
                    futures::stream::iter(vec![Ok(vec![LanguageModelStreamChunk::Delta(
                        LanguageModelStreamChunkType::Failed(reason),
                    )])])
                    .boxed()
                }
            }
        });

        Ok(Box::pin(stream))
    }

    //async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
    //let mut options: OpenAIOptions = options.into();
    //options.model = self.options.model.to_string();
    //options.stream = Some(true);

    //self.options = options;

    //let openai_stream = self.send_and_stream(self.settings.base_url.clone()).await?;

    //#[derive(Default)]
    //struct StreamState {
    //completed: bool,
    //}

    //let stream = openai_stream.scan::<_, Result<Vec<LanguageModelStreamChunk>>, _, _>(
    //StreamState::default(),
    //|state, evt_res| {
    //// If already completed, don't emit anything more
    //if state.completed {
    //return futures::future::ready(None);
    //};

    //futures::future::ready(match evt_res {
    //Ok(client::OpenAiStreamEvent::ResponseOutputTextDelta { delta, .. }) => {
    //Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
    //LanguageModelStreamChunkType::Text(delta),
    //)])))
    //}
    //Ok(client::OpenAiStreamEvent::ResponseOutputTextDone { text, .. }) => {
    //state.completed = true;
    //Some(Ok(Vec::from([LanguageModelStreamChunk::Done(
    //AssistantMessage {
    //content: LanguageModelResponseContentType::new(text),
    //usage: None,
    //},
    //)])))
    //}
    //Ok(client::OpenAiStreamEvent::ResponseFunctionCallArgumentsDelta {
    //delta,
    //..
    //}) => Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
    //LanguageModelStreamChunkType::ToolCall(delta),
    //)]))),
    //Ok(client::OpenAiStreamEvent::ResponseFunctionCallArgumentsDone {
    //name,
    //..
    //}) => Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
    //LanguageModelStreamChunkType::NotSupported(format!(
    //"FunctionCall: {name:?}"
    //)),
    //)]))),
    //Ok(client::OpenAiStreamEvent::ResponseCompleted { response, .. }) => {
    //state.completed = true;

    //let mut collected: Vec<LanguageModelResponseContentType> = Vec::new();

    //for out in response.output.unwrap_or_default() {
    //if let types::MessageItem::FunctionCall {
    //call_id,
    //arguments,
    //name,
    //..
    //} = out
    //{
    //let mut tool_info = ToolCallInfo::new(name);
    //tool_info.id(call_id);
    //tool_info.input(arguments);
    //collected
    //.push(LanguageModelResponseContentType::ToolCall(tool_info));
    //}
    //}

    //Some(Ok(collected
    //.into_iter()
    //.map(|ref c| {
    //LanguageModelStreamChunk::Done(AssistantMessage {
    //content: c.clone(),
    //usage: response
    //.usage
    //.as_ref()
    //.map(|usage| usage.clone().into()),
    //})
    //})
    //.collect()))
    //}
    //Ok(client::OpenAiStreamEvent::ResponseIncomplete { response, .. }) => {
    //Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
    //LanguageModelStreamChunkType::Incomplete(
    //response
    //.incomplete_details
    //.map(|details| details.reason)
    //.unwrap_or("Incomplete with unknown reason".to_string()),
    //),
    //)])))
    //}
    //Ok(client::OpenAiStreamEvent::ResponseError { code, message, .. }) => {
    //state.completed = true;
    //let reason =
    //format!("{}: {}", code.unwrap_or("unknown".to_string()), message);
    //Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
    //LanguageModelStreamChunkType::Failed(reason),
    //)])))
    //}
    //Ok(evt) => Some(Ok(Vec::from([LanguageModelStreamChunk::Delta(
    //LanguageModelStreamChunkType::NotSupported(format!("{evt:?}")),
    //)]))),
    //Err(e) => {
    //state.completed = true;
    //Some(Err(e))
    //}
    //})
    //},
    //);

    //Ok(Box::pin(stream))
    //}
}
