#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERSION="$(grep '^version\s*=\s*' Cargo.toml | head -n1 | sed -E 's/.*"([^"]+)".*/\1/')"
DIST="dist"

mkdir -p "$DIST"

package_unix() {
  local target="$1"
  local platform="$2"
  local bin="target/$target/release/scalpel"
  [[ -f "$bin" ]] || return 0

  local folder="scalpel-${VERSION}-${platform}"
  mkdir -p "$DIST/$folder"
  cp "$bin" "$DIST/$folder/"
  cp README.md LICENSE "$DIST/$folder/"
  cp -r docs "$DIST/$folder/docs"
  (cd "$DIST" && tar -czf "${folder}.tar.gz" "$folder")
  (cd "$DIST" && sha256sum "${folder}.tar.gz" > "${folder}.sha256")
}

package_windows() {
  local target="$1"
  local platform="$2"
  local bin="target/$target/release/scalpel.exe"
  [[ -f "$bin" ]] || return 0

  local folder="scalpel-${VERSION}-${platform}"
  mkdir -p "$DIST/$folder"
  cp "$bin" "$DIST/$folder/"
  cp README.md LICENSE "$DIST/$folder/"
  cp -r docs "$DIST/$folder/docs"
  (cd "$DIST" && zip -qr "${folder}.zip" "$folder")
  (cd "$DIST" && sha256sum "${folder}.zip" > "${folder}.sha256")
}

package_unix x86_64-unknown-linux-gnu linux-x86_64
package_unix aarch64-unknown-linux-gnu linux-aarch64
package_unix x86_64-apple-darwin macos-x86_64
package_unix aarch64-apple-darwin macos-aarch64
package_windows x86_64-pc-windows-gnu windows-x86_64

echo "matrix packages written to $DIST"
