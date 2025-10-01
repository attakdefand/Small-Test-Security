//! Test helpers for integration/router/e2e tests.
//! - `demo_router()` tiny app with /health
//! - `spawn_http(app)` bind on 127.0.0.1:0 and serve `app`
//! - `start_test_server()` convenience to spawn `demo_router()`

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Tiny demo router used by router/e2e tests.
pub fn demo_router() -> Router {
    Router::new().route("/health", get(|| async { axum::http::StatusCode::OK }))
}

/// Spawn an Axum server for the provided `app` on 127.0.0.1:0 (ephemeral port).
/// Returns the bound address and a JoinHandle that keeps the server alive.
pub async fn spawn_http(app: Router) -> (SocketAddr, tokio::task::JoinHandle<()>) {
    // Bind with Tokio's TcpListener (required by axum::serve)
    let listener = TcpListener::bind(("127.0.0.1", 0))
        .await
        .expect("bind 127.0.0.1:0");
    let addr = listener.local_addr().expect("get local addr");

    // Spawn the server future
    let handle = tokio::spawn(async move {
        if let Err(err) = axum::serve(listener, app).await {
            eprintln!("test server error: {err}");
        }
    });

    (addr, handle)
}

/// Start a minimal Axum server (using `demo_router`) on an ephemeral port.
/// Returns the bound addr and a JoinHandle so tests can keep it alive.
///
/// Example:
/// ```ignore
/// let (addr, _handle) = start_test_server().await;
/// let url = format!("http://{addr}/health");
/// ```
pub async fn start_test_server() -> (SocketAddr, tokio::task::JoinHandle<()>) {
    let app = demo_router();
    spawn_http(app).await
}
