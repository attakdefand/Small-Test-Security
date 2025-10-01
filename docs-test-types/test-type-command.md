# normal/unit + integration + async tests
cargo test

# doc tests
cargo test --doc

# property tests
cargo test --test fee_props

# snapshot tests (first accept, then verify)
cargo install cargo-insta
cargo insta test --accept
cargo insta test

# CLI test
cargo test --test cli

# DB/container (requires Docker)
cargo test --test postgres_it -- --test-threads=1

# benchmarks
cargo bench
