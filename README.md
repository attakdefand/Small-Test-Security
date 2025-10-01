# Small-Test-Security

A comprehensive API testing framework built in Rust for testing REST APIs, GraphQL endpoints, and WebSocket connections.

## Features

- ✅ Unit Testing
- ✅ Integration Testing
- ✅ Security Testing (OWASP Top 10)
- ✅ Performance Testing with k6
- ✅ GraphQL Testing
- ✅ WebSocket Testing
- ✅ Chaos Engineering
- ✅ Property-Based Testing
- ✅ Snapshot Testing
- ✅ Automated Reporting
- ✅ Distributed Load Testing
- ✅ Advanced Security Scanning
- ✅ API Contract Evolution
- ✅ Advanced Monitoring & Alerting
- ✅ API Documentation Generation
- ✅ Test Data Management
- ✅ Performance Profiling

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads)
- [k6](https://k6.io/docs/getting-started/installation/) (optional, for load testing)

## Setup

1. Clone this repository:
   ```bash
   git clone https://github.com/attakdefand/Small-Test-Security.git
   cd Small-Test-Security
   ```

2. Navigate to the API test directory:
   ```bash
   cd super-test-types-rust/api-test
   ```

3. Configure your API details in the `.env` file:
   ```bash
   cp .env.sample .env
   # Edit .env with your API information
   ```

4. Check that everything compiles:
   ```bash
   cargo check
   ```

## Running Tests

### Basic Tests
```bash
# Run basic functionality tests
cargo test
```

### Security Tests
```bash
# Run security tests (requires API to be running)
cargo test -- --ignored
```

### Extended Tests
```bash
# Run extended tests with reporting
./run-extended-tests.ps1
```

### Advanced Comprehensive Tests
The framework includes 7 advanced testing extensions:

1. **Distributed Load Testing**
   ```bash
   cargo test --features performance-testing distributed_load_test_simulation -- --ignored
   ```

2. **Advanced Security Scanning**
   ```bash
   cargo test --features security-testing advanced_security_vulnerability_scan -- --ignored
   ```

3. **API Contract Evolution**
   ```bash
   cargo test api_contract_backward_compatibility_test -- --ignored
   ```

4. **Advanced Monitoring & Alerting**
   ```bash
   cargo test --features monitoring advanced_api_monitoring_and_alerting -- --ignored
   ```

5. **Advanced API Documentation**
   ```bash
   cargo test advanced_api_documentation_generation -- --ignored
   ```

6. **Advanced Test Data Management**
   ```bash
   cargo test advanced_test_data_management_system -- --ignored
   ```

7. **Advanced Performance Profiling**
   ```bash
   cargo test --features performance-testing advanced_api_performance_profiling -- --ignored
   ```

### Run All Advanced Tests
```bash
# Run all advanced tests with all features
cargo test --features all -- --ignored
```

### View Results
```bash
# View test results dashboard
./test-dashboard.ps1
```

## Project Structure

```
super-test-types-rust/
├── api-test/                 # Main testing framework
│   ├── Cargo.toml           # Dependencies
│   ├── .env                 # Configuration
│   ├── tests/               # Test files
│   │   ├── 01_unit.rs       # Unit tests
│   │   ├── 02_handler.rs    # Handler tests
│   │   ├── 03_router.rs     # Router tests
│   │   └── ...              # Other test files
│   ├── k6-tests/            # Load testing scripts
│   └── test-results/        # Test reports
├── docs-test-types/         # Documentation
└── src/                     # Main library code
```

## Comprehensive Testing Guide

For detailed information about each of the 7 advanced testing extensions, see [COMPREHENSIVE-TESTING-GUIDE.md](api-test/COMPREHENSIVE-TESTING-GUIDE.md)

## Configuration

The framework uses environment variables for configuration. See `.env.sample` for all available options.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a pull request

## License

This project is licensed under the MIT License.