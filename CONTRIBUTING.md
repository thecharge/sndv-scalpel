# Contributing

## Development setup

1. Install stable Rust with rustup.
2. Clone the repository.
3. Run `./scripts/handle.sh check`.
4. Install local hooks with `./scripts/install-hooks.sh`.

## Pull request workflow

1. Create a branch from `main`.
2. Keep commits focused and atomic.
3. Ensure `cargo fmt`, `cargo clippy`, and `cargo test` pass.
4. Update docs for any behavior change.
5. Open a PR using the template in `.github/PULL_REQUEST_TEMPLATE.md`.

## Standards

- Safety-first changes: default behavior must remain dry-run for file modifications.
- New language support must include parser tests and integration tests.
- Keep error messages actionable.

## Local verification checklist

- `./scripts/handle.sh fmt`
- `./scripts/handle.sh lint`
- `./scripts/handle.sh test`
- At least one manual end-to-end command from docs/integration-e2e.md

See `docs/development-guidelines.md` for architecture and coding constraints.
