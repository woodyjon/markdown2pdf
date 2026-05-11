import MarkdownIt from 'markdown-it';
import taskLists from 'markdown-it-task-lists';
import anchor from 'markdown-it-anchor';
import hljs from 'highlight.js';

function makeRenderer(opts: { anchors: boolean }) {
	const md = new MarkdownIt({
		html: false,
		linkify: true,
		typographer: true,
		highlight(str: string, lang: string): string {
			if (lang && hljs.getLanguage(lang)) {
				try {
					return `<pre class="hljs"><code>${hljs.highlight(str, { language: lang, ignoreIllegals: true }).value}</code></pre>`;
				} catch {
					// fall through to default
				}
			}
			return `<pre class="hljs"><code>${md.utils.escapeHtml(str)}</code></pre>`;
		}
	});
	md.use(taskLists, { enabled: true, label: true });
	if (opts.anchors) {
		md.use(anchor, {
			permalink: anchor.permalink.headerLink({ safariReaderFix: true })
		});
	}
	return md;
}

const previewRenderer = makeRenderer({ anchors: false });
const docsRenderer = makeRenderer({ anchors: true });

export function renderMarkdown(source: string): string {
	return previewRenderer.render(source);
}

export function renderDocsMarkdown(source: string): string {
	return docsRenderer.render(source);
}
