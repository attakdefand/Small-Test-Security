//! Utility functions for web3 testing.

#[cfg(feature = "web3-testing")]
use alloy::{
    primitives::{Address, U256},
};

/// Convert ETH to wei
#[cfg(feature = "web3-testing")]
pub fn eth_to_wei(eth: u64) -> U256 {
    // 1 ETH = 10^18 wei
    let wei_per_eth = U256::from(10u64.pow(18));
    U256::from(eth) * wei_per_eth
}

/// Format wei to ETH string
#[cfg(feature = "web3-testing")]
pub fn wei_to_eth_string(wei: U256) -> String {
    // 1 ETH = 10^18 wei
    let eth = wei.to_string();
    if eth.len() <= 18 {
        format!("0.{}", "0".repeat(18 - eth.len()) + &eth)
    } else {
        let whole = &eth[..eth.len() - 18];
        let fractional = &eth[eth.len() - 18..];
        format!("{}.{}", whole, fractional)
    }
}

/// Generate a test address
#[cfg(feature = "web3-testing")]
pub fn generate_test_address(index: u8) -> Address {
    let mut bytes = [0u8; 20];
    bytes[19] = index;
    Address::from(bytes)
}

#[cfg(test)]
#[cfg(feature = "web3-testing")]
mod tests {
    use super::*;

    #[test]
    fn test_eth_to_wei() {
        let one_eth = eth_to_wei(1);
        assert_eq!(one_eth, U256::from(10u64.pow(18)));
    }

    #[test]
    fn test_wei_to_eth_string() {
        let one_eth_wei = U256::from(10u64.pow(18));
        let eth_string = wei_to_eth_string(one_eth_wei);
        assert_eq!(eth_string, "1.000000000000000000");
    }

    #[test]
    fn test_generate_test_address() {
        let addr1 = generate_test_address(1);
        let addr2 = generate_test_address(2);
        assert_ne!(addr1, addr2);
        assert_eq!(addr1.0[19], 1);
        assert_eq!(addr2.0[19], 2);
    }
}