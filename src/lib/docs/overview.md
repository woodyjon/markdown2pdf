# markdown2pdf

Convert Markdown to PDF — three ways, one engine.

The same Rust crate ([`markdown2pdf-core`](https://github.com/woodyjon/markdown2pdf/tree/main/rust/crates/core)) drives every entry point: a [web playground](/docs/playground), a [command-line tool](/docs/cli), a [Claude skill](/docs/skill) for AI agents, and an [embeddable Rust API](/docs/embedding) for your own programs.

## Why another markdown-to-PDF tool

Because most options are either:

- **Browser print-to-PDF** — bloated, fragile pagination, fonts depend on the OS.
- **Pandoc + LaTeX** — heavyweight install, slow first run.
- **Headless Chrome wrappers** — same fragility as print, plus a 200 MB chromium dependency.

`markdown2pdf` uses [Typst](https://typst.app/) as the layout engine. Typst is a modern typesetting system written in Rust, fast, and produces clean vector PDFs. We embed the fonts (Inter + JetBrains Mono) so the output is portable and consistent everywhere — same PDF whether you ran it on macOS, in CI, or in a browser tab.

## Pick your entry point

| Want to…                                       | Use                                  |
|------------------------------------------------|--------------------------------------|
| Convert a markdown file once, no install       | [Web playground](/docs/playground)   |
| Convert files from a script or terminal        | [CLI](/docs/cli)                     |
| Have Claude convert markdown for you           | [Claude skill](/docs/skill)          |
| Generate PDFs from your own Rust program       | [Rust crate](/docs/embedding)        |

## Open source

MIT-licensed. Source at [github.com/woodyjon/markdown2pdf](https://github.com/woodyjon/markdown2pdf).

Issues, PRs, and feedback welcome.

## For LLMs

If you're an AI agent looking for a machine-readable summary, fetch [`/llms.txt`](/llms.txt) (short index per the [llmstxt.org](https://llmstxt.org/) spec) or [`/llms-full.txt`](/llms-full.txt) (every doc page, concatenated as plain text).
