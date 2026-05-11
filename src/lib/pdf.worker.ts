/// <reference lib="webworker" />
/**
 * PDF worker — loads the markdown2pdf WASM module once, then answers
 * `pdf` / `typst` requests over postMessage so the main thread stays free.
 *
 * Protocol:
 *   in:  { id: number, kind: 'pdf' | 'typst', markdown: string }
 *   out: { id: number, ok: true, kind: 'pdf', bytes: Uint8Array }
 *        | { id: number, ok: true, kind: 'typst', source: string }
 *        | { id: number, ok: false, error: string }
 */

import init, {
	markdown_to_pdf,
	markdown_to_typst
} from './wasm/markdown2pdf_wasm.js';
import wasmUrl from './wasm/markdown2pdf_wasm_bg.wasm?url';

export type WorkerRequest =
	| { id: number; kind: 'pdf'; markdown: string }
	| { id: number; kind: 'typst'; markdown: string };

export type WorkerResponse =
	| { id: number; ok: true; kind: 'pdf'; bytes: Uint8Array }
	| { id: number; ok: true; kind: 'typst'; source: string }
	| { id: number; ok: false; error: string };

let ready: Promise<void> | null = null;
function ensureReady(): Promise<void> {
	if (!ready) ready = init({ module_or_path: wasmUrl }).then(() => undefined);
	return ready;
}

self.onmessage = async (event: MessageEvent<WorkerRequest>) => {
	const { id, kind, markdown } = event.data;
	try {
		await ensureReady();
		if (kind === 'pdf') {
			const bytes = markdown_to_pdf(markdown);
			const reply: WorkerResponse = { id, ok: true, kind: 'pdf', bytes };
			(self as DedicatedWorkerGlobalScope).postMessage(reply, [bytes.buffer]);
		} else {
			const source = markdown_to_typst(markdown);
			const reply: WorkerResponse = { id, ok: true, kind: 'typst', source };
			(self as DedicatedWorkerGlobalScope).postMessage(reply);
		}
	} catch (err) {
		const reply: WorkerResponse = {
			id,
			ok: false,
			error: err instanceof Error ? err.message : String(err)
		};
		(self as DedicatedWorkerGlobalScope).postMessage(reply);
	}
};
