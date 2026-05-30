<svelte:head>
	<title>Usage — cora docs</title>
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
	Usage
</h1>

<div class="docs-section scroll-reveal">
	<h2>Review Modes</h2>
	<p>cora supports four review modes, each suited to a different workflow:</p>

	<div style="overflow-x: auto;">
		<table class="compare-table">
			<thead>
				<tr>
					<th>Mode</th>
					<th>Flag</th>
					<th>Scope</th>
					<th>Best For</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td style="font-weight: 500;">Staged</td>
					<td><code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">--staged</code></td>
					<td>Files in git staging area</td>
					<td>Pre-commit review</td>
				</tr>
				<tr>
					<td style="font-weight: 500;">Branch</td>
					<td><code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">--branch</code></td>
					<td>Diff against base branch</td>
					<td>PR review workflow</td>
				</tr>
				<tr>
					<td style="font-weight: 500;">Full</td>
					<td><code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">--full</code></td>
					<td>Entire repository</td>
					<td>Comprehensive audit</td>
				</tr>
				<tr>
					<td style="font-weight: 500;">Incremental</td>
					<td><code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">--incremental</code></td>
					<td>Only new/changed files</td>
					<td>Large codebases</td>
				</tr>
			</tbody>
		</table>
	</div>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);"># Review staged changes</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora review</span> <span style="color: oklch(0.75 0.15 240);">--staged</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># Review against main branch</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora review</span> <span style="color: oklch(0.75 0.15 240);">--branch</span> <span style="color: var(--success);">main</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># Full project scan</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora review</span> <span style="color: oklch(0.75 0.15 240);">--full</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># Incremental (only changed files)</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora review</span> <span style="color: oklch(0.75 0.15 240);">--incremental</span></div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Output Formats</h2>
	<p>cora can output results in three formats:</p>

	<div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1rem;">
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<span style="color: var(--accent); font-weight: 600; font-family: var(--font-mono);">--format pretty</span> — Human-readable terminal output (default)
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<span style="color: var(--accent); font-weight: 600; font-family: var(--font-mono);">--format json</span> — Machine-readable JSON for CI/CD pipelines
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground);">
			<span style="color: var(--accent); font-weight: 600; font-family: var(--font-mono);">--format sarif</span> — SARIF format for GitHub Code Scanning
		</div>
	</div>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);"># JSON output example</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora review</span> <span style="color: oklch(0.75 0.15 240);">--staged</span> <span style="color: oklch(0.75 0.15 240);">--format</span> <span style="color: var(--success);">json</span></div>
			<div></div>
			<div>{'{'}</div>
			<div>  <span style="color: var(--accent);">"files"</span>: [</div>
			<div>    {'{'}</div>
			<div>      <span style="color: var(--accent);">"path"</span>: <span style="color: var(--success);">"src/auth/login.ts"</span>,</div>
			<div>      <span style="color: var(--accent);">"issues"</span>: [</div>
			<div>        {'{'}</div>
			<div>          <span style="color: var(--accent);">"line"</span>: <span style="color: oklch(0.75 0.15 240);">42</span>,</div>
			<div>          <span style="color: var(--accent);">"severity"</span>: <span style="color: var(--success);">"warning"</span>,</div>
			<div>          <span style="color: var(--accent);">"message"</span>: <span style="color: var(--success);">"Potential SQL injection"</span></div>
			<div>{'}'}</div>
			<div>      ]</div>
			<div>    {'}'}</div>
			<div>  ],</div>
			<div>  <span style="color: var(--accent);">"summary"</span>: {'{'}</div>
			<div>    <span style="color: var(--accent);">"total_files"</span>: <span style="color: oklch(0.75 0.15 240);">3</span>,</div>
			<div>    <span style="color: var(--accent);">"total_issues"</span>: <span style="color: oklch(0.75 0.15 240);">3</span></div>
			<div>{'}'}</div>
			<div>{'}'}</div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Configuration File</h2>
	<p>The <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">.cora.yaml</code> file provides persistent configuration. Place it in your project root or use <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">~/.cora/config.yaml</code> for global settings.</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);"># .cora.yaml — full example</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># LLM provider: openai, anthropic, groq, ollama, zai</span></div>
			<div><span style="color: var(--accent);">provider</span>: <span style="color: var(--success);">openai</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># Model name</span></div>
			<div><span style="color: var(--accent);">model</span>: <span style="color: var(--success);">gpt-4o</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># Custom base URL (for proxies or self-hosted endpoints)</span></div>
			<div><span style="color: var(--muted-foreground);"># base_url: https://api.example.com/v1</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># Custom review prompt</span></div>
			<div><span style="color: var(--muted-foreground);"># custom_prompt: |</span></div>
			<div><span style="color: var(--muted-foreground);">#   Focus on security and performance only.</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># File patterns to include</span></div>
			<div><span style="color: var(--muted-foreground);"># include:</span></div>
			<div><span style="color: var(--muted-foreground);">#   - "src/**"</span></div>
			<div><span style="color: var(--muted-foreground);">#   - "lib/**"</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># File patterns to exclude</span></div>
			<div><span style="color: var(--muted-foreground);"># exclude:</span></div>
			<div><span style="color: var(--muted-foreground);">#   - "**/*.test.ts"</span></div>
			<div><span style="color: var(--muted-foreground);">#   - "**/node_modules/**"</span></div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Environment Variables</h2>
	<p>Environment variables override configuration file settings:</p>

	<div style="overflow-x: auto;">
		<table class="compare-table">
			<thead>
				<tr>
					<th>Variable</th>
					<th>Description</th>
					<th>Required</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td><code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">CORA_API_KEY</code></td>
					<td>API key for the configured provider</td>
					<td>Yes (unless using cora auth)</td>
				</tr>
				<tr>
					<td><code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">CORA_PROVIDER</code></td>
					<td>Override the LLM provider</td>
					<td>No</td>
				</tr>
				<tr>
					<td><code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">CORA_MODEL</code></td>
					<td>Override the model name</td>
					<td>No</td>
				</tr>
				<tr>
					<td><code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">CORA_BASE_URL</code></td>
					<td>Override the API base URL</td>
					<td>No</td>
				</tr>
				<tr>
					<td><code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">CORA_CONFIG</code></td>
					<td>Path to alternative config file</td>
					<td>No</td>
				</tr>
			</tbody>
		</table>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Working with Monorepos</h2>
	<p>cora works well in monorepo setups. Use include patterns to limit review scope to specific packages:</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--destructive);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--warning);"></span>
			<span style="width: 8px; height: 8px; border-radius: 50%; background: var(--success);"></span>
		</div>
		<div class="terminal-body">
			<div><span style="color: var(--muted-foreground);"># Review only the backend package</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora review</span> <span style="color: oklch(0.75 0.15 240);">--staged</span> <span style="color: oklch(0.75 0.15 240);">--include</span> <span style="color: var(--success);">"packages/backend/**"</span></div>
			<div></div>
			<div><span style="color: var(--muted-foreground);"># Exclude test and generated files</span></div>
			<div><span style="color: var(--muted-foreground);">$</span> <span style="color: var(--accent);">cora review</span> <span style="color: oklch(0.75 0.15 240);">--staged</span> <span style="color: oklch(0.75 0.15 240);">--exclude</span> <span style="color: var(--success);">"**/*.test.ts"</span> <span style="color: oklch(0.75 0.15 240);">--exclude</span> <span style="color: var(--success);">"**/generated/**"</span></div>
		</div>
	</div>

	<p>Alternatively, set include/exclude patterns in <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">.cora.yaml</code> for persistent configuration.</p>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Exit Codes</h2>
	<p>cora uses standard exit codes for scripting and CI integration:</p>

	<div style="overflow-x: auto;">
		<table class="compare-table">
			<thead>
				<tr>
					<th>Code</th>
					<th>Meaning</th>
					<th>CI Behavior</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td><code style="color: var(--success); font-family: var(--font-mono); font-size: 13px;">0</code></td>
					<td>No issues found</td>
					<td>Pass</td>
				</tr>
				<tr>
					<td><code style="color: var(--warning); font-family: var(--font-mono); font-size: 13px;">1</code></td>
					<td>Issues found</td>
					<td>Fail (warning/error)</td>
				</tr>
				<tr>
					<td><code style="color: var(--destructive); font-family: var(--font-mono); font-size: 13px;">2</code></td>
					<td>Review blocked</td>
					<td>Fail (auth/config error)</td>
				</tr>
				<tr>
					<td><code style="color: var(--destructive); font-family: var(--font-mono); font-size: 13px;">3</code></td>
					<td>Authentication error</td>
					<td>Fail (missing API key)</td>
				</tr>
			</tbody>
		</table>
	</div>
</div>

<div class="docs-section scroll-reveal" style="margin-top: 3rem;">
	<h2>Tips</h2>
	<div style="display: flex; flex-direction: column; gap: 0.75rem;">
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			Use <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">--staged</code> as a pre-commit hook for the fastest feedback loop
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			Combine <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">--format json</code> with <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">--branch main</code> in CI pipelines
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			Use <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">--incremental</code> for large codebases — only changed files are analyzed
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			Set <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">custom_prompt</code> to match your team's coding standards
		</div>
		<div style="font-size: 14px; color: var(--muted-foreground); display: flex; gap: 0.5rem;">
			<span style="color: var(--accent);">&#8226;</span>
			Use <code style="color: var(--accent); font-family: var(--font-mono); font-size: 13px;">cora auth login</code> to store API keys securely instead of environment variables
		</div>
	</div>
</div>
