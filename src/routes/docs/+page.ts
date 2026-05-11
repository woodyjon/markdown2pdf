import { docsBySlug } from '$lib/docs';

export function load() {
	return {
		page: docsBySlug.overview
	};
}
