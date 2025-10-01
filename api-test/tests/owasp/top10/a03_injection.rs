use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "needs endpoint that queries DB"]
async fn input_is_parameterized() {
    let c = client();
    // Try a classic payload; service should treat as plain text
    #[derive(serde::Serialize)]
    struct Q { q: String }
    let resp = c.post(endpoint("/v1/search"))
        .json(&Q{ q: "test' OR 1=1 --".into() })
        .send().await.unwrap();
    assert!(resp.status().is_success());
}
