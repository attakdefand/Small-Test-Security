# Web3 Testing Guide

This guide explains how to use the Web3 testing features added to the Rust testing framework.

## Overview

The Web3 testing framework provides comprehensive tools for testing Ethereum-based applications, including:

1. Blockchain interaction testing
2. Smart contract testing capabilities
3. Wallet and transaction testing
4. Security testing features
5. Utility functions for common Web3 operations

## Prerequisites

To use the Web3 testing features, you need:

1. An Ethereum node (like Geth, Erigon, or Anvil for local testing)
2. The `web3-testing` feature enabled in your Cargo.toml

## Enabling Web3 Testing

To enable Web3 testing features, add the following to your Cargo.toml:

```toml
[dependencies]
api-test = { path = "../api-test", features = ["web3-testing"] }

[features]
web3-tests = ["api-test/web3-testing"]
```

Then run tests with:

```bash
cargo test --features web3-tests
```

## Web3 Testing Modules

### 1. Provider Module

The provider module provides utilities for interacting with Ethereum nodes.

```rust
use api_test::web3::provider::{create_test_provider, create_default_test_provider, get_chain_id};

#[tokio::test]
async fn example_provider_usage() {
    // Create a provider with a custom RPC endpoint
    let provider = create_test_provider("http://localhost:8545").unwrap();
    
    // Or create a provider with the default endpoint (http://localhost:8545)
    let provider = create_default_test_provider().unwrap();
    
    // Get the chain ID
    let chain_id = get_chain_id(&provider).await.unwrap();
    println!("Chain ID: {}", chain_id);
}
```

### 2. Contract Module

The contract module provides utilities for testing smart contracts.

```rust
use api_test::web3::contract::TestContract;
use alloy::primitives::address;

#[tokio::test]
async fn example_contract_usage() {
    // Create a contract instance for testing
    let contract_address = address!("0000000000000000000000000000000000000000");
    
    let contract = TestContract::new(contract_address);
    
    // Get the contract address
    let address = contract.address();
}
```

### 3. Wallet Module

The wallet module provides utilities for testing wallets and transactions.

```rust
use api_test::web3::wallet::TestWallet;
use alloy::primitives::U256;

#[tokio::test]
async fn example_wallet_usage() {
    // Create a random test wallet
    let wallet = TestWallet::random();
    
    // Create a wallet from a private key (use with caution)
    let private_key = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let wallet = TestWallet::from_private_key(private_key);
    
    // Get wallet address
    let address = wallet.address();
    
    // Get wallet signer (if available)
    let signer = wallet.signer();
}
```

### 4. Security Module

The security module provides utilities for testing Web3 security features.

```rust
use api_test::web3::security::{is_known_scam_address, AttackSimulator};
use alloy::primitives::address;

#[tokio::test]
async fn example_security_usage() {
    // Check if an address is a known scam address
    let test_address = address!("0000000000000000000000000000000000000000");
    let is_scam = is_known_scam_address(test_address);
    
    // Simulate a reentrancy attack
    let contract_address = address!("0000000000000000000000000000000000000000");
    let attack_result = AttackSimulator::simulate_reentrancy_attack(contract_address).await;
}
```

### 5. Utils Module

The utils module provides utility functions for common Web3 operations.

```rust
use api_test::web3::utils::{eth_to_wei, wei_to_eth_string, generate_test_address};

#[test]
fn example_utils_usage() {
    // Convert ETH to wei
    let one_eth_in_wei = eth_to_wei(1);
    
    // Convert wei to ETH string
    let eth_string = wei_to_eth_string(one_eth_in_wei);
    
    // Generate a test address
    let test_address = generate_test_address(1);
}
```

## Running Web3 Tests

To run the Web3 tests, you need to have an Ethereum node running. For local development, we recommend using Anvil:

```bash
# Install Foundry (which includes Anvil)
curl -L https://foundry.paradigm.xyz | bash
foundryup

# Run Anvil in another terminal
anvil
```

Then run the tests:

```bash
cargo test --features web3-tests -- --test-threads=1
```

Note: We recommend using `--test-threads=1` to avoid issues with shared state between tests.

## Test Examples

### Blockchain Interaction Tests

```bash
cargo test --features web3-tests test_blockchain_connection
cargo test --features web3-tests test_get_balance
cargo test --features web3-tests test_get_block
cargo test --features web3-tests test_send_transaction
```

### Smart Contract Tests

```bash
cargo test --features web3-tests test_contract_creation
cargo test --features web3-tests test_erc20_contract
cargo test --features web3-tests test_contract_deployment
```

### Wallet and Transaction Tests

```bash
cargo test --features web3-tests test_wallet_creation
cargo test --features web3-tests test_transaction_sending
cargo test --features web3-tests test_wallet_signing
```

### Security Tests

```bash
cargo test --features web3-tests test_scam_address_detection
cargo test --features web3-tests test_reentrancy_attack_simulation
```

## Best Practices

1. **Use local testnets**: Always use local testnets like Anvil for testing to avoid spending real ETH.

2. **Enable features conditionally**: Use feature flags to enable Web3 testing only when needed.

3. **Handle network failures gracefully**: Web3 tests may fail due to network issues, so handle these cases appropriately.

4. **Use deterministic testing**: When possible, use deterministic test data to ensure consistent test results.

5. **Clean up test state**: Make sure to clean up any state changes made during tests.

## Troubleshooting

### Common Issues

1. **Connection refused**: Make sure your Ethereum node is running and accessible.

2. **Insufficient funds**: Ensure accounts have enough ETH for transactions.

3. **Nonce issues**: Use fresh accounts or reset the node state between tests.

### Debugging Tips

1. **Enable logging**: Use `RUST_LOG=debug` to get more detailed output.

2. **Check node logs**: Look at your Ethereum node logs for error messages.

3. **Use specific test filters**: Run individual tests to isolate issues.

## Advanced Features

### Custom Transport Layers

The framework supports different transport layers:

```rust
// HTTP transport (default)
let provider = ProviderBuilder::new().on_http("http://localhost:8545".parse().unwrap());

// WebSocket transport
let provider = ProviderBuilder::new().on_ws("ws://localhost:8546".parse().unwrap()).await?;

// IPC transport
let provider = ProviderBuilder::new().on_ipc("/path/to/ipc".parse().unwrap()).await?;
```

### Middleware Support

You can add middleware to providers for additional functionality:

```rust
use alloy::providers::ProviderLayer;

let provider = ProviderBuilder::new()
    .layer(SomeMiddleware::new())
    .on_http("http://localhost:8545".parse().unwrap());
```

## Conclusion

The Web3 testing framework provides a comprehensive set of tools for testing Ethereum-based applications. By following this guide, you should be able to effectively test your Web3 applications with confidence.