# CLAUDE.md

## What this is

`scalpel` — a structural-aware, multi-language CLI for safe code discovery and scoped edits. Instead of fragile line numbers, you query by symbol identity (`fn:CalculateTotal`, `class:InvoiceRepository`, `key:service.mode`). Every write is transactional (snapshot → atomic rename → rollback). Every command has `--json` output for agent pipelines.

Binary name: `scalpel`. Crate name: `sndv-scalpel`. Creator: Radoslav Sandov.

---

## Required check before finishing any change

```bash
./scripts/handle.sh check
```

This runs fmt → clippy → all tests. Do not skip it.

---

## Project layout

```
src/
  main.rs           # entry: run(), exit 1 for no-match, exit 2 for errors
  lib.rs            # pub module exports
  app.rs            # CLI parse → config load → registry build → dispatch
  cli.rs            # clap command model (all subcommands defined here)
  commands/
    mod.rs          # dispatch() — routes Command enum to handlers
    find.rs         # concurrent file traversal, symbol filter, sorted output
    view.rs         # symbol match + context, outline tree, line range window
    peek.rs         # paginated file read (no language registry needed)
    info.rs         # file metadata + symbol list
    patch.rs        # diff/patch handler, PatchRequest struct, transactional apply
    util.rs         # shared helpers: collect_files, select_symbol, scoped ops
    completion.rs   # shell completion via clap_complete
  parser/
    mod.rs          # parse_path() dispatcher → ParsedFile
    regex_parser.rs # config-driven regex symbol extraction for code languages
    data_parser.rs  # markdown, yaml, json, jsonl, toml parsers
    stream_io.rs    # buffered async read with max_bytes guard
  config.rs         # AppConfig + LanguageConfig loader (scalpel.yaml)
  constants.rs      # shared string constants and default values
  error.rs          # ScalpelError enum (thiserror)
  lang.rs           # LanguageRegistry: extension → LanguageConfig
  model.rs          # Symbol, MatchOutput, SymbolKind, EngineMode, Confidence
  query.rs          # shorthand query parsing: fn:calc* → Query with glob matcher
  transaction.rs    # snapshot + atomic_write + rollback
config/
  scalpel.yaml      # language registry: all language support lives here
tests/
  cli_integration.rs
  heavy_paths.rs
  transaction_chaos.rs
  format_ops.rs
  peek_completion.rs
  fixtures/         # sample source files per language + big/ for 10k LOC tests
docs/
  llm-usage.md      # agent/LLM usage patterns, exit codes, JSON shapes
  architecture.md
  quickstart.md
  ...
AGENTS.md           # agent usage guide + repo contributor rules
SKILL.md            # LLM skill metadata and JSON output shapes
```

---

## Exit codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | No matches (`ScalpelError::NoMatch` or `NoMatchFound`) |
| `2` | All other errors |

The downcast in `main.rs` controls this. Keep both `NoMatch` (single-file, has a path) and `NoMatchFound` (find across N files, no path) triggering exit 1.

---

## Key design rules

- **Config-driven language support**: all symbol patterns live in `scalpel.yaml`. Adding a language requires zero Rust changes — just a new YAML entry. Do not hardcode language behavior in Rust.
- **Safety defaults**: `diff` is always dry-run. `patch` requires explicit `--apply`. Never change this default.
- **Byte-precise scoping**: edits use `start_byte`/`end_byte` from the parsed Symbol. The scope boundary must hold — renames and replacements must not bleed outside the matched symbol range.
- **Deterministic `find` output**: results are sorted by `(file, start_line)` before printing. `buffer_unordered` is non-deterministic; the sort is load-bearing for agent reliability.
- **No AST**: symbol extraction is regex-based (fast, config-simple). Structured data formats (JSON, YAML, TOML, JSONL) use RFC-compliant crate parsers.
- **Shallow control flow**: prefer early returns, max 5 nesting levels (clippy enforces it).
- **Source files ≤ 300 lines**.

---

## Adding a new language

Edit `config/scalpel.yaml` only — add a new entry under `languages` with `id`, `extensions`, `strategy: regex`, `tier`, and `patterns`. No Rust changes needed.

## Adding a new command

1. Add variant to `cli.rs` `Command` enum with clap attrs and after_long_help examples.
2. Create `src/commands/<cmd>.rs` with a `run()` function.
3. Add module and dispatch arm in `commands/mod.rs`.
4. Add integration tests in `tests/cli_integration.rs`.
5. Update `docs/quickstart.md`, `README.md`, `AGENTS.md`, `SKILL.md`.
6. Update shell completion examples in the CLI's `after_long_help`.

---

## Clippy / fmt rules

- Max cognitive complexity: 25
- No single-char binding names
- Max 5 nesting levels
- Max 500 type complexity
- Line width: 100 (`rustfmt.toml`)

---

## Tests

Each test file has a clear role:

| File | What it tests |
|------|---------------|
| `cli_integration.rs` | All CLI commands across languages, happy + side + critical paths |
| `heavy_paths.rs` | 10k LOC parsing, surgical JSONL precision, large file handling |
| `transaction_chaos.rs` | Rollback after write failure |
| `format_ops.rs` | `--body`, `--replace` across data formats |
| `peek_completion.rs` | Pagination, position ranges, shell completion output |

For new code: unit tests inline in the module (`#[cfg(test)]`), integration tests in `tests/`.

---

## Docs to update when behavior changes

- `README.md` — core commands section
- `docs/quickstart.md`
- `docs/integration-e2e.md`
- `docs/language-support.md` — when language behavior changes
- `docs/extension-guide.md` — when extension surface changes
- `AGENTS.md` + `SKILL.md` — when CLI surface or exit codes change
- `docs/llm-usage.md` — when JSON output shapes change
