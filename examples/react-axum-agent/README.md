# React + Axum Agent Example

A full-stack AI chat app built with **React** and **Axum (Rust)**, demonstrating streaming responses, tool calling with `aisdk.rs`, and integration with the [Vercel AI SDK UI](https://ai-sdk.dev/docs/ai-sdk-ui/chatbot).

![aisdkdemo](https://github.com/user-attachments/assets/e4b6f068-ff81-4190-a80a-9d3040c1a9bf)

## What It Shows

* **Tool calling with `#[tool]`**:
  Define callable functions with structured inputs/outputs.

* **Streaming + Tool progress events**:
  Stream model responses while tracking tool execution in real time.

* **Axum SSE integration**:
  Use `AxumSseResponse` for clean state handling and streaming responses.

* **React + [Vercel AI SDK](https://ai-sdk.dev/docs/ai-sdk-ui/chatbot) (`useChat`)**
  Text + ToolCall Streaming with seamless frontend integration.

## Structure

```
react-axum-agent/
├── backend/          # Rust Axum server
│   ├── src/main.rs   # Server + web_fetch tool
│   └── Cargo.toml
└── frontend/         # React + Vite
    ├── src/
    │   ├── App.tsx              # Main chat UI
    │   └── components/
    └── package.json
```

## Running

```bash
# Backend
cd backend
cp .env.example .env  # Add API_KEY
cargo run

# Frontend (new terminal)
cd frontend
pnpm install
pnpm dev
```

Open [http://localhost:5173](http://localhost:5173)
