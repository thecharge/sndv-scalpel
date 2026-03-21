#!/usr/bin/env bash
set -euo pipefail

REPO="${SCALPEL_REPO:-thecharge/sndv-scalpel}"
VERSION="${1:-latest}"
BIN_NAME="scalpel"
INSTALL_DIR="${SCALPEL_INSTALL_DIR:-$HOME/.local/bin}"

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Linux) platform="linux" ;;
  Darwin) platform="macos" ;;
  *) echo "unsupported OS: $os"; exit 1 ;;
esac

case "$arch" in
  x86_64|amd64) arch_name="x86_64" ;;
  arm64|aarch64) arch_name="aarch64" ;;
  *) echo "unsupported arch: $arch"; exit 1 ;;
esac

if [[ "$VERSION" == "latest" ]]; then
  VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"v\{0,1\}\([^"]*\)".*/\1/p' | head -n1)"
fi

if [[ -z "$VERSION" ]]; then
  echo "failed to resolve version"
  exit 1
fi

asset="scalpel-${VERSION}-${platform}-${arch_name}.tar.gz"
url="https://github.com/${REPO}/releases/download/v${VERSION}/${asset}"

mkdir -p "$INSTALL_DIR"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

curl -fL "$url" -o "$tmp_dir/$asset"
tar -xzf "$tmp_dir/$asset" -C "$tmp_dir"

src_bin="$(find "$tmp_dir" -type f -name "$BIN_NAME" | head -n1)"
if [[ -z "$src_bin" ]]; then
  echo "binary not found in archive"
  exit 1
fi

install -m 0755 "$src_bin" "$INSTALL_DIR/$BIN_NAME"

echo "installed $BIN_NAME to $INSTALL_DIR/$BIN_NAME"
echo "add this to your shell profile if needed:"
echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
