use mockall::{automock, predicate::*};

#[automock]
trait FeeCalc {
    fn fee(&self, amount: u64) -> u64;
}

fn charge(c: &dyn FeeCalc, amount: u64) -> u64 {
    amount + c.fee(amount)
}

#[test]
fn handler_uses_fee_mock() {
    let mut m = MockFeeCalc::new();
    m.expect_fee()
        .with(eq(100))
        .return_const(3u64);

    let out = charge(&m, 100);
    assert_eq!(out, 103);
}
