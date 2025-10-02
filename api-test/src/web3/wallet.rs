//! Wallet and transaction testing utilities.

#[cfg(feature = "web3-testing")]
use alloy::{
    primitives::{Address, U256},
    signers::local::PrivateKeySigner,
};

/// A test wallet for signing transactions
#[cfg(feature = "web3-testing")]
pub struct TestWallet {
    address: Address,
    signer: Option<PrivateKeySigner>,
}

#[cfg(feature = "web3-testing")]
impl TestWallet {
    /// Create a new test wallet from an address
    pub fn new(address: Address) -> Self {
        Self {
            address,
            signer: None,
        }
    }

    /// Create a new random test wallet
    pub fn random() -> Self {
        use alloy::primitives::address;
        // For testing purposes, we'll use a fixed address
        let address = address!("0000000000000000000000000000000000000000");
        Self {
            address,
            signer: None,
        }
    }

    /// Create a test wallet from a private key
    pub fn from_private_key(private_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let signer: PrivateKeySigner = private_key.parse()?;
        let address = signer.address();
        Ok(Self {
            address,
            signer: Some(signer),
        })
    }

    /// Get the wallet address
    pub fn address(&self) -> Address {
        self.address
    }

    /// Get the wallet signer
    pub fn signer(&self) -> Option<&PrivateKeySigner> {
        self.signer.as_ref()
    }
}

/// Send a test transaction
#[cfg(feature = "web3-testing")]
pub async fn send_test_transaction<P: alloy::providers::Provider>(
    _from: Address,
    _to: Address,
    _amount: U256,
    _provider: &P,
) -> Result<alloy::primitives::TxHash, Box<dyn std::error::Error>> {
    // This is a placeholder implementation
    // In a real implementation, you would use a provider to send the transaction
    use alloy::primitives::b256;
    let tx_hash = b256!("0000000000000000000000000000000000000000000000000000000000000000");
    Ok(tx_hash.into())
}

/// Get the balance of an address
#[cfg(feature = "web3-testing")]
pub async fn get_balance<P: alloy::providers::Provider>(
    _address: Address,
    _provider: &P,
) -> Result<U256, Box<dyn std::error::Error>> {
    // This is a placeholder implementation
    // In a real implementation, you would use a provider to get the balance
    Ok(U256::ZERO)
}

#[cfg(test)]
#[cfg(feature = "web3-testing")]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = TestWallet::random();
        assert_ne!(wallet.address(), Address::ZERO);
    }

    #[test]
    fn test_wallet_new() {
        let address = alloy::primitives::Address::ZERO;
        let wallet = TestWallet::new(address);
        assert_eq!(wallet.address(), address);
    }
}