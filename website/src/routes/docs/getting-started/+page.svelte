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

<div class="docs-content">
<h1 class="scroll-reveal">
	Getting Started
</h1>

<div class="docs-section scroll-reveal">
	<h2>Quick Start</h2>
	<p>Get up and running with cora in four simple steps.</p>

	<div class="docs-card">
		<div class="docs-card-number primary">1</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<strong class="text-[var(--foreground)]">Install cora</strong> — Single binary via Cargo:
			<code>cargo install cora-cli</code>
		</div>
	</div>

	<div class="docs-card">
		<div class="docs-card-number primary">2</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<strong class="text-[var(--foreground)]">Configure</strong> — Initialize your project:
			<code>cora init</code> creates <code>.cora.yaml</code>
		</div>
	</div>

	<div class="docs-card">
		<div class="docs-card-number primary">3</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<strong class="text-[var(--foreground)]">Add API key</strong> — Run <code>cora auth login</code> or set <code>CORA_API_KEY</code> environment variable
		</div>
	</div>

	<div class="docs-card">
		<div class="docs-card-number primary">4</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<strong class="text-[var(--foreground)]">Review</strong> — Analyze your staged changes:
			<code>cora review</code>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal">
	<h2>Understanding the Output</h2>
	<p>cora outputs a structured, color-coded summary of findings for each file reviewed.</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot-red"></span>
			<span class="terminal-dot-yellow"></span>
			<span class="terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<div><span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span></div>
			<div></div>
			<div><span class="syntax-cmd">Analyzing 3 files...</span></div>
			<div><span class="syntax-success">&#10003;</span> src/auth/login.ts <span class="syntax-cmd">— 2 issues found</span></div>
			<div>  <span class="syntax-warning">&#9888;</span> <span class="syntax-cmd">Line 42:</span> Potential SQL injection</div>
			<div>  <span class="syntax-warning">&#9888;</span> <span class="syntax-cmd">Line 87:</span> Hardcoded secret</div>
			<div><span class="syntax-success">&#10003;</span> src/utils/parser.ts <span class="syntax-cmd">— clean</span></div>
			<div><span class="syntax-success">&#10003;</span> src/api/routes.ts <span class="syntax-cmd">— 1 issue found</span></div>
			<div>  <span class="syntax-error">&#10007;</span> <span class="syntax-cmd">Line 23:</span> Missing error handling</div>
			<div></div>
			<div>3 issues found in 3 files</div>
		</div>
	</div>

	<p>Each line in the output contains:</p>
	<div class="docs-term-list">
		<div class="docs-term-item">
			<span class="docs-term-key">file path</span>
			<span>The relative path to the file being reviewed</span>
		</div>
		<div class="docs-term-item">
			<span class="docs-term-key">line number</span>
			<span>Specific line where the issue was found</span>
		</div>
		<div class="docs-term-item">
			<span class="docs-term-key">severity</span>
			<span>Suggestion (&#9888;), Warning (&#9888;), or Error (&#10007;)</span>
		</div>
		<div class="docs-term-item">
			<span class="docs-term-key">description</span>
			<span>Brief explanation of the issue</span>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal">
	<h2>Customizing Your Review</h2>
	<p>The <code>.cora.yaml</code> file controls how cora reviews your code.</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot-red"></span>
			<span class="terminal-dot-yellow"></span>
			<span class="terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<div><span class="syntax-comment"># .cora.yaml</span></div>
			<div><span class="syntax-highlight">review:</span></div>
			<div>  <span class="syntax-flag">severity:</span> <span class="syntax-string">warning</span></div>
			<div>  <span class="syntax-flag">focus:</span> <span class="syntax-string">security,performance</span></div>
			<div></div>
			<div><span class="syntax-highlight">providers:</span></div>
			<div>  <span class="syntax-flag">openai:</span></div>
			<div>    <span class="syntax-flag">model:</span> <span class="syntax-string">gpt-4o</span></div>
		</div>
	</div>

	<div class="docs-term-list">
		<div class="text-sm text-[var(--muted-foreground)]">
			<span class="docs-term-key">review.severity</span> — Minimum severity level (info, minor, major, critical)
		</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<span class="docs-term-key">review.focus</span> — Focus areas for review (e.g., security, performance)
		</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<span class="docs-term-key">providers</span> — Provider-specific model overrides (openai, anthropic, groq, ollama, zai)
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal">
	<h2>Next Steps</h2>
	<p>Now that you have cora running, explore these topics to get the most out of it:</p>
	<div class="docs-link-list">
		<a href="/docs/installation">Installation — install options and shell completions</a>
		<a href="/docs/usage">Usage — review modes, output formats, and configuration</a>
		<a href="/docs/configuration">Configuration — full .cora.yaml reference</a>
		<a href="/docs/providers">Providers — setting up OpenAI, Anthropic, Groq, Ollama, and Z.AI</a>
		<a href="/docs/cli-reference">CLI Reference — full command documentation</a>
	</div>
</div>
</div>
