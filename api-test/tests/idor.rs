mod common;
use common::*;
use http::StatusCode;

#[tokio::test]
async fn cannot_read_other_users_record() {
    let other_id = "00000000-0000-4000-8000-000000000999";
    let path = format!("{}/{}", std::env::var("USER_RESOURCE_PATH").unwrap(), other_id);
    let url = base_url().join(&path).unwrap();

    let res = client().get(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .send().await.unwrap();

    assert!(matches!(res.status(), StatusCode::FORBIDDEN | StatusCode::NOT_FOUND));
}
