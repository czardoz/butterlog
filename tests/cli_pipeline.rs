use assert_cmd::Command;
use predicates::str::contains;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn cli_no_ui_runs_pipeline() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "ERR one").unwrap();
    writeln!(file, "INFO two").unwrap();
    file.flush().unwrap();

    let mut cmd = Command::cargo_bin("butterlog").expect("binary exists");
    cmd.arg(file.path()).arg("--no-ui")
        .assert()
        .success()
        .stdout(contains("partitions:"));
}
