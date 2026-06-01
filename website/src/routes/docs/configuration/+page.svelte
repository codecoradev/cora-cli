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
			{ num: '2', label: 'Environment variables', desc: 'CORA_API_KEY, CORA_PROVIDER, CORA_MODEL, etc.', accent: false },
			{ num: '3', label: '.cora.yaml', desc: 'Project root config file', accent: false },
			{ num: '4', label: '~/.cora/config.yaml', desc: 'Global config (optional)', accent: false },
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
<span class="syntax-highlight">provider:</span>
  <span class="syntax-flag">provider:</span> <span class="syntax-string">openai</span>
  <span class="syntax-flag">model:</span> <span class="syntax-string">gpt-4o</span>
  <span class="syntax-flag">base_url:</span> <span class="syntax-string">https://api.openai.com/v1</span>

<span class="syntax-highlight">llm:</span>
  <span class="syntax-flag">temperature:</span> <span class="text-[var(--foreground)]">0</span>
  <span class="syntax-flag">max_tokens:</span> <span class="text-[var(--foreground)]">4096</span>
  <span class="syntax-flag">timeout:</span> <span class="text-[var(--foreground)]">120</span>
  <span class="syntax-flag">cache_ttl:</span> <span class="text-[var(--foreground)]">1440</span>

<span class="syntax-highlight">review:</span>
  <span class="syntax-flag">system_prompt:</span> <span class="syntax-string">"You are a senior code reviewer."</span>
  <span class="syntax-comment"># system_prompt_file: ./review-prompt.md</span>
  <span class="syntax-flag">response_format:</span> <span class="syntax-string">json_object</span>

<span class="syntax-highlight">focus:</span> <span class="syntax-string">security, performance, bugs</span>

<span class="syntax-highlight">hook:</span>
  <span class="syntax-flag">mode:</span> <span class="syntax-string">warn</span>
  <span class="syntax-flag">min_severity:</span> <span class="syntax-string">major</span>
  <span class="syntax-flag">max_diff_size:</span> <span class="text-[var(--foreground)]">51200</span>

<span class="syntax-highlight">ignore:</span>
  <span class="syntax-flag">files:</span>
    <span class="syntax-cmd">- "vendor/**"</span>
    <span class="syntax-cmd">- "*.min.js"</span></pre>
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
				<td><code class="syntax-highlight">CORA_NO_CACHE</code></td>
				<td>Skip diff-hash review cache (same as <code class="syntax-highlight">--no-cache</code>)</td>
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

<!-- Caching Behavior -->
<section class="docs-section scroll-reveal">
	<h2 class="flex items-center gap-2">
		<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
		Diff-Hash Caching
	</h2>
	<p class="text-[var(--muted-foreground)] mb-4">cora caches review results by diff hash in <code class="syntax-highlight">~/.cache/cora/reviews/</code>. If you re-review the same diff, the cached result is returned instantly.</p>
	<div class="glass-card p-4 mb-4">
		<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
			<div>
				<div class="text-xs font-semibold text-[var(--foreground)] mb-1">Config</div>
				<div class="text-sm text-[var(--muted-foreground)]">
					<code class="syntax-highlight">llm.cache_ttl</code> — TTL in minutes (default: 1440 / 24h)
				</div>
			</div>
			<div>
				<div class="text-xs font-semibold text-[var(--foreground)] mb-1">CLI / Env</div>
				<div class="text-sm text-[var(--muted-foreground)]">
					<code class="syntax-highlight">--no-cache</code> or <code class="syntax-highlight">CORA_NO_CACHE=1</code> to bypass
				</div>
			</div>
		</div>
	</div>
</section>

<!-- Custom System Prompts -->
<section class="docs-section scroll-reveal">
	<h2 class="flex items-center gap-2">
		<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20h9"/><path d="M16.5 3.5a2.121 2.121 0 013 3L7 19l-4 1 1-4L16.5 3.5z"/></svg>
		Custom System Prompts
	</h2>
	<p class="text-[var(--muted-foreground)] mb-4">Override the default system prompt for <code class="syntax-highlight">review</code> or <code class="syntax-highlight">scan</code> commands to match your project's coding standards and review criteria.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
			<span class="terminal-title">.cora.yaml</span>
		</div>
		<div class="terminal-body">
<pre class="whitespace-pre"><span class="syntax-highlight">review:</span>
  <span class="syntax-flag">system_prompt:</span> <span class="syntax-string">"Focus on Rust idioms and error handling."</span>
  <span class="syntax-comment"># Or load from a file:</span>
  <span class="syntax-flag">system_prompt_file:</span> <span class="syntax-string">./prompts/review.md</span>

<span class="syntax-highlight">scan:</span>
  <span class="syntax-flag">system_prompt:</span> <span class="syntax-string">"Check for OWASP Top 10 vulnerabilities."</span>
  <span class="syntax-flag">system_prompt_file:</span> <span class="syntax-string">./prompts/scan.md</span></pre>
		</div>
	</div>
	<p class="text-[var(--muted-foreground)] mt-3 text-sm">If both <code class="syntax-highlight">system_prompt</code> and <code class="syntax-highlight">system_prompt_file</code> are set, the file takes precedence.</p>
</section>

<!-- Response Format -->
<section class="docs-section scroll-reveal">
	<h2 class="flex items-center gap-2">
		<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
		Response Format (JSON Mode)
	</h2>
	<p class="text-[var(--muted-foreground)] mb-4">Opt into structured JSON output from the LLM by setting <code class="syntax-highlight">review.response_format</code> to <code class="syntax-highlight">json_object</code>. This instructs the LLM to return valid JSON, enabling machine-readable parsing and pipeline integration.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
			<span class="terminal-title">.cora.yaml</span>
		</div>
		<div class="terminal-body">
<pre class="whitespace-pre"><span class="syntax-highlight">review:</span>
  <span class="syntax-flag">response_format:</span> <span class="syntax-string">json_object</span></pre>
		</div>
	</div>
	<p class="text-[var(--muted-foreground)] mt-3 text-sm">Requires provider support for structured output. Works with OpenAI, Anthropic, and compatible APIs.</p>
</section>

<!-- Anti-Hallucination -->
<section class="docs-section scroll-reveal">
	<h2 class="flex items-center gap-2">
		<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>
		Anti-Hallucination
	</h2>
	<p class="text-[var(--muted-foreground)] mb-4">cora uses two mechanisms to prevent the LLM from fabricating findings:</p>
	<div class="glass-card p-4">
		<ul class="text-sm text-[var(--muted-foreground)] space-y-2 list-none pl-0">
			<li><span class="text-[var(--accent)] font-semibold">File path injection</span> — Actual file paths are embedded in the prompt, anchoring the LLM to real files in the diff.</li>
			<li><span class="text-[var(--accent)] font-semibold">Post-parse filtering</span> — After parsing, any reported file paths or line numbers that don't exist in the actual diff are discarded.</li>
		</ul>
	</div>
</section>
</div>
