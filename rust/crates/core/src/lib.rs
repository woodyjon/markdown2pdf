//! Convert Markdown to PDF using typst as the layout engine.
//!
//! Public entry point: [`markdown_to_pdf`].

mod convert;
mod world;

pub use convert::markdown_to_typst;

use ::ecow::EcoVec;
use typst::diag::SourceDiagnostic;
use typst::WorldExt;

#[derive(Debug, Default, Clone)]
pub struct Options {
    /// Optional document title (used as PDF metadata).
    pub title: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    Compile(Vec<String>),
    Export(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Compile(msgs) => {
                writeln!(f, "typst compile error:")?;
                for m in msgs {
                    writeln!(f, "  - {m}")?;
                }
                Ok(())
            }
            Error::Export(s) => write!(f, "typst pdf export error: {s}"),
        }
    }
}

impl std::error::Error for Error {}

/// Convert a markdown source string to a PDF byte vector.
pub fn markdown_to_pdf(markdown: &str, opts: &Options) -> Result<Vec<u8>, Error> {
    let typst_source = convert::markdown_to_typst(markdown);
    typst_source_to_pdf(&typst_source, opts)
}

/// Compile a typst source string directly to PDF (skips the markdown stage).
pub fn typst_source_to_pdf(typst_source: &str, opts: &Options) -> Result<Vec<u8>, Error> {
    let mut source = String::new();
    if let Some(title) = &opts.title {
        source.push_str(&format!(
            "#set document(title: \"{}\")\n",
            escape_typst_string(title)
        ));
    }
    source.push_str(typst_source);

    let world = world::EmbeddedWorld::new(source);

    let result = typst::compile::<typst::layout::PagedDocument>(&world);
    let document = match result.output {
        Ok(doc) => doc,
        Err(errors) => {
            return Err(Error::Compile(format_diagnostics(&errors, &world)));
        }
    };

    let pdf_options = typst_pdf::PdfOptions::default();

    typst_pdf::pdf(&document, &pdf_options)
        .map_err(|errors| Error::Export(format_diagnostics(&errors, &world).join("; ")))
}

fn escape_typst_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}

fn format_diagnostics(diags: &EcoVec<SourceDiagnostic>, world: &dyn typst::World) -> Vec<String> {
    diags
        .iter()
        .map(|d| {
            let mut s = String::new();
            if let Some(span) = world.range(d.span) {
                s.push_str(&format!("{:?}: ", span));
            }
            s.push_str(&d.message);
            for hint in &d.hints {
                s.push_str(" (hint: ");
                s.push_str(hint);
                s.push(')');
            }
            s
        })
        .collect()
}
