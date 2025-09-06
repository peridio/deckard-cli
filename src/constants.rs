//! Compile-time constants for the application.

/// GitHub organization or username that owns this repository.
pub const GITHUB_OWNER: &str = "peridio";

/// GitHub repository name.
pub const GITHUB_REPO: &str = "deckard-cli";

/// Application version from Cargo.toml.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// CLI binary name.
pub const CLI_BIN: &str = "deckard";
