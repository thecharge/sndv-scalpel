#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
IMAGE_TAG="scalpel-local-e2e"

cd "$ROOT_DIR"

cargo build --release
podman build -f Containerfile -t "$IMAGE_TAG" .
podman run --rm \
  -v "$ROOT_DIR:/workspace:Z" \
  -w /workspace \
  "$IMAGE_TAG" \
  /workspace/scripts/generate-usage-guide.sh /workspace/target/release/scalpel /workspace/docs/usage-guide.md

echo "container e2e complete; usage guide updated at docs/usage-guide.md"
