use proptest::prelude::*;
use rust_decimal::Decimal;
use super_test_types_rust::{fee, Side};

proptest! {
    #[test]
    fn taker_fee_monotonic(amount in 1u64..1_000_000, bps in 0i32..=10_000) {
        let a = Decimal::from(amount);
        let b = Decimal::from(amount + 1);
        let fa = fee(a, bps, Side::Taker).unwrap();
        let fb = fee(b, bps, Side::Taker).unwrap();
        prop_assert!(fb >= fa);
    }
}
