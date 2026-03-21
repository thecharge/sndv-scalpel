# Language Support Matrix

Language support is dynamic. You control it from the config file.

Recommended path for user-level setup:

- `$HOME/.config/scalpel/scalpel.yaml`

Project-level override:

- `config/scalpel.yaml`

## Configured support

- JavaScript (`.js`, `.cjs`, `.mjs`, `.jsx`)
- TypeScript (`.ts`, `.tsx`)
- Go (`.go`)
- Rust (`.rs`)
- Lua (`.lua`)
- Markdown (`.md`, `.markdown`)
- Text (`.txt`)
- YAML (`.yaml`, `.yml`)
- JSON (`.json`)
- JSON Lines (`.jsonl`)
- TOML (`.toml`)

## Symbol kinds by language

- JavaScript/TypeScript/Lua: function, class, import (based on config)
- Go: function, method, type, import
- Rust: function, type, method (impl), import
- Markdown: heading
- Text/YAML/JSON/JSONL/TOML: key paths

## Pattern shorthands

- `fn:*`
- `class:*`
- `method:*`
- `type:*`
- `import:*`
- `heading:*`
- `key:*`

## Proven coverage

See integration tests in `tests/cli_integration.rs`, heavy tests in `tests/heavy_paths.rs`, and chaos tests in `tests/transaction_chaos.rs`.

Quick command to verify language coverage:

```bash
cargo test --test cli_integration
```
