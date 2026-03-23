#!/usr/bin/env bash
set -euo pipefail

TOOLCHAIN="${CONTRACT_TOOLCHAIN:-1.86.0}"

exec rustup run "${TOOLCHAIN}" cargo clippy --target wasm32-unknown-unknown -- -D warnings
