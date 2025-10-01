use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "needs outbound call proxy"]
async fn metadata_ip_is_blocked() {
    let c = client();
    // Endpoint that fetches a URL should reject link-local / 169.254.169.254 etc.
    let resp = c.post(endpoint("/v1/fetch"))
        .json(&serde_json::json!({"url":"http://169.254.169.254/latest/meta-data"}))
        .send().await.unwrap();
    assert!(resp.status().is_client_error(), "should block metadata host");
}
