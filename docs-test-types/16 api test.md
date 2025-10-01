Yep—each of the 16 has its own “sub-types” (common variants/focus areas). Here’s a compact map so you can pick exactly what you need per endpoint/protocol in your Axum/SQLx stack.

# Sub-types per API-testing type

| #  | Parent type                 | Common sub-types (what you focus on)                                                                     | Typical Rust helpers                          |
| -- | --------------------------- | -------------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| 1  | **Unit (pure)**             | math/rounding • parsing/formatting • validators (email/UUID) • error mapping • feature flags             | `#[test]`, `assert_*`, `thiserror`, small fns |
| 2  | **Handler/component**       | happy-path • authZ branch (allow/deny) • input validation errors • DB-error branch • pagination branch   | `axum` handler fn + trait mocks (`mockall`)   |
| 3  | **Router (in-memory)**      | routing matrix • middleware order (CORS, auth) • header propagation • content-negotiation                | `tower::ServiceExt::oneshot`, `http`          |
| 4  | **Integration (real deps)** | migrations apply • repo methods • N+1 checks • transaction semantics (commit/rollback)                   | `testcontainers`, `sqlx::migrate!`, `PgPool`  |
| 5  | **End-to-End (E2E)**        | startup/health • TLS/CORS • env/feature flags • binary CLI flags • static files                          | spawn server on ephemeral port + `reqwest`    |
| 6  | **Contract**                | consumer-driven pact • provider verification • backward/forward compat • optional fields • error shapes  | `pact_consumer`, `pact_verifier`              |
| 7  | **OpenAPI conformance**     | schema ↔ impl drift • status codes • required/nullable • enum exhaustiveness • examples                  | `utoipa`/`poem-openapi` + `insta` snapshot    |
| 8  | **Snapshot (golden)**       | JSON bodies • HTML/text • error payloads • event payloads • OpenAPI text                                 | `insta` with redactions/filters               |
| 9  | **Property-based**          | pagination invariants • idempotency keys • sorting/ordering • amount bounds • round-trip (encode→decode) | `proptest` strategies + invariants            |
| 10 | **Fuzz**                    | parsers (JSON/CSV) • path/query handling • auth header parsing • binary decoders • state desync          | `cargo fuzz` libFuzzer harness                |
| 11 | **Performance/bench**       | hot handler path • serialization • DB query time • caching hit/miss • allocator pressure                 | `criterion` benches & flamegraph              |
| 12 | **Load/Soak**               | RPS/latency SLO • percentiles • leak detection (RSS) • connection churn • burst vs steady                | `goose` client, long-run `--run-time`         |
| 13 | **Security checks**         | headers (CSP, X-CT-O) • auth (JWT, HMAC) • path traversal • SSRF guards • rate-limit behavior            | `reqwest` asserts; ZAP/Burp (offline)         |
| 14 | **Resilience/Chaos**        | timeouts • retries/backoff • circuit breaker • partial failures • slowloris • jitter                     | `wiremock` delays/faults; your retry layer    |
| 15 | **Concurrency/Idempotency** | duplicate POST racing • optimistic locking • unique constraints • exactly-once effects                   | `tokio::join!`, DB unique + `RETURNING`       |
| 16 | **Migration/Data**          | forward/backward migration • seed data integrity • non-destructive changes • zero-downtime paths         | `sqlx::migrate!`, checksum assertions         |

---

## How to pick sub-types (quick rules)

* **External breaking risk?** Add **Contract + OpenAPI** sub-types.
* **Write endpoints?** Add **Concurrency/Idempotency** (race, dupes) and **Property** (invariants).
* **Upstream calls?** Add **Resilience** (timeouts/retries/circuit).
* **User input heavy?** Add **Handler: validation branches** + **Fuzz** (parsers).
* **Regulated/fintech (you):** Add **Security** (headers/JWT/HMAC), **Snapshot** (receipts), **Migration/Data**.

---

## Example: map to one REST endpoint (POST /orders)

* Unit: fee math, quantity rounding
* Handler: 200 (ok), 400 (validation), 401/403 (authZ), 409 (duplicate idempotency)
* Router: auth middleware before rate-limit, JSON content-type
* Integration: inserts roll back on failure, indices used
* E2E: server up, CORS allow your web origin
* Contract/OpenAPI: body schema, error schema
* Snapshot: created order JSON stable
* Property: total not negative; price*qty within bounds
* Fuzz: parser never panics
* Perf/Load: p99 latency under SLO @ target RPS
* Security: headers present; JWT required
* Resilience: upstream risk-engine timeout → 502 + retry policy
* Concurrency: duplicate idempotency-key → one row committed
* Migration: table/schema exist with expected defaults

If you want, tell me a specific service/route (e.g., `orders POST /v1/orders` in `api-gateway`) and I’ll generate the exact Rust test modules with the right sub-types wired to your workspace.
