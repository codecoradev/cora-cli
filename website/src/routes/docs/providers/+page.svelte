<script lang="ts">
	import { onMount } from 'svelte';

	onMount(() => {
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) entry.target.classList.add('visible');
				});
			},
			{ threshold: 0.05, rootMargin: '0px 0px -20px 0px' }
		);
		document.querySelectorAll('.scroll-reveal').forEach((el) => observer.observe(el));
		return () => observer.disconnect();
	});
</script>

<svelte:head>
	<title>Providers — cora docs</title>
	<meta name="description" content="Supported AI providers for cora - OpenAI, Anthropic, Groq, Ollama, Z.AI." />
</svelte:head>

<h1 class="scroll-reveal" style="font-size: 2rem; font-weight: 600; letter-spacing: -0.02em; margin-bottom: 0.5rem;">Providers</h1>
<p class="scroll-reveal" style="color: var(--text-secondary); margin-bottom: 2.5rem;">cora supports multiple AI providers. Use your own API key &mdash; no subscriptions to us.</p>

<!-- Supported Providers -->
<section class="docs-section scroll-reveal">
	<h2 style="display: flex; align-items: center; gap: 0.5rem;">
		<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"/><path d="M8 21h8"/><path d="M12 17v4"/></svg>
		Supported Providers
	</h2>
	<div style="border: 1px solid var(--border); border-radius: 12px; overflow-x: auto;">
		<table class="compare-table">
			<thead>
				<tr>
					<th>Provider</th>
					<th>Default Model</th>
					<th>Env Var</th>
					<th>Custom Base URL</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td class="highlight-col">OpenAI</td>
					<td><code style="color: var(--text-secondary);">gpt-4o</code></td>
					<td><code class="syntax-flag">OPENAI_API_KEY</code></td>
					<td><code class="syntax-flag">OPENAI_BASE_URL</code></td>
				</tr>
				<tr>
					<td class="highlight-col">Anthropic</td>
					<td><code style="color: var(--text-secondary);">claude-sonnet-4-20250514</code></td>
					<td><code class="syntax-flag">ANTHROPIC_API_KEY</code></td>
					<td><code class="syntax-flag">ANTHROPIC_BASE_URL</code></td>
				</tr>
				<tr>
					<td class="highlight-col">Groq</td>
					<td><code style="color: var(--text-secondary);">llama-3.3-70b-versatile</code></td>
					<td><code class="syntax-flag">GROQ_API_KEY</code></td>
					<td><code class="syntax-flag">GROQ_BASE_URL</code></td>
				</tr>
				<tr>
					<td class="highlight-col">Ollama</td>
					<td><code style="color: var(--text-secondary);">llama3.1</code></td>
					<td><span style="color: var(--text-tertiary);">&mdash; (local)</span></td>
					<td><code class="syntax-flag">OLLAMA_BASE_URL</code> <span style="color: var(--text-tertiary); font-size: 0.75rem;">(default: http://localhost:11434)</span></td>
				</tr>
				<tr>
					<td class="highlight-col">Z.AI</td>
					<td><code style="color: var(--text-secondary);">glm-5.1</code></td>
					<td><code class="syntax-flag">ZAI_API_KEY</code></td>
					<td><code class="syntax-flag">ZAI_BASE_URL</code></td>
				</tr>
			</tbody>
		</table>
	</div>
</section>

<!-- Auto Detection -->
<section class="docs-section scroll-reveal">
	<h2 style="display: flex; align-items: center; gap: 0.5rem;">
		<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/></svg>
		Auto-Detection
	</h2>
	<p style="color: var(--text-secondary); margin-bottom: 1.5rem;">
		cora automatically detects which provider to use by checking environment variables in this order:
	</p>
	<div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1.5rem;">
		{#each [
			'OPENAI_API_KEY \u2192 uses OpenAI',
			'ANTHROPIC_API_KEY \u2192 uses Anthropic',
			'GROQ_API_KEY \u2192 uses Groq',
			'ZAI_API_KEY \u2192 uses Z.AI',
			'OLLAMA_BASE_URL \u2192 uses Ollama (localhost)'
		] as item, i}
			<div class="docs-card">
				<div class="docs-card-number muted">{i + 1}</div>
				<code style="font-size: 0.875rem; color: var(--text-secondary);">{item}</code>
			</div>
		{/each}
	</div>
	<p style="font-size: 0.875rem; color: var(--text-secondary);">
		Override auto-detection with <code class="syntax-highlight">CORA_PROVIDER</code> env var or <code class="syntax-flag">--provider</code> flag.
	</p>
</section>

<!-- Usage Examples -->
<section class="docs-section scroll-reveal">
	<h2 style="display: flex; align-items: center; gap: 0.5rem;">
		<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>
		Usage Examples
	</h2>
	<div style="display: flex; flex-direction: column; gap: 0.75rem;">
		<div class="docs-terminal">
			<div class="terminal-bar">
				<span class="terminal-dot terminal-dot-red"></span>
				<span class="terminal-dot terminal-dot-yellow"></span>
				<span class="terminal-dot terminal-dot-green"></span>
			</div>
			<div class="terminal-body">
				<span class="syntax-comment"># Use OpenAI (auto-detected from OPENAI_API_KEY)</span><br/>
				<span class="syntax-cmd">$</span> <span class="syntax-flag">OPENAI_API_KEY</span>=<span class="syntax-string">sk-...</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span>
			</div>
		</div>

		<div class="docs-terminal">
			<div class="terminal-bar">
				<span class="terminal-dot terminal-dot-red"></span>
				<span class="terminal-dot terminal-dot-yellow"></span>
				<span class="terminal-dot terminal-dot-green"></span>
			</div>
			<div class="terminal-body">
				<span class="syntax-comment"># Use Anthropic with explicit provider</span><br/>
				<span class="syntax-cmd">$</span> <span class="syntax-flag">CORA_PROVIDER</span>=<span class="syntax-string">anthropic</span> <span class="syntax-flag">CORA_API_KEY</span>=<span class="syntax-string">sk-ant-...</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span>
			</div>
		</div>

		<div class="docs-terminal">
			<div class="terminal-bar">
				<span class="terminal-dot terminal-dot-red"></span>
				<span class="terminal-dot terminal-dot-yellow"></span>
				<span class="terminal-dot terminal-dot-green"></span>
			</div>
			<div class="terminal-body">
				<span class="syntax-comment"># Use Ollama locally (no API key needed)</span><br/>
				<span class="syntax-cmd">$</span> <span class="syntax-flag">CORA_PROVIDER</span>=<span class="syntax-string">ollama</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span>
			</div>
		</div>

		<div class="docs-terminal">
			<div class="terminal-bar">
				<span class="terminal-dot terminal-dot-red"></span>
				<span class="terminal-dot terminal-dot-yellow"></span>
				<span class="terminal-dot terminal-dot-green"></span>
			</div>
			<div class="terminal-body">
				<span class="syntax-comment"># Use a custom model</span><br/>
				<span class="syntax-cmd">$</span> <span class="syntax-flag">CORA_MODEL</span>=<span class="syntax-string">gpt-4o-mini</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span>
			</div>
		</div>
	</div>
</section>
