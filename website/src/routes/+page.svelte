<script lang="ts">
	import { onMount } from 'svelte';

	interface Feature {
		icon: string;
		title: string;
		desc: string;
		detail: string;
	}

	interface Comparison {
		feature: string;
		cora: string;
		coderabbit: string;
		copilot: string;
		sonarqube: string;
	}

	interface Command {
		cmd: string;
		desc: string;
		output: string;
	}

	const features: Feature[] = [
		{
			icon: '🔑',
			title: 'BYOK Any LLM',
			desc: 'Zero lock-in',
			detail: 'OpenAI, Anthropic, Groq, local models — any OpenAI-compatible endpoint. You control the model, you control the cost.'
		},
		{
			icon: '🛡️',
			title: 'Deterministic Rules Engine',
			desc: 'Zero token cost',
			detail: 'Zero-cost regex rules catch secrets, SQL injection, TLS issues BEFORE the LLM call. No tokens wasted on obvious bugs.'
		},
		{
			icon: '🔗',
			title: 'Cross-file Context Chain',
			desc: 'Full codebase awareness',
			detail: 'AST-based symbol extraction understands your codebase, not just the diff. Follows imports, resolves types.'
		},
		{
			icon: '📦',
			title: 'File Bundling',
			desc: 'Smart grouping',
			detail: 'Smart grouping of related files for consistent review across large PRs. Reduces context window waste.'
		},
		{
			icon: '⚡',
			title: 'Pre-commit Hooks',
			desc: 'Before you push',
			detail: 'Automatic review on every git commit. Blocks bad code before it\'s pushed. Zero friction, maximum impact.'
		},
		{
			icon: '📊',
			title: 'SARIF + CI Integration',
			desc: 'GitHub native',
			detail: 'GitHub Actions composite action, SARIF upload, PR comments, blocking on error. Works in any CI/CD pipeline.'
		}
	];

	const comparisons: Comparison[] = [
		{ feature: 'Pricing', cora: '✅ Free (MIT)', coderabbit: '❌ $12/mo+', copilot: '❌ $10/mo+', sonarqube: '❌ $150+/mo' },
		{ feature: 'BYOK', cora: '✅ Any LLM', coderabbit: '❌ No', copilot: '❌ No', sonarqube: '❌ No' },
		{ feature: 'Pre-commit hook', cora: '✅ Native', coderabbit: '❌ PR-only', copilot: '❌ PR-only', sonarqube: '❌ CI-only' },
		{ feature: 'CLI-first', cora: '✅ Local', coderabbit: '❌ Cloud', copilot: '❌ Cloud', sonarqube: '❌ CI' },
		{ feature: 'Zero telemetry', cora: '✅', coderabbit: '❌ Cloud', copilot: '❌ Cloud', sonarqube: '❌ Cloud' },
		{ feature: 'Rules engine', cora: '✅ Deterministic', coderabbit: '❌ LLM-only', copilot: '❌ LLM-only', sonarqube: '❌ Static only' },
		{ feature: 'Context chain', cora: '✅ Cross-file', coderabbit: '⚠️ Diff-only', copilot: '⚠️ Diff-only', sonarqube: '❌ No' },
		{ feature: 'Privacy', cora: '✅ Local-first', coderabbit: '❌ Cloud', copilot: '❌ Cloud', sonarqube: '❌ Cloud' }
	];

	const commands: Command[] = [
		{
			cmd: 'cora review --staged',
			desc: 'Review staged changes before commit',
			output: '🛡️  Rules engine: 3 files scanned\n    ✖ src/api/auth.ts:42  — hardcoded API key detected\n    ✖ src/db/query.ts:18  — potential SQL injection\n\n🤖  LLM review: GPT-4o-mini\n    ⚠ src/api/auth.ts:67  — missing error boundary\n\n✓ Review complete — 3 issues found'
		},
		{
			cmd: 'cora review --pr 42',
			desc: 'Review an open pull request',
			output: 'Fetching PR #42 diff... (12 files)\n🔗  Context chain: resolving 8 imports\n\n✓ Found 2 issues + 1 suggestion\n✓ SARIF uploaded to GitHub'
		},
		{
			cmd: 'cora config set llm.model gpt-4o',
			desc: 'Switch to a different model',
			output: '✓ LLM model set to gpt-4o\n✓ Config saved to .cora.yaml'
		},
		{
			cmd: 'cora review --full',
			desc: 'Full codebase review',
			output: 'Scanning 247 files...\n🛡️  Rules: 5 violations (secrets, SQL)\n🤖  LLM: 12 issues across 8 files\n\n✓ Report written to .cora/review.json'
		}
	];

	let visibleCmd = $state(0);
	let openFaq = $state<number | null>(null);

	const faqs = [
		{ q: 'What is cora?', a: 'cora is a CLI-first AI code review tool. It reviews your code before you commit using deterministic rules + LLM analysis, runs entirely locally, and uploads SARIF results to GitHub.' },
		{ q: 'What providers are supported?', a: 'Any OpenAI-compatible endpoint — OpenAI, Anthropic, Groq, local models via Ollama, LiteLLM proxy, and more. You bring your own key.' },
		{ q: 'Does cora work offline?', a: 'The rules engine works fully offline with zero API calls. LLM review requires network access to your chosen provider.' },
		{ q: 'How is this different from CodeRabbit?', a: 'cora runs locally as a CLI, supports BYOK (use any LLM), has deterministic rules that catch issues before LLM review, and is free/open source (MIT). CodeRabbit is cloud-only with fixed pricing.' },
		{ q: 'What is the rules engine?', a: 'A zero-cost regex-based scanner that catches secrets, SQL injection, TLS issues, and common anti-patterns BEFORE the LLM call. No tokens wasted on obvious bugs.' },
		{ q: 'Can I use cora in CI?', a: 'Yes. GitHub Actions composite action, SARIF upload, PR comments, and blocking on error. Works in any CI/CD pipeline.' },
		{ q: 'Is it really free?', a: 'Yes. MIT licensed, fully open source. No usage limits, no account required. You only pay for your own LLM API usage.' }
	];

	onMount(() => {
		const interval = setInterval(() => {
			visibleCmd = (visibleCmd + 1) % commands.length;
		}, 4000);

		return () => clearInterval(interval);
	});
</script>

<svelte:head>
	<title>cora — AI Code Review CLI · Open Source · MIT</title>
	<meta name="description" content="CLI-first AI code review. Runs locally, BYOK any LLM, deterministic rules engine, pre-commit hooks. Free, open source, MIT license." />
	<meta property="og:title" content="cora — AI Code Review CLI" />
	<meta property="og:description" content="Ship with confidence. AI code review before you commit. CLI-first, BYOK, deterministic rules." />
	<meta property="og:type" content="website" />
	<meta property="og:image" content="/og.png" />
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content="cora — AI Code Review CLI" />
	<meta name="twitter:description" content="Ship with confidence. AI code review before you commit." />
	<meta name="twitter:image" content="/og.png" />
</svelte:head>

<!-- Hero -->
<section class="grid-bg relative overflow-hidden">
	<div class="absolute inset-0 bg-gradient-to-b from-[oklch(0.45_0.12_270_/_0.08)] to-transparent pointer-events-none"></div>

	<div class="relative max-w-4xl mx-auto px-6 pt-24 md:pt-36 pb-20 text-center">
		<!-- Badge -->
		<div class="animate-fade-in inline-flex items-center gap-2 px-3 py-1.5 rounded-full border border-[var(--border)] bg-[var(--card)] text-xs text-[var(--muted-foreground)] mb-8">
			<span class="w-2 h-2 rounded-full bg-[var(--success)]"></span>
			<span>v0.4 released</span>
			<span class="text-[var(--border)]">—</span>
			<a href="https://github.com/codecoradev/cora-cli/releases" target="_blank" rel="noopener" class="text-[var(--accent)] hover:underline">release notes →</a>
		</div>

		<!-- Headline -->
		<h1 class="animate-fade-in-delay-1 text-4xl sm:text-5xl md:text-7xl font-bold tracking-tight mb-6 leading-[1.1]">
			Ship with<br />
			<span class="hero-gradient glow-text">Confidence.</span>
		</h1>

		<!-- Subheadline -->
		<p class="animate-fade-in-delay-2 text-base md:text-xl text-[var(--muted-foreground)] max-w-2xl mx-auto mb-10 leading-relaxed">
			AI code review, <strong class="text-[var(--foreground)]">before you commit.</strong><br class="hidden md:block" />
			CLI-first, BYOK any LLM, deterministic rules, fully open source.
		</p>

		<!-- Install command -->
		<div class="animate-fade-in-delay-3 inline-flex flex-col items-center gap-4">
			<div class="terminal-block inline-flex items-center gap-3 px-5 py-3.5 rounded-xl font-mono text-sm glow-purple">
				<span class="text-[var(--success)]">$</span>
				<span class="text-[var(--foreground)]">curl -fsSL https://cora.dev/install.sh | sh</span>
				<button
					class="text-[var(--muted-foreground)] hover:text-[var(--accent)] transition-colors ml-2"
					onclick={() => navigator.clipboard.writeText('curl -fsSL https://cora.dev/install.sh | sh')}
					title="Copy"
				>
					<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
						<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
					</svg>
				</button>
			</div>

			<div class="flex items-center gap-4 text-sm">
				<span class="text-[var(--muted-foreground)]">MIT License</span>
				<span class="text-[var(--border)]">·</span>
				<span class="text-[var(--muted-foreground)]">No account</span>
				<span class="text-[var(--border)]">·</span>
				<span class="text-[var(--muted-foreground)]">No telemetry</span>
			</div>

			<div class="flex items-center gap-3 mt-2">
				<a
					href="https://github.com/codecoradev/cora-cli"
					target="_blank"
					rel="noopener"
					class="cta-primary inline-flex items-center gap-2 px-5 py-2.5 rounded-lg bg-[var(--accent)] text-[var(--accent-foreground)] font-semibold text-sm"
				>
					<svg class="w-4 h-4" viewBox="0 0 16 16" fill="currentColor"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8z" /></svg>
					GitHub
				</a>
				<a
					href="/docs"
					class="cta-secondary inline-flex items-center gap-2 px-5 py-2.5 rounded-lg border border-[var(--border)] text-[var(--muted-foreground)] font-medium text-sm"
				>
					Install & Get Started
				</a>
			</div>
		</div>
	</div>
</section>

<!-- Terminal Demo -->
<section class="max-w-4xl mx-auto px-6 -mt-4 pb-16 md:pb-24">
	<div class="terminal-block rounded-2xl overflow-hidden glow-purple">
		<!-- Title bar -->
		<div class="flex items-center gap-2 px-4 py-3 border-b border-[var(--border)] bg-[var(--card)]">
			<div class="flex gap-1.5">
				<div class="w-3 h-3 rounded-full bg-red-500/60"></div>
				<div class="w-3 h-3 rounded-full bg-yellow-500/60"></div>
				<div class="w-3 h-3 rounded-full bg-green-500/60"></div>
			</div>
			<span class="text-xs text-[var(--muted-foreground)] ml-2 font-mono">cora — AI code review engine</span>
		</div>

		<!-- Terminal content -->
		<div class="p-5 md:p-6 font-mono text-sm space-y-3 min-h-[200px]">
			{#each commands as c, i}
				<div class="transition-all duration-300 {i === visibleCmd ? 'opacity-100' : 'opacity-25'}">
					{#if i === visibleCmd}
						<div class="flex items-start gap-2">
							<span class="text-[var(--success)] select-none">❯</span>
							<div>
								<span class="cmd-highlight">{c.cmd.split(' ')[0]}</span>
								<span class="text-[var(--foreground)]"> {c.cmd.split(' ').slice(1).join(' ')}</span>
							</div>
						</div>
						<p class="text-[var(--muted-foreground)] ml-5 mt-0.5 text-xs"># {c.desc}</p>
						<div class="ml-5 mt-2 text-[var(--muted-foreground)] text-xs whitespace-pre-line">{c.output}</div>
					{:else}
						<div class="flex items-start gap-2">
							<span class="text-[var(--muted-foreground)] select-none">❯</span>
							<span class="text-[var(--muted-foreground)]">{c.cmd}</span>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	</div>
</section>

<!-- Social Proof -->
<section class="max-w-4xl mx-auto px-6 py-10 text-center">
	<div class="flex flex-wrap items-center justify-center gap-6 text-[var(--muted-foreground)]">
		<div class="flex items-center gap-2">
			<span class="text-xl">🦀</span>
			<span class="text-sm">Built with Rust</span>
		</div>
		<div class="flex items-center gap-2">
			<span class="text-xl">🔒</span>
			<span class="text-sm">Zero Telemetry</span>
		</div>
		<div class="flex items-center gap-2">
			<span class="text-xl">⚡</span>
			<span class="text-sm">Pre-commit Hooks</span>
		</div>
		<div class="flex items-center gap-2">
			<span class="text-xl">🛡️</span>
			<span class="text-sm">Rules Engine</span>
		</div>
		<div class="flex items-center gap-2">
			<span class="text-xl">🔑</span>
			<span class="text-sm">BYOK Any LLM</span>
		</div>
		<div class="flex items-center gap-2">
			<span class="text-xl">📦</span>
			<span class="text-sm">Single Binary</span>
		</div>
	</div>
</section>

<div class="separator-gradient"></div>

<!-- Problem → Solution -->
<section class="max-w-4xl mx-auto px-6 py-16 md:py-24">
	<div class="max-w-3xl mx-auto text-center">
		<p class="text-sm font-medium text-[var(--accent)] mb-4 tracking-wide uppercase">The Problem</p>
		<h2 class="text-2xl md:text-4xl font-bold mb-6">
			Code review shouldn't be<br />
			<span class="hero-gradient">slow or expensive</span>
		</h2>
		<p class="text-base md:text-lg text-[var(--muted-foreground)] leading-relaxed mb-12">
			Stop waiting hours for feedback. cora catches issues in seconds — <strong class="text-[var(--foreground)]">before you commit.</strong>
		</p>
	</div>

	<div class="max-w-3xl mx-auto">
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<!-- Before -->
			<div class="terminal-block rounded-xl p-5">
				<p class="text-xs font-medium text-red-400 mb-3">❌ Without cora</p>
				<div class="font-mono text-xs space-y-2 text-[var(--muted-foreground)]">
					<p>> Waiting for PR review...</p>
					<p class="text-red-400/70">Still waiting (4 hours)</p>
					<p>> Reviewer missed the SQL injection</p>
					<p class="text-red-400/70">Merged to production with bug</p>
					<p>> Now paying $12/mo per reviewer seat</p>
				</div>
			</div>

			<!-- After -->
			<div class="terminal-block rounded-xl p-5" style="border-color: var(--accent-dim);">
				<p class="text-xs font-medium text-[var(--success)] mb-3">✅ With cora</p>
				<div class="font-mono text-xs space-y-2">
					<p class="text-[var(--muted-foreground)]">> git commit -m "feat: auth"</p>
					<p class="text-[var(--success)]">cora review: 2 issues found (0.8s)</p>
					<p class="text-[var(--muted-foreground)]">> Fixed both issues, committed</p>
					<p class="text-[var(--success)]">Zero cost. Zero waiting. Zero bugs shipped.</p>
				</div>
			</div>
		</div>
	</div>
</section>

<div class="separator-gradient"></div>

<!-- Features -->
<section class="max-w-4xl mx-auto px-6 py-16 md:py-24">
	<div class="text-center mb-14">
		<p class="text-sm font-medium text-[var(--accent)] mb-4 tracking-wide uppercase">Features</p>
		<h2 class="text-2xl md:text-4xl font-bold">Everything you need.<br />Nothing you don't.</h2>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
		{#each features as feat}
			<div class="feature-card p-6 rounded-xl border border-[var(--border)] bg-[var(--card)]">
				<span class="text-2xl">{feat.icon}</span>
				<h3 class="text-lg font-semibold mt-3 mb-1">{feat.title}</h3>
				<p class="text-sm text-[var(--accent)] mb-2">{feat.desc}</p>
				<p class="text-sm text-[var(--muted-foreground)] leading-relaxed">{feat.detail}</p>
			</div>
		{/each}
	</div>
</section>

<div class="separator-gradient"></div>

<!-- Quick Start -->
<section class="max-w-4xl mx-auto px-6 py-16 md:py-24">
	<div class="text-center mb-14">
		<p class="text-sm font-medium text-[var(--accent)] mb-4 tracking-wide uppercase">Get Started</p>
		<h2 class="text-2xl md:text-4xl font-bold">Three steps to<br />confident commits</h2>
	</div>

	<div class="max-w-2xl mx-auto space-y-6">
		<div class="flex gap-4">
			<div class="flex-shrink-0 w-8 h-8 rounded-full bg-[var(--accent-dim)] flex items-center justify-center text-sm font-bold text-[var(--accent-bright)]">1</div>
			<div class="min-w-0 flex-1">
				<p class="font-medium mb-2">Install</p>
				<code class="block px-4 py-3 rounded-lg bg-[var(--card)] border border-[var(--border)] text-sm font-mono text-[var(--muted-foreground)] overflow-x-auto">
					<span class="cmd-highlight">curl</span> -fsSL https://cora.dev/install.sh <span class="cmd-flag">|</span> sh
				</code>
			</div>
		</div>
		<div class="flex gap-4">
			<div class="flex-shrink-0 w-8 h-8 rounded-full bg-[var(--accent-dim)] flex items-center justify-center text-sm font-bold text-[var(--accent-bright)]">2</div>
			<div class="min-w-0 flex-1">
				<p class="font-medium mb-2">Configure your LLM</p>
				<code class="block px-4 py-3 rounded-lg bg-[var(--card)] border border-[var(--border)] text-sm font-mono text-[var(--muted-foreground)] overflow-x-auto">
					<span class="cmd-highlight">cora</span> init
				</code>
			</div>
		</div>
		<div class="flex gap-4">
			<div class="flex-shrink-0 w-8 h-8 rounded-full bg-[var(--accent-dim)] flex items-center justify-center text-sm font-bold text-[var(--accent-bright)]">3</div>
			<div class="min-w-0 flex-1">
				<p class="font-medium mb-2">Review your code</p>
				<code class="block px-4 py-3 rounded-lg bg-[var(--card)] border border-[var(--border)] text-sm font-mono text-[var(--muted-foreground)] overflow-x-auto">
					<span class="cmd-highlight">cora</span> review <span class="cmd-flag">--staged</span>
				</code>
			</div>
		</div>
		<div class="flex gap-4">
			<div class="flex-shrink-0 w-8 h-8 rounded-full bg-[var(--accent-dim)] flex items-center justify-center text-sm font-bold text-[var(--accent)]">✓</div>
			<div class="min-w-0 flex-1">
				<p class="font-medium text-[var(--success)]">That's it. No account. No server. No lock-in.</p>
				<p class="text-sm text-[var(--muted-foreground)] mt-1">Works out of the box with sensible defaults. Customize with <code class="font-mono">.cora.yaml</code> when you need more control.</p>
			</div>
		</div>
	</div>
</section>

<div class="separator-gradient"></div>

<!-- Comparison -->
<section class="max-w-4xl mx-auto px-6 py-16 md:py-24">
	<div class="text-center mb-14">
		<p class="text-sm font-medium text-[var(--accent)] mb-4 tracking-wide uppercase">Compare</p>
		<h2 class="text-2xl md:text-4xl font-bold">Why developers choose cora</h2>
	</div>

	<div class="overflow-x-auto rounded-xl border border-[var(--border)]">
		<table class="compare-table w-full text-sm">
			<thead>
				<tr class="border-b border-[var(--border)]">
					<th class="text-left px-4 py-3 font-medium text-[var(--muted-foreground)]">Feature</th>
					<th class="highlight-col text-center px-4 py-3 font-semibold text-[var(--accent)]">cora</th>
					<th class="text-center px-4 py-3 font-medium text-[var(--muted-foreground)] hidden sm:table-cell">CodeRabbit</th>
					<th class="text-center px-4 py-3 font-medium text-[var(--muted-foreground)] hidden md:table-cell">Copilot</th>
					<th class="text-center px-4 py-3 font-medium text-[var(--muted-foreground)] hidden lg:table-cell">SonarQube</th>
				</tr>
			</thead>
			<tbody>
				{#each comparisons as row}
					<tr class="border-b border-[var(--border-subtle)] hover:bg-[var(--card)] transition-colors">
						<td class="px-4 py-3 text-[var(--muted-foreground)]">{row.feature}</td>
						<td class="highlight-col px-4 py-3 text-center font-medium">{row.cora}</td>
						<td class="px-4 py-3 text-center text-[var(--muted-foreground)] hidden sm:table-cell">{row.coderabbit}</td>
						<td class="px-4 py-3 text-center text-[var(--muted-foreground)] hidden md:table-cell">{row.copilot}</td>
						<td class="px-4 py-3 text-center text-[var(--muted-foreground)] hidden lg:table-cell">{row.sonarqube}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</section>

<div class="separator-gradient"></div>

<!-- FAQ -->
<section class="max-w-4xl mx-auto px-6 py-16 md:py-24">
	<div class="text-center mb-14">
		<p class="text-sm font-medium text-[var(--accent)] mb-4 tracking-wide uppercase">FAQ</p>
		<h2 class="text-2xl md:text-4xl font-bold">Frequently asked questions</h2>
	</div>

	<div class="max-w-2xl mx-auto space-y-3">
		{#each faqs as faq, i}
			<div
				class="feature-card w-full text-left px-5 py-4 rounded-xl border border-[var(--border)] bg-[var(--card)] cursor-pointer"
				role="button"
				tabindex="0"
				onclick={() => openFaq = openFaq === i ? null : i}
				onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); openFaq = openFaq === i ? null : i; } }}
			>
				<div class="flex items-center justify-between gap-4">
					<span class="font-medium text-sm">{faq.q}</span>
					<span class="text-[var(--muted-foreground)] transition-transform duration-200 {openFaq === i ? 'rotate-45' : ''}">+</span>
				</div>
				{#if openFaq === i}
					<span class="text-sm text-[var(--muted-foreground)] mt-3 leading-relaxed block">{faq.a}</span>
				{/if}
			</div>
		{/each}
	</div>
</section>

<div class="separator-gradient"></div>

<!-- CTA -->
<section class="max-w-4xl mx-auto px-6 py-20 md:py-28 text-center">
	<h2 class="text-2xl md:text-4xl font-bold mb-4">
		Ready to ship<br />
		<span class="hero-gradient">better code?</span>
	</h2>
	<p class="text-[var(--muted-foreground)] mb-8 text-base">Free forever · No account required · Start in 30 seconds</p>
	<div class="flex items-center justify-center gap-4">
		<a
			href="https://github.com/codecoradev/cora-cli"
			target="_blank"
			rel="noopener"
			class="cta-primary inline-flex items-center gap-2 px-6 py-3 rounded-lg bg-[var(--accent)] text-[var(--accent-foreground)] font-semibold"
		>
			<svg class="w-5 h-5" viewBox="0 0 16 16" fill="currentColor"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8z" /></svg>
			Star on GitHub
		</a>
		<a
			href="/docs"
			class="cta-secondary inline-flex items-center gap-2 px-6 py-3 rounded-lg border border-[var(--border)] text-[var(--muted-foreground)] font-medium"
		>
			Read the Docs
		</a>
	</div>
</section>
