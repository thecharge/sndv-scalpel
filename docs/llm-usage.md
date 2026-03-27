# LLM Usage Guide

## Purpose

This project is designed to be LLM-friendly by exposing deterministic, parseable command outputs and safety-first write semantics.

## Prerequisites and installation

Before invoking any scalpel command, verify the binary is available:

```bash
command -v scalpel && scalpel --version
```

If the check fails, use one of the install paths below based on OS.

### Linux / macOS — automated one-liner (recommended for agents)

```bash
curl -fsSL https://raw.githubusercontent.com/thecharge/sndv-scalpel/main/scripts/ensure-scalpel.sh | bash
```

`ensure-scalpel.sh` handles everything: OS detection, binary download, source build fallback, PATH output. Safe to run in any pipeline — exits 0 immediately if already installed.

Required tools for binary install path:

| OS | Command |
|----|---------|
| Debian / Ubuntu | `sudo apt-get install -y curl tar` |
| Fedora / RHEL | `sudo dnf install -y curl tar` |
| Alpine | `apk add curl tar` |
| macOS | ships with OS |

If no prebuilt binary exists for the platform, the script falls back to building from source. That requires the Rust toolchain:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Linux / macOS — binary only

```bash
curl -fsSL https://raw.githubusercontent.com/thecharge/sndv-scalpel/main/scripts/install-from-github.sh | bash
```

### Windows — PowerShell

```powershell
iwr https://raw.githubusercontent.com/thecharge/sndv-scalpel/main/scripts/install-from-github.ps1 -OutFile install-scalpel.ps1
powershell -ExecutionPolicy Bypass -File .\install-scalpel.ps1
```

### From source (any OS with Rust)

```bash
cargo install --git https://github.com/thecharge/sndv-scalpel --bin scalpel
```

### Add to PATH and enable completion

```bash
export PATH="$HOME/.local/bin:$PATH"    # add to ~/.bashrc or ~/.zshrc for persistence
source ~/.bashrc                         # reload

scalpel completion bash >> ~/.bashrc    # optional tab completion
scalpel --version                       # verify
```

---

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | No matches — valid query, zero results |
| `2` | Error — bad input, file missing, ambiguous match, write failure |

Exit code `1` is not an error. It means the pattern matched nothing. Agents must distinguish `1` (empty result) from `2` (hard failure) in control flow.

## Safety-first contract

- Prefer `find` to locate symbols before proposing edits.
- Use `diff` first and show patch output.
- Only invoke `patch --apply` when user intent is explicit.
- Use `--json` for structured downstream reasoning.

## Tool-calling pattern (generic)

Use this sequence in any agent/tool-calling runtime:

1. discover: `scalpel --json find 'fn:calculate*' src --recursive`
2. inspect: `scalpel --json view 'fn:calculate_total' src/lib.rs --context 3`
3. preview: `scalpel diff 'fn:calculate_total' src/lib.rs --rename sum=total`
4. apply: `scalpel patch 'fn:calculate_total' src/lib.rs --rename sum=total --apply`

## Framework examples

### 1. OpenAI/Responses style tool wrapper

```bash
scalpel --json find 'method:*' src --recursive
```

Treat stdout JSON as tool result and feed it back to the model for target selection.

### 2. LangChain shell tool

Use a shell tool step with deterministic commands:

```bash
scalpel --json find 'import:*' src --recursive
scalpel diff 'import:*' src/main.ts --index 2 --replace 'from "a"=>from "b"'
```

### 3. MCP server workflows

When exposing `scalpel` through MCP tools, map these operations directly:

- `findSymbols`: `scalpel --json find ...`
- `previewPatch`: `scalpel diff ...`
- `applyPatch`: `scalpel patch ... --apply`
- `peekFile`: `scalpel --json peek ...`
- `outlineFile`: `scalpel --json view <path> --outline`
- `readLineWindow`: `scalpel --json view <path> --lines start:end [--all]`

## Workflow recipes

### Refactor workflow

```bash
scalpel --json find 'fn:build*' src --recursive
scalpel diff 'fn:build_report' src/report.rs --rename data=input
scalpel patch 'fn:build_report' src/report.rs --rename data=input --apply
```

### Import swap workflow

```bash
scalpel find 'import:*' src/app.ts
scalpel patch 'import:*' src/app.ts --index 1 --replace 'from "lib-a"=>from "lib-b"' --apply
```

### Go grouped import block workflow

```bash
cat > /tmp/imports.go.frag << 'EOF'
import (
	"strings"
	"fmt"
)
EOF

scalpel patch 'import:import' src/main.go --body-file /tmp/imports.go.frag --apply
```

### Context retrieval workflow (paged)

```bash
scalpel peek src/main.go --from-line 1 --page-size 40 --page 1
scalpel --json peek src/main.go --from-pos 120 --to-pos 200 --all
```

## CI and automation examples (CI-agnostic)

### 1. Dry-run safety gate for pull requests

```bash
set -euo pipefail
scalpel diff 'fn:CalculateTotal' tests/fixtures/sample.go --rename sum=total > /tmp/scalpel.patch
test -s /tmp/scalpel.patch
```

### 2. Structured report artifact for pipelines

```bash
set -euo pipefail
mkdir -p artifacts
scalpel --json find 'fn:*' src --recursive > artifacts/scalpel-find.json
scalpel --json peek src/main.rs --from-line 1 --page-size 80 --page 1 > artifacts/scalpel-peek.json
```

### 3. Deterministic patch pipeline

```bash
set -euo pipefail
scalpel diff 'key:service.mode' config/app.json --replace 'safe=>strict'
scalpel patch 'key:service.mode' config/app.json --replace 'safe=>strict' --apply
./scripts/handle.sh check
```

## Completion for agent shells

```bash
scalpel completion bash > /tmp/scalpel.bash
source /tmp/scalpel.bash
```
