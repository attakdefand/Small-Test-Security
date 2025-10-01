use proptest::prelude::*;

proptest! {
  #[test]
  fn sum_is_commutative(a in 0i64..1_000_000, b in 0i64..1_000_000) {
      prop_assert_eq!(a + b, b + a);
  }
}
