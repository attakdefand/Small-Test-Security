#![cfg(feature = "contracts")]
use pact_consumer::prelude::*;
use serde_json::json;

#[tokio::test]
async fn gets_user() {
    let pact = PactBuilder::new("gateway","user-service")
      .interaction("get user 42", |i| {
        i.request.path("/users/42");
        i.response.status(200).header("content-type","application/json")
          .json_body(json!({"id":42,"name":"Alice"}));
      }).build();

    pact.run(|ms| async move {
        let got: serde_json::Value = reqwest::get(format!("{}/users/42", ms.path(""))).await.unwrap().json().await.unwrap();
        assert_eq!(got["id"], 42);
    }).await;
}
