# technical specs

## Architecture

```
┌─────────────────┐      ┌─────────────────────┐
│  Firebase CDN   │ ───▶ │  Static SvelteKit   │
│  (Hosting)      │      │  bundle in build/   │
└─────────────────┘      └──────────┬──────────┘
                                    │
                                    ▼
                         ┌─────────────────────┐
                         │  Lazy-loaded WASM   │
                         │  (markdown2pdf)     │
                         │  in user's browser  │
                         └─────────────────────┘
```

The site is a pure static bundle — no server, no database, no auth. PDF generation happens entirely in the user's browser via the WASM build of the Rust converter.

### Environments

There is currently one environment: prd, deployed at <https://markdown2pdf.eu>.

## Tech Stack

### Web app: SvelteKit (static)

- Version: latest stable (Svelte 5, SvelteKit 2)
- TypeScript enabled
- `@sveltejs/adapter-static` — every route is prerendered to HTML at build time
- No `+page.server.ts`, no `+server.ts` runtime code; the few `+server.ts` files (`/llms.txt`, `/llms-full.txt`, `/sitemap.xml`) are prerendered to static text/XML files
- Root layout sets `export const prerender = true` so the entire site is static

### Hosting: Firebase Hosting

- Free tier (10 GB storage, 360 MB/day egress) is generous for the project's needs
- No cold start (CDN serves the static bundle directly)
- HTTPS automatic
- [`firebase.json`](firebase.json) sets long-cache immutable headers for `/_app/immutable/**`, short cache for HTML and `llms*.txt`
- [`.firebaserc`](.firebaserc) holds the project ID (replace placeholder before first deploy)
- Deployment: [`./deploy.sh`](deploy.sh) → `bun run build:wasm` + `bun run build` + `firebase deploy --only hosting`

The static bundle works on any host (Cloudflare Pages, GitHub Pages, Netlify, S3+CloudFront, self-hosted nginx, …); Firebase is the chosen default for the public deployment.

### DNS

External (Gandi for `markdown2pdf.eu`) — points the apex / `www` record at Firebase Hosting per Firebase's custom-domain setup.

### PDF generation (the engine)

The Rust crate [`markdown2pdf-core`](rust/crates/core) parses markdown with [`pulldown-cmark`](https://github.com/raphlinus/pulldown-cmark), converts the event stream to [Typst](https://typst.app/) markup, then compiles via the `typst` crate (0.14) into a vector PDF.

Inter and JetBrains Mono are embedded into the binary via `include_bytes!` so the output is portable.

The same crate produces three artifacts:

- [`markdown2pdf-cli`](rust/crates/cli) — native binary (`markdown2pdf`)
- [`markdown2pdf-wasm`](rust/crates/wasm) — WASM bindings consumed by the web app
- A library API for direct embedding in other Rust programs

### WASM build

Built by [`rust/build-wasm.sh`](rust/build-wasm.sh) (or `bun run build:wasm`) and deposited in `src/lib/wasm/`:

1. `cargo build -p markdown2pdf-wasm --target wasm32-unknown-unknown --release`
2. `wasm-bindgen --target web` — generates the JS glue
3. `wasm-opt -Oz` if Binaryen is installed

The web app lazy-loads it the first time the user clicks **Download PDF**, then keeps it warm for subsequent clicks.

### Markdown rendering (live preview only)

Separate from the PDF engine — used purely for the in-browser preview pane and for rendering doc pages:

- [`markdown-it`](https://github.com/markdown-it/markdown-it) with GFM defaults
- [`highlight.js`](https://highlightjs.org/) for code-block highlighting in the on-screen preview
- [`markdown-it-task-lists`](https://github.com/revin/markdown-it-task-lists) for checkbox support

## Documentation routes

Doc content is authored as markdown in [`src/lib/docs/*.md`](src/lib/docs/) and surfaced via:

- `/docs` (index, slug `overview`)
- `/docs/playground`
- `/docs/cli`
- `/docs/skill`
- `/docs/embedding`

The same source files are also exposed for AI agents:

- `/llms.txt` — short index per [llmstxt.org](https://llmstxt.org/)
- `/llms-full.txt` — every doc page concatenated as plain text
- `/install.sh` — POSIX-shell installer (macOS/Linux/WSL/Git Bash) — detects platform, fetches the latest release, verifies SHA256
- `/install.ps1` — PowerShell installer for native Windows — same logic, installs to `%LOCALAPPDATA%\Programs\markdown2pdf\` and updates user PATH

The docs manifest ([`src/lib/docs/index.ts`](src/lib/docs/index.ts)) is the single source of truth: adding a new doc page means adding a markdown file and one line to the manifest. Sidebar nav, sitemap, llms.txt, and llms-full.txt all derive from it.

## Project Structure

```
src/
  app.html              # HTML shell
  app.css               # Global styles + markdown theme
  app.d.ts              # TypeScript declarations
  lib/
    markdown.ts          # markdown-it config (preview + docs HTML)
    pdf.ts               # Rust+WASM PDF path (lazy-loads markdown2pdf-wasm)
    wasm/                # generated — see `bun run build:wasm`
    docs/
      index.ts             # docs manifest
      *.md                 # doc page content
  routes/
    +layout.svelte       # root layout
    +layout.ts           # prerender = true
    +page.svelte         # main page: editor + preview + download
    docs/
      +layout.svelte       # docs layout (sidebar nav)
      +page.svelte         # docs landing
      [slug]/+page.svelte  # docs page renderer
    llms.txt/+server.ts        # llmstxt.org index (prerendered)
    llms-full.txt/+server.ts   # full doc bundle (prerendered)
    sitemap.xml/+server.ts     # sitemap (prerendered)
static/
  favicon.svg
  robots.txt
rust/
  Cargo.toml             # workspace
  build-wasm.sh          # cargo + wasm-bindgen → src/lib/wasm/
  crates/
    core/                # library — markdown_to_pdf(md, opts) -> Vec<u8>
    cli/                 # binary — `markdown2pdf` CLI
    wasm/                # cdylib — wasm-bindgen exports
.claude-plugin/
  marketplace.json       # repo-as-marketplace catalog (one plugin: markdown2pdf)
  plugin.json            # Claude Code plugin manifest — bundles skills/markdown2pdf/
skills/
  markdown2pdf/          # agent skill (SKILL.md + README.md) — Claude, Codex, Pi, etc.
.github/workflows/
  ci.yml                 # Rust tests + web build on push/PR
  release.yml            # cross-compile CLI binaries on tag push
```

### Database / Auth / Storage

None at present. The app has no server-side state. Earlier versions of this spec referenced Supabase as a planned dependency — that is not currently used. Re-evaluate when paid downloads (see specs-functional.md) are implemented.

## Environment Variables

None. The app is a pure static bundle with no analytics, no auth, and no API keys baked in. If future features (e.g. paid downloads) need configuration, read it via `$env/dynamic/public` so values resolve at runtime and the same `build/` can ship to multiple hosts.

## Deployment

```sh
./deploy.sh
```

Equivalent to `bun run deploy`. Runs `bun run build:wasm`, `bun run build`, then `firebase deploy --only hosting`.

CLI releases are tagged (`git tag v0.1.x && git push origin v0.1.x`); the `release.yml` workflow handles cross-compilation and publishing.
