---
name: scalpel
description: Use scalpel when you need structural-aware multi-language symbol discovery, diff preview, and safe patching with dry-run defaults.
---

# scalpel skill

Use this CLI when tasks require editing by symbol identity instead of fragile line numbers.

## Step 0 — ensure scalpel is installed

Before invoking any scalpel command, verify the tool is present. Run this check first:

```bash
command -v scalpel && scalpel --version
```

If that fails (exit code non-zero or command not found), install scalpel for the current OS.

### Linux / macOS — one-liner binary install

```bash
curl -fsSL https://raw.githubusercontent.com/thecharge/sndv-scalpel/main/scripts/install-from-github.sh | bash
```

Prereqs for the one-liner: `curl` and `tar`.

| OS | Install curl + tar |
|----|-------------------|
| Debian / Ubuntu | `sudo apt-get install -y curl tar` |
| Fedora / RHEL | `sudo dnf install -y curl tar` |
| Alpine | `apk add curl tar` |
| macOS | ships with the OS; or `brew install curl` |

### Windows — PowerShell

```powershell
iwr https://raw.githubusercontent.com/thecharge/sndv-scalpel/main/scripts/install-from-github.ps1 -OutFile install-scalpel.ps1
powershell -ExecutionPolicy Bypass -File .\install-scalpel.ps1
```

### Fallback — build from source (any OS with Rust)

```bash
# 1. install Rust if not present
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# 2. build and install
cargo install --git https://github.com/thecharge/sndv-scalpel --bin scalpel
```

### Or use the automated ensure script (check + install + PATH in one step)

```bash
curl -fsSL https://raw.githubusercontent.com/thecharge/sndv-scalpel/main/scripts/ensure-scalpel.sh | bash
```

The `ensure-scalpel.sh` script:
- exits 0 immediately if scalpel is already on PATH
- detects OS and architecture
- attempts a binary release download first
- falls back to `cargo install` if no binary exists for the platform
- prints exact PATH export instructions after installing

### After installing — add to PATH and enable completion

```bash
# add to PATH (add this line to ~/.bashrc or ~/.zshrc for persistence)
export PATH="$HOME/.local/bin:$PATH"

# reload shell profile
source ~/.bashrc   # or: source ~/.zshrc

# optional: enable tab completion
scalpel completion bash >> ~/.bashrc
scalpel completion zsh  >> ~/.zshrc
```

### Verify

```bash
scalpel --version
scalpel --help
```

---

## Best-fit scenarios

- Rename identifiers within a matched function or block.
- Locate functions, classes, types, headings, and YAML keys.
- Peek files by page or line position before editing.
- Produce machine-readable JSON symbol output.
- Preview modifications before any write.

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | No matches found (valid query, zero results) |
| `2` | Error (bad input, file not found, ambiguous match, write failure) |

Always distinguish exit code `1` from `2` — `1` is not an error.

## Safety behavior

- `diff` is always dry-run.
- `patch` is dry-run unless `--apply` is passed.
- Writes use snapshot + atomic rename + rollback.

## Required language coverage

JavaScript, TypeScript, Go, Rust, Lua, Markdown, YAML, JSON, JSONL, TOML.

## Canonical commands

```bash
# discover
scalpel --json find 'fn:*' ./src --recursive

# inspect
scalpel --json view 'fn:main' src/main.rs --context 3

# file outline
scalpel --json view src/main.rs --outline

# line window
scalpel --json view src/main.rs --lines 10:40

# paginated read
scalpel --json peek src/main.rs --from-line 1 --page-size 40 --page 1

# preview rename (dry-run)
scalpel diff 'fn:CalculateTotal' app.go --rename sum=total

# apply rename
scalpel patch 'fn:CalculateTotal' app.go --rename sum=total --apply

# swap function body from file
scalpel patch 'fn:CalculateTotal' app.go --body-file ./new-total.go --apply

# literal scoped replace
scalpel patch 'method:chooseTier' sample-complex.ts --replace 'if (amount > 1000) { return "enterprise"; }=>if (amount > 1000) { return "platinum"; }' --apply

# disambiguate with --index
scalpel patch 'import:*' app.ts --index 2 --replace 'from "lib-a"=>from "lib-b"' --apply

# shell completion
scalpel completion bash > /tmp/scalpel.bash
```

## JSON output shapes

**`find --json`** → array of match objects, sorted by file + line:
```json
[
  {
    "pattern": "fn:calculate*",
    "language": "go",
    "mode": "structural",
    "tier": 1,
    "confidence": "high",
    "symbol": {
      "file": "app.go",
      "kind": "function",
      "name": "CalculateTotal",
      "start_line": 12,
      "end_line": 24,
      "start_byte": 280,
      "end_byte": 490,
      "signature": "func CalculateTotal(items []Item) int"
    }
  }
]
```

**`diff/patch --json`** → patch status:
```json
{
  "path": "app.go",
  "applied": false,
  "dry_run": true,
  "changed": true,
  "diff": "--- a/app.go\n+++ b/app.go\n..."
}
```

**`peek --json`** → paginated lines with `has_next_page`:
```json
{
  "path": "src/main.rs",
  "from_line": 1,
  "to_line": 50,
  "page_size": 50,
  "page": 1,
  "has_next_page": true,
  "lines": [{ "number": 1, "text": "use sndv_scalpel::run;" }]
}
```
