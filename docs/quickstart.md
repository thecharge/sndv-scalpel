# Quick Start

## 1. Build

```bash
./scripts/build.sh
./target/release/scalpel --version
```

## 2. Run basic discovery

```bash
./target/release/scalpel find 'fn:*' tests/fixtures --recursive
```

## 3. Preview a safe change

```bash
./target/release/scalpel diff 'fn:CalculateTotal' tests/fixtures/sample.go --rename sum=total
```

## 4. Apply with transaction safety

```bash
./target/release/scalpel patch 'fn:CalculateTotal' tests/fixtures/sample.go --rename sum=total --apply
```

## 5. Use centralized configuration

```bash
./target/release/scalpel --config config/scalpel.yaml find 'key:*' tests/fixtures/sample.json
```

## 6. Prove quality gates

```bash
./scripts/handle.sh check
```

## 7. Inline body and scoped data replacements

```bash
./target/release/scalpel patch 'method:chooseTier' tests/fixtures/sample-complex.ts --body 'public chooseTier(amount: number): "basic" | "enterprise" { return "basic"; }' --apply
./target/release/scalpel patch 'key:status' tests/fixtures/sample.txt --replace 'queued=>running' --apply
```

## 8. Peek file content by page or position

```bash
./target/release/scalpel peek tests/fixtures/sample.go --page-size 5 --page 1
./target/release/scalpel peek tests/fixtures/sample.go --from-pos 7 --to-pos 12 --all
```

## 9. Enable bash completion

```bash
./target/release/scalpel completion bash > /tmp/scalpel.bash
source /tmp/scalpel.bash
```

## 10. Add shell alias for daily usage

```bash
echo 'alias scalpel="$HOME/.local/bin/scalpel"' >> ~/.bashrc
source ~/.bashrc
```

## 11. Next docs to read

- Full command examples: docs/usage-guide.md
- Extension steps: docs/extension-guide.md
- Feature and issue process: docs/features-and-issues.md
- Compliance and proof details: docs/compliance-and-proof.md
- Deep test/benchmark examples: docs/integration-e2e.md

## 12. View parity commands

```bash
./target/release/scalpel view tests/fixtures/sample.go --outline
./target/release/scalpel view tests/fixtures/sample.rs --lines 1:40
./target/release/scalpel --json view 'fn:calculate_total' tests/fixtures/sample.rs
```

## 13. JSON diff and patch

```bash
./target/release/scalpel --json diff 'fn:CalculateTotal' tests/fixtures/sample.go --rename sum=total
./target/release/scalpel --json patch 'fn:CalculateTotal' tests/fixtures/sample.go --rename sum=total --apply
```
