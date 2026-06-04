<script lang="ts">
	import { onMount } from 'svelte';

	// ---- Terminal typing animation state ----
	let terminalLines = $state<string[]>([]);
	let terminalComplete = $state(false);
	let demoStarted = $state(false);

	const installCmd = 'curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh';

	const terminalOutput = [
		{ text: '$ cora review --staged', color: 'var(--muted-foreground)' },
		{ text: '', color: '' },
		{ text: '🛡️  Rules engine: 3 files scanned (zero-cost regex)', color: 'var(--muted-foreground)' },
		{ text: '    ✖ src/api/auth.ts:42  — hardcoded API key detected', color: 'var(--destructive)' },
		{ text: '    ✖ src/db/query.ts:18  — potential SQL injection', color: 'var(--destructive)' },
		{ text: '', color: '' },
		{ text: '🔗  Context chain: resolving cross-file references...', color: 'var(--muted-foreground)' },
		{ text: '    → src/api/auth.ts imports validateToken from src/lib/token.ts', color: 'var(--muted-foreground)' },
		{ text: '    → src/db/query.ts uses pool from src/lib/db.ts', color: 'var(--muted-foreground)' },
		{ text: '', color: '' },
		{ text: '🤖  LLM review: OpenAI GPT-4o-mini', color: 'var(--muted-foreground)' },
		{ text: '    ⚠ src/api/auth.ts:67  — missing error boundary for token refresh', color: 'var(--warning)' },
		{ text: '    ⚠ src/lib/token.ts:23  — token expiry not validated', color: 'var(--warning)' },
		{ text: '', color: '' },
		{ text: '📊  SARIF: review-results.sarif written (4 findings)', color: 'var(--muted-foreground)' },
		{ text: '', color: '' },
		{ text: '✓ Review complete — 4 issues found (2 critical, 2 major)', color: 'var(--accent)' },
	];

	// ---- FAQ state ----
	let openFAQIndex = $state<number | null>(null);

	function toggleFAQ(index: number) {
		openFAQIndex = openFAQIndex === index ? null : index;
	}

	// ---- Copy install command ----
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

		// Typing animation observer - start when terminal section visible
		const demoObserver = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting && !demoStarted) {
						demoStarted = true;
						let lineIndex = 0;
						const typeInterval = setInterval(() => {
							if (lineIndex < terminalOutput.length) {
								terminalLines = [...terminalLines, terminalOutput[lineIndex].text];
								lineIndex++;
							} else {
								terminalComplete = true;
								clearInterval(typeInterval);
							}
						}, 150);
					}
				});
			},
			{ threshold: 0.3 }
		);

		const demoEl = document.getElementById('demo-terminal');
		if (demoEl) demoObserver.observe(demoEl);
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

<div class="bg-[var(--background)]">
	<!-- ====== FLOATING BLOBS (Background) ====== -->
	<div class="fixed inset-0 pointer-events-none overflow-hidden z-0" aria-hidden="true">
		<div class="blob blob-1"></div>
		<div class="blob blob-2"></div>
		<div class="blob blob-3"></div>
	</div>

	<!-- ====== S1 — HERO ====== -->
	<section class="section section-hero relative flex items-center justify-center min-h-[calc(100vh-3.5rem)]">
		<!-- Subtle radial gradient glow -->
		<div class="absolute inset-0 pointer-events-none" style="background: radial-gradient(ellipse 50% 40% at 50% 40%, oklch(0.65 0.22 270 / 0.06), transparent);"></div>

		<div class="relative z-10 max-w-3xl mx-auto text-center">
			<!-- Badge -->
			<div class="animate-fade-in-up">
				<span class="accent-badge">
					<span class="badge-dot"></span>
					v0.4 · Open Source · MIT
				</span>
			</div>

			<!-- Headline -->
			<h1 class="mt-8 mb-0 animate-fade-in-up delay-100 text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold text-[var(--foreground)] leading-none -tracking-tighter">
				Ship with Confidence.
				<span class="bg-gradient-to-br from-[oklch(0.65_0.22_270)] to-[oklch(0.7_0.15_240)] bg-clip-text text-transparent"> AI Code Review, Before You Commit.</span>
			</h1>

			<!-- Subtitle -->
			<p class="max-w-xl mx-auto mt-6 animate-fade-in-up delay-200 text-base sm:text-lg text-[var(--muted-foreground)] leading-normal sm:leading-relaxed -tracking-tight">
				CLI-first AI code review. BYOK any LLM. Pre-commit hooks. Zero telemetry. Open source.
			</p>

			<!-- Install Terminal -->
			<div class="max-w-2xl w-full mx-auto mt-10 animate-fade-in-up delay-300">
				<div class="terminal relative">
					<div class="terminal-header">
						<span class="terminal-dot terminal-dot-red"></span>
						<span class="terminal-dot terminal-dot-yellow"></span>
						<span class="terminal-dot terminal-dot-green"></span>
						<span class="terminal-title">Install cora</span>
					</div>
					<div class="terminal-body relative overflow-x-auto">
						<span class="syntax-cmd">$</span>
						<span class="syntax-highlight"> curl -fsSL</span>
						<span class="syntax-string"> https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh</span>
						<span class="syntax-highlight"> | sh</span>
						<span class="typing-cursor"></span>
						<button class="copy-btn" onclick={copyClicked ? undefined : copyInstall} class:copied={copyClicked} aria-label="Copy install command">
							{#if copyClicked}
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
							{:else}
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
							{/if}
						</button>
					</div>
				</div>
			</div>

			<!-- CTAs -->
			<div class="flex flex-wrap justify-center gap-4 items-center mt-8 animate-fade-in-up delay-400">
				<a href="https://github.com/codecoradev/cora-cli" target="__blank" rel="noopener" class="btn-primary">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
					Star on GitHub
				</a>
				<a href="/docs/getting-started" class="btn-ghost">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
					Install & Get Started
				</a>
			</div>

			<!-- Trust Line -->
			<p class="mt-6 animate-fade-in-up delay-500 text-xs text-[var(--muted-foreground)] tracking-wide">
				<span class="flex items-center justify-center flex-wrap gap-x-4 gap-y-1">
					<span>MIT License</span>
					<span>·</span>
					<span>No account</span>
					<span>·</span>
					<span>No telemetry</span>
					<span>·</span>
					<span>Rust &amp; WebAssembly</span>
				</span>
			</p>
		</div>
	</section>

	<!-- ====== S2 — PROBLEM / SOLUTION CARDS ====== -->
	<section class="section section-tall relative z-10">
		<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
			Code review shouldn't be slow or expensive
		</h2>
		<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)] max-w-[32rem] mx-auto">
			Stop waiting hours for feedback. cora catches issues in seconds — before you commit.
		</p>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-10 max-w-4xl mx-auto">
			{#each comparisons as comp, i}
				<div class="grid gap-4 scroll-reveal" style="transition-delay: {i * 100}ms;">
					<!-- Problem card -->
					<div class="rounded-xl p-6 border" style="background: oklch(0.55 0.22 25 / 0.08); border-color: oklch(0.55 0.22 25 / 0.2);">
						<div class="flex items-center gap-3" style="color: var(--destructive);">
							<span>✕</span>
							<span class="font-semibold">{comp.problem}</span>
						</div>
					</div>
					<!-- Solution card -->
					<div class="rounded-xl p-6 border" style="background: oklch(0.72 0.19 145 / 0.08); border-color: oklch(0.72 0.19 145 / 0.2);">
						<div class="flex items-center gap-3" style="color: var(--success);">
							<span>✓</span>
							<span class="font-semibold">{comp.solution}</span>
						</div>
					</div>
				</div>
			{/each}
		</div>
	</section>

	<!-- ====== S3 — LIVE TERMINAL DEMO ====== -->
	<section class="section section-tall relative z-10" id="demo-terminal">
		<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
			See it in action
		</h2>
		<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)] max-w-[32rem] mx-auto">
			Run cora against staged changes. Rules engine + LLM + SARIF output — in one pass.
		</p>

		<div class="max-w-3xl mx-auto mt-10 scroll-reveal">
			<div class="terminal">
				<div class="terminal-header">
					<span class="terminal-dot terminal-dot-red"></span>
					<span class="terminal-dot terminal-dot-yellow"></span>
					<span class="terminal-dot terminal-dot-green"></span>
					<span class="terminal-title">cora v0.4 — review --staged</span>
				</div>
				<div class="terminal-body">
					{#each terminalLines as line, i}
						<div class="min-h-[1.45em]" style="color: {terminalOutput[i]?.color || 'var(--foreground)'}; white-space: pre-wrap;">{line}</div>
					{/each}
					{#if terminalComplete}
						<span class="typing-cursor"></span>
					{/if}
				</div>
			</div>
		</div>
	</section>

	<!-- ====== S4 — FEATURES GRID ====== -->
	<section class="section section-tall relative z-10">
		<div class="text-center scroll-reveal">
			<h2 class="text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
				Everything you need. Nothing you don't.
			</h2>
			<p class="mt-4 text-sm text-[var(--muted-foreground)]">
				Built for developers who value speed, privacy, and control.
			</p>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-10">
			{#each features as feature, i}
				<div class="glass-card scroll-reveal" style="transition-delay: {i * 80}ms;">
					<div class="text-2xl mb-4">{feature.icon}</div>
					<div class="text-xs font-medium text-[var(--accent)] mb-2 tracking-wide uppercase">{feature.tag}</div>
					<h3 class="text-lg font-semibold text-[var(--foreground)]">{feature.title}</h3>
					<p class="mt-2 text-sm text-[var(--muted-foreground)] leading-relaxed">{@html feature.description}</p>
				</div>
			{/each}
		</div>
	</section>

	<!-- ====== S5 — HOW IT WORKS ====== -->
	<section class="section section-tall relative z-10">
		<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
			Three steps to confident commits
		</h2>
		<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)] max-w-[32rem] mx-auto">
			Install, configure, review. That's it.
		</p>

		<div class="flex flex-col md:flex-row items-stretch mt-10 gap-6 max-w-5xl mx-auto">
			<!-- Step 1 -->
			<div class="glass-card flex-1 text-center scroll-reveal">
				<div class="text-2xl font-bold text-[var(--accent)] -tracking-tight font-mono opacity-50">01</div>
				<div class="flex justify-center mt-4">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
				</div>
				<h3 class="mt-4 text-xl font-semibold text-[var(--foreground)] -tracking-tight leading-snug">Install</h3>
				<p class="mt-2 text-sm text-[var(--muted-foreground)]">One command. Single binary. No dependencies.</p>
				<div class="mt-4 mx-auto max-w-fit">
					<div class="terminal">
						<div class="terminal-header py-2 px-3">
							<span class="terminal-dot terminal-dot-red" style="width:10px;height:10px;"></span>
							<span class="terminal-dot terminal-dot-yellow" style="width:10px;height:10px;"></span>
							<span class="terminal-dot terminal-dot-green" style="width:10px;height:10px;"></span>
						</div>
						<div class="terminal-body py-3 px-4 text-[12px] leading-relaxed overflow-x-auto">
							<span class="syntax-cmd">$</span> <span class="syntax-highlight">curl -fsSL</span> <span class="syntax-string">…install.sh</span> <span class="syntax-highlight">| sh</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Connector -->
			<div class="connect-line hidden md:flex scroll-reveal">
				<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
			</div>

			<!-- Step 2 -->
			<div class="glass-card flex-1 text-center scroll-reveal [transition-delay:100ms]">
				<div class="text-2xl font-bold text-[var(--accent)] -tracking-tight font-mono opacity-50">02</div>
				<div class="flex justify-center mt-4">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"/></svg>
				</div>
				<h3 class="mt-4 text-xl font-semibold text-[var(--foreground)] -tracking-tight leading-snug">Configure</h3>
				<p class="mt-2 text-sm text-[var(--muted-foreground)]">Creates <code class="text-[var(--accent)] font-mono text-[13px]">.cora.yaml</code> with your LLM settings.</p>
				<div class="mt-4 mx-auto max-w-fit">
					<div class="terminal">
						<div class="terminal-header py-2 px-3">
							<span class="terminal-dot terminal-dot-red" style="width:10px;height:10px;"></span>
							<span class="terminal-dot terminal-dot-yellow" style="width:10px;height:10px;"></span>
							<span class="terminal-dot terminal-dot-green" style="width:10px;height:10px;"></span>
						</div>
						<div class="terminal-body py-3 px-4 text-[12px]">
							<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora init</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Connector -->
			<div class="connect-line hidden md:flex scroll-reveal [transition-delay:100ms]">
				<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
			</div>

			<!-- Step 3 -->
			<div class="glass-card flex-1 text-center scroll-reveal [transition-delay:200ms]">
				<div class="text-2xl font-bold text-[var(--accent)] -tracking-tight font-mono opacity-50">03</div>
				<div class="flex justify-center mt-4">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--success)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 11-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
				</div>
				<h3 class="mt-4 text-xl font-semibold text-[var(--foreground)] -tracking-tight leading-snug">Review</h3>
				<p class="mt-2 text-sm text-[var(--muted-foreground)]">Review staged changes before every commit.</p>
				<div class="mt-4 mx-auto max-w-fit">
					<div class="terminal">
						<div class="terminal-header py-2 px-3">
							<span class="terminal-dot terminal-dot-red" style="width:10px;height:10px;"></span>
							<span class="terminal-dot terminal-dot-yellow" style="width:10px;height:10px;"></span>
							<span class="terminal-dot terminal-dot-green" style="width:10px;height:10px;"></span>
						</div>
						<div class="terminal-body py-3 px-4 text-[12px]">
							<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- ====== S6 — REPOS USING CORA ====== -->
	<section class="section section-compact relative z-10">
		<p class="text-center scroll-reveal text-xs font-medium text-[var(--muted-foreground)] uppercase tracking-widest">
			Trusted across 13 repositories
		</p>
		<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-4 mt-8 max-w-4xl mx-auto">
			{#each repos as repo, i}
				<div class="glass-card text-center scroll-reveal py-6" style="transition-delay: {i * 60}ms;">
					<div class="text-base font-semibold text-[var(--foreground)] -tracking-tight">{repo.name}</div>
					<div class="text-xs text-[var(--muted-foreground)] mt-1">{repo.desc}</div>
				</div>
			{/each}
		</div>
	</section>

	<!-- ====== S7 — COMPARISON TABLE ====== -->
	<section class="section section-tall relative z-10">
		<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
			Why developers choose cora
		</h2>
		<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)]">
			Feature-by-feature with popular code review tools.
		</p>

		<div class="glass-card scroll-reveal mt-10 p-0 max-w-[56rem] mx-auto overflow-hidden">
			<div class="overflow-x-auto">
				<table class="compare-table">
					<thead>
						<tr>
							<th>Feature</th>
							<th class="cora-col">cora</th>
							<th>CodeRabbit</th>
							<th>Copilot</th>
							<th>SonarQube</th>
						</tr>
					</thead>
					<tbody>
						{#each competitors as comp}
							<tr>
								<td>{comp.feature}</td>
								<td class="cora-col">{@html comp.cora}</td>
								<td>{@html comp.coderabbit}</td>
								<td>{@html comp.copilot}</td>
								<td>{@html comp.sonarqube}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	</section>

	<!-- ====== S8 — FAQ ACCORDION ====== -->
	<section class="section section-tall relative z-10" id="faq">
		<div class="max-w-3xl mx-auto">
			<div class="text-center scroll-reveal">
				<h2 class="text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
					Frequently asked questions
				</h2>
				<p class="mt-4 text-sm text-[var(--muted-foreground)]">
					Everything you need to know about cora.
				</p>
			</div>

			<div class="mt-10 flex flex-col gap-3">
				{#each faqs as faq, index}
					<div class="scroll-reveal" style="transition-delay: {index * 50}ms;">
						<div class="rounded-xl border overflow-hidden" style="background: var(--card); border-color: oklch(0.27 0.01 270 / 0.5);">
							<button
								onclick={() => toggleFAQ(index)}
								class="w-full px-6 py-5 text-left flex items-center justify-between gap-4 cursor-pointer hover:opacity-80 transition-opacity"
								aria-expanded={openFAQIndex === index}
							>
								<span class="font-semibold text-sm sm:text-base text-[var(--foreground)]">{faq.q}</span>
								<span class="text-xl text-[var(--muted-foreground)] flex-shrink-0 transition-transform duration-300" style="transform: {openFAQIndex === index ? 'rotate(180deg)' : 'rotate(0deg)'}">
									{openFAQIndex === index ? '−' : '+'}
								</span>
							</button>
							{#if openFAQIndex === index}
								<div class="px-6 pb-5 text-sm text-[var(--muted-foreground)] leading-relaxed border-t" style="border-color: oklch(0.27 0.01 270 / 0.5);">
									<div class="pt-4">{@html faq.a}</div>
								</div>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- ====== S9 — FINAL CTA ====== -->
	<section class="section section-hero relative z-10">
		<div class="absolute inset-0 pointer-events-none" style="background: radial-gradient(ellipse 40% 30% at 50% 50%, oklch(0.65 0.22 270 / 0.05), transparent);"></div>

		<div class="relative z-10 max-w-3xl mx-auto text-center">
			<h2 class="scroll-reveal text-3xl md:text-4xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
				Start Shipping Better Code
			</h2>
			<p class="mt-4 scroll-reveal text-base text-[var(--muted-foreground)]">
				Free forever. No account required. Open source MIT.
			</p>

			<div class="max-w-2xl w-full mx-auto mt-10 scroll-reveal">
				<div class="terminal relative">
					<div class="terminal-header">
						<span class="terminal-dot terminal-dot-red"></span>
						<span class="terminal-dot terminal-dot-yellow"></span>
						<span class="terminal-dot terminal-dot-green"></span>
						<span class="terminal-title">Install cora</span>
					</div>
					<div class="terminal-body relative overflow-x-auto">
						<span class="syntax-cmd">$</span>
						<span class="syntax-highlight"> curl -fsSL</span>
						<span class="syntax-string"> https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh</span>
						<span class="syntax-highlight"> | sh</span>
						<button class="copy-btn" onclick={copyClicked ? undefined : copyInstall} class:copied={copyClicked} aria-label="Copy install command">
							{#if copyClicked}
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
							{:else}
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
							{/if}
						</button>
					</div>
				</div>
			</div>

			<div class="flex flex-wrap justify-center gap-4 items-center mt-8 scroll-reveal">
				<a href="https://github.com/codecoradev/cora-cli" target="_blank" rel="noopener" class="btn-primary">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
					Star on GitHub
				</a>
				<a href="/docs/getting-started" class="btn-ghost">
					Read the Docs
				</a>
			</div>

			<p class="mt-6 text-xs text-[var(--muted-foreground)] tracking-wide scroll-reveal">
				<span class="flex items-center justify-center flex-wrap gap-x-4 gap-y-1">
					<span>🦀 Built in Rust</span>
					<span>·</span>
					<span>🔒 Zero telemetry</span>
					<span>·</span>
					<span>⚡ Pre-commit hooks</span>
					<span>·</span>
					<span>MIT License</span>
				</span>
			</p>
		</div>
	</section>

	<!-- ====== FOOTER ====== -->
	<footer class="border-t border-[var(--border)] relative z-10" style="background: var(--background);">
		<div class="section py-8">
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
</div>

<style>
	/* ---- Floating Blobs ---- */
	.blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.4;
	}

	.blob-1 {
		width: 400px;
		height: 400px;
		background: oklch(0.65 0.22 270 / 0.15);
		top: -100px;
		left: -100px;
		animation: blob-drift-1 20s ease-in-out infinite;
	}

	.blob-2 {
		width: 350px;
		height: 350px;
		background: oklch(0.7 0.15 240 / 0.1);
		top: 40%;
		right: -80px;
		animation: blob-drift-2 25s ease-in-out infinite;
	}

	.blob-3 {
		width: 300px;
		height: 300px;
		background: oklch(0.72 0.19 145 / 0.08);
		bottom: 10%;
		left: 20%;
		animation: blob-drift-3 22s ease-in-out infinite;
	}

	@keyframes blob-drift-1 {
		0%, 100% { transform: translate(0, 0) scale(1); }
		33% { transform: translate(60px, 40px) scale(1.1); }
		66% { transform: translate(-30px, 60px) scale(0.95); }
	}

	@keyframes blob-drift-2 {
		0%, 100% { transform: translate(0, 0) scale(1); }
		33% { transform: translate(-50px, -30px) scale(1.05); }
		66% { transform: translate(40px, -50px) scale(0.9); }
	}

	@keyframes blob-drift-3 {
		0%, 100% { transform: translate(0, 0) scale(1); }
		33% { transform: translate(30px, -40px) scale(1.08); }
		66% { transform: translate(-40px, 20px) scale(0.95); }
	}

	/* Reduce motion for blobs */
	@media (prefers-reduced-motion: reduce) {
		.blob {
			animation: none !important;
		}
	}
</style>
