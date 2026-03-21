#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

TARGETS=(
  x86_64-unknown-linux-gnu
  aarch64-unknown-linux-gnu
  x86_64-apple-darwin
  aarch64-apple-darwin
  x86_64-pc-windows-gnu
)

for target in "${TARGETS[@]}"; do
  echo "==> target: $target"
  rustup target add "$target" || true
  cargo build --release --target "$target" || {
    echo "warning: build failed for $target (toolchain/linker may be missing)"
  }
done
