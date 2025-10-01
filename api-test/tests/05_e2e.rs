use api_test::{spawn_http, demo_router};

#[tokio::test]
async fn e2e_status() {
    let (addr, _handle) = spawn_http(demo_router()).await;
    let url = format!("http://{}/health", addr);
    let response = reqwest::get(&url).await.unwrap();
    assert_eq!(response.status(), 200);
    
    // The demo router returns StatusCode::OK with no body, so we expect an empty string
    let text = response.text().await.unwrap();
    assert_eq!(text, "");
}