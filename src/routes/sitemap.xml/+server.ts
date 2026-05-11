import { docs } from '$lib/docs';

export const prerender = true;

const SITE = 'https://markdown2pdf.eu';
const TODAY = new Date().toISOString().slice(0, 10);

export function GET() {
	const urls: { loc: string; priority: string; changefreq: string }[] = [
		{ loc: `${SITE}/`, priority: '1.0', changefreq: 'monthly' },
		{ loc: `${SITE}/docs`, priority: '0.8', changefreq: 'monthly' }
	];
	for (const doc of docs) {
		if (doc.slug === 'overview') continue;
		urls.push({ loc: `${SITE}/docs/${doc.slug}`, priority: '0.7', changefreq: 'monthly' });
	}

	const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls
	.map(
		(u) => `  <url>
    <loc>${u.loc}</loc>
    <lastmod>${TODAY}</lastmod>
    <changefreq>${u.changefreq}</changefreq>
    <priority>${u.priority}</priority>
  </url>`
	)
	.join('\n')}
</urlset>
`;

	return new Response(xml, {
		headers: { 'content-type': 'application/xml; charset=utf-8' }
	});
}
