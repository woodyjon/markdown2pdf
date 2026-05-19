# Agent skill

A drop-in [agent skill](https://docs.claude.com/en/docs/claude-code/skills) so any coding agent — Claude (Code, the API, claude.ai), Codex, Pi, or your other preferred agent — can convert Markdown to PDF on request.

The skill is self-contained: a single SKILL.md with trigger conditions, install logic, and usage examples. The agent reads it on demand and runs the CLI when the user asks for a PDF.

**The user does not need to install the CLI separately.** The skill auto-fetches the latest `markdown2pdf` binary into `~/.cache/markdown2pdf/` on first use — no sudo, no PATH change, no manual download. If `markdown2pdf` is already on the user's PATH, the skill uses that one and skips the download.

## Install the skill

### Just ask your agent (recommended)

Most modern coding agents know how to install a skill from a GitHub repo. The simplest path is to tell yours:

> Install the markdown2pdf skill from <https://github.com/woodyjon/markdown2pdf>

The agent will fetch the skill folder (`skills/markdown2pdf/`) and drop it into the right place for its runtime — `~/.claude/skills/` for Claude Code, the equivalent skills directory for Codex, Pi, and others. After that, ask it to *"convert this README to PDF"* and the skill takes over.

This works for any agent that supports skills. The skill folder follows the standard `SKILL.md` format, so it is portable across runtimes.

### Claude Code — via `/plugin`

The repo is also a [Claude Code plugin marketplace](https://code.claude.com/docs/en/plugin-marketplaces), so inside Claude Code you can install through the plugin manager:

```text
/plugin marketplace add woodyjon/markdown2pdf
/plugin install markdown2pdf@markdown2pdf
```

The first line registers the GitHub repo as a marketplace; the second installs the `markdown2pdf` plugin, which bundles this skill. Run `/plugin` on its own for the interactive manager (Discover / Installed / Marketplaces tabs), and `/plugin marketplace update` to pull a newer version later.

You can also pre-trust the marketplace and enable the plugin for a project by committing this to `.claude/settings.json`:

```json
{
  "extraKnownMarketplaces": {
    "markdown2pdf": {
      "source": { "source": "github", "repo": "woodyjon/markdown2pdf" }
    }
  },
  "enabledPlugins": {
    "markdown2pdf@markdown2pdf": true
  }
}
```

### Manual copy (any agent)

If your agent can't fetch the skill itself, drop the folder in by hand:

```sh
mkdir -p ~/.claude/skills
cd ~/.claude/skills
curl -L https://github.com/woodyjon/markdown2pdf/archive/refs/heads/main.tar.gz \
  | tar xz --strip-components=2 markdown2pdf-main/skills/markdown2pdf
```

Replace `~/.claude/skills` with whichever skills directory your agent reads from.

Or clone the whole repo and symlink:

```sh
git clone https://github.com/woodyjon/markdown2pdf.git
ln -s "$(pwd)/markdown2pdf/skills/markdown2pdf" ~/.claude/skills/markdown2pdf
```

### Project-level (Claude Code)

```sh
mkdir -p .claude/skills
cp -R path/to/markdown2pdf/skills/markdown2pdf .claude/skills/
```

### Anthropic API / Claude Agent SDK

Upload the skill folder via the [Skills API](https://docs.claude.com/en/api/agent-skills). The folder follows the standard `SKILL.md` + supporting files format.

## How it picks the right binary

When the agent triggers the skill and `markdown2pdf` isn't on the user's `PATH`, the embedded snippet in `SKILL.md`:

1. Checks `command -v markdown2pdf` — if found, uses it.
2. Checks `~/.cache/markdown2pdf/markdown2pdf` — if cached, uses it.
3. Otherwise:
   - Detects platform via `uname -s` / `uname -m`
   - Maps to a release target (`aarch64-apple-darwin`, `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`)
   - Downloads `https://github.com/woodyjon/markdown2pdf/releases/latest/download/markdown2pdf-<target>.tar.gz`
   - Verifies SHA256 against the release's `SHA256SUMS` file
   - Extracts the binary into `~/.cache/markdown2pdf/`

The `releases/latest/download/<asset>` URL always redirects to the current latest non-prerelease, so the skill never has to query the GitHub API or pin a version.

## Permanent install (optional)

If the user expects to use `markdown2pdf` outside the skill — in their own shell scripts, terminal, CI — they can install it system-wide once.

**macOS / Linux / WSL:**

```sh
curl -fsSL https://markdown2pdf.eu/install.sh | sh
```

Writes to `/usr/local/bin/` (or `~/.local/bin/` if it can't write there without sudo).

**Windows (PowerShell):**

```powershell
powershell -c "irm https://markdown2pdf.eu/install.ps1 | iex"
```

Writes to `%LOCALAPPDATA%\Programs\markdown2pdf\` and adds it to the user PATH.

Both installers run the same logic the skill uses internally, but install to a permanent location instead of the per-user cache. After this, both the shell and the skill find the binary on `PATH` and the cache fallback is never used.

## What it does

When the user asks for a PDF — *"convert this README to PDF"*, *"make a PDF report from these notes"*, *"export this to PDF"* — the agent:

1. Resolves or downloads the binary (one-time `~/.cache/markdown2pdf/` setup).
2. Locates or creates the source markdown.
3. Runs `markdown2pdf input.md -o output.pdf`.
4. Reports the output path.

The skill knows the CLI flags, file formats, and common patterns (stdin/stdout piping, batch conversion, title metadata). It does not need the network after the first download, doesn't talk to the web playground, and works fully offline.

## Why a skill (and not just a tool call)

Skills let the agent pick up the capability without you wiring it into your prompt every time. Once installed, the agent triggers the skill whenever a markdown→PDF request comes up — across projects, across sessions. You don't need to remember the install command, the binary location, or the CLI flags.
