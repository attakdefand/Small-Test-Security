mod common;
use common::*;
use http::StatusCode;

#[tokio::test(flavor = "multi_thread")]
async fn rate_limit_trips() {
    let url = base_url().join(&path("RATELIMIT_PATH")).unwrap();
    let c = client();

    let mut last = StatusCode::OK;
    for _ in 0..25 {
        let res = c.get(url.clone())
            .header("Authorization", bearer("USER_TOKEN"))
            .send().await.unwrap();
        last = res.status();
        if last == StatusCode::TOO_MANY_REQUESTS {
            let retry = res.headers().get("Retry-After").and_then(|h| h.to_str().ok());
            assert!(retry.is_some(), "429 should include Retry-After");
            return;
        }
    }
    panic!("rate limit did not trigger; last status = {last}");
}
