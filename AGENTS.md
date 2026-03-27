# AGENTS.md

Shared operating guide for coding agents working in this repository, and for AI agents that use `scalpel` as a tool.

---

## Installing scalpel

Before using any scalpel command, verify it is present:

```bash
command -v scalpel && scalpel --version
```

If that fails, install using one of the paths below.

### Linux / macOS — automated (recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/thecharge/sndv-scalpel/main/scripts/ensure-scalpel.sh | bash
```

`ensure-scalpel.sh` detects the OS and architecture, downloads the correct prebuilt binary, falls back to `cargo install` if no binary exists, and prints the PATH setup commands. Safe to run repeatedly — exits immediately if already installed.

**Prereqs for the binary path:** `curl` and `tar`.

| OS | Install prereqs |
|----|----------------|
| Debian / Ubuntu | `sudo apt-get install -y curl tar` |
| Fedora / RHEL | `sudo dnf install -y curl tar` |
| Alpine | `apk add curl tar` |
| macOS | ships with the OS |

**Prereqs for the source build fallback:** Rust toolchain.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Linux / macOS — direct binary one-liner

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

### After installing

```bash
# add to PATH — add to ~/.bashrc or ~/.zshrc for persistence
export PATH="$HOME/.local/bin:$PATH"

# reload
source ~/.bashrc   # or: source ~/.zshrc

# enable tab completion (optional)
scalpel completion bash >> ~/.bashrc
scalpel completion zsh  >> ~/.zshrc

# verify
scalpel --version
scalpel --help
```

---

## Using scalpel as a tool

### Recommended workflow

```bash
# 1. discover symbols
scalpel --json find 'fn:*' ./src --recursive

# 2. inspect a matched symbol with context
scalpel --json view 'fn:calculate_total' src/lib.rs --context 3

# 3. preview change (always dry-run)
scalpel diff 'fn:calculate_total' src/lib.rs --rename sum=total

# 4. apply once intent is clear
scalpel patch 'fn:calculate_total' src/lib.rs --rename sum=total --apply
```

### Exit codes

| Code | Meaning |
|------|---------|
| `0` | Success — command completed, results found |
| `1` | No matches — valid query, zero results |
| `2` | Error — bad input, file not found, ambiguous pattern, write failure |

Agents must check exit code `1` separately from `2`. Exit code `1` from `find` is not an error — it means the pattern matched nothing.

### JSON output shapes

All commands accept `--json` for machine-readable output.

**`find --json`** returns an array of match objects:
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

**`view --json`** returns a single match with rendered lines:
```json
{
  "path": "app.go",
  "language": "go",
  "mode": "structural",
  "tier": 1,
  "pattern": "fn:CalculateTotal",
  "symbol": { ... },
  "lines": [
    { "number": 10, "text": "" },
    { "number": 11, "text": "// CalculateTotal sums items." },
    { "number": 12, "text": "func CalculateTotal(items []Item) int {" }
  ]
}
```

**`diff/patch --json`** returns patch status:
```json
{
  "path": "app.go",
  "applied": false,
  "dry_run": true,
  "changed": true,
  "diff": "--- a/app.go\n+++ b/app.go\n..."
}
```

**`peek --json`** returns paginated lines:
```json
{
  "path": "src/main.rs",
  "from_line": 1,
  "to_line": 50,
  "page_size": 50,
  "page": 1,
  "has_next_page": true,
  "lines": [
    { "number": 1, "text": "use sndv_scalpel::run;" }
  ]
}
```

**`view --outline --json`** returns the symbol tree:
```json
{
  "path": "src/lib.rs",
  "language": "rust",
  "mode": "structural",
  "tier": 1,
  "symbols": [
    {
      "name": "InvoiceRepository",
      "kind": "class",
      "start_line": 3,
      "end_line": 80,
      "children": [
        { "name": "computeInvoice", "kind": "method", "start_line": 10, "end_line": 25, "children": [] }
      ]
    }
  ]
}
```

### Pattern syntax

Patterns use `kind:glob` format:

| Prefix | Matches |
|--------|---------|
| `fn:` | functions |
| `method:` | methods |
| `class:` | classes |
| `type:` | types, structs, enums, traits |
| `import:` | imports / require |
| `heading:` | Markdown headings |
| `key:` | YAML / JSON / TOML / text keys |

Glob wildcards: `fn:calc*` matches any function starting with `calc`. `fn:*` matches all functions.

### Disambiguation

When multiple symbols match, `view`/`diff`/`patch` fail with exit code 2 and list the candidates. Use `--index N` to select:

```bash
scalpel find 'import:*' src/app.ts         # lists all imports with their index
scalpel patch 'import:*' src/app.ts --index 2 --replace 'from "lib-a"=>from "lib-b"' --apply
```

### Operations

Exactly one operation flag is required for `diff`/`patch`:

| Flag | Format | Effect |
|------|--------|--------|
| `--rename old=new` | `identifier=identifier` | Word-boundary replace scoped to symbol byte range |
| `--replace old=>new` | `literal=>literal` | Literal string replace (first occurrence) in scope |
| `--body 'text'` | inline string | Replace entire symbol block |
| `--body-file path` | file path | Replace entire symbol block with file contents |

### Safety contract

- `diff` is always dry-run. Safe to call.
- `patch` without `--apply` is also dry-run.
- `patch --apply` uses snapshot + atomic rename + rollback — never leaves partial writes.

### Supported languages

JavaScript, TypeScript, Go, Rust, Lua, Markdown, YAML, JSON, JSONL, TOML, Text.

---

## Maintaining this repository

### Mission

Build and maintain a safe, production-ready CLI for structural find/diff/patch workflows.

### Core rules

- Keep safety defaults: `diff` is dry-run and `patch` requires `--apply`.
- Keep parser behavior configuration-driven from `scalpel.yaml`.
- Prefer early returns and shallow control flow.
- Keep source files under 300 lines.
- Keep tests for happy, side, and critical paths.
- Update docs when behavior changes.
- For new CLI commands, add integration tests and update completion/docs examples.
- Keep `peek` behavior stable: paginated reading plus explicit position ranges.
- Keep command behavior and docs aligned for full parity: `view --outline/--lines`, JSON outputs, and transaction safety.

### Required checks before finishing

```bash
./scripts/handle.sh check
```

### Required docs to update when adding behavior

- `README.md`
- `docs/quickstart.md`
- `docs/integration-e2e.md`
- `docs/language-support.md` when language behavior changes
- `docs/extension-guide.md` when extension surface changes

### Testing expectations

- Add focused unit tests for logic changes.
- Add integration tests for CLI behavior changes.
- Add critical safety tests for transaction and rollback paths.
- Keep heavy-path coverage for large inputs.

### Useful commands

```bash
./scripts/build.sh
./scripts/test.sh
./scripts/lint.sh
./scripts/fmt.sh
./scripts/package-release.sh
```
