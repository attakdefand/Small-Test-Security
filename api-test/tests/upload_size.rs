mod common;
use common::*;
use http::StatusCode;

#[tokio::test]
async fn large_upload_rejected() {
    let url = base_url().join(&path("UPLOAD_PATH")).unwrap();
    let too_big = vec![0u8; 15 * 1024 * 1024]; // 15MB
    let res = client().post(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .header("Content-Type", "application/octet-stream")
        .body(too_big)
        .send().await.unwrap();

    assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
}
