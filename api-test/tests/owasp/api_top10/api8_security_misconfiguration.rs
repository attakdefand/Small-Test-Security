use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "needs header/CSP config"]
async fn csp_header_present() {
    let c = crate::owasp::common::client();
    let r = c.get(endpoint("/health")).send().await.unwrap();
    assert!(r.headers().get("content-security-policy").is_some(), "CSP required");
}
