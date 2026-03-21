#!/usr/bin/env bash
set -euo pipefail

cmd="${1:-}"

if [[ -z "$cmd" ]]; then
  echo "usage: scripts/handle.sh <fmt|lint|test|build|check|package|build-matrix|package-matrix>"
  exit 2
fi

case "$cmd" in
  fmt)
    ./scripts/fmt.sh
    ;;
  lint)
    ./scripts/lint.sh
    ;;
  test)
    ./scripts/test.sh
    ;;
  build)
    ./scripts/build.sh
    ;;
  check)
    ./scripts/fmt.sh
    ./scripts/lint.sh
    ./scripts/test.sh
    ;;
  package)
    ./scripts/package-release.sh
    ;;
  build-matrix)
    ./scripts/build-matrix.sh
    ;;
  package-matrix)
    ./scripts/package-matrix.sh
    ;;
  *)
    echo "unknown command: $cmd"
    exit 2
    ;;
esac
