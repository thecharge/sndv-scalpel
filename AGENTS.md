# AGENTS.md

## Purpose

Shared operating guide for coding agents working in this repository.

## Mission

Build and maintain a safe, production-ready CLI for structural find/diff/patch workflows.

## Core rules

- Keep safety defaults: `diff` is dry-run and `patch` requires `--apply`.
- Keep parser behavior configuration-driven from `scalpel.yaml`.
- Prefer early returns and shallow control flow.
- Keep source files under 300 lines.
- Keep tests for happy, side, and critical paths.
- Update docs when behavior changes.

## Required checks before finishing

```bash
./scripts/handle.sh check
```

## Required docs to update when adding behavior

- `README.md`
- `docs/quickstart.md`
- `docs/integration-e2e.md`
- `docs/language-support.md` when language behavior changes
- `docs/extension-guide.md` when extension surface changes

## Testing expectations

- Add focused unit tests for logic changes.
- Add integration tests for CLI behavior changes.
- Add critical safety tests for transaction and rollback paths.
- Keep heavy-path coverage for large inputs.

## Useful commands

```bash
./scripts/build.sh
./scripts/test.sh
./scripts/lint.sh
./scripts/fmt.sh
./scripts/package-release.sh
```
