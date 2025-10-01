use axum::{Router, routing::get, body::Body};
use tower::util::ServiceExt;
use axum::http::Request;

#[tokio::test]
async fn health_ok() {
    let app = Router::new().route("/health", get(|| async { "ok" }));
    let res = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), http::StatusCode::OK);
}