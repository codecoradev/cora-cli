<script lang="ts">
	import { page } from '$app/stores';

	let { children } = $props();

	const navLinks = [
		{ href: '/docs/getting-started', label: 'Getting Started' },
		{ href: '/docs/installation', label: 'Installation' },
		{ href: '/docs/usage', label: 'Usage' },
		{ href: '/docs/configuration', label: 'Configuration' },
		{ href: '/docs/providers', label: 'Providers' },
		{ href: '/docs/examples', label: 'Examples' },
		{ href: '/docs/roadmap', label: 'Roadmap' },
		{ href: '/docs/cli-reference', label: 'CLI Reference' },
		{ href: '/docs/changelog', label: 'Changelog' },
	];
</script>

<svelte:head>
	<title>Docs — cora</title>
</svelte:head>

<div class="min-h-screen flex bg-[var(--background)]">

	<!-- Sidebar (desktop) -->
	<aside class="docs-sidebar hidden lg:block">
		<div class="p-6">
			<a href="/" class="flex items-center gap-2 mb-8 text-sm text-[var(--muted-foreground)] no-underline transition-colors duration-200 min-h-11 hover:text-[var(--foreground)]">
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
				Back to Home
			</a>

			<div class="mb-6">
				<a href="/docs" class="text-lg font-semibold text-[var(--accent)] no-underline -tracking-tight leading-tight">cora docs</a>
			</div>

			<nav class="flex flex-col gap-1">
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
		<a href="/" class="text-[var(--muted-foreground)]" aria-label="Back to Home">
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
		</a>
		{#each navLinks as link}
			<a href={link.href} class:active={$page.url.pathname === link.href}>
				{link.label}
			</a>
		{/each}
	</nav>

	<!-- Main content -->
	<main class="flex-1 min-w-0">
		<div class="max-w-3xl mx-auto px-8 py-12">
			{@render children()}
		</div>
	</main>
</div>
