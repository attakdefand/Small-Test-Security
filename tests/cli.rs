use assert_cmd::prelude::*; // CommandCargoExt for cargo_bin
use predicates::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn cli_uppercases() {
    let mut cmd = Command::cargo_bin("cli").unwrap();
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    // write to stdin
    child.stdin.as_mut().unwrap().write_all(b"hello world").unwrap();

    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(predicate::str::contains("HELLO WORLD").eval(&stdout));
}
