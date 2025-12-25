//! Integration with Axum.

use crate::integrations::vercel_aisdk_ui::VercelUIStreamBuilder;
use axum::response::Sse;
use axum::response::sse::{Event, KeepAliveStream};
use futures::StreamExt;

/// Type alias for the Axum SSE response with boxed stream for trait implementations.
pub type AxumSseResponse = Sse<
    KeepAliveStream<
        std::pin::Pin<Box<dyn futures::Stream<Item = crate::Result<Event>> + Send + 'static>>,
    >,
>;

/// Implement From trait for StreamTextResponse to AxumSseResponse.
impl From<crate::core::StreamTextResponse> for AxumSseResponse {
    fn from(response: crate::core::StreamTextResponse) -> Self {
        response
            .to_axum_vercel_ui_stream()
            .send_reasoning()
            .send_start()
            .send_finish()
            .build()
    }
}

impl crate::core::StreamTextResponse {
    /// Creates a builder for configuring a Vercel AI SDK UI compatible stream response from this `StreamTextResponse`.
    ///
    /// Allows configuration of options such as sending reasoning chunks, start signals, finish signals,
    /// and custom message ID generation. The final build produces an Axum SSE response containing
    /// Vercel UI chunks (e.g., text deltas, reasoning deltas).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let sse_response = response
    ///     .to_axum_vercel_ui_stream()
    ///     .send_reasoning() // Enable reasoning chunks
    ///     .send_start()    // Enable start signals
    ///     .send_finish()  // Enable finish signals
    ///     .build();
    ///     
    /// ```
    ///
    /// # Returns
    /// A `VercelUIStreamBuilder` for configuring and building the Axum SSE stream.
    pub fn to_axum_vercel_ui_stream(self) -> VercelUIStreamBuilder<Self, AxumSseResponse> {
        VercelUIStreamBuilder::new(self, |context, options| {
            let ui_stream = context.into_vercel_ui_stream(options);

            let mapped_stream = ui_stream.map(|result| match result {
                Ok(chunk) => {
                    let json = serde_json::to_string(&chunk).map_err(|e| {
                        crate::error::Error::Other(format!("JSON serialization error: {}", e))
                    })?;
                    Ok(axum::response::sse::Event::default().data(json))
                }
                Err(e) => Err(e),
            });

            let boxed_stream = Box::pin(mapped_stream)
                as std::pin::Pin<
                    Box<dyn futures::Stream<Item = crate::Result<Event>> + Send + 'static>,
                >;

            axum::response::Sse::new(boxed_stream).keep_alive(axum::response::sse::KeepAlive::new())
        })
    }
}
