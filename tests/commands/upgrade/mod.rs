use predicates::prelude::*;

use crate::support::cli;

#[test]
fn test_upgrade_help() {
    cli()
        .arg("upgrade")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Upgrade the CLI to a newer version",
        ))
        .stdout(predicate::str::contains("--version"))
        .stdout(predicate::str::contains("--force"));
}

#[test]
fn test_upgrade_alias() {
    // Test 'u' alias works
    cli()
        .arg("u")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Upgrade"));
}

// Note: Actual upgrade operations require network access to GitHub API
// and should not be tested in integration tests. The upgrade command
// functionality should be tested with mocks in unit tests instead.

// The following tests would require network access and are commented out:
// - test_upgrade_check_only
// - test_upgrade_with_specific_version
// - test_upgrade_force_flag
// - test_upgrade_invalid_version
// - test_upgrade_with_verbose
