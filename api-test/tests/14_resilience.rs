use wiremock::{MockServer, Mock, ResponseTemplate, matchers::{method, path}};
use std::time::Duration;

#[tokio::test]
async fn times_out_on_slow_upstream() {
    let ms = MockServer::start().await;
    Mock::given(method("GET")).and(path("/price"))
      .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(1500)))
      .mount(&ms).await;

    // Replace with your real retrying client & timeout policy
    let client = reqwest::Client::builder().timeout(Duration::from_millis(200)).build().unwrap();
    let err = client.get(format!("{}/price", ms.uri())).send().await.err();
    assert!(err.is_some(), "should time out");
}
