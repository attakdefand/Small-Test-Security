use reqwest::Client;
use serde_json::Value;

#[tokio::test]
#[ignore = "enable when OpenAPI spec is available"]
async fn openapi_spec_validation() {
    let client = Client::new();
    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    let spec_path = std::env::var("OPENAPI_PATH").unwrap_or("/openapi.json".into());
    
    // Fetch OpenAPI specification
    let spec_url = format!("{}{}", base_url, spec_path);
    let res = client
        .get(&spec_url)
        .send()
        .await
        .expect("Failed to fetch OpenAPI spec");
    
    assert_eq!(res.status(), 200);
    
    let spec: Value = res.json().await.expect("Failed to parse OpenAPI spec");
    
    // Validate basic OpenAPI structure
    assert_eq!(spec["openapi"].as_str().unwrap(), "3.0.3");
    assert!(spec["info"].is_object());
    assert!(spec["paths"].is_object());
    
    // Validate that required endpoints exist
    let health_path = std::env::var("HEALTH_PATH").unwrap_or("/health".into());
    assert!(spec["paths"][&health_path].is_object());
}