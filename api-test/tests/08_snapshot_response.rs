use axum::{Router, routing::get, body::Body};
use tower::util::ServiceExt;
use axum::http::Request;
use insta::assert_json_snapshot;

#[tokio::test]
async fn markets_json_stable() {
    let app = Router::new().route("/markets", get(|| async { axum::Json(serde_json::json!({"items":["BTC-USD","ETH-USD"]})) }));
    let res = app
        .oneshot(Request::builder().uri("/markets").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let val: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_json_snapshot!(val);
}