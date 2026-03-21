# Compliance and Proof

## What RFC/spec-compliant means here

This tool uses mature parser libraries for structured data formats:

- JSON: `serde_json` (RFC 8259 behavior)
- TOML: `toml` crate (TOML spec behavior)
- YAML: `serde_yaml` (YAML parser behavior)

For programming languages (Go, Rust, JS, TS, Lua), symbol extraction is config-driven regex parsing. That is practical and fast, but it is not a full compiler parser.

## Proof that required tests exist

- Happy flows:
  - `tests/cli_integration.rs`
  - `tests/heavy_paths.rs`
- Side flows:
  - `tests/cli_integration.rs` (ambiguity and JSONL patch)
- Critical paths:
  - `tests/transaction_chaos.rs` (rollback after failure)

## Commands used to prove behavior

```bash
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo run --quiet --bin scalpel -- --help
```

## Streaming buffer proof

- Stream read path: `src/parser/stream_io.rs`
- Buffered write path: `src/transaction.rs`

Both are active in parse and patch flows.
