use aisdk::{
    core::{LanguageModelRequest, Tool},
    integrations::{axum::AxumSseResponse, vercel_aisdk_ui::VercelUIRequest},
    macros::tool,
    providers::Google,
};

// Example handler function
async fn chat_handler(axum::Json(request): axum::Json<VercelUIRequest>) -> AxumSseResponse {
    // Convert the Message sent by the frontend to AISDK.rs Messages
    let messages = request.into();

    // Generate streaming response
    let response = LanguageModelRequest::builder()
        .model(Google::gemini_2_5_flash())
        .system("You are helpful assistant. who explains topics about the aisdk.rs library.")
        .messages(messages)
        .with_tool(web_fetch())
        .build()
        .stream_text()
        .await;

    match response {
        Ok(response) => response.into(), // Convert to Axum SSE response (Vercel UI compatible)
        Err(err) => panic!("Error: {:?}", err),
    }
}

#[tool]
/// Fetches content from a URL on https://aisdk.rs/docs/ to get the latest version of the SDK documentation
/// Use .mdx extension for markdown format
/// example: https://aisdk.rs/docs/concepts/generating-text.mdx or
/// just https://aisdk.rs/docs/concepts/generating-text to get the html content
async fn web_fetch(url: String) -> Tool {
    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let body = response.text().await.map_err(|e| e.to_string())?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    // load .env
    dotenv::dotenv().ok();

    let app = axum::Router::new()
        .route("/api/chat", axum::routing::post(chat_handler))
        .layer(tower_http::cors::CorsLayer::permissive());

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
