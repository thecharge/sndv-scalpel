# Security Policy

## Supported versions

Only the latest release is supported for security fixes.

## Reporting a vulnerability

1. Do not open public issues for security vulnerabilities.
2. Contact maintainers privately with reproduction steps, impact, and affected files.
3. Allow reasonable time for triage and remediation before disclosure.

## Secure defaults in this project

- `patch` is dry-run unless `--apply` is provided.
- Writes are atomic via temp-file + rename.
- Automatic rollback restores snapshot copies on write failure.
- File handling uses buffered I/O to avoid partial stream writes.
