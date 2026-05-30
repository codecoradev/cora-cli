<script lang="ts">
	import { onMount } from 'svelte';

	// Terminal typing animation state
	let terminalLines = $state<string[]>([]);
	let terminalComplete = $state(false);
	let demoStarted = $state(false);

	const terminalOutput = [
		{ text: '$ cora review --staged', color: 'var(--muted-foreground)' },
		{ text: '', color: '' },
		{ text: 'Analyzing 3 files...', color: 'var(--muted-foreground)' },
		{ text: '\u2713 src/auth/login.ts \u2014 2 issues found', color: 'var(--success)' },
		{ text: '  \u26A0 Line 42: Potential SQL injection', color: 'var(--warning)' },
		{ text: '  \u26A0 Line 87: Hardcoded secret', color: 'var(--warning)' },
		{ text: '\u2713 src/utils/parser.ts \u2014 clean', color: 'var(--muted-foreground)' },
		{ text: '\u2713 src/api/routes.ts \u2014 1 issue found', color: 'var(--success)' },
		{ text: '  \u2717 Line 23: Missing error handling', color: 'var(--destructive)' },
		{ text: '', color: '' },
		{ text: '3 issues found in 3 files', color: 'var(--foreground)' },
	];

	let copyClicked = $state(false);
	function copyInstall() {
		navigator.clipboard.writeText('cargo install cora');
		copyClicked = true;
		setTimeout(() => { copyClicked = false; }, 2000);
	}

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
						}, 200);
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
	<title>cora — AI Code Review CLI</title>
	<meta name="description" content="cora is a CLI-first AI code reviewer. BYOK, zero config, runs in your terminal. Your code never leaves your machine." />
</svelte:head>

<div class="bg-[var(--background)]">

	<!-- ====== S1 — HERO ====== -->
	<section class="section section-hero relative flex items-center justify-center min-h-[calc(100vh-3.5rem)]">
		<!-- Subtle radial gradient glow -->
		<div class="absolute inset-0 pointer-events-none" style="background: radial-gradient(ellipse 50% 40% at 50% 40%, oklch(0.65 0.22 270 / 0.04), transparent);"></div>

		<div class="relative z-10 max-w-3xl mx-auto text-center">
			<!-- Badge -->
			<div class="animate-fade-in-up">
				<span class="accent-badge">
					<span class="badge-dot"></span>
					AI Code Review for Developers
				</span>
			</div>

			<!-- Headline — mt-8 (32px) badge-to-heading, mb-0 -->
			<h1 class="mt-8 mb-0 animate-fade-in-up delay-100 text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold text-[var(--foreground)] leading-none -tracking-tighter">
				Review code.<br />
				<span class="bg-gradient-to-br from-[oklch(0.65_0.22_270)] to-[oklch(0.7_0.15_240)] bg-clip-text text-transparent">Ship faster.</span>
			</h1>

			<!-- Subtitle — mt-6 (24px) heading-to-subtitle -->
			<p class="max-w-xl mx-auto mt-6 animate-fade-in-up delay-200 text-base sm:text-lg text-[var(--muted-foreground)] leading-normal sm:leading-relaxed -tracking-tight">
				cora catches bugs, security issues, and style violations before they merge. CLI-first. BYOK. Runs in your terminal.
			</p>

			<!-- Install Terminal — mt-10 (40px) subtitle-to-terminal -->
			<div class="max-w-lg w-full mx-auto mt-10 animate-fade-in-up delay-300">
				<div class="terminal relative">
					<div class="terminal-header">
						<span class="terminal-dot terminal-dot-red"></span>
						<span class="terminal-dot terminal-dot-yellow"></span>
						<span class="terminal-dot terminal-dot-green"></span>
						<span class="terminal-title">Terminal</span>
					</div>
					<div class="terminal-body relative">
						<span class="syntax-cmd">$</span>
						<span class="syntax-highlight"> cargo install</span>
						<span class="syntax-string"> cora</span>
						<span class="typing-cursor"></span>
						<button class="copy-btn" onclick={copyClicked ? undefined : copyInstall} class:copied={copyClicked} aria-label="Copy command">
							{#if copyClicked}
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
							{:else}
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
							{/if}
						</button>
					</div>
				</div>
			</div>

			<!-- CTAs — mt-8 (32px) terminal-to-cta -->
			<div class="flex flex-wrap justify-center gap-4 items-center mt-8 animate-fade-in-up delay-400">
				<a href="#quick-start" class="btn-primary">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
					Get Started
				</a>
				<a href="https://github.com/ajianaz/cora-cli" target="_blank" rel="noopener" class="btn-ghost">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
					View on GitHub
				</a>
			</div>

			<!-- Bottom text — mt-6 (24px) cta-to-text -->
			<p class="mt-6 animate-fade-in-up delay-500 text-xs text-[var(--muted-foreground)] tracking-wide">
				MIT License &middot; No account &middot; OpenAI &middot; Anthropic &middot; Groq &middot; Ollama
			</p>
		</div>
	</section>

	<!-- ====== S2 — KPI STATS ====== -->
	<section class="section section-compact">
		<p class="text-center mb-10 scroll-reveal text-xs font-medium text-[var(--muted-foreground)] uppercase tracking-widest">
			Trusted by developers who ship fast
		</p>
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4 max-w-3xl mx-auto">
			<div class="glass-card text-center scroll-reveal py-8">
				<div class="text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">5</div>
				<div class="text-sm text-[var(--muted-foreground)] mt-2">AI Providers</div>
			</div>
			<div class="glass-card text-center scroll-reveal py-8 [transition-delay:100ms]">
				<div class="text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">&lt; 3s</div>
				<div class="text-sm text-[var(--muted-foreground)] mt-2">Review Time</div>
			</div>
			<div class="glass-card text-center scroll-reveal py-8 [transition-delay:200ms]">
				<div class="text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">Zero</div>
				<div class="text-sm text-[var(--muted-foreground)] mt-2">Config Required</div>
			</div>
		</div>
	</section>

	<!-- ====== S3 — LIVE TERMINAL DEMO ====== -->
	<section class="section section-tall" id="demo-terminal">
		<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
			See it in action
		</h2>
		<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)] max-w-[32rem] mx-auto">
			Run cora against staged changes. Results in seconds, not minutes.
		</p>

		<div class="max-w-2xl mx-auto mt-10 scroll-reveal">
			<div class="terminal">
				<div class="terminal-header">
					<span class="terminal-dot terminal-dot-red"></span>
					<span class="terminal-dot terminal-dot-yellow"></span>
					<span class="terminal-dot terminal-dot-green"></span>
					<span class="terminal-title">cora \u2014 review</span>
				</div>
				<div class="terminal-body">
					{#each terminalLines as line, i}
						<div class="min-h-[1.45em]" style="color: {terminalOutput[i]?.color || 'var(--foreground)'};">{line}</div>
					{/each}
					{#if terminalComplete}
						<span class="typing-cursor"></span>
					{/if}
				</div>
			</div>
		</div>
	</section>

	<!-- ====== S4 — HOW IT WORKS ====== -->
	<section class="section section-compact">
		<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
			How it works
		</h2>
		<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)] max-w-[32rem] mx-auto">
			Three steps from code to confidence.
		</p>

		<div class="flex flex-col md:flex-row items-stretch mt-10 gap-6">
			<!-- Step 1 -->
			<div class="glass-card flex-1 text-center scroll-reveal">
				<div class="text-2xl font-bold text-[var(--accent)] -tracking-tight font-mono opacity-50">01</div>
				<div class="flex justify-center mt-4">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
				</div>
				<h3 class="mt-4 text-xl font-semibold text-[var(--foreground)] -tracking-tight leading-snug">Write code</h3>
				<p class="mt-2 text-sm text-[var(--muted-foreground)]">Push your changes as normal. cora only sees your diff.</p>
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
				<h3 class="mt-4 text-xl font-semibold text-[var(--foreground)] -tracking-tight leading-snug">Review with AI</h3>
				<p class="mt-2 text-sm text-[var(--muted-foreground)]">cora analyzes your diff with the LLM of your choice.</p>
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
				<h3 class="mt-4 text-xl font-semibold text-[var(--foreground)] -tracking-tight leading-snug">Ship with confidence</h3>
				<p class="mt-2 text-sm text-[var(--muted-foreground)]">Merge clean, production-ready code. Every time.</p>
			</div>
		</div>
	</section>

	<!-- ====== S5 — FEATURES ====== -->
	<section class="section section-tall">
		<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
			Built for developers who value control
		</h2>
		<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)]">
			Everything you need, nothing you don't.
		</p>

		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-10">
			<!-- Feature 1: AI Code Review -->
			<div class="glass-card scroll-reveal">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/><path d="M11 8v6"/><path d="M8 11h6"/></svg>
				</div>
				<h3 class="mt-4 text-lg font-semibold text-[var(--foreground)]">AI Code Review</h3>
				<p class="mt-2 text-sm text-[var(--accent)]">Diff, branch, or full scan</p>
				<p class="mt-2 text-sm text-[var(--muted-foreground)] leading-relaxed">Three review modes: staged diff, branch comparison, or full project scan. LLM-powered analysis catches bugs, security issues, and style violations.</p>
			</div>

			<!-- Feature 2: BYOK -->
			<div class="glass-card scroll-reveal [transition-delay:100ms]">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 11-7.778 7.778 5.5 5.5 0 017.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/></svg>
				</div>
				<h3 class="mt-4 text-lg font-semibold text-[var(--foreground)]">Bring Your Own Key</h3>
				<p class="mt-2 text-sm text-[var(--accent)]">No subscriptions, no lock-in</p>
				<p class="mt-2 text-sm text-[var(--muted-foreground)] leading-relaxed">Uses YOUR OpenAI, Anthropic, Groq, Ollama, or Z.AI API key. No data stored on our servers. You control the model, you control the cost.</p>
			</div>

			<!-- Feature 3: Pre-commit Hooks -->
			<div class="glass-card scroll-reveal [transition-delay:200ms]">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22V8"/><path d="M5 12H2a10 10 0 0020 0h-3"/><circle cx="12" cy="5" r="3"/></svg>
				</div>
				<h3 class="mt-4 text-lg font-semibold text-[var(--foreground)]">Pre-commit Hooks</h3>
				<p class="mt-2 text-sm text-[var(--accent)]">Review before you push</p>
				<p class="mt-2 text-sm text-[var(--muted-foreground)] leading-relaxed">Install once. Every commit gets reviewed automatically. Block bad code from entering your branch before it ships.</p>
			</div>

			<!-- Feature 4: Incremental Scan -->
			<div class="glass-card scroll-reveal [transition-delay:300ms]">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>
				</div>
				<h3 class="mt-4 text-lg font-semibold text-[var(--foreground)]">Incremental Scan</h3>
				<p class="mt-2 text-sm text-[var(--accent)]">Only scan what changed</p>
				<p class="mt-2 text-sm text-[var(--muted-foreground)] leading-relaxed">SHA256 content hash cache. First scan indexes your codebase. Subsequent scans only review new or modified files.</p>
			</div>

			<!-- Feature 5: SARIF Output -->
			<div class="glass-card scroll-reveal [transition-delay:400ms]">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><path d="M9 12l2 2 4-4"/></svg>
				</div>
				<h3 class="mt-4 text-lg font-semibold text-[var(--foreground)]">SARIF Output</h3>
				<p class="mt-2 text-sm text-[var(--accent)]">GitHub Code Scanning</p>
				<p class="mt-2 text-sm text-[var(--muted-foreground)] leading-relaxed">Upload review findings directly to GitHub's Security tab. Track issues across PRs. Works with any CI/CD pipeline.</p>
			</div>

			<!-- Feature 6: Fully Private -->
			<div class="glass-card scroll-reveal [transition-delay:500ms]">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0110 0v4"/><circle cx="12" cy="16" r="1"/></svg>
				</div>
				<h3 class="mt-4 text-lg font-semibold text-[var(--foreground)]">Fully Private</h3>
				<p class="mt-2 text-sm text-[var(--accent)]">Your code stays yours</p>
				<p class="mt-2 text-sm text-[var(--muted-foreground)] leading-relaxed">Runs entirely on your machine. No cloud, no telemetry, no data leaving your network. Perfect for Gitea and air-gapped environments.</p>
			</div>
		</div>
	</section>

	<!-- ====== S6 — COMPARISON TABLE ====== -->
	<section class="section section-compact">
		<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
			Why developers choose cora
		</h2>
		<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)]">
			Side-by-side with popular code review tools.
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
						<tr>
							<td>BYOK</td>
							<td class="cora-col"><span class="symbol-check">&#10003;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
							<td class="text-[var(--muted-foreground)]">&mdash;</td>
						</tr>
						<tr>
							<td>Self-hosted</td>
							<td class="cora-col"><span class="symbol-check">&#10003;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
							<td><span class="symbol-check">&#10003;</span></td>
						</tr>
						<tr>
							<td>Gitea</td>
							<td class="cora-col"><span class="symbol-check">&#10003;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
							<td><span class="symbol-check">&#10003;</span></td>
						</tr>
						<tr>
							<td>CLI</td>
							<td class="cora-col"><span class="symbol-check">&#10003;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
						</tr>
						<tr>
							<td>Pre-commit</td>
							<td class="cora-col"><span class="symbol-check">&#10003;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
							<td><span class="symbol-cross">&#10007;</span></td>
						</tr>
						<tr>
							<td>SARIF</td>
							<td class="cora-col"><span class="symbol-check">&#10003;</span></td>
							<td><span class="symbol-check">&#10003;</span></td>
							<td><span class="symbol-check">&#10003;</span></td>
							<td><span class="symbol-check">&#10003;</span></td>
						</tr>
						<tr>
							<td class="font-semibold">Cost</td>
							<td class="cora-col font-semibold">Free + API</td>
							<td>$12-39/mo</td>
							<td>$10-39/mo</td>
							<td>Free / $150+</td>
						</tr>
						<tr>
							<td>License</td>
							<td class="cora-col">MIT</td>
							<td>Apache 2.0</td>
							<td>Proprietary</td>
							<td>LGPL</td>
						</tr>
					</tbody>
				</table>
			</div>
		</div>
	</section>

	<!-- ====== S7 — QUICK START + CTA + FOOTER ====== -->
	<section class="section section-tall" id="quick-start">
		<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
			Start in 30 seconds
		</h2>
		<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)]">
			No account required. No subscription. No cloud.
		</p>

		<div class="flex flex-col mt-10 max-w-[40rem] mx-auto gap-6">
			<!-- Step 1 -->
			<div class="timeline-step scroll-reveal">
				<div class="timeline-number">1</div>
				<div>
					<h3 class="text-lg font-semibold text-[var(--foreground)] -tracking-tight leading-snug">Install</h3>
					<p class="mt-1 mb-4 text-sm text-[var(--muted-foreground)]">Single binary, no dependencies.</p>
					<div class="terminal">
						<div class="terminal-header">
							<span class="terminal-dot terminal-dot-red"></span>
							<span class="terminal-dot terminal-dot-yellow"></span>
							<span class="terminal-dot terminal-dot-green"></span>
						</div>
						<div class="terminal-body py-3 px-4">
							<span class="syntax-cmd">$</span> <span class="syntax-highlight">cargo install</span> <span class="syntax-string">cora</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Step 2 -->
			<div class="timeline-step scroll-reveal [transition-delay:100ms]">
				<div class="timeline-number">2</div>
				<div>
					<h3 class="text-lg font-semibold text-[var(--foreground)] -tracking-tight leading-snug">Initialize</h3>
					<p class="mt-1 mb-4 text-sm text-[var(--muted-foreground)]">Creates <code class="text-[var(--accent)] font-mono text-[13px]">.cora.yaml</code> config file.</p>
					<div class="terminal">
						<div class="terminal-header">
							<span class="terminal-dot terminal-dot-red"></span>
							<span class="terminal-dot terminal-dot-yellow"></span>
							<span class="terminal-dot terminal-dot-green"></span>
						</div>
						<div class="terminal-body py-3 px-4">
							<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora init</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Step 3 -->
			<div class="timeline-step scroll-reveal [transition-delay:200ms]">
				<div class="timeline-number">3</div>
				<div>
					<h3 class="text-lg font-semibold text-[var(--foreground)] -tracking-tight leading-snug">Review</h3>
					<p class="mt-1 mb-4 text-sm text-[var(--muted-foreground)]">Review your staged changes.</p>
					<div class="terminal">
						<div class="terminal-header">
							<span class="terminal-dot terminal-dot-red"></span>
							<span class="terminal-dot terminal-dot-yellow"></span>
							<span class="terminal-dot terminal-dot-green"></span>
						</div>
						<div class="terminal-body py-3 px-4">
							<span class="syntax-cmd">$</span> <span class="syntax-flag">CORA_API_KEY</span>=<span class="syntax-string">key</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Step 4 -->
			<div class="timeline-step scroll-reveal [transition-delay:300ms]">
				<div class="timeline-number">4</div>
				<div>
					<h3 class="text-lg font-semibold text-[var(--foreground)] -tracking-tight leading-snug">Done</h3>
					<p class="mt-1 text-sm text-[var(--success)]">That's it. No account. No subscription.</p>
				</div>
			</div>
		</div>

		<!-- CTA — mt-20 (80px) separation from steps -->
		<div class="text-center mt-20 scroll-reveal">
			<h2 class="text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
				Ready to ship better code?
			</h2>
			<p class="mt-3 text-sm text-[var(--muted-foreground)]">
				No account. No subscription. No cloud.
			</p>
			<div class="flex flex-wrap justify-center gap-4 items-center mt-8">
				<a href="/docs" class="btn-primary">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
					Get Started
				</a>
				<a href="https://github.com/ajianaz/cora-cli" target="_blank" rel="noopener" class="btn-ghost">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
					Star on GitHub
				</a>
			</div>
		</div>

		<!-- Footer — mt-20 (80px) -->
		<footer class="border-t border-[var(--border)] mt-20 py-8">
			<div class="max-w-6xl mx-auto flex flex-col md:flex-row items-center justify-between gap-4">
				<div>
					<span class="text-sm font-semibold text-[var(--foreground)]">cora</span>
					<span class="text-xs text-[var(--muted-foreground)] ml-3">MIT License</span>
				</div>
				<div class="flex items-center gap-6">
					<a href="https://github.com/ajianaz/cora-cli" target="_blank" rel="noopener" class="text-sm text-[var(--muted-foreground)] no-underline transition-colors duration-200 min-h-11 inline-flex items-center hover:text-[var(--foreground)]">GitHub</a>
					<a href="/docs" class="text-sm text-[var(--muted-foreground)] no-underline transition-colors duration-200 min-h-11 inline-flex items-center hover:text-[var(--foreground)]">Docs</a>
				</div>
			</div>
		</footer>
	</section>
</div>
