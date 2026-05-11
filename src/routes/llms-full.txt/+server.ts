import { docs } from '$lib/docs';

export const prerender = true;

const SITE = 'https://markdown2pdf.eu';

export function GET() {
	const sections: string[] = [];
	sections.push('# markdown2pdf — full documentation bundle');
	sections.push('');
	sections.push(`Source: ${SITE} · Repository: https://github.com/woodyjon/markdown2pdf · License: MIT`);
	sections.push('');
	sections.push('---');
	sections.push('');

	for (const doc of docs) {
		const path = doc.slug === 'overview' ? '/docs' : `/docs/${doc.slug}`;
		sections.push(`<!-- url: ${SITE}${path} -->`);
		sections.push(doc.source.trim());
		sections.push('');
		sections.push('---');
		sections.push('');
	}

	return new Response(sections.join('\n'), {
		headers: { 'content-type': 'text/plain; charset=utf-8' }
	});
}
