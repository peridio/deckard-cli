use predicates::prelude::*;

use crate::support::cli;

pub mod convert;
pub mod upgrade;

#[test]
fn test_cli_version() {
    cli()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_NAME")));
}

#[test]
fn test_cli_help() {
    cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A Rust CLI application template"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("convert"))
        .stdout(predicate::str::contains("upgrade"));
}

#[test]
fn test_invalid_command() {
    cli()
        .arg("invalid-command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_global_verbose_flag() {
    cli()
        .arg("-vv")
        .arg("convert")
        .write_stdin(r#"{"type":"object"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("INFO"));
}

#[test]
fn test_global_log_level() {
    cli()
        .arg("-L")
        .arg("debug")
        .arg("convert")
        .write_stdin(r#"{"type":"object"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("DEBUG"));
}

#[test]
fn test_syslog_numeric_levels() {
    // Test numeric level 6 (info)
    cli()
        .arg("-L")
        .arg("6")
        .arg("convert")
        .write_stdin(r#"{"type":"object"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("INFO"));
}

#[test]
fn test_case_insensitive_log_levels() {
    // Test uppercase
    cli()
        .arg("-L")
        .arg("INFO")
        .arg("convert")
        .write_stdin(r#"{"type":"object"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("INFO"));

    // Test mixed case
    cli()
        .arg("-L")
        .arg("Info")
        .arg("convert")
        .write_stdin(r#"{"type":"object"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("INFO"));
}

#[test]
fn test_verbose_increment() {
    // Test multiple -v flags
    cli()
        .arg("-v")
        .arg("-v")
        .arg("-v")
        .arg("convert")
        .write_stdin(r#"{"type":"object"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("INFO"));
}

#[test]
fn test_log_level_with_verbose() {
    // Test log level with verbose increment
    cli()
        .arg("-L")
        .arg("warn")
        .arg("-vv")
        .arg("convert")
        .write_stdin(r#"{"type":"object"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("INFO"));
}

#[test]
fn test_short_command_aliases() {
    // Test 'c' alias for convert
    cli()
        .arg("c")
        .write_stdin(r#"{"type":"object"}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("schema-container"));

    // Test 'u' alias for upgrade
    cli()
        .arg("u")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Upgrade"));
}
