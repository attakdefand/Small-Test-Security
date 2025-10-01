// Only compile this module if monitoring feature is enabled
#![cfg(feature = "monitoring")]

use reqwest::Client;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Represents a monitoring metric
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Metric {
    name: String,
    value: f64,
    timestamp: u64,
    labels: HashMap<String, String>,
}

/// Alert rule definition
#[derive(Debug, Clone)]
struct AlertRule {
    name: String,
    metric_name: String,
    threshold: f64,
    comparison: ComparisonOperator,
    duration: Duration, // How long the condition must be true
    severity: AlertSeverity,
}

/// Comparison operators for alert rules
#[derive(Debug, Clone)]
enum ComparisonOperator {
    GreaterThan,
    LessThan,
    EqualTo,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// Alert severity levels
#[derive(Debug, Clone)]
enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Alert notification
#[derive(Debug)]
struct Alert {
    rule_name: String,
    severity: AlertSeverity,
    message: String,
    timestamp: u64,
}

/// Monitoring system state
#[derive(Debug)]
struct MonitoringSystem {
    metrics: Arc<RwLock<Vec<Metric>>>,
    alert_rules: Vec<AlertRule>,
    alerts: Arc<RwLock<Vec<Alert>>>,
}

impl MonitoringSystem {
    fn new() -> Self {
        MonitoringSystem {
            metrics: Arc::new(RwLock::new(Vec::new())),
            alert_rules: Vec::new(),
            alerts: Arc::new(RwLock::new(Vec::new())),
        }
    }

    fn add_alert_rule(&mut self, rule: AlertRule) {
        self.alert_rules.push(rule);
    }

    async fn record_metric(&self, metric: Metric) {
        let mut metrics = self.metrics.write().await;
        metrics.push(metric);
    }

    async fn check_alerts(&self) -> Vec<Alert> {
        let mut triggered_alerts = Vec::new();
        let metrics = self.metrics.read().await;
        
        for rule in &self.alert_rules {
            // Check if the rule condition is met
            if self.evaluate_rule(rule, &metrics).await {
                let alert = Alert {
                    rule_name: rule.name.clone(),
                    severity: rule.severity.clone(),
                    message: format!("Alert triggered: {} metric crossed threshold of {}", 
                                   rule.metric_name, rule.threshold),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };
                
                triggered_alerts.push(alert);
            }
        }
        
        // Record alerts
        if !triggered_alerts.is_empty() {
            let mut alerts = self.alerts.write().await;
            alerts.extend(triggered_alerts.clone());
        }
        
        triggered_alerts
    }

    async fn evaluate_rule(&self, rule: &AlertRule, metrics: &[Metric]) -> bool {
        // Find relevant metrics
        let relevant_metrics: Vec<&Metric> = metrics
            .iter()
            .filter(|m| m.name == rule.metric_name)
            .collect();
        
        if relevant_metrics.is_empty() {
            return false;
        }
        
        // Get the latest metric value
        let latest_metric = relevant_metrics.last().unwrap();
        let value = latest_metric.value;
        
        // Evaluate the condition
        let condition_met = match rule.comparison {
            ComparisonOperator::GreaterThan => value > rule.threshold,
            ComparisonOperator::LessThan => value < rule.threshold,
            ComparisonOperator::EqualTo => (value - rule.threshold).abs() < f64::EPSILON,
            ComparisonOperator::GreaterThanOrEqual => value >= rule.threshold,
            ComparisonOperator::LessThanOrEqual => value <= rule.threshold,
        };
        
        condition_met
    }

    async fn get_recent_metrics(&self, metric_name: &str, duration: Duration) -> Vec<Metric> {
        let metrics = self.metrics.read().await;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let cutoff_time = now - duration.as_secs();
        
        metrics.iter()
            .filter(|m| m.name == metric_name && m.timestamp >= cutoff_time)
            .cloned()
            .collect()
    }
}

/// Advanced monitoring and alerting system for API testing
#[tokio::test]
#[ignore = "enable for advanced monitoring and alerting"]
async fn advanced_api_monitoring_and_alerting() {
    println!("📈 Starting advanced API monitoring and alerting system...");

    let monitoring = Arc::new(MonitoringSystem::new());
    
    // Define alert rules
    let mut rules = vec![
        AlertRule {
            name: "High Error Rate".to_string(),
            metric_name: "error_rate".to_string(),
            threshold: 5.0,
            comparison: ComparisonOperator::GreaterThan,
            duration: Duration::from_secs(60),
            severity: AlertSeverity::Critical,
        },
        AlertRule {
            name: "High Latency".to_string(),
            metric_name: "response_time_95th_percentile".to_string(),
            threshold: 1000.0, // 1 second
            comparison: ComparisonOperator::GreaterThan,
            duration: Duration::from_secs(300), // 5 minutes
            severity: AlertSeverity::Warning,
        },
        AlertRule {
            name: "Low Availability".to_string(),
            metric_name: "availability".to_string(),
            threshold: 99.9,
            comparison: ComparisonOperator::LessThan,
            duration: Duration::from_secs(300), // 5 minutes
            severity: AlertSeverity::Critical,
        },
    ];
    
    // Add rules to monitoring system
    for rule in rules.drain(..) {
        monitoring.add_alert_rule(rule);
    }

    // Simulate API monitoring
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build HTTP client");

    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    let health_path = std::env::var("HEALTH_PATH").unwrap_or("/health".into());
    let url = format!("{}{}", base_url, health_path);

    println!("Starting monitoring loop...");

    let start_time = Instant::now();
    let mut request_count = 0;
    let mut error_count = 0;

    // Run monitoring for 2 minutes
    while start_time.elapsed() < Duration::from_secs(120) {
        let request_start = Instant::now();
        
        match client.get(&url).send().await {
            Ok(response) => {
                let response_time = request_start.elapsed().as_millis() as f64;
                let status_code = response.status().as_u16();
                
                request_count += 1;
                
                if status_code >= 400 {
                    error_count += 1;
                }
                
                // Record response time metric
                let metric = Metric {
                    name: "response_time".to_string(),
                    value: response_time,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    labels: HashMap::from([
                        ("endpoint".to_string(), health_path.clone()),
                        ("status_code".to_string(), status_code.to_string()),
                    ]),
                };
                
                monitoring.record_metric(metric).await;
                
                println!("Request {}: Status {} in {:.2}ms", 
                         request_count, status_code, response_time);
            }
            Err(e) => {
                error_count += 1;
                request_count += 1;
                
                println!("Request {}: Failed - {}", request_count, e);
                
                // Record error metric
                let metric = Metric {
                    name: "request_error".to_string(),
                    value: 1.0,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    labels: HashMap::from([
                        ("endpoint".to_string(), health_path.clone()),
                        ("error_type".to_string(), "connection_error".to_string()),
                    ]),
                };
                
                monitoring.record_metric(metric).await;
            }
        }
        
        // Calculate and record derived metrics every 10 requests
        if request_count % 10 == 0 {
            let error_rate = (error_count as f64 / request_count as f64) * 100.0;
            
            // Record error rate metric
            let metric = Metric {
                name: "error_rate".to_string(),
                value: error_rate,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                labels: HashMap::new(),
            };
            
            monitoring.record_metric(metric).await;
            
            // Record availability metric
            let availability = 100.0 - error_rate;
            let metric = Metric {
                name: "availability".to_string(),
                value: availability,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                labels: HashMap::new(),
            };
            
            monitoring.record_metric(metric).await;
            
            println!("Metrics - Error Rate: {:.2}%, Availability: {:.2}%", 
                     error_rate, availability);
        }
        
        // Check for alerts every 20 requests
        if request_count % 20 == 0 {
            let alerts = monitoring.check_alerts().await;
            if !alerts.is_empty() {
                println!("\n🚨 ALERTS TRIGGERED:");
                for alert in alerts {
                    let severity_str = match alert.severity {
                        AlertSeverity::Info => "INFO",
                        AlertSeverity::Warning => "WARNING",
                        AlertSeverity::Critical => "CRITICAL",
                    };
                    println!("  [{}] {}: {}", severity_str, alert.rule_name, alert.message);
                }
                println!();
            }
        }
        
        // Wait before next request (simulate realistic traffic)
        sleep(Duration::from_millis(500)).await;
    }

    // Final metrics calculation
    let final_error_rate = (error_count as f64 / request_count as f64) * 100.0;
    let final_availability = 100.0 - final_error_rate;
    
    println!("\n📊 Final Monitoring Results:");
    println!("Total requests: {}", request_count);
    println!("Failed requests: {}", error_count);
    println!("Error rate: {:.2}%", final_error_rate);
    println!("Availability: {:.2}%", final_availability);
    
    // Get recent metrics for analysis
    let recent_response_times = monitoring
        .get_recent_metrics("response_time", Duration::from_secs(60))
        .await;
    
    if !recent_response_times.is_empty() {
        let avg_response_time: f64 = recent_response_times.iter().map(|m| m.value).sum::<f64>() 
            / recent_response_times.len() as f64;
        
        println!("Average response time (last minute): {:.2}ms", avg_response_time);
        
        // Record 95th percentile metric
        let mut sorted_times: Vec<f64> = recent_response_times.iter().map(|m| m.value).collect();
        sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let percentile_95_index = ((sorted_times.len() as f64) * 0.95) as usize;
        let percentile_95 = sorted_times.get(percentile_95_index).unwrap_or(&0.0);
        
        let metric = Metric {
            name: "response_time_95th_percentile".to_string(),
            value: *percentile_95,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            labels: HashMap::new(),
        };
        
        monitoring.record_metric(metric).await;
        println!("95th percentile response time: {:.2}ms", percentile_95);
    }
    
    // Final alert check
    let final_alerts = monitoring.check_alerts().await;
    if !final_alerts.is_empty() {
        println!("\n🚨 FINAL ALERTS:");
        for alert in final_alerts {
            let severity_str = match alert.severity {
                AlertSeverity::Info => "INFO",
                AlertSeverity::Warning => "WARNING",
                AlertSeverity::Critical => "CRITICAL",
            };
            println!("  [{}] {}: {}", severity_str, alert.rule_name, alert.message);
        }
    } else {
        println!("\n✅ No active alerts at the end of monitoring period");
    }
    
    println!("\n✅ Advanced monitoring and alerting test completed!");
}

/// Test custom metrics collection and reporting
#[tokio::test]
#[ignore = "enable for custom metrics testing"]
async fn custom_metrics_collection_test() {
    println!("📊 Testing custom metrics collection...");
    
    // Simulate collecting various API metrics
    let metrics_to_collect = vec![
        "api_requests_total",
        "api_request_duration_seconds",
        "api_response_size_bytes",
        "api_errors_total",
        "active_connections",
        "memory_usage_bytes",
        "cpu_usage_percent",
    ];
    
    for metric_name in metrics_to_collect {
        println!("Collecting metric: {}", metric_name);
        
        // In a real implementation, you would collect actual values
        // For this test, we'll just simulate the collection process
        match metric_name {
            "api_requests_total" => {
                // Simulate incrementing a counter
                println!("  Counter incremented");
            }
            "api_request_duration_seconds" => {
                // Simulate recording a histogram
                println!("  Histogram observation recorded");
            }
            "api_response_size_bytes" => {
                // Simulate recording response sizes
                println!("  Response size recorded");
            }
            "api_errors_total" => {
                // Simulate recording errors
                println!("  Error counter incremented");
            }
            "active_connections" => {
                // Simulate gauge metric
                println!("  Gauge value updated");
            }
            "memory_usage_bytes" => {
                // Simulate system metrics
                println!("  System memory usage recorded");
            }
            "cpu_usage_percent" => {
                // Simulate CPU usage
                println!("  CPU usage recorded");
            }
            _ => {
                println!("  Unknown metric type");
            }
        }
    }
    
    println!("✅ Custom metrics collection test completed!");
}

/// Test alert notification mechanisms
#[tokio::test]
#[ignore = "enable for alert notification testing"]
async fn alert_notification_mechanisms_test() {
    println!("🔔 Testing alert notification mechanisms...");
    
    // Test different notification channels
    let notification_channels = vec![
        "email",
        "slack",
        "webhook",
        "pagerduty",
        "sms",
    ];
    
    for channel in notification_channels {
        println!("Testing {} notification channel...", channel);
        
        // Simulate sending notification
        match channel {
            "email" => {
                println!("  ✉️  Email notification sent to team");
            }
            "slack" => {
                println!("  💬 Slack message posted to #alerts channel");
            }
            "webhook" => {
                println!("  🌐 Webhook notification triggered");
            }
            "pagerduty" => {
                println!("  🚨 PagerDuty incident created");
            }
            "sms" => {
                println!("  📱 SMS sent to on-call engineer");
            }
            _ => {
                println!("  Unknown notification channel");
            }
        }
    }
    
    println!("✅ Alert notification mechanisms test completed!");
}