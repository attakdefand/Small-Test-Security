use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "needs CORS layer in gateway"]
async fn cors_is_restricted() {
    let c = client();
    let resp = c.get(endpoint("/health")).send().await.unwrap();
    // If you reflect origins, you'll fail this; you want an allowlist.
    if let Some(v) = resp.headers().get("access-control-allow-origin") {
        let s = v.to_str().unwrap_or_default();
        assert_ne!(s, "*", "CORS must NOT be wildcard");
    }
}
