#!/usr/bin/env bash

set -e

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets

# Check for cargo-audit (skip in CI as it's handled by GitHub Action)
if [ -z "$CI" ]; then
    if ! command -v cargo-audit &> /dev/null; then
        echo -e "\033[1;31m[ERROR]\033[0m cargo-audit is not installed."
        echo "cargo install cargo-audit"
        exit 1
    fi
    cargo audit
fi

# Check for cargo-llvm-cov (skip check in CI as it's installed by workflow)
if [ -z "$CI" ]; then
    if ! command -v cargo-llvm-cov &> /dev/null; then
        echo -e "\033[1;31m[ERROR]\033[0m cargo-llvm-cov is not installed."
        echo "cargo install cargo-llvm-cov"
        exit 1
    fi
fi

# Run tests and measure code coverage during
if [ -z "$CI" ]; then
    cargo llvm-cov --all-features --workspace --html
else
    cargo llvm-cov --all-features --workspace --cobertura --output-path cobertura.xml
fi
