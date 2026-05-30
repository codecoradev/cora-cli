<script lang="ts">
	import { page } from '$app/stores';

	let { children } = $props();

	const navLinks = [
		{ href: '/docs/cli-reference', label: 'CLI Reference' },
		{ href: '/docs/configuration', label: 'Configuration' },
		{ href: '/docs/providers', label: 'Providers' },
		{ href: '/docs/examples', label: 'Examples' }
	];
</script>

<svelte:head>
	<title>Docs — cora</title>
</svelte:head>

<div style="min-height: 100vh; display: flex; background: var(--bg-primary);">

	<!-- Sidebar (desktop) -->
	<aside class="docs-sidebar hidden lg:block">
		<div style="padding: 1.5rem;">
			<a href="/" class="flex items-center gap-2 mb-8" style="color: var(--text-tertiary); text-decoration: none; font-size: 0.875rem; transition: color 0.2s;">
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
				Back to Home
			</a>

			<div style="margin-bottom: 1.5rem;">
				<a href="/docs" style="font-size: 1.125rem; font-weight: 600; color: var(--accent); text-decoration: none;">cora docs</a>
			</div>

			<nav style="display: flex; flex-direction: column; gap: 0.25rem;">
				{#each navLinks as link}
					<a
						href={link.href}
						class="docs-sidebar-link"
						class:active={$page.url.pathname === link.href}
					>
						{link.label}
					</a>
				{/each}
			</nav>
		</div>
	</aside>

	<!-- Mobile nav -->
	<nav class="mobile-docs-nav">
    <a href="/" style="color: var(--text-tertiary);" aria-label="Back to Home">
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
		</a>
		{#each navLinks as link}
			<a href={link.href} class:active={$page.url.pathname === link.href}>
				{link.label}
			</a>
		{/each}
	</nav>

	<!-- Main content -->
	<main style="flex: 1; min-width: 0;">
		<div style="max-width: 56rem; margin: 0 auto; padding: 3rem 2rem;">
			{@render children()}
		</div>
	</main>
</div>
