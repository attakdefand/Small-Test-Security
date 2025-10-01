mod common;
use common::*;
use http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn rejects_sqlish_strings() {
    let url = base_url().join(&path("QUOTES_PATH")).unwrap();
    let body = json!({ "symbol": "BTCUSDT' OR 1=1 --", "amount": "1e9999" });

    let res = client().post(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .json(&body)
        .send().await.unwrap();

    assert!(matches!(res.status(), StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY));
    let text = res.text().await.unwrap();
    assert!(!text.to_lowercase().contains("stacktrace"), "should not leak stack traces");
}

#[tokio::test]
async fn rejects_malformed_json() {
    use reqwest::Body;
    let url = base_url().join(&path("QUOTES_PATH")).unwrap();

    let res = client().post(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .header("Content-Type", "application/json")
        .body(Body::from("{ not: valid json"))
        .send().await.unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
