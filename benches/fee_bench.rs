use criterion::{criterion_group, criterion_main, Criterion};
use rust_decimal::prelude::*;
use super_test_types_rust::{fee, Side};
use rust_decimal_macros::dec;

fn bench_fee(c: &mut Criterion) {
    c.bench_function("fee 10k iters", |b| {
        b.iter(|| {
            let mut acc = dec!(0);
            for _ in 0..10_000 {
                acc += fee(dec!(1234.5678), 12, Side::Taker).unwrap();
            }
            acc
        })
    });
}
criterion_group!(benches, bench_fee);
criterion_main!(benches);
