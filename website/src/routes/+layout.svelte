<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';

	let { children } = $props();

	const isDocs = $derived($page.url.pathname.startsWith('/docs'));
</script>

<div class="bg-background min-h-screen">
	<!-- Header -->
	<header class="site-header">
		<div class="max-w-6xl mx-auto px-4 sm:px-6 flex items-center justify-between h-14">
			<a href="/" class="site-header-link font-semibold text-foreground">
				cora
			</a>

			<nav class="hidden sm:flex items-center gap-6">
				<a href="/docs" class="site-header-link" class:active={isDocs}>Docs</a>
				<a href="https://github.com/codecoradev/cora-cli" target="_blank" rel="noopener" class="site-header-link">
					GitHub
				</a>
			</nav>

			<!-- Desktop CTA -->
			<div class="hidden sm:block">
				<a href="/docs/getting-started" class="btn-primary text-[13px] px-3.5 py-1.5">Get Started</a>
			</div>

			<!-- Mobile-only CTA -->
			<div class="sm:hidden">
				<a href="/docs/getting-started" class="btn-primary text-xs px-3 py-1.5">Get Started</a>
			</div>
		</div>
	</header>

	<main>
		{@render children()}
	</main>
</div>
