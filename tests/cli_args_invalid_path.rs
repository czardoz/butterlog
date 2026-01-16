use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

#[test]
fn invalid_path_errors() {
    let mut cmd = Command::cargo_bin("butterlog").expect("binary exists");
    cmd.arg("/no/such/file.log")
        .assert()
        .failure()
        .stderr(contains("log file not found"));
}

#[test]
fn directory_path_errors() {
    let dir = tempdir().expect("temp dir");
    let mut cmd = Command::cargo_bin("butterlog").expect("binary exists");
    cmd.arg(dir.path())
        .assert()
        .failure()
        .stderr(contains("path is not a file"));
}
