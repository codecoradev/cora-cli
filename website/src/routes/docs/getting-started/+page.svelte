<svelte:head>
	<title>Getting Started — cora docs</title>
</svelte:head>

<script lang="ts">
	import { onMount } from 'svelte';

	onMount(() => {
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
	});
</script>

<h1 style="font-size: 28px; font-weight: 700; color: var(--foreground); letter-spacing: -0.02em; line-height: 1.2; margin-bottom: 1.5rem;">
	Getting Started
</h1>

<div class="docs-section scroll-reveal">
	<h2>Quick Start</h2>
	<p>Get up and running with cora in four simple steps.</p>

	<div class="docs-card">
		<div class="docs-card-number primary">1</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<strong style="color: var(--foreground);">Install cora</strong> — Single binary via Cargo:
			<code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">cargo install cora</code>
		</div>
	</div>

	<div class="docs-card">
		<div class="docs-card-number primary">2</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<strong style="color: var(--foreground);">Configure</strong> — Initialize your project:
			<code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">cora init</code> creates <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">.cora.yaml</code>
		</div>
	</div>

	<div class="docs-card">
		<div class="docs-card-number primary">3</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<strong style="color: var(--foreground);">Add API key</strong> — Run <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">cora auth login</code> or set <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">CORA_API_KEY</code> environment variable
		</div>
	</div>

	<div class="docs-card">
		<div class="docs-card-number primary">4</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<strong style="color: var(--foreground);">Review</strong> — Analyze your staged changes:
			<code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">cora review --staged</code>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Understanding the Output</h2>
	<p>cora outputs a structured, color-coded summary of findings for each file reviewed.</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora review</span> <span style="color: oklch(0.75 0.15 240);">--staged</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);">Analyzing 3 files...</span></div>
			<div><span style="color: var(--success);">&#10003;</span> src/auth/login.ts <span style="color: var(--muted-foreground);">— 2 issues found</span></div>
			<div>  <span style="color: var(--warning);">&#9888;</span> <span style="color: var(--muted-foreground);">Line 42:</span> Potential SQL injection</div>
			<div>  <span style="color: var(--warning);">&#9888;</span> <span style="color: var(--muted-foreground);">Line 87:</span> Hardcoded secret</div>
			<div><span style="color: var(--success);">&#10003;</span> src/utils/parser.ts <span style="color: var(--muted-foreground);">— clean</span></div>
			<div><span style="color: var(--success);">&#10003;</span> src/api/routes.ts <span style="color: var(--muted-foreground);">— 1 issue found</span></div>
			<div>  <span style="color: var(--destructive);">&#10007;</span> <span style="color: var(--muted-foreground);">Line 23:</span> Missing error handling</div>
			<div></div>
			<div>3 issues found in 3 files</div>
		</div>
	</div>

	<p>Each line in the output contains:</p>
	<div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent); font-family: var(--font-mono);">file path</span>
			<span>The relative path to the file being reviewed</span>
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent); font-family: var(--font-mono);">line number</span>
			<span>Specific line where the issue was found</span>
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent); font-family: var(--font-mono);">severity</span>
			<span>Suggestion (&#9888;), Warning (&#9888;), or Error (&#10007;)</span>
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent); font-family: var(--font-mono);">description</span>
			<span>Brief explanation of the issue</span>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Customizing Your Review</h2>
	<p>The <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">.cora.yaml</code> file controls how cora reviews your code.</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);"># .cora.yaml</span></div>
			<div><span style="color: var(--accent);">provider</span>: <span style="color: var(--success);">openai</span></div>
			<div><span style="color: var(--accent);">model</span>: <span style="color: var(--success);">gpt-4o</span></div>
			<div></div>
			<div><span style="color: var(--accent);">custom_prompt</span>: <span style="color: var(--success);">|</span></div>
			<div><span style="color: var(--success);">  Focus on security vulnerabilities and</span></div>
			<div><span style="color: var(--success);">  performance issues. Ignore style linting.</span></div>
		</div>
	</div>

	<div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<span style="color: var(--accent); font-family: var(--font-mono);">provider</span> — Which LLM provider to use (openai, anthropic, groq, ollama, zai)
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<span style="color: var(--accent); font-family: var(--font-mono);">model</span> — Specific model name (e.g., gpt-4o, claude-sonnet-4-20250514)
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<span style="color: var(--accent); font-family: var(--font-mono);">custom_prompt</span> — Override the default review prompt to focus on specific concerns
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Next Steps</h2>
	<p>Now that you have cora running, explore these topics to get the most out of it:</p>
	<div style="display: flex; flex-direction: column; gap: 0.5rem;">
		<a href="/docs/installation" style="font-size: 14px; color: var(--accent); text-decoration: none;">Installation — install options and shell completions</a>
		<a href="/docs/usage" style="font-size: 14px; color: var(--accent); text-decoration: none;">Usage — review modes, output formats, and configuration</a>
		<a href="/docs/configuration" style="font-size: 14px; color: var(--accent); text-decoration: none;">Configuration — full .cora.yaml reference</a>
		<a href="/docs/providers" style="font-size: 14px; color: var(--accent); text-decoration: none;">Providers — setting up OpenAI, Anthropic, Groq, Ollama, and Z.AI</a>
		<a href="/docs/cli-reference" style="font-size: 14px; color: var(--accent); text-decoration: none;">CLI Reference — full command documentation</a>
	</div>
</div>
