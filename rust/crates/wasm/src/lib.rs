//! WASM bindings for browser usage.
//!
//! Exposes `markdown_to_pdf(md: string) -> Uint8Array`.

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn markdown_to_pdf(markdown: &str) -> Result<Vec<u8>, JsError> {
    let opts = markdown2pdf_core::Options::default();
    markdown2pdf_core::markdown_to_pdf(markdown, &opts).map_err(|e| JsError::new(&format!("{e}")))
}

#[wasm_bindgen]
pub fn markdown_to_typst(markdown: &str) -> String {
    markdown2pdf_core::markdown_to_typst(markdown)
}
