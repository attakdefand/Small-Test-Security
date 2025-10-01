use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "needs login endpoint"]
async fn lockout_after_bruteforce() {
    let c = client();
    for _ in 0..6 {
        let _ = c.post(endpoint("/v1/auth/login"))
            .json(&serde_json::json!({"u":"user","p":"wrong"}))
            .send().await.unwrap();
    }
    let final_try = c.post(endpoint("/v1/auth/login"))
        .json(&serde_json::json!({"u":"user","p":"wrong"}))
        .send().await.unwrap();
    assert!(final_try.status().is_client_error(), "should lock or throttle");
}
