use assert_cmd::Command;
use predicates::str::contains;
use std::io::Write;
use tempfile::NamedTempFile;

fn tmp(text: &str) -> NamedTempFile {
    let mut f = NamedTempFile::new().unwrap();
    f.write_all(text.as_bytes()).unwrap();
    f
}

#[test]
fn counts_stdin() {
    Command::cargo_bin("rwc")
        .unwrap()
        .write_stdin("one two three\n")
        .assert()
        .success()
        .stdout(contains("1").and(contains("3")));
}

#[test]
fn counts_a_file() {
    let f = tmp("alpha beta\n");
    Command::cargo_bin("rwc")
        .unwrap()
        .arg(f.path())
        .assert()
        .success()
        .stdout(contains("1").and(contains("2")));
}

#[test]
fn totals_across_several_files() {
    let a = tmp("a b\n");
    let b = tmp("c d e\n");
    Command::cargo_bin("rwc")
        .unwrap()
        .args([a.path(), b.path()])
        .assert()
        .success()
        .stdout(contains("total"));
}

#[test]
fn missing_file_sets_exit_code() {
    Command::cargo_bin("rwc")
        .unwrap()
        .arg("/no/such/file.xyz")
        .assert()
        .failure();
}
