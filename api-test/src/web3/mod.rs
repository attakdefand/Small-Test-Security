//! Web3 testing utilities and helpers.
//!
//! This module provides utilities for testing Ethereum-based applications,
//! including blockchain interactions, smart contract testing, and wallet functionality.

pub mod provider;
pub mod contract;
pub mod wallet;
pub mod security;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}