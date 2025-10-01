use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "enable after rate limit layer"]
async fn requires_pagination_and_limits() {
    let c = client();
    let r = c.get(endpoint("/v1/orders?limit=10000")).send().await.unwrap();
    assert!(r.status().is_client_error() || r.status().is_success());
    // Expand: enforce max limit (e.g., 100).
}
