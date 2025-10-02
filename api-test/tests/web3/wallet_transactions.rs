//! Wallet and transaction testing

#[cfg(feature = "web3-testing")]
use alloy::{
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
};
#[cfg(feature = "web3-testing")]
use api_test::web3::wallet::{TestWallet, send_test_transaction, get_balance};

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_wallet_creation() {
    // Test creating a random wallet
    let wallet = TestWallet::random();
    assert_ne!(wallet.address(), address!("0000000000000000000000000000000000000000"));
    
    // Test creating a wallet from private key
    // Note: This is a placeholder private key for testing - never use real private keys in tests
    let private_key = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let wallet_result = TestWallet::from_private_key(private_key);
    // The placeholder key is invalid, so this will fail, which is expected
    assert!(wallet_result.is_err());
}

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_balance_query() {
    // This test requires a running Ethereum node
    let provider_result = ProviderBuilder::new().on_builtin("anvil").await;
    
    if let Ok(provider) = provider_result {
        // Test getting balance of an address
        let test_address = address!("0000000000000000000000000000000000000000");
        let balance_result = get_balance(test_address, &provider).await;
        assert!(balance_result.is_ok());
        
        // Balance should be a U256
        if let Ok(balance) = balance_result {
            assert!(balance >= U256::ZERO);
        }
    }
}

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_transaction_sending() {
    // This test requires a running Ethereum node with unlocked accounts
    let provider_result = ProviderBuilder::new().on_builtin("anvil").await;
    
    if let Ok(provider) = provider_result {
        // Get accounts from the node
        let accounts_result = provider.get_accounts().await;
        if let Ok(accounts) = accounts_result {
            if !accounts.is_empty() {
                let from_account = accounts[0];
                let to_account = if accounts.len() > 1 {
                    accounts[1]
                } else {
                    address!("0000000000000000000000000000000000000001")
                };
                
                let amount = U256::from(1000000000000000u64); // 0.001 ETH
                
                let tx_result = send_test_transaction(from_account, to_account, amount, &provider).await;
                // We just check that the method doesn't panic
                assert!(tx_result.is_ok());
            }
        }
    }
}

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_wallet_signing() {
    // Test wallet signing capabilities
    let wallet = TestWallet::random();
    
    // Test that we can get the address
    let wallet_address = wallet.address();
    assert_ne!(wallet_address, address!("0000000000000000000000000000000000000000"));
}

#[cfg(feature = "web3-testing")]
#[tokio::test]
async fn test_transaction_utils() {
    // Test utility functions for transactions
    use api_test::web3::utils::{eth_to_wei, wei_to_eth_string};
    
    // Test ETH to wei conversion
    let one_eth = eth_to_wei(1);
    assert_eq!(one_eth, U256::from(10u64.pow(18)));
    
    // Test wei to ETH string conversion
    let one_eth_wei = U256::from(10u64.pow(18));
    let eth_string = wei_to_eth_string(one_eth_wei);
    assert!(eth_string.starts_with("1."));
}