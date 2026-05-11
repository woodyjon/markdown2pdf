//! Minimal `typst::World` for in-memory single-file compilation with embedded fonts.

use std::sync::OnceLock;

use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

// Static per-weight fonts. We tried variable fonts (Inter-Variable +
// JetBrainsMono-Variable) but Typst 0.14 only registers the default weight
// from a variable font's OS/2 table — it does not interpolate the weight
// axis at runtime. Result: every "bold" / "semibold" request silently fell
// back to Regular and the rendered PDF lost its weight contrast (table
// headers and **bold** text became indistinguishable from body text).
// Static per-weight files give Typst exact matches for each weight.
const FONT_BYTES: &[(&str, &[u8])] = &[
    (
        "Inter-Regular.ttf",
        include_bytes!("../fonts/Inter-Regular.ttf"),
    ),
    (
        "Inter-Italic.ttf",
        include_bytes!("../fonts/Inter-Italic.ttf"),
    ),
    (
        "Inter-SemiBold.ttf",
        include_bytes!("../fonts/Inter-SemiBold.ttf"),
    ),
    (
        "Inter-SemiBoldItalic.ttf",
        include_bytes!("../fonts/Inter-SemiBoldItalic.ttf"),
    ),
    ("Inter-Bold.ttf", include_bytes!("../fonts/Inter-Bold.ttf")),
    (
        "JetBrainsMono-Regular.ttf",
        include_bytes!("../fonts/JetBrainsMono-Regular.ttf"),
    ),
    (
        "JetBrainsMono-Bold.ttf",
        include_bytes!("../fonts/JetBrainsMono-Bold.ttf"),
    ),
];

/// Lazily-built fonts shared by every compilation.
struct FontStore {
    fonts: Vec<Font>,
    book: LazyHash<FontBook>,
}

fn font_store() -> &'static FontStore {
    static STORE: OnceLock<FontStore> = OnceLock::new();
    STORE.get_or_init(|| {
        let mut fonts: Vec<Font> = Vec::new();
        for (_name, bytes) in FONT_BYTES {
            let buf = Bytes::new(bytes.to_vec());
            // Each face within a TTF/OTF file gets its own Font instance.
            for face in 0.. {
                match Font::new(buf.clone(), face) {
                    Some(font) => fonts.push(font),
                    None => break,
                }
            }
        }
        let book = FontBook::from_fonts(&fonts);
        FontStore {
            fonts,
            book: LazyHash::new(book),
        }
    })
}

fn library() -> &'static LazyHash<Library> {
    static LIB: OnceLock<LazyHash<Library>> = OnceLock::new();
    LIB.get_or_init(|| LazyHash::new(Library::default()))
}

pub struct EmbeddedWorld {
    main_id: FileId,
    main_source: Source,
}

impl EmbeddedWorld {
    pub fn new(source: String) -> Self {
        let main_id = FileId::new(None, VirtualPath::new("main.typ"));
        let main_source = Source::new(main_id, source);
        Self {
            main_id,
            main_source,
        }
    }
}

impl World for EmbeddedWorld {
    fn library(&self) -> &LazyHash<Library> {
        library()
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &font_store().book
    }

    fn main(&self) -> FileId {
        self.main_id
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_id {
            Ok(self.main_source.clone())
        } else {
            Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(id.vpath().as_rootless_path().into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        font_store().fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        // No clock on wasm32 without extra plumbing; documents that call
        // `datetime.today()` simply receive `none`.
        None
    }
}
