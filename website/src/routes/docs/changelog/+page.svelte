<script lang="ts">
	import { onMount } from 'svelte';
	import changelogRaw from '../../../CHANGELOG.md?raw';

	interface ChangelogSection {
		version: string;
		date: string;
		url: string;
		categories: { type: string; items: string[] }[];
	}

	const REPO = 'https://github.com/ajianaz/cora-cli';

	function parseChangelog(md: string): ChangelogSection[] {
		const sections: ChangelogSection[] = [];
		// Split into version sections at ## headers
		const blocks = md.split(/^## \[/m);
		for (const block of blocks) {
			const headerMatch = block.match(/^\[([^\]]+)\]\s*(?:-\s*)?(\d{4}-\d{2}-\d{2})/);
			if (!headerMatch) continue;
			const version = headerMatch[1];
			const date = headerMatch[2];

			// Skip Unreleased if empty (no ### headers with content)
			if (version === 'Unreleased' && !block.match(/^###\s+\w+/m)) continue;

			const categories: { type: string; items: string[] }[] = [];
			const catBlocks = block.split(/^###\s+/m);
			for (const catBlock of catBlocks) {
				const catMatch = catBlock.match(/^(\w[\w\s]*?)(?:\s*)\n([\s\S]*)$/);
				if (!catMatch) continue;
				const type = catMatch[1].trim();
				const items = catMatch[2]
					.split('\n')
					.map((l) => l.replace(/^-\s*/, '').trim())
					.filter(Boolean);
				if (items.length > 0) {
					categories.push({ type, items });
				}
			}

			// Build compare URL
			let url: string;
			if (version === 'Unreleased') {
				url = `${REPO}/compare/v0.1.3...develop`;
			} else {
				url = `${REPO}/releases/tag/v${version}`;
			}

			sections.push({ version, date, url, categories });
		}
		return sections;
	}

	const releases = parseChangelog(changelogRaw);

	onMount(() => {
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						entry.target.classList.add('revealed');
						observer.unobserve(entry.target);
					}
				});
			},
			{ threshold: 0.1 }
		);
		document.querySelectorAll('.scroll-reveal').forEach((el) => observer.observe(el));
		return () => observer.disconnect();
	});
</script>

<svelte:head>
	<title>Changelog — cora docs</title>
	<meta name="description" content="cora-cli changelog — release history and version notes." />
</svelte:head>

<div class="docs-content">
	<h1 class="scroll-reveal">Changelog</h1>
	<p class="scroll-reveal text-[var(--muted-foreground)]">
		For the full changelog, see the
		<a href="{REPO}/blob/develop/CHANGELOG.md" class="text-[var(--accent)] hover:underline" target="_blank" rel="noopener noreferrer">repository</a>.
	</p>

	{#each releases as release, i}
		<section class="docs-section scroll-reveal">
			<h2 class="flex items-center gap-3">
				{#if release.version === 'Unreleased'}
					<span class="inline-flex items-center gap-1.5 px-2 py-0.5 text-xs font-medium rounded-full bg-[var(--accent)]/15 text-[var(--accent)] border border-[var(--accent)]/25">
						Unreleased
					</span>
				{:else}
					<a href={release.url} class="text-[var(--accent)] hover:underline no-underline" target="_blank" rel="noopener noreferrer">
						v{release.version}
					</a>
				{/if}
				<span class="text-sm font-normal text-[var(--muted-foreground)]">{release.date}</span>
			</h2>

			{#each release.categories as category}
				<div class="mt-4">
					<h3 class="text-sm font-semibold uppercase tracking-wider text-[var(--muted-foreground)] mb-2">{category.type}</h3>
					<ul class="list-disc list-outside ml-4 flex flex-col gap-1.5">
						{#each category.items as item}
							<li class="text-[var(--foreground)] text-sm leading-relaxed pl-1">{@html item}</li>
						{/each}
					</ul>
				</div>
			{/each}
		</section>
	{/each}
</div>
