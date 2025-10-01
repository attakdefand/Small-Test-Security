mod common;
use common::*;
use http::StatusCode;

#[tokio::test]
async fn page_size_is_capped() {
    let url = base_url().join(&format!("{}?limit=100000", std::env::var("ADMIN_USERS_PATH").unwrap())).unwrap();
    let res = client().get(url)
        .header("Authorization", bearer("ADMIN_TOKEN"))
        .send().await.unwrap();

    assert!(res.status().is_success());
    if let Some(limit) = res.headers().get("X-Effective-Limit") {
        let n: u32 = limit.to_str().unwrap().parse().unwrap();
        assert!(n <= 1000, "server must enforce a sane cap");
    }
}
