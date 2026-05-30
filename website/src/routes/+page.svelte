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

<div style="background: var(--background);">

	<!-- ====== S1 — HERO (TALL) ====== -->
	<section class="relative flex items-center justify-center" style="min-height: 100vh; padding: 6rem 1.5rem 4rem;">
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

			<!-- Headline -->
			<h1 class="mt-8 mb-0 animate-fade-in-up delay-100" style="font-size: clamp(3rem, 6vw, 4.5rem); font-weight: 700; letter-spacing: -0.035em; color: var(--foreground); line-height: 1.05;">
				Review code.<br />
				<span style="background: linear-gradient(135deg, oklch(0.65 0.22 270), oklch(0.7 0.15 240)); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;">Ship faster.</span>
			</h1>

			<!-- Subtitle -->
			<p class="max-w-xl mx-auto mt-6 animate-fade-in-up delay-200" style="font-size: 18px; color: var(--muted-foreground); line-height: 1.5; letter-spacing: -0.005em;">
				cora catches bugs, security issues, and style violations before they merge. CLI-first. BYOK. Runs in your terminal.
			</p>

			<!-- Install Terminal -->
			<div class="max-w-md w-full mx-auto mt-10 animate-fade-in-up delay-300">
				<div class="terminal" style="position: relative;">
					<div class="terminal-header">
						<span class="terminal-dot terminal-dot-red"></span>
						<span class="terminal-dot terminal-dot-yellow"></span>
						<span class="terminal-dot terminal-dot-green"></span>
						<span class="terminal-title">Terminal</span>
					</div>
					<div class="terminal-body" style="position: relative;">
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

			<!-- CTAs -->
			<div class="flex flex-wrap justify-center gap-4 items-center mt-8 animate-fade-in-up delay-400">
				<a href="#quick-start" class="btn-primary">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
					Get Started
				</a>
				<a href="https://github.com/nousresearch/cora-cli" target="_blank" rel="noopener" class="btn-ghost">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
					View on GitHub
				</a>
			</div>

			<!-- Bottom text -->
			<p class="mt-6 animate-fade-in-up delay-500" style="font-size: 12px; color: var(--muted-foreground); letter-spacing: 0.01em;">
				MIT License &middot; No account &middot; OpenAI &middot; Anthropic &middot; Groq &middot; Ollama
			</p>
		</div>
	</section>

	<!-- ====== S2 — KPI STATS (COMPACT) ====== -->
	<section class="section section-compact">
		<p class="text-center mb-8 scroll-reveal" style="font-size: 12px; font-weight: 500; color: var(--muted-foreground); text-transform: uppercase; letter-spacing: 0.08em;">
			Trusted by developers who ship fast
		</p>
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4 max-w-3xl mx-auto">
			<div class="glass-card text-center scroll-reveal" style="padding-top: 2rem; padding-bottom: 2rem;">
				<div style="font-size: 32px; font-weight: 700; color: var(--foreground); letter-spacing: -0.025em; line-height: 1.2;">5</div>
				<div style="font-size: 14px; color: var(--muted-foreground); margin-top: 0.5rem;">AI Providers</div>
			</div>
			<div class="glass-card text-center scroll-reveal" style="padding-top: 2rem; padding-bottom: 2rem; transition-delay: 100ms;">
				<div style="font-size: 32px; font-weight: 700; color: var(--foreground); letter-spacing: -0.025em; line-height: 1.2;">&lt; 3s</div>
				<div style="font-size: 14px; color: var(--muted-foreground); margin-top: 0.5rem;">Review Time</div>
			</div>
			<div class="glass-card text-center scroll-reveal" style="padding-top: 2rem; padding-bottom: 2rem; transition-delay: 200ms;">
				<div style="font-size: 32px; font-weight: 700; color: var(--foreground); letter-spacing: -0.025em; line-height: 1.2;">Zero</div>
				<div style="font-size: 14px; color: var(--muted-foreground); margin-top: 0.5rem;">Config Required</div>
			</div>
		</div>
	</section>

	<!-- ====== S3 — LIVE TERMINAL DEMO (TALL) ====== -->
	<section class="section section-tall" id="demo-terminal">
		<h2 class="text-center scroll-reveal" style="font-size: 32px; font-weight: 700; color: var(--foreground); letter-spacing: -0.025em; line-height: 1.2;">
			See it in action
		</h2>
		<p class="text-center mt-3 scroll-reveal" style="color: var(--muted-foreground); max-width: 32rem; margin-left: auto; margin-right: auto; font-size: 14px;">
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
						<div style="min-height: 1.45em; color: {terminalOutput[i]?.color || 'var(--foreground)'};">{line}</div>
					{/each}
					{#if terminalComplete}
						<span class="typing-cursor"></span>
					{/if}
				</div>
			</div>
		</div>
	</section>

	<!-- ====== S4 — HOW IT WORKS (COMPACT) ====== -->
	<section class="section section-compact">
		<h2 class="text-center scroll-reveal" style="font-size: 32px; font-weight: 700; color: var(--foreground); letter-spacing: -0.025em; line-height: 1.2;">
			How it works
		</h2>
		<p class="text-center mt-3 scroll-reveal" style="color: var(--muted-foreground); max-width: 32rem; margin-left: auto; margin-right: auto; font-size: 14px;">
			Three steps from code to confidence.
		</p>

		<div class="flex flex-col md:flex-row items-stretch mt-10" style="gap: 1.5rem;">
			<!-- Step 1 -->
			<div class="glass-card flex-1 text-center scroll-reveal">
				<div style="font-size: 24px; font-weight: 700; color: var(--accent); letter-spacing: -0.02em; font-family: var(--font-mono); opacity: 0.4;">01</div>
				<div class="flex justify-center mt-4">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
				</div>
				<h3 class="mt-4" style="font-size: 20px; font-weight: 600; color: var(--foreground); letter-spacing: -0.01em; line-height: 1.35;">Write code</h3>
				<p class="mt-2" style="font-size: 14px; color: var(--muted-foreground);">Push your changes as normal. cora only sees your diff.</p>
			</div>

			<!-- Connector -->
			<div class="connect-line hidden md:flex scroll-reveal">
				<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
			</div>

			<!-- Step 2 -->
			<div class="glass-card flex-1 text-center scroll-reveal" style="transition-delay: 100ms;">
				<div style="font-size: 24px; font-weight: 700; color: var(--accent); letter-spacing: -0.02em; font-family: var(--font-mono); opacity: 0.4;">02</div>
				<div class="flex justify-center mt-4">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"/></svg>
				</div>
				<h3 class="mt-4" style="font-size: 20px; font-weight: 600; color: var(--foreground); letter-spacing: -0.01em; line-height: 1.35;">Review with AI</h3>
				<p class="mt-2" style="font-size: 14px; color: var(--muted-foreground);">cora analyzes your diff with the LLM of your choice.</p>
			</div>

			<!-- Connector -->
			<div class="connect-line hidden md:flex scroll-reveal" style="transition-delay: 100ms;">
				<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
			</div>

			<!-- Step 3 -->
			<div class="glass-card flex-1 text-center scroll-reveal" style="transition-delay: 200ms;">
				<div style="font-size: 24px; font-weight: 700; color: var(--accent); letter-spacing: -0.02em; font-family: var(--font-mono); opacity: 0.4;">03</div>
				<div class="flex justify-center mt-4">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--success)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 11-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
				</div>
				<h3 class="mt-4" style="font-size: 20px; font-weight: 600; color: var(--foreground); letter-spacing: -0.01em; line-height: 1.35;">Ship with confidence</h3>
				<p class="mt-2" style="font-size: 14px; color: var(--muted-foreground);">Merge clean, production-ready code. Every time.</p>
			</div>
		</div>
	</section>

	<!-- ====== S5 — FEATURES (TALL) ====== -->
	<section class="section section-tall">
		<h2 class="text-center scroll-reveal" style="font-size: 32px; font-weight: 700; color: var(--foreground); letter-spacing: -0.025em; line-height: 1.2;">
			Built for developers who value control
		</h2>
		<p class="text-center mt-3 scroll-reveal" style="color: var(--muted-foreground); font-size: 14px;">
			Everything you need, nothing you don't.
		</p>

		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mt-10">
			<!-- Feature 1: AI Code Review -->
			<div class="glass-card scroll-reveal">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/><path d="M11 8v6"/><path d="M8 11h6"/></svg>
				</div>
				<h3 class="mt-4" style="font-size: 18px; font-weight: 600; color: var(--foreground);">AI Code Review</h3>
				<p class="mt-1" style="font-size: 14px; color: var(--accent);">Diff, branch, or full scan</p>
				<p class="mt-3" style="font-size: 14px; color: var(--muted-foreground); line-height: 1.5;">Three review modes: staged diff, branch comparison, or full project scan. LLM-powered analysis catches bugs, security issues, and style violations.</p>
			</div>

			<!-- Feature 2: BYOK -->
			<div class="glass-card scroll-reveal" style="transition-delay: 100ms;">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 11-7.778 7.778 5.5 5.5 0 017.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/></svg>
				</div>
				<h3 class="mt-4" style="font-size: 18px; font-weight: 600; color: var(--foreground);">Bring Your Own Key</h3>
				<p class="mt-1" style="font-size: 14px; color: var(--accent);">No subscriptions, no lock-in</p>
				<p class="mt-3" style="font-size: 14px; color: var(--muted-foreground); line-height: 1.5;">Uses YOUR OpenAI, Anthropic, Groq, Ollama, or Z.AI API key. No data stored on our servers. You control the model, you control the cost.</p>
			</div>

			<!-- Feature 3: Pre-commit Hooks -->
			<div class="glass-card scroll-reveal" style="transition-delay: 200ms;">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22V8"/><path d="M5 12H2a10 10 0 0020 0h-3"/><circle cx="12" cy="5" r="3"/></svg>
				</div>
				<h3 class="mt-4" style="font-size: 18px; font-weight: 600; color: var(--foreground);">Pre-commit Hooks</h3>
				<p class="mt-1" style="font-size: 14px; color: var(--accent);">Review before you push</p>
				<p class="mt-3" style="font-size: 14px; color: var(--muted-foreground); line-height: 1.5;">Install once. Every commit gets reviewed automatically. Block bad code from entering your branch before it ships.</p>
			</div>

			<!-- Feature 4: Incremental Scan -->
			<div class="glass-card scroll-reveal" style="transition-delay: 300ms;">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>
				</div>
				<h3 class="mt-4" style="font-size: 18px; font-weight: 600; color: var(--foreground);">Incremental Scan</h3>
				<p class="mt-1" style="font-size: 14px; color: var(--accent);">Only scan what changed</p>
				<p class="mt-3" style="font-size: 14px; color: var(--muted-foreground); line-height: 1.5;">SHA256 content hash cache. First scan indexes your codebase. Subsequent scans only review new or modified files.</p>
			</div>

			<!-- Feature 5: SARIF Output -->
			<div class="glass-card scroll-reveal" style="transition-delay: 400ms;">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><path d="M9 12l2 2 4-4"/></svg>
				</div>
				<h3 class="mt-4" style="font-size: 18px; font-weight: 600; color: var(--foreground);">SARIF Output</h3>
				<p class="mt-1" style="font-size: 14px; color: var(--accent);">GitHub Code Scanning</p>
				<p class="mt-3" style="font-size: 14px; color: var(--muted-foreground); line-height: 1.5;">Upload review findings directly to GitHub's Security tab. Track issues across PRs. Works with any CI/CD pipeline.</p>
			</div>

			<!-- Feature 6: Fully Private -->
			<div class="glass-card scroll-reveal" style="transition-delay: 500ms;">
				<div class="feature-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0110 0v4"/><circle cx="12" cy="16" r="1"/></svg>
				</div>
				<h3 class="mt-4" style="font-size: 18px; font-weight: 600; color: var(--foreground);">Fully Private</h3>
				<p class="mt-1" style="font-size: 14px; color: var(--accent);">Your code stays yours</p>
				<p class="mt-3" style="font-size: 14px; color: var(--muted-foreground); line-height: 1.5;">Runs entirely on your machine. No cloud, no telemetry, no data leaving your network. Perfect for Gitea and air-gapped environments.</p>
			</div>
		</div>
	</section>

	<!-- ====== S6 — COMPARISON TABLE (COMPACT) ====== -->
	<section class="section section-compact">
		<h2 class="text-center scroll-reveal" style="font-size: 32px; font-weight: 700; color: var(--foreground); letter-spacing: -0.025em; line-height: 1.2;">
			Why developers choose cora
		</h2>
		<p class="text-center mt-3 scroll-reveal" style="color: var(--muted-foreground); font-size: 14px;">
			Side-by-side with popular code review tools.
		</p>

		<div class="glass-card scroll-reveal mt-10" style="padding: 0; max-width: 56rem; margin-left: auto; margin-right: auto; overflow: hidden;">
			<div style="overflow-x: auto;">
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
							<td style="color: var(--muted-foreground);">&mdash;</td>
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
							<td style="font-weight: 600;">Cost</td>
							<td class="cora-col" style="font-weight: 600;">Free + API</td>
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

	<!-- ====== S7 — QUICK START + CTA + FOOTER (TALL) ====== -->
	<section class="section section-tall" id="quick-start">
		<h2 class="text-center scroll-reveal" style="font-size: 32px; font-weight: 700; color: var(--foreground); letter-spacing: -0.025em; line-height: 1.2;">
			Start in 30 seconds
		</h2>
		<p class="text-center mt-3 scroll-reveal" style="color: var(--muted-foreground); font-size: 14px;">
			No account required. No subscription. No cloud.
		</p>

		<div class="flex flex-col mt-10" style="max-width: 40rem; margin-left: auto; margin-right: auto; gap: 1.5rem;">
			<!-- Step 1 -->
			<div class="timeline-step scroll-reveal">
				<div class="timeline-number">1</div>
				<div>
					<h3 style="font-size: 18px; font-weight: 600; color: var(--foreground); letter-spacing: -0.01em; line-height: 1.35;">Install</h3>
					<p class="mt-1 mb-4" style="font-size: 14px; color: var(--muted-foreground);">Single binary, no dependencies.</p>
					<div class="terminal" style="font-size: 13px;">
						<div class="terminal-header">
							<span class="terminal-dot terminal-dot-red"></span>
							<span class="terminal-dot terminal-dot-yellow"></span>
							<span class="terminal-dot terminal-dot-green"></span>
						</div>
						<div class="terminal-body" style="padding: 0.75rem 1rem; font-size: 13px;">
							<span class="syntax-cmd">$</span> <span class="syntax-highlight">cargo install</span> <span class="syntax-string">cora</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Step 2 -->
			<div class="timeline-step scroll-reveal" style="transition-delay: 100ms;">
				<div class="timeline-number">2</div>
				<div>
					<h3 style="font-size: 18px; font-weight: 600; color: var(--foreground); letter-spacing: -0.01em; line-height: 1.35;">Initialize</h3>
					<p class="mt-1 mb-4" style="font-size: 14px; color: var(--muted-foreground);">Creates <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">.cora.yaml</code> config file.</p>
					<div class="terminal" style="font-size: 13px;">
						<div class="terminal-header">
							<span class="terminal-dot terminal-dot-red"></span>
							<span class="terminal-dot terminal-dot-yellow"></span>
							<span class="terminal-dot terminal-dot-green"></span>
						</div>
						<div class="terminal-body" style="padding: 0.75rem 1rem; font-size: 13px;">
							<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora init</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Step 3 -->
			<div class="timeline-step scroll-reveal" style="transition-delay: 200ms;">
				<div class="timeline-number">3</div>
				<div>
					<h3 style="font-size: 18px; font-weight: 600; color: var(--foreground); letter-spacing: -0.01em; line-height: 1.35;">Review</h3>
					<p class="mt-1 mb-4" style="font-size: 14px; color: var(--muted-foreground);">Review your staged changes.</p>
					<div class="terminal" style="font-size: 13px;">
						<div class="terminal-header">
							<span class="terminal-dot terminal-dot-red"></span>
							<span class="terminal-dot terminal-dot-yellow"></span>
							<span class="terminal-dot terminal-dot-green"></span>
						</div>
						<div class="terminal-body" style="padding: 0.75rem 1rem; font-size: 13px;">
							<span class="syntax-cmd">$</span> <span class="syntax-flag">CORA_API_KEY</span>=<span class="syntax-string">key</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span>
						</div>
					</div>
				</div>
			</div>

			<!-- Step 4 -->
			<div class="timeline-step scroll-reveal" style="transition-delay: 300ms;">
				<div class="timeline-number">4</div>
				<div>
					<h3 style="font-size: 18px; font-weight: 600; color: var(--foreground); letter-spacing: -0.01em; line-height: 1.35;">Done</h3>
					<p class="mt-1" style="font-size: 14px; color: var(--success);">That's it. No account. No subscription.</p>
				</div>
			</div>
		</div>

		<!-- CTA -->
		<div class="text-center mt-24 scroll-reveal">
			<h2 style="font-size: 32px; font-weight: 700; color: var(--foreground); letter-spacing: -0.025em; line-height: 1.2;">
				Ready to ship better code?
			</h2>
			<p class="mt-3" style="color: var(--muted-foreground); font-size: 14px;">
				No account. No subscription. No cloud.
			</p>
			<div class="flex flex-wrap justify-center gap-4 items-center mt-8">
				<a href="/docs" class="btn-primary">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
					Get Started
				</a>
				<a href="https://github.com/nousresearch/cora-cli" target="_blank" rel="noopener" class="btn-ghost">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
					Star on GitHub
				</a>
			</div>
		</div>

		<!-- Footer -->
		<footer style="border-top: 1px solid var(--border); margin-top: 6rem; padding: 2rem 1.5rem;">
			<div class="max-w-6xl mx-auto flex flex-col md:flex-row items-center justify-between gap-4">
				<div>
					<span style="font-size: 14px; font-weight: 600; color: var(--foreground);">cora</span>
					<span style="font-size: 12px; color: var(--muted-foreground); margin-left: 0.75rem;">MIT License</span>
				</div>
				<div class="flex items-center gap-6">
					<a href="https://github.com/nousresearch/cora-cli" target="_blank" rel="noopener" style="font-size: 14px; color: var(--muted-foreground); text-decoration: none; transition: color 0.2s ease;">GitHub</a>
					<a href="/docs" style="font-size: 14px; color: var(--muted-foreground); text-decoration: none; transition: color 0.2s ease;">Docs</a>
				</div>
			</div>
		</footer>
	</section>
</div>
