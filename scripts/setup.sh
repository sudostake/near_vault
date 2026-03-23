#!/usr/bin/env bash
set -euo pipefail

HOST_TOOLCHAIN="${HOST_TOOLCHAIN:-1.89.0}"
CONTRACT_TOOLCHAIN="${CONTRACT_TOOLCHAIN:-1.86.0}"
CARGO_NEAR_VERSION="${CARGO_NEAR_VERSION:-0.19.1}"

rustup toolchain install "${HOST_TOOLCHAIN}" --profile minimal \
  --component rustfmt \
  --component clippy \
  --component rust-analyzer

rustup toolchain install "${CONTRACT_TOOLCHAIN}" --profile minimal \
  --component clippy \
  --target wasm32-unknown-unknown

rustup target add wasm32-unknown-unknown --toolchain "${HOST_TOOLCHAIN}"

cargo install cargo-near --version "${CARGO_NEAR_VERSION}" --locked
