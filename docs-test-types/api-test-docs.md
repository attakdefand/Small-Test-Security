Great—once your team hands you the running API (a base URL and any keys/tokens), you can slam it with an automated Rust test pack that checks both correctness and the most important **security behaviors** (authn/authz, rate-limits, headers, input validation, IDOR, etc.).

Below is a ready-to-drop **`api-test/` crate** you can add to your workspace and run immediately.

---

# 1) Create the test crate

```bash
# from repo root (same level as services/, libs/, etc.)
cargo new api-test --lib
```

Add it to your workspace `Cargo.toml` (top level):

```toml
[workspace]
members = [
  # ...your existing crates...
  "api-test"
]
```

`api-test/Cargo.toml`:

```toml
[package]
name = "api-test"
version = "0.1.0"
edition = "2021"

[lib]
# Only integration tests in /tests; no lib code exported.
path = "src/lib.rs"

[dependencies]
reqwest = { version = "0.12", features = ["json", "gzip", "brotli", "deflate", "cookies", "rustls-tls"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
http = "1"
dotenvy = "0.15"
chrono = { version = "0.4", features = ["clock"] }
jsonwebtoken = "9"
hmac = "0.12"
sha2 = "0.10"
rand = "0.8"
uuid = { version = "1", features = ["v4"] }
regex = "1"

# testing utilities
tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
insta = { version = "1", features = ["redactions", "json"] }
proptest = "1"

[dev-dependencies]
# sometimes handy to separate, but ok as regular deps above
```

Create files:

```bash
mkdir -p api-test/tests/common
touch api-test/src/lib.rs
touch api-test/tests/common/mod.rs
touch api-test/tests/health.rs
touch api-test/tests/auth_and_rbac.rs
touch api-test/tests/rate_limit.rs
touch api-test/tests/security_headers.rs
touch api-test/tests/cors.rs
touch api-test/tests/idor.rs
touch api-test/tests/input_validation.rs
touch api-test/tests/pagination_and_limits.rs
touch api-test/tests/upload_size.rs
touch api-test/tests/json_shape_snapshot.rs
touch api-test/tests/fuzz_props.rs
```

`.env.sample` (put in `api-test/`):

```
BASE_URL=https://api.example.com
PUBLIC_ORIGIN=https://admin.example.com
USER_TOKEN=eyJhbGciOi...
ADMIN_TOKEN=eyJhbGciOi...
API_KEY=your-hmac-key
# for time-skew tests
ALLOWED_SKEW_SECS=60
# endpoints (adjust to your API)
HEALTH_PATH=/health
ME_PATH=/v1/me
ADMIN_USERS_PATH=/v1/admin/users
USER_RESOURCE_PATH=/v1/users
QUOTES_PATH=/v1/quotes
UPLOAD_PATH=/v1/upload
RATELIMIT_PATH=/v1/quotes
```

Copy to `.env` and fill real values.

---

# 2) Shared helpers (`tests/common/mod.rs`)

```rust
use dotenvy::dotenv;
use std::env;
use reqwest::{Client, Url};

pub fn init() {
    let _ = dotenv();
    // set sensible defaults if missing
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
```

`src/lib.rs` can stay empty (no public API needed).

---

# 3) Example test suites (security-focused)

### Health (smoke)

```rust
// tests/health.rs
mod common;
use common::*;
use http::StatusCode;

#[tokio::test]
async fn health_ok() {
    let url = base_url().join(&path("HEALTH_PATH")).unwrap();
    let res = client().get(url).send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
```

### AuthN + RBAC (401/403 correctness)

```rust
// tests/auth_and_rbac.rs
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
```

### Rate limiting (expect 429 + Retry-After)

```rust
// tests/rate_limit.rs
mod common;
use common::*;
use http::StatusCode;

#[tokio::test(flavor = "multi_thread")]
async fn rate_limit_trips() {
    let url = base_url().join(&path("RATELIMIT_PATH")).unwrap();
    let c = client();

    // Burst 25 requests; tune to your policy
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
```

### Security headers (no-sniff, referrer-policy, HSTS if TLS)

```rust
// tests/security_headers.rs
mod common;
use common::*;
use http::HeaderMap;

#[tokio::test]
async fn baseline_security_headers_present() {
    let url = base_url().join("/").unwrap();
    let res = client().get(url).send().await.unwrap();
    let headers: &HeaderMap = res.headers();

    // Loosely check; adjust to your policy
    assert!(headers.get("x-content-type-options").is_some(), "nosniff missing");
    assert!(headers.get("referrer-policy").is_some(), "referrer-policy missing");
    // If served behind TLS/app-gateway:
    // assert!(headers.get("strict-transport-security").is_some(), "HSTS missing");
    // Optional CSP if you configured it:
    // assert!(headers.get("content-security-policy").is_some(), "CSP missing");
}
```

### CORS preflight

```rust
// tests/cors.rs
mod common;
use common::*;
use http::{HeaderValue, Method, StatusCode};

#[tokio::test]
async fn cors_preflight_ok() {
    let url = base_url().join(&path("QUOTES_PATH")).unwrap();
    let origin = std::env::var("PUBLIC_ORIGIN").expect("PUBLIC_ORIGIN");
    let res = client()
        .request(Method::OPTIONS, url)
        .header("Origin", origin)
        .header("Access-Control-Request-Method", "GET")
        .send().await.unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT, "preflight status");
    let h = res.headers();
    assert!(h.get("access-control-allow-origin").is_some());
    assert!(h.get("access-control-allow-methods").is_some());
}
```

### IDOR (user A must not read user B)

```rust
// tests/idor.rs
mod common;
use common::*;
use http::StatusCode;

#[tokio::test]
async fn cannot_read_other_users_record() {
    // Pick a different user ID that the test-user doesn't own
    let other_id = "00000000-0000-4000-8000-000000000999";
    let path = format!("{}/{}", std::env::var("USER_RESOURCE_PATH").unwrap(), other_id);
    let url = base_url().join(&path).unwrap();

    let res = client().get(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .send().await.unwrap();

    assert!(matches!(res.status(), StatusCode::FORBIDDEN | StatusCode::NOT_FOUND));
}
```

### Input validation (reject injections, oversize, bad JSON)

```rust
// tests/input_validation.rs
mod common;
use common::*;
use http::StatusCode;
use serde_json::json;

#[tokio::test]
async fn rejects_sqlish_strings() {
    let url = base_url().join(&path("QUOTES_PATH")).unwrap();
    let body = json!({ "symbol": "BTCUSDT' OR 1=1 --", "amount": "1e9999" });

    let res = client().post(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .json(&body)
        .send().await.unwrap();

    assert!(matches!(res.status(), StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY));
    let text = res.text().await.unwrap();
    assert!(!text.to_lowercase().contains("stacktrace"), "should not leak stack traces");
}

#[tokio::test]
async fn rejects_malformed_json() {
    use reqwest::Body;
    let url = base_url().join(&path("QUOTES_PATH")).unwrap();

    let res = client().post(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .header("Content-Type", "application/json")
        .body(Body::from("{ not: valid json"))
        .send().await.unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
```

### Pagination & hard limits (no unbounded responses)

```rust
// tests/pagination_and_limits.rs
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
    // Ideally the API echoes the effective limit header
    if let Some(limit) = res.headers().get("X-Effective-Limit") {
        let n: u32 = limit.to_str().unwrap().parse().unwrap();
        assert!(n <= 1000, "server must enforce a sane cap");
    }
}
```

### Upload size limit (413)

```rust
// tests/upload_size.rs
mod common;
use common::*;
use http::StatusCode;

#[tokio::test]
async fn large_upload_rejected() {
    let url = base_url().join(&path("UPLOAD_PATH")).unwrap();
    let too_big = vec![0u8; 15 * 1024 * 1024]; // 15MB (tune policy)
    let res = client().post(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .header("Content-Type", "application/octet-stream")
        .body(too_big)
        .send().await.unwrap();

    assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
}
```

### JSON shape snapshots (compatibility guard)

```rust
// tests/json_shape_snapshot.rs
mod common;
use common::*;
use http::StatusCode;
use insta::{assert_json_snapshot, with_settings};

#[tokio::test]
async fn me_response_shape() {
    let url = base_url().join(&path("ME_PATH")).unwrap();
    let res = client().get(url)
        .header("Authorization", bearer("USER_TOKEN"))
        .send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let json: serde_json::Value = res.json().await.unwrap();

    // redact volatile fields
    with_settings!({filters => vec![
        (r#""id":"[^"]+""#, r#""id":"<redacted>""#),
        (r#""updated_at":"[^"]+""#, r#""updated_at":"<ts>""#),
        (r#""created_at":"[^"]+""#, r#""created_at":"<ts>""#)
    ]}, {
        assert_json_snapshot!("me_response", json);
    });
}
```

### Property/fuzz checks (edge inputs never 500)

```rust
// tests/fuzz_props.rs
mod common;
use common::*;
use proptest::prelude::*;
use http::StatusCode;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]
    #[test]
    fn never_500_on_symbol(symbol in "[A-Za-z0-9._-]{0,32}") {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let base = base_url();
            let mut url = base.join(&path("QUOTES_PATH")).unwrap();
            url.set_query(Some(&format!("symbol={}", symbol)));
            let res = client().get(url)
                .header("Authorization", bearer("USER_TOKEN"))
                .send().await.unwrap();
            assert_ne!(res.status(), StatusCode::INTERNAL_SERVER_ERROR, "no 500s for weird inputs");
        });
    }
}
```

---

# 4) Run the suite

```bash
# from repo root
cd api-test
cp .env.sample .env   # fill values
cargo test -p api-test

# Snapshot tests: first accept, later verify
cargo install cargo-insta
cargo insta test --accept
cargo insta test

# Some tests (rate-limit) may be sensitive to concurrency:
cargo test -p api-test -- --test-threads=1
```

---

# 5) What security behaviors these tests cover

* **Authentication required** (401) + **RBAC** (403).
* **IDOR** prevention (no cross-tenant/cross-user reads).
* **Input validation**: rejects injectiony strings, malformed JSON, absurd numbers; never leaks stack traces.
* **Rate limiting**: 429 with `Retry-After`.
* **Security headers**: `X-Content-Type-Options: nosniff`, `Referrer-Policy`, (optionally `HSTS`, `CSP`).
* **CORS**: only allowed origins/methods; preflight behavior is correct.
* **Resource bounding**: page size caps, 413 on oversized uploads.
* **Schema/compatibility**: response shape snapshots (prevents accidental breaking changes).
* **Resilience to weird inputs**: property tests assert “no 500s”.

---

# 6) Nice add-ons (optional but recommended)

* **External scanners**: run OWASP ZAP or Nuclei in CI against a staging URL; keep these separate from your Rust tests but gate merges on basic pass/fail.
* **k6 or oha load tests**: confirm rate limits, latency SLOs, and error budgets.
* **Coverage/CI**: add a CI job (`cargo test -p api-test`) on every PR; publish insta snapshots as artifacts.
* **Secrets discipline**: never hard-code tokens in tests; always pull from `.env` / CI secrets.

---

If you want, I can also tailor these files to your exact endpoint names and auth scheme (JWT vs HMAC timestamp signatures) and drop in a ready-made `.env.sample` for your rx-exchange services.
