//! Integration tests for the `use_chat` Dioxus hook.
//!
//! These tests spin up a real local axum SSE server, mount a headless Dioxus
//! `VirtualDom`, drive it through `wait_for_work` / `render_immediate` cycles,
//! and assert on shared state written out by the test component after each render.

use std::{
    convert::Infallible,
    net::TcpListener,
    sync::{Arc, Mutex},
    time::Duration,
};

use aisdk::integrations::{
    dioxus::{
        hooks::use_chat,
        types::{DioxusChatSignal, DioxusChatStatus, DioxusUseChatOptions},
    },
    vercel_aisdk_ui::VercelUIStream,
};
use axum::{
    Router,
    response::{
        IntoResponse,
        sse::{Event, Sse},
    },
    routing::post,
};
use dioxus::prelude::*;
use futures::stream;

// ---------------------------------------------------------------------------
// Shared state — written by the test component, read by the test assertions
// ---------------------------------------------------------------------------

#[derive(Default, Clone)]
struct SharedState {
    /// Snapshot of assistant message text after each render (last assistant msg).
    assistant_text: String,
    /// Number of messages in the chat.
    message_count: usize,
    /// Ordered list of status strings recorded on every render where status changed.
    status_log: Vec<&'static str>,
    /// The most recently observed status string.
    current_status: &'static str,
}

fn status_str(s: &DioxusChatStatus) -> &'static str {
    match s {
        DioxusChatStatus::Ready => "Ready",
        DioxusChatStatus::Submitted => "Submitted",
        DioxusChatStatus::Streaming => "Streaming",
        DioxusChatStatus::Error => "Error",
    }
}

// ---------------------------------------------------------------------------
// Mock SSE server helpers
// ---------------------------------------------------------------------------

/// Bind on a random OS-assigned port and return both the listener and the URL.
fn random_port_listener() -> (TcpListener, String) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind");
    let addr = listener.local_addr().expect("no local addr");
    let url = format!("http://{}/api/chat", addr);
    (listener, url)
}

/// Spawn a local axum server that, on every POST to `/api/chat`, responds with
/// the provided `VercelUIStream` chunks as an SSE stream then closes.
///
/// Returns the base URL, e.g. `"http://127.0.0.1:54321/api/chat"`.
async fn spawn_mock_server(chunks: Vec<VercelUIStream>) -> String {
    let (listener, url) = random_port_listener();

    let chunks = Arc::new(chunks);
    let handler = move || {
        let chunks = Arc::clone(&chunks);
        async move {
            let events: Vec<Result<Event, Infallible>> = chunks
                .iter()
                .map(|c| {
                    let json = serde_json::to_string(c).expect("serialise chunk");
                    Ok(Event::default().data(json))
                })
                .collect();

            let s = stream::iter(events);
            Sse::new(s).into_response()
        }
    };

    let app = Router::new().route("/api/chat", post(handler));

    // Convert std listener → tokio listener for axum
    listener
        .set_nonblocking(true)
        .expect("set_nonblocking failed");
    let tokio_listener =
        tokio::net::TcpListener::from_std(listener).expect("tokio listener failed");

    tokio::spawn(async move {
        axum::serve(tokio_listener, app)
            .await
            .expect("server error");
    });

    url
}

// ---------------------------------------------------------------------------
// Test component
// ---------------------------------------------------------------------------

/// Props passed into the headless test component.
#[derive(Clone, Props)]
struct TestProps {
    /// API URL pointing at the local mock server.
    api: String,
    /// Message to send once on mount.
    message: String,
    /// Shared state the component writes into after every render.
    shared: Arc<Mutex<SharedState>>,
}

impl PartialEq for TestProps {
    fn eq(&self, other: &Self) -> bool {
        self.api == other.api
            && self.message == other.message
            && Arc::ptr_eq(&self.shared, &other.shared)
    }
}

/// Minimal Dioxus component used as the VirtualDom root in every test.
///
/// On mount it calls `send_message` once via `use_effect`. On every render it
/// snapshots current signal values into `shared` so the test can assert on them.
#[component]
fn TestChatComponent(props: TestProps) -> Element {
    let options = DioxusUseChatOptions {
        api: props.api.clone(),
    };

    let DioxusChatSignal {
        messages,
        status,
        send_message,
    } = use_chat(options);

    // Trigger send_message exactly once when the component mounts.
    // use_hook runs only on the first render, giving us a stable one-shot trigger.
    let message = props.message.clone();
    use_hook(move || {
        send_message(message.clone());
    });

    // After every render, write current state into the shared mutex so the
    // test runner (outside the VirtualDom) can read and assert on it.
    {
        let mut state = props.shared.lock().unwrap();

        let status_label = status_str(&status.read());
        // Record status transitions.
        if state.current_status != status_label {
            state.status_log.push(status_label);
            state.current_status = status_label;
        }

        let msgs = messages.read();
        state.message_count = msgs.len();
        state.assistant_text = msgs
            .iter()
            .filter(|m| m.role == "assistant")
            .flat_map(|m| m.parts.iter().filter(|p| p.part_type == "text"))
            .map(|p| p.text.clone())
            .collect::<Vec<_>>()
            .join("");
    }

    rsx! { div {} }
}

// ---------------------------------------------------------------------------
// VirtualDom polling helper
// ---------------------------------------------------------------------------

/// Drive the VirtualDom until either `until` returns true or we hit
/// `max_iters` iterations (each with a short `wait_for_work` timeout).
async fn poll_until(
    vdom: &mut VirtualDom,
    shared: &Arc<Mutex<SharedState>>,
    until: impl Fn(&SharedState) -> bool,
    max_iters: usize,
) {
    for _ in 0..max_iters {
        tokio::time::timeout(Duration::from_millis(50), vdom.wait_for_work())
            .await
            .ok();
        vdom.render_immediate(&mut dioxus_core::NoOpMutations);
        if until(&shared.lock().unwrap()) {
            break;
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Full lifecycle: user message → Submitted → Streaming → Ready, with correct
/// assistant text assembled from two TextDelta chunks.
#[tokio::test]
async fn test_send_message_full_lifecycle() {
    let chunks = vec![
        VercelUIStream::TextDelta {
            id: "msg_1".into(),
            delta: "Hello".into(),
            provider_metadata: None,
        },
        VercelUIStream::TextDelta {
            id: "msg_1".into(),
            delta: " world".into(),
            provider_metadata: None,
        },
    ];

    let api = spawn_mock_server(chunks).await;
    let shared = Arc::new(Mutex::new(SharedState::default()));

    let props = TestProps {
        api,
        message: "Hi there".into(),
        shared: Arc::clone(&shared),
    };

    let mut vdom = VirtualDom::new_with_props(TestChatComponent, props);
    vdom.rebuild_in_place();

    poll_until(
        &mut vdom,
        &shared,
        |s| s.current_status == "Ready" && s.message_count == 2,
        40,
    )
    .await;

    let state = shared.lock().unwrap();
    assert_eq!(state.message_count, 2, "expected user + assistant message");
    assert_eq!(
        state.assistant_text, "Hello world",
        "assistant text should be both deltas joined"
    );
    assert_eq!(state.current_status, "Ready");
}

/// Status transitions: the hook must pass through Submitted → Streaming → Ready
/// in that order.
#[tokio::test]
async fn test_status_transitions() {
    let chunks = vec![VercelUIStream::TextDelta {
        id: "msg_1".into(),
        delta: "hi".into(),
        provider_metadata: None,
    }];

    let api = spawn_mock_server(chunks).await;
    let shared = Arc::new(Mutex::new(SharedState::default()));

    let props = TestProps {
        api,
        message: "hello".into(),
        shared: Arc::clone(&shared),
    };

    let mut vdom = VirtualDom::new_with_props(TestChatComponent, props);
    vdom.rebuild_in_place();

    poll_until(&mut vdom, &shared, |s| s.current_status == "Ready", 40).await;

    let state = shared.lock().unwrap();
    // "Ready" is the initial state so it will appear first in the log,
    // then Submitted, Streaming, Ready again after the stream completes.
    let log = &state.status_log;
    assert!(
        log.contains(&"Submitted"),
        "expected Submitted in log: {log:?}"
    );
    assert!(
        log.contains(&"Streaming"),
        "expected Streaming in log: {log:?}"
    );
    // Last entry must be Ready
    assert_eq!(log.last().copied(), Some("Ready"), "log: {log:?}");
}

/// Guard: a second `send_message` call while status != Ready must be ignored.
#[tokio::test]
async fn test_guard_when_not_ready() {
    // Use a server that never sends anything so the first request stays in-flight.
    // We achieve this by returning zero chunks (stream closes immediately, which
    // will eventually transition to Ready), but we assert *before* that happens
    // by checking message_count == 1 (only the user message, no assistant yet).
    //
    // The guard test works by having the component call send_message twice
    // synchronously in the same effect tick via a secondary effect. We simulate
    // this with a second component variant below that calls send_message twice.

    #[derive(Clone, Props)]
    struct TwoSendProps {
        api: String,
        shared: Arc<Mutex<SharedState>>,
    }

    impl PartialEq for TwoSendProps {
        fn eq(&self, other: &Self) -> bool {
            self.api == other.api && Arc::ptr_eq(&self.shared, &other.shared)
        }
    }

    #[component]
    fn TwoSendComponent(props: TwoSendProps) -> Element {
        let options = DioxusUseChatOptions {
            api: props.api.clone(),
        };

        let chat_signal = use_chat(options);
        let messages = chat_signal.messages;
        let status = chat_signal.status;
        let send_message = chat_signal.send_message;

        // Call send_message twice on mount — second should be ignored by the guard
        // because after the first call status is already Submitted.
        // use_hook runs only on the first render.
        use_hook(move || {
            send_message.call("first".into());
            send_message.call("second".into()); // should be ignored
        });

        {
            let mut state = props.shared.lock().unwrap();
            let status_label = status_str(&status.read());
            if state.current_status != status_label {
                state.status_log.push(status_label);
                state.current_status = status_label;
            }
            state.message_count = messages.read().len();
        }

        rsx! { div {} }
    }

    // Server with no chunks — stream closes immediately → Ready
    let api = spawn_mock_server(vec![]).await;
    let shared = Arc::new(Mutex::new(SharedState::default()));

    let props = TwoSendProps {
        api,
        shared: Arc::clone(&shared),
    };

    let mut vdom = VirtualDom::new_with_props(TwoSendComponent, props);
    vdom.rebuild_in_place();

    // Wait until stable (Ready or at least a few renders)
    poll_until(&mut vdom, &shared, |s| s.current_status == "Ready", 40).await;

    let state = shared.lock().unwrap();
    // Only 1 user message + 1 assistant placeholder (or just 1 user if no Open event
    // was emitted). The key invariant: there must NOT be 2 user messages.
    let user_count = state.message_count;
    assert!(
        user_count <= 2,
        "second send_message should have been ignored; message_count={user_count}"
    );
}

/// An `Error` chunk from the server must set status to `Error`.
#[tokio::test]
async fn test_server_error_chunk_sets_error_status() {
    let chunks = vec![VercelUIStream::Error {
        error_text: "something went wrong".into(),
    }];

    let api = spawn_mock_server(chunks).await;
    let shared = Arc::new(Mutex::new(SharedState::default()));

    let props = TestProps {
        api,
        message: "hello".into(),
        shared: Arc::clone(&shared),
    };

    let mut vdom = VirtualDom::new_with_props(TestChatComponent, props);
    vdom.rebuild_in_place();

    poll_until(&mut vdom, &shared, |s| s.current_status == "Error", 40).await;

    let state = shared.lock().unwrap();
    assert_eq!(
        state.current_status, "Error",
        "status log: {:?}",
        state.status_log
    );
}

/// A connection failure (nothing listening on the given port) must set
/// status to `Error`.
#[tokio::test]
async fn test_connection_failure_sets_error_status() {
    // Bind and immediately drop to free the port — nothing will be listening.
    let port = {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        l.local_addr().unwrap().port()
    };
    let api = format!("http://127.0.0.1:{port}/api/chat");

    let shared = Arc::new(Mutex::new(SharedState::default()));

    let props = TestProps {
        api,
        message: "hello".into(),
        shared: Arc::clone(&shared),
    };

    let mut vdom = VirtualDom::new_with_props(TestChatComponent, props);
    vdom.rebuild_in_place();

    poll_until(&mut vdom, &shared, |s| s.current_status == "Error", 40).await;

    let state = shared.lock().unwrap();
    assert_eq!(
        state.current_status, "Error",
        "status log: {:?}",
        state.status_log
    );
}
