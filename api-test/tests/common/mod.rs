use dotenvy::dotenv;
use std::env;
use reqwest::{Client, Url};

pub fn init() {
    let _ = dotenv();
}

pub fn base_url() -> Url {
    init();
    Url::parse(&env::var("BASE_URL").expect("BASE_URL not set")).expect("invalid BASE_URL")
}

pub fn client() -> Client {
    Client::builder()
        .cookie_store(true)
        .build()
        .expect("client build")
}

pub fn bearer(token_env: &str) -> String {
    format!(
        "Bearer {}",
        std::env::var(token_env).unwrap_or_else(|_| panic!("{token_env} not set"))
    )
}

pub fn path(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} not set"))
}
