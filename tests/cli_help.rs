use assert_cmd::Command;
use predicates::prelude::*;
use predicates::str::contains;

#[test]
fn help_shows_usage_and_name() {
    let mut cmd = Command::cargo_bin("butterlog").expect("binary exists");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(contains("butterlog").and(contains("Usage").or(contains("USAGE"))));
}
