//! Deckard CLI library
//!
//! This library provides the core functionality for the Deckard CLI tool.

pub mod args;
pub mod commands;
pub mod constants;
pub mod error;
pub mod html;
pub mod json_schema;

// Re-export commonly used types
pub use error::{Error, Result};
