// Smoke-test the WASM module: load it in bun, compile a tiny markdown sample,
// write the resulting PDF to /tmp/wasm-smoke.pdf and verify it has the %PDF
// magic header.

import { readFileSync, writeFileSync } from 'node:fs';
import init, { markdown_to_pdf } from '../src/lib/wasm/markdown2pdf_wasm.js';

const wasmBytes = readFileSync(
	new URL('../src/lib/wasm/markdown2pdf_wasm_bg.wasm', import.meta.url)
);
await init({ module_or_path: wasmBytes });

const md = `# Hello\n\nThis is **bold** and *italic*.\n\n- [x] item one\n- [ ] item two\n`;
const pdf = markdown_to_pdf(md);
const out = '/tmp/wasm-smoke.pdf';
writeFileSync(out, pdf);
const head = Array.from(pdf.slice(0, 5)).map((b) => String.fromCharCode(b)).join('');
console.log(`wrote ${pdf.length} bytes to ${out}, head=${JSON.stringify(head)}`);
if (head !== '%PDF-') {
	console.error('FAIL: bytes are not a PDF');
	process.exit(1);
}
console.log('PASS');
