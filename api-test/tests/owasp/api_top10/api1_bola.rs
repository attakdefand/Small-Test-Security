use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "needs seeded users + auth"]
async fn owner_only_get_object() {
    let c = client();
    let resp = c.get(endpoint("/v1/orders/abcd-foreign"))
        .header("Authorization", "Bearer USER_A")
        .send().await.unwrap();
    assert!(resp.status().is_client_error(), "BOLA: must not read foreign object");
}
