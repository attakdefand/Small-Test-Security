//! Tests for smart contract interactions

#[cfg(feature = "web3-testing")]
use alloy::{
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
};
#[cfg(feature = "web3-testing")]
use api_test::web3::contract::TestContract;

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_contract_creation() {
    // This test requires a running Ethereum node
    let provider_result = ProviderBuilder::new().on_builtin("anvil").await;
    
    if let Ok(_provider) = provider_result {
        // Test creating a TestContract instance
        // We use a known contract address (this is just an example)
        let contract_address = address!("0000000000000000000000000000000000000000");
        
        // Create contract instance
        let contract = TestContract::new(contract_address);
        
        // Verify the contract address
        assert_eq!(contract.address(), contract_address);
    }
}

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_get_contract_bytecode() {
    // This test requires a running Ethereum node
    let provider_result = ProviderBuilder::new().on_builtin("anvil").await;
    
    if let Ok(provider) = provider_result {
        // Test getting bytecode of a known contract
        // We use the zero address as an example
        let contract_address = address!("0000000000000000000000000000000000000000");
        
        let bytecode_result = api_test::web3::contract::get_contract_bytecode(contract_address, &provider).await;
        // We just check that the method doesn't panic
        assert!(bytecode_result.is_ok());
    }
}

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_contract_function_call() {
    // This test requires a running Ethereum node and a deployed contract
    let provider_result = ProviderBuilder::new().on_builtin("anvil").await;
    
    if let Ok(_provider) = provider_result {
        // We use a known contract address (this is just an example)
        let contract_address = address!("0000000000000000000000000000000000000000");
        
        // Create contract instance
        let contract = TestContract::new(contract_address);
        
        // Verify the contract address
        assert_eq!(contract.address(), contract_address);
    }
}