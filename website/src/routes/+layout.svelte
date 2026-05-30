<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';

	let { children } = $props();

	const isDocs = $derived($page.url.pathname.startsWith('/docs'));
</script>

<div style="background: var(--background); min-height: 100vh;">
	<!-- Header -->
	<header class="site-header">
		<div class="max-w-6xl mx-auto px-6 flex items-center justify-between" style="height: 3.5rem;">
			<a href="/" class="site-header-link" style="font-weight: 600; color: var(--foreground);">
				cora
			</a>

			<nav class="hidden sm:flex items-center gap-6">
				<a href="/docs" class="site-header-link" class:active={isDocs}>Docs</a>
				<a href="https://github.com/ajianaz/cora-cli" target="_blank" rel="noopener" class="site-header-link">
					GitHub
				</a>
			</nav>

			<div class="hidden sm:block">
				<a href="/docs/getting-started" class="btn-primary" style="font-size: 13px; padding: 0.375rem 0.875rem;">Get Started</a>
			</div>
		</div>
	</header>

	<main>
		{@render children()}
	</main>
</div>
