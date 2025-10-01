// Only compile this module if security-testing feature is enabled
#![cfg(feature = "security-testing")]

use reqwest::Client;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct SecurityScanResult {
    vulnerability_type: String,
    severity: String,
    endpoint: String,
    description: String,
    recommendation: String,
    cvss_score: Option<f32>,
}

/// Advanced security scanning for common API vulnerabilities
#[tokio::test]
#[ignore = "enable for advanced security scanning"]
async fn advanced_security_vulnerability_scan() {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client");

    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    let mut vulnerabilities_found = Vec::new();

    println!("🛡️ Starting advanced security vulnerability scan...");

    // Test 1: SQL Injection attempts
    vulnerabilities_found.extend(test_sql_injection(&client, &base_url).await);

    // Test 2: Cross-Site Scripting (XSS) attempts
    vulnerabilities_found.extend(test_xss_vulnerabilities(&client, &base_url).await);

    // Test 3: Command injection testing
    vulnerabilities_found.extend(test_command_injection(&client, &base_url).await);

    // Test 4: Authentication bypass attempts
    vulnerabilities_found.extend(test_auth_bypass(&client, &base_url).await);

    // Test 5: Insecure direct object references
    vulnerabilities_found.extend(test_idor_vulnerabilities(&client, &base_url).await);

    // Test 6: Security header checks
    vulnerabilities_found.extend(test_security_headers(&client, &base_url).await);

    // Test 7: Rate limiting verification
    vulnerabilities_found.extend(test_rate_limiting(&client, &base_url).await);

    // Report findings
    println!("\n🔍 Security Scan Results:");
    println!("Found {} potential vulnerabilities", vulnerabilities_found.len());

    for (index, vuln) in vulnerabilities_found.iter().enumerate() {
        println!("\n{}. {} - {} (CVSS: {:?})", 
                 index + 1, 
                 vuln.vulnerability_type, 
                 vuln.severity, 
                 vuln.cvss_score);
        println!("   Endpoint: {}", vuln.endpoint);
        println!("   Description: {}", vuln.description);
        println!("   Recommendation: {}", vuln.recommendation);
    }

    // Fail the test if high/critical vulnerabilities are found
    let critical_vulns = vulnerabilities_found.iter()
        .filter(|v| v.severity == "Critical" || v.severity == "High")
        .count();

    if critical_vulns > 0 {
        panic!("❌ Found {} critical/high severity vulnerabilities that must be addressed!", critical_vulns);
    }

    println!("\n✅ Advanced security scan completed!");
}

async fn test_sql_injection(client: &Client, base_url: &str) -> Vec<SecurityScanResult> {
    let mut results = Vec::new();
    
    // Common SQL injection payloads
    let payloads = vec![
        "' OR '1'='1",
        "'; DROP TABLE users; --",
        "' UNION SELECT username, password FROM users --",
        "admin'--",
    ];

    let endpoints = vec![
        "/api/v1/users/search",
        "/api/v1/products",
        "/api/v1/login",
    ];

    for endpoint in &endpoints {
        let url = format!("{}{}", base_url, endpoint);
        
        for payload in &payloads {
            // Test with query parameter
            let test_url = format!("{}?q={}", url, payload);
            
            match client.get(&test_url).send().await {
                Ok(response) => {
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();
                    
                    // Check for SQL error indicators in response
                    if body.contains("SQL syntax") || 
                       body.contains("mysql_fetch") || 
                       body.contains("ORA-") ||
                       status == 500 {
                        
                        results.push(SecurityScanResult {
                            vulnerability_type: "SQL Injection".to_string(),
                            severity: if status == 500 { "High".to_string() } else { "Medium".to_string() },
                            endpoint: endpoint.to_string(),
                            description: format!("Potential SQL injection vulnerability detected with payload: {}", payload),
                            recommendation: "Use parameterized queries and input validation".to_string(),
                            cvss_score: Some(if status == 500 { 8.1 } else { 5.3 }),
                        });
                    }
                }
                Err(_) => {
                    // Connection errors might indicate WAF blocking
                }
            }
        }
    }
    
    results
}

async fn test_xss_vulnerabilities(client: &Client, base_url: &str) -> Vec<SecurityScanResult> {
    let mut results = Vec::new();
    
    // Common XSS payloads
    let payloads = vec![
        "<script>alert('XSS')</script>",
        "javascript:alert('XSS')",
        "<img src=x onerror=alert('XSS')>",
        "<svg/onload=alert('XSS')>",
    ];

    let endpoints = vec![
        "/api/v1/search",
        "/api/v1/comments",
        "/api/v1/profile",
    ];

    for endpoint in &endpoints {
        let url = format!("{}{}", base_url, endpoint);
        
        for payload in &payloads {
            // Test with query parameter
            let test_url = format!("{}?q={}", url, payload);
            
            match client.get(&test_url).send().await {
                Ok(response) => {
                    let body = response.text().await.unwrap_or_default();
                    
                    // Check if payload appears in response unescaped
                    if body.contains(payload) && !body.contains("&lt;script&gt;") {
                        results.push(SecurityScanResult {
                            vulnerability_type: "Cross-Site Scripting (XSS)".to_string(),
                            severity: "High".to_string(),
                            endpoint: endpoint.to_string(),
                            description: format!("Potential XSS vulnerability detected with payload: {}", payload),
                            recommendation: "Sanitize and escape user input properly".to_string(),
                            cvss_score: Some(6.1),
                        });
                    }
                }
                Err(_) => {
                    // Connection errors might indicate WAF blocking
                }
            }
        }
    }
    
    results
}

async fn test_command_injection(client: &Client, base_url: &str) -> Vec<SecurityScanResult> {
    let mut results = Vec::new();
    
    // Common command injection payloads
    let payloads = vec![
        "; ls -la",
        "| cat /etc/passwd",
        "& dir",
        "`whoami`",
    ];

    let endpoints = vec![
        "/api/v1/ping",
        "/api/v1/file",
        "/api/v1/system",
    ];

    for endpoint in &endpoints {
        let url = format!("{}{}", base_url, endpoint);
        
        for payload in &payloads {
            // Test with parameter that might be used in system commands
            let test_url = format!("{}?host={}", url, payload);
            
            match client.get(&test_url).send().await {
                Ok(response) => {
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();
                    
                    // Check for command execution indicators
                    if (status == 200 && (body.contains("root:") || body.contains("Directory"))) ||
                       status == 500 {
                        results.push(SecurityScanResult {
                            vulnerability_type: "Command Injection".to_string(),
                            severity: if status == 500 { "Critical".to_string() } else { "High".to_string() },
                            endpoint: endpoint.to_string(),
                            description: format!("Potential command injection vulnerability detected with payload: {}", payload),
                            recommendation: "Validate and sanitize all inputs used in system commands".to_string(),
                            cvss_score: Some(if status == 500 { 9.9 } else { 8.1 }),
                        });
                    }
                }
                Err(_) => {
                    // Connection errors might indicate WAF blocking
                }
            }
        }
    }
    
    results
}

async fn test_auth_bypass(client: &Client, base_url: &str) -> Vec<SecurityScanResult> {
    let mut results = Vec::new();
    
    // Test endpoints that should require authentication
    let protected_endpoints = vec![
        "/api/v1/admin",
        "/api/v1/users",
        "/api/v1/settings",
    ];

    for endpoint in &protected_endpoints {
        let url = format!("{}{}", base_url, endpoint);
        
        // Test without authentication
        match client.get(&url).send().await {
            Ok(response) => {
                let status = response.status();
                
                // If we get a successful response without auth, it's a problem
                if status.is_success() {
                    results.push(SecurityScanResult {
                        vulnerability_type: "Authentication Bypass".to_string(),
                        severity: "Critical".to_string(),
                        endpoint: endpoint.to_string(),
                        description: "Endpoint accessible without authentication".to_string(),
                        recommendation: "Implement proper authentication checks".to_string(),
                        cvss_score: Some(9.2),
                    });
                }
            }
            Err(_) => {
                // Connection errors are expected for non-existent endpoints
            }
        }
    }
    
    results
}

async fn test_idor_vulnerabilities(client: &Client, base_url: &str) -> Vec<SecurityScanResult> {
    let mut results = Vec::new();
    
    // Test with different user IDs to see if we can access other users' data
    let test_ids = vec!["1", "2", "999999", "0"];
    
    for id in test_ids {
        let url = format!("{}/api/v1/users/{}", base_url, id);
        
        match client.get(&url).send().await {
            Ok(response) => {
                let status = response.status();
                
                // If we can access another user's data without proper authorization
                if status.is_success() {
                    // Try to determine if this is our own data or someone else's
                    let body = response.text().await.unwrap_or_default();
                    
                    // If the response contains data that doesn't match the ID we requested
                    // (this is a simplified check - in reality, you'd need to parse the response)
                    if !body.contains(&format!("\"id\":\"{}\"", id)) && !body.is_empty() {
                        results.push(SecurityScanResult {
                            vulnerability_type: "Insecure Direct Object Reference (IDOR)".to_string(),
                            severity: "High".to_string(),
                            endpoint: format!("/api/v1/users/{}", id),
                            description: "Possibly able to access other users' data".to_string(),
                            recommendation: "Implement proper authorization checks".to_string(),
                            cvss_score: Some(7.5),
                        });
                    }
                }
            }
            Err(_) => {
                // Connection errors are expected
            }
        }
    }
    
    results
}

async fn test_security_headers(client: &Client, base_url: &str) -> Vec<SecurityScanResult> {
    let mut results = Vec::new();
    
    let url = format!("{}{}", base_url, "/health");
    
    match client.get(&url).send().await {
        Ok(response) => {
            let headers = response.headers();
            
            // Check for missing security headers
            let security_headers = [
                ("X-Content-Type-Options", "nosniff"),
                ("X-Frame-Options", "DENY"),
                ("X-XSS-Protection", "1; mode=block"),
                ("Strict-Transport-Security", "max-age=31536000"),
            ];
            
            for (header_name, _) in &security_headers {
                if !headers.contains_key(*header_name) {
                    results.push(SecurityScanResult {
                        vulnerability_type: "Missing Security Header".to_string(),
                        severity: "Medium".to_string(),
                        endpoint: "/health".to_string(),
                        description: format!("Missing security header: {}", header_name),
                        recommendation: format!("Add {} header to responses", header_name),
                        cvss_score: Some(5.3),
                    });
                }
            }
        }
        Err(_) => {
            // Connection errors
        }
    }
    
    results
}

async fn test_rate_limiting(client: &Client, base_url: &str) -> Vec<SecurityScanResult> {
    let mut results = Vec::new();
    
    let url = format!("{}{}", base_url, "/api/v1/login");
    
    // Send multiple rapid requests to test rate limiting
    let mut status_codes = Vec::new();
    
    for _ in 0..20 {
        match client.post(&url)
            .json(&serde_json::json!({"username": "test", "password": "test"}))
            .send()
            .await {
            Ok(response) => {
                status_codes.push(response.status().as_u16());
            }
            Err(_) => {
                status_codes.push(0); // Connection error
            }
        }
    }
    
    // Count 429 (Too Many Requests) responses
    let rate_limit_count = status_codes.iter().filter(|&&code| code == 429).count();
    
    // If we don't get any 429 responses, rate limiting might not be implemented
    if rate_limit_count == 0 {
        results.push(SecurityScanResult {
            vulnerability_type: "Missing Rate Limiting".to_string(),
            severity: "Medium".to_string(),
            endpoint: "/api/v1/login".to_string(),
            description: "No rate limiting detected - vulnerable to brute force attacks",
            recommendation: "Implement rate limiting on authentication endpoints", 
            cvss_score: Some(5.3),
        });
    }
    
    results
}