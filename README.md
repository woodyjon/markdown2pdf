# markdown2pdf

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Convert Markdown to PDF — three ways, one engine.

- **Web playground**: <https://markdown2pdf.eu>
- **CLI**: single binary, `markdown2pdf in.md -o out.pdf`
- **Claude skill** (or any other ai agent): drop-in skill so your agent can do it for you ([`skills/markdown2pdf/`](skills/markdown2pdf/))
- **Rust crate**: embed the converter in your own program ([`rust/crates/core`](rust/crates/core))

The same Rust code drives every entry point. Typst as the layout engine, Inter + JetBrains Mono embedded for portable vector PDFs.

---

## Quickstart

### Use it

Just go to <https://markdown2pdf.eu>. Paste markdown, click Download.

## AI agent skill

Just tell your agent to check that repo and grab the skill, then ask him to convert a markdown file to a pdf file.

### CLI

```sh
# macOS / Linux / WSL — download + install the latest CLI release
curl -fsSL https://markdown2pdf.eu/install.sh | sh

# Windows (PowerShell) — same, for Windows x86_64
powershell -c "irm https://markdown2pdf.eu/install.ps1 | iex"
```

Both installers detect your OS/arch, download the matching archive from the latest [GitHub Release](https://github.com/woodyjon/markdown2pdf/releases), verify SHA256, and install the binary to a sensible location (`/usr/local/bin/` on Unix, `%LOCALAPPDATA%\Programs\markdown2pdf\` on Windows). Or grab the archive manually from Releases. Or build from source:

```sh
# build the CLI from source and install it into ~/.cargo/bin
cargo install --git https://github.com/woodyjon/markdown2pdf markdown2pdf-cli
```

Then:

```sh
# convert a file to PDF (filename argument + explicit output path)
markdown2pdf README.md -o README.pdf

# read markdown from stdin, write PDF to stdout (pipe-friendly)
cat notes.md | markdown2pdf > notes.pdf

# set the PDF document title via -t (shows in the PDF reader's title bar)
markdown2pdf -t "Q1 Report" report.md -o report.pdf
```

Full CLI docs: [`/docs/cli`](https://markdown2pdf.eu/docs/cli) (or [`src/lib/docs/cli.md`](src/lib/docs/cli.md)).

### Claude skill

The skill lives in [`skills/markdown2pdf/`](skills/markdown2pdf/) and follows the standard Anthropic skill format. The repo is also a [Claude Code plugin marketplace](https://code.claude.com/docs/en/plugin-marketplaces) ([`.claude-plugin/`](.claude-plugin/)), so the easiest install is the `/plugin` command.

**Recommended — install via `/plugin` (Claude Code):**

```text
/plugin marketplace add woodyjon/markdown2pdf
/plugin install markdown2pdf@markdown2pdf
```

The first command registers this repo as a marketplace; the second installs the `markdown2pdf` plugin (which bundles the skill). Run `/plugin` on its own to open the interactive plugin manager.

**Manual — copy the skill folder (any agent):**

```sh
# make sure the user-level skills dir exists, then move into it
mkdir -p ~/.claude/skills
cd ~/.claude/skills

# download the repo tarball and extract just the skill folder
# (strip-components=2 drops the `markdown2pdf-main/skills/` prefix)
curl -L https://github.com/woodyjon/markdown2pdf/archive/refs/heads/main.tar.gz \
  | tar xz --strip-components=2 markdown2pdf-main/skills/markdown2pdf
```

Either way, ask Claude: *"convert this README to PDF"*. The skill triggers, runs the CLI, and reports the output path.

**You don't need to install the CLI separately.** The skill auto-fetches the latest binary into `~/.cache/markdown2pdf/` on first use (no sudo, no PATH change). If `markdown2pdf` is already on your PATH, the skill uses that one and skips the download.

Full skill docs: [`/docs/skill`](https://markdown2pdf.eu/docs/skill) (or [`skills/markdown2pdf/SKILL.md`](skills/markdown2pdf/SKILL.md)).

### Embed in Rust

```toml
[dependencies]
markdown2pdf-core = { git = "https://github.com/woodyjon/markdown2pdf", package = "markdown2pdf-core" }
```

```rust
use markdown2pdf_core::{markdown_to_pdf, Options};

let pdf: Vec<u8> = markdown_to_pdf("# Hello\n\nWorld", &Options::default())
    .map_err(|e| anyhow::anyhow!("{e}"))?;
std::fs::write("out.pdf", pdf)?;
```

Full embed docs: [`/docs/embedding`](https://markdown2pdf.eu/docs/embedding) (or [`src/lib/docs/embedding.md`](src/lib/docs/embedding.md)).

---

## Architecture

```
rust/crates/core              ← markdown_to_pdf(md, opts) -> Vec<u8>
   ├─→ rust/crates/cli        ← markdown2pdf binary (~20 MB, no runtime deps)
   └─→ rust/crates/wasm       ← src/lib/wasm/  (~23 MB, lazy-loaded by the web app)

src/                          ← SvelteKit static site
   routes/+page.svelte         editor + preview + download (the playground)
   routes/docs/                rendered docs pages
   routes/llms.txt/            llmstxt.org index for AI agents
   routes/llms-full.txt/       full doc bundle for AI agents
```

The conversion path: `markdown` → (`pulldown-cmark` events) → `typst` markup → `typst::compile` → PDF bytes. See [`rust/crates/core/src/convert.rs`](rust/crates/core/src/convert.rs) for the converter.

The live preview pane uses `markdown-it` + `highlight.js` — separate from the PDF engine, just for instant visual feedback.

---

## Develop

### Prerequisites

- [Bun](https://bun.sh/) — for the SvelteKit frontend
- [Rust](https://rustup.rs/) (1.89+) — for building the WASM module and CLI
  - The `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
  - `wasm-bindgen-cli`: `cargo install wasm-bindgen-cli --version "^0.2" --locked`
  - *(optional)* `wasm-opt` from [Binaryen](https://github.com/WebAssembly/binaryen) to shrink the WASM further: `brew install binaryen`

### Setup

```sh
bun install         # JS deps
bun run build:wasm  # build the Rust → WASM module → src/lib/wasm/
```

`build:wasm` runs [`rust/build-wasm.sh`](rust/build-wasm.sh):

1. `cargo build -p markdown2pdf-wasm --target wasm32-unknown-unknown --release`
2. `wasm-bindgen --target web` — generates the JS glue
3. `wasm-opt -Oz` if Binaryen is installed

`src/lib/wasm/` is gitignored; every fresh clone needs `bun run build:wasm` once.

`bun install` also runs a `prepare` script that points `core.hooksPath` at [`.githooks/`](.githooks/). The [`pre-commit`](.githooks/pre-commit) hook runs `cargo fmt --check` + `cargo clippy -D warnings` whenever Rust sources are staged — the same gates CI enforces. Bypass once with `git commit --no-verify` if you ever need to.

### Dev server

```sh
bun run dev   # start the Vite dev server on http://localhost:5173
```

Hot-reload works for Svelte / TS / docs markdown. **If you change the Rust source**, run `bun run build:wasm` again — the dev server picks up the new WASM on next page load.

### Tests

```sh
bun run test:rust   # run the Rust test suite (= cargo test --workspace)
```

Covers the markdown→typst converter (headings, em/strong/strike, lists, task lists, tables, fenced code, etc.).

### Build the CLI locally

```sh
bun run build:cli              # release build of the CLI binary, ~2-3 min cold
./rust/target/release/markdown2pdf --help   # sanity-check the freshly built binary
```

The binary is fully self-contained — no font files or external dependencies needed at runtime.

### Build the static site

```sh
bun run build:wasm   # if not already up to date
bun run build        # vite build → build/  (SvelteKit + adapter-static)
bun run preview      # local preview of the production bundle
```

---

## Deploy

The site is a pure static bundle (`build/`). Drop it on any static host.

### Firebase Hosting (used by markdown2pdf.eu)

One-time setup:

1. `npm i -g firebase-tools` (or use `bunx firebase-tools` everywhere)
2. `firebase login`
3. `cp .firebaserc.example .firebaserc` (the real `.firebaserc` is gitignored so each deployer can point at their own project)
4. Edit `.firebaserc`:
   - Set `projects.default` to your Firebase project ID
   - Rename the key under `targets` to that same project ID, and set `targets.<project-id>.hosting.markdown2pdf` to `["<your-site-id>"]` (the Hosting site name within that project). The `firebase.json` hosting block already references the `markdown2pdf` target, so multi-site projects deploy to the correct site.

Verify the target resolves before the first deploy:

```sh
# list configured hosting targets — confirms .firebaserc resolves correctly
firebase target
# → markdown2pdf (<your-site-id>)
```

Deploy:

```sh
# one-shot: rebuild WASM + site, then ship to Firebase Hosting
./deploy.sh
# = bun run build:wasm + bun run build + firebase deploy --only hosting
# (also: bun run deploy)
```

[`firebase.json`](firebase.json) sets cache headers (`max-age=31536000` immutable for `/_app/immutable/**`, short cache for HTML and `llms*.txt`).

#### Continuous deploy via GitHub Actions

[`.github/workflows/ci.yml`](.github/workflows/ci.yml) has a `deploy` job that runs on every push to `main` (after the rust + web jobs pass) and ships the freshly built `build/` to Firebase Hosting via [`FirebaseExtended/action-hosting-deploy`](https://github.com/FirebaseExtended/action-hosting-deploy).

To enable it on your fork, add the following to the GitHub repo (Settings → Secrets and variables → Actions):

**Repository variables** (non-secret, hold your project/site IDs out of the public workflow file):

- `FIREBASE_PROJECT_ID` — e.g. `your-firebase-project-id`
- `FIREBASE_SITE_ID` — e.g. `your-hosting-site-id`

**Repository secret**:

- `FIREBASE_SERVICE_ACCOUNT` — the full JSON of a Google service account key with the **Firebase Hosting Admin** role on that project.

Easy way to generate the service account and store the secret automatically:

```sh
# guided setup: creates a deploy service account and stores its key as a GitHub secret
firebase init hosting:github
# Pick the same project, decline its workflow scaffolding (we already have one).
# It creates the service account, grants Hosting Admin, and uploads the JSON
# as a GitHub secret (named for the project — rename it to FIREBASE_SERVICE_ACCOUNT,
# or update the secret reference in ci.yml).
```

Manual way: in GCP IAM, create a service account with role *Firebase Hosting Admin*, generate a JSON key, paste it into a `FIREBASE_SERVICE_ACCOUNT` repo secret.

The deploy job:

1. Downloads the `build/` artifact uploaded by the `web` job.
2. Writes `.firebaserc` from the two variables (so the public workflow file never embeds your project/site IDs).
3. Deploys to the `live` channel of your hosting site.

If the variables aren't set, the job fails fast with a clear error message instead of deploying to the wrong target.

### Other static hosts

The contents of `build/` are pure HTML/JS/WASM. Any static host works:

- **Cloudflare Pages**: connect the repo, build command `bun run build:wasm && bun run build`, output `build/`
- **GitHub Pages**: build in CI, push `build/` to `gh-pages` branch
- **Netlify**: same idea — `bun run build:wasm && bun run build`, publish `build/`
- **Self-hosted**: serve `build/` with caddy / nginx / `python -m http.server`

Make sure the host serves `.wasm` with `Content-Type: application/wasm` and ideally enables Brotli/gzip — the 23 MB WASM compresses to ~7 MB.

---

## Releasing the CLI

Tagging a release triggers [`.github/workflows/release.yml`](.github/workflows/release.yml):

```sh
# create a release tag locally
git tag v0.1.1

# push the tag — this fires release.yml and publishes the GitHub Release
git push origin v0.1.1
```

The workflow cross-compiles the CLI for macOS arm64 (Apple Silicon), Linux x86_64, and Windows x86_64, and uploads the archives + a `SHA256SUMS` file to a GitHub Release. Intel-Mac users build from source (`cargo install --git https://github.com/woodyjon/markdown2pdf markdown2pdf-cli`) — GitHub-hosted Intel-Mac runners are too unreliable to release against.

---

## Project layout

```
src/
├─ app.css                   global styles (preview matches GitHub-markdown-css)
├─ app.html
├─ lib/
│  ├─ markdown.ts            markdown-it config — renders preview HTML and docs pages
│  ├─ pdf.ts                 WASM loader — drives the Download PDF button
│  ├─ wasm/                  generated — see `bun run build:wasm`
│  └─ docs/
│     ├─ index.ts            docs manifest (slug, title, source)
│     ├─ overview.md
│     ├─ playground.md
│     ├─ cli.md
│     ├─ skill.md
│     └─ embedding.md
└─ routes/
   ├─ +layout.svelte
   ├─ +layout.ts             prerender = true (everything is static)
   ├─ +page.svelte           editor + preview UI (the playground)
   ├─ docs/
   │  ├─ +layout.svelte      sidebar nav
   │  ├─ +page.svelte        docs landing
   │  └─ [slug]/+page.svelte docs page renderer
   ├─ llms.txt/+server.ts        llmstxt.org index
   ├─ llms-full.txt/+server.ts   full doc bundle
   └─ sitemap.xml/+server.ts

.claude-plugin/                 Claude Code plugin + marketplace manifests
├─ marketplace.json             repo-as-marketplace catalog (one plugin: markdown2pdf)
└─ plugin.json                  plugin manifest — bundles skills/markdown2pdf/

skills/
└─ markdown2pdf/             drop into ~/.claude/skills/ (or install via /plugin)
   ├─ SKILL.md
   └─ README.md

rust/
├─ Cargo.toml                workspace
├─ build-wasm.sh             cargo + wasm-bindgen → src/lib/wasm/
└─ crates/
   ├─ core/                  library: markdown_to_pdf(md, opts) -> Vec<u8>
   │  ├─ src/convert.rs        pulldown-cmark events → typst markup
   │  ├─ src/world.rs          minimal typst::World with embedded fonts
   │  └─ fonts/                Inter, JetBrains Mono (TTF, embedded via include_bytes!)
   ├─ cli/                   binary: `markdown2pdf` (clap + stdin/stdout)
   └─ wasm/                  cdylib: wasm-bindgen exports

.github/workflows/
├─ ci.yml                    Rust tests + web build on push/PR
└─ release.yml               cross-compile CLI on tag push

firebase.json                Hosting config
deploy.sh                    build + firebase deploy
```

---

## Contributing

Issues and PRs welcome. The repo is MIT-licensed.

A few quick notes:

- Use `bun`, not `npm`.
- Markdown converter changes go in [`rust/crates/core/src/convert.rs`](rust/crates/core/src/convert.rs); add a test in the same file.
- Doc edits go in [`src/lib/docs/*.md`](src/lib/docs) — the website, `/llms.txt`, and `/llms-full.txt` all rebuild from those sources.
- Skill changes go in [`skills/markdown2pdf/SKILL.md`](skills/markdown2pdf/SKILL.md). If you bump the skill, also bump `version` in [`.claude-plugin/plugin.json`](.claude-plugin/plugin.json) and [`.claude-plugin/marketplace.json`](.claude-plugin/marketplace.json) so `/plugin` installs pick up the change.

## Troubleshooting

**`Failed to generate PDF: ...`** in the browser — open DevTools, the WASM panic message will show in the console. Common causes: a markdown construct the converter doesn't handle yet (open an issue with the markdown that triggered it).

**Vite says "Cannot import non-asset file /wasm/..."** — you ran `bun run dev` before `bun run build:wasm`. The `src/lib/wasm/` directory is empty. Run `bun run build:wasm` then restart the dev server.

**`cargo build` is slow on the first run** — typst pulls in ~80 transitive crates. Subsequent builds use the local cache and finish in seconds.

## Specs

- [`specs-functional.md`](./specs-functional.md) — what the app does
- [`specs-technical.md`](./specs-technical.md) — how it's built

## Related projects

- [`theiskaa/markdown2pdf`](https://github.com/theiskaa/markdown2pdf) — different `markdown2pdf` Rust crate (no relation, MIT-licensed). Uses a custom PDF renderer with PDF built-in fonts, has its own TOML configuration system, and owns the `markdown2pdf` name on crates.io. Smaller binary and smaller output, but as of v0.2.2 it does not render inline bold/italic/strikethrough, drops blockquote contents, and renders task-list checkboxes incorrectly — so it suits a narrower set of inputs. To avoid the name conflict, this project ships its CLI binary (also called `markdown2pdf`) via GitHub Releases instead of crates.io. Distinct downstreams: install the right one for your needs.

## License

MIT — see [LICENSE](LICENSE).

The bundled Inter and JetBrains Mono fonts are redistributed under the SIL Open Font License 1.1 — see [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md).
