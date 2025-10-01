use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "needs role matrix"]
async fn admin_only_action_denied_for_user_role() {
    let c = client();
    let r = c.post(endpoint("/v1/admin/markets/halt"))
        .header("Authorization","Bearer NORMAL_USER")
        .send().await.unwrap();
    assert!(r.status().is_client_error(), "non-admin must be denied");
}
