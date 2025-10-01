use axum::{routing::get, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Health { status: &'static str, version: &'static str }

pub fn app() -> Router {
    Router::new().route("/health", get(|| async {
        axum::Json(Health { status: "ok", version: "v1" })
    }))
}
