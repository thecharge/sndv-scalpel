# Extension Guide

## Adding a new language

1. Add a language entry in `config/scalpel.yaml`.
2. Choose a parser strategy (`regex`, `markdown`, `yaml`, `json`, `jsonl`, `toml`).
3. For `regex`, add one or more symbol patterns.
4. Add fixture content under `tests/fixtures`.
5. Add integration assertions in `tests/cli_integration.rs`.
6. Update `docs/language-support.md`.

## Adding a new symbol kind

1. Extend `SymbolKind` in `src/model.rs`.
2. Add shorthand mapping in `src/query.rs`.
3. Update parser mappings.
4. Add tests for matching and output display.

## Adding a new command

1. Add command shape in `src/cli.rs`.
2. Implement handler module under `src/commands/`.
3. Wire dispatch in `src/commands/mod.rs`.
3. Add integration test path and expected output.
4. Update README usage examples.
