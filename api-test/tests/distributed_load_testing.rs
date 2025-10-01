// Only compile this module if performance-testing feature is enabled
#![cfg(feature = "performance-testing")]

use reqwest::Client;
use std::time::{Duration, Instant};
use tokio::{task, time::sleep};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct LoadTestConfig {
    /// Number of concurrent users to simulate
    concurrent_users: usize,
    /// Duration of the test in seconds
    duration_seconds: u64,
    /// Target requests per second
    target_rps: usize,
    /// API endpoints to test
    endpoints: Vec<String>,
    /// Distribution of requests across endpoints (percentages)
    endpoint_weights: Vec<f64>,
}

#[derive(Debug, Clone)]
struct LoadTestMetrics {
    total_requests: usize,
    successful_requests: usize,
    failed_requests: usize,
    total_response_time: Duration,
    min_response_time: Duration,
    max_response_time: Duration,
    status_codes: HashMap<u16, usize>,
}

impl LoadTestMetrics {
    fn new() -> Self {
        LoadTestMetrics {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_response_time: Duration::from_secs(0),
            min_response_time: Duration::from_secs(u64::MAX),
            max_response_time: Duration::from_secs(0),
            status_codes: HashMap::new(),
        }
    }

    fn record_request(&mut self, status_code: u16, response_time: Duration) {
        self.total_requests += 1;
        
        if status_code >= 200 && status_code < 300 {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }
        
        *self.status_codes.entry(status_code).or_insert(0) += 1;
        
        self.total_response_time += response_time;
        
        if response_time < self.min_response_time {
            self.min_response_time = response_time;
        }
        
        if response_time > self.max_response_time {
            self.max_response_time = response_time;
        }
    }

    fn average_response_time(&self) -> Duration {
        if self.total_requests == 0 {
            return Duration::from_secs(0);
        }
        self.total_response_time / self.total_requests as u32
    }

    fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        (self.successful_requests as f64 / self.total_requests as f64) * 100.0
    }
}

/// Distributed load testing simulation
/// This simulates multiple clients hitting the API simultaneously
#[tokio::test]
#[ignore = "enable for distributed load testing"]
async fn distributed_load_test_simulation() {
    let config = LoadTestConfig {
        concurrent_users: 50,
        duration_seconds: 30,
        target_rps: 100,
        endpoints: vec![
            "/health".to_string(),
            "/api/v1/users".to_string(),
            "/api/v1/products".to_string(),
        ],
        endpoint_weights: vec![0.1, 0.4, 0.5], // 10%, 40%, 50% distribution
    };

    println!("🚀 Starting distributed load test simulation...");
    println!("Concurrent users: {}", config.concurrent_users);
    println!("Duration: {} seconds", config.duration_seconds);
    println!("Target RPS: {}", config.target_rps);
    println!("Endpoints: {:?}", config.endpoints);

    let client = Arc::new(Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client"));

    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    let metrics = Arc::new(std::sync::Mutex::new(LoadTestMetrics::new()));

    let start_time = Instant::now();
    let mut handles = vec![];

    // Spawn concurrent user tasks
    for user_id in 0..config.concurrent_users {
        let client = client.clone();
        let base_url = base_url.clone();
        let metrics = metrics.clone();
        let config = config.clone();

        let handle = task::spawn(async move {
            let mut request_count = 0;
            let user_start = Instant::now();

            while user_start.elapsed().as_secs() < config.duration_seconds {
                // Select endpoint based on weights
                let endpoint_index = select_weighted_endpoint(&config.endpoint_weights);
                let endpoint = &config.endpoints[endpoint_index];

                let url = format!("{}{}", base_url, endpoint);
                let request_start = Instant::now();

                match client.get(&url).send().await {
                    Ok(response) => {
                        let status = response.status().as_u16();
                        let response_time = request_start.elapsed();

                        let mut metrics_guard = metrics.lock().unwrap();
                        metrics_guard.record_request(status, response_time);
                        drop(metrics_guard);

                        println!("User {}: Request to {} completed with status {} in {:?}", 
                                 user_id, endpoint, status, response_time);
                    }
                    Err(e) => {
                        let response_time = request_start.elapsed();
                        
                        let mut metrics_guard = metrics.lock().unwrap();
                        metrics_guard.record_request(500, response_time); // Internal Server Error
                        drop(metrics_guard);

                        println!("User {}: Request to {} failed: {} in {:?}", 
                                 user_id, endpoint, e, response_time);
                    }
                }

                request_count += 1;

                // Control request rate to achieve target RPS per user
                let target_delay = Duration::from_micros(1_000_000 / (config.target_rps / config.concurrent_users));
                let elapsed = request_start.elapsed();
                if elapsed < target_delay {
                    sleep(target_delay - elapsed).await;
                }
            }

            println!("User {} completed {} requests in {:?}", user_id, request_count, user_start.elapsed());
        });

        handles.push(handle);
    }

    // Wait for all user tasks to complete
    for handle in handles {
        handle.await.expect("User task panicked");
    }

    let test_duration = start_time.elapsed();
    let final_metrics = metrics.lock().unwrap();

    println!("\n📊 Load Test Results:");
    println!("Total duration: {:?}", test_duration);
    println!("Total requests: {}", final_metrics.total_requests);
    println!("Successful requests: {}", final_metrics.successful_requests);
    println!("Failed requests: {}", final_metrics.failed_requests);
    println!("Success rate: {:.2}%", final_metrics.success_rate());
    println!("Average response time: {:?}", final_metrics.average_response_time());
    println!("Min response time: {:?}", final_metrics.min_response_time);
    println!("Max response time: {:?}", final_metrics.max_response_time);
    
    println!("\nStatus code distribution:");
    for (status, count) in &final_metrics.status_codes {
        println!("  {}: {}", status, count);
    }

    // Assert that success rate is acceptable
    assert!(final_metrics.success_rate() > 95.0, 
            "Success rate {}% is below threshold of 95%", final_metrics.success_rate());

    println!("\n✅ Distributed load test simulation completed!");
}

/// Select an endpoint index based on weighted probabilities
fn select_weighted_endpoint(weights: &[f64]) -> usize {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random_value = rng.gen::<f64>();
    
    let mut cumulative_weight = 0.0;
    for (index, &weight) in weights.iter().enumerate() {
        cumulative_weight += weight;
        if random_value <= cumulative_weight {
            return index;
        }
    }
    
    // Fallback to last endpoint if something goes wrong
    weights.len().saturating_sub(1)
}

/// Test for testing API behavior under burst traffic
#[tokio::test]
#[ignore = "enable for burst traffic testing"]
async fn burst_traffic_test() {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build HTTP client");

    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    let health_path = std::env::var("HEALTH_PATH").unwrap_or("/health".into());
    let url = format!("{}{}", base_url, health_path);

    println!("💥 Testing API behavior under burst traffic...");

    // Normal traffic - 10 requests per second
    println!("Sending normal traffic...");
    for i in 1..=10 {
        let response = client.get(&url).send().await;
        match response {
            Ok(res) => println!("Normal request {}: Status {}", i, res.status()),
            Err(e) => println!("Normal request {}: Failed - {}", i, e),
        }
        sleep(Duration::from_millis(100)).await;
    }

    // Burst traffic - 50 requests in quick succession
    println!("Sending burst traffic (50 requests)...");
    let burst_start = Instant::now();
    let mut successful = 0;
    let mut failed = 0;

    for i in 1..=50 {
        let response = client.get(&url).send().await;
        match response {
            Ok(res) => {
                if res.status().is_success() {
                    successful += 1;
                } else {
                    failed += 1;
                }
                println!("Burst request {}: Status {}", i, res.status());
            }
            Err(e) => {
                failed += 1;
                println!("Burst request {}: Failed - {}", i, e);
            }
        }
    }

    let burst_duration = burst_start.elapsed();
    println!("Burst traffic completed in {:?}", burst_duration);
    println!("Successful: {}, Failed: {}", successful, failed);

    // Recovery period - 5 seconds of normal traffic
    println!("Recovery period...");
    sleep(Duration::from_secs(5)).await;

    for i in 1..=5 {
        let response = client.get(&url).send().await;
        match response {
            Ok(res) => println!("Recovery request {}: Status {}", i, res.status()),
            Err(e) => println!("Recovery request {}: Failed - {}", i, e),
        }
        sleep(Duration::from_millis(200)).await;
    }

    println!("✅ Burst traffic test completed!");
}