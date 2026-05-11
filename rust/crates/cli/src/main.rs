//! CLI: read markdown, write PDF.

use std::io::{Read, Write};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "markdown2pdf", version, about = "Convert markdown to PDF.")]
struct Args {
    /// Input file. Reads stdin if omitted.
    input: Option<PathBuf>,

    /// Output PDF path. Writes to stdout if omitted.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Optional title metadata for the PDF.
    #[arg(short, long)]
    title: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let markdown = match args.input {
        Some(path) => std::fs::read_to_string(path)?,
        None => {
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s)?;
            s
        }
    };

    let opts = markdown2pdf_core::Options { title: args.title };
    let bytes =
        markdown2pdf_core::markdown_to_pdf(&markdown, &opts).map_err(|e| anyhow::anyhow!("{e}"))?;

    match args.output {
        Some(path) => std::fs::write(path, &bytes)?,
        None => std::io::stdout().write_all(&bytes)?,
    }
    Ok(())
}
