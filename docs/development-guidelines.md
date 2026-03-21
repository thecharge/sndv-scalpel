# Development Guidelines

## Design constraints

- Keep source files under 300 lines.
- Keep control flow shallow: prefer early returns and guard clauses.
- Avoid stringly-typed protocol logic where enums or typed structs fit.
- Reuse shared helpers to stay DRY.
- Keep command orchestration in handlers, not in one monolithic app file.

## SOLID usage in this codebase

- Single responsibility: each command handler owns one use case.
- Open/closed: new parser strategies are added via config and parser strategy enum.
- Liskov and interface discipline: parser strategy handlers return the same symbol model.
- Interface segregation: command modules only depend on required helpers.
- Dependency inversion: app depends on config and registry abstractions.

## Parser policy

- Prefer RFC-compatible libraries for structured data:
  - JSON/JSONL via `serde_json`
  - YAML via `serde_yaml`
  - TOML via `toml`
- Regex parser is configuration-driven for language-specific symbol extraction.

## Testing policy

- Happy flow tests: expected command outcomes.
- Side flow tests: ambiguity, no-match, or invalid input behavior.
- Critical path tests: rollback and write-failure safety.
- Large input tests: at least one >=10k LOC scenario.
