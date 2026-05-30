<svelte:head>
	<title>Configuration — cora docs</title>
	<meta name="description" content="Configure cora - config resolution, .cora.yaml, environment variables, and CLI flags." />
</svelte:head>

<h1 class="text-3xl md:text-4xl font-bold mb-2">Configuration</h1>
<p class="text-[var(--color-text-muted)] mb-10">cora uses a layered config system. Later sources override earlier ones.</p>

<!-- Config Resolution Order -->
<section class="mb-12">
	<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">📋</span> Config Resolution Order
	</h2>
	<p class="text-[var(--color-text-muted)] mb-6">Settings are resolved in this order (highest priority first):</p>

	<div class="space-y-3">
		{#each [
			{ num: '1', label: 'CLI flags', desc: '--provider, --model, --base-url, etc.', accent: true },
			{ num: '2', label: '.cora.yaml', desc: 'Project root config file', accent: false },
			{ num: '3', label: '~/.cora/config.toml', desc: 'User-level config', accent: false },
			{ num: '4', label: 'Environment variables', desc: 'CORA_API_KEY, CORA_PROVIDER, etc.', accent: false },
			{ num: '5', label: 'Built-in defaults', desc: 'Sensible defaults for all settings', accent: false }
		] as item}
			<div class="flex items-center gap-4 px-4 py-3 rounded-lg {item.accent ? 'bg-[var(--color-accent-dim)] border border-[var(--color-accent)]/30' : 'bg-[var(--color-surface)] border border-[var(--color-border)]'}">
				<span class="flex-shrink-0 w-8 h-8 rounded-full {item.accent ? 'bg-[var(--color-accent)] text-[var(--color-bg)]' : 'bg-[var(--color-surface-2)] text-[var(--color-text-muted)]'} flex items-center justify-center text-sm font-bold">
					{item.num}
				</span>
				<div>
					<div class="font-semibold text-sm">{item.label}</div>
					<div class="text-xs text-[var(--color-text-muted)]">{item.desc}</div>
				</div>
			</div>
		{/each}
	</div>
</section>

<!-- .cora.yaml Example -->
<section class="mb-12">
	<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">📄</span> .cora.yaml Example
	</h2>
	<p class="text-[var(--color-text-muted)] mb-4">Create this file in your project root. Run <code class="cmd-highlight">cora init</code> to generate it.</p>

	<div class="terminal-block">
<pre class="whitespace-pre text-sm"><span class="cmd-comment"># cora project config</span>
<span class="cmd-highlight">review:</span>
  <span class="cmd-flag">severity:</span> <span class="cmd-string">warning</span>          <span class="cmd-comment"># minimum severity: info, warning, error</span>
  <span class="cmd-flag">max_issues:</span> <span class="text-[var(--color-text)]">20</span>             <span class="cmd-comment"># max issues to report</span>
  <span class="cmd-flag">focus:</span> <span class="cmd-string">security,performance</span>  <span class="cmd-comment"># focus areas</span>

<span class="cmd-highlight">ignore:</span>
  <span class="text-[var(--color-text)]">- "vendor/**"</span>
  <span class="text-[var(--color-text)]">- "*.min.js"</span>
  <span class="text-[var(--color-text)]">- "migrations/**"</span>

<span class="cmd-highlight">providers:</span>
  <span class="cmd-flag">openai:</span>
    <span class="cmd-flag">model:</span> <span class="cmd-string">gpt-4o</span>
  <span class="cmd-flag">anthropic:</span>
    <span class="cmd-flag">model:</span> <span class="cmd-string">claude-sonnet-4-20250514</span></pre>
	</div>
</section>

<!-- Environment Variables -->
<section class="mb-12">
	<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">🔑</span> Environment Variables
	</h2>
	<div class="rounded-xl border border-[var(--color-border)] overflow-hidden">
		<table class="compare-table">
			<thead>
				<tr class="bg-[var(--color-surface)]">
					<th class="w-1/3">Variable</th>
					<th>Description</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td><code class="cmd-highlight">CORA_API_KEY</code></td>
					<td>API key for the active provider</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">CORA_PROVIDER</code></td>
					<td>Active provider (openai, anthropic, groq, ollama, zai)</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">CORA_MODEL</code></td>
					<td>Model name override</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">CORA_BASE_URL</code></td>
					<td>Custom API base URL</td>
				</tr>
			</tbody>
		</table>
	</div>
</section>

<!-- Provider env vars -->
<section>
	<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">🔐</span> Provider-Specific Env Vars
	</h2>
	<p class="text-[var(--color-text-muted)] mb-4">Each provider has its own API key variable. cora checks these for auto-detection.</p>
	<div class="terminal-block">
<pre class="whitespace-pre text-sm"><span class="cmd-comment"># OpenAI</span>
<span class="cmd-flag">OPENAI_API_KEY</span>=<span class="cmd-string">sk-...</span>
<span class="cmd-flag">OPENAI_BASE_URL</span>=<span class="cmd-string">https://api.openai.com/v1</span>

<span class="cmd-comment"># Anthropic</span>
<span class="cmd-flag">ANTHROPIC_API_KEY</span>=<span class="cmd-string">sk-ant-...</span>

<span class="cmd-comment"># Groq</span>
<span class="cmd-flag">GROQ_API_KEY</span>=<span class="cmd-string">gsk_...</span>

<span class="cmd-comment"># Ollama (local, no key needed)</span>
<span class="cmd-flag">OLLAMA_BASE_URL</span>=<span class="cmd-string">http://localhost:11434</span>

<span class="cmd-comment"># Z.AI</span>
<span class="cmd-flag">ZAI_API_KEY</span>=<span class="cmd-string">...</span></pre>
	</div>
</section>
