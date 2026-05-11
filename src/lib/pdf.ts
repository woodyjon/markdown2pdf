/**
 * Compile markdown → PDF entirely in the browser, off the main thread.
 *
 * A dedicated Web Worker owns the WASM module (`markdown2pdf-core` compiled
 * to WebAssembly — same Rust source that produces the native CLI). The worker
 * is lazily instantiated on the first call, so the ~23 MB blob isn't fetched
 * until the user actually generates a PDF.
 *
 * Multiple concurrent calls are safe: each request gets a unique id and the
 * matching response resolves only its own promise.
 */

import type { WorkerRequest, WorkerResponse } from './pdf.worker';

let worker: Worker | null = null;
let nextId = 0;
const pending = new Map<
	number,
	{ resolve: (value: WorkerResponse) => void; reject: (reason: unknown) => void }
>();

function getWorker(): Worker {
	if (worker) return worker;
	worker = new Worker(new URL('./pdf.worker.ts', import.meta.url), { type: 'module' });
	worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
		const entry = pending.get(event.data.id);
		if (!entry) return;
		pending.delete(event.data.id);
		entry.resolve(event.data);
	};
	worker.onerror = (event) => {
		const error = new Error(event.message || 'PDF worker crashed');
		for (const entry of pending.values()) entry.reject(error);
		pending.clear();
		worker?.terminate();
		worker = null;
	};
	return worker;
}

function request(req: Omit<WorkerRequest, 'id'>): Promise<WorkerResponse> {
	const id = nextId++;
	const w = getWorker();
	return new Promise((resolve, reject) => {
		pending.set(id, { resolve, reject });
		w.postMessage({ id, ...req } as WorkerRequest);
	});
}

export interface PdfOptions {
	filename?: string;
}

export async function downloadPDF(
	markdownSource: string,
	options: PdfOptions = {}
): Promise<void> {
	const { filename = 'document.pdf' } = options;
	const reply = await request({ kind: 'pdf', markdown: markdownSource });
	if (!reply.ok) throw new Error(reply.error);
	if (reply.kind !== 'pdf') throw new Error('unexpected worker reply');

	const blob = new Blob([reply.bytes as BlobPart], { type: 'application/pdf' });
	const url = URL.createObjectURL(blob);
	const a = document.createElement('a');
	a.href = url;
	a.download = filename;
	document.body.appendChild(a);
	a.click();
	a.remove();
	setTimeout(() => URL.revokeObjectURL(url), 1000);
}

/** Debug helper: returns the typst source for the given markdown. */
export async function previewTypstSource(markdownSource: string): Promise<string> {
	const reply = await request({ kind: 'typst', markdown: markdownSource });
	if (!reply.ok) throw new Error(reply.error);
	if (reply.kind !== 'typst') throw new Error('unexpected worker reply');
	return reply.source;
}
