//! Language model implementation for the OpenAI Chat Completions provider.

use crate::core::capabilities::ModelName;
use crate::core::client::LanguageModelClient;
use crate::core::language_model::{
    LanguageModel, LanguageModelOptions, LanguageModelResponse, LanguageModelResponseContentType,
    LanguageModelStreamChunk, LanguageModelStreamChunkType, ProviderStream,
};
use crate::core::messages::AssistantMessage;
use crate::core::tools::ToolCallInfo;
use crate::error::Result;
use crate::providers::openai_chat_completions::OpenAIChatCompletions;
use crate::providers::openai_chat_completions::client::{self, types};
use async_trait::async_trait;
use futures::StreamExt;

#[async_trait]
impl<M: ModelName> LanguageModel for OpenAIChatCompletions<M> {
    fn name(&self) -> String {
        self.options.model.clone()
    }

    async fn generate_text(
        &mut self,
        options: LanguageModelOptions,
    ) -> Result<LanguageModelResponse> {
        let mut options: client::ChatCompletionsOptions = options.into();
        options.model = self.options.model.clone();
        self.options = options;

        let response: types::ChatCompletionsResponse = self.send(&self.settings.base_url).await?;

        // Convert choices to LanguageModelResponse
        let mut contents = Vec::new();

        for choice in response.choices {
            // Handle text content
            if let Some(text) = choice.message.content
                && !text.is_empty()
            {
                contents.push(LanguageModelResponseContentType::Text(text));
            }

            // Handle tool calls
            if let Some(tool_calls) = choice.message.tool_calls {
                for tool_call in tool_calls {
                    let mut tool_info = ToolCallInfo::new(tool_call.function.name);
                    tool_info.id(tool_call.id);
                    tool_info.input(
                        serde_json::from_str(&tool_call.function.arguments)
                            .unwrap_or_else(|_| serde_json::Value::Object(serde_json::Map::new())),
                    );
                    contents.push(LanguageModelResponseContentType::ToolCall(tool_info));
                }
            }
        }

        Ok(LanguageModelResponse {
            contents,
            usage: response.usage.map(|u| u.into()),
        })
    }

    async fn stream_text(&mut self, options: LanguageModelOptions) -> Result<ProviderStream> {
        let mut options: client::ChatCompletionsOptions = options.into();
        options.model = self.options.model.clone();
        options.stream = Some(true);
        // Note: stream_options is not sent to maintain compatibility with
        // OpenAI-compatible providers that don't support this field (e.g., Z.ai)
        // TODO: There should be a correct way to override options for different
        // open ai compatible providers
        self.options = options;

        let stream = self.send_and_stream(&self.settings.base_url).await?;

        // State for accumulating tool calls across chunks
        use std::collections::HashMap;
        let mut accumulated_tool_calls: HashMap<u32, (String, String, String)> = HashMap::new();

        // Map stream events to SDK stream chunks
        let stream = stream.map(move |evt_res| match evt_res {
            Ok(types::ChatCompletionsStreamEvent::Chunk(chunk)) => {
                let mut results = Vec::new();

                for choice in chunk.choices {
                    // Reasoning delta (for reasoning models like o1, DeepSeek R1)
                    if let Some(reasoning) = choice.delta.reasoning_content
                        && !reasoning.is_empty()
                    {
                        results.push(LanguageModelStreamChunk::Delta(
                            LanguageModelStreamChunkType::Reasoning(reasoning),
                        ));
                    }

                    // Text delta
                    if let Some(content) = choice.delta.content
                        && !content.is_empty()
                    {
                        results.push(LanguageModelStreamChunk::Delta(
                            LanguageModelStreamChunkType::Text(content),
                        ));
                    }

                    // Accumulate tool call deltas
                    if let Some(tool_calls) = choice.delta.tool_calls {
                        for tool_call in tool_calls {
                            let entry = accumulated_tool_calls.entry(tool_call.index).or_insert((
                                String::new(),
                                String::new(),
                                String::new(),
                            ));

                            // Accumulate ID
                            if let Some(id) = tool_call.id {
                                entry.0 = id;
                            }

                            // Accumulate name and arguments
                            if let Some(function) = tool_call.function {
                                if let Some(name) = function.name {
                                    entry.1 = name;
                                }
                                if let Some(args) = function.arguments {
                                    entry.2.push_str(&args);
                                    results.push(LanguageModelStreamChunk::Delta(
                                        LanguageModelStreamChunkType::ToolCall(args),
                                    ));
                                }
                            }
                        }
                    }

                    if let Some(finish_reason) = choice.finish_reason {
                        let usage = chunk.usage.clone().map(|u| u.into());

                        match finish_reason.as_str() {
                            "stop" | "length" => {
                                results.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: LanguageModelResponseContentType::Text(String::new()),
                                    usage,
                                }));
                            }
                            "tool_calls" | "function_call" => {
                                // Send accumulated tool calls
                                for (id, name, args) in accumulated_tool_calls.values() {
                                    let mut tool_info = ToolCallInfo::new(name.clone());
                                    tool_info.id(id.clone());
                                    tool_info.input(serde_json::from_str(args).unwrap_or_else(
                                        |_| serde_json::Value::Object(serde_json::Map::new()),
                                    ));
                                    results.push(LanguageModelStreamChunk::Done(
                                        AssistantMessage {
                                            content: LanguageModelResponseContentType::ToolCall(
                                                tool_info,
                                            ),
                                            usage: usage.clone(),
                                        },
                                    ));
                                }
                            }
                            "content_filter" => {
                                results.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: LanguageModelResponseContentType::Text(String::new()),
                                    usage,
                                }));
                                results.push(LanguageModelStreamChunk::Delta(
                                    LanguageModelStreamChunkType::Failed(
                                        "Content filtered".to_string(),
                                    ),
                                ));
                            }
                            // For any unknown finish reason, treat as normal completion
                            _ => {
                                results.push(LanguageModelStreamChunk::Done(AssistantMessage {
                                    content: LanguageModelResponseContentType::Text(String::new()),
                                    usage,
                                }));
                            }
                        }
                    }
                }

                Ok(results)
            }
            Ok(types::ChatCompletionsStreamEvent::Open) => Ok(vec![]),
            Ok(types::ChatCompletionsStreamEvent::Done) => Ok(vec![]),
            Ok(types::ChatCompletionsStreamEvent::Error(e)) => {
                Ok(vec![LanguageModelStreamChunk::Delta(
                    LanguageModelStreamChunkType::Failed(e),
                )])
            }
            Err(e) => Err(e),
        });

        Ok(Box::pin(stream))
    }
}
