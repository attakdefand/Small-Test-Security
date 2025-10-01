#[test]
fn caps_fee() {
    fn cap_fee(fee: i64, cap: i64) -> i64 { fee.min(cap) }
    assert_eq!(cap_fee(120, 100), 100);
}
