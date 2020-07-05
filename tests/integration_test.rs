extern crate harvest;
use assert_cmd::Command;

#[test]
fn test_run_with_pattern() {
    let mut cmd = Command::cargo_bin("harvest").unwrap();
    let assert = cmd.arg("string").assert();

    assert.success();
}

#[test]
fn test_run_without_pattern() {
    let mut cmd = Command::cargo_bin("harvest").unwrap();
    cmd.assert().failure();
}
