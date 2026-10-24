#!/usr/bin/env bash
# Runs the CI checks except e2e: formatting, clippy (all and default features),
# tests, generated-code freshness and docs.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --locked
cargo clippy -p koof --all-targets --no-default-features --locked
cargo test --workspace --all-features --locked
scripts/check-generated.sh
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked -p koof -p koof-gen
