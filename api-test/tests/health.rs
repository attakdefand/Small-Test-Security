mod common;
use common::*;
use http::StatusCode;

#[tokio::test]
async fn health_ok() {
    let url = base_url().join(&path("HEALTH_PATH")).unwrap();
    let res = client().get(url).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
