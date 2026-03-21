#!/bin/sh
set -eu

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
IMAGE_TAG="scalpel-local-e2e"

cd "$ROOT_DIR"

podman build -f Containerfile -t "$IMAGE_TAG" .
podman run --rm \
  -v "$ROOT_DIR:/workspace:Z" \
  -w /workspace \
  "$IMAGE_TAG" \
  /workspace/scripts/generate-usage-guide.sh /usr/local/bin/scalpel /workspace/docs/usage-guide.md

echo "container e2e complete; usage guide updated at docs/usage-guide.md"
