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
	<p>Get up and running with cora in three simple steps.</p>

	<div class="docs-card">
		<div class="docs-card-number primary">1</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<strong class="text-[var(--foreground)]">Install cora</strong> — Single binary, no runtime dependencies:
			<code>curl -fsSL https://raw.githubusercontent.com/codecoradev/cora-cli/main/install.sh | sh</code>
		</div>
	</div>

	<div class="docs-card">
		<div class="docs-card-number primary">2</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<strong class="text-[var(--foreground)]">Authenticate</strong> — Run <code>cora auth login</code> to pick your provider and enter your API key.
			Cora stores it securely in <code>~/.cora/auth.toml</code> (never committed to git).
		</div>
	</div>

	<div class="docs-card">
		<div class="docs-card-number primary">3</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<strong class="text-[var(--foreground)]">Review</strong> — Analyze your staged changes:
			<code>cora review</code>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal">
	<h2>Authentication — <code>cora auth login</code></h2>
	<p>
		The interactive login guides you through provider selection:
	</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot-red"></span>
			<span class="terminal-dot-yellow"></span>
			<span class="terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<div><span class="syntax-cmd">$</span> <span class="syntax-highlight">cora auth login</span></div>
			<div></div>
			<div><span class="syntax-cmd">🔑 Cora Auth Setup</span></div>
			<div>   Choose your LLM provider:</div>
			<div></div>
			<div>  <span class="syntax-flag">[1]</span> <span class="syntax-highlight">openai</span> — https://api.openai.com/v1</div>
			<div>  <span class="syntax-flag">[2]</span> <span class="syntax-highlight">anthropic</span> — https://api.anthropic.com/v1</div>
			<div>  <span class="syntax-flag">[3]</span> <span class="syntax-highlight">groq</span> — https://api.groq.com/openai/v1</div>
			<div>  <span class="syntax-flag">[4]</span> <span class="syntax-highlight">ollama</span> — http://localhost:11434/v1</div>
			<div>  <span class="syntax-flag">[5]</span> <span class="syntax-highlight">zai</span> — https://api.z.ai/api/coding/paas/v4</div>
			<div>  <span class="syntax-flag">[6]</span> <span class="syntax-highlight">custom</span> — any OpenAI-compatible endpoint</div>
			<div></div>
			<div>  Select provider [1-6]: <span class="syntax-string">1</span></div>
			<div></div>
			<div>  → Provider: <span class="syntax-success">openai</span></div>
			<div>  → Model: <span class="syntax-success">gpt-4o-mini</span></div>
			<div>  → Base URL: https://api.openai.com/v1</div>
			<div></div>
			<div>  🔑 Enter your API key: <span class="syntax-string">****</span></div>
			<div></div>
			<div><span class="syntax-success">✅</span> API key saved to ~/.cora/auth.toml</div>
		</div>
	</div>

	<div class="docs-term-list">
		<div class="docs-term-item">
			<span class="docs-term-key">Known providers</span>
			<span>Just enter your API key — model and base URL are pre-configured</span>
		</div>
		<div class="docs-term-item">
			<span class="docs-term-key">Custom provider</span>
			<span>Enter your own base URL, model name, and API key for any OpenAI-compatible API</span>
		</div>
		<div class="docs-term-item">
			<span class="docs-term-key">Provider info stored</span>
			<span>Provider name, model, and base URL are saved alongside your API key for easy reference</span>
		</div>
	</div>

	<p>Check your auth status anytime:</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot-red"></span>
			<span class="terminal-dot-yellow"></span>
			<span class="terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<div><span class="syntax-cmd">$</span> <span class="syntax-highlight">cora auth status</span></div>
			<div><span class="syntax-success">✅</span> API key is configured.</div>
			<div>   Source: ~/.cora/auth.toml</div>
			<div></div>
			<div>   <span class="syntax-flag">Provider:</span> <span class="syntax-success">openai</span></div>
			<div>   <span class="syntax-flag">Model:</span> <span class="syntax-success">gpt-4o-mini</span></div>
			<div>   <span class="syntax-flag">Base URL:</span> https://api.openai.com/v1</div>
		</div>
	</div>

	<p>
		You can also use environment variables instead of <code>cora auth login</code>:
		<code>CORA_API_KEY</code>, <code>OPENAI_API_KEY</code>, <code>ANTHROPIC_API_KEY</code>, etc.
	</p>
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
	<h2>Project Configuration — <code>cora init</code></h2>
	<p>
		Create a <code>.cora.yaml</code> config file in your project root.
		<strong>Automatically installs a pre-commit hook</strong> that runs <code>cora review --staged --format compact</code> before each commit.
		Use <code>--no-hook</code> to skip hook installation.
	</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot-red"></span>
			<span class="terminal-dot-yellow"></span>
			<span class="terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<div><span class="syntax-cmd">$</span> <span class="syntax-highlight">cora init</span></div>
			<div><span class="syntax-success">✅</span> Created .cora.yaml with example configuration.</div>
			<div><span class="syntax-success">✅</span> Pre-commit hook installed at .git/hooks/pre-commit</div>
		</div>
	</div>

	<p>Set provider, model, and base URL directly in <code>.cora.yaml</code> (no nested section needed):</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot-red"></span>
			<span class="terminal-dot-yellow"></span>
			<span class="terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<div><span class="syntax-comment"># .cora.yaml — shortcut format</span></div>
			<div><span class="syntax-flag">provider:</span> openai</div>
			<div><span class="syntax-flag">model:</span> gpt-4o-mini</div>
			<div><span class="syntax-flag">base_url:</span> https://api.openai.com/v1</div>
		</div>
	</div>

	<p>Key configuration options:</p>
	<div class="docs-term-list">
		<div class="text-sm text-[var(--muted-foreground)]">
			<span class="docs-term-key">focus</span> — Review focus areas: security, performance, bugs, best_practice
		</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<span class="docs-term-key">ignore</span> — File patterns and rules to skip
		</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<span class="docs-term-key">hook</span> — Pre-commit hook settings: mode, severity threshold, max diff size
		</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<span class="docs-term-key">llm</span> — LLM parameters: temperature, max_tokens, timeout
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
