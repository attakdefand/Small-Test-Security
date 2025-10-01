use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "enable when withdrawal flow exists"]
async fn withdrawal_flow_requires_hardening_steps() {
    let c = client();
    let r = c.post(endpoint("/v1/withdrawals"))
        .json(&serde_json::json!({"asset":"USDT","amount":"1000000","address":"..." }))
        .header("Authorization","Bearer USER")
        .send().await.unwrap();
    assert!(r.status().is_client_error() || r.status().is_success());
    // Expand: require 2FA, velocity limits, review state.
}
