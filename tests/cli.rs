{% if include-tests %}
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("{{project-name}}").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("{{project-description}}"));
}

#[test]
fn test_example_command() {
    let mut cmd = Command::cargo_bin("{{project-name}}").unwrap();
    cmd.arg("example")
        .arg("--name")
        .arg("Test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello, Test!"));
}
{% endif %}