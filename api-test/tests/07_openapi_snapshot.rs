use insta::assert_json_snapshot;

#[test]
fn openapi_stable() {
    // Replace with your real exporter, e.g., app::openapi()
    let spec = serde_json::json!({"openapi":"3.0.3","paths":{r"/health":{}}});
    assert_json_snapshot!("openapi.json", spec);
}