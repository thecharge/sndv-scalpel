# Architecture

## Components

- CLI layer (`src/cli.rs`): command model and user input validation.
- App layer (`src/app.rs`): loads central config and dispatches commands.
- Command handlers (`src/commands/*`): command-pattern style execution per use case.
- Config layer (`src/config.rs` + `config/scalpel.yaml`): dynamic language and parser settings.
- Language registry (`src/lang.rs`): extension to language config resolution.
- Parser layer (`src/parser/*`): strategy-based symbol extraction.
- Query layer (`src/query.rs`): shorthand matching with typed prefixes.
- Transaction layer (`src/transaction.rs`): snapshot, atomic write, rollback.

## Safety model

1. Parse and match symbol range.
2. Generate diff for visibility.
3. Write only when `--apply` is passed.
4. Snapshot files before write.
5. Perform atomic temp-file rename.
6. Roll back from snapshot on any failure.

## Async and stream usage

- Concurrent file parsing with `futures::stream::iter(...).buffer_unordered(...)`.
- Buffered line streaming reads via `tokio::io::BufReader` and `read_line` loops.
- Buffered writes via `tokio::io::BufWriter`.

## Why this is production-oriented

- Explicit error types for user-facing failures.
- Deterministic default-safe behavior.
- Integration tests covering multi-language and patch flows.
