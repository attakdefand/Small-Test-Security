use axum::body::Body;
use axum::http::Request;
use tower::util::ServiceExt; // <-- oneshot lives here with tower 0.5
use super_test_types_rust::http::app;

#[tokio::test]
async fn async_route_ok() {
    let res = app()
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), http::StatusCode::OK);
}
