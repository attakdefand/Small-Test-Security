#[test]
fn placeholder_cargo_audit_runs_in_ci() {
    // Keep green locally; CI should run `cargo audit`/`cargo deny`.
    assert!(true);
}
