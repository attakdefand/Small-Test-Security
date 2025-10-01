# Advanced Testing Guide

This guide explains how to use the advanced testing capabilities in this API testing framework.

## Overview

The framework now includes several advanced testing modules:

1. **Distributed Load Testing** - Simulates multiple concurrent users
2. **Advanced Security Scanning** - Tests for common vulnerabilities
3. **API Contract Evolution** - Ensures backward compatibility
4. **Advanced Monitoring & Alerting** - Real-time monitoring simulation
5. **Advanced API Documentation** - OpenAPI specification generation
6. **Advanced Test Data Management** - Data generation and anonymization
7. **Advanced Performance Profiling** - Performance analysis and optimization

## Prerequisites

1. Rust toolchain installed
2. A running API service to test against
3. Environment variables configured in `.env` file

## Running Advanced Tests

### Basic Setup

First, ensure your `.env` file is configured:
```bash
BASE_URL=http://localhost:8080
HEALTH_PATH=/health
USER_TOKEN=your-test-token
```

### Running Tests by Category

#### 1. Performance Testing
```bash
# Run performance-related tests
cargo test --features performance-testing -- --ignored
```

#### 2. WebSocket Testing
```bash
# Run WebSocket tests
cargo test --features websocket-testing -- --ignored
```

#### 3. Security Testing
```bash
# Run security tests (may require OpenSSL)
cargo test --features security-testing -- --ignored
```

#### 4. Monitoring Tests
```bash
# Run monitoring-related tests
cargo test --features monitoring -- --ignored
```

#### 5. GraphQL Testing
```bash
# Run GraphQL tests
cargo test --features graphql-testing -- --ignored
```

#### 6. All Features
```bash
# Run all tests with all features enabled
cargo test --features all -- --ignored
```

### Running Specific Tests

#### Distributed Load Testing
```bash
cargo test --features performance-testing distributed_load_test_simulation -- --ignored
```

#### Advanced Security Scanning
```bash
cargo test --features security-testing advanced_security_vulnerability_scan -- --ignored
```

#### API Contract Evolution
```bash
cargo test api_contract_backward_compatibility_test -- --ignored
```

#### Advanced Monitoring & Alerting
```bash
cargo test --features monitoring advanced_api_monitoring_and_alerting -- --ignored
```

#### Advanced API Documentation
```bash
cargo test advanced_api_documentation_generation -- --ignored
```

#### Advanced Test Data Management
```bash
cargo test advanced_test_data_management_system -- --ignored
```

#### Advanced Performance Profiling
```bash
cargo test --features performance-testing advanced_api_performance_profiling -- --ignored
```

## Test Output

Tests generate various outputs:

- Terminal logs with detailed results
- JSON files for test data exports
- OpenAPI specifications
- Performance flamegraphs
- HTML reports (in some cases)

## Troubleshooting

### OpenSSL Issues on Windows

If you encounter OpenSSL-related errors on Windows, skip security tests:
```bash
cargo test --features "performance-testing,monitoring,websocket-testing" -- --ignored
```

### Feature Conflicts

If you encounter issues with conflicting features, run tests with individual features:
```bash
cargo test --features performance-testing -- --ignored
cargo test --features monitoring -- --ignored
```

## CI/CD Integration

For CI/CD pipelines, run all tests with all features:
```bash
cargo test --features all -- --ignored --nocapture
```

The `--nocapture` flag shows all output, which is useful for CI logs.

## Test Descriptions

### Distributed Load Testing
Simulates multiple concurrent users hitting API endpoints with configurable load patterns.

### Advanced Security Scanning
Tests for common vulnerabilities including SQL injection, XSS, command injection, and authentication bypass.

### API Contract Evolution
Ensures API changes maintain backward compatibility and detects breaking changes.

### Advanced Monitoring & Alerting
Simulates real-time monitoring with custom metrics and alerting rules.

### Advanced API Documentation
Generates OpenAPI specifications and validates documentation compliance.

### Advanced Test Data Management
Provides data generation, anonymization, versioning, and lifecycle management.

### Advanced Performance Profiling
Analyzes API performance under various load conditions and generates profiling data.

## Environment Variables

Some tests require specific environment variables:

- `BASE_URL`: The base URL of your API (default: http://localhost:8080)
- `HEALTH_PATH`: The health check endpoint path (default: /health)
- `USER_TOKEN`: Authentication token for protected endpoints
- `WEBSOCKET_URL`: WebSocket endpoint URL for WebSocket tests
- `TRADING_PATH`: Trading endpoint path for trading tests

## Best Practices

1. Always run tests against a test environment, not production
2. Configure appropriate timeouts for your API
3. Use the `--ignored` flag to run advanced tests
4. Enable only the features you need to avoid dependency conflicts
5. Review test output and generated files for insights