use reqwest::Client;
use serde_json::Value;

#[tokio::test]
#[ignore = "enable when GraphQL endpoint is available"]
async fn graphql_query_test() {
    let client = Client::new();
    let base_url = std::env::var("BASE_URL").unwrap_or("http://localhost:8080".into());
    let graphql_endpoint = format!("{}/graphql", base_url);
    
    let query = r#"
    {
        users {
            id
            name
            email
        }
    }
    "#;
    
    let request_body = serde_json::json!({
        "query": query
    });
    
    let res = client
        .post(&graphql_endpoint)
        .json(&request_body)
        .send()
        .await
        .expect("Failed to send GraphQL request");
    
    assert_eq!(res.status(), 200);
    
    let response_body: Value = res.json().await.expect("Failed to parse JSON response");
    assert!(!response_body["data"]["users"].as_array().unwrap().is_empty());
}