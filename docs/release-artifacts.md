# Release Artifacts

## Generate package

```bash
./scripts/package-release.sh
```

Generate multi-platform build outputs and packages:

```bash
./scripts/build-matrix.sh
./scripts/package-matrix.sh
```

## Output

- `dist/scalpel-<version>-linux-x86_64.tar.gz`
- `dist/scalpel-<version>-linux-x86_64.sha256`
- `dist/scalpel-<version>-linux-aarch64.tar.gz` (if target built)
- `dist/scalpel-<version>-macos-x86_64.tar.gz` (if target built)
- `dist/scalpel-<version>-macos-aarch64.tar.gz` (if target built)
- `dist/scalpel-<version>-windows-x86_64.zip` (if target built)

## Package contents

- `scalpel` binary
- `README.md`
- `LICENSE`
- `docs/`

## Verification

```bash
cd dist
sha256sum -c scalpel-<version>-linux-x86_64.sha256
```

## Installer scripts for users

- Linux/macOS installer: `scripts/install-from-github.sh`
- Windows installer: `scripts/install-from-github.ps1`
