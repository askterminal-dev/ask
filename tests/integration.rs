use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_version_short() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.arg("-v")
        .assert()
        .success()
        .stdout(predicate::str::contains("ask 0.3.0"));
}

#[test]
fn test_version_long() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("ask 0.3.0"));
}

#[test]
fn test_help_short() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("multi-purpose CLI query tool"));
}

#[test]
fn test_help_long() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("multi-purpose CLI query tool"));
}

#[test]
fn test_no_args_shows_help() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage: ask <query>"));
}

#[test]
fn test_howto_compress() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.args(["how", "do", "I", "compress", "a", "folder"])
        .assert()
        .success()
        .stdout(predicate::str::contains("tar -czvf"))
        .stdout(predicate::str::contains("zip -r"));
}

#[test]
fn test_howto_find() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.args(["how", "to", "find", "a", "file"])
        .assert()
        .success()
        .stdout(predicate::str::contains("find /path"));
}

#[test]
fn test_howto_delete() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.args(["how", "to", "delete", "files"])
        .assert()
        .success()
        .stdout(predicate::str::contains("rm "));
}

#[test]
fn test_system_disk() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.args(["system", "disk"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Filesystem"));
}

#[test]
fn test_system_uptime() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.args(["system", "uptime"])
        .assert()
        .success()
        .stdout(predicate::str::contains("up"));
}

#[test]
fn test_system_unknown() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.args(["system", "unknown123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown resource"));
}

#[test]
fn test_explain_nonexistent_command() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.args(["explain", "nonexistentcmd12345"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Command not found"));
}

#[test]
fn test_config_show() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Current configuration"));
}

#[test]
fn test_config_path() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.args(["config", "path"])
        .assert()
        .success()
        .stdout(predicate::str::contains("config.json"));
}

#[test]
fn test_pipe_mode() {
    let mut cmd = Command::cargo_bin("ask").unwrap();
    cmd.write_stdin("how to copy files")
        .assert()
        .success()
        .stdout(predicate::str::contains("cp "));
}
