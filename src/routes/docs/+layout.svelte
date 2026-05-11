<script lang="ts">
	import { page } from '$app/state';
	import { docs } from '$lib/docs';

	let { children } = $props();

	let currentSlug = $derived.by(() => {
		const m = page.url.pathname.match(/^\/docs\/([^/]+)/);
		return m ? m[1] : 'overview';
	});

	let mobileNavOpen = $state(false);
</script>

<div class="docs">
	<header class="docs-header">
		<a class="brand" href="/">
			<svg class="brand-logo" viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg">
				<rect width="32" height="32" rx="4" fill="#2563eb" />
				<text
					x="16" y="22"
					font-family="Arial, sans-serif" font-size="16" font-weight="bold"
					fill="white" text-anchor="middle"
				>M</text>
			</svg>
			<span class="brand-title">Markdown to PDF</span>
		</a>
		<nav class="header-links">
			<a href="/" class="header-link">Playground</a>
			<a href="https://github.com/woodyjon/markdown2pdf" class="header-link" target="_blank" rel="noopener">GitHub</a>
		</nav>
		<button
			class="mobile-toggle"
			aria-label="Toggle docs navigation"
			aria-expanded={mobileNavOpen}
			onclick={() => (mobileNavOpen = !mobileNavOpen)}
		>
			<svg viewBox="0 0 20 20" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
				<path d="M3 5h14a1 1 0 010 2H3a1 1 0 010-2zm0 4h14a1 1 0 010 2H3a1 1 0 010-2zm0 4h14a1 1 0 010 2H3a1 1 0 010-2z" />
			</svg>
		</button>
	</header>

	<div class="docs-body">
		<aside class="sidebar" class:open={mobileNavOpen}>
			<div class="sidebar-section-title">Documentation</div>
			<ul class="sidebar-nav">
				{#each docs as doc}
					{@const href = doc.slug === 'overview' ? '/docs' : `/docs/${doc.slug}`}
					<li>
						<a
							{href}
							class:active={currentSlug === doc.slug}
							onclick={() => (mobileNavOpen = false)}
						>
							<span class="nav-title">{doc.title}</span>
							<span class="nav-tagline">{doc.tagline}</span>
						</a>
					</li>
				{/each}
			</ul>
			<div class="sidebar-section-title">For agents</div>
			<ul class="sidebar-nav">
				<li>
					<a href="/llms.txt">
						<span class="nav-title">/llms.txt</span>
						<span class="nav-tagline">Short index</span>
					</a>
				</li>
				<li>
					<a href="/llms-full.txt">
						<span class="nav-title">/llms-full.txt</span>
						<span class="nav-tagline">Full bundle</span>
					</a>
				</li>
			</ul>
		</aside>

		<main class="docs-main">
			{@render children()}
		</main>
	</div>
</div>

<style>
	.docs {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	.docs-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		height: var(--header-height);
		padding: 0 20px;
		background: var(--color-surface);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
	}

	.brand {
		display: flex;
		align-items: center;
		gap: 10px;
		text-decoration: none;
		color: var(--color-text);
	}

	.brand-logo {
		width: 28px;
		height: 28px;
	}

	.brand-title {
		font-size: 16px;
		font-weight: 600;
	}

	.header-links {
		display: flex;
		gap: 4px;
		align-items: center;
		margin-left: auto;
	}

	.header-link {
		padding: 6px 12px;
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

	.mobile-toggle {
		display: none;
		padding: 6px;
		background: transparent;
		border: 1px solid var(--color-border);
		border-radius: 6px;
		color: var(--color-text-secondary);
		cursor: pointer;
	}

	.mobile-toggle svg {
		width: 20px;
		height: 20px;
		display: block;
	}

	.docs-body {
		display: flex;
		flex: 1;
		min-height: 0;
	}

	.sidebar {
		width: 260px;
		flex-shrink: 0;
		padding: 24px 12px;
		border-right: 1px solid var(--color-border-light);
		background: var(--color-surface);
		overflow-y: auto;
	}

	.sidebar-section-title {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.06em;
		color: var(--color-text-secondary);
		padding: 0 12px;
		margin-bottom: 8px;
	}

	.sidebar-section-title:not(:first-child) {
		margin-top: 28px;
	}

	.sidebar-nav {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.sidebar-nav li {
		margin: 0;
	}

	.sidebar-nav a {
		display: block;
		padding: 8px 12px;
		border-radius: 6px;
		text-decoration: none;
		color: var(--color-text);
		transition: background 0.12s;
	}

	.sidebar-nav a:hover {
		background: var(--color-bg);
	}

	.sidebar-nav a.active {
		background: var(--color-accent);
	}

	.sidebar-nav a.active .nav-title {
		color: var(--color-primary);
	}

	.nav-title {
		display: block;
		font-size: 14px;
		font-weight: 500;
	}

	.nav-tagline {
		display: block;
		font-size: 12px;
		color: var(--color-text-secondary);
		margin-top: 2px;
	}

	.docs-main {
		flex: 1;
		min-width: 0;
		padding: 40px 48px;
		overflow-y: auto;
		background: var(--color-bg);
	}

	@media (max-width: 768px) {
		.mobile-toggle {
			display: block;
		}

		.header-links {
			display: none;
		}

		.docs-body {
			flex-direction: column;
		}

		.sidebar {
			display: none;
			width: 100%;
			border-right: none;
			border-bottom: 1px solid var(--color-border-light);
			padding: 12px;
		}

		.sidebar.open {
			display: block;
		}

		.docs-main {
			padding: 24px 16px;
		}
	}
</style>
