mod owasp {
    pub mod common;
}


use std::time::Duration;

pub struct TestCfg {
    /// Base URL for your gateway/service under test.
    pub base: String,
}

impl Default for TestCfg {
    fn default() -> Self {
        // Adjust to your gateway address/port if different
        Self { base: std::env::var("API_BASE").unwrap_or_else(|_| "http://127.0.0.1:8080".into()) }
    }
}

pub fn endpoint(path: &str) -> String {
    format!("{}{}", TestCfg::default().base, path)
}

pub fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("client")
}
