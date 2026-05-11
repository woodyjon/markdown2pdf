#!/usr/bin/env sh
# markdown2pdf installer — fetches the latest CLI binary from GitHub Releases.
#
#   curl -fsSL https://markdown2pdf.eu/install.sh | sh
#   curl -fsSL https://markdown2pdf.eu/install.sh | sh -s -- --to ~/.local/bin
#
# Flags:
#   --to <dir>    Install directory (default: /usr/local/bin, falls back to ~/.local/bin if not writable)
#   --version vX  Pin a release tag (default: latest)
#   --no-sudo     Never call sudo; use ~/.local/bin if /usr/local/bin needs sudo
#   --help        Show this help and exit
#
# The script:
#   1. detects your OS/arch via uname
#   2. downloads from https://github.com/woodyjon/markdown2pdf/releases/<tag>/download/markdown2pdf-<target>.<ext>
#   3. verifies the SHA256 against the SHA256SUMS asset published with the release
#      (fail-closed: if SHA256SUMS can't be fetched or doesn't match, the install aborts)
#   4. extracts the binary into the install dir

set -eu

REPO="woodyjon/markdown2pdf"
INSTALL_DIR="/usr/local/bin"
RELEASE="latest"
NO_SUDO=0

usage() {
  sed -n '2,17p' "$0" | sed 's/^# \{0,1\}//'
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --to)        INSTALL_DIR="$2"; shift 2 ;;
    --version)   RELEASE="$2"; shift 2 ;;
    --no-sudo)   NO_SUDO=1; shift ;;
    --help|-h)   usage; exit 0 ;;
    *)           printf 'Unknown flag: %s\n' "$1" >&2; usage >&2; exit 2 ;;
  esac
done

uname_s=$(uname -s 2>/dev/null || echo unknown)
uname_m=$(uname -m 2>/dev/null || echo unknown)

case "${uname_s}-${uname_m}" in
  Darwin-arm64)                            target=aarch64-apple-darwin;     ext=tar.gz; bin=markdown2pdf ;;
  Darwin-x86_64)
    printf 'Intel-Mac binaries are not prebuilt for this release.\n' >&2
    printf 'On Apple Silicon, install Rosetta and re-run; or build from source:\n' >&2
    printf '  cargo install --git https://github.com/%s markdown2pdf-cli\n' "$REPO" >&2
    exit 1 ;;
  Linux-x86_64|Linux-amd64)                target=x86_64-unknown-linux-gnu; ext=tar.gz; bin=markdown2pdf ;;
  MINGW*-x86_64|MSYS*-x86_64|CYGWIN*-x86_64)
    # Git Bash / MSYS / Cygwin on Windows. Use the .zip and the native PowerShell to extract.
    target=x86_64-pc-windows-msvc; ext=zip; bin=markdown2pdf.exe ;;
  Windows*)
    printf 'It looks like you are on native Windows but running this script through a non-Unix shell.\n' >&2
    printf 'Use the PowerShell installer instead:\n' >&2
    printf '  powershell -c "irm https://markdown2pdf.eu/install.ps1 | iex"\n' >&2
    exit 1 ;;
  *)
    printf 'Unsupported platform: %s-%s\n' "$uname_s" "$uname_m" >&2
    printf 'No prebuilt binary is published for this OS/arch combination.\n' >&2
    printf 'Options:\n' >&2
    printf '  1. Browse releases:  https://github.com/%s/releases/latest\n' "$REPO" >&2
    printf '  2. Build from source: cargo install --git https://github.com/%s markdown2pdf-cli\n' "$REPO" >&2
    exit 1 ;;
esac

if [ "$RELEASE" = "latest" ]; then
  base_url="https://github.com/${REPO}/releases/latest/download"
else
  base_url="https://github.com/${REPO}/releases/download/${RELEASE}"
fi

asset="markdown2pdf-${target}.${ext}"
url="${base_url}/${asset}"
sums_url="${base_url}/SHA256SUMS"

tmp=$(mktemp -d 2>/dev/null || mktemp -d -t m2p-install)
trap 'rm -rf "$tmp"' EXIT

printf 'Downloading %s\n' "$url" >&2
curl -fsSL "$url"      -o "$tmp/$asset"

printf 'Fetching SHA256SUMS\n' >&2
if ! curl -fsSL "$sums_url" -o "$tmp/SHA256SUMS"; then
  printf 'Refusing to install: could not fetch %s for verification.\n' "$sums_url" >&2
  printf 'If you trust your source, you can build from source instead:\n' >&2
  printf '  cargo install --git https://github.com/%s markdown2pdf-cli\n' "$REPO" >&2
  exit 1
fi
if [ ! -s "$tmp/SHA256SUMS" ]; then
  printf 'Refusing to install: SHA256SUMS is empty.\n' >&2
  exit 1
fi

printf 'Verifying SHA256\n' >&2
if ! ( cd "$tmp" && grep " ${asset}\$" SHA256SUMS > expected.sums && [ -s expected.sums ] && shasum -a 256 -c expected.sums >/dev/null ); then
  printf 'Checksum verification failed for %s. Aborting install.\n' "$asset" >&2
  exit 1
fi

printf 'Extracting\n' >&2
case "$ext" in
  tar.gz)
    tar xzf "$tmp/$asset" -C "$tmp"
    chmod +x "$tmp/$bin"
    ;;
  zip)
    if command -v unzip >/dev/null 2>&1; then
      ( cd "$tmp" && unzip -q "$asset" )
    elif command -v powershell.exe >/dev/null 2>&1; then
      powershell.exe -NoProfile -Command "Expand-Archive -Force -Path '$tmp\\$asset' -DestinationPath '$tmp'" >/dev/null
    else
      printf 'No unzip available. Install unzip, or use the PowerShell installer instead:\n' >&2
      printf '  powershell -c "irm https://markdown2pdf.eu/install.ps1 | iex"\n' >&2
      exit 1
    fi
    ;;
  *)
    printf 'Internal error: unknown archive format %s\n' "$ext" >&2; exit 1 ;;
esac

# Decide install dir and move the binary into place — no eval, just direct calls.
if [ ! -d "$INSTALL_DIR" ]; then
  mkdir -p "$INSTALL_DIR" 2>/dev/null || true
fi
if [ -w "$INSTALL_DIR" ]; then
  mv "$tmp/$bin" "$INSTALL_DIR/$bin"
elif [ "$NO_SUDO" -eq 1 ]; then
  INSTALL_DIR="${HOME}/.local/bin"
  mkdir -p "$INSTALL_DIR"
  mv "$tmp/$bin" "$INSTALL_DIR/$bin"
elif command -v sudo >/dev/null 2>&1; then
  sudo mv "$tmp/$bin" "$INSTALL_DIR/$bin"
else
  printf '%s is not writable and sudo is unavailable. Re-run with --to ~/.local/bin or --no-sudo.\n' "$INSTALL_DIR" >&2
  exit 1
fi

printf '\nInstalled %s/%s\n' "$INSTALL_DIR" "$bin"
"$INSTALL_DIR/$bin" --version || true

case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *) printf '\nNote: %s is not on your PATH. Add it to your shell rc, or run %s/markdown2pdf directly.\n' "$INSTALL_DIR" "$INSTALL_DIR" >&2 ;;
esac
