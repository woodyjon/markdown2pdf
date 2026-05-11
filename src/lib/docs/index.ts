import overview from './overview.md?raw';
import playground from './playground.md?raw';
import cli from './cli.md?raw';
import skill from './skill.md?raw';
import embedding from './embedding.md?raw';

export interface DocPage {
	slug: string;
	title: string;
	tagline: string;
	source: string;
}

export const docs: DocPage[] = [
	{ slug: 'overview', title: 'Overview', tagline: 'What markdown2pdf is and why it exists', source: overview },
	{ slug: 'playground', title: 'Web playground', tagline: 'Use it in the browser, no install', source: playground },
	{ slug: 'cli', title: 'CLI', tagline: 'Single binary, file in / PDF out', source: cli },
	{ slug: 'skill', title: 'Claude skill', tagline: 'Drop-in skill for Claude agents', source: skill },
	{ slug: 'embedding', title: 'Embed in Rust', tagline: 'Use the core crate from your own code', source: embedding }
];

export const docsBySlug: Record<string, DocPage> = Object.fromEntries(
	docs.map((d) => [d.slug, d])
);
