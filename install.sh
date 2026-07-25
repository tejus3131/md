#!/usr/bin/env bash
# Install the latest md release binary into ~/.local/bin
set -euo pipefail

REPO="tejus3131/md"
INSTALL_DIR="${MD_INSTALL_DIR:-${HOME}/.local/bin}"

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Darwin) os_tag="apple-darwin" ;;
  Linux) os_tag="unknown-linux-gnu" ;;
  *)
    echo "unsupported OS: $os" >&2
    exit 1
    ;;
esac

case "$arch" in
  arm64|aarch64) arch_tag="aarch64" ;;
  x86_64|amd64) arch_tag="x86_64" ;;
  *)
    echo "unsupported arch: $arch" >&2
    exit 1
    ;;
esac

TARGET="${arch_tag}-${os_tag}"
ASSET="md-${TARGET}.tar.gz"

echo "detecting latest release…"
API="https://api.github.com/repos/${REPO}/releases/latest"
TAG="$(curl -fsSL "$API" | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' | head -1)"
if [ -z "$TAG" ]; then
  echo "could not find a release. Install from source:" >&2
  echo "  cargo install --git https://github.com/${REPO} --locked" >&2
  exit 1
fi

URL="https://github.com/${REPO}/releases/download/${TAG}/${ASSET}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

echo "downloading ${URL}"
if ! curl -fsSL "$URL" -o "${TMP}/${ASSET}"; then
  echo "download failed for ${ASSET}." >&2
  echo "Install from source: cargo install --git https://github.com/${REPO} --locked" >&2
  exit 1
fi

tar -xzf "${TMP}/${ASSET}" -C "$TMP"
mkdir -p "$INSTALL_DIR"
install -m 755 "${TMP}/md-${TARGET}/md" "${INSTALL_DIR}/md"

echo "installed ${INSTALL_DIR}/md (${TAG})"
case ":$PATH:" in
  *":${INSTALL_DIR}:"*) ;;
  *)
    echo "add to PATH: export PATH=\"${INSTALL_DIR}:\$PATH\""
    ;;
esac

"${INSTALL_DIR}/md" --version || true
