---
name: markdown2pdf
description: Convert Markdown files (or arbitrary markdown text) into clean, formatted PDF documents using the `markdown2pdf` CLI. Trigger when the user wants to turn a `.md` file into a `.pdf`, asks to "export this markdown as PDF", "make a PDF from these notes", "convert my README to PDF", or similar. Produces vector PDFs (searchable text, embedded fonts, GitHub-style typography). Auto-fetches the CLI on first use — no manual install needed.
license: MIT
metadata:
  homepage: https://markdown2pdf.eu
  repository: https://github.com/woodyjon/markdown2pdf
  version: "0.1.0"
---

# markdown2pdf

Convert Markdown to PDF via the `markdown2pdf` CLI. Vector text, embedded Inter + JetBrains Mono fonts, GitHub-style typography. The skill handles fetching the binary on first use — the user does not need to install anything manually.

## When to use this skill

Trigger on requests like:

- "Convert this README to PDF"
- "Make a PDF report from these notes"
- "Export this markdown as a PDF"
- "I need a PDF of `<file.md>`"
- "Generate a printable version of this document"

Do **not** trigger for:

- HTML → PDF (use a print/headless-browser tool instead)
- Filling existing PDF forms (use the `pdf` skill)
- Combining/splitting PDFs (use the `pdf` skill)
- Making a PDF from a Word doc, slide deck, or spreadsheet

## Step 1 — resolve the binary (auto-fetch if missing)

Pick the snippet matching your shell. After it runs, `$M2P` (bash) or `$Env:M2P` (PowerShell) points at a usable binary. The latest release is downloaded into a cache directory on first use; subsequent runs reuse it. **No sudo, no PATH change, no user prompt** — the user invoked the skill, so the implicit consent is to make it work.

### macOS / Linux / WSL / Git Bash (bash or sh)

```sh
ensure_markdown2pdf() {
  if M2P=$(command -v markdown2pdf 2>/dev/null) && [ -n "$M2P" ]; then
    export M2P
    return 0
  fi

  cache_dir="${XDG_CACHE_HOME:-$HOME/.cache}/markdown2pdf"

  uname_s=$(uname -s 2>/dev/null || echo unknown)
  uname_m=$(uname -m 2>/dev/null || echo unknown)
  case "${uname_s}-${uname_m}" in
    Darwin-arm64)                            target=aarch64-apple-darwin;     ext=tar.gz; bin=markdown2pdf ;;
    Darwin-x86_64)
      echo "Intel-Mac binaries are not prebuilt; build from source: cargo install --git https://github.com/woodyjon/markdown2pdf markdown2pdf-cli" >&2
      exit 1 ;;
    Linux-x86_64|Linux-amd64)                target=x86_64-unknown-linux-gnu; ext=tar.gz; bin=markdown2pdf ;;
    MINGW*-x86_64|MSYS*-x86_64|CYGWIN*-x86_64)
      target=x86_64-pc-windows-msvc;         ext=zip;     bin=markdown2pdf.exe ;;
    *)
      echo "markdown2pdf: no prebuilt binary for ${uname_s}-${uname_m}." >&2
      echo "Build from source: cargo install --git https://github.com/woodyjon/markdown2pdf markdown2pdf-cli" >&2
      return 1
      ;;
  esac

  M2P="$cache_dir/$bin"
  if [ -x "$M2P" ]; then
    export M2P
    return 0
  fi

  url="https://github.com/woodyjon/markdown2pdf/releases/latest/download/markdown2pdf-${target}.${ext}"
  sums_url="https://github.com/woodyjon/markdown2pdf/releases/latest/download/SHA256SUMS"
  asset="markdown2pdf-${target}.${ext}"

  mkdir -p "$cache_dir"
  echo "markdown2pdf: fetching latest release for ${target}..." >&2
  tmp=$(mktemp -d)
  if ! curl -fsSL "$url" -o "$tmp/$asset"; then
    echo "markdown2pdf: download failed from $url" >&2
    rm -rf "$tmp"; return 1
  fi
  if curl -fsSL "$sums_url" -o "$tmp/SHA256SUMS" 2>/dev/null; then
    ( cd "$tmp" && grep " ${asset}\$" SHA256SUMS > expected.sums && shasum -a 256 -c expected.sums >/dev/null ) \
      || { echo "markdown2pdf: SHA256 mismatch — refusing to install" >&2; rm -rf "$tmp"; return 1; }
  fi
  case "$ext" in
    tar.gz)
      tar xzf "$tmp/$asset" -C "$cache_dir"
      ;;
    zip)
      if command -v unzip >/dev/null 2>&1; then
        ( cd "$cache_dir" && unzip -qo "$tmp/$asset" )
      elif command -v powershell.exe >/dev/null 2>&1; then
        powershell.exe -NoProfile -Command "Expand-Archive -Force -Path '$tmp\\$asset' -DestinationPath '$cache_dir'" >/dev/null
      else
        echo "markdown2pdf: no unzip available; install unzip or use the PowerShell snippet instead." >&2
        rm -rf "$tmp"; return 1
      fi
      ;;
  esac
  rm -rf "$tmp"
  chmod +x "$M2P" 2>/dev/null || true
  export M2P
  echo "markdown2pdf: cached at $M2P" >&2
}
ensure_markdown2pdf || exit 1
```

After this snippet runs, all subsequent commands in the SAME bash invocation can call `"$M2P"`. Bash invocations are independent (each tool call is a fresh shell), so re-run the snippet each time, OR chain it inline with `&&` in the same Bash call as your conversion.

After first run, the snippet is fast: it hits the `command -v` check or the cached binary and skips the download.

### Windows (native PowerShell, when bash isn't available)

If you're invoking the binary through PowerShell directly (no Git Bash/WSL), use this snippet instead. It exports `$Env:M2P` to a usable `.exe`.

```powershell
function Ensure-Markdown2pdf {
  $existing = Get-Command markdown2pdf -ErrorAction SilentlyContinue
  if ($existing) { $script:M2P = $existing.Source; $env:M2P = $script:M2P; return }

  $cacheDir = Join-Path $env:LOCALAPPDATA 'markdown2pdf\cache'
  $exe      = Join-Path $cacheDir 'markdown2pdf.exe'
  if (Test-Path $exe) { $script:M2P = $exe; $env:M2P = $exe; return }

  if (-not [Environment]::Is64BitOperatingSystem -or $env:PROCESSOR_ARCHITECTURE -eq 'ARM64') {
    throw "markdown2pdf: no prebuilt binary for Windows-$($env:PROCESSOR_ARCHITECTURE)"
  }

  $target  = 'x86_64-pc-windows-msvc'
  $asset   = "markdown2pdf-$target.zip"
  $baseUrl = 'https://github.com/woodyjon/markdown2pdf/releases/latest/download'
  $url     = "$baseUrl/$asset"

  Write-Host "markdown2pdf: fetching latest release for $target..." -ForegroundColor DarkGray
  New-Item -ItemType Directory -Force -Path $cacheDir | Out-Null
  $tmp = Join-Path $env:TEMP "m2p-$([guid]::NewGuid().ToString('N'))"
  New-Item -ItemType Directory -Path $tmp | Out-Null
  try {
    $zip = Join-Path $tmp $asset
    Invoke-WebRequest -Uri $url -OutFile $zip -UseBasicParsing

    try {
      $sumsPath = Join-Path $tmp 'SHA256SUMS'
      Invoke-WebRequest -Uri "$baseUrl/SHA256SUMS" -OutFile $sumsPath -UseBasicParsing -ErrorAction Stop
      $line = Get-Content $sumsPath | Where-Object { $_ -match " $([regex]::Escape($asset))$" } | Select-Object -First 1
      if ($line) {
        $expected = ($line -split '\s+')[0].ToLower()
        $actual   = (Get-FileHash $zip -Algorithm SHA256).Hash.ToLower()
        if ($expected -ne $actual) { throw "SHA256 mismatch" }
      }
    } catch { Write-Warning "SHA256 verification skipped: $_" }

    Expand-Archive -Path $zip -DestinationPath $tmp -Force
    Move-Item -Force (Join-Path $tmp 'markdown2pdf.exe') $exe
  } finally {
    Remove-Item -Recurse -Force -ErrorAction SilentlyContinue $tmp
  }

  $script:M2P = $exe
  $env:M2P    = $exe
  Write-Host "markdown2pdf: cached at $exe" -ForegroundColor DarkGray
}
Ensure-Markdown2pdf
```

After it runs, call the binary as `& $env:M2P file.md -o out.pdf` (or `& $M2P ...` within the same script).

## Step 2 — convert

`$M2P` (bash) / `$env:M2P` (PowerShell) is the resolved binary path. Use it as you would `markdown2pdf`. The examples below are bash; for PowerShell, replace `"$M2P"` with `& $env:M2P`.

```text
$M2P [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Input file. Reads stdin if omitted.

Options:
  -o, --output <OUTPUT>  Output PDF path. Writes to stdout if omitted.
  -t, --title <TITLE>    Optional title metadata for the PDF.
  -h, --help             Print help.
  -V, --version          Print version.
```

### File → file (most common)

```sh
"$M2P" README.md -o README.pdf
```

### Pipe stdin → file

```sh
cat notes.md | "$M2P" -o notes.pdf
```

### Stdin → stdout

```sh
echo "# Hello" | "$M2P" > hello.pdf
```

### With PDF metadata title

```sh
"$M2P" -t "Quarterly Report" report.md -o report.pdf
```

### Markdown content directly from the conversation

If the user gives you markdown content inline (no file), write it to a temp file first:

```sh
TMP=$(mktemp -t md2pdf.XXXX.md)
cat > "$TMP" <<'EOF'
# Their content goes here
...
EOF
"$M2P" "$TMP" -o output.pdf
rm "$TMP"
```

### Batch convert all `.md` in a directory

```sh
for f in *.md; do
  "$M2P" "$f" -o "${f%.md}.pdf"
done
```

## Putting it together

A complete one-shot Bash invocation looks like:

```sh
ensure_markdown2pdf() { ...as above... }
ensure_markdown2pdf || exit 1
"$M2P" README.md -o README.pdf
ls -lh README.pdf
```

For brevity in agent transcripts, keep the `ensure_markdown2pdf` block in a variable or paste it once at the start of each Bash call that needs the binary.

## After running

- Confirm the output path to the user (`ls -lh output.pdf` to show file size).
- If the user asked to "open" the result on macOS: `open output.pdf`.
- On Linux: `xdg-open output.pdf`.

## Suggesting a permanent install (optional)

If the user expects to use `markdown2pdf` outside the skill (in their own scripts/terminal), suggest installing it system-wide once.

**macOS / Linux / WSL:**

```sh
curl -fsSL https://markdown2pdf.eu/install.sh | sh
```

Installs to `/usr/local/bin/` (or `~/.local/bin/` if no sudo).

**Windows (native PowerShell):**

```powershell
powershell -c "irm https://markdown2pdf.eu/install.ps1 | iex"
```

Installs to `%LOCALAPPDATA%\Programs\markdown2pdf\` and adds it to the user PATH.

After either install, both the user's shell and this skill's `command -v markdown2pdf` (or `Get-Command`) lookup will find it directly — the cache fallback is never used.

Don't push this on every conversion; mention it once when the user has run a few PDFs and would clearly benefit.

## Markdown features supported

GitHub-flavored markdown:

- Headings (h1–h6)
- Bold, italic, strikethrough
- Links. **Images are not yet rendered**: `![alt](url "title")` emits the title in italics (or the literal word *image* if no title), with the alt text currently ignored.
- Inline code and fenced code blocks (syntax-highlighted by Typst)
- Tables with headers
- Ordered, unordered, and task lists (`- [x]`, `- [ ]`)
- Blockquotes
- Horizontal rules

## What you get

- A4 paper, sensible margins
- Vector text — searchable and copy-pasteable
- Inter (body) and JetBrains Mono (code), embedded in the binary
- Same renderer as the [web playground](https://markdown2pdf.eu)

## Troubleshooting

- **"typst compile error: ..."** — usually means the markdown has a construct the converter doesn't fully handle yet (rare). Show the error to the user; suggest opening an issue at https://github.com/woodyjon/markdown2pdf/issues with the offending markdown.
- **"no prebuilt binary for <plat>"** — prebuilt targets are macOS arm64 (Apple Silicon), Linux x86_64, and Windows x86_64. For Intel Macs and anything else (Linux ARM64, FreeBSD, etc.) tell the user to build from source: `cargo install --git https://github.com/woodyjon/markdown2pdf markdown2pdf-cli`.
- **Windows + Git Bash, no `unzip` and no `powershell.exe`** — extremely rare. Tell the user to use the PowerShell snippet instead, or install via `irm https://markdown2pdf.eu/install.ps1 | iex`.
- **`curl` not on PATH** — fall back to `wget -qO- "$url"` if available.
- **SHA256 mismatch** — do NOT auto-retry; report the mismatch and stop. The release artifact may be corrupted in transit, or the SHA256SUMS file is out of sync.
- **Empty PDF / very small file** — confirm the input markdown isn't empty (`wc -l input.md`).
- **Unicode rendering issues for unusual scripts** — the embedded fonts cover Latin, basic punctuation, and common code-block characters. Exotic glyphs may render as `□`. There is no per-file font override yet.
