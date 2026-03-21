#!/bin/sh
set -eu

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
BIN_PATH="${1:-$ROOT_DIR/target/release/scalpel}"
OUT_PATH="${2:-$ROOT_DIR/docs/usage-guide.md}"

if [ ! -x "$BIN_PATH" ]; then
  echo "binary not found or not executable: $BIN_PATH"
  exit 2
fi

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

cp -R "$ROOT_DIR/tests/fixtures" "$WORK_DIR/fixtures"
cp "$WORK_DIR/fixtures/sample.go" "$WORK_DIR/sample.go"
cp "$WORK_DIR/fixtures/sample.rs" "$WORK_DIR/sample.rs"
cp "$WORK_DIR/fixtures/sample-complex.ts" "$WORK_DIR/sample-complex.ts"
cp "$WORK_DIR/fixtures/sample.json" "$WORK_DIR/sample.json"
cp "$WORK_DIR/fixtures/sample.md" "$WORK_DIR/sample.md"
cp "$WORK_DIR/fixtures/sample-import-groups.go" "$WORK_DIR/sample-import-groups.go"
cp "$WORK_DIR/fixtures/sample.txt" "$WORK_DIR/sample.txt"

cat > "$WORK_DIR/new-total.go" << 'EOF'
func CalculateTotal(items []int) int {
    total := 100
    return total
}
EOF

cat > "$WORK_DIR/new-imports.go.frag" << 'EOF'
import (
    "strings"
    "fmt"
)
EOF

case_index=0

sanitize_output() {
  sed \
    -e "s|$ROOT_DIR/||g" \
    -e "s|$WORK_DIR|/tmp/scalpel-work|g"
}

run_case() {
  title="$1"
  file_links="$2"
  display_cmd="$3"
  exec_cmd="$4"
  expected="$5"

  case_index=$((case_index + 1))

  output=""
  status=0
  set +e
  output="$(eval "$exec_cmd" 2>&1)"
  status=$?
  set -e

  if [ "$status" -ne 0 ]; then
    echo "case failed: $title"
    echo "$output"
    exit $status
  fi

  if [ -n "$expected" ]; then
    if ! printf "%s\n" "$output" | grep -Fq "$expected"; then
      echo "case assertion failed: $title"
      echo "expected substring: $expected"
      echo "$output"
      exit 3
    fi
  fi

  {
    echo "## $case_index. $title"
    echo
    echo "Example files:"
    printf "%s\n" "$file_links" | while IFS= read -r file_link; do
      [ -z "$file_link" ] && continue
      echo "- [$file_link](../$file_link)"
    done
    echo
    echo "Command:"
    echo
    echo '```bash'
    echo "$display_cmd"
    echo '```'
    echo
    echo "Actual output:"
    echo
    echo '```text'
    printf "%s\n" "$output" | sanitize_output
    echo '```'
    echo
  } >> "$OUT_PATH"
}

cat > "$OUT_PATH" << 'EOF'
# Usage Guide

This guide is auto-generated from a shell test suite run against real CLI commands.
EOF

run_case \
  "Find across fixture languages" \
  "tests/fixtures/sample.js
tests/fixtures/sample.ts
tests/fixtures/sample.go
tests/fixtures/sample.rs" \
  "scalpel find 'fn:*' tests/fixtures --recursive" \
  "\"$BIN_PATH\" find 'fn:*' \"$WORK_DIR/fixtures\" --recursive" \
  "sample.go"

run_case \
  "View matched function with context" \
  "tests/fixtures/sample.go" \
  "scalpel view 'fn:CalculateTotal' tests/fixtures/sample.go --context 2" \
  "\"$BIN_PATH\" view 'fn:CalculateTotal' \"$WORK_DIR/sample.go\" --context 2" \
  "CalculateTotal"

run_case \
  "View structural outline" \
  "tests/fixtures/sample.go" \
  "scalpel view tests/fixtures/sample.go --outline" \
  "\"$BIN_PATH\" view \"$WORK_DIR/sample.go\" --outline" \
  "type Config"

run_case \
  "View explicit line window" \
  "tests/fixtures/sample.rs" \
  "scalpel view tests/fixtures/sample.rs --lines 1:6" \
  "\"$BIN_PATH\" view \"$WORK_DIR/sample.rs\" --lines 1:6" \
  "1 |"

run_case \
  "Preview rename diff (dry-run)" \
  "tests/fixtures/sample.go" \
  "scalpel diff 'fn:CalculateTotal' tests/fixtures/sample.go --rename sum=total" \
  "\"$BIN_PATH\" diff 'fn:CalculateTotal' \"$WORK_DIR/sample.go\" --rename sum=total" \
  "dry-run only"

run_case \
  "Apply transactional rename patch" \
  "tests/fixtures/sample.rs" \
  "scalpel patch 'fn:calculate_total' tests/fixtures/sample.rs --rename sum=total --apply" \
  "\"$BIN_PATH\" patch 'fn:calculate_total' \"$WORK_DIR/sample.rs\" --rename sum=total --apply" \
  "applied:"

run_case \
  "Swap direct line range" \
  "tests/fixtures/sample.txt" \
  "scalpel patch '*' tests/fixtures/sample.txt --from-line 2 --to-line 2 --body \"\$(printf 'status: running\\n')\" --apply" \
  "\"$BIN_PATH\" patch '*' \"$WORK_DIR/sample.txt\" --from-line 2 --to-line 2 --body \"\$(printf 'status: running\\n')\" --apply" \
  "applied:"

run_case \
  "Swap Go function body from file" \
  "tests/fixtures/sample.go" \
  "scalpel patch 'fn:CalculateTotal' tests/fixtures/sample.go --body-file /tmp/new-total.go --apply" \
  "\"$BIN_PATH\" patch 'fn:CalculateTotal' \"$WORK_DIR/sample.go\" --body-file \"$WORK_DIR/new-total.go\" --apply" \
  "applied:"

run_case \
  "Swap grouped Go imports" \
  "tests/fixtures/sample-import-groups.go" \
  "scalpel patch 'import:import' tests/fixtures/sample-import-groups.go --body-file /tmp/new-imports.go.frag --apply" \
  "\"$BIN_PATH\" patch 'import:import' \"$WORK_DIR/sample-import-groups.go\" --body-file \"$WORK_DIR/new-imports.go.frag\" --apply" \
  "applied:"

run_case \
  "Scoped JSON key replacement" \
  "tests/fixtures/sample.json" \
  "scalpel patch 'key:service.mode' tests/fixtures/sample.json --replace 'safe=>strict' --apply" \
  "\"$BIN_PATH\" patch 'key:service.mode' \"$WORK_DIR/sample.json\" --replace 'safe=>strict' --apply" \
  "applied:"

run_case \
  "Structured JSON output from view" \
  "tests/fixtures/sample.rs" \
  "scalpel --json view 'fn:calculate_total' tests/fixtures/sample.rs" \
  "\"$BIN_PATH\" --json view 'fn:calculate_total' \"$WORK_DIR/sample.rs\"" \
  "\"symbol\""

run_case \
  "Structured JSON output from diff" \
  "tests/fixtures/sample.go" \
  "scalpel --json diff 'fn:CalculateTotal' tests/fixtures/sample.go --rename sum=total" \
  "\"$BIN_PATH\" --json diff 'fn:CalculateTotal' \"$WORK_DIR/sample.go\" --rename sum=total" \
  "\"dry_run\": true"

run_case \
  "Peek paginated content" \
  "tests/fixtures/sample.go" \
  "scalpel peek tests/fixtures/sample.go --page-size 5 --page 1" \
  "\"$BIN_PATH\" peek \"$WORK_DIR/sample.go\" --page-size 5 --page 1" \
  "next page: --page 2"

run_case \
  "Generate bash completion script" \
  "README.md" \
  "scalpel completion bash" \
  "\"$BIN_PATH\" completion bash" \
  "_scalpel"

echo "generated usage guide: $OUT_PATH"
