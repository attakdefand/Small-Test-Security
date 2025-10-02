//! Web3 provider utilities for testing blockchain interactions.

#[cfg(feature = "web3-testing")]
use alloy::{
    providers::{Provider, ProviderBuilder, RootProvider},
    transports::http::Http,
};

/// Create a provider for testing with a given RPC endpoint
#[cfg(feature = "web3-testing")]
pub fn create_test_provider(rpc_url: &str) -> Result<RootProvider<Http>, Box<dyn std::error::Error>> {
    let url = rpc_url.parse()?;
    Ok(ProviderBuilder::new().on_http(url))
}

/// Create a provider for testing with the default localhost RPC endpoint
#[cfg(feature = "web3-testing")]
pub fn create_default_test_provider() -> Result<RootProvider<Http>, Box<dyn std::error::Error>> {
    create_test_provider("http://localhost:8545")
}

/// Get the chain ID from the provider
#[cfg(feature = "web3-testing")]
pub async fn get_chain_id<P: Provider>(provider: &P) -> Result<u64, Box<dyn std::error::Error>> {
    let chain_id = provider.get_chain_id().await?;
    Ok(chain_id)
}

#[cfg(test)]
#[cfg(feature = "web3-testing")]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_provider() {
        // This test would require a running Ethereum node
        // For CI/CD, you might want to use Anvil or similar
        let provider = create_default_test_provider();
        // We just test that the function returns without panicking
        assert!(provider.is_ok())
    }
}