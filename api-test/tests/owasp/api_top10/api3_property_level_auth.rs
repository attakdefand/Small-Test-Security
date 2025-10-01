use crate::owasp::common::{client, endpoint};
#[tokio::test]
#[ignore = "needs field masking in responses"]
async fn sensitive_fields_are_masked() {
    let c = client();
    let r = c.get(endpoint("/v1/users/me")).header("Authorization","Bearer USER").send().await.unwrap();
    let body: serde_json::Value = r.json().await.unwrap_or(serde_json::json!({}));
    assert!(body.get("ssn").is_none(), "no SSN exposure");
    assert!(body.get("api_secret").is_none(), "no secret exposure");
}
