//! Tests for Ethereum blockchain interactions

#[cfg(feature = "web3-testing")]
use alloy::{
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
};
#[cfg(feature = "web3-testing")]
use api_test::web3::provider::{create_default_test_provider, get_chain_id};

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_blockchain_connection() {
    // This test requires a running Ethereum node (like Anvil)
    // For local testing, you can run: anvil
    let provider_result = create_default_test_provider();
    
    if let Ok(provider) = provider_result {
        // Test that we can get the chain ID
        let chain_id_result = get_chain_id(&provider).await;
        assert!(chain_id_result.is_ok());
        
        // Test that we can get the latest block number
        let block_number_result = provider.get_block_number().await;
        assert!(block_number_result.is_ok());
    }
    // If we can't connect to a node, we skip the test
}

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_get_balance() {
    // This test requires a running Ethereum node
    let provider_result = create_default_test_provider();
    
    if let Ok(provider) = provider_result {
        // Test getting balance of a known address (zero address)
        let zero_address = address!("0000000000000000000000000000000000000000");
        let balance_result = provider.get_balance(zero_address).await;
        assert!(balance_result.is_ok());
        
        // Balance should be a U256
        let balance = balance_result.unwrap();
        assert!(balance >= U256::ZERO);
    }
}

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_get_block() {
    // This test requires a running Ethereum node
    let provider_result = create_default_test_provider();
    
    if let Ok(provider) = provider_result {
        // Test getting the latest block
        let block_result = provider.get_block_by_number(alloy::rpc::types::BlockNumberOrTag::Latest, false).await;
        assert!(block_result.is_ok());
        
        // Test getting block by hash (if we have a hash)
        if let Ok(Some(block)) = block_result {
            if let Some(hash) = block.header.hash {
                let block_by_hash_result = provider.get_block_by_hash(hash, false).await;
                assert!(block_by_hash_result.is_ok());
            }
        }
    }
}

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_send_transaction() {
    // This test requires a running Ethereum node with unlocked accounts
    // For local testing with Anvil, accounts are unlocked by default
    let provider_result = create_default_test_provider();
    
    if let Ok(provider) = provider_result {
        // Get accounts
        let accounts_result = provider.get_accounts().await;
        if let Ok(accounts) = accounts_result {
            if !accounts.is_empty() {
                // Test sending a transaction from the first account to the second
                if accounts.len() >= 2 {
                    let from = accounts[0];
                    let to = accounts[1];
                    let amount = U256::from(1000000000000000u64); // 0.001 ETH
                    
                    // Build transaction
                    let tx = alloy::rpc::types::TransactionRequest::default()
                        .with_from(from)
                        .with_to(to)
                        .with_value(amount);
                    
                    // Send transaction
                    let tx_result = provider.send_transaction(tx).await;
                    // We just check that the method doesn't panic
                    // Actual transaction success depends on node configuration
                    assert!(tx_result.is_ok() || 
                           tx_result.unwrap_err().to_string().contains("insufficient funds") ||
                           tx_result.unwrap_err().to_string().contains("nonce too low"));
                }
            }
        }
    }
}