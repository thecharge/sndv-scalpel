---
name: scalpel
description: Use scalpel when you need structural-aware multi-language symbol discovery, diff preview, and safe patching with dry-run defaults.
---

# scalpel skill

Use this CLI when tasks require editing by symbol identity instead of fragile line numbers.

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
