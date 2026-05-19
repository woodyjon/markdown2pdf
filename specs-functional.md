# Markdown2pdf

## Functional Specifications

### Overview

A lightweight project that converts Markdown text into downloadable PDF documents. Three entry points share one Rust engine:

1. **Web playground** at <https://markdown2pdf.eu> — paste, preview, download.
2. **Command-line tool** (`markdown2pdf`) — single binary for use in scripts and terminals.
3. **Agent skill** at [`skills/markdown2pdf/`](skills/markdown2pdf/) — drop-in skill so any agent that supports skills (Claude, Codex, Pi, etc.) can convert markdown to PDF on request.

The Rust crate [`markdown2pdf-core`](rust/crates/core) is also available for embedding in other Rust programs.

### Core Feature: Markdown to PDF Conversion (web playground)

#### Editor (Left Pane / Top on Mobile)

- A full-height textarea where users type or paste Markdown
- Monospace font for comfortable editing
- Placeholder text with instructions
- "Clear" button to reset the editor

#### Preview (Right Pane / Bottom on Mobile)

- Real-time rendered preview of the Markdown content
- Styled to match the final PDF output (GitHub-flavored Markdown)
- Scrollable, with a paper-like card appearance
- Updates live as the user types (no delay)

#### Supported Markdown Features

- Headings (h1-h6) with visual hierarchy
- **Bold**, *italic*, ~~strikethrough~~
- Links (auto-linked URLs and explicit links). Intra-document links such as `[text](#heading-slug)` become clickable jumps to the matching heading inside the PDF; the heading slug follows the conventional lowercase-with-hyphens form (e.g. `## Vue d'ensemble` → `#vue-densemble`).
- Blockquotes
- Ordered and unordered lists
- Task lists (checkboxes)
- Tables with headers and striped rows
- Inline code and fenced code blocks with syntax highlighting
- Horizontal rules
- Images (inline, by URL)

#### PDF Download

- "Download PDF" button in the header
- User can set a custom filename (default: "document")
- A4 format with proper margins
- Loading spinner while generating
- Button is disabled when the editor is empty

### Layout (web playground)

- **Header**: App logo, title with tagline, "Docs" link, GitHub link, filename input, and download button. Tagline hidden on mobile; the text "Docs" link collapses to icon-only on mobile.
- **Main area**: Split into two equal panes (side by side on desktop, stacked on mobile)
- Responsive breakpoint at 768px

### Default Content & Onboarding

- The editor loads with a welcome Markdown document that explains how to use the app and showcases supported formatting
- Users are instructed to clear the content and start their own document

### Documentation

- A `/docs` section accessible from the playground header.
- Pages: Overview, Web playground, CLI, Agent skill, Embed in Rust.
- Sidebar navigation (collapsible on mobile). Doc content renders with the same `markdown-body` style as the playground preview.
- Doc source lives in [`src/lib/docs/*.md`](src/lib/docs/) — single source of truth for the docs site, the LLM endpoints, and the README cross-links.

### LLM-friendly endpoints

For AI agents that want a machine-readable view of the project:

- `/llms.txt` — short index following the [llmstxt.org](https://llmstxt.org/) spec, listing each doc page with its tagline.
- `/llms-full.txt` — every doc page concatenated as plain text, with absolute source URLs as HTML comments.

Both are prerendered (`+server.ts` with `export const prerender = true`) and ship as static text files.

### Agent skill

A self-contained skill at [`skills/markdown2pdf/`](skills/markdown2pdf/), in the standard `SKILL.md` + `README.md` format supported by Claude, Codex, Pi, and other agents:

- Triggers on natural-language requests like "convert this README to PDF", "make a PDF report from these notes".
- Calls the `markdown2pdf` CLI as an external command (the binary must be installed separately).
- Tells the user how to install the CLI if missing.
- Handles file → file, stdin/stdout, inline markdown, and batch conversion.

The recommended install path is to simply ask the user's agent to install the skill from the GitHub repo — most agents know how to fetch a skill from a repository URL and place it in their own skills directory. The skill is also portable as a folder: copy it into `~/.claude/skills/`, into a project's `.claude/skills/`, or upload via the Anthropic Skills API.

The repo also ships a `.claude-plugin/` manifest (`marketplace.json` + `plugin.json`) so the repo doubles as a single-plugin Claude Code marketplace. Users on Claude Code can install with `/plugin marketplace add woodyjon/markdown2pdf` followed by `/plugin install markdown2pdf@markdown2pdf` instead of copying files by hand.

### CLI

`markdown2pdf` is a single self-contained binary. Usage:

```text
markdown2pdf [OPTIONS] [INPUT]
  -o, --output <OUTPUT>   Output PDF path. Writes to stdout if omitted.
  -t, --title  <TITLE>    Optional PDF metadata title.
```

Reads from stdin if no input file is given. Writes to stdout if no `-o` is given. Cross-compiled for macOS arm64 (Apple Silicon), Linux x86_64, and Windows x86_64. Released via GitHub Releases on every git tag (`v*`), with a `SHA256SUMS` file alongside the archives. Intel-Mac users build from source.

### SEO & Social Sharing

- Page title: "Markdown to PDF — Online Converter"
- Canonical URLs set per page
- Open Graph and Twitter Card meta tags for link previews on the playground
- Theme color set to brand blue (#2563eb)
- `robots.txt` allowing all crawlers
- `sitemap.xml` generated dynamically from the docs manifest

### Analytics

- None. The site loads no analytics, no tracking pixels, and sets no cookies. Markdown stays in the browser; the only network requests after page load are for the static assets / WASM blob.

### Open source

- MIT-licensed
- Public repo at <https://github.com/woodyjon/markdown2pdf>
- Contributing notes in the README

### Future Features (Planned)

- **Paid downloads**: Users will pay 0.50 EUR per full PDF download. The preview remains free. This will require auth and a payment gateway (e.g., Stripe). The download action is isolated to [`src/lib/pdf.ts`](src/lib/pdf.ts) so it can be gated behind authentication/payment later. Adding this back will require reintroducing a server runtime (or a serverless function) — not possible on a fully-static deployment.
- **Image rendering**: images (`![alt](url "title")`) are not yet rendered in the PDF — the renderer emits the *title* in italics if present, otherwise the literal word *image* (the Markdown alt text is dropped). Adding real image support requires extending the Typst `World` to provide bytes for local files (CLI) and inline data URIs (WASM playground).
- **Per-document font override** in the CLI/Rust API.
