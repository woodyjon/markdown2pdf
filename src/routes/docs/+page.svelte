<script lang="ts">
	import { renderDocsMarkdown } from '$lib/markdown';
	let { data } = $props();
	let html = $derived(renderDocsMarkdown(data.page.source));
</script>

<svelte:head>
	<title>{data.page.title} — markdown2pdf</title>
	<link rel="canonical" href="https://markdown2pdf.eu/docs" />
	<meta name="description" content={data.page.tagline} />
</svelte:head>

<article class="markdown-body doc-content">
	{@html html}
</article>

<style>
	.doc-content {
		max-width: 760px;
	}
</style>
