#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

export CARGO_TERM_COLOR=always
export RUST_BACKTRACE=1

echo '==> format'
cargo fmt --all -- --check

echo '==> compile all targets/features'
cargo check --locked --all-targets --all-features

echo '==> clippy'
cargo clippy --locked --all-targets --all-features -- -D warnings

echo '==> library tests'
cargo test --locked --all-features --lib

echo '==> integration tests'
cargo test --locked --all-features --tests

echo '==> real concurrent claim acceptance'
cargo test --locked --all-features --test completion_concurrency -- --nocapture

echo '==> documentation'
RUSTDOCFLAGS='-D warnings' cargo doc --locked --all-features --no-deps

echo '==> shell export contract'
bash scripts/verify-shell-export.sh

echo '==> repository claim hygiene'
bash scripts/verify-claims.sh

echo 'COMPLETION_GATE=ALIVE'
