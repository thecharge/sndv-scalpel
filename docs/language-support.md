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
- Go: function, method, type, import (single-line and grouped import blocks)
- Rust: function, type, method (impl), import
- Markdown: heading
- Text/YAML/JSON/JSONL/TOML: key paths

Import swap notes:

- Go grouped imports can be swapped as one block with `patch 'import:import' --body-file ...`.
- JS/TS/Rust imports are typically line-scoped and are easiest to target via `find 'import:*'` plus `--index`.

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

## Command parity highlights

- `view --outline` supports structural file outlines.
- `view --lines start:end` supports explicit line windows with `--all` to disable safety caps.
- JSON outputs are available for `find`, `peek`, `view`, `diff`, and `patch`.

Quick command to verify language coverage:

```bash
cargo test --test cli_integration
```
