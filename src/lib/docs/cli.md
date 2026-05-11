# Command-line tool

`markdown2pdf` is a single self-contained binary. No fonts, no runtime dependencies — just one file you can copy anywhere.

## Install

### One-line installer (recommended)

**macOS / Linux / WSL:**

```sh
curl -fsSL https://markdown2pdf.eu/install.sh | sh
```

Detects your OS/arch (`uname`), downloads the matching archive from the latest [GitHub Release](https://github.com/woodyjon/markdown2pdf/releases), verifies its SHA256, and drops the `markdown2pdf` binary into `/usr/local/bin/` (falls back to `~/.local/bin/` if it can't write there without sudo).

Flags:

```sh
curl -fsSL https://markdown2pdf.eu/install.sh | sh -s -- --to ~/.local/bin
curl -fsSL https://markdown2pdf.eu/install.sh | sh -s -- --version v0.1.1
curl -fsSL https://markdown2pdf.eu/install.sh | sh -s -- --no-sudo
```

**Windows (PowerShell):**

```powershell
powershell -c "irm https://markdown2pdf.eu/install.ps1 | iex"
```

Downloads `markdown2pdf-x86_64-pc-windows-msvc.zip` from the latest release, verifies SHA256, extracts `markdown2pdf.exe` into `%LOCALAPPDATA%\Programs\markdown2pdf\`, and adds that directory to your user PATH.

Optional environment variables (set before piping to `iex`):

```powershell
$env:M2P_INSTALL_DIR = "$HOME\.local\bin"
$env:M2P_VERSION     = "v0.1.1"
$env:M2P_NO_PATH     = "1"   # skip the PATH update
irm https://markdown2pdf.eu/install.ps1 | iex
```

### Manual download

If you'd rather not pipe the script: grab the right archive from the [GitHub Releases](https://github.com/woodyjon/markdown2pdf/releases) page:

- `markdown2pdf-aarch64-apple-darwin.tar.gz` — macOS Apple Silicon
- `markdown2pdf-x86_64-unknown-linux-gnu.tar.gz` — Linux x86_64
- `markdown2pdf-x86_64-pc-windows-msvc.zip` — Windows x86_64

Intel Macs aren't covered by a prebuilt binary (GitHub-hosted Intel-Mac runners are too unreliable to release against). Build from source instead: `cargo install --git https://github.com/woodyjon/markdown2pdf markdown2pdf-cli`.

Extract the archive and put the binary somewhere on your `PATH`:

```sh
# macOS / Linux
tar xzf markdown2pdf-*.tar.gz
sudo mv markdown2pdf /usr/local/bin/
```

Each release also publishes `SHA256SUMS` so you can verify the download:

```sh
sha256sum -c SHA256SUMS --ignore-missing
```

### Build from source

Requires a Rust toolchain (1.89+):

```sh
cargo install --git https://github.com/woodyjon/markdown2pdf markdown2pdf-cli
```

The binary lands in `~/.cargo/bin/markdown2pdf`.

### Via the Claude skill

If you only want PDFs through Claude, install the [Claude skill](/docs/skill) instead — it auto-fetches the binary into `~/.cache/markdown2pdf/` on first use. No manual install step.

## Usage

```text
markdown2pdf [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Input file. Reads stdin if omitted.

Options:
  -o, --output <OUTPUT>  Output PDF path. Writes to stdout if omitted.
  -t, --title <TITLE>    Optional title metadata for the PDF.
  -h, --help             Print help.
  -V, --version          Print version.
```

## Examples

```sh
# File → file
markdown2pdf README.md -o README.pdf

# Pipe in, pipe out
cat notes.md | markdown2pdf > notes.pdf

# Set PDF metadata title
markdown2pdf -t "Q1 Report" report.md -o report.pdf

# Convert all .md files in a directory
for f in *.md; do markdown2pdf "$f" -o "${f%.md}.pdf"; done
```

## What you get

PDFs come out with:

- A4 paper size, sensible margins
- Vector text (searchable, copy-pasteable, no rasterization)
- [Inter](https://rsms.me/inter/) for body, [JetBrains Mono](https://www.jetbrains.com/lp/mono/) for code — both embedded in the binary
- GitHub-style typography (headings, tables, lists, blockquotes)
- Syntax-highlighted code blocks via Typst's built-in highlighter

The same renderer drives the [web playground](/docs/playground) and the [Rust crate](/docs/embedding).
