<script lang="ts">
	import { onMount } from 'svelte';

	let currentSlide = $state(0);
	let mounted = $state(false);

	const slides = [
		{
			cmd: 'cora init',
			output: '✓ Created .cora.yaml in /home/dev/my-project\n✓ Default provider: auto-detect\n✓ Ready to review. Run: cora review --staged'
		},
		{
			cmd: 'cora review --staged',
			output: '🔍 Reviewing 3 files (142 additions, 23 deletions)...\n\n⚠️ src/auth/login.ts:45  — SQL injection risk in query builder\n⚠️ src/api/handler.ts:22 — Unhandled promise rejection\n✅ src/utils/format.ts:12  — Good error handling pattern\n\nFound 2 issues in 2.3s'
		},
		{
			cmd: 'cora review --branch feature/auth',
			output: '🔍 Comparing feature/auth → main (12 files, 847 changes)...\n\n❌ src/auth/oauth.ts:89   — Hardcoded client secret\n❌ src/auth/session.ts:34  — Session fixation vulnerability\n⚠️ src/auth/middleware.ts:12 — Missing rate limiting\n✅ src/auth/csrf.ts:8      — CSRF token implementation looks good\n\nFound 3 issues in 8.1s'
		},
		{
			cmd: 'cora scan --incremental',
			output: `⚡ Incremental scan — checking 4 modified files...\n\n✅ src/index.ts     — No issues\n✅ src/config.ts    — No issues\n⚠️ src/logger.ts:18 — Console.log left in production code\n✅ src/parser.ts    — No issues\n\n1 issue found in 1.2s (cached: 147 files)`
		},
		{
			cmd: 'cora hook install',
			output: '🪝 Installing pre-commit hook...\n✓ Hook installed to .git/hooks/pre-commit\n✓ Hook will run: cora review --staged --fail-on error\n\nNext commit will be automatically reviewed.'
		}
	];

	onMount(() => {
		mounted = true;
		const interval = setInterval(() => {
			currentSlide = (currentSlide + 1) % slides.length;
		}, 4000);
		return () => clearInterval(interval);
	});

	const features = [
		{
			icon: '🔍',
			title: 'AI Code Review',
			subtitle: 'Diff, branch, or full scan',
			description: 'Three review modes: staged diff (<3s), branch comparison, full project scan. LLM-powered analysis catches bugs, security issues, and style violations.'
		},
		{
			icon: '🔑',
			title: 'Bring Your Own Key',
			subtitle: 'No subscriptions, no lock-in',
			description: 'Uses YOUR OpenAI, Anthropic, Groq, Ollama, or Z.AI API key. No data stored on our servers. You control the model, you control the cost.'
		},
		{
			icon: '🪝',
			title: 'Pre-commit Hooks',
			subtitle: 'Review before you push',
			description: 'Install once: <code class="cmd-highlight">cora hook install</code>. Every commit gets reviewed automatically. Block bad code from entering your branch.'
		},
		{
			icon: '📊',
			title: 'SARIF Output',
			subtitle: 'GitHub Code Scanning integration',
			description: 'Upload review findings directly to GitHub\'s Security tab. Track issues across PRs. Works with any CI/CD pipeline.'
		},
		{
			icon: '⚡',
			title: 'Incremental Scan',
			subtitle: 'Only scan what changed',
			description: 'SHA256 content hash cache. First scan indexes your codebase. Subsequent scans only review new or modified files.'
		},
		{
			icon: '🏠',
			title: 'Self-Hosted',
			subtitle: 'Your code stays yours',
			description: 'Runs entirely on your machine. No cloud, no telemetry, no data leaving your network. Perfect for Gitea and air-gapped environments.'
		}
	];

	const quickStart = [
		{
			step: '1',
			label: 'Install',
			cmd: 'cargo install cora',
			desc: 'Single binary, no dependencies'
		},
		{
			step: '2',
			label: 'Initialize',
			cmd: 'cora init',
			desc: 'Creates .cora.yaml config'
		},
		{
			step: '3',
			label: 'Review',
			cmd: 'CORA_API_KEY=your-key cora review --staged',
			desc: 'Review your staged changes'
		},
		{
			step: '4',
			label: 'Done',
			cmd: null,
			desc: 'No account. No subscription. No cloud.'
		}
	];

	const comparisonRows = [
		['Install', '1 binary', 'GitHub App', 'Copilot Sub', 'Docker + DB'],
		['BYOK', '✅', '❌', '❌', 'N/A'],
		['Self-hosted', '✅', '❌', '❌', '✅'],
		['Gitea Support', '✅', '❌', '❌', '✅'],
		['Pre-commit Hook', '✅', '❌', '❌', '✅'],
		['CLI', '✅', '❌', '❌', '✅'],
		['SARIF', '✅', '✅', '✅', '✅'],
		['Cost', 'Free + API', '$12-24/user/mo', '$10-39/mo', 'Free/$150+'],
		['License', 'MIT', 'Apache 2.0', 'Proprietary', 'LGPL']
	];
</script>

<svelte:head>
	<title>cora — AI Code Review CLI</title>
	<meta name="description" content="cora is a CLI-first AI code reviewer. BYOK, zero config, runs in your terminal. Your code never leaves your machine." />
</svelte:head>

<div class="grid-bg min-h-screen">

	<!-- Hero Section -->
	<section class="relative pt-24 pb-20 px-6">
		<div class="max-w-4xl mx-auto text-center">
			<div class="animate-fade-in">
				<a
					href="https://github.com/nousresearch/cora-cli/releases"
					target="_blank"
					rel="noopener"
					class="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border border-[var(--color-border)] text-sm text-[var(--color-text-muted)] hover:border-[var(--color-border-light)] hover:text-[var(--color-text)] transition-all mb-8"
				>
					<span class="w-2 h-2 rounded-full bg-[var(--color-success)]"></span>
					v0.1.1 released
					<span class="text-[var(--color-text-dim)]">→</span>
				</a>
			</div>

			<h1 class="text-5xl md:text-7xl font-extrabold tracking-tight leading-tight mb-6 animate-fade-in-delay-1">
				Ship better code.<br />
				<span class="hero-gradient">Get it reviewed by AI.</span>
			</h1>

			<p class="text-lg md:text-xl text-[var(--color-text-muted)] max-w-2xl mx-auto mb-10 animate-fade-in-delay-2">
				cora is a CLI-first AI code reviewer. BYOK, zero config, runs in your terminal.
				Your code never leaves your machine.
			</p>

			<div class="animate-fade-in-delay-3">
				<div class="terminal-block max-w-lg mx-auto mb-8">
					<div class="flex items-center gap-2 mb-3">
						<span class="w-3 h-3 rounded-full bg-red-500/60"></span>
						<span class="w-3 h-3 rounded-full bg-yellow-500/60"></span>
						<span class="w-3 h-3 rounded-full bg-green-500/60"></span>
					</div>
					<div class="text-left">
						<span class="cmd-comment">$</span> <span class="cmd-highlight">cargo install</span> <span class="cmd-string">cora</span>
					</div>
				</div>

				<div class="flex flex-wrap justify-center gap-4">
					<a href="https://github.com/nousresearch/cora-cli" class="cta-primary" target="_blank" rel="noopener">
						<svg class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
						GitHub
					</a>
					<a href="/docs" class="cta-secondary">
						Documentation
						<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
					</a>
				</div>
			</div>
		</div>
	</section>

	<div class="separator-gradient max-w-6xl mx-auto"></div>

	<!-- Terminal Demo Section -->
	<section class="py-20 px-6">
		<div class="max-w-3xl mx-auto">
			<h2 class="text-2xl font-bold text-center mb-10 text-[var(--color-text-muted)]">See it in action</h2>
			<div class="terminal-block glow-amber">
				<div class="flex items-center gap-2 mb-4">
					<span class="w-3 h-3 rounded-full bg-red-500/60"></span>
					<span class="w-3 h-3 rounded-full bg-yellow-500/60"></span>
					<span class="w-3 h-3 rounded-full bg-green-500/60"></span>
					<span class="ml-4 text-xs text-[var(--color-text-dim)]">cora</span>
				</div>
				{#if mounted}
					{#key currentSlide}
						<div class="transition-all duration-300">
							<div class="mb-4">
								<span class="cmd-comment">$</span>
								<span class="cmd-highlight">{slides[currentSlide].cmd}</span>
							</div>
							<pre class="whitespace-pre-wrap text-[var(--color-text-muted)] text-xs leading-relaxed">{slides[currentSlide].output}</pre>
						</div>
					{/key}
				{/if}
				<div class="flex justify-center gap-2 mt-6">
					{#each slides as _, i}
						<button
							class="w-2 h-2 rounded-full transition-all {i === currentSlide ? 'bg-[var(--color-accent)] w-6' : 'bg-[var(--color-border)]'}"
							onclick={() => currentSlide = i}
							aria-label="Slide {i + 1}"
						></button>
					{/each}
				</div>
			</div>
		</div>
	</section>

	<div class="separator-gradient max-w-6xl mx-auto"></div>

	<!-- Problem → Solution Section -->
	<section class="py-20 px-6">
		<div class="max-w-5xl mx-auto">
			<h2 class="text-3xl md:text-4xl font-bold text-center mb-4">Your PRs ship with bugs<br/>you could have caught</h2>
			<p class="text-[var(--color-text-muted)] text-center mb-16 max-w-2xl mx-auto">Code review is the bottleneck. Manual reviews are slow, inconsistent, and don't scale.</p>

			<div class="grid md:grid-cols-2 gap-8">
				<!-- Before -->
				<div class="rounded-xl border border-[var(--color-border)] p-8">
					<div class="text-sm font-semibold text-[var(--color-error)] mb-4 uppercase tracking-wider">Before cora</div>
					<ul class="space-y-3 text-[var(--color-text-muted)]">
						<li class="flex items-start gap-3">
							<span class="text-[var(--color-error)] mt-1">✗</span>
							<span>Manual review — hours per PR</span>
						</li>
						<li class="flex items-start gap-3">
							<span class="text-[var(--color-error)] mt-1">✗</span>
							<span>Slow feedback — bugs found in production</span>
						</li>
						<li class="flex items-start gap-3">
							<span class="text-[var(--color-error)] mt-1">✗</span>
							<span>Inconsistent standards — every reviewer is different</span>
						</li>
						<li class="flex items-start gap-3">
							<span class="text-[var(--color-error)] mt-1">✗</span>
							<span>Review fatigue — important issues get missed</span>
						</li>
					</ul>
				</div>
				<!-- After -->
				<div class="rounded-xl border border-[var(--color-accent)]/30 bg-[var(--color-accent-dim)] p-8">
					<div class="text-sm font-semibold text-[var(--color-accent)] mb-4 uppercase tracking-wider">With cora</div>
					<ul class="space-y-3 text-[var(--color-text)]">
						<li class="flex items-start gap-3">
							<span class="text-[var(--color-success)] mt-1">✓</span>
							<span>Instant AI review — seconds per diff</span>
						</li>
						<li class="flex items-start gap-3">
							<span class="text-[var(--color-success)] mt-1">✓</span>
							<span>Catch bugs before they merge</span>
						</li>
						<li class="flex items-start gap-3">
							<span class="text-[var(--color-success)] mt-1">✓</span>
							<span>Consistent rules — same standards every time</span>
						</li>
						<li class="flex items-start gap-3">
							<span class="text-[var(--color-success)] mt-1">✓</span>
							<span>Pre-commit hooks — automatic review gate</span>
						</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<div class="separator-gradient max-w-6xl mx-auto"></div>

	<!-- Features Section -->
	<section class="py-20 px-6">
		<div class="max-w-6xl mx-auto">
			<h2 class="text-3xl md:text-4xl font-bold text-center mb-4">Everything you need,<br/>nothing you don't</h2>
			<p class="text-[var(--color-text-muted)] text-center mb-16 max-w-2xl mx-auto">Built for developers who value simplicity and control.</p>

			<div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
				{#each features as feature}
					<div class="feature-card">
						<div class="text-3xl mb-4">{feature.icon}</div>
						<h3 class="text-lg font-semibold mb-1">{feature.title}</h3>
						<p class="text-sm text-[var(--color-accent)] mb-3">{feature.subtitle}</p>
						<p class="text-sm text-[var(--color-text-muted)] leading-relaxed">{@html feature.description}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<div class="separator-gradient max-w-6xl mx-auto"></div>

	<!-- Quick Start Section -->
	<section class="py-20 px-6">
		<div class="max-w-4xl mx-auto">
			<h2 class="text-3xl md:text-4xl font-bold text-center mb-4">Start in 30 seconds</h2>
			<p class="text-[var(--color-text-muted)] text-center mb-16">No account required. No subscription. No cloud.</p>

			<div class="grid md:grid-cols-2 gap-6">
				{#each quickStart as item}
					<div class="flex gap-4 items-start">
						<div class="flex-shrink-0 w-10 h-10 rounded-lg bg-[var(--color-accent-dim)] border border-[var(--color-accent)]/30 flex items-center justify-center text-[var(--color-accent)] font-bold text-sm">
							{item.step}
						</div>
						<div class="flex-1">
							<div class="font-semibold mb-1">{item.label}</div>
							{#if item.cmd}
								<div class="terminal-block text-xs py-2 px-3 mt-2">
									<span class="cmd-comment">$</span> <span class="cmd-highlight">{item.cmd}</span>
								</div>
							{:else}
								<p class="text-[var(--color-text-muted)] text-sm mt-2">{item.desc}</p>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<div class="separator-gradient max-w-6xl mx-auto"></div>

	<!-- Comparison Table -->
	<section class="py-20 px-6">
		<div class="max-w-5xl mx-auto">
			<h2 class="text-3xl md:text-4xl font-bold text-center mb-4">How cora compares</h2>
			<p class="text-[var(--color-text-muted)] text-center mb-16 max-w-2xl mx-auto">Side-by-side with popular code review tools.</p>

			<div class="rounded-xl border border-[var(--color-border)] overflow-hidden overflow-x-auto">
				<table class="compare-table">
					<thead>
						<tr class="bg-[var(--color-surface)]">
							<th>Feature</th>
							<th class="highlight-col">cora</th>
							<th>CodeRabbit</th>
							<th>Copilot Review</th>
							<th>SonarQube</th>
						</tr>
					</thead>
					<tbody>
						{#each comparisonRows as row}
							<tr>
								{#each row as cell, i}
									<td class={i === 1 ? 'highlight-col' : ''}>{cell}</td>
								{/each}
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	</section>

	<div class="separator-gradient max-w-6xl mx-auto"></div>

	<!-- Architecture Section -->
	<section class="py-20 px-6">
		<div class="max-w-5xl mx-auto">
			<h2 class="text-3xl md:text-4xl font-bold text-center mb-16">How it works</h2>

			<div class="flex flex-col md:flex-row items-center justify-center gap-4 mb-12">
				<!-- Developer -->
				<div class="rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-6 py-4 text-center">
					<div class="text-2xl mb-2">👨‍💻</div>
					<div class="font-semibold text-sm">Developer</div>
				</div>
				<!-- Arrow -->
				<div class="text-[var(--color-accent)] text-2xl rotate-90 md:rotate-0">→</div>
				<!-- cora CLI -->
				<div class="rounded-xl border border-[var(--color-accent)]/30 bg-[var(--color-accent-dim)] px-6 py-4 text-center glow-amber">
					<div class="text-2xl mb-2">🔧</div>
					<div class="font-semibold text-sm text-[var(--color-accent)]">cora CLI</div>
				</div>
				<!-- Arrow -->
				<div class="text-[var(--color-accent)] text-2xl rotate-90 md:rotate-0">→</div>
				<!-- LLM API -->
				<div class="rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-6 py-4 text-center">
					<div class="text-2xl mb-2">🧠</div>
					<div class="font-semibold text-sm">LLM API</div>
					<div class="text-xs text-[var(--color-text-dim)] mt-1">OpenAI · Anthropic · Groq · Ollama</div>
				</div>
				<!-- Arrow -->
				<div class="text-[var(--color-accent)] text-2xl rotate-90 md:rotate-0">→</div>
				<!-- Results -->
				<div class="rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-6 py-4 text-center">
					<div class="text-2xl mb-2">📋</div>
					<div class="font-semibold text-sm">Review Results</div>
				</div>
			</div>

			<!-- Storage -->
			<div class="flex justify-center mb-8">
				<div class="terminal-block text-sm py-3 px-4">
					<span class="cmd-comment">~/.cora/</span>
					<span class="text-[var(--color-text-dim)]">→ config.toml · cache · reviews</span>
				</div>
			</div>

			<!-- Tech badges -->
			<div class="flex flex-wrap justify-center gap-3">
				{#each ['Rust', 'Tokio', 'Git2', 'Clap', 'MIT'] as badge}
					<span class="px-3 py-1 rounded-full border border-[var(--color-border)] text-xs text-[var(--color-text-muted)]">{badge}</span>
				{/each}
			</div>
		</div>
	</section>

	<div class="separator-gradient max-w-6xl mx-auto"></div>

	<!-- CTA Section -->
	<section class="py-24 px-6">
		<div class="max-w-3xl mx-auto text-center">
			<h2 class="text-4xl md:text-5xl font-extrabold mb-6">Ready to ship better code?</h2>
			<p class="text-[var(--color-text-muted)] text-lg mb-10">Open source · MIT · Start in 30 seconds</p>
			<div class="flex flex-wrap justify-center gap-4">
				<a href="https://github.com/nousresearch/cora-cli" class="cta-primary" target="_blank" rel="noopener">
					<svg class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/></svg>
					Star on GitHub
				</a>
				<a href="/docs" class="cta-secondary">
					Read the Docs
					<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M5 12h14M12 5l7 7-7 7"/></svg>
				</a>
			</div>
		</div>
	</section>

	<!-- Footer -->
	<footer class="border-t border-[var(--color-border)] py-8 px-6">
		<div class="max-w-6xl mx-auto flex flex-col md:flex-row items-center justify-between gap-4 text-sm text-[var(--color-text-dim)]">
			<span>cora · MIT License · Built with Rust</span>
			<div class="flex items-center gap-6">
				<a href="https://github.com/nousresearch/cora-cli" class="text-[var(--color-text-dim)] hover:text-[var(--color-text)]" target="_blank" rel="noopener">GitHub</a>
				<a href="/docs" class="text-[var(--color-text-dim)] hover:text-[var(--color-text)]">Docs</a>
			</div>
		</div>
	</footer>
</div>
