mod common;
use common::*;
use http::StatusCode;
use insta::assert_json_snapshot;
use serde_json::{json, Value};

fn redact_keys_deep(value: &mut Value, keys: &[&str], replacement: &Value) {
    match value {
        Value::Object(map) => {
            for k in keys {
                if let Some(v) = map.get_mut(*k) {
                    *v = replacement.clone();
                }
            }
            for v in map.values_mut() {
                redact_keys_deep(v, keys, replacement);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                redact_keys_deep(v, keys, replacement);
            }
        }
        _ => {}
    }
}

#[tokio::test]
async fn me_response_shape() {
    let url = base_url().join(&path("ME_PATH")).unwrap();
    let res = client().get(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let mut json: Value = res.json().await.unwrap();

    // Redact volatile fields recursively (works for top-level or nested).
    redact_keys_deep(&mut json, &["id"], &Value::String("<redacted>".into()));
    redact_keys_deep(&mut json, &["created_at", "updated_at"], &Value::String("<ts>".into()));

    assert_json_snapshot!("me_response", json);
}
