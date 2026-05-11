# Embed in Rust

The PDF generator is a small Rust crate, [`markdown2pdf-core`](https://github.com/woodyjon/markdown2pdf/tree/main/rust/crates/core). The CLI and the WASM module both wrap it. Use it directly in your own Rust project to skip the binary entirely.

## Add the dependency

Until the crate is on crates.io, depend on it via git:

```toml
[dependencies]
markdown2pdf-core = { git = "https://github.com/woodyjon/markdown2pdf", package = "markdown2pdf-core" }
```

## Convert markdown to PDF bytes

```rust
use markdown2pdf_core::{markdown_to_pdf, Options};

fn main() -> anyhow::Result<()> {
    let markdown = std::fs::read_to_string("README.md")?;
    let opts = Options {
        title: Some("My Document".into()),
    };
    let pdf_bytes: Vec<u8> = markdown_to_pdf(&markdown, &opts)
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    std::fs::write("README.pdf", pdf_bytes)?;
    Ok(())
}
```

That's the entire API. The function returns a `Vec<u8>` containing the rendered PDF — write it to a file, stream it over HTTP, attach it to an email, anything.

## Inspect the intermediate Typst source

Useful for debugging conversion issues:

```rust
use markdown2pdf_core::markdown_to_typst;

let typst_source: String = markdown_to_typst("# Hello\n\nWorld");
println!("{}", typst_source);
```

## Use Typst directly

If you want to skip the markdown step and feed Typst markup directly:

```rust
use markdown2pdf_core::{typst_source_to_pdf, Options};

let typst = r#"
= My Heading

Some *bold* text in Typst.
"#;
let pdf = typst_source_to_pdf(typst, &Options::default())
    .map_err(|e| anyhow::anyhow!("{e}"))?;
```

## Embedded fonts

[Inter](https://rsms.me/inter/) and [JetBrains Mono](https://www.jetbrains.com/lp/mono/) ship inside the crate via `include_bytes!`. The compiled binary is ~20 MB but has zero runtime dependencies — copy it anywhere and it works.

## Build for WebAssembly

The same crate compiles to WASM via [`markdown2pdf-wasm`](https://github.com/woodyjon/markdown2pdf/tree/main/rust/crates/wasm). The build script `rust/build-wasm.sh` runs `cargo build --target wasm32-unknown-unknown`, then `wasm-bindgen --target web`, then `wasm-opt -Oz` if available. Output lands in `src/lib/wasm/`.

The WASM exports a single function: `markdown_to_pdf(md: string) -> Uint8Array`.
