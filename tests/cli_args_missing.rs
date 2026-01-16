use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn missing_log_file_arg_errors() {
    let mut cmd = Command::cargo_bin("butterlog").expect("binary exists");
    cmd.assert()
        .failure()
        .stderr(contains("missing log file argument"));
}
