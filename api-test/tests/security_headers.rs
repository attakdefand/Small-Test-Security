mod common;
use common::*;
use http::HeaderMap;

#[tokio::test]
async fn baseline_security_headers_present() {
    let url = base_url().join("/").unwrap();
    let res = client().get(url).send().await.unwrap();
    let headers: &HeaderMap = res.headers();

    assert!(headers.get("x-content-type-options").is_some(), "nosniff missing");
    assert!(headers.get("referrer-policy").is_some(), "referrer-policy missing");
    // Optionally:
    // assert!(headers.get("strict-transport-security").is_some(), "HSTS missing");
    // assert!(headers.get("content-security-policy").is_some(), "CSP missing");
}
