# markdown2pdf — Claude skill

A drop-in [Claude skill](https://docs.claude.com/en/docs/claude-code/skills) so any Claude (Code, the API, claude.ai with skills enabled) can convert Markdown to PDF on request.

When you ask Claude *"convert this README to PDF"*, *"make a PDF report from these notes"*, or similar, it triggers this skill, runs the CLI, and tells you where the output landed.

**You do not need to install the CLI separately.** The skill auto-fetches the latest `markdown2pdf` binary on first use, caches it under `~/.cache/markdown2pdf/`, and reuses it for subsequent conversions. No sudo, no PATH change, no manual download. (If you already have `markdown2pdf` on your PATH, the skill uses that one and skips the download.)

## Install the skill

### Claude Code — user-level (available in every project)

```sh
mkdir -p ~/.claude/skills
cd ~/.claude/skills
curl -L https://github.com/woodyjon/markdown2pdf/archive/refs/heads/main.tar.gz \
  | tar xz --strip-components=2 markdown2pdf-main/skills/markdown2pdf
```

Or, if you cloned the repo:

```sh
ln -s "$(pwd)/markdown2pdf/skills/markdown2pdf" ~/.claude/skills/markdown2pdf
```

### Claude Code — project-level (only this repo)

```sh
mkdir -p .claude/skills
cp -R path/to/markdown2pdf/skills/markdown2pdf .claude/skills/
```

### Anthropic API / Claude Agent SDK

Upload the skill folder via the [Skills API](https://docs.claude.com/en/api/agent-skills). The folder follows the standard `SKILL.md` + supporting files format.

## Verify

In Claude Code, ask:

> Make a PDF of this README

The skill should trigger automatically. On first use you'll see a one-time message like *"markdown2pdf: fetching latest release…"* (~10 MB download), then it produces the PDF. Subsequent runs skip the download — they use the cached binary in `~/.cache/markdown2pdf/`.

## Permanent install (optional)

If you want `markdown2pdf` available outside the skill (in your own shell scripts, terminal, etc.), install it once system-wide.

**macOS / Linux / WSL:**

```sh
curl -fsSL https://markdown2pdf.eu/install.sh | sh
```

Drops the binary in `/usr/local/bin/` (or `~/.local/bin/` if it can't write there without sudo).

**Windows (native PowerShell):**

```powershell
powershell -c "irm https://markdown2pdf.eu/install.ps1 | iex"
```

Installs to `%LOCALAPPDATA%\Programs\markdown2pdf\` and adds it to your user PATH. Open a new shell after install for the PATH change to take effect.

After either install, the skill detects the binary on `PATH` and skips the cache.

## Updating

The skill always downloads the *latest* GitHub Release into the cache. To force a re-fetch (e.g., after a new release lands and you want it now), delete the cache:

```sh
rm -rf ~/.cache/markdown2pdf
```

Next conversion re-fetches.

## License

MIT — same as the parent project. See [LICENSE](https://github.com/woodyjon/markdown2pdf/blob/main/LICENSE).
