# Running Advanced Tests

This document explains how to run the advanced tests in this API testing framework.

## Prerequisites

Before running the advanced tests, make sure you have:

1. A running API service to test against
2. Environment variables configured in your `.env` file:
   ```
   BASE_URL=http://localhost:8080
   HEALTH_PATH=/health
   USER_TOKEN=your-test-token
   ```

## Running Tests with Features

The advanced tests are organized into feature groups that can be enabled selectively:

### 1. Basic Tests (No Features Required)
```bash
# Run simple advanced tests
cargo test simple_advanced_test -- --ignored
```

### 2. Performance Testing
```bash
# Run performance-related tests
cargo test --features performance-testing -- --ignored
```

### 3. WebSocket Testing
```bash
# Run WebSocket tests
cargo test --features websocket-testing -- --ignored
```

### 4. Security Testing
```bash
# Run security tests (may require OpenSSL on some platforms)
cargo test --features security-testing -- --ignored
```

### 5. Monitoring Tests
```bash
# Run monitoring-related tests
cargo test --features monitoring -- --ignored
```

### 6. GraphQL Testing
```bash
# Run GraphQL tests
cargo test --features graphql-testing -- --ignored
```

### 7. All Features
```bash
# Run all tests with all features enabled
cargo test --features all -- --ignored
```

## Running Specific Test Suites

### Distributed Load Testing
```bash
cargo test --features performance-testing distributed_load_test_simulation -- --ignored
```

### Advanced Security Scanning
```bash
cargo test --features security-testing advanced_security_vulnerability_scan -- --ignored
```

### API Contract Evolution
```bash
cargo test api_contract_backward_compatibility_test -- --ignored
```

### Advanced Monitoring & Alerting
```bash
cargo test --features monitoring advanced_api_monitoring_and_alerting -- --ignored
```

### Advanced API Documentation
```bash
cargo test advanced_api_documentation_generation -- --ignored
```

### Advanced Test Data Management
```bash
cargo test advanced_test_data_management_system -- --ignored
```

### Advanced Performance Profiling
```bash
cargo test --features performance-testing advanced_api_performance_profiling -- --ignored
```

## Environment Variables

Some tests require specific environment variables:

- `BASE_URL`: The base URL of your API (default: http://localhost:8080)
- `HEALTH_PATH`: The health check endpoint path (default: /health)
- `USER_TOKEN`: Authentication token for protected endpoints
- `WEBSOCKET_URL`: WebSocket endpoint URL for WebSocket tests
- `TRADING_PATH`: Trading endpoint path for trading tests

## Test Output

Test results are displayed in the terminal. Some tests may generate additional output files:

- `test-data-export.json`: Exported test data
- `openapi-spec.json`: Generated OpenAPI specification
- `api-performance-flamegraph.svg`: Performance profiling flamegraph

## Troubleshooting

### OpenSSL Issues on Windows
If you encounter OpenSSL-related errors on Windows, you can skip security tests:
```bash
cargo test --features "performance-testing,monitoring,websocket-testing" -- --ignored
```

### Feature Conflicts
If you encounter issues with conflicting features, try running tests with individual features:
```bash
cargo test --features performance-testing -- --ignored
cargo test --features monitoring -- --ignored
```

## Continuous Integration

For CI/CD pipelines, you can run all tests with all features:
```bash
cargo test --features all -- --ignored --nocapture
```

The `--nocapture` flag shows all output, which is useful for CI logs.