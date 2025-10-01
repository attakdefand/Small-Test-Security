mod common;
use common::*;
use http::StatusCode;

#[tokio::test]
async fn requires_auth() {
    let url = base_url().join(&path("ME_PATH")).unwrap();
    let res = client().get(url).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn user_can_get_self() {
    let url = base_url().join(&path("ME_PATH")).unwrap();
    let res = client().get(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn user_cannot_access_admin() {
    let url = base_url().join(&path("ADMIN_USERS_PATH")).unwrap();
    let res = client().get(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .send().await.unwrap();
    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn admin_can_access_admin() {
    let url = base_url().join(&path("ADMIN_USERS_PATH")).unwrap();
    let res = client().get(url)
        .header("Authorization", bearer("ADMIN_TOKEN"))
        .send().await.unwrap();
    assert!(res.status().is_success());
}
