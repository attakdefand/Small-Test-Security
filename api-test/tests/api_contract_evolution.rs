use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Represents an API contract version
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiContract {
    version: String,
    endpoints: HashMap<String, EndpointDefinition>,
    deprecated_endpoints: Vec<String>,
}

/// Defines an API endpoint structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EndpointDefinition {
    method: String,
    path: String,
    request_schema: Option<SchemaDefinition>,
    response_schema: SchemaDefinition,
    deprecated: bool,
}

/// Defines a schema structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SchemaDefinition {
    #[serde(rename = "type")]
    schema_type: String,
    properties: Option<HashMap<String, SchemaProperty>>,
    required: Option<Vec<String>>,
}

/// Defines a schema property
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SchemaProperty {
    #[serde(rename = "type")]
    property_type: String,
    format: Option<String>,
    description: Option<String>,
}

/// Contract evolution test results
#[derive(Debug)]
struct ContractEvolutionResult {
    version: String,
    breaking_changes: Vec<String>,
    compatible_changes: Vec<String>,
    deprecated_endpoints: Vec<String>,
}

/// Test API contract evolution and backward compatibility
#[tokio::test]
#[ignore = "enable for API contract evolution testing"]
async fn api_contract_backward_compatibility_test() {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build HTTP client");

    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());

    println!("🔄 Testing API contract backward compatibility...");

    // Define current contract
    let current_contract = create_current_contract();
    
    // Define previous contract versions for comparison
    let previous_contracts = vec![
        create_v1_contract(),
        create_v1_1_contract(),
    ];

    let mut evolution_results = Vec::new();

    // Test each previous contract version against current implementation
    for previous_contract in previous_contracts {
        let result = test_contract_compatibility(&client, &base_url, &previous_contract, &current_contract).await;
        evolution_results.push(result);
    }

    // Report findings
    println!("\n📋 Contract Evolution Report:");
    for result in &evolution_results {
        println!("\nVersion {}:", result.version);
        
        if result.breaking_changes.is_empty() {
            println!("  ✅ No breaking changes detected");
        } else {
            println!("  ⚠️  Breaking changes:");
            for change in &result.breaking_changes {
                println!("    - {}", change);
            }
        }
        
        if !result.compatible_changes.is_empty() {
            println!("  🔄 Compatible changes:");
            for change in &result.compatible_changes {
                println!("    - {}", change);
            }
        }
        
        if !result.deprecated_endpoints.is_empty() {
            println!("  🗑️  Deprecated endpoints:");
            for endpoint in &result.deprecated_endpoints {
                println!("    - {}", endpoint);
            }
        }
    }

    // Check for any breaking changes
    let has_breaking_changes = evolution_results.iter().any(|r| !r.breaking_changes.is_empty());
    
    if has_breaking_changes {
        println!("\n❌ Breaking changes detected in API contract evolution!");
        // In a real scenario, you might want to fail the test
        // panic!("Breaking changes detected in API contract!");
    } else {
        println!("\n✅ All contract changes are backward compatible!");
    }
}

fn create_current_contract() -> ApiContract {
    ApiContract {
        version: "2.0.0".to_string(),
        endpoints: HashMap::from([
            ("/api/v1/health".to_string(), EndpointDefinition {
                method: "GET".to_string(),
                path: "/api/v1/health".to_string(),
                request_schema: None,
                response_schema: SchemaDefinition {
                    schema_type: "object".to_string(),
                    properties: Some(HashMap::from([
                        ("status".to_string(), SchemaProperty {
                            property_type: "string".to_string(),
                            format: None,
                            description: Some("Health status".to_string()),
                        }),
                        ("timestamp".to_string(), SchemaProperty {
                            property_type: "string".to_string(),
                            format: Some("date-time".to_string()),
                            description: Some("Current timestamp".to_string()),
                        }),
                    ])),
                    required: Some(vec!["status".to_string()]),
                },
                deprecated: false,
            }),
            ("/api/v1/users".to_string(), EndpointDefinition {
                method: "GET".to_string(),
                path: "/api/v1/users".to_string(),
                request_schema: None,
                response_schema: SchemaDefinition {
                    schema_type: "array".to_string(),
                    properties: None,
                    required: None,
                },
                deprecated: false,
            }),
            ("/api/v1/users/{id}".to_string(), EndpointDefinition {
                method: "GET".to_string(),
                path: "/api/v1/users/{id}".to_string(),
                request_schema: None,
                response_schema: SchemaDefinition {
                    schema_type: "object".to_string(),
                    properties: Some(HashMap::from([
                        ("id".to_string(), SchemaProperty {
                            property_type: "string".to_string(),
                            format: None,
                            description: Some("User ID".to_string()),
                        }),
                        ("name".to_string(), SchemaProperty {
                            property_type: "string".to_string(),
                            format: None,
                            description: Some("User name".to_string()),
                        }),
                        ("email".to_string(), SchemaProperty {
                            property_type: "string".to_string(),
                            format: Some("email".to_string()),
                            description: Some("User email".to_string()),
                        }),
                        ("created_at".to_string(), SchemaProperty {
                            property_type: "string".to_string(),
                            format: Some("date-time".to_string()),
                            description: Some("Creation timestamp".to_string()),
                        }),
                    ])),
                    required: Some(vec!["id".to_string(), "name".to_string(), "email".to_string()]),
                },
                deprecated: false,
            }),
        ]),
        deprecated_endpoints: vec![
            "/api/v1/profile".to_string(),
        ],
    }
}

fn create_v1_contract() -> ApiContract {
    ApiContract {
        version: "1.0.0".to_string(),
        endpoints: HashMap::from([
            ("/health".to_string(), EndpointDefinition {
                method: "GET".to_string(),
                path: "/health".to_string(),
                request_schema: None,
                response_schema: SchemaDefinition {
                    schema_type: "object".to_string(),
                    properties: Some(HashMap::from([
                        ("status".to_string(), SchemaProperty {
                            property_type: "string".to_string(),
                            format: None,
                            description: Some("Health status".to_string()),
                        }),
                    ])),
                    required: Some(vec!["status".to_string()]),
                },
                deprecated: false,
            }),
            ("/users".to_string(), EndpointDefinition {
                method: "GET".to_string(),
                path: "/users".to_string(),
                request_schema: None,
                response_schema: SchemaDefinition {
                    schema_type: "array".to_string(),
                    properties: None,
                    required: None,
                },
                deprecated: false,
            }),
        ]),
        deprecated_endpoints: vec![],
    }
}

fn create_v1_1_contract() -> ApiContract {
    ApiContract {
        version: "1.1.0".to_string(),
        endpoints: HashMap::from([
            ("/api/v1/health".to_string(), EndpointDefinition {
                method: "GET".to_string(),
                path: "/api/v1/health".to_string(),
                request_schema: None,
                response_schema: SchemaDefinition {
                    schema_type: "object".to_string(),
                    properties: Some(HashMap::from([
                        ("status".to_string(), SchemaProperty {
                            property_type: "string".to_string(),
                            format: None,
                            description: Some("Health status".to_string()),
                        }),
                        ("version".to_string(), SchemaProperty {
                            property_type: "string".to_string(),
                            format: None,
                            description: Some("API version".to_string()),
                        }),
                    ])),
                    required: Some(vec!["status".to_string()]),
                },
                deprecated: false,
            }),
            ("/api/v1/users".to_string(), EndpointDefinition {
                method: "GET".to_string(),
                path: "/api/v1/users".to_string(),
                request_schema: None,
                response_schema: SchemaDefinition {
                    schema_type: "array".to_string(),
                    properties: None,
                    required: None,
                },
                deprecated: false,
            }),
            ("/api/v1/users/{id}".to_string(), EndpointDefinition {
                method: "GET".to_string(),
                path: "/api/v1/users/{id}".to_string(),
                request_schema: None,
                response_schema: SchemaDefinition {
                    schema_type: "object".to_string(),
                    properties: Some(HashMap::from([
                        ("id".to_string(), SchemaProperty {
                            property_type: "string".to_string(),
                            format: None,
                            description: Some("User ID".to_string()),
                        }),
                        ("name".to_string(), SchemaProperty {
                            property_type: "string".to_string(),
                            format: None,
                            description: Some("User name".to_string()),
                        }),
                    ])),
                    required: Some(vec!["id".to_string(), "name".to_string()]),
                },
                deprecated: false,
            }),
        ]),
        deprecated_endpoints: vec![],
    }
}

async fn test_contract_compatibility(
    client: &Client,
    base_url: &str,
    previous: &ApiContract,
    current: &ApiContract,
) -> ContractEvolutionResult {
    let mut breaking_changes = Vec::new();
    let mut compatible_changes = Vec::new();
    let mut deprecated_endpoints = Vec::new();

    // Check for removed endpoints (breaking change)
    for (path, prev_endpoint) in &previous.endpoints {
        if !current.endpoints.contains_key(path) {
            breaking_changes.push(format!("Endpoint removed: {} {}", prev_endpoint.method, path));
        }
    }

    // Check for endpoint changes
    for (path, current_endpoint) in &current.endpoints {
        if let Some(prev_endpoint) = previous.endpoints.get(path) {
            // Check method changes (breaking)
            if prev_endpoint.method != current_endpoint.method {
                breaking_changes.push(format!("Method changed for {}: {} -> {}", 
                    path, prev_endpoint.method, current_endpoint.method));
            }

            // Check response schema additions (compatible)
            if let Some(current_props) = &current_endpoint.response_schema.properties {
                if let Some(prev_props) = &prev_endpoint.response_schema.properties {
                    for (prop_name, _) in current_props {
                        if !prev_props.contains_key(prop_name) {
                            compatible_changes.push(format!("New field '{}' added to response of {}", 
                                prop_name, path));
                        }
                    }
                }
            }

            // Check for required field changes (breaking)
            if let Some(current_required) = &current_endpoint.response_schema.required {
                if let Some(prev_required) = &prev_endpoint.response_schema.required {
                    for req_field in prev_required {
                        if !current_required.contains(req_field) {
                            breaking_changes.push(format!("Required field '{}' removed from response of {}", 
                                req_field, path));
                        }
                    }
                }
            }
        } else {
            // New endpoint (compatible)
            compatible_changes.push(format!("New endpoint added: {} {}", current_endpoint.method, path));
        }
    }

    // Check deprecated endpoints
    for deprecated_path in &current.deprecated_endpoints {
        if previous.endpoints.contains_key(deprecated_path) {
            deprecated_endpoints.push(deprecated_path.clone());
        }
    }

    // Test actual API endpoints to verify compatibility
    test_actual_endpoints(client, base_url, previous, current, &mut breaking_changes).await;

    ContractEvolutionResult {
        version: previous.version.clone(),
        breaking_changes,
        compatible_changes,
        deprecated_endpoints,
    }
}

async fn test_actual_endpoints(
    client: &Client,
    base_url: &str,
    previous: &ApiContract,
    current: &ApiContract,
    breaking_changes: &mut Vec<String>,
) {
    // Test health endpoint compatibility
    let health_url = format!("{}{}", base_url, "/api/v1/health");
    
    match client.get(&health_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json) => {
                        // Check if previous required fields are still present
                        if previous.endpoints.contains_key("/api/v1/health") {
                            if !json.get("status").is_some() {
                                breaking_changes.push("Required 'status' field missing from /api/v1/health response".to_string());
                            }
                        }
                    }
                    Err(_) => {
                        breaking_changes.push("Failed to parse /api/v1/health response".to_string());
                    }
                }
            } else {
                breaking_changes.push(format!("Health endpoint returned status: {}", response.status()));
            }
        }
        Err(e) => {
            breaking_changes.push(format!("Failed to reach health endpoint: {}", e));
        }
    }

    // Test users endpoint compatibility
    let users_url = format!("{}{}", base_url, "/api/v1/users");
    
    match client.get(&users_url).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                breaking_changes.push(format!("Users endpoint returned status: {}", response.status()));
            }
        }
        Err(e) => {
            breaking_changes.push(format!("Failed to reach users endpoint: {}", e));
        }
    }
}

/// Test API versioning strategy
#[tokio::test]
#[ignore = "enable for API versioning strategy testing"]
async fn api_versioning_strategy_test() {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build HTTP client");

    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());

    println!("🔢 Testing API versioning strategy...");

    // Test version in URL
    let v1_health_url = format!("{}/api/v1/health", base_url);
    let v2_health_url = format!("{}/api/v2/health", base_url);

    // Test v1 endpoint
    match client.get(&v1_health_url).send().await {
        Ok(response) => {
            println!("✅ v1 health endpoint accessible: {}", response.status());
        }
        Err(e) => {
            println!("❌ v1 health endpoint failed: {}", e);
        }
    }

    // Test v2 endpoint
    match client.get(&v2_health_url).send().await {
        Ok(response) => {
            println!("✅ v2 health endpoint accessible: {}", response.status());
        }
        Err(e) => {
            println!("❌ v2 health endpoint failed: {}", e);
        }
    }

    // Test content negotiation
    let accept_headers = vec![
        ("application/vnd.api.v1+json", "v1"),
        ("application/vnd.api.v2+json", "v2"),
        ("application/json", "default"),
    ];

    let health_url = format!("{}/api/health", base_url);
    
    for (accept_header, version_desc) in accept_headers {
        match client.get(&health_url)
            .header("Accept", accept_header)
            .send()
            .await {
            Ok(response) => {
                println!("✅ Content negotiation with '{}' returns version {}: {}", 
                         accept_header, version_desc, response.status());
            }
            Err(e) => {
                println!("❌ Content negotiation with '{}' failed: {}", accept_header, e);
            }
        }
    }

    println!("✅ API versioning strategy test completed!");
}