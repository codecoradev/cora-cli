<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { ModeWatcher } from 'mode-watcher';
	import { Sun, Moon, Menu, X } from '@lucide/svelte';

	let { children } = $props();

	const isDocs = $derived($page.url.pathname.startsWith('/docs'));

	let mobileMenuOpen = $state(false);
</script>

<ModeWatcher />

<div class="bg-background min-h-screen">
	<!-- Header -->
	<header class="site-header">
		<div class="max-w-6xl mx-auto px-4 sm:px-6 flex items-center justify-between h-14">
			<a href="/" class="font-semibold text-foreground text-lg tracking-tight flex items-center gap-2">
				<span class="inline-flex items-center justify-center w-7 h-7 rounded-md bg-[var(--accent)] text-[var(--accent-foreground)] text-xs font-bold">C</span>
				cora
			</a>

			<!-- Desktop nav -->
			<nav class="hidden sm:flex items-center gap-6">
				<a href="/docs" class="site-header-link" class:active={isDocs}>Docs</a>
				<a href="/docs/roadmap" class="site-header-link">Roadmap</a>
				<a href="https://github.com/codecoradev/cora-cli" target="_blank" rel="noopener" class="site-header-link">
					GitHub
				</a>
			</nav>

			<!-- Desktop CTA + theme toggle -->
			<div class="hidden sm:flex items-center gap-3">
				<button
					onclick={() => {
						const isDark = document.documentElement.classList.contains('dark');
						document.documentElement.classList.toggle('dark', !isDark);
						localStorage.setItem('mode', isDark ? 'light' : 'dark');
					}}
					class="p-2 rounded-md text-[var(--muted-foreground)] hover:text-[var(--foreground)] transition-colors"
					aria-label="Toggle theme"
				>
					<Sun class="w-4 h-4 hidden dark:block" />
					<Moon class="w-4 h-4 block dark:hidden" />
				</button>
				<a href="/docs/getting-started" class="btn-primary text-[13px] px-3.5 py-1.5">Get Started</a>
			</div>

			<!-- Mobile hamburger -->
			<div class="sm:hidden flex items-center gap-2">
				<button
					onclick={() => {
						const isDark = document.documentElement.classList.contains('dark');
						document.documentElement.classList.toggle('dark', !isDark);
						localStorage.setItem('mode', isDark ? 'light' : 'dark');
					}}
					class="p-2 rounded-md text-[var(--muted-foreground)]"
					aria-label="Toggle theme"
				>
					<Sun class="w-4 h-4 hidden dark:block" />
					<Moon class="w-4 h-4 block dark:hidden" />
				</button>
				<button
					onclick={() => mobileMenuOpen = !mobileMenuOpen}
					class="p-2 rounded-md text-[var(--muted-foreground)]"
					aria-label="Menu"
				>
					{#if mobileMenuOpen}
						<X class="w-5 h-5" />
					{:else}
						<Menu class="w-5 h-5" />
					{/if}
				</button>
			</div>
		</div>

		<!-- Mobile menu -->
		{#if mobileMenuOpen}
			<div class="sm:hidden border-t border-[var(--border)] bg-[var(--background)] px-4 pb-4 pt-2 space-y-1">
				<a href="/docs" class="block py-2 site-header-link" onclick={() => mobileMenuOpen = false}>Docs</a>
				<a href="/docs/roadmap" class="block py-2 site-header-link" onclick={() => mobileMenuOpen = false}>Roadmap</a>
				<a href="https://github.com/codecoradev/cora-cli" target="_blank" rel="noopener" class="block py-2 site-header-link">GitHub</a>
				<div class="pt-2">
					<a href="/docs/getting-started" class="btn-primary text-xs px-3 py-1.5" onclick={() => mobileMenuOpen = false}>Get Started</a>
				</div>
			</div>
		{/if}
	</header>

	<main>
		{@render children()}
	</main>
</div>
