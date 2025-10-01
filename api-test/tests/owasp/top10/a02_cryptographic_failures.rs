use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "enable when TLS/headers are wired"]
async fn security_headers_present() {
    let c = client();
    let resp = c.get(endpoint("/health")).send().await.unwrap();
    // Basic examples; expand as you add layers
    let headers = resp.headers();
    assert!(headers.get("x-content-type-options").is_some(), "nosniff required");
    assert!(headers.get("referrer-policy").is_some(), "referrer policy required");
}
