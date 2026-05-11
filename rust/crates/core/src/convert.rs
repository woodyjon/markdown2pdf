//! Convert markdown to typst markup.
//!
//! Mirrors the TypeScript implementation in `src/lib/markdown-to-typst.ts`,
//! but driven by `pulldown-cmark`'s event stream rather than markdown-it tokens.

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

const PREAMBLE: &str = r##"
#set page(paper: "a4", margin: (x: 1.8cm, y: 2cm))
#set text(
  font: ("Inter", "Helvetica", "Arial"),
  size: 11pt,
  fill: rgb("#1f2328"),
  lang: "en",
)
#set par(leading: 0.65em, justify: false, spacing: 0.9em)

#show heading.where(level: 1): it => {
  block(below: 0.4em, above: 1.4em)[#set text(weight: 600, size: 1.85em); #it.body]
  v(-0.55em)
  line(length: 100%, stroke: 0.5pt + rgb("#d1d9e0"))
  v(0.6em)
}
#show heading.where(level: 2): it => {
  block(below: 0.4em, above: 1.3em)[#set text(weight: 600, size: 1.5em); #it.body]
  v(-0.55em)
  line(length: 100%, stroke: 0.5pt + rgb("#d1d9e0"))
  v(0.6em)
}
#show heading.where(level: 3): it => block(above: 1.1em, below: 0.8em, text(weight: 600, size: 1.25em, it.body))
#show heading.where(level: 4): it => block(above: 1em, below: 0.7em, text(weight: 600, size: 1em, it.body))
#show heading.where(level: 5): it => block(above: 1em, below: 0.7em, text(weight: 600, size: 0.875em, it.body))
#show heading.where(level: 6): it => block(above: 1em, below: 0.7em, text(weight: 600, size: 0.85em, fill: rgb("#59636e"), it.body))

#show raw.where(block: false): it => box(
  fill: rgb(212, 222, 231, 92),
  inset: (x: 3pt, y: 0pt),
  outset: (y: 3pt),
  radius: 3pt,
  text(font: ("JetBrains Mono",), size: 0.88em, it.text),
)

#show raw.where(block: true): it => block(
  fill: rgb("#f6f8fa"),
  radius: 6pt,
  inset: 12pt,
  width: 100%,
  text(font: ("JetBrains Mono",), size: 0.88em, it),
)

#show link: it => text(fill: rgb("#0969da"), it)
#show strong: it => text(weight: 600, it.body)

#let gh-quote(body) = block(
  inset: (left: 12pt, top: 2pt, bottom: 2pt, right: 0pt),
  stroke: (left: 3pt + rgb("#d1d9e0")),
  text(fill: rgb("#59636e"), body),
)

#let gh-hr() = block(above: 1.5em, below: 1.5em,
  line(length: 100%, stroke: 2pt + rgb("#d1d9e0")))

#let gh-checkbox(checked) = box(
  width: 0.95em, height: 0.95em,
  baseline: 0.18em,
  fill: if checked { rgb("#0969da") } else { white },
  stroke: 0.6pt + (if checked { rgb("#0969da") } else { rgb("#59636e") }),
  radius: 2pt,
  if checked {
    align(center + horizon, text(fill: white, size: 0.7em, weight: 700)[#sym.checkmark])
  } else { [] },
)

#let gh-table(headers, ..rows) = {
  let n = headers.len()
  let cells = ()
  for h in headers {
    cells.push(table.cell(fill: rgb("#f6f8fa"), text(weight: 600, h)))
  }
  let r = 0
  for row in rows.pos() {
    let bg = if calc.odd(r) { rgb("#f6f8fa") } else { white }
    for c in row {
      cells.push(table.cell(fill: bg, c))
    }
    r += 1
  }
  table(
    columns: n,
    stroke: 0.5pt + rgb("#d1d9e0"),
    inset: (x: 9pt, y: 6pt),
    ..cells,
  )
}
"##;

/// Escape characters that would otherwise be interpreted as typst markup.
fn escape_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '*' => out.push_str("\\*"),
            '_' => out.push_str("\\_"),
            '`' => out.push_str("\\`"),
            '#' => out.push_str("\\#"),
            '$' => out.push_str("\\$"),
            '@' => out.push_str("\\@"),
            '~' => out.push_str("\\~"),
            '[' => out.push_str("\\["),
            ']' => out.push_str("\\]"),
            '<' => out.push_str("\\<"),
            '>' => out.push_str("\\>"),
            _ => out.push(ch),
        }
    }
    out
}

fn escape_string_literal(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn longest_backtick_run(s: &str) -> usize {
    let mut max = 0usize;
    let mut cur = 0usize;
    for ch in s.chars() {
        if ch == '`' {
            cur += 1;
            if cur > max {
                max = cur;
            }
        } else {
            cur = 0;
        }
    }
    max
}

#[derive(Default)]
struct ListBuf {
    ordered: bool,
    items: Vec<String>,         // Rendered item bodies.
    current_item: Option<Item>, // Open item, if any.
    has_any_task: bool,
}

#[derive(Default)]
struct Item {
    body: String,
    is_task: bool,
    checked: bool,
}

#[derive(Default)]
struct TableBuf {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    in_head: bool,
    current_row: Vec<String>,
    current_cell: Option<String>,
}

enum Frame {
    /// Top-level document body — never popped.
    Doc,
    /// Inside a paragraph; flush adds two newlines on close.
    Paragraph,
    /// Inside a heading.
    Heading,
    /// Inside a fenced or indented code block. Content is appended raw.
    CodeBlock {
        info: String,
    },
    /// Inside a blockquote — children are rendered into a sub-buffer
    /// that is wrapped in `#gh-quote[...]` on close.
    BlockQuote,
    /// Inside a list (bullet or ordered).
    List(ListBuf),
    /// Inside a list item — body accumulates, finalised on End(Item).
    Item,
    /// Inside a table — rows/cells accumulate.
    Table(TableBuf),
    /// Inside a table row.
    TableRow,
    /// Inside a table cell — content goes into the current cell buffer.
    TableCell,
    /// Inside emphasis / strong / strike — wraps inline content.
    Emph,
    Strong,
    Strike,
    /// Inside a link — wraps inline content with `#link("href")[...]`.
    Link,
}

struct Emitter {
    /// Stack of output buffers.  Always at least one (the doc body).
    bufs: Vec<String>,
    /// Stack of frames mirroring `bufs` plus inline-only frames that
    /// don't push a buffer.
    frames: Vec<Frame>,
}

impl Emitter {
    fn new() -> Self {
        Self {
            bufs: vec![String::new()],
            frames: vec![Frame::Doc],
        }
    }

    fn out_mut(&mut self) -> &mut String {
        // The top buffer is always the active one; inline frames
        // (Emph/Strong/Strike/Link) write into the same buffer they were
        // opened in. Block frames that need their own buffer push one.
        self.bufs.last_mut().unwrap()
    }

    fn push_buf(&mut self) {
        self.bufs.push(String::new());
    }

    fn pop_buf(&mut self) -> String {
        self.bufs.pop().expect("buffer underflow")
    }

    fn write(&mut self, s: &str) {
        self.out_mut().push_str(s);
    }

    fn handle(&mut self, ev: Event<'_>) {
        match ev {
            Event::Start(tag) => self.start(tag),
            Event::End(tag) => self.end(tag),
            Event::Text(t) => self.text(&t),
            Event::Code(t) => self.inline_code(&t),
            Event::SoftBreak => self.write(" "),
            Event::HardBreak => self.write(" \\ "),
            Event::Rule => self.write("\n#gh-hr()\n\n"),
            Event::TaskListMarker(checked) => {
                if let Some(Frame::Item) = self.frames.last() {
                    // The current open ListBuf::current_item is in the
                    // *parent* List frame; mark it as a task.
                    if let Some(Frame::List(list)) = self
                        .frames
                        .iter_mut()
                        .rev()
                        .find(|f| matches!(f, Frame::List(_)))
                    {
                        list.has_any_task = true;
                        if let Some(item) = list.current_item.as_mut() {
                            item.is_task = true;
                            item.checked = checked;
                        }
                    }
                }
                // Don't emit anything yet; the checkbox is rendered when
                // the list closes (so we can pick #list(marker: none)).
            }
            Event::FootnoteReference(_) | Event::Html(_) | Event::InlineHtml(_) => {
                // Drop raw HTML — we don't try to faithfully render it.
            }
            Event::InlineMath(s) | Event::DisplayMath(s) => {
                // No math support yet — render the LaTeX source as inline code.
                self.write(&format!("#raw(\"{}\")", escape_string_literal(&s)));
            }
        }
    }

    fn start(&mut self, tag: Tag<'_>) {
        match tag {
            Tag::Paragraph => self.frames.push(Frame::Paragraph),
            Tag::Heading { level, .. } => {
                let n = heading_level_to_int(level);
                self.write(&format!("\n{} ", "=".repeat(n)));
                self.frames.push(Frame::Heading);
            }
            Tag::BlockQuote(_) => {
                self.frames.push(Frame::BlockQuote);
                self.push_buf();
            }
            Tag::CodeBlock(kind) => {
                let info = match &kind {
                    CodeBlockKind::Fenced(s) => s.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                self.frames.push(Frame::CodeBlock { info });
                self.push_buf();
            }
            Tag::List(start) => {
                let ordered = start.is_some();
                self.frames.push(Frame::List(ListBuf {
                    ordered,
                    ..Default::default()
                }));
                // We don't push a buffer here; items push their own.
            }
            Tag::Item => {
                if let Some(Frame::List(list)) = self.frames.last_mut() {
                    list.current_item = Some(Item::default());
                }
                self.frames.push(Frame::Item);
                self.push_buf();
            }
            Tag::Table(_aligns) => {
                self.frames.push(Frame::Table(TableBuf::default()));
            }
            Tag::TableHead => {
                if let Some(Frame::Table(t)) = self.frames.last_mut() {
                    t.in_head = true;
                }
                self.frames.push(Frame::TableRow);
                if let Some(Frame::Table(t)) = self.frames.iter_mut().rev().nth(1) {
                    t.current_row = Vec::new();
                }
            }
            Tag::TableRow => {
                if let Some(Frame::Table(t)) = self.frames.last_mut() {
                    t.in_head = false;
                    t.current_row = Vec::new();
                }
                self.frames.push(Frame::TableRow);
            }
            Tag::TableCell => {
                if let Some(Frame::Table(t)) = self
                    .frames
                    .iter_mut()
                    .rev()
                    .find(|f| matches!(f, Frame::Table(_)))
                {
                    t.current_cell = Some(String::new());
                }
                self.frames.push(Frame::TableCell);
                self.push_buf();
            }
            Tag::Emphasis => {
                self.write("#emph[");
                self.frames.push(Frame::Emph);
            }
            Tag::Strong => {
                self.write("#strong[");
                self.frames.push(Frame::Strong);
            }
            Tag::Strikethrough => {
                self.write("#strike[");
                self.frames.push(Frame::Strike);
            }
            Tag::Link { dest_url, .. } => {
                self.write(&format!("#link(\"{}\")[", escape_string_literal(&dest_url)));
                self.frames.push(Frame::Link);
            }
            Tag::Image {
                dest_url: _, title, ..
            } => {
                // Images aren't rendered (no FS access for remote URLs in WASM).
                // Emit the image title in italics if present, otherwise the
                // literal word "image". The Markdown alt text is currently
                // dropped along with the inner Text events (see sink frame below).
                self.write(&format!(
                    "#emph[{}]",
                    escape_text(if title.is_empty() { "image" } else { &title })
                ));
                // Push a frame so the matching End consumes correctly,
                // but also push a sink buffer so inner Text events are dropped.
                self.frames.push(Frame::Emph);
                self.push_buf();
            }
            Tag::HtmlBlock
            | Tag::FootnoteDefinition(_)
            | Tag::DefinitionList
            | Tag::DefinitionListTitle
            | Tag::DefinitionListDefinition
            | Tag::MetadataBlock(_)
            | Tag::Superscript
            | Tag::Subscript => {
                // Push a sink frame & buffer so corresponding text/end events
                // don't bleed into the document.
                self.frames.push(Frame::Paragraph);
                self.push_buf();
            }
        }
    }

    fn end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => {
                let _ = self.frames.pop();
                self.write("\n\n");
            }
            TagEnd::Heading(_) => {
                let _ = self.frames.pop();
                self.write("\n\n");
            }
            TagEnd::BlockQuote(_) => {
                let inner = self.pop_buf();
                let _ = self.frames.pop();
                let trimmed = inner.trim_end_matches('\n');
                self.write(&format!("\n#gh-quote[\n{}\n]\n\n", trimmed));
            }
            TagEnd::CodeBlock => {
                let content = self.pop_buf();
                let frame = self.frames.pop();
                let info = if let Some(Frame::CodeBlock { info }) = frame {
                    info
                } else {
                    String::new()
                };
                let n = std::cmp::max(3, longest_backtick_run(&content) + 1);
                let fence: String = "`".repeat(n);
                self.write(&format!("\n{fence}{}\n", info));
                self.write(&content);
                if !content.ends_with('\n') {
                    self.write("\n");
                }
                self.write(&format!("{fence}\n\n"));
            }
            TagEnd::List(_ordered) => {
                let frame = self.frames.pop();
                let Some(Frame::List(list)) = frame else {
                    return;
                };
                self.write(&render_list(&list));
            }
            TagEnd::Item => {
                let body = self.pop_buf();
                let _ = self.frames.pop();
                if let Some(Frame::List(list)) = self.frames.last_mut() {
                    let mut item = list.current_item.take().unwrap_or_default();
                    item.body = body.trim().to_string();
                    list.items.push(format_item(item));
                }
            }
            TagEnd::Table => {
                let frame = self.frames.pop();
                let Some(Frame::Table(t)) = frame else {
                    return;
                };
                self.write(&render_table(&t));
            }
            TagEnd::TableHead | TagEnd::TableRow => {
                let _ = self.frames.pop();
                if let Some(Frame::Table(t)) = self.frames.last_mut() {
                    let row = std::mem::take(&mut t.current_row);
                    if t.in_head && t.headers.is_empty() {
                        t.headers = row;
                        t.in_head = false;
                    } else {
                        t.rows.push(row);
                    }
                }
            }
            TagEnd::TableCell => {
                let cell = self.pop_buf();
                let _ = self.frames.pop();
                if let Some(Frame::Table(t)) = self
                    .frames
                    .iter_mut()
                    .rev()
                    .find(|f| matches!(f, Frame::Table(_)))
                {
                    t.current_row.push(cell.trim().to_string());
                    t.current_cell = None;
                }
            }
            TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough => {
                let _ = self.frames.pop();
                self.write("]");
            }
            TagEnd::Link => {
                let _ = self.frames.pop();
                self.write("]");
            }
            TagEnd::Image => {
                let _ = self.pop_buf();
                let _ = self.frames.pop();
            }
            TagEnd::HtmlBlock
            | TagEnd::FootnoteDefinition
            | TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition
            | TagEnd::MetadataBlock(_)
            | TagEnd::Subscript
            | TagEnd::Superscript => {
                let _ = self.pop_buf();
                let _ = self.frames.pop();
            }
        }
    }

    fn text(&mut self, t: &str) {
        // Inside a code block, text is passed through verbatim.
        if matches!(self.frames.last(), Some(Frame::CodeBlock { .. })) {
            self.write(t);
        } else {
            let escaped = escape_text(t);
            self.write(&escaped);
        }
    }

    fn inline_code(&mut self, t: &str) {
        // Use the function form `#raw(...)` so any specials in the code are
        // passed through cleanly via the string literal.
        self.write(&format!("#raw(\"{}\")", escape_string_literal(t)));
    }

    fn finish(self) -> String {
        debug_assert_eq!(self.bufs.len(), 1, "buffer stack should unwind cleanly");
        self.bufs.into_iter().next().unwrap()
    }
}

fn heading_level_to_int(level: HeadingLevel) -> usize {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn format_item(item: Item) -> String {
    if item.is_task {
        format!("#gh-checkbox({})~{}{}", item.checked, " ", item.body)
    } else {
        item.body
    }
}

fn render_list(list: &ListBuf) -> String {
    if list.items.is_empty() {
        return String::new();
    }
    let parts: String = list.items.iter().map(|b| format!("[{}]", b)).collect();
    if list.has_any_task {
        format!(
            "\n#list(marker: none, indent: 0pt, body-indent: 0.4em){}\n\n",
            parts
        )
    } else if list.ordered {
        format!("\n#enum{}\n\n", parts)
    } else {
        format!("\n#list{}\n\n", parts)
    }
}

fn render_table(t: &TableBuf) -> String {
    if t.headers.is_empty() {
        return String::new();
    }
    let header_arr: String = format!(
        "({})",
        t.headers
            .iter()
            .map(|c| format!("[{}]", c))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let row_arrs: Vec<String> = t
        .rows
        .iter()
        .map(|r| {
            format!(
                "({})",
                r.iter()
                    .map(|c| format!("[{}]", c))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect();
    let mut all = vec![header_arr];
    all.extend(row_arrs);
    format!("\n#gh-table(\n  {},\n)\n\n", all.join(",\n  "))
}

/// Convert a markdown source string to typst markup.
pub fn markdown_to_typst(src: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_GFM);

    let parser = Parser::new_ext(src, opts);
    let mut em = Emitter::new();
    for ev in parser {
        em.handle(ev);
    }
    let body = em.finish();

    let mut full = String::with_capacity(PREAMBLE.len() + body.len() + 1);
    full.push_str(PREAMBLE);
    full.push('\n');
    full.push_str(&body);
    full
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let out = markdown_to_typst("# Hello\n\nWorld with **bold** and *italic*.");
        assert!(out.contains("= Hello"));
        assert!(out.contains("#strong[bold]"));
        assert!(out.contains("#emph[italic]"));
    }

    #[test]
    fn task_list() {
        let out = markdown_to_typst("- [x] done\n- [ ] todo\n");
        assert!(out.contains("#gh-checkbox(true)"));
        assert!(out.contains("#gh-checkbox(false)"));
    }

    #[test]
    fn table() {
        let out = markdown_to_typst("| A | B |\n| - | - |\n| 1 | 2 |\n");
        assert!(out.contains("#gh-table"));
        assert!(out.contains("[A]"));
        assert!(out.contains("[1]"));
    }
}
