# Integration and E2E Guide

## Build and test

```bash
cargo test --all-targets
```

## E2E scenario: safe rename in Go

```bash
cp tests/fixtures/sample.go /tmp/scalpel-go-sample.go
scalpel diff 'fn:CalculateTotal' /tmp/scalpel-go-sample.go --rename sum=total
scalpel patch 'fn:CalculateTotal' /tmp/scalpel-go-sample.go --rename sum=total --apply
```

Expected:

- `diff` prints a unified patch and does not modify files.
- `patch --apply` writes changes and prints an applied message.

## E2E scenario: Go function body swap

```bash
cat > /tmp/new-total.go << 'EOF'
func CalculateTotal(items []int) int {
	total := 100
	return total
}
EOF

scalpel diff 'fn:CalculateTotal' tests/fixtures/sample.go --body-file /tmp/new-total.go
scalpel patch 'fn:CalculateTotal' tests/fixtures/sample.go --body-file /tmp/new-total.go --apply
```

Expected:

- matched function block is swapped with new file content
- patch remains transactional

## E2E scenario: inline body swap without temp file

```bash
cp tests/fixtures/sample-complex.ts /tmp/scalpel-complex.ts
scalpel patch 'method:chooseTier' /tmp/scalpel-complex.ts --body 'public chooseTier(amount: number): "basic" | "enterprise" { return "basic"; }' --apply
```

Expected:

- selected method block is replaced directly from the command line
- no temporary body file is required

## E2E scenario: ternary and if-block scoped replacements

```bash
scalpel patch 'fn:main' app.js --replace 'flag ? true : false=>flag ? 1 : 0' --apply
scalpel patch 'fn:run' app.ts --replace 'if (flag) { return 1; }=>if (flag) { return 2; }' --apply
scalpel patch 'fn:CalculateTotal' app.go --replace 'items []int=>numbers []int' --apply
```

Expected:

- replacement happens only in selected symbol scope
- neighboring blocks stay unchanged

## E2E scenario: complex TypeScript class/method operations

```bash
scalpel find 'class:*' tests/fixtures/sample-complex.ts
scalpel find 'method:*' tests/fixtures/sample-complex.ts
```

Method swap:

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

Whole class replacement:

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

Scoped if-body replacement in method:

```bash
scalpel patch 'method:chooseTier' tests/fixtures/sample-complex.ts --replace 'if (amount > 1000) {\n      return "enterprise";\n    }=>if (amount > 1000) {\n      return "platinum";\n    }' --apply
```

Expected:

- large method block swap works
- whole class block replacement works
- if-body replacement happens only in selected method scope

## E2E scenario: language inventory sweep

```bash
scalpel find 'fn:*' tests/fixtures --recursive
scalpel find 'heading:*' tests/fixtures/sample.md
scalpel find 'key:*' tests/fixtures/sample.yaml
scalpel find 'fn:*' tests/fixtures/sample.lua
scalpel find 'key:*' tests/fixtures/sample.json
```

Expected:

- JS/TS/Go/Rust functions are discovered.
- Lua functions are discovered.
- Markdown headings and YAML/JSON keys are discovered.

## E2E scenario: JSONL patch side flow

```bash
cp tests/fixtures/sample.jsonl /tmp/scalpel-jsonl-sample.jsonl
scalpel patch 'key:line1.state' /tmp/scalpel-jsonl-sample.jsonl --rename queued=running --apply
```

Expected:

- first record state transitions from `queued` to `running`
- operation is transactional and atomic

## E2E scenario: text and structured data scoped replacements

```bash
cp tests/fixtures/sample.txt /tmp/scalpel-sample.txt
cp tests/fixtures/sample.yaml /tmp/scalpel-sample.yaml
cp tests/fixtures/sample.json /tmp/scalpel-sample.json
cp tests/fixtures/sample.toml /tmp/scalpel-sample.toml

scalpel patch 'key:status' /tmp/scalpel-sample.txt --replace 'queued=>running' --apply
scalpel patch 'key:mode' /tmp/scalpel-sample.yaml --replace 'safe=>strict' --apply
scalpel patch 'key:service.mode' /tmp/scalpel-sample.json --replace 'safe=>strict' --apply
scalpel patch 'key:service.mode' /tmp/scalpel-sample.toml --replace 'safe=>strict' --apply
```

Expected:

- replacements stay scoped to the selected key symbol
- only targeted values change

## E2E scenario: huge one-line JSONL surgical patch

Use real fixture dataset commands:

```bash
cp tests/fixtures/big/events-10k.jsonl /tmp/scalpel-events-10k.jsonl
scalpel patch 'key:line9001.state' /tmp/scalpel-events-10k.jsonl --rename queued=running --apply
sed -n '9000,9002p' /tmp/scalpel-events-10k.jsonl
```

Automated assertive proof commands:

```bash
cargo test --test heavy_paths huge_jsonl_surgical_patch_single_line_only
cargo test --test heavy_paths deep_line_target_jsonl_patch
cargo test --test heavy_paths fixture_big_jsonl_precise_line_patch
```

Expected:

- only the selected JSONL line is updated
- neighboring lines remain unchanged
- fixture-based 10k dataset patch stays surgical

## E2E scenario: huge JSONL dry-run safety

```bash
cargo test --test heavy_paths huge_jsonl_diff_does_not_modify_file
```

Expected:

- `diff` reports changes
- file content remains unchanged

## E2E scenario: large codebase fixtures

```bash
scalpel find 'fn:*' tests/fixtures/big/large-service.rs
scalpel find 'fn:*' tests/fixtures/big/large-service.go
```

Expected:

- parser scans large code files and returns structural matches

## E2E scenario: 10k LOC heavy path

Use automated test proof:

```bash
cargo test --test heavy_paths parses_10k_loc_file_happy_flow
```

Expected:

- parser and query flow remains fast and stable at 10k LOC scale

## JSON integration mode

```bash
scalpel --json find 'fn:*' tests/fixtures --recursive
```

Use this for tooling pipelines and machine consumption.

For full examples, see `docs/usage-guide.md`.
