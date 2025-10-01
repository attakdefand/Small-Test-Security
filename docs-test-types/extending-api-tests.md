# Extending the API Testing Framework

This document explains how to extend the existing API testing framework to cover additional testing scenarios.

For a complete guide to all possible extensions and making the framework production-ready, see [complete-extension-guide.md](file:///c:/Users/RMT/Documents/vscodium/Master-Test-Cases-Rust/super-test-types-rust/docs-test-types/complete-extension-guide.md).

## Current Coverage

The framework currently covers:
- Unit testing
- Integration testing
- Security testing (OWASP Top 10)
- Contract testing
- Snapshot testing
- Property-based testing
- Authentication/Authorization testing
- CORS testing
- Rate limiting testing
- Input validation testing
- Header security testing
- IDOR testing

## Extension Points

### 1. Load Testing with k6

Directory: `k6-tests/`
Add k6 JavaScript tests for performance and load testing.

Example:
```javascript
import http from 'k6/http';
import { check } from 'k6';

export default function () {
  const res = http.get('${BASE_URL}/health');
  check(res, { 'status is 200': (r) => r.status === 200 });
}
```

Run with: `k6 run k6-tests/basic-load-test.js`

### 2. GraphQL Testing

File: `tests/graphql.rs`
Test GraphQL endpoints and queries.

### 3. WebSocket Testing

File: `tests/websocket.rs`
Test real-time WebSocket connections and messaging.

### 4. Advanced Fuzzing

File: `tests/advanced_fuzzing.rs`
Extended property-based testing with more complex input generation.

### 5. API Documentation Validation

File: `tests/openapi_validation.rs`
Validate that API endpoints match OpenAPI/Swagger specifications.

### 6. Chaos Engineering

File: `tests/chaos_engineering.rs`
Test API resilience under various failure conditions.

## Adding New Test Categories

1. **Create a new test file** in the `tests/` directory
2. **Use environment variables** for configuration (BASE_URL, etc.)
3. **Mark with #[ignore]** if they require external services
4. **Add required dependencies** to Cargo.toml
5. **Document the test purpose** in comments

## Best Practices for Extensions

1. **Environment Configuration**: Always use environment variables for service URLs and credentials
2. **Graceful Failures**: Tests should fail gracefully with clear error messages
3. **Performance Considerations**: Don't overload external services during testing
4. **Security**: Never commit real credentials to version control
5. **Documentation**: Keep documentation updated with new testing capabilities

## Running Extended Tests

```bash
# Run all tests including ignored ones
cargo test -- --ignored

# Run specific extended test
cargo test graphql_query_test

# Run k6 load tests (separate tool)
k6 run k6-tests/basic-load-test.js
```