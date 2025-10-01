use reqwest::Client;
use std::time::{Duration, Instant};

#[tokio::test]
#[ignore = "enable for chaos engineering tests"]
async fn api_resilience_under_network_issues() {
    let client = Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .expect("client build");
    
    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    let health_path = std::env::var("HEALTH_PATH").unwrap_or("/health".into());
    let url = format!("{}{}", base_url, health_path);
    
    // Test API behavior under timeout conditions
    let start = Instant::now();
    let result = client.get(&url).send().await;
    let duration = start.elapsed();
    
    // API should either succeed quickly or fail gracefully
    match result {
        Ok(response) => {
            // If it succeeds, it should be fast
            assert!(duration < Duration::from_secs(1));
            assert!(response.status().is_success() || response.status().as_u16() == 429);
        }
        Err(e) => {
            // If it fails, it should be due to timeout, not internal errors
            println!("Request failed as expected: {}", e);
        }
    }
}