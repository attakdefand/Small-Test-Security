use rust_decimal_macros::dec; // <-- bring in the dec! macro
use super_test_types_rust::{fee, Side};

#[test]
fn public_api_works() {
    let f = fee(dec!(250), 15, Side::Taker).unwrap();
    assert_eq!(f, dec!(0.375));
}
