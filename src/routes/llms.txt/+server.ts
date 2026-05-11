import { docs } from '$lib/docs';

export const prerender = true;

const SITE = 'https://markdown2pdf.eu';

export function GET() {
	const lines: string[] = [];
	lines.push('# markdown2pdf');
	lines.push('');
	lines.push('> Convert Markdown to PDF — same Rust engine drives a web playground, a CLI, a Claude skill, and an embeddable Rust crate. MIT licensed.');
	lines.push('');
	lines.push('## Docs');
	lines.push('');
	for (const doc of docs) {
		const path = doc.slug === 'overview' ? '/docs' : `/docs/${doc.slug}`;
		lines.push(`- [${doc.title}](${SITE}${path}): ${doc.tagline}`);
	}
	lines.push('');
	lines.push('## Source');
	lines.push('');
	lines.push('- [GitHub repository](https://github.com/woodyjon/markdown2pdf): Source, issues, releases');
	lines.push('- [CLI binaries](https://github.com/woodyjon/markdown2pdf/releases): Prebuilt for macOS, Linux, Windows');
	lines.push('- [Claude skill](https://github.com/woodyjon/markdown2pdf/tree/main/skills/markdown2pdf): Drop into ~/.claude/skills/');
	lines.push('');
	lines.push('## Optional');
	lines.push('');
	lines.push(`- [Full doc bundle](${SITE}/llms-full.txt): All docs concatenated as plain text`);
	lines.push('');

	return new Response(lines.join('\n'), {
		headers: { 'content-type': 'text/plain; charset=utf-8' }
	});
}
