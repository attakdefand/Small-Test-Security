mod common;
use common::*;
use http::{Method, StatusCode};

#[tokio::test]
async fn cors_preflight_ok() {
    let url = base_url().join(&path("QUOTES_PATH")).unwrap();
    let origin = std::env::var("PUBLIC_ORIGIN").expect("PUBLIC_ORIGIN");
    let res = client()
        .request(Method::OPTIONS, url)
        .header("Origin", origin)
        .header("Access-Control-Request-Method", "GET")
        .send().await.unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT, "preflight status");
    let h = res.headers();
    assert!(h.get("access-control-allow-origin").is_some());
    assert!(h.get("access-control-allow-methods").is_some());
}
