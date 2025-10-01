//! Minimal "fee engine" to demonstrate testing.
//!
//! ## Doc test example
//!
//! ```
//! use rust_decimal::prelude::*;
//! use rust_decimal_macros::dec;
//! use super_test_types_rust::{fee, Side};
//! let f = fee(dec!(100), 25, Side::Taker).unwrap();
//! assert_eq!(f, dec!(0.25));
//! ```

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum Side { Maker, Taker }

#[derive(Debug, Error)]
pub enum FeeError {
    #[error("amount must be positive")]
    NonPositive,
    #[error("rate basis points out of range")]
    BadBps,
}

pub fn fee(amount: Decimal, bps: i32, side: Side) -> Result<Decimal, FeeError> {
    if amount <= dec!(0) { return Err(FeeError::NonPositive) }
    if !(0..=10_000).contains(&bps) { return Err(FeeError::BadBps) }
    let base = amount * Decimal::from(bps) / dec!(10000);
    Ok(match side {
        Side::Maker => (base * dec!(0.5)).round_dp(8),
        Side::Taker => base.round_dp(8),
    })
}

// --- Unit tests live next to the code ---
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn maker_half_fee() {
        let f = fee(dec!(100.00), 10, Side::Maker).unwrap();
        assert_eq!(f, dec!(0.05));
    }

    #[test]
    fn taker_full_fee() {
        let f = fee(dec!(100.00), 10, Side::Taker).unwrap();
        assert_eq!(f, dec!(0.10));
    }

    #[test]
    fn rejects_non_positive() {
        assert!(matches!(fee(dec!(0), 10, Side::Taker), Err(FeeError::NonPositive)));
    }
}

// expose http module to tests/integration
pub mod http;
