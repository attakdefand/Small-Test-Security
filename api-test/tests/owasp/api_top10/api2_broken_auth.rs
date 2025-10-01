use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "needs auth endpoints"]
async fn refresh_tokens_rotate() {
    let c = client();
    let r = c.post(endpoint("/v1/auth/refresh"))
        .header("Authorization", "Bearer OLD_REFRESH")
        .send().await.unwrap();
    assert!(r.status().is_client_error() || r.status().is_success());
    // Expand: ensure old token becomes invalid after rotation.
}
