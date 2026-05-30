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

<div class="grid-bg min-h-screen flex">
	<!-- Sidebar -->
	<aside class="w-64 flex-shrink-0 border-r border-[var(--color-border)] bg-[var(--color-surface)] sticky top-0 h-screen overflow-y-auto hidden lg:block">
		<div class="p-6">
			<a href="/" class="flex items-center gap-2 text-[var(--color-text-muted)] hover:text-[var(--color-text)] transition-colors mb-8">
				<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M19 12H5M12 19l-7-7 7-7"/></svg>
				<span class="text-sm">Back to Home</span>
			</a>

			<div class="mb-4">
				<a href="/docs" class="text-lg font-bold text-[var(--color-accent)]">cora docs</a>
			</div>

			<nav class="space-y-1">
				{#each navLinks as link}
					<a
						href={link.href}
						class="flex items-center gap-2 px-3 py-2 rounded-lg text-sm transition-all {$page.url.pathname === link.href ? 'bg-[var(--color-accent-dim)] text-[var(--color-accent)] font-medium' : 'text-[var(--color-text-muted)] hover:text-[var(--color-text)] hover:bg-[var(--color-surface-2)]'}"
					>
						{link.label}
					</a>
				{/each}
			</nav>
		</div>
	</aside>

	<!-- Mobile nav -->
	<div class="lg:hidden fixed top-0 left-0 right-0 z-50 bg-[var(--color-surface)] border-b border-[var(--color-border)] px-4 py-3">
		<div class="flex items-center justify-between">
			<a href="/" class="text-sm text-[var(--color-text-muted)] hover:text-[var(--color-text)]">
				← Home
			</a>
			<span class="text-[var(--color-accent)] font-semibold">Docs</span>
			<div class="flex gap-3">
				{#each navLinks as link}
					<a href={link.href} class="text-xs text-[var(--color-text-muted)] hover:text-[var(--color-accent)]">{$page.url.pathname === link.href ? '✓ ' : ''}{link.label.split(' ')[0]}</a>
				{/each}
			</div>
		</div>
	</div>

	<!-- Main content -->
	<main class="flex-1 min-w-0 lg:p-0">
		<div class="max-w-4xl mx-auto px-6 py-12 lg:py-16 lg:px-12">
			{@render children()}
		</div>
	</main>
</div>
