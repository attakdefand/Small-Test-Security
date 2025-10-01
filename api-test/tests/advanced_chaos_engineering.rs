use reqwest::Client;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::test]
#[ignore = "enable for advanced chaos engineering tests"]
async fn api_resilience_under_network_chaos() {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("client build");
    
    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    let health_path = std::env::var("HEALTH_PATH").unwrap_or("/health".into());
    let url = format!("{}{}", base_url, health_path);
    
    println!("🧪 Starting advanced chaos engineering test...");
    
    // Test 1: Network latency simulation
    println!("Testing network latency tolerance...");
    let start = Instant::now();
    let result = client.get(&url)
        .timeout(Duration::from_secs(5)) // Normal timeout
        .send()
        .await;
    let duration = start.elapsed();
    
    match result {
        Ok(response) => {
            assert!(response.status().is_success());
            println!("✅ Normal request succeeded in {:?}", duration);
        }
        Err(e) => {
            panic!("❌ Normal request failed: {}", e);
        }
    }
    
    // Test 2: High latency simulation (artificial delay)
    println!("Testing high latency tolerance...");
    let start = Instant::now();
    sleep(Duration::from_millis(1000)).await; // Simulate network delay
    
    let result = client.get(&url)
        .timeout(Duration::from_secs(10)) // Extended timeout for high latency
        .send()
        .await;
    let duration = start.elapsed();
    
    match result {
        Ok(response) => {
            assert!(response.status().is_success());
            println!("✅ High latency request succeeded in {:?}", duration);
        }
        Err(e) => {
            println!("⚠️  High latency request failed (might be expected): {}", e);
        }
    }
    
    // Test 3: Partial network failure simulation
    println!("Testing partial network failure tolerance...");
    for i in 1..=5 {
        let result = client.get(&url)
            .timeout(Duration::from_millis(500)) // Very short timeout
            .send()
            .await;
            
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    println!("✅ Request {} succeeded", i);
                } else {
                    println!("⚠️  Request {} failed with status: {}", i, response.status());
                }
            }
            Err(e) => {
                println!("⚠️  Request {} failed (might be expected): {}", i, e);
            }
        }
        
        sleep(Duration::from_millis(200)).await;
    }
    
    // Test 4: Rate limiting under stress
    println!("Testing rate limiting behavior under stress...");
    let mut success_count = 0;
    let mut failure_count = 0;
    
    let start_time = Instant::now();
    while start_time.elapsed() < Duration::from_secs(10) {
        let result = client.get(&url).send().await;
        
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    success_count += 1;
                } else {
                    failure_count += 1;
                }
            }
            Err(_) => {
                failure_count += 1;
            }
        }
        
        // Small delay to avoid overwhelming the server
        sleep(Duration::from_millis(50)).await;
    }
    
    println!("📊 Stress test results: {} successful, {} failed", success_count, failure_count);
    
    // Test 5: Circuit breaker pattern simulation
    println!("Testing circuit breaker behavior...");
    let mut consecutive_failures = 0;
    const MAX_CONSECUTIVE_FAILURES: u32 = 3;
    
    for i in 1..=10 {
        let result = client.get(&url)
            .timeout(Duration::from_millis(100)) // Very short timeout
            .send()
            .await;
            
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    consecutive_failures = 0; // Reset on success
                    println!("✅ Circuit breaker test {} succeeded", i);
                } else {
                    consecutive_failures += 1;
                    println!("⚠️  Circuit breaker test {} failed with status: {}", i, response.status());
                }
            }
            Err(e) => {
                consecutive_failures += 1;
                println!("⚠️  Circuit breaker test {} failed: {}", i, e);
            }
        }
        
        // Check if circuit should open
        if consecutive_failures >= MAX_CONSECUTIVE_FAILURES {
            println!("⚡ Circuit breaker opened after {} consecutive failures", consecutive_failures);
            break;
        }
        
        sleep(Duration::from_millis(100)).await;
    }
    
    println!("🧪 Advanced chaos engineering test completed!");
}

#[tokio::test]
#[ignore = "enable for database chaos testing"]
async fn database_resilience_test() {
    let client = Client::new();
    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    
    // Test database connection resilience
    let db_test_url = format!("{}/api/v1/health/db", base_url);
    
    println!("Testing database resilience...");
    
    // Rapid database health checks
    for i in 1..=20 {
        let start = Instant::now();
        let result = client.get(&db_test_url).send().await;
        let duration = start.elapsed();
        
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    println!("✅ DB health check {} succeeded in {:?}", i, duration);
                } else {
                    println!("⚠️  DB health check {} failed with status: {} in {:?}", 
                             i, response.status(), duration);
                }
            }
            Err(e) => {
                println!("⚠️  DB health check {} failed: {} in {:?}", i, e, duration);
            }
        }
        
        sleep(Duration::from_millis(50)).await;
    }
    
    println!("Database resilience test completed!");
}