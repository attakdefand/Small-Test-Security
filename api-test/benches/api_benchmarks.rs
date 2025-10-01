use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use reqwest::Client;
use std::time::Duration;

fn benchmark_health_endpoint(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("health_endpoint", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let client = Client::new();
                let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
                let health_path = std::env::var("HEALTH_PATH").unwrap_or("/health".into());
                let url = format!("{}{}", base_url, health_path);
                
                let response = client.get(&url).send().await.unwrap();
                black_box(response.status());
            })
        })
    });
}

fn benchmark_user_profile_endpoint(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("user_profile_endpoint", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let client = Client::builder()
                    .timeout(Duration::from_secs(5))
                    .build()
                    .expect("client build");
                    
                let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
                let me_path = std::env::var("ME_PATH").unwrap_or("/api/v1/me".into());
                let url = format!("{}{}", base_url, me_path);
                
                // Use a dummy token for benchmarking
                let token = std::env::var("USER_TOKEN").unwrap_or("dummy-token".into());
                
                let response = client
                    .get(&url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .unwrap();
                    
                black_box(response.status());
            })
        })
    });
}

fn benchmark_concurrent_requests(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("concurrent_health_requests_10", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let client = Client::new();
                let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
                let health_path = std::env::var("HEALTH_PATH").unwrap_or("/health".into());
                let url = format!("{}{}", base_url, health_path);
                
                // Create 10 concurrent requests
                let mut handles = vec![];
                for _ in 0..10 {
                    let client = client.clone();
                    let url = url.clone();
                    let handle = tokio::spawn(async move {
                        let response = client.get(&url).send().await.unwrap();
                        black_box(response.status())
                    });
                    handles.push(handle);
                }
                
                // Wait for all requests to complete
                for handle in handles {
                    handle.await.unwrap();
                }
            })
        })
    });
}

fn benchmark_json_parsing(c: &mut Criterion) {
    let json_data = r#"{
        "id": "123456789",
        "username": "testuser",
        "email": "test@example.com",
        "profile": {
            "firstName": "Test",
            "lastName": "User",
            "age": 30,
            "preferences": {
                "theme": "dark",
                "notifications": true,
                "language": "en"
            }
        },
        "accounts": [
            {
                "id": "acc_001",
                "type": "checking",
                "balance": 1250.75,
                "currency": "USD"
            },
            {
                "id": "acc_002",
                "type": "savings",
                "balance": 5500.00,
                "currency": "USD"
            }
        ],
        "createdAt": "2023-01-15T10:30:00Z",
        "lastLogin": "2023-06-20T14:45:30Z"
    }"#;
    
    c.bench_function("json_parsing_large_object", |b| {
        b.iter(|| {
            let parsed: serde_json::Value = serde_json::from_str(black_box(json_data)).unwrap();
            black_box(parsed);
        })
    });
}

fn benchmark_token_validation(c: &mut Criterion) {
    // Sample JWT token (this is a valid token structure, not a real token)
    let sample_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    
    c.bench_function("jwt_token_header_decoding", |b| {
        b.iter(|| {
            // Split the token to get the header (first part)
            let parts: Vec<&str> = sample_token.split('.').collect();
            if parts.len() >= 1 {
                let header = black_box(parts[0]);
                // In a real implementation, you'd base64 decode here
                black_box(header.len());
            }
        })
    });
}

// Custom benchmark group with specific configurations
fn custom_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("custom_api_benchmarks");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("warmup_health_check", |b: &mut Bencher| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            runtime.block_on(async {
                let client = Client::new();
                let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
                let health_path = std::env::var("HEALTH_PATH").unwrap_or("/health".into());
                let url = format!("{}{}", base_url, health_path);
                
                let response = client.get(&url).send().await.unwrap();
                black_box(response.status());
            })
        })
    });
    
    group.finish();
}

// Throughput benchmarks
fn throughput_benchmarks(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("throughput");
    group.throughput(criterion::Throughput::Elements(1));
    
    group.bench_function("health_requests_per_second", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let client = Client::new();
                let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
                let health_path = std::env::var("HEALTH_PATH").unwrap_or("/health".into());
                let url = format!("{}{}", base_url, health_path);
                
                let response = client.get(&url).send().await.unwrap();
                black_box(response.status());
            })
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_health_endpoint,
    benchmark_user_profile_endpoint,
    benchmark_concurrent_requests,
    benchmark_json_parsing,
    benchmark_token_validation,
    custom_benchmarks,
    throughput_benchmarks
);

criterion_main!(benches);