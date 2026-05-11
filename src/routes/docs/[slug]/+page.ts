import { error } from '@sveltejs/kit';
import { docs, docsBySlug } from '$lib/docs';

export function entries() {
	return docs.filter((d) => d.slug !== 'overview').map((d) => ({ slug: d.slug }));
}

export function load({ params }) {
	const page = docsBySlug[params.slug];
	if (!page || page.slug === 'overview') throw error(404, 'Not found');
	return { page };
}
