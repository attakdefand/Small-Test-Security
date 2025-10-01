#![no_main]
use libfuzzer_sys::fuzz_target;
use rust_decimal::Decimal;
use super_test_types_rust::{fee, Side};

fuzz_target!(|data: &[u8]| {
    if data.len() < 3 { return; }
    let amount = Decimal::from(data[0] as u64);
    let bps = (data[1] as i32) % 10001; // 0..=10000
    let side = if data[2] % 2 == 0 { Side::Maker } else { Side::Taker };
    let _ = fee(amount, bps, side);
});
