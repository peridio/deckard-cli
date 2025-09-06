use assert_cmd::Command;
use deckard::constants;

/// Create a new Command instance for the CLI binary
pub fn cli() -> Command {
    Command::cargo_bin(constants::CLI_BIN).unwrap()
}
