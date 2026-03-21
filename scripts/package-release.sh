#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

version="$(grep '^version\s*=\s*' Cargo.toml | head -n1 | sed -E 's/.*"([^"]+)".*/\1/')"
out_dir="dist"
archive="scalpel-${version}-linux-x86_64"

rm -rf "$out_dir"
mkdir -p "$out_dir/$archive"

cargo build --release
cp target/release/scalpel "$out_dir/$archive/"
cp README.md LICENSE "$out_dir/$archive/"
cp -r docs "$out_dir/$archive/docs"

( cd "$out_dir" && tar -czf "${archive}.tar.gz" "$archive" )
( cd "$out_dir" && sha256sum "${archive}.tar.gz" > "${archive}.sha256" )

echo "release package: $out_dir/${archive}.tar.gz"
