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
				'Basic diff review with OpenAI',
				'CLI interface with review command',
				'Staged/unstaged file detection',
				'JSON output format',
			] as item}
				<div class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)]">
					<span class="text-sm">{item}</span>
					<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>
				</div>
			{/each}
		</div>
	</section>

	<!-- Completed v0.2.0 -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.2.0</span>
			Multi-Provider & Deterministic Rules
		</h2>
		<div class="space-y-3">
			{#each [
				'BYOK — Anthropic, Groq, Ollama support',
				'Deterministic rule engine (12 built-in)',
				'SARIF output format',
				'Branch review mode',
			] as item}
				<div class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)]">
					<span class="text-sm">{item}</span>
					<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>
				</div>
			{/each}
		</div>
	</section>

	<!-- Completed v0.3.0 -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.3.0</span>
			Scan Engine & Caching
		</h2>
		<div class="space-y-3">
			{#each [
				'Full project scan (cora scan)',
				'SHA256 diff-hash caching',
				'Incremental scan mode',
				'Cross-file context chain (5 languages)',
				'Pre-commit hooks (cora hook install)',
			] as item}
				<div class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)]">
					<span class="text-sm">{item}</span>
					<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>
				</div>
			{/each}
		</div>
	</section>

	<!-- Completed v0.4.0 -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.4.0</span>
			SARIF Upload & Polish
		</h2>
		<div class="space-y-3">
			{#each [
				'SARIF upload to GitHub Code Scanning',
				'Custom review/scan prompts',
				'Anti-hallucination filtering',
				'Shell completions (bash/zsh/fish)',
				'Z.AI provider support',
			] as item}
				<div class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)]">
					<span class="text-sm">{item}</span>
					<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>
				</div>
			{/each}
		</div>
	</section>

	<!-- Phase A — Near Term -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-[var(--accent)]/20 text-[var(--accent)]">Phase A</span>
			Ecosystem & Performance
		</h2>
		<p class="text-sm text-[var(--muted-foreground)] mb-4">Shipped on demand — prioritized by community requests.</p>
		<div class="space-y-3">
			{#each [
				{ title: 'Homebrew tap (brew install cora)', status: 'Planned' },
				{ title: 'Parallel file review (multi-threaded)', status: 'Planned' },
				{ title: 'Review summary (compact / detailed mode)', status: 'Planned' },
				{ title: 'GitLab CI/CD integration guide', status: 'Planned' },
			] as item}
				<div class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] opacity-70">
					<span class="text-sm">{item.title}</span>
					<span class="text-xs px-2 py-0.5 rounded-full bg-[var(--muted)] text-[var(--muted-foreground)]">{item.status}</span>
				</div>
			{/each}
		</div>
	</section>

	<!-- Phase B — Growth -->
	<section>
		<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
			<span class="px-2 py-0.5 rounded text-xs font-semibold bg-[var(--muted)] text-[var(--muted-foreground)]">Phase B</span>
			Editor & Team Features
		</h2>
		<p class="text-sm text-[var(--muted-foreground)] mb-4">Requires sufficient community adoption.</p>
		<div class="space-y-3">
			{#each [
				'VS Code extension (LSP-based)',
				'JetBrains plugin',
				'Team config (.cora.yaml inheritance)',
				'Custom rule authoring API',
			] as item}
				<div class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] opacity-50">
					<span class="text-sm">{item}</span>
					<span class="text-xs px-2 py-0.5 rounded-full bg-[var(--muted)] text-[var(--muted-foreground)]">Future</span>
				</div>
			{/each}
		</div>
	</section>

</div>
