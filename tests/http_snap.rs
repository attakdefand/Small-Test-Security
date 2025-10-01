use axum::body::{to_bytes, Body};   // <-- import to_bytes here
use axum::http::Request;
use tower::util::ServiceExt;
use super_test_types_rust::http::app;

#[test]
fn health_snapshot() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let res = app()
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(res.status(), http::StatusCode::OK);

        // axum 0.7: to_bytes(body, limit)
        let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let json = String::from_utf8(bytes.to_vec()).unwrap();

        insta::assert_json_snapshot!("health_v1", json);
    });
}
