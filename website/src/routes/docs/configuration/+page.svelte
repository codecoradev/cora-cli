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
		return () => observer.disconnect();
	});
</script>

<svelte:head>
	<title>Configuration — cora docs</title>
	<meta name="description" content="Configure cora - config resolution, .cora.yaml, environment variables, and CLI flags." />
</svelte:head>

<div class="docs-content">
<h1 class="scroll-reveal">Configuration</h1>
<p class="scroll-reveal">cora uses a layered config system. Later sources override earlier ones.</p>

<!-- Config Resolution Order -->
<section class="docs-section scroll-reveal">
	<h2 class="flex items-center gap-2">
		<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg>
		Config Resolution Order
	</h2>
	<p class="text-[var(--muted-foreground)] mb-6">Settings are resolved in this order (highest priority first):</p>
	<div class="flex flex-col gap-2">
		{#each [
			{ num: '1', label: 'CLI flags', desc: '--provider, --model, --base-url, etc.', accent: true },
			{ num: '2', label: '.cora.yaml', desc: 'Project root config file', accent: false },
			{ num: '3', label: '~/.cora/config.toml', desc: 'User-level config', accent: false },
			{ num: '4', label: 'Environment variables', desc: 'CORA_API_KEY, CORA_PROVIDER, etc.', accent: false },
			{ num: '5', label: 'Built-in defaults', desc: 'Sensible defaults for all settings', accent: false }
		] as item}
			<div class="docs-card" class:accent={item.accent}>
				<div class="docs-card-number" class:primary={item.accent} class:muted={!item.accent}>{item.num}</div>
				<div>
					<div class="text-sm font-semibold text-[var(--foreground)]">{item.label}</div>
					<div class="text-xs text-[var(--muted-foreground)] tracking-wide">{item.desc}</div>
				</div>
			</div>
		{/each}
	</div>
</section>

<!-- .cora.yaml Example -->
<section class="docs-section scroll-reveal">
	<h2 class="flex items-center gap-2">
		<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/><polyline points="13 2 13 9 20 9"/></svg>
		.cora.yaml Example
	</h2>
	<p class="text-[var(--muted-foreground)] mb-4">Create this file in your project root. Run <code class="syntax-highlight">cora init</code> to generate it.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
			<span class="terminal-title">.cora.yaml</span>
		</div>
		<div class="terminal-body">
<pre class="whitespace-pre"><span class="syntax-comment"># cora project config</span>
<span class="syntax-highlight">review:</span>
  <span class="syntax-flag">severity:</span> <span class="syntax-string">warning</span>          <span class="syntax-comment"># minimum severity: info, minor, major, critical</span>
  <span class="syntax-flag">max_issues:</span> <span class="text-[var(--foreground)]">20</span>             <span class="syntax-comment"># max issues to report</span>
  <span class="syntax-flag">focus:</span> <span class="syntax-string">security,performance</span>  <span class="syntax-comment"># focus areas</span>

<span class="syntax-highlight">ignore:</span>
  <span class="syntax-cmd">- "vendor/**"</span>
  <span class="syntax-cmd">- "*.min.js"</span>
  <span class="syntax-cmd">- "migrations/**"</span>

<span class="syntax-highlight">providers:</span>
  <span class="syntax-flag">openai:</span>
    <span class="syntax-flag">model:</span> <span class="syntax-string">gpt-4o</span>
  <span class="syntax-flag">anthropic:</span>
    <span class="syntax-flag">model:</span> <span class="syntax-string">claude-sonnet-4-20250514</span></pre>
		</div>
	</div>
</section>

<!-- Environment Variables -->
<section class="docs-section scroll-reveal">
	<h2 class="flex items-center gap-2">
		<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 11-7.778 7.778 5.5 5.5 0 017.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"/></svg>
		Environment Variables
	</h2>
	<div class="glass-card p-0 overflow-hidden">
		<table class="compare-table">
			<thead>
				<tr>
					<th class="w-1/3">Variable</th>
					<th>Description</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td><code class="syntax-highlight">CORA_API_KEY</code></td>
					<td>API key for the active provider</td>
				</tr>
				<tr>
					<td><code class="syntax-highlight">CORA_PROVIDER</code></td>
					<td>Active provider (openai, anthropic, groq, ollama, zai)</td>
				</tr>
				<tr>
					<td><code class="syntax-highlight">CORA_MODEL</code></td>
					<td>Model name override</td>
				</tr>
				<tr>
					<td><code class="syntax-highlight">CORA_BASE_URL</code></td>
					<td>Custom API base URL</td>
				</tr>
				<tr>
					<td><code class="syntax-highlight">CORA_CONFIG</code></td>
					<td>Path to config file</td>
				</tr>
				<tr>
					<td><code class="syntax-highlight">CORA_FORMAT</code></td>
					<td>Output format (pretty, json, compact, sarif)</td>
				</tr>
				<tr>
					<td><code class="syntax-highlight">CORA_NO_COLOR</code></td>
					<td>Disable colored output</td>
				</tr>
				<tr>
					<td><code class="syntax-highlight">GITHUB_TOKEN</code></td>
					<td>GitHub token for SARIF upload</td>
				</tr>
				<tr>
					<td><code class="syntax-highlight">GITHUB_REPOSITORY</code></td>
					<td>GitHub repo for SARIF upload</td>
				</tr>
				<tr>
					<td><code class="syntax-highlight">GITHUB_REF</code></td>
					<td>GitHub ref for SARIF upload</td>
				</tr>
			</tbody>
		</table>
	</div>
</section>

<!-- Provider env vars -->
<section class="docs-section scroll-reveal">
	<h2 class="flex items-center gap-2">
		<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0110 0v4"/></svg>
		Provider-Specific Env Vars
	</h2>
	<p class="text-[var(--muted-foreground)] mb-4">Each provider has its own API key variable. cora checks these for auto-detection.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
			<span class="terminal-title">env vars</span>
		</div>
		<div class="terminal-body">
<pre class="whitespace-pre"><span class="syntax-comment"># OpenAI</span>
<span class="syntax-flag">OPENAI_API_KEY</span>=<span class="syntax-string">sk-...</span>
<span class="syntax-flag">OPENAI_BASE_URL</span>=<span class="syntax-string">https://api.openai.com/v1</span>

<span class="syntax-comment"># Anthropic</span>
<span class="syntax-flag">ANTHROPIC_API_KEY</span>=<span class="syntax-string">sk-ant-...</span>

<span class="syntax-comment"># Groq</span>
<span class="syntax-flag">GROQ_API_KEY</span>=<span class="syntax-string">gsk_...</span>

<span class="syntax-comment"># Ollama (local, no key needed)</span>
<span class="syntax-flag">OLLAMA_HOST</span>=<span class="syntax-string">http://localhost:11434</span>
<span class="syntax-comment"># Optional: OLLAMA_API_KEY if your Ollama instance requires auth</span>
<span class="syntax-flag">OLLAMA_API_KEY</span>=<span class="syntax-string">...</span>

<span class="syntax-comment"># Z.AI</span>
<span class="syntax-flag">ZAI_API_KEY</span>=<span class="syntax-string">...</span></pre>
		</div>
	</div>
</section>
</div>
