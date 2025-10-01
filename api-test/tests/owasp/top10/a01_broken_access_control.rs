use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "needs running API with seeded users/tenants"]
async fn forbid_cross_tenant_read() {
    // Example: user from tenant A must not read tenant B resource.
    let c = client();
    // Typically you'd issue a token for tenant A; we demo with a placeholder header.
    let resp = c
        .get(endpoint("/v1/tenants/b/resources/123"))
        .header("Authorization", "Bearer TENANT_A_TOKEN")
        .send()
        .await
        .expect("http ok");
    assert!(resp.status().is_client_error(), "should NOT read foreign tenant");
}
