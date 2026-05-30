<svelte:head>
	<title>Providers — cora docs</title>
	<meta name="description" content="Supported AI providers for cora - OpenAI, Anthropic, Groq, Ollama, Z.AI." />
</svelte:head>

<h1 class="text-3xl md:text-4xl font-bold mb-2">Providers</h1>
<p class="text-[var(--color-text-muted)] mb-10">cora supports multiple AI providers. Use your own API key — no subscriptions to us.</p>

<!-- Supported Providers -->
<section class="mb-12">
	<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">🧠</span> Supported Providers
	</h2>
	<div class="rounded-xl border border-[var(--color-border)] overflow-x-auto">
		<table class="compare-table">
			<thead>
				<tr class="bg-[var(--color-surface)]">
					<th>Provider</th>
					<th>Default Model</th>
					<th>Env Var</th>
					<th>Custom Base URL</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td class="font-medium highlight-col">OpenAI</td>
					<td><code>gpt-4o</code></td>
					<td><code class="cmd-flag">OPENAI_API_KEY</code></td>
					<td><code class="cmd-flag">OPENAI_BASE_URL</code></td>
				</tr>
				<tr>
					<td class="font-medium highlight-col">Anthropic</td>
					<td><code>claude-sonnet-4-20250514</code></td>
					<td><code class="cmd-flag">ANTHROPIC_API_KEY</code></td>
					<td><code class="cmd-flag">ANTHROPIC_BASE_URL</code></td>
				</tr>
				<tr>
					<td class="font-medium highlight-col">Groq</td>
					<td><code>llama-3.3-70b-versatile</code></td>
					<td><code class="cmd-flag">GROQ_API_KEY</code></td>
					<td><code class="cmd-flag">GROQ_BASE_URL</code></td>
				</tr>
				<tr>
					<td class="font-medium highlight-col">Ollama</td>
					<td><code>llama3.1</code></td>
					<td><span class="text-[var(--color-text-dim)]">— (local)</span></td>
					<td><code class="cmd-flag">OLLAMA_BASE_URL</code> <span class="text-[var(--color-text-dim)] text-xs">(default: http://localhost:11434)</span></td>
				</tr>
				<tr>
					<td class="font-medium highlight-col">Z.AI</td>
					<td><code>glm-5.1</code></td>
					<td><code class="cmd-flag">ZAI_API_KEY</code></td>
					<td><code class="cmd-flag">ZAI_BASE_URL</code></td>
				</tr>
			</tbody>
		</table>
	</div>
</section>

<!-- Auto Detection -->
<section class="mb-12">
	<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">🔍</span> Auto-Detection
	</h2>
	<p class="text-[var(--color-text-muted)] mb-6">
		cora automatically detects which provider to use by checking environment variables in this order:
	</p>

	<div class="space-y-2 mb-6">
		{#each [
			'OPENAI_API_KEY → uses OpenAI',
			'ANTHROPIC_API_KEY → uses Anthropic',
			'GROQ_API_KEY → uses Groq',
			'ZAI_API_KEY → uses Z.AI',
			'OLLAMA_BASE_URL → uses Ollama (localhost)'
		] as item, i}
			<div class="flex items-center gap-3 px-4 py-2.5 rounded-lg bg-[var(--color-surface)] border border-[var(--color-border)]">
				<span class="flex-shrink-0 w-6 h-6 rounded-full bg-[var(--color-surface-2)] text-[var(--color-text-muted)] flex items-center justify-center text-xs font-bold">
					{i + 1}
				</span>
				<code class="text-sm">{item}</code>
			</div>
		{/each}
	</div>

	<p class="text-sm text-[var(--color-text-muted)]">
		Override auto-detection with <code class="cmd-highlight">CORA_PROVIDER</code> env var or <code class="cmd-flag">--provider</code> flag.
	</p>
</section>

<!-- Examples -->
<section>
	<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">💡</span> Usage Examples
	</h2>
	<div class="space-y-4">
		<div class="terminal-block">
			<span class="cmd-comment"># Use OpenAI (auto-detected from OPENAI_API_KEY)</span><br/>
			<span class="cmd-comment">$</span> <span class="cmd-highlight">OPENAI_API_KEY</span>=<span class="cmd-string">sk-...</span> <span class="cmd-highlight">cora review</span> <span class="cmd-flag">--staged</span>
		</div>
		<div class="terminal-block">
			<span class="cmd-comment"># Use Anthropic with explicit provider</span><br/>
			<span class="cmd-comment">$</span> <span class="cmd-highlight">CORA_PROVIDER</span>=<span class="cmd-string">anthropic</span> <span class="cmd-highlight">CORA_API_KEY</span>=<span class="cmd-string">sk-ant-...</span> <span class="cmd-highlight">cora review</span> <span class="cmd-flag">--staged</span>
		</div>
		<div class="terminal-block">
			<span class="cmd-comment"># Use Ollama locally (no API key needed)</span><br/>
			<span class="cmd-comment">$</span> <span class="cmd-highlight">CORA_PROVIDER</span>=<span class="cmd-string">ollama</span> <span class="cmd-highlight">cora review</span> <span class="cmd-flag">--staged</span>
		</div>
		<div class="terminal-block">
			<span class="cmd-comment"># Use a custom model</span><br/>
			<span class="cmd-comment">$</span> <span class="cmd-highlight">CORA_MODEL</span>=<span class="cmd-string">gpt-4o-mini</span> <span class="cmd-highlight">cora review</span> <span class="cmd-flag">--staged</span>
		</div>
	</div>
</section>
