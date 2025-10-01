#[tokio::test]
#[ignore] // unignore when server exposes these headers
async fn security_headers_present() {
    let base = std::env::var("BASE_URL").unwrap_or("http://127.0.0.1:8080".into());
    let res = reqwest::get(format!("{base}/health")).await.unwrap();
    let h = res.headers();
    assert_eq!(h.get("x-content-type-options").map(|v| v == "nosniff"), Some(true));
    assert_eq!(h.get("referrer-policy").map(|v| v == "no-referrer"), Some(true));
}
