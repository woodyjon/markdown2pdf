<script lang="ts">
	import { renderMarkdown } from '$lib/markdown';
	import { downloadPDF } from '$lib/pdf';

	let markdownSource = $state(DEFAULT_MARKDOWN);
	let renderedHtml = $derived(renderMarkdown(markdownSource));
	let generating = $state(false);
	let filename = $state('document');

	async function handleDownload() {
		if (generating) return;
		generating = true;
		try {
			const base = filename || 'document';
			await downloadPDF(markdownSource, { filename: `${base}.pdf` });
		} catch (err) {
			console.error('PDF generation failed:', err);
			alert('Failed to generate PDF: ' + (err instanceof Error ? err.message : err));
		} finally {
			generating = false;
		}
	}
</script>

<svelte:head>
	<title>Markdown to PDF — Easy Online Converter</title>
	<link rel="canonical" href="https://markdown2pdf.eu/" />
	<meta name="description" content="Convert Markdown to clean, formatted PDF documents. Easy to print markdown. Download and print. Works in your browser." />
	<meta property="og:title" content="Markdown to PDF — Easy Online Converter" />
	<meta property="og:description" content="Paste your Markdown, preview it live, download as a beautifully formatted PDF, then print easily." />
	<meta property="og:url" content="https://markdown2pdf.eu/" />
	<meta property="og:type" content="website" />
	<meta name="twitter:card" content="summary" />
	<meta name="twitter:title" content="Markdown to PDF — Easy Online Converter" />
	<meta name="twitter:description" content="Paste your Markdown, preview it live, download as a beautifully formatted PDF, then print easily." />
</svelte:head>

<div class="app">
	<!-- Header -->
	<header class="header">
		<div class="header-brand">
			<svg class="header-logo" viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg">
				<rect width="32" height="32" rx="4" fill="#2563eb"/>
				<text x="16" y="22" font-family="Arial, sans-serif" font-size="16" font-weight="bold" fill="white" text-anchor="middle">M</text>
			</svg>
			<div>
				<h1 class="header-title">Markdown to PDF</h1>
				<p class="header-tagline">Paste and preview markdown → download PDF (then print easily). Nothing is stored.</p>
			</div>
		</div>
		<div class="header-actions">
			<a class="header-link" href="/docs">Docs</a>
			<a
				class="header-link header-link-icon"
				href="https://github.com/woodyjon/markdown2pdf"
				aria-label="GitHub"
				target="_blank"
				rel="noopener"
			>
				<svg viewBox="0 0 24 24" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
					<path d="M12 .5a12 12 0 00-3.79 23.4c.6.11.82-.26.82-.58v-2.16c-3.34.73-4.04-1.42-4.04-1.42-.55-1.4-1.34-1.78-1.34-1.78-1.1-.74.08-.73.08-.73 1.21.08 1.85 1.24 1.85 1.24 1.08 1.84 2.83 1.31 3.52 1 .11-.78.42-1.31.76-1.61-2.66-.3-5.46-1.33-5.46-5.93 0-1.31.46-2.39 1.23-3.23-.12-.3-.54-1.52.12-3.16 0 0 1-.32 3.3 1.23a11.46 11.46 0 016 0c2.3-1.55 3.3-1.23 3.3-1.23.66 1.64.24 2.86.12 3.16.77.84 1.23 1.92 1.23 3.23 0 4.62-2.81 5.62-5.49 5.92.43.37.81 1.1.81 2.22v3.29c0 .32.22.7.83.58A12 12 0 0012 .5z"/>
				</svg>
			</a>
			<div class="filename-input-wrap">
				<input
					type="text"
					class="filename-input"
					bind:value={filename}
					placeholder="document"
					aria-label="PDF filename"
				/>
				<span class="filename-ext">.pdf</span>
			</div>
			<button
				class="btn-download"
				onclick={handleDownload}
				disabled={generating || !markdownSource.trim()}
				aria-label="Download PDF"
			>
				{#if generating}
					<span class="spinner"></span>
					Generating...
				{:else}
					<svg class="btn-icon" viewBox="0 0 20 20" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
						<path d="M10 3a1 1 0 011 1v7.586l2.293-2.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 111.414-1.414L9 11.586V4a1 1 0 011-1z"/>
						<path d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"/>
					</svg>
					Download PDF
				{/if}
			</button>
		</div>
	</header>

	<!-- Quick-use info strip -->
	<div class="info-strip">
		<div class="info-item">
			<svg class="info-icon" viewBox="0 0 20 20" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
				<path d="M10 2a4 4 0 00-4 4v1H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2V9a2 2 0 00-2-2h-1V6a4 4 0 00-4-4zm-2 5V6a2 2 0 114 0v1H8zm-1 4a1 1 0 112 0 1 1 0 01-2 0zm5 0a1 1 0 112 0 1 1 0 01-2 0z"/>
			</svg>
			<span>
				<strong>Using an AI agent?</strong> Install the
				<a href="/docs/skill">Claude skill</a> — your agent will convert Markdown to PDF on request (auto-installs the CLI).
			</span>
		</div>
		<div class="info-item">
			<svg class="info-icon" viewBox="0 0 20 20" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
				<path d="M3 4a2 2 0 012-2h10a2 2 0 012 2v12a2 2 0 01-2 2H5a2 2 0 01-2-2V4zm3.3 3.3a1 1 0 011.4 0L10 9.6l2.3-2.3a1 1 0 011.4 1.4L11.4 11l2.3 2.3a1 1 0 01-1.4 1.4L10 12.4l-2.3 2.3a1 1 0 01-1.4-1.4L8.6 11 6.3 8.7a1 1 0 010-1.4z" opacity=".25"/>
				<path d="M4 5h12v2H4V5zm1.5 4.3a.75.75 0 011.06 0L8.7 11.4a.75.75 0 010 1.06l-2.14 2.13a.75.75 0 11-1.06-1.06L7.1 11.94 5.5 10.36a.75.75 0 010-1.06zM10 14h4v1.5h-4V14z"/>
			</svg>
			<span>
				<strong>Prefer the terminal?</strong> Install the CLI:
				<code>curl -fsSL https://markdown2pdf.eu/install.sh | sh</code>
				— see <a href="/docs/cli">CLI docs</a>.
			</span>
		</div>
	</div>

	<!-- Main content: Editor + Preview -->
	<main class="main">
		<!-- Editor pane -->
		<section class="pane pane-editor">
			<div class="pane-header">
				<span class="pane-label">Markdown</span>
				<button
					class="btn-clear"
					onclick={() => { markdownSource = ''; }}
					disabled={!markdownSource}
					aria-label="Clear editor"
				>
					Clear
				</button>
			</div>
			<textarea
				class="editor"
				bind:value={markdownSource}
				placeholder="Type or paste your Markdown here..."
				spellcheck="false"
			></textarea>
		</section>

		<!-- Preview pane -->
		<section class="pane pane-preview">
			<div class="pane-header">
				<span class="pane-label">Preview</span>
			</div>
			<div class="preview-scroll">
				<div class="preview-paper">
					<div class="markdown-body">
						{@html renderedHtml}
					</div>
				</div>
			</div>
		</section>
	</main>
</div>

<script module lang="ts">
const DEFAULT_MARKDOWN = `# Welcome to Markdown to PDF

A simple tool to convert Markdown into clean PDF documents you can download then print — right in your browser.

## How to use

1. **Clear** this text and paste or type your Markdown on the left
2. **Preview** your formatted document on the right
3. **Name** your file in the header, then hit **Download PDF**

That's it. No signup, no tracking. Nothing is stored — everything runs in your browser.

---

## What's supported

Everything you'd expect from GitHub-flavored Markdown:

| Element           | Example                        |
|-------------------|--------------------------------|
| Headings          | \`# H1\` through \`###### H6\`  |
| **Bold** / *Italic* | \`**bold**\` / \`*italic*\`    |
| Links             | \`[text](url)\`                |
| Code blocks       | Fenced with \`\`\` + language   |
| Tables            | Like this one                  |
| Task lists        | \`- [x] done\`                 |
| Blockquotes       | \`> quoted text\`              |

### Code highlighting

\`\`\`javascript
function greet(name) {
  return \`Hello, \${name}!\`;
}
\`\`\`

### Task list

- [x] Write Markdown
- [x] Preview the result
- [ ] Download as PDF

> **Tip:** The PDF matches what you see in the preview panel.

---

*Go ahead — clear this and start writing!*
`;
</script>

<style>
	/* Layout */
	.app {
		display: flex;
		flex-direction: column;
		height: 100vh;
		overflow: hidden;
	}

	/* Header */
	.header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		height: var(--header-height);
		padding: 0 16px;
		background: var(--color-surface);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
		gap: 12px;
	}

	.header-brand {
		display: flex;
		align-items: center;
		gap: 10px;
		flex-shrink: 0;
	}

	.header-logo {
		width: 28px;
		height: 28px;
	}

	.header-title {
		font-size: 18px;
		font-weight: 600;
		color: var(--color-text);
		white-space: nowrap;
	}

	.header-tagline {
		font-size: 12px;
		color: var(--color-text-secondary);
		white-space: nowrap;
		margin-top: 1px;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.header-link {
		padding: 6px 10px;
		font-size: 14px;
		color: var(--color-text-secondary);
		text-decoration: none;
		border-radius: 6px;
		transition: background 0.15s, color 0.15s;
	}

	.header-link:hover {
		background: var(--color-bg);
		color: var(--color-text);
	}

	.header-link-icon {
		display: inline-flex;
		align-items: center;
		padding: 6px;
	}

	.header-link-icon svg {
		width: 18px;
		height: 18px;
		display: block;
	}

	/* Filename input */
	.filename-input-wrap {
		display: flex;
		align-items: center;
		background: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: 6px;
		overflow: hidden;
	}

	.filename-input {
		border: none;
		background: transparent;
		padding: 6px 8px;
		font-size: 14px;
		font-family: var(--font-sans);
		color: var(--color-text);
		width: 140px;
		outline: none;
	}

	.filename-input::placeholder {
		color: var(--color-text-secondary);
	}

	.filename-ext {
		padding-right: 8px;
		font-size: 14px;
		color: var(--color-text-secondary);
		user-select: none;
	}

	/* Download button */
	.btn-download {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 8px 16px;
		background: var(--color-primary);
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 14px;
		font-weight: 500;
		font-family: var(--font-sans);
		cursor: pointer;
		transition: background 0.15s;
		white-space: nowrap;
	}

	.btn-download:hover:not(:disabled) {
		background: var(--color-primary-hover);
	}

	.btn-download:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-icon {
		width: 16px;
		height: 16px;
	}

	.spinner {
		width: 14px;
		height: 14px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Info strip */
	.info-strip {
		display: flex;
		align-items: center;
		gap: 24px;
		padding: 8px 16px;
		background: #eff6ff;
		border-bottom: 1px solid #dbeafe;
		font-size: 13px;
		color: #1e3a8a;
		flex-shrink: 0;
		flex-wrap: wrap;
	}

	.info-item {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		min-width: 0;
	}

	.info-icon {
		width: 16px;
		height: 16px;
		flex-shrink: 0;
		color: #2563eb;
	}

	.info-strip a {
		color: #2563eb;
		text-decoration: underline;
		text-underline-offset: 2px;
	}

	.info-strip a:hover {
		color: #1d4ed8;
	}

	.info-strip code {
		font-family: var(--font-mono);
		font-size: 12px;
		background: white;
		border: 1px solid #dbeafe;
		border-radius: 4px;
		padding: 1px 6px;
		color: #1e3a8a;
	}

	/* Main layout */
	.main {
		display: flex;
		flex: 1;
		min-height: 0;
	}

	/* Panes */
	.pane {
		display: flex;
		flex-direction: column;
		flex: 1;
		min-width: 0;
	}

	.pane-editor {
		border-right: 1px solid var(--color-border);
	}

	.pane-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 8px 16px;
		background: var(--color-bg);
		border-bottom: 1px solid var(--color-border-light);
		flex-shrink: 0;
	}

	.pane-label {
		font-size: 12px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-secondary);
	}

	.btn-clear {
		padding: 3px 10px;
		font-size: 12px;
		font-family: var(--font-sans);
		color: var(--color-text-secondary);
		background: transparent;
		border: 1px solid var(--color-border);
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-clear:hover:not(:disabled) {
		background: var(--color-bg);
		color: var(--color-text);
	}

	.btn-clear:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	/* Editor */
	.editor {
		flex: 1;
		padding: 20px;
		border: none;
		resize: none;
		outline: none;
		font-family: var(--font-mono);
		font-size: 14px;
		line-height: 1.6;
		color: var(--color-text);
		background: var(--color-surface);
		tab-size: 2;
	}

	.editor::placeholder {
		color: var(--color-text-secondary);
	}

	/* Preview */
	.pane-preview {
		background: var(--color-bg);
	}

	.preview-scroll {
		flex: 1;
		overflow-y: auto;
		padding: 24px;
	}

	.preview-paper {
		max-width: 800px;
		margin: 0 auto;
		background: var(--color-surface);
		border: 1px solid var(--color-border-light);
		border-radius: 8px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
		padding: 40px;
		min-height: 200px;
	}

	/* Responsive: stack on mobile */
	@media (max-width: 768px) {
		.header {
			flex-wrap: wrap;
			height: auto;
			padding: 10px 12px;
			gap: 8px;
		}

		.header-title {
			font-size: 16px;
		}

		.header-tagline {
			display: none;
		}

		.header-actions {
			width: 100%;
			justify-content: flex-end;
			flex-wrap: wrap;
		}

		.header-link {
			display: none;
		}

		.header-link-icon {
			display: inline-flex;
		}

		.filename-input {
			width: 100px;
		}

		.info-strip {
			flex-direction: column;
			align-items: flex-start;
			gap: 6px;
			font-size: 12px;
		}

		.info-strip code {
			font-size: 11px;
			word-break: break-all;
		}

		.main {
			flex-direction: column;
		}

		.pane-editor {
			border-right: none;
			border-bottom: 1px solid var(--color-border);
			flex: 1;
			min-height: 200px;
		}

		.pane-preview {
			flex: 1;
			min-height: 200px;
		}

		.preview-scroll {
			padding: 16px;
		}

		.preview-paper {
			padding: 20px;
		}
	}
</style>
