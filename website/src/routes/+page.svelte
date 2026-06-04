<script lang="ts">
	import { onMount } from 'svelte';

	// ---- Cycling command demo state ----
	interface Command {
		cmd: string;
		desc: string;
		output: string;
	}

	const commands: Command[] = [
		{
			cmd: 'cora review --staged',
			desc: 'Review staged changes before commit',
			output: `🛡️  Rules engine: 3 files scanned (zero-cost regex)
    ✖ src/api/auth.ts:42  — hardcoded API key detected
    ✖ src/db/query.ts:18  — potential SQL injection

🔗  Context chain: resolving cross-file references...
    → src/api/auth.ts imports validateToken from src/lib/token.ts

🤖  LLM review: OpenAI GPT-4o-mini
    ⚠ src/api/auth.ts:67  — missing error boundary

✓ Review complete — 4 issues found`
		},
		{
			cmd: 'cora review --pr 42',
			desc: 'Review an existing pull request',
			output: `📦  Fetching PR #42: feat: add payment processing
📝  12 files changed, +340 −89

🛡️  Rules engine: 12 files scanned
    ✖ src/payments/stripe.ts:15  — hardcoded secret key

🤖  LLM review: Anthropic Claude 3.5 Sonnet
    ⚠ src/payments/webhook.ts:45  — missing idempotency check

📊  SARIF: review-results.sarif written (3 findings)
✓ Review complete — 3 issues found`
		},
		{
			cmd: 'cora config set llm.model gpt-4o',
			desc: 'Configure your LLM provider',
			output: `✓ Config updated: llm.model = "gpt-4o"
  Config file: .cora.yaml
  
  Run 'cora review' to start using the new model.`
		},
		{
			cmd: 'cora review --full',
			desc: 'Full codebase review with all checks',
			output: `🛡️  Rules engine: 47 files scanned (zero-cost regex)
    ✖ 3 hardcoded secrets detected
    ✖ 2 potential SQL injection patterns

🔗  Context chain: 23 cross-file references resolved

🤖  LLM review: OpenAI GPT-4o
    ⚠ 8 issues found across 6 files
    💡 3 suggestions for improvement

📊  SARIF: review-results.sarif written (13 findings)
✓ Review complete — 13 issues found`
		},
	];

	let visibleCmd: number = $state(0);

	// ---- FAQ state ----
	let openFAQIndex: number | null = $state(null);

	function toggleFAQ(index: number) {
		openFAQIndex = openFAQIndex === index ? null : index;
	}

	// ---- Copy install command ----
	const installCmd = 'curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh';
	let copyClicked = $state(false);
	function copyInstall() {
		navigator.clipboard.writeText(installCmd);
		copyClicked = true;
		setTimeout(() => { copyClicked = false; }, 2000);
	}

	// ---- Problem/Solution pairs ----
	const comparisons = [
		{
			problem: 'Code review takes hours',
			solution: 'AI review in under 3 seconds',
		},
		{
			problem: 'Locked to one AI provider',
			solution: 'BYOK — use any LLM you want',
		},
		{
			problem: 'CI reviews are expensive',
			solution: 'Free, open source, MIT license',
		},
		{
			problem: 'Reviewers miss context',
			solution: 'Cross-file context chain + deterministic rules',
		},
	];

	// ---- Features ----
	const features = [
		{
			icon: '🔑',
			title: 'BYOK Any LLM',
			description: 'OpenAI, Anthropic, Groq, local models, any OpenAI-compatible endpoint. You control the model, you control the cost.',
			tag: 'Zero lock-in',
		},
		{
			icon: '🛡️',
			title: 'Deterministic Rules Engine',
			description: 'Zero-cost regex rules catch secrets, SQL injection, TLS issues BEFORE the LLM call. No tokens wasted on obvious bugs.',
			tag: 'Zero token cost',
		},
		{
			icon: '🔗',
			title: 'Cross-file Context Chain',
			description: 'AST-based symbol extraction understands your codebase, not just the diff. Follows imports, resolves types.',
			tag: 'Full codebase awareness',
		},
		{
			icon: '📦',
			title: 'File Bundling',
			description: 'Smart grouping of related files for consistent review across large PRs. Reduces context window waste.',
			tag: 'Smart grouping',
		},
		{
			icon: '⚡',
			title: 'Pre-commit Hooks',
			description: 'Automatic review on every git commit. Blocks bad code before it\'s pushed. Zero friction, maximum impact.',
			tag: 'Before you push',
		},
		{
			icon: '📊',
			title: 'SARIF + CI Integration',
			description: 'GitHub Actions composite action, SARIF upload, PR comments, blocking on error. Works in any CI/CD pipeline.',
			tag: 'GitHub native',
		},
		{
			icon: '🔒',
			title: 'Privacy First',
			description: 'Runs locally, zero telemetry, your code never leaves your machine. Perfect for sensitive codebases.',
			tag: 'Your code stays yours',
		},
		{
			icon: '🚀',
			title: 'Zero Config',
			description: 'Works out of the box with sensible defaults. Customize with <code class="text-[var(--accent)] font-mono text-[13px]">.cora.yaml</code> when you need more control.',
			tag: 'Works immediately',
		},
		{
			icon: '🦀',
			title: 'Built in Rust',
			description: 'Single binary, no runtime dependencies, cross-platform. macOS, Linux, Windows — one download, done.',
			tag: 'Single binary',
		},
	];

	// ---- Comparison table ----
	const competitors = [
		{ feature: 'Free forever', cora: '✅ MIT', coderabbit: '❌ $12/mo+', copilot: '❌ $10/mo+', sonarqube: '❌ $150+/mo' },
		{ feature: 'BYOK', cora: '✅ Any LLM', coderabbit: '❌ No', copilot: '❌ No', sonarqube: '❌ No' },
		{ feature: 'Pre-commit hook', cora: '✅ Native', coderabbit: '❌ PR-only', copilot: '❌ PR-only', sonarqube: '❌ CI-only' },
		{ feature: 'CLI-first', cora: '✅ Local', coderabbit: '❌ Cloud', copilot: '❌ Cloud', sonarqube: '❌ CI' },
		{ feature: 'Zero telemetry', cora: '✅', coderabbit: '❌ Cloud', copilot: '❌ Cloud', sonarqube: '❌ Cloud' },
		{ feature: 'Rules engine', cora: '✅ Deterministic', coderabbit: '❌ LLM-only', copilot: '❌ LLM-only', sonarqube: '❌ Static only' },
	];

	// ---- FAQ ----
	const faqs = [
		{
			q: 'What is cora?',
			a: 'cora is a CLI-first AI code review tool that catches bugs, security issues, and code quality problems before you commit. 100% open source under the MIT license.'
		},
		{
			q: 'What providers are supported?',
			a: 'Any OpenAI-compatible endpoint: OpenAI, Anthropic (via proxy), Groq, Ollama, vLLM, LiteLLM, and more. If it speaks the OpenAI API format, it works.'
		},
		{
			q: 'Does cora work offline?',
			a: 'cora runs entirely locally on your machine. It needs internet for LLM API calls (unless you use a local model like Ollama), but no code data is sent to any cora server — there is no cora server.'
		},
		{
			q: 'How is this different from CodeRabbit?',
			a: 'Four key differences: 1) CLI-first — review before you commit, not after the PR is opened. 2) BYOK — use any LLM, not locked to one provider. 3) Zero telemetry — no data leaves your machine. 4) Deterministic rules engine catches issues with zero token cost.'
		},
		{
			q: 'What is the rules engine?',
			a: 'A pre-LLM layer that runs zero-cost regex rules against your diff before calling the LLM. It catches hardcoded secrets, SQL injection patterns, debug print statements, weak TLS configs, and more — without spending a single token.'
		},
		{
			q: 'Can I use cora in CI?',
			a: 'Yes. There\'s a GitHub Actions composite action that runs cora review, uploads SARIF results to GitHub Code Scanning, posts PR comments, and can optionally block the PR on error findings.'
		},
		{
			q: 'How do I track my review stats?',
			a: 'Run <code class="text-[var(--accent)] font-mono text-[13px]">cora gain</code> for a local dashboard showing your review history, issues found, time saved, and streaks. All data stays on your machine.'
		},
		{
			q: 'Is it really free?',
			a: '100% MIT license, no paid tiers, no account required, no telemetry. The only cost is your LLM API usage — and with the deterministic rules engine, most issues are caught before any LLM call.'
		},
	];

	// ---- Repos using cora ----
	const repos = [
		{ name: 'gofin', desc: 'Finance CLI' },
		{ name: 'bond', desc: 'Bond calculator' },
		{ name: 'vivd', desc: 'Vivid terminal UI' },
		{ name: 'uteke', desc: 'Task engine' },
		{ name: 'termul', desc: 'Terminal multiplexer' },
		{ name: 'agentboard', desc: 'Agent dashboard' },
	];

	onMount(() => {
		// Scroll reveal observer
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						entry.target.classList.add('revealed');
						observer.unobserve(entry.target);
					}
				});
			},
			{ threshold: 0.1 }
		);

		document.querySelectorAll('.scroll-reveal').forEach((el) => observer.observe(el));

		// Cycle through commands
		const interval = setInterval(() => {
			visibleCmd = (visibleCmd + 1) % commands.length;
		}, 4000);

		return () => clearInterval(interval);
	});
</script>

<svelte:head>
	<title>cora — AI Code Review CLI · Open Source · MIT</title>
	<meta name="description" content="CLI-first AI code review. BYOK any LLM. Pre-commit hooks. Deterministic rules engine. Zero telemetry. Open source MIT license. Built in Rust." />
	<link rel="canonical" href="https://codecora.dev" />

	<!-- FAQPage Schema for Rich Snippets -->
	<script type="application/ld+json">{"@context":"https://schema.org","@type":"FAQPage","mainEntity":[{"@type":"Question","name":"What is cora?","acceptedAnswer":{"@type":"Answer","text":"cora is a CLI-first AI code review tool that catches bugs, security issues, and code quality problems before you commit. 100% open source under the MIT license."}},{"@type":"Question","name":"What providers are supported?","acceptedAnswer":{"@type":"Answer","text":"Any OpenAI-compatible endpoint: OpenAI, Anthropic (via proxy), Groq, Ollama, vLLM, LiteLLM, and more."}},{"@type":"Question","name":"Does cora work offline?","acceptedAnswer":{"@type":"Answer","text":"cora runs entirely locally. It needs internet for LLM API calls (unless using a local model like Ollama), but no code data is sent to any cora server."}},{"@type":"Question","name":"How is this different from CodeRabbit?","acceptedAnswer":{"@type":"Answer","text":"CLI-first (before commit, not after PR), BYOK, zero telemetry, deterministic rules engine."}},{"@type":"Question","name":"What is the rules engine?","acceptedAnswer":{"@type":"Answer","text":"A pre-LLM layer that runs zero-cost regex rules to catch secrets, SQL injection, debug prints, weak TLS configs without spending tokens."}},{"@type":"Question","name":"Can I use cora in CI?","acceptedAnswer":{"@type":"Answer","text":"Yes. GitHub Actions composite action with SARIF upload, PR comments, and optional PR blocking."}},{"@type":"Question","name":"How do I track my review stats?","acceptedAnswer":{"@type":"Answer","text":"Run cora gain for a local dashboard showing review history, issues found, time saved, and streaks."}},{"@type":"Question","name":"Is it really free?","acceptedAnswer":{"@type":"Answer","text":"100% MIT license, no paid tiers, no account required, no telemetry."}}]}</script>

	<meta property="og:title" content="cora — AI Code Review CLI · v0.4 · Open Source · MIT" />
	<meta property="og:description" content="CLI-first AI code review. BYOK any LLM. Pre-commit hooks. Deterministic rules engine. Zero telemetry. Open source MIT license." />
</svelte:head>

<!-- ====== HERO ====== -->
<section class="grid-bg relative overflow-hidden">
	<!-- Subtle top gradient -->
	<div class="absolute inset-0 bg-gradient-to-b from-purple-950/10 to-transparent pointer-events-none"></div>

	<div class="relative max-w-6xl mx-auto px-6 pt-24 md:pt-36 pb-20 text-center">
		<!-- Badge -->
		<div class="animate-fade-in inline-flex items-center gap-2 px-3 py-1.5 rounded-full border border-[var(--border)] bg-[var(--card)] text-xs text-[var(--muted-foreground)] mb-8">
			<span class="w-2 h-2 rounded-full bg-[var(--success)]"></span>
			<span>v0.4 released</span>
			<span class="text-[var(--muted-foreground)]">—</span>
			<a href="https://github.com/codecoradev/cora-cli" target="_blank" rel="noopener" class="text-[var(--accent)] hover:underline">release notes →</a>
		</div>

		<!-- Headline -->
		<h1 class="animate-fade-in-delay-1 text-4xl sm:text-5xl md:text-7xl font-bold tracking-tight mb-6 leading-[1.1]">
			Ship with Confidence.<br />
			<span class="hero-gradient glow-text">AI Code Review, Before You Commit.</span>
		</h1>

		<!-- Subheadline -->
		<p class="animate-fade-in-delay-2 text-base md:text-xl text-[var(--muted-foreground)] max-w-2xl mx-auto mb-10 leading-relaxed">
			CLI-first AI code review. <strong class="text-[var(--foreground)]">BYOK any LLM, zero telemetry, open source.</strong>
		</p>

		<!-- Install command -->
		<div class="animate-fade-in-delay-3 inline-flex flex-col items-center gap-4">
			<div class="terminal-block inline-flex items-center gap-3 px-5 py-3.5 rounded-xl font-mono text-sm glow-purple">
				<span class="text-[var(--success)]">$</span>
				<span class="hidden sm:inline text-[var(--foreground)]">curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh</span>
				<span class="sm:hidden text-[var(--foreground)]">curl -fsSL .../install.sh | sh</span>
				<button
					class="text-[var(--muted-foreground)] hover:text-[var(--accent)] transition-colors ml-2"
					onclick={() => navigator.clipboard.writeText(installCmd)}
					title="Copy"
				>
					<svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
						<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
					</svg>
				</button>
			</div>

			<div class="flex items-center gap-3 text-sm">
				<a
					href="https://github.com/codecoradev/cora-cli"
					target="_blank"
					rel="noopener"
					class="cta-primary inline-flex items-center gap-2 px-5 py-2.5 rounded-lg bg-[var(--accent)] text-[var(--accent-foreground)] font-semibold text-sm"
				>
					<svg class="w-4 h-4" viewBox="0 0 16 16" fill="currentColor"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8z"/></svg>
					GitHub
				</a>
				<a
					href="/docs/getting-started"
					class="cta-secondary inline-flex items-center gap-2 px-5 py-2.5 rounded-lg border border-[var(--border)] text-[var(--muted-foreground)] font-medium text-sm"
				>
					Install & Get Started
				</a>
			</div>
		</div>

		<!-- Trust line -->
		<p class="mt-8 animate-fade-in-delay-4 text-xs text-[var(--muted-foreground)] tracking-wide">
			<span class="flex items-center justify-center flex-wrap gap-x-4 gap-y-1">
				<span>MIT License</span>
				<span>·</span>
				<span>No account</span>
				<span>·</span>
				<span>No telemetry</span>
				<span>·</span>
				<span>Rust & WebAssembly</span>
			</span>
		</p>
	</div>
</section>

<!-- ====== TERMINAL DEMO (Cycling Commands) ====== -->
<section class="max-w-6xl mx-auto px-6 -mt-4 pb-16 md:pb-24">
	<div class="terminal-block rounded-2xl overflow-hidden glow-purple">
		<!-- Title bar -->
		<div class="flex items-center gap-2 px-4 py-3 border-b border-[var(--border)] bg-[var(--card)]">
			<div class="flex gap-1.5">
				<div class="w-3 h-3 rounded-full bg-red-500/60"></div>
				<div class="w-3 h-3 rounded-full bg-yellow-500/60"></div>
				<div class="w-3 h-3 rounded-full bg-green-500/60"></div>
			</div>
			<span class="text-xs text-[var(--muted-foreground)] ml-2 font-mono">cora v0.4 — AI code review engine</span>
		</div>

		<!-- Terminal content -->
		<div class="p-5 md:p-6 font-mono text-sm space-y-3 min-h-[260px]">
			{#each commands as c, i}
				<div class="transition-all duration-300 {i === visibleCmd ? 'opacity-100' : 'opacity-30'}">
					{#if i === visibleCmd}
						<div class="flex items-start gap-2">
							<span class="text-[var(--success)] select-none">❯</span>
							<div>
								<span class="cmd-highlight">{c.cmd.split(' ')[0]}</span>
								<span class="cmd-flag"> {c.cmd.split(' ').slice(1).join(' ')}</span>
							</div>
						</div>
						<p class="text-[var(--muted-foreground)] ml-5 mt-0.5 text-xs cmd-comment"># {c.desc}</p>
						<div class="ml-5 mt-2 text-[var(--muted-foreground)] text-xs whitespace-pre-wrap leading-relaxed">{c.output}</div>
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

<!-- ====== SOCIAL PROOF ====== -->
<section class="max-w-6xl mx-auto px-6 py-10 text-center">
	<div class="flex flex-wrap items-center justify-center gap-8 text-[var(--muted-foreground)]">
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

<!-- ====== PROBLEM / SOLUTION CARDS ====== -->
<section class="max-w-6xl mx-auto px-6 py-16 md:py-24">
	<div class="max-w-3xl mx-auto text-center">
		<p class="text-sm font-medium text-[var(--accent)] mb-4 tracking-wide uppercase">The Problem</p>
		<h2 class="text-2xl md:text-4xl font-bold mb-6">
			Code review shouldn't be<br />slow or expensive
		</h2>
		<p class="text-base md:text-lg text-[var(--muted-foreground)] leading-relaxed mb-12">
			Stop waiting hours for feedback. cora catches issues in seconds — <span class="text-[var(--foreground)]">before you commit.</span>
		</p>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-4xl mx-auto">
		{#each comparisons as comp, i}
			<div class="grid gap-4 scroll-reveal" style="transition-delay: {i * 100}ms;">
				<!-- Problem card -->
				<div class="feature-card rounded-xl p-6 border border-[var(--border)] bg-[var(--card)]">
					<div class="flex items-center gap-3 text-[var(--destructive)]">
						<span>✕</span>
						<span class="font-semibold">{comp.problem}</span>
					</div>
				</div>
				<!-- Solution card -->
				<div class="feature-card rounded-xl p-6 border" style="background: oklch(0.72 0.19 145 / 0.05); border-color: oklch(0.72 0.19 145 / 0.2);">
					<div class="flex items-center gap-3 text-[var(--success)]">
						<span>✓</span>
						<span class="font-semibold">{comp.solution}</span>
					</div>
				</div>
			</div>
		{/each}
	</div>
</section>

<div class="separator-gradient"></div>

<!-- ====== FEATURES GRID ====== -->
<section class="max-w-6xl mx-auto px-6 py-16 md:py-24">
	<div class="text-center mb-14">
		<p class="text-sm font-medium text-[var(--accent)] mb-4 tracking-wide uppercase scroll-reveal">Features</p>
		<h2 class="text-2xl md:text-4xl font-bold scroll-reveal">Everything you need.<br />Nothing you don't.</h2>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
		{#each features as feat, i}
			<div class="feature-card p-6 rounded-xl border border-[var(--border)] bg-[var(--card)] scroll-reveal" style="transition-delay: {i * 60}ms;">
				<span class="text-2xl">{feat.icon}</span>
				<h3 class="text-lg font-semibold mt-3 mb-1">{feat.title}</h3>
				<p class="text-sm text-[var(--accent)] mb-2">{feat.tag}</p>
				<p class="text-sm text-[var(--muted-foreground)] leading-relaxed">{@html feat.description}</p>
			</div>
		{/each}
	</div>
</section>

<div class="separator-gradient"></div>

<!-- ====== QUICK START / HOW IT WORKS ====== -->
<section class="grid-bg relative">
	<div class="max-w-6xl mx-auto px-6 py-16 md:py-24 relative z-10">
		<div class="text-center mb-14">
			<p class="text-sm font-medium text-[var(--accent)] mb-4 tracking-wide uppercase scroll-reveal">Get Started</p>
			<h2 class="text-2xl md:text-4xl font-bold scroll-reveal">Three steps to<br />confident commits</h2>
		</div>

		<div class="max-w-2xl mx-auto space-y-6">
			<div class="flex gap-4 scroll-reveal">
				<div class="flex-shrink-0 w-8 h-8 rounded-full bg-[var(--accent-dim)] flex items-center justify-center text-sm font-bold text-[var(--accent)]">1</div>
				<div class="min-w-0 flex-1">
					<p class="font-medium mb-2">Install</p>
					<code class="block px-4 py-3 rounded-lg bg-[var(--card)] border border-[var(--border)] text-sm font-mono text-[var(--muted-foreground)] overflow-x-auto">
						<span class="cmd-highlight">curl</span> -fsSL .../install.sh <span class="cmd-flag">|</span> sh
					</code>
				</div>
			</div>
			<div class="flex gap-4 scroll-reveal">
				<div class="flex-shrink-0 w-8 h-8 rounded-full bg-[var(--accent-dim)] flex items-center justify-center text-sm font-bold text-[var(--accent)]">2</div>
				<div class="min-w-0 flex-1">
					<p class="font-medium mb-2">Configure</p>
					<code class="block px-4 py-3 rounded-lg bg-[var(--card)] border border-[var(--border)] text-sm font-mono text-[var(--muted-foreground)] overflow-x-auto">
						<span class="cmd-highlight">cora</span> init
					</code>
				</div>
			</div>
			<div class="flex gap-4 scroll-reveal">
				<div class="flex-shrink-0 w-8 h-8 rounded-full bg-[var(--accent-dim)] flex items-center justify-center text-sm font-bold text-[var(--accent)]">3</div>
				<div class="min-w-0 flex-1">
					<p class="font-medium mb-2">Review</p>
					<code class="block px-4 py-3 rounded-lg bg-[var(--card)] border border-[var(--border)] text-sm font-mono text-[var(--muted-foreground)] overflow-x-auto">
						<span class="cmd-highlight">cora review</span> <span class="cmd-flag">--staged</span>
					</code>
				</div>
			</div>
			<div class="flex gap-4 scroll-reveal">
				<div class="flex-shrink-0 w-8 h-8 rounded-full bg-[var(--success)]/20 flex items-center justify-center text-sm font-bold text-[var(--success)]">✓</div>
				<div class="min-w-0 flex-1">
					<p class="font-medium text-[var(--success)]">That's it. No account. No server. No lock-in.</p>
					<p class="text-sm text-[var(--muted-foreground)] mt-1">Works out of the box with sensible defaults. Customize with <code class="text-[var(--accent)] font-mono text-[12px]">.cora.yaml</code> when you need more control.</p>
				</div>
			</div>
		</div>
	</div>
</section>

<div class="separator-gradient"></div>

<!-- ====== REPOS USING CORA ====== -->
<section class="max-w-6xl mx-auto px-6 py-16 md:py-24">
	<p class="text-center scroll-reveal text-xs font-medium text-[var(--muted-foreground)] uppercase tracking-widest">
		Trusted across 13 repositories
	</p>
	<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-4 mt-8 max-w-4xl mx-auto">
		{#each repos as repo, i}
			<div class="feature-card text-center scroll-reveal py-6 rounded-xl border border-[var(--border)] bg-[var(--card)]" style="transition-delay: {i * 60}ms;">
				<div class="text-base font-semibold text-[var(--foreground)] -tracking-tight">{repo.name}</div>
				<div class="text-xs text-[var(--muted-foreground)] mt-1">{repo.desc}</div>
			</div>
		{/each}
	</div>
</section>

<div class="separator-gradient"></div>

<!-- ====== COMPARISON TABLE ====== -->
<section class="max-w-6xl mx-auto px-6 py-16 md:py-24">
	<div class="text-center mb-14">
		<p class="text-sm font-medium text-[var(--accent)] mb-4 tracking-wide uppercase scroll-reveal">Compare</p>
		<h2 class="text-2xl md:text-4xl font-bold scroll-reveal">Why developers choose cora</h2>
	</div>

	<div class="overflow-x-auto rounded-xl border border-[var(--border)] scroll-reveal">
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
				{#each competitors as row}
					<tr class="border-b border-[var(--border-subtle)] hover:bg-[var(--card)] transition-colors">
						<td class="px-4 py-3 text-[var(--muted-foreground)]">{row.feature}</td>
						<td class="highlight-col px-4 py-3 text-center font-medium">{@html row.cora}</td>
						<td class="px-4 py-3 text-center text-[var(--muted-foreground)] hidden sm:table-cell">{@html row.coderabbit}</td>
						<td class="px-4 py-3 text-center text-[var(--muted-foreground)] hidden md:table-cell">{@html row.copilot}</td>
						<td class="px-4 py-3 text-center text-[var(--muted-foreground)] hidden lg:table-cell">{@html row.sonarqube}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</section>

<div class="separator-gradient"></div>

<!-- ====== FAQ ACCORDION ====== -->
<section class="max-w-6xl mx-auto px-6 py-16 md:py-24" id="faq">
	<div class="max-w-3xl mx-auto">
		<div class="text-center">
			<p class="text-sm font-medium text-[var(--accent)] mb-4 tracking-wide uppercase scroll-reveal">FAQ</p>
			<h2 class="text-2xl md:text-4xl font-bold scroll-reveal">Frequently asked questions</h2>
		</div>

		<div class="mt-10 flex flex-col gap-3">
			{#each faqs as faq, index}
				<div class="scroll-reveal" style="transition-delay: {index * 50}ms;">
					<div class="feature-card rounded-xl border overflow-hidden" style="background: var(--card); border-color: var(--border);">
						<button
							onclick={() => toggleFAQ(index)}
							class="w-full px-6 py-5 text-left flex items-center justify-between gap-4 cursor-pointer hover:opacity-80 transition-opacity"
							aria-expanded={openFAQIndex === index}
						>
							<span class="font-semibold text-sm sm:text-base text-[var(--foreground)]">{faq.q}</span>
							<span class="text-xl text-[var(--muted-foreground)] flex-shrink-0 transition-transform duration-300" style="transform: {openFAQIndex === index ? 'rotate(45deg)' : 'rotate(0deg)'}">
								+
							</span>
						</button>
						{#if openFAQIndex === index}
							<div class="px-6 pb-5 text-sm text-[var(--muted-foreground)] leading-relaxed border-t" style="border-color: var(--border);">
								<div class="pt-4">{@html faq.a}</div>
							</div>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</div>
</section>

<div class="separator-gradient"></div>

<!-- ====== FINAL CTA ====== -->
<section class="max-w-6xl mx-auto px-6 py-20 md:py-28 text-center">
	<h2 class="text-2xl md:text-4xl font-bold mb-4 scroll-reveal">
		Start Shipping<br />
		<span class="hero-gradient">Better Code</span>
	</h2>
	<p class="text-[var(--muted-foreground)] mb-8 text-base scroll-reveal">Free forever · No account required · Open source MIT</p>
	<div class="flex items-center justify-center gap-4 scroll-reveal">
		<a
			href="https://github.com/codecoradev/cora-cli"
			target="_blank"
			rel="noopener"
			class="cta-primary inline-flex items-center gap-2 px-6 py-3 rounded-lg bg-[var(--accent)] text-[var(--accent-foreground)] font-semibold"
		>
			<svg class="w-5 h-5" viewBox="0 0 16 16" fill="currentColor"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8z"/></svg>
			Star on GitHub
		</a>
		<a
			href="/docs/getting-started"
			class="cta-secondary inline-flex items-center gap-2 px-6 py-3 rounded-lg border border-[var(--border)] text-[var(--muted-foreground)] font-medium"
		>
			Read the Docs
		</a>
	</div>
</section>

<!-- ====== FOOTER ====== -->
<div class="separator-gradient"></div>
<footer style="background: var(--background);">
	<div class="max-w-6xl mx-auto px-6 py-8">
		<div class="flex flex-col md:flex-row items-center justify-between gap-4">
			<div class="flex items-center gap-3">
				<span class="text-sm font-semibold text-[var(--foreground)]">cora</span>
				<span class="text-xs text-[var(--muted-foreground)]">v0.4 · MIT License</span>
			</div>
			<div class="flex items-center gap-6">
				<a href="https://github.com/codecoradev/cora-cli" target="_blank" rel="noopener" class="text-sm text-[var(--muted-foreground)] no-underline transition-colors duration-200 min-h-11 inline-flex items-center hover:text-[var(--foreground)]">GitHub</a>
				<a href="/docs" class="text-sm text-[var(--muted-foreground)] no-underline transition-colors duration-200 min-h-11 inline-flex items-center hover:text-[var(--foreground)]">Docs</a>
				<a href="#faq" class="text-sm text-[var(--muted-foreground)] no-underline transition-colors duration-200 min-h-11 inline-flex items-center hover:text-[var(--foreground)]">FAQ</a>
			</div>
		</div>
	</div>
</footer>
