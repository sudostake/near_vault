#!/usr/bin/env bash
set -euo pipefail

TOOLCHAIN="${CONTRACT_TOOLCHAIN:-1.86.0}"

if [ "$#" -eq 0 ]; then
  exec rustup run "${TOOLCHAIN}" cargo near build non-reproducible-wasm
fi

exec rustup run "${TOOLCHAIN}" cargo near build "$@"
