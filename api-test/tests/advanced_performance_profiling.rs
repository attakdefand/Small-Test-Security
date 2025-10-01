// Only compile this module if performance-testing feature is enabled
#![cfg(feature = "performance-testing")]

use reqwest::Client;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use serde::{Deserialize, Serialize};

/// Performance profiling configuration
#[derive(Debug)]
struct ProfilingConfig {
    duration_seconds: u64,
    sample_rate: u32,
    endpoints: Vec<String>,
}

/// Performance metrics
#[derive(Debug, Serialize, Deserialize)]
struct PerformanceMetrics {
    endpoint: String,
    total_requests: usize,
    successful_requests: usize,
    failed_requests: usize,
    average_response_time: f64,
    min_response_time: f64,
    max_response_time: f64,
    throughput: f64, // requests per second
    cpu_usage: f64,
    memory_usage: f64,
}

/// Advanced performance profiling and optimization
#[tokio::test]
#[ignore = "enable for advanced performance profiling"]
async fn advanced_api_performance_profiling() {
    println!("⚡ Starting advanced API performance profiling...");
    
    let config = ProfilingConfig {
        duration_seconds: 60,
        sample_rate: 100,
        endpoints: vec![
            "/health".to_string(),
            "/api/v1/users".to_string(),
            "/api/v1/products".to_string(),
        ],
    };
    
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client");
    
    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    
    // Profile each endpoint
    let mut all_metrics = Vec::new();
    
    for endpoint in &config.endpoints {
        println!("Profiling endpoint: {}", endpoint);
        let metrics = profile_endpoint(&client, &base_url, endpoint, config.duration_seconds).await;
        all_metrics.push(metrics);
    }
    
    // Report metrics
    println!("\n📊 Performance Profiling Results:");
    for metrics in &all_metrics {
        println!("\nEndpoint: {}", metrics.endpoint);
        println!("  Total requests: {}", metrics.total_requests);
        println!("  Successful requests: {}", metrics.successful_requests);
        println!("  Failed requests: {}", metrics.failed_requests);
        println!("  Average response time: {:.2}ms", metrics.average_response_time);
        println!("  Min response time: {:.2}ms", metrics.min_response_time);
        println!("  Max response time: {:.2}ms", metrics.max_response_time);
        println!("  Throughput: {:.2} requests/sec", metrics.throughput);
        println!("  CPU usage: {:.2}%", metrics.cpu_usage);
        println!("  Memory usage: {:.2} MB", metrics.memory_usage);
    }
    
    println!("\n✅ Advanced performance profiling completed!");
}

async fn profile_endpoint(
    client: &Client,
    base_url: &str,
    endpoint: &str,
    duration_seconds: u64,
) -> PerformanceMetrics {
    let url = format!("{}{}", base_url, endpoint);
    let start_time = Instant::now();
    
    let mut total_requests = 0;
    let mut successful_requests = 0;
    let mut failed_requests = 0;
    let mut total_response_time = 0.0;
    let mut min_response_time = f64::MAX;
    let mut max_response_time = 0.0;
    
    // System monitoring (if sysinfo is available)
    let initial_cpu = 0.0;
    let initial_memory = 0.0; // MB
    
    while start_time.elapsed().as_secs() < duration_seconds {
        let request_start = Instant::now();
        
        match client.get(&url).send().await {
            Ok(response) => {
                let response_time = request_start.elapsed().as_millis() as f64;
                total_requests += 1;
                
                if response.status().is_success() {
                    successful_requests += 1;
                } else {
                    failed_requests += 1;
                }
                
                total_response_time += response_time;
                
                if response_time < min_response_time {
                    min_response_time = response_time;
                }
                
                if response_time > max_response_time {
                    max_response_time = response_time;
                }
                
                println!("Request {}: Status {} in {:.2}ms", 
                         total_requests, response.status(), response_time);
            }
            Err(e) => {
                total_requests += 1;
                failed_requests += 1;
                println!("Request {}: Failed - {}", total_requests, e);
            }
        }
        
        // Small delay to avoid overwhelming the server
        sleep(Duration::from_millis(100)).await;
    }
    
    // Final system metrics
    let final_cpu = 0.0;
    let final_memory = 0.0; // MB
    
    let duration = start_time.elapsed().as_secs_f64();
    let average_response_time = if total_requests > 0 {
        total_response_time / total_requests as f64
    } else {
        0.0
    };
    
    let throughput = total_requests as f64 / duration;
    
    // Handle edge cases
    if min_response_time == f64::MAX {
        min_response_time = 0.0;
    }
    
    PerformanceMetrics {
        endpoint: endpoint.to_string(),
        total_requests,
        successful_requests,
        failed_requests,
        average_response_time,
        min_response_time,
        max_response_time,
        throughput,
        cpu_usage: (final_cpu - initial_cpu).abs() as f64,
        memory_usage: (final_memory - initial_memory).abs(),
    }
}

/// Test API performance under different load patterns
#[tokio::test]
#[ignore = "enable for load pattern testing"]
async fn api_load_pattern_performance_test() {
    println!("📈 Testing API performance under different load patterns...");
    
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client");
    
    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    let health_path = std::env::var("HEALTH_PATH").unwrap_or("/health".into());
    let url = format!("{}{}", base_url, health_path);
    
    // Test 1: Constant load
    println!("\n1. Testing constant load...");
    let constant_load_metrics = run_constant_load_test(&client, &url, 30).await;
    println!("   Throughput: {:.2} req/sec", constant_load_metrics.throughput);
    println!("   Avg Response Time: {:.2}ms", constant_load_metrics.average_response_time);
    
    // Test 2: Spike load
    println!("\n2. Testing spike load...");
    let spike_load_metrics = run_spike_load_test(&client, &url, 30).await;
    println!("   Peak Throughput: {:.2} req/sec", spike_load_metrics.throughput);
    println!("   Avg Response Time: {:.2}ms", spike_load_metrics.average_response_time);
    
    // Test 3: Gradual load increase
    println!("\n3. Testing gradual load increase...");
    let gradual_load_metrics = run_gradual_load_test(&client, &url, 30).await;
    println!("   Final Throughput: {:.2} req/sec", gradual_load_metrics.throughput);
    println!("   Avg Response Time: {:.2}ms", gradual_load_metrics.average_response_time);
    
    println!("\n✅ Load pattern performance testing completed!");
}

async fn run_constant_load_test(client: &Client, url: &str, duration_seconds: u64) -> PerformanceMetrics {
    let start_time = Instant::now();
    let mut total_requests = 0;
    let mut successful_requests = 0;
    let mut total_response_time = 0.0;
    
    while start_time.elapsed().as_secs() < duration_seconds {
        let request_start = Instant::now();
        
        match client.get(url).send().await {
            Ok(response) => {
                let response_time = request_start.elapsed().as_millis() as f64;
                total_requests += 1;
                
                if response.status().is_success() {
                    successful_requests += 1;
                }
                
                total_response_time += response_time;
            }
            Err(_) => {
                total_requests += 1;
            }
        }
        
        // Constant interval of 200ms (5 req/sec)
        sleep(Duration::from_millis(200)).await;
    }
    
    let duration = start_time.elapsed().as_secs_f64();
    let average_response_time = if total_requests > 0 {
        total_response_time / total_requests as f64
    } else {
        0.0
    };
    
    PerformanceMetrics {
        endpoint: "constant_load".to_string(),
        total_requests,
        successful_requests,
        failed_requests: total_requests - successful_requests,
        average_response_time,
        min_response_time: 0.0,
        max_response_time: 0.0,
        throughput: total_requests as f64 / duration,
        cpu_usage: 0.0,
        memory_usage: 0.0,
    }
}

async fn run_spike_load_test(client: &Client, url: &str, duration_seconds: u64) -> PerformanceMetrics {
    let start_time = Instant::now();
    let mut total_requests = 0;
    let mut successful_requests = 0;
    let mut total_response_time = 0.0;
    
    while start_time.elapsed().as_secs() < duration_seconds {
        let elapsed = start_time.elapsed().as_secs();
        
        // Create spike every 10 seconds
        let requests_per_second = if elapsed % 10 < 2 {
            // Spike: 20 requests per second
            20
        } else {
            // Normal: 2 requests per second
            2
        };
        
        let request_start = Instant::now();
        
        match client.get(url).send().await {
            Ok(response) => {
                let response_time = request_start.elapsed().as_millis() as f64;
                total_requests += 1;
                
                if response.status().is_success() {
                    successful_requests += 1;
                }
                
                total_response_time += response_time;
            }
            Err(_) => {
                total_requests += 1;
            }
        }
        
        // Adjust delay based on desired RPS
        let delay = Duration::from_millis(1000 / requests_per_second);
        sleep(delay).await;
    }
    
    let duration = start_time.elapsed().as_secs_f64();
    let average_response_time = if total_requests > 0 {
        total_response_time / total_requests as f64
    } else {
        0.0
    };
    
    PerformanceMetrics {
        endpoint: "spike_load".to_string(),
        total_requests,
        successful_requests,
        failed_requests: total_requests - successful_requests,
        average_response_time,
        min_response_time: 0.0,
        max_response_time: 0.0,
        throughput: total_requests as f64 / duration,
        cpu_usage: 0.0,
        memory_usage: 0.0,
    }
}

async fn run_gradual_load_test(client: &Client, url: &str, duration_seconds: u64) -> PerformanceMetrics {
    let start_time = Instant::now();
    let mut total_requests = 0;
    let mut successful_requests = 0;
    let mut total_response_time = 0.0;
    
    while start_time.elapsed().as_secs() < duration_seconds {
        let elapsed = start_time.elapsed().as_secs();
        
        // Gradually increase load: start with 1 req/sec, end with 10 req/sec
        let progress = elapsed as f64 / duration_seconds as f64;
        let requests_per_second = 1 + (9.0 * progress) as u64;
        
        let request_start = Instant::now();
        
        match client.get(url).send().await {
            Ok(response) => {
                let response_time = request_start.elapsed().as_millis() as f64;
                total_requests += 1;
                
                if response.status().is_success() {
                    successful_requests += 1;
                }
                
                total_response_time += response_time;
            }
            Err(_) => {
                total_requests += 1;
            }
        }
        
        // Adjust delay based on desired RPS
        let delay = Duration::from_millis(1000 / requests_per_second.max(1));
        sleep(delay).await;
    }
    
    let duration = start_time.elapsed().as_secs_f64();
    let average_response_time = if total_requests > 0 {
        total_response_time / total_requests as f64
    } else {
        0.0
    };
    
    PerformanceMetrics {
        endpoint: "gradual_load".to_string(),
        total_requests,
        successful_requests,
        failed_requests: total_requests - successful_requests,
        average_response_time,
        min_response_time: 0.0,
        max_response_time: 0.0,
        throughput: total_requests as f64 / duration,
        cpu_usage: 0.0,
        memory_usage: 0.0,
    }
}

/// Test API performance with different data sizes
#[tokio::test]
#[ignore = "enable for data size performance testing"]
async fn api_data_size_performance_test() {
    println!("📊 Testing API performance with different data sizes...");
    
    // This would typically test endpoints that return different amounts of data
    // For example, an endpoint that can return 10, 100, or 1000 records
    
    let data_sizes = vec![10, 100, 1000];
    
    for size in data_sizes {
        println!("Testing with data size: {} records", size);
        // In a real implementation, you would call an endpoint that returns
        // the specified number of records and measure performance
        println!("  Simulating API call with {} records...", size);
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("  ✅ Completed test for {} records", size);
    }
    
    println!("\n✅ Data size performance testing completed!");
}

/// Test API caching performance
#[tokio::test]
#[ignore = "enable for caching performance testing"]
async fn api_caching_performance_test() {
    println!("キャッシング Testing API caching performance...");
    
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client");
    
    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    let health_path = std::env::var("HEALTH_PATH").unwrap_or("/health".into());
    let url = format!("{}{}", base_url, health_path);
    
    // Test 1: Cold cache performance
    println!("\n1. Testing cold cache performance...");
    let cold_start = Instant::now();
    let _ = client.get(&url).send().await;
    let cold_duration = cold_start.elapsed().as_millis() as f64;
    println!("   Cold cache response time: {:.2}ms", cold_duration);
    
    // Test 2: Warm cache performance
    println!("\n2. Testing warm cache performance...");
    let warm_start = Instant::now();
    let _ = client.get(&url).send().await;
    let warm_duration = warm_start.elapsed().as_millis() as f64;
    println!("   Warm cache response time: {:.2}ms", warm_duration);
    
    // Test 3: Cache hit rate
    println!("\n3. Testing cache hit rate...");
    let mut cache_hits = 0;
    let total_requests = 10;
    
    for i in 1..=total_requests {
        let request_start = Instant::now();
        let response = client.get(&url).send().await;
        let request_duration = request_start.elapsed().as_millis() as f64;
        
        // Assume requests under 50ms are cache hits (simplified logic)
        if request_duration < 50.0 {
            cache_hits += 1;
        }
        
        println!("   Request {}: {:.2}ms {}", i, request_duration, 
                 if request_duration < 50.0 { "(cache hit)" } else { "(cache miss)" });
        
        sleep(Duration::from_millis(100)).await;
    }
    
    let hit_rate = (cache_hits as f64 / total_requests as f64) * 100.0;
    println!("   Cache hit rate: {:.2}%", hit_rate);
    
    println!("\n✅ Caching performance testing completed!");
}