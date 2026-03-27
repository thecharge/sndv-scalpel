#!/usr/bin/env bash
# ensure-scalpel.sh — check if scalpel is installed, install it if not, and verify the alias.
#
# Usage:
#   ./scripts/ensure-scalpel.sh              # install latest release
#   ./scripts/ensure-scalpel.sh 0.1.0        # install specific version
#
# After running, scalpel is available at $HOME/.local/bin/scalpel and on PATH.
# The script is safe to run repeatedly — it exits 0 immediately if scalpel is
# already installed and on PATH.

set -euo pipefail

VERSION="${1:-latest}"
INSTALL_DIR="${SCALPEL_INSTALL_DIR:-$HOME/.local/bin}"
BIN="$INSTALL_DIR/scalpel"
REPO="thecharge/sndv-scalpel"

# ─── helpers ────────────────────────────────────────────────────────────────

info()  { echo "[scalpel] $*"; }
warn()  { echo "[scalpel] WARNING: $*" >&2; }
die()   { echo "[scalpel] ERROR: $*" >&2; exit 1; }

require_cmd() {
  if ! command -v "$1" &>/dev/null; then
    echo ""
    echo "  Missing required tool: $1"
    echo "  $2"
    echo ""
    die "prerequisite not satisfied: $1"
  fi
}

# ─── already installed? ──────────────────────────────────────────────────────

if command -v scalpel &>/dev/null; then
  info "scalpel is already on PATH: $(command -v scalpel)"
  scalpel --version
  exit 0
fi

if [[ -x "$BIN" ]]; then
  info "scalpel found at $BIN but not on PATH — adding to PATH for this session"
  export PATH="$INSTALL_DIR:$PATH"
  scalpel --version
  info "To make this permanent, add to your shell profile:"
  info "  export PATH=\"$INSTALL_DIR:\$PATH\""
  exit 0
fi

# ─── detect OS / arch ───────────────────────────────────────────────────────

OS="$(uname -s 2>/dev/null || echo unknown)"
ARCH="$(uname -m 2>/dev/null || echo unknown)"

case "$OS" in
  Linux)  platform="linux" ;;
  Darwin) platform="macos" ;;
  MINGW*|MSYS*|CYGWIN*)
    echo ""
    echo "  Windows detected. Run the PowerShell installer instead:"
    echo ""
    echo "    iwr https://raw.githubusercontent.com/${REPO}/main/scripts/install-from-github.ps1 -OutFile install-scalpel.ps1"
    echo "    powershell -ExecutionPolicy Bypass -File .\\install-scalpel.ps1"
    echo ""
    exit 1
    ;;
  *)
    warn "unknown OS '$OS' — will attempt Linux binary"
    platform="linux"
    ;;
esac

case "$ARCH" in
  x86_64|amd64)   arch_name="x86_64" ;;
  arm64|aarch64)  arch_name="aarch64" ;;
  *)
    warn "unknown arch '$ARCH' — falling back to source build via cargo"
    arch_name=""
    ;;
esac

# ─── prereq check ───────────────────────────────────────────────────────────

if [[ -n "$arch_name" ]]; then
  # binary install path — needs curl + tar
  require_cmd curl  "Install curl:
    Linux (Debian/Ubuntu):  sudo apt-get install -y curl
    Linux (Fedora/RHEL):    sudo dnf install -y curl
    Linux (Alpine):         apk add curl
    macOS:                  brew install curl  (or it ships with macOS)"
  require_cmd tar   "Install tar:
    Linux (Debian/Ubuntu):  sudo apt-get install -y tar
    Linux (Alpine):         apk add tar
    macOS:                  ships with macOS"
else
  # source build path — needs cargo
  require_cmd cargo "Install Rust + cargo:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    Then restart your shell or run:  source \$HOME/.cargo/env"
fi

# ─── install ────────────────────────────────────────────────────────────────

mkdir -p "$INSTALL_DIR"

if [[ -n "$arch_name" ]]; then
  # ── binary install from GitHub releases ────────────────────────────────

  if [[ "$VERSION" == "latest" ]]; then
    info "resolving latest release version..."
    VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
      | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"v\{0,1\}\([^"]*\)".*/\1/p' \
      | head -n1)"
    [[ -n "$VERSION" ]] || die "failed to resolve latest version from GitHub API"
  fi

  asset="scalpel-${VERSION}-${platform}-${arch_name}.tar.gz"
  url="https://github.com/${REPO}/releases/download/v${VERSION}/${asset}"

  info "downloading $asset..."
  tmp_dir="$(mktemp -d)"
  trap 'rm -rf "$tmp_dir"' EXIT

  if ! curl -fL "$url" -o "$tmp_dir/$asset" 2>/dev/null; then
    warn "binary download failed — release may not exist yet"
    warn "falling back to source build via cargo"
    arch_name=""  # trigger source build below
  else
    tar -xzf "$tmp_dir/$asset" -C "$tmp_dir"
    src_bin="$(find "$tmp_dir" -type f -name "scalpel" | head -n1)"
    [[ -n "$src_bin" ]] || die "scalpel binary not found in downloaded archive"
    install -m 0755 "$src_bin" "$BIN"
    info "installed scalpel $VERSION to $BIN"
  fi
fi

if [[ -z "$arch_name" ]] && [[ ! -x "$BIN" ]]; then
  # ── source build via cargo ─────────────────────────────────────────────

  require_cmd cargo "Install Rust + cargo:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    Then restart your shell or run:  source \$HOME/.cargo/env"

  info "building scalpel from source (this takes ~1 minute)..."
  if [[ "$VERSION" == "latest" ]]; then
    cargo install --git "https://github.com/${REPO}" --bin scalpel --root "$tmp_dir/.cargo_install" 2>&1
    src_bin="$(find "$tmp_dir/.cargo_install" -type f -name "scalpel" | head -n1)"
  else
    cargo install --git "https://github.com/${REPO}" --tag "v${VERSION}" --bin scalpel --root "$tmp_dir/.cargo_install" 2>&1
    src_bin="$(find "$tmp_dir/.cargo_install" -type f -name "scalpel" | head -n1)"
  fi
  [[ -n "$src_bin" ]] || die "scalpel binary not found after cargo build"
  install -m 0755 "$src_bin" "$BIN"
  info "installed scalpel to $BIN"
fi

# ─── PATH setup ─────────────────────────────────────────────────────────────

export PATH="$INSTALL_DIR:$PATH"

if ! command -v scalpel &>/dev/null; then
  die "installation appeared to succeed but scalpel is not on PATH — check $BIN"
fi

info "scalpel is ready: $(scalpel --version)"
echo ""
echo "  To make scalpel available in every new shell, add to your profile"
echo "  (~/.bashrc, ~/.zshrc, or ~/.profile):"
echo ""
echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
echo ""
echo "  Then reload your profile:"
echo ""
echo "    source ~/.bashrc   # or: source ~/.zshrc"
echo ""
echo "  Optional — enable tab completion:"
echo ""
echo "    scalpel completion bash >> ~/.bashrc"
echo "    scalpel completion zsh  >> ~/.zshrc"
echo ""
