<svelte:head>
	<title>Roadmap — cora Docs</title>
</svelte:head>

<h1 class="text-3xl font-bold mb-6">Roadmap</h1>

<p class="text-[var(--muted-foreground)] mb-10">Demand-gated — we build what people actually need. Track progress on <a href="https://github.com/codecoradev/cora-cli/issues" target="_blank" rel="noopener" class="text-[var(--accent)] hover:underline">GitHub Issues</a>.</p>

<div class="space-y-10">

	<!-- Completed v0.1.5 -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.1.5</span>
			Initial Release
		</h2>
		<div class="space-y-3">
			{#each [
				{ title: 'Basic diff review with OpenAI', issue: 90 },
				{ title: 'JSON response repair & unicode handling', issue: 89 },
				{ title: 'CLI interface with review command', issue: 90 },
			] as item}
				<a
					href="https://github.com/codecoradev/cora-cli/issues/{item.issue}"
					target="_blank"
					rel="noopener"
					class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"
				>
					<div class="flex items-center gap-3">
						<span class="text-xs text-[var(--muted-foreground)] font-mono">#{item.issue}</span>
						<span class="text-sm">{item.title}</span>
					</div>
					<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>
				</a>
			{/each}
		</div>
	</section>

	<!-- Completed v0.1.6 -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.1.6</span>
			Custom Prompts & Path Injection
		</h2>
		<div class="space-y-3">
			{#each [
				{ title: 'Enhanced default system prompts', issue: 95 },
				{ title: 'Custom system prompt via .cora.yaml config', issue: 94 },
				{ title: 'Inject valid file paths into system prompt', issue: 93 },
				{ title: 'JSON object response format (opt-in)', issue: 92 },
			] as item}
				<a
					href="https://github.com/codecoradev/cora-cli/issues/{item.issue}"
					target="_blank"
					rel="noopener"
					class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"
				>
					<div class="flex items-center gap-3">
						<span class="text-xs text-[var(--muted-foreground)] font-mono">#{item.issue}</span>
						<span class="text-sm">{item.title}</span>
					</div>
					<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>
				</a>
			{/each}
		</div>
	</section>

	<!-- Completed v0.1.7 -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.1.7</span>
			Deterministic & Reliable
		</h2>
		<div class="space-y-3">
			{#each [
				{ title: 'Deterministic reviews — temperature=0', issue: 98 },
				{ title: 'Non-deterministic output bug fix', issue: 97 },
				{ title: 'HTTP timeout + connection pooling', issue: 99 },
				{ title: 'Diff-hash caching for repeat reviews', issue: 100 },
				{ title: 'Configurable max_tokens', issue: 101 },
			] as item}
				<a
					href="https://github.com/codecoradev/cora-cli/issues/{item.issue}"
					target="_blank"
					rel="noopener"
					class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"
				>
					<div class="flex items-center gap-3">
						<span class="text-xs text-[var(--muted-foreground)] font-mono">#{item.issue}</span>
						<span class="text-sm">{item.title}</span>
					</div>
					<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>
				</a>
			{/each}
		</div>
	</section>

	<!-- Completed v0.2.0 -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.2.0</span>
			Multi-Provider & SARIF
		</h2>
		<div class="space-y-3">
			{#each [
				{ title: 'BYOK — Anthropic, Groq, Ollama support', issue: 106 },
				{ title: 'SARIF output format', issue: 106 },
				{ title: 'Branch review mode', issue: 106 },
				{ title: 'Output footer watermark', issue: 106 },
			] as item}
				<a
					href="https://github.com/codecoradev/cora-cli/issues/{item.issue}"
					target="_blank"
					rel="noopener"
					class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"
				>
					<div class="flex items-center gap-3">
						<span class="text-xs text-[var(--muted-foreground)] font-mono">#{item.issue}</span>
						<span class="text-sm">{item.title}</span>
					</div>
					<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>
				</a>
			{/each}
		</div>
	</section>

	<!-- Completed v0.3 -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.3</span>
			Progress & CI Hardening
		</h2>
		<div class="space-y-3">
			{#each [
				{ title: 'Static analysis context injection (reduce false positives)', issue: 140 },
				{ title: '--progress flag for machine-readable output', issue: 108 },
				{ title: 'Composite action crash fix (KeyError)', issue: 102 },
				{ title: 'Config validate command', issue: 88 },
			] as item}
				<a
					href="https://github.com/codecoradev/cora-cli/issues/{item.issue}"
					target="_blank"
					rel="noopener"
					class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"
				>
					<div class="flex items-center gap-3">
						<span class="text-xs text-[var(--muted-foreground)] font-mono">#{item.issue}</span>
						<span class="text-sm">{item.title}</span>
					</div>
					<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>
				</a>
			{/each}
		</div>
	</section>

	<!-- Completed v0.4 -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.4</span>
			Deterministic Engine Pipeline
		</h2>
		<div class="space-y-3">
			{#each [
				{ title: 'Deterministic rule engine — 12 built-in rules', issue: 116 },
				{ title: 'File bundling — parallel per-bundle review', issue: 115 },
				{ title: 'AST-based cross-file dependency extraction', issue: 114 },
				{ title: 'Hunk header regex panic fix + 5MB diff support', issue: 159 },
			] as item}
				<a
					href="https://github.com/codecoradev/cora-cli/issues/{item.issue}"
					target="_blank"
					rel="noopener"
					class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"
				>
					<div class="flex items-center gap-3">
						<span class="text-xs text-[var(--muted-foreground)] font-mono">#{item.issue}</span>
						<span class="text-sm">{item.title}</span>
					</div>
					<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>
				</a>
			{/each}
		</div>
	</section>

	<!-- In Progress v0.5 -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-blue-500/20 text-blue-400">v0.5</span>
			Install & Distribution
		</h2>
		<div class="space-y-3">
			{#each [
				{ title: 'Easy install — Homebrew tap & install script', issue: 151, status: 'done' },
				{ title: 'CI gate mode — block PRs on review findings', issue: 149, status: 'done' },
				{ title: 'Website redesign — landing page + docs', issue: 163, status: 'done' },
				{ title: 'Interactive auth login with tiered provider selection', issue: 172, status: 'done' },
				{ title: 'README overhaul — market-facing copy', issue: 162, status: 'planned' },
			] as item}
				<a
					href="https://github.com/codecoradev/cora-cli/issues/{item.issue}"
					target="_blank"
					rel="noopener"
					class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"
				>
					<div class="flex items-center gap-3">
						<span class="text-xs text-[var(--muted-foreground)] font-mono">#{item.issue}</span>
						<span class="text-sm">{item.title}</span>
					</div>
					{#if item.status === 'done'}
						<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>
					{:else}
						<span class="text-xs px-2 py-0.5 rounded-full bg-yellow-500/20 text-yellow-400">◎ Planned</span>
					{/if}
				</a>
			{/each}
		</div>
	</section>

	<!-- Future -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-purple-500/20 text-purple-400">Future</span>
			What's Next
		</h2>
		<div class="space-y-3">
			{#each [
				{ title: 'Lightweight agent follow-up — 1 capped tool-call', issue: 117 },
				{ title: '`cora gain` — local productivity stats + viral sharing', issue: 161 },
				{ title: 'GitHub App backend MVP in Rust (Axum)', issue: 132 },
				{ title: 'Publish cora-review as GitHub Marketplace action', issue: 47 },
			] as item}
				<a
					href="https://github.com/codecoradev/cora-cli/issues/{item.issue}"
					target="_blank"
					rel="noopener"
					class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"
				>
					<div class="flex items-center gap-3">
						<span class="text-xs text-[var(--muted-foreground)] font-mono">#{item.issue}</span>
						<span class="text-sm">{item.title}</span>
					</div>
					<span class="text-xs px-2 py-0.5 rounded-full bg-purple-500/20 text-purple-400">→ Planned</span>
				</a>
			{/each}
		</div>
	</section>

</div>
