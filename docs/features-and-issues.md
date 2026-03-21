# Features and Issues

## Feature list

- Safe diff first, patch only with `--apply`
- Transaction rollback on write failure
- Dynamic language support via config
- JSON output for automation
- Large-file and chaos test coverage

## Where to request features

- Open a feature request in `.github/ISSUE_TEMPLATE/feature_request.md`

## Where to report bugs

- Open a bug report in `.github/ISSUE_TEMPLATE/bug_report.md`

## How features are added

1. Update `config/scalpel.yaml` or parser modules
2. Add tests for happy/side/critical paths
3. Update docs and quick start examples
4. Open PR using `.github/PULL_REQUEST_TEMPLATE.md`
