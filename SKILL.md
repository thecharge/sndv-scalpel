---
name: scalpel
description: Use scalpel when you need structural-aware multi-language symbol discovery, diff preview, and safe patching with dry-run defaults.
---

# scalpel skill

Use this CLI when tasks require editing by symbol identity instead of fragile line numbers.

Current command parity includes `view --outline`, `view --lines`, and JSON outputs for `find`, `peek`, `view`, `diff`, and `patch`.

## Best-fit scenarios

- Rename identifiers within a matched function or block.
- Locate functions, classes, types, headings, and YAML keys.
- Peek files by page or line position before editing.
- Produce machine-readable JSON symbol output.
- Preview modifications before any write.

## Safety behavior

- `diff` is always dry-run.
- `patch` is dry-run unless `--apply` is passed.
- Writes use snapshot + atomic rename + rollback.

## Required language coverage

- JavaScript
- TypeScript
- Go
- Rust
- Lua
- Markdown
- YAML
- JSON
- JSONL
- TOML

## Canonical commands

```bash
scalpel find 'fn:*' ./src --recursive
scalpel view 'fn:main' src/main.rs --context 3
scalpel diff 'fn:CalculateTotal' app.go --rename sum=total
scalpel peek src/main.rs --from-line 1 --page-size 40 --page 1
scalpel peek src/main.rs --from-pos 80 --to-pos 140 --all
scalpel patch 'fn:CalculateTotal' app.go --rename sum=total --apply
scalpel patch 'method:computeInvoice' tests/fixtures/sample-complex.ts --body-file /tmp/compute-invoice-method.tsfrag --apply
scalpel patch 'class:InvoiceRepository' tests/fixtures/sample-complex.ts --body-file /tmp/replacement-class.tsfrag --apply
scalpel patch 'method:chooseTier' tests/fixtures/sample-complex.ts --replace 'if (amount > 1000) { return "enterprise"; }=>if (amount > 1000) { return "platinum"; }' --apply
scalpel completion bash > /tmp/scalpel.bash
```
