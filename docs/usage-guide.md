# Usage Guide

This guide shows practical command examples for common edits.

## 1. Find and inspect

```bash
scalpel find 'fn:*' tests/fixtures --recursive
scalpel view 'fn:CalculateTotal' tests/fixtures/sample.go --context 2
```

## 2. Swap a Go function body from file

Create replacement body file:

```bash
cat > /tmp/new-total.go << 'EOF'
func CalculateTotal(items []int) int {
    total := 100
    return total
}
EOF
```

Preview and apply:

```bash
scalpel diff 'fn:CalculateTotal' tests/fixtures/sample.go --body-file /tmp/new-total.go
scalpel patch 'fn:CalculateTotal' tests/fixtures/sample.go --body-file /tmp/new-total.go --apply
```

Inline body replacement (no temp file):

```bash
scalpel patch 'method:chooseTier' tests/fixtures/sample-complex.ts --body 'public chooseTier(amount: number): "basic" | "enterprise" { return "basic"; }' --apply
```

## 3. Replace ternary expression in one line

```bash
scalpel patch 'fn:main' app.js --replace 'flag ? true : false=>flag ? 1 : 0' --apply
```

## 4. Replace an if block in a function

```bash
scalpel patch 'fn:run' app.ts --replace 'if (flag) { return 1; }=>if (flag) { return 2; }' --apply
```

## 5. Replace function arguments in a signature

```bash
scalpel patch 'fn:CalculateTotal' app.go --replace 'items []int=>numbers []int' --apply
```

## 6. Safety checks

- `diff` never writes files.
- `patch` writes only with `--apply`.
- Every patch uses snapshot + rollback safety.

## 7. Large-data surgical JSONL command

```bash
cp tests/fixtures/big/events-10k.jsonl /tmp/events-10k.jsonl
scalpel patch 'key:line9001.state' /tmp/events-10k.jsonl --replace 'queued=>running' --apply
sed -n '9000,9002p' /tmp/events-10k.jsonl
```

## 8. Complex TypeScript fixture workflow

Fixture file:

```bash
tests/fixtures/sample-complex.ts
```

### 8.1 Swap a large class method body

```bash
cat > /tmp/compute-invoice-method.tsfrag << 'EOF'
    public computeInvoice(lines: InvoiceLine[], discountRate: number): InvoiceSummary {
        const subtotal = lines.reduce((acc, line) => acc + line.qty * line.unitPrice, 0);
        const discount = subtotal * discountRate;
        return { subtotal, discount, total: subtotal - discount, tier: "basic" };
    }
EOF

scalpel patch 'method:computeInvoice' tests/fixtures/sample-complex.ts --body-file /tmp/compute-invoice-method.tsfrag --apply
```

### 8.2 Replace a whole class

```bash
cat > /tmp/replacement-class.tsfrag << 'EOF'
export class InvoiceRepository {
    public async loadInvoice(id: string): Promise<Invoice> {
        return { id, currency: "USD", lines: [] };
    }

    public sanitizeLines(lines: InvoiceLine[]): InvoiceLine[] {
        return lines;
    }
}
EOF

scalpel patch 'class:InvoiceRepository' tests/fixtures/sample-complex.ts --body-file /tmp/replacement-class.tsfrag --apply
```

### 8.3 Replace if statement body in one method

```bash
scalpel patch 'method:chooseTier' tests/fixtures/sample-complex.ts --replace 'if (amount > 1000) {\n      return "enterprise";\n    }=>if (amount > 1000) {\n      return "platinum";\n    }' --apply
```

## 9. Markdown and text updates

```bash
scalpel patch 'heading:Scalpel Guide' tests/fixtures/sample.md --body '# Scalpel Guide Updated' --apply
scalpel patch 'key:status' tests/fixtures/sample.txt --replace 'queued=>running' --apply
```

## 10. JSON/YAML/TOML/JSONL scoped replacements

```bash
scalpel patch 'key:service.mode' tests/fixtures/sample.json --replace 'safe=>strict' --apply
scalpel patch 'key:mode' tests/fixtures/sample.yaml --replace 'safe=>strict' --apply
scalpel patch 'key:service.mode' tests/fixtures/sample.toml --replace 'safe=>strict' --apply
scalpel patch 'key:line1.state' tests/fixtures/sample.jsonl --replace 'queued=>running' --apply
```

## 11. Peek command for paginated file reading

```bash
scalpel peek tests/fixtures/sample.go --page-size 5 --page 1
scalpel peek tests/fixtures/sample.go --from-line 7 --to-line 12 --all
scalpel --json peek tests/fixtures/sample.go --from-pos 7 --to-pos 12 --all
```

## 12. Bash completion

```bash
scalpel completion bash > /tmp/scalpel.bash
source /tmp/scalpel.bash
```

## 13. Import group and import-line swaps

Go grouped imports as one structural block:

```bash
cat > /tmp/imports.go.frag << 'EOF'
import (
    "strings"
    "fmt"
)
EOF

scalpel patch 'import:import' tests/fixtures/sample-import-groups.go --body-file /tmp/imports.go.frag --apply
```

Rust/TS/JS import-line swaps with explicit index:

```bash
scalpel find 'import:*' src/main.rs
scalpel patch 'import:*' src/main.rs --index 1 --replace 'std::io=>std::fs' --apply

scalpel find 'import:*' app.ts
scalpel patch 'import:*' app.ts --index 2 --replace 'from "lib-a"=>from "lib-b"' --apply

scalpel find 'import:*' app.js
scalpel patch 'import:*' app.js --index 1 --replace 'from "node:fs"=>from "node:fs/promises"' --apply
```
