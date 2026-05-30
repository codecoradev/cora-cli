<svelte:head>
	<title>Self-Hosted — cora docs</title>
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
	Self-Hosted
</h1>

<div class="docs-section scroll-reveal">
	<h2>Why Self-Hosted?</h2>
	<p>cora is designed to run entirely on your infrastructure. Self-hosting gives you:</p>
	<div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			<span style="color: var(--foreground); font-weight: 500;">Privacy</span> — Your code never leaves your network
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			<span style="color: var(--foreground); font-weight: 500;">Compliance</span> — Meet regulatory requirements for code review in sensitive environments
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			<span style="color: var(--foreground); font-weight: 500;">Air-gapped</span> — Works in fully offline environments with local LLMs
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			<span style="color: var(--foreground); font-weight: 500;">No vendor lock-in</span> — Switch providers or models at any time
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Using Ollama (Local LLM)</h2>
	<p>Ollama lets you run LLMs locally. cora works with any Ollama model that supports chat completions.</p>

	<div class="docs-card" style="margin-top: 1rem; margin-bottom: 0.5rem;">
		<div class="docs-card-number primary">1</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<strong style="color: var(--foreground);">Install Ollama</strong> — Follow the instructions at <a href="https://ollama.ai" target="_blank" rel="noopener" style="color: var(--accent); text-decoration: none;">ollama.ai</a>
		</div>
	</div>

	<div class="docs-card" style="margin-bottom: 0.5rem;">
		<div class="docs-card-number primary">2</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<strong style="color: var(--foreground);">Pull a model</strong> — For code review, use <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">ollama pull codellama</code>
		</div>
	</div>

	<div class="docs-card" style="margin-bottom: 0.5rem;">
		<div class="docs-card-number primary">3</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<strong style="color: var(--foreground);">Configure cora</strong> — Point cora to your local Ollama instance
		</div>
	</div>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);"># Start Ollama</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">ollama serve</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># Pull a code-focused model</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">ollama pull</span> <span style="color: var(--success);">codellama</span></div>
		</div>
	</div>

	<p style="margin-top: 1rem;">Configure cora to use Ollama via environment variables:</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: oklch(0.75 0.15 240);">CORA_PROVIDER</span>=<span style="color: var(--success);">ollama</span> <span style="color: oklch(0.75 0.15 240);">CORA_BASE_URL</span>=<span style="color: var(--success);">http://localhost:11434/v1</span> <span style="color: oklch(0.75 0.15 240);">CORA_MODEL</span>=<span style="color: var(--success);">codellama</span> <span style="color: var(--accent);">cora review</span> <span style="color: oklch(0.75 0.15 240);">--staged</span></div>
		</div>
	</div>

	<p style="margin-top: 1rem;">Or persist the configuration in <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">.cora.yaml</code>:</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);"># .cora.yaml</span></div>
			<div><span style="color: var(--accent);">provider</span>: <span style="color: var(--success);">ollama</span></div>
			<div><span style="color: var(--accent);">base_url</span>: <span style="color: var(--success);">http://localhost:11434/v1</span></div>
			<div><span style="color: var(--accent);">model</span>: <span style="color: var(--success);">codellama</span></div>
		</div>
	</div>

	<p style="margin-top: 1rem;">To authenticate with Ollama (no API key needed for local use):</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora auth login</span> <span style="color: oklch(0.75 0.15 240);">--provider</span> <span style="color: var(--success);">ollama</span> <span style="color: oklch(0.75 0.15 240);">--base-url</span> <span style="color: var(--success);">http://localhost:11434/v1</span></div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Using with Gitea</h2>
	<p>cora works seamlessly with Gitea repositories. Since cora reads your local git state directly, no Gitea integration or plugin is needed:</p>

	<div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			cora reads <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">git diff</code> output — it works with any git remote (GitHub, Gitea, GitLab, Bitbucket)
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			No API tokens, webhooks, or server-side configuration required
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			Works with Gitea's CI/CD pipelines — just add <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">cora review --branch main</code> as a pipeline step
		</div>
	</div>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);"># Clone your Gitea repo and review</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">git clone</span> <span style="color: var(--success);">gitea.example.com/org/repo.git</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cd</span> repo</div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora init</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora review</span> <span style="color: oklch(0.75 0.15 240);">--staged</span></div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Network Isolation</h2>
	<p>For fully offline or air-gapped environments:</p>

	<div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			Download the cora binary and transfer it to the isolated machine
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			Run Ollama on the same machine with a locally-pulled model
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			Configure cora to use <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">http://localhost:11434/v1</code> as the base URL
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			No outbound network connections are made — everything stays on the local machine
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>No Telemetry</h2>
	<p>cora is designed with privacy as a core principle:</p>

	<div style="display: flex; flex-direction: column; gap: 0.75rem;">
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			<span style="color: var(--foreground); font-weight: 500;">Zero data collection</span> — cora does not send any data to any server
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			<span style="color: var(--foreground); font-weight: 500;">No analytics</span> — No usage tracking, no metrics, no phone home
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			<span style="color: var(--foreground); font-weight: 500;">Local execution</span> — Everything runs on your machine
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			<span style="color: var(--foreground); font-weight: 500;">The only network traffic</span> — API calls to your configured LLM provider (which you control)
		</div>
	</div>

	<p style="margin-top: 1rem;">You can verify this by inspecting the source code at <a href="https://github.com/ajianaz/cora-cli" target="_blank" rel="noopener" style="color: var(--accent); text-decoration: none;">github.com/ajianaz/cora-cli</a>. The codebase is open source under the MIT license.</p>
</div>
