<script lang="ts">
	import { onMount } from 'svelte';

	// Terminal typing animation state
	let terminalLines = $state<string[]>([]);
	let terminalComplete = $state(false);
	let mounted = $state(false);

	const terminalOutput = [
		{ text: '$ cora review --staged', cls: 'syntax-cmd' },
		{ text: 'Analyzing 3 files...', cls: 'syntax-comment' },
		{ text: '', cls: '' },
		{ text: '\u2713 src/auth/login.ts \u2014 2 issues found', cls: 'syntax-success' },
		{ text: '  \u26A0 Line 42: Potential SQL injection \u2014 use parameterized queries', cls: 'syntax-warning' },
		{ text: '  \u26A0 Line 87: Hardcoded secret \u2014 move to environment variable', cls: 'syntax-warning' },
		{ text: '\u2713 src/utils/parser.ts \u2014 clean', cls: 'syntax-success' },
		{ text: '\u2713 src/api/routes.ts \u2014 1 issue found', cls: 'syntax-success' },
		{ text: '  \u{1F534} Line 23: Missing error handling on async call', cls: 'syntax-error' },
		{ text: '', cls: '' },
		{ text: '3 issues found in 3 files', cls: 'syntax-highlight' },
	];

	onMount(() => {
		mounted = true;

		// Typing animation
		let lineIndex = 0;
		const typeInterval = setInterval(() => {
			if (lineIndex < terminalOutput.length) {
				terminalLines = [...terminalLines, terminalOutput[lineIndex].text];
				lineIndex++;
			} else {
				terminalComplete = true;
				clearInterval(typeInterval);
			}
		}, 280);

		return () => clearInterval(typeInterval);

		// Scroll reveal observer
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						entry.target.classList.add('visible');
					}
				});
			},
			{ threshold: 0.1, rootMargin: '0px 0px -40px 0px' }
		);

		document.querySelectorAll('.scroll-reveal').forEach((el) => observer.observe(el));

		return () => observer.disconnect();
	});

	let copyClicked = $state(false);
	function copyInstall() {
		navigator.clipboard.writeText('cargo install cora');
		copyClicked = true;
		setTimeout(() => { copyClicked = false; }, 2000);
	}

	// Code rain lines
	const codeSnippets = [
		'fn review(diff: Diff) -> Result',
		'const config = load(".cora.yaml")',
		'async function analyze(file) {',
		'  let issues = await llm.scan(',
		'    context: diff.context(),',
		'    rules: config.severity',
		'  )',
		'  return issues.filter()',
		'}',
		'pub fn main() -> anyhow',
		'  let args = Cli::parse()',
		'  match args.command {',
		'    Review(staged) => {',
		'      let diff = git.diff()',
		'      let results = ai.run(',
		'        &diff, &config',
		'      )',
		'      output::print(results)',
		'    }',
		'  }',
		'impl Provider for OpenAI',
		'  fn call(&self, prompt)',
		'    -> Response',
		'  {',
		'    self.client.complete(',
		'      model: "gpt-4o",',
		'      messages: prompt',
		'    )',
		'  }',
	];

	let rainLines = $state<Array<{ text: string; left: number; delay: number; duration: number }>>([]);

	onMount(() => {
		rainLines = Array.from({ length: 12 }, (_, i) => ({
			text: codeSnippets[i % codeSnippets.length],
			left: 5 + (i * 8.5) % 90,
			delay: Math.random() * 15,
			duration: 18 + Math.random() * 12,
		}));
	});
</script>

<svelte:head>
	<title>cora — AI Code Review CLI</title>
	<meta name="description" content="cora is a CLI-first AI code reviewer. BYOK, zero config, runs in your terminal. Your code never leaves your machine." />
</svelte:head>

<div class="min-h-screen" style="background: var(--bg-primary);">

	<!-- ====== HERO ====== -->
	<section class="hero-bg relative flex items-center justify-center" style="min-height: 100vh; padding: 6rem 1.5rem 4rem;">
		<!-- Code rain background -->
		<div class="code-rain">
			{#each rainLines as line}
				<div
					class="code-rain-line"
					style="left: {line.left}%; animation-delay: {line.delay}s; animation-duration: {line.duration}s;"
				>
					{line.text}
				</div>
			{/each}
		</div>

		<div class="relative z-10 max-w-3xl mx-auto text-center">
			<!-- Badge -->
			<div class="animate-fade-in-up">
				<span class="badge">
					<span class="badge-dot"></span>
					AI-powered code review for developers
				</span>
			</div>

			<!-- Headline -->
			<h1 class="mt-8 mb-6 animate-fade-in-up delay-1" style="font-size: clamp(2.5rem, 5vw, 4rem);">
				Review code.<br />
				<span class="gradient-text">Ship faster.</span>
			</h1>

			<!-- Subtitle -->
			<p class="mb-10 max-w-xl mx-auto animate-fade-in-up delay-2" style="font-size: 1.125rem; color: var(--text-secondary); line-height: 1.7;">
				cora catches bugs, security issues, and style violations before they merge.
				CLI-first, BYOK, runs in your terminal.
			</p>

			<!-- Install terminal -->
			<div class="max-w-lg mx-auto mb-8 animate-fade-in-up delay-3">
				<div class="terminal-window glow" style="position: relative;">
					<div class="terminal-bar">
						<span class="terminal-dot terminal-dot-red"></span>
						<span class="terminal-dot terminal-dot-yellow"></span>
						<span class="terminal-dot terminal-dot-green"></span>
						<span class="terminal-title">terminal</span>
					</div>
					<div class="terminal-body">
						<span class="syntax-cmd">$</span>
						<span class="syntax-highlight"> cargo install</span>
						<span class="syntax-string"> cora</span>
						<span class="typing-cursor"></span>
					</div>
					<!-- Copy button -->
					<button class="copy-btn" onclick={copyClicked ? undefined : copyInstall} class:copied={copyClicked} aria-label="Copy command">
						{#if copyClicked}
							<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
							Copied
						{:else}
							<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
							Copy
						{/if}
					</button>
				</div>
			</div>

			<!-- CTAs -->
			<div class="flex flex-wrap justify-center gap-4 animate-fade-in-up delay-4">
				<a href="/docs" class="btn-primary">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
					Get Started
				</a>
				<a href="https://github.com/nousresearch/cora-cli" target="_blank" rel="noopener" class="btn-ghost">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
					View on GitHub
				</a>
			</div>

			<!-- Sub text -->
			<p class="mt-6 animate-fade-in-up delay-5" style="font-size: 0.8125rem; color: var(--text-tertiary);">
				MIT License &middot; No account needed &middot; Works with OpenAI, Anthropic, Groq, Ollama
			</p>
		</div>
	</section>

	<div class="section-divider"></div>

	<!-- ====== WHY CORA — Stats Bar ====== -->
	<section style="padding: 5rem 1.5rem;">
		<div class="max-w-5xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-6">
			<div class="glass-card text-center scroll-reveal">
				<div class="flex justify-center gap-2 mb-3">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"/><path d="M8 21h8"/><path d="M12 17v4"/></svg>
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
				</div>
				<div style="font-size: 1.5rem; font-weight: 600; color: var(--text-primary);">5 AI Providers</div>
				<div style="font-size: 0.875rem; color: var(--text-secondary);">OpenAI, Anthropic, Groq, Ollama, Z.AI</div>
			</div>

			<div class="glass-card text-center scroll-reveal">
				<div class="mb-3">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--success)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
				</div>
				<div style="font-size: 1.5rem; font-weight: 600; color: var(--text-primary);">&lt; 3s</div>
				<div style="font-size: 0.875rem; color: var(--text-secondary);">Staged diff review</div>
			</div>

			<div class="glass-card text-center scroll-reveal">
				<div class="mb-3">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
				</div>
				<div style="font-size: 1.5rem; font-weight: 600; color: var(--text-primary);">Zero config</div>
				<div style="font-size: 0.875rem; color: var(--text-secondary);">cora init and go</div>
			</div>
		</div>
	</section>

	<div class="section-divider"></div>

	<!-- ====== HOW IT WORKS ====== -->
	<section style="padding: 5rem 1.5rem;">
		<div class="max-w-5xl mx-auto">
			<h2 class="text-center mb-4 scroll-reveal">How it works</h2>
			<p class="text-center mb-16 scroll-reveal" style="color: var(--text-secondary); max-width: 32rem; margin-left: auto; margin-right: auto;">
				Three simple steps from code to confidence.
			</p>

			<!-- Steps -->
			<div class="flex flex-col md:flex-row items-stretch gap-4 md:gap-0 mb-16">
				<!-- Step 1 -->
				<div class="glass-card flex-1 scroll-reveal" style="text-align: center;">
					<div class="step-num mb-4">01</div>
					<div class="mb-3">
						<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
					</div>
					<h3>Write code</h3>
					<p style="font-size: 0.875rem; margin-top: 0.5rem;">Push your changes as normal.</p>
				</div>

				<!-- Connector -->
				<div class="connect-line hidden md:flex scroll-reveal">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
				</div>

				<!-- Step 2 -->
				<div class="glass-card flex-1 scroll-reveal" style="text-align: center;">
					<div class="step-num mb-4">02</div>
					<div class="mb-3">
						<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"/></svg>
					</div>
					<h3>Review with AI</h3>
					<p style="font-size: 0.875rem; margin-top: 0.5rem;">cora analyzes your diff with LLM.</p>
				</div>

				<!-- Connector -->
				<div class="connect-line hidden md:flex scroll-reveal">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
				</div>

				<!-- Step 3 -->
				<div class="glass-card flex-1 scroll-reveal" style="text-align: center;">
					<div class="step-num mb-4">03</div>
					<div class="mb-3">
						<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="var(--success)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 11-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>
					</div>
					<h3>Ship with confidence</h3>
					<p style="font-size: 0.875rem; margin-top: 0.5rem;">Merge clean, production-ready code.</p>
				</div>
			</div>

			<!-- Live Terminal Demo -->
			<div class="max-w-2xl mx-auto scroll-reveal">
				<div class="terminal-window glow">
					<div class="terminal-bar">
						<span class="terminal-dot terminal-dot-red"></span>
						<span class="terminal-dot terminal-dot-yellow"></span>
						<span class="terminal-dot terminal-dot-green"></span>
						<span class="terminal-title">cora — review</span>
					</div>
					<div class="terminal-body" style="font-size: 0.8125rem;">
						{#each terminalLines as line, i}
							<div class="line-{i}" style="min-height: 1.45em;">{line}</div>
						{/each}
						{#if terminalComplete}
							<span class="typing-cursor"></span>
						{/if}
					</div>
				</div>
			</div>
		</div>
	</section>

	<div class="section-divider"></div>

	<!-- ====== FEATURES ====== -->
	<section style="padding: 5rem 1.5rem;">
		<div class="max-w-6xl mx-auto">
			<h2 class="text-center mb-4 scroll-reveal">Everything you need,</h2>
			<h2 class="text-center mb-4 scroll-reveal"><span class="gradient-text">nothing you don't</span></h2>
			<p class="text-center mb-16 scroll-reveal" style="color: var(--text-secondary); max-width: 32rem; margin-left: auto; margin-right: auto;">
				Built for developers who value simplicity and control.
			</p>

			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
				<!-- Feature 1: AI Code Review -->
				<div class="glass-card scroll-reveal">
					<div class="mb-4">
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/><path d="M11 8v6"/><path d="M8 11h6"/></svg>
					</div>
					<h3>AI Code Review</h3>
					<p style="font-size: 0.8125rem; color: var(--accent); margin: 0.25rem 0 0.5rem;">Diff, branch, or full scan</p>
					<p style="font-size: 0.875rem;">Three review modes: staged diff (&lt;3s), branch comparison, full project scan. LLM-powered analysis catches bugs, security issues, and style violations.</p>
				</div>

				<!-- Feature 2: BYOK -->
				<div class="glass-card scroll-reveal">
					<div class="mb-4">
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 11-7.778 7.778 5.5 5.5 0 017.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/></svg>
					</div>
					<h3>Bring Your Own Key</h3>
					<p style="font-size: 0.8125rem; color: var(--accent); margin: 0.25rem 0 0.5rem;">No subscriptions, no lock-in</p>
					<p style="font-size: 0.875rem;">Uses YOUR OpenAI, Anthropic, Groq, Ollama, or Z.AI API key. No data stored on our servers. You control the model, you control the cost.</p>
				</div>

				<!-- Feature 3: Pre-commit Hooks -->
				<div class="glass-card scroll-reveal">
					<div class="mb-4">
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22V8"/><path d="M5 12H2a10 10 0 0020 0h-3"/><circle cx="12" cy="5" r="3"/></svg>
					</div>
					<h3>Pre-commit Hooks</h3>
					<p style="font-size: 0.8125rem; color: var(--accent); margin: 0.25rem 0 0.5rem;">Review before you push</p>
					<p style="font-size: 0.875rem;">Install once with <code style="color: var(--accent); font-family: 'JetBrains Mono', monospace; font-size: 0.8125rem;">cora hook install</code>. Every commit gets reviewed automatically. Block bad code from entering your branch.</p>
				</div>

				<!-- Feature 4: Incremental Scan -->
				<div class="glass-card scroll-reveal">
					<div class="mb-4">
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 7 12 12 22 7 12 2"/><polyline points="2 17 12 22 22 17"/><polyline points="2 12 12 17 22 12"/></svg>
					</div>
					<h3>Incremental Scan</h3>
					<p style="font-size: 0.8125rem; color: var(--accent); margin: 0.25rem 0 0.5rem;">Only scan what changed</p>
					<p style="font-size: 0.875rem;">SHA256 content hash cache. First scan indexes your codebase. Subsequent scans only review new or modified files.</p>
				</div>

				<!-- Feature 5: SARIF Output -->
				<div class="glass-card scroll-reveal">
					<div class="mb-4">
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><path d="M9 12l2 2 4-4"/></svg>
					</div>
					<h3>SARIF Output</h3>
					<p style="font-size: 0.8125rem; color: var(--accent); margin: 0.25rem 0 0.5rem;">GitHub Code Scanning</p>
					<p style="font-size: 0.875rem;">Upload review findings directly to GitHub's Security tab. Track issues across PRs. Works with any CI/CD pipeline.</p>
				</div>

				<!-- Feature 6: Fully Private -->
				<div class="glass-card scroll-reveal">
					<div class="mb-4">
						<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0110 0v4"/><circle cx="12" cy="16" r="1"/></svg>
					</div>
					<h3>Fully Private</h3>
					<p style="font-size: 0.8125rem; color: var(--accent); margin: 0.25rem 0 0.5rem;">Your code stays yours</p>
					<p style="font-size: 0.875rem;">Runs entirely on your machine. No cloud, no telemetry, no data leaving your network. Perfect for Gitea and air-gapped environments.</p>
				</div>
			</div>
		</div>
	</section>

	<div class="section-divider"></div>

	<!-- ====== COMPARISON TABLE ====== -->
	<section style="padding: 5rem 1.5rem;">
		<div class="max-w-5xl mx-auto">
			<h2 class="text-center mb-4 scroll-reveal">Why developers choose cora</h2>
			<p class="text-center mb-16 scroll-reveal" style="color: var(--text-secondary); max-width: 32rem; margin-left: auto; margin-right: auto;">
				Side-by-side with popular code review tools.
			</p>

			<div class="glass-card scroll-reveal" style="padding: 0; overflow: hidden;">
				<div style="overflow-x: auto;">
					<table class="compare-table">
						<thead>
							<tr>
								<th>Feature</th>
								<th class="highlight-col">cora</th>
								<th>CodeRabbit</th>
								<th>Copilot Review</th>
								<th>SonarQube</th>
							</tr>
						</thead>
						<tbody>
							<tr>
								<td>BYOK</td>
								<td class="highlight-col"><span class="symbol-check">&#10003;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
								<td style="color: var(--text-tertiary);">N/A</td>
							</tr>
							<tr>
								<td>Self-hosted</td>
								<td class="highlight-col"><span class="symbol-check">&#10003;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
								<td><span class="symbol-check">&#10003;</span></td>
							</tr>
							<tr>
								<td>Gitea</td>
								<td class="highlight-col"><span class="symbol-check">&#10003;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
								<td><span class="symbol-check">&#10003;</span></td>
							</tr>
							<tr>
								<td>CLI</td>
								<td class="highlight-col"><span class="symbol-check">&#10003;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
							</tr>
							<tr>
								<td>Pre-commit</td>
								<td class="highlight-col"><span class="symbol-check">&#10003;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
								<td><span class="symbol-cross">&#10007;</span></td>
							</tr>
							<tr>
								<td>SARIF</td>
								<td class="highlight-col"><span class="symbol-check">&#10003;</span></td>
								<td><span class="symbol-check">&#10003;</span></td>
								<td><span class="symbol-check">&#10003;</span></td>
								<td><span class="symbol-check">&#10003;</span></td>
							</tr>
							<tr>
								<td>Cost</td>
								<td class="highlight-col">Free + API</td>
								<td>$12-39/mo</td>
								<td>$10-39/mo</td>
								<td>Free / $150+</td>
							</tr>
							<tr>
								<td>License</td>
								<td class="highlight-col">MIT</td>
								<td>Apache 2.0</td>
								<td>Proprietary</td>
								<td>LGPL</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</section>

	<div class="section-divider"></div>

	<!-- ====== QUICK START ====== -->
	<section style="padding: 5rem 1.5rem;">
		<div class="max-w-2xl mx-auto">
			<h2 class="text-center mb-4 scroll-reveal">Start in 30 seconds</h2>
			<p class="text-center mb-16 scroll-reveal" style="color: var(--text-secondary);">No account required. No subscription. No cloud.</p>

			<div style="display: flex; flex-direction: column; gap: 2rem;">
				<!-- Step 1 -->
				<div class="timeline-step scroll-reveal">
					<div class="timeline-number">1</div>
					<h3>Install</h3>
					<p style="font-size: 0.875rem; margin-top: 0.25rem; margin-bottom: 0.75rem;">Single binary, no dependencies.</p>
					<div class="terminal-window">
						<div class="terminal-bar">
							<span class="terminal-dot terminal-dot-red"></span>
							<span class="terminal-dot terminal-dot-yellow"></span>
							<span class="terminal-dot terminal-dot-green"></span>
						</div>
						<div class="terminal-body">
							<span class="syntax-cmd">$</span> <span class="syntax-highlight">cargo install</span> <span class="syntax-string">cora</span>
						</div>
					</div>
				</div>

				<!-- Step 2 -->
				<div class="timeline-step scroll-reveal">
					<div class="timeline-number">2</div>
					<h3>Initialize</h3>
					<p style="font-size: 0.875rem; margin-top: 0.25rem; margin-bottom: 0.75rem;">Creates <code style="color: var(--accent); font-family: 'JetBrains Mono', monospace; font-size: 0.8125rem;">.cora.yaml</code> config.</p>
					<div class="terminal-window">
						<div class="terminal-bar">
							<span class="terminal-dot terminal-dot-red"></span>
							<span class="terminal-dot terminal-dot-yellow"></span>
							<span class="terminal-dot terminal-dot-green"></span>
						</div>
						<div class="terminal-body">
							<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora init</span>
						</div>
					</div>
				</div>

				<!-- Step 3 -->
				<div class="timeline-step scroll-reveal">
					<div class="timeline-number">3</div>
					<h3>Review</h3>
					<p style="font-size: 0.875rem; margin-top: 0.25rem; margin-bottom: 0.75rem;">Review your staged changes.</p>
					<div class="terminal-window">
						<div class="terminal-bar">
							<span class="terminal-dot terminal-dot-red"></span>
							<span class="terminal-dot terminal-dot-yellow"></span>
							<span class="terminal-dot terminal-dot-green"></span>
						</div>
						<div class="terminal-body">
							<span class="syntax-cmd">$</span> <span class="syntax-flag">CORA_API_KEY</span>=<span class="syntax-string">key</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span>
						</div>
					</div>
				</div>

				<!-- Step 4 -->
				<div class="timeline-step scroll-reveal">
					<div class="timeline-number">4</div>
					<h3>Done</h3>
					<p style="font-size: 0.875rem; margin-top: 0.25rem; color: var(--success);">
						That's it. No account. No subscription.
					</p>
				</div>
			</div>
		</div>
	</section>

	<div class="section-divider"></div>

	<!-- ====== CTA / FOOTER ====== -->
	<section style="padding: 6rem 1.5rem 4rem;">
		<div class="max-w-3xl mx-auto text-center scroll-reveal">
			<h2 class="mb-6" style="font-size: 2.5rem;">Ready to ship <span class="gradient-text">better code</span>?</h2>
			<p class="mb-10" style="font-size: 1.125rem; color: var(--text-secondary);">Open source. MIT license. Start in 30 seconds.</p>

			<div class="flex flex-wrap justify-center gap-4 mb-16">
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
	</section>

	<!-- Footer -->
	<footer style="border-top: 1px solid var(--border); padding: 2rem 1.5rem;">
		<div class="max-w-6xl mx-auto flex flex-col md:flex-row items-center justify-between gap-4" style="font-size: 0.8125rem; color: var(--text-tertiary);">
			<span>cora &middot; MIT License &middot; Built with Rust</span>
			<div class="flex items-center gap-6">
				<a href="https://github.com/nousresearch/cora-cli" target="_blank" rel="noopener" style="color: var(--text-tertiary); transition: color 0.2s;">GitHub</a>
				<a href="/docs" style="color: var(--text-tertiary); transition: color 0.2s;">Docs</a>
			</div>
		</div>
	</footer>
</div>
