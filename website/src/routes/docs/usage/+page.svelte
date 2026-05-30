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

<div class="docs-content">
<h1 class="scroll-reveal">
	Usage
</h1>

<div class="docs-section scroll-reveal">
	<h2>Review Modes</h2>
	<p>cora supports four review modes, each suited to a different workflow:</p>

	<div class="overflow-x-auto">
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
					<td class="font-medium">Staged</td>
					<td><code>--staged</code></td>
					<td>Files in git staging area</td>
					<td>Pre-commit review</td>
				</tr>
				<tr>
					<td class="font-medium">Branch</td>
					<td><code>--branch</code></td>
					<td>Diff against base branch</td>
					<td>PR review workflow</td>
				</tr>
				<tr>
					<td class="font-medium">Full</td>
					<td><code>--full</code></td>
					<td>Entire repository</td>
					<td>Comprehensive audit</td>
				</tr>
				<tr>
					<td class="font-medium">Incremental</td>
					<td><code>--incremental</code></td>
					<td>Only new/changed files</td>
					<td>Large codebases</td>
				</tr>
			</tbody>
		</table>
	</div>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot-red"></span>
			<span class="terminal-dot-yellow"></span>
			<span class="terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<div><span class="syntax-comment"># Review staged changes</span></div>
			<div><span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span></div>
			<div></div>
			<div><span class="syntax-comment"># Review against main branch</span></div>
			<div><span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--branch</span> <span class="syntax-string">main</span></div>
			<div></div>
			<div><span class="syntax-comment"># Full project scan</span></div>
			<div><span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--full</span></div>
			<div></div>
			<div><span class="syntax-comment"># Incremental (only changed files)</span></div>
			<div><span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--incremental</span></div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal">
	<h2>Output Formats</h2>
	<p>cora can output results in three formats:</p>

	<div class="docs-term-list">
		<div class="text-sm text-[var(--muted-foreground)]">
			<span class="font-semibold text-[var(--accent)] font-mono">--format pretty</span> — Human-readable terminal output (default)
		</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<span class="font-semibold text-[var(--accent)] font-mono">--format json</span> — Machine-readable JSON for CI/CD pipelines
		</div>
		<div class="text-sm text-[var(--muted-foreground)]">
			<span class="font-semibold text-[var(--accent)] font-mono">--format sarif</span> — SARIF format for GitHub Code Scanning
		</div>
	</div>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot-red"></span>
			<span class="terminal-dot-yellow"></span>
			<span class="terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<div><span class="syntax-comment"># JSON output example</span></div>
			<div><span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span> <span class="syntax-flag">--format</span> <span class="syntax-string">json</span></div>
			<div></div>
			<div>{'{'}</div>
			<div>  <span class="syntax-highlight">"files"</span>: [</div>
			<div>    {'{'}</div>
			<div>      <span class="syntax-highlight">"path"</span>: <span class="syntax-string">"src/auth/login.ts"</span>,</div>
			<div>      <span class="syntax-highlight">"issues"</span>: [</div>
			<div>        {'{'}</div>
			<div>          <span class="syntax-highlight">"line"</span>: <span class="syntax-flag">42</span>,</div>
			<div>          <span class="syntax-highlight">"severity"</span>: <span class="syntax-string">"warning"</span>,</div>
			<div>          <span class="syntax-highlight">"message"</span>: <span class="syntax-string">"Potential SQL injection"</span></div>
			<div>{'}'}</div>
			<div>      ]</div>
			<div>    {'}'}</div>
			<div>  ],</div>
			<div>  <span class="syntax-highlight">"summary"</span>: {'{'}</div>
			<div>    <span class="syntax-highlight">"total_files"</span>: <span class="syntax-flag">3</span>,</div>
			<div>    <span class="syntax-highlight">"total_issues"</span>: <span class="syntax-flag">3</span></div>
			<div>{'}'}</div>
			<div>{'}'}</div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal">
	<h2>Configuration File</h2>
	<p>The <code>.cora.yaml</code> file provides persistent configuration. Place it in your project root or use <code>~/.cora/config.yaml</code> for global settings.</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot-red"></span>
			<span class="terminal-dot-yellow"></span>
			<span class="terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<div><span class="syntax-comment"># .cora.yaml — full example</span></div>
			<div></div>
			<div><span class="syntax-comment"># LLM provider: openai, anthropic, groq, ollama, zai</span></div>
			<div><span class="syntax-highlight">provider</span>: <span class="syntax-string">openai</span></div>
			<div></div>
			<div><span class="syntax-comment"># Model name</span></div>
			<div><span class="syntax-highlight">model</span>: <span class="syntax-string">gpt-4o</span></div>
			<div></div>
			<div><span class="syntax-comment"># Custom base URL (for proxies or self-hosted endpoints)</span></div>
			<div><span class="syntax-comment"># base_url: https://api.example.com/v1</span></div>
			<div></div>
			<div><span class="syntax-comment"># Custom review prompt</span></div>
			<div><span class="syntax-comment"># custom_prompt: |</span></div>
			<div><span class="syntax-comment">#   Focus on security and performance only.</span></div>
			<div></div>
			<div><span class="syntax-comment"># File patterns to include</span></div>
			<div><span class="syntax-comment"># include:</span></div>
			<div><span class="syntax-comment">#   - "src/**"</span></div>
			<div><span class="syntax-comment">#   - "lib/**"</span></div>
			<div></div>
			<div><span class="syntax-comment"># File patterns to exclude</span></div>
			<div><span class="syntax-comment"># exclude:</span></div>
			<div><span class="syntax-comment">#   - "**/*.test.ts"</span></div>
			<div><span class="syntax-comment">#   - "**/node_modules/**"</span></div>
		</div>
	</div>
</div>

<div class="docs-section scroll-reveal">
	<h2>Environment Variables</h2>
	<p>Environment variables override configuration file settings:</p>

	<div class="overflow-x-auto">
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
					<td><code>CORA_API_KEY</code></td>
					<td>API key for the configured provider</td>
					<td>Yes (unless using cora auth)</td>
				</tr>
				<tr>
					<td><code>CORA_PROVIDER</code></td>
					<td>Override the LLM provider</td>
					<td>No</td>
				</tr>
				<tr>
					<td><code>CORA_MODEL</code></td>
					<td>Override the model name</td>
					<td>No</td>
				</tr>
				<tr>
					<td><code>CORA_BASE_URL</code></td>
					<td>Override the API base URL</td>
					<td>No</td>
				</tr>
				<tr>
					<td><code>CORA_CONFIG</code></td>
					<td>Path to alternative config file</td>
					<td>No</td>
				</tr>
			</tbody>
		</table>
	</div>
</div>

<div class="docs-section scroll-reveal">
	<h2>Working with Monorepos</h2>
	<p>cora works well in monorepo setups. Use include patterns to limit review scope to specific packages:</p>

	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot-red"></span>
			<span class="terminal-dot-yellow"></span>
			<span class="terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<div><span class="syntax-comment"># Review only the backend package</span></div>
			<div><span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span> <span class="syntax-flag">--include</span> <span class="syntax-string">"packages/backend/**"</span></div>
			<div></div>
			<div><span class="syntax-comment"># Exclude test and generated files</span></div>
			<div><span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span> <span class="syntax-flag">--exclude</span> <span class="syntax-string">"**/*.test.ts"</span> <span class="syntax-flag">--exclude</span> <span class="syntax-string">"**/generated/**"</span></div>
		</div>
	</div>

	<p>Alternatively, set include/exclude patterns in <code>.cora.yaml</code> for persistent configuration.</p>
</div>

<div class="docs-section scroll-reveal">
	<h2>Exit Codes</h2>
	<p>cora uses standard exit codes for scripting and CI integration:</p>

	<div class="overflow-x-auto">
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
					<td><code class="syntax-success">0</code></td>
					<td>No issues found</td>
					<td>Pass</td>
				</tr>
				<tr>
					<td><code class="syntax-warning">1</code></td>
					<td>Issues found</td>
					<td>Fail (warning/error)</td>
				</tr>
				<tr>
					<td><code class="syntax-error">2</code></td>
					<td>Review blocked</td>
					<td>Fail (auth/config error)</td>
				</tr>
				<tr>
					<td><code class="syntax-error">3</code></td>
					<td>Authentication error</td>
					<td>Fail (missing API key)</td>
				</tr>
			</tbody>
		</table>
	</div>
</div>

<div class="docs-section scroll-reveal">
	<h2>Tips</h2>
	<div class="flex flex-col gap-3">
		<div class="docs-term-item">
			<span class="text-[var(--accent)]">&#8226;</span>
			Use <code>--staged</code> as a pre-commit hook for the fastest feedback loop
		</div>
		<div class="docs-term-item">
			<span class="text-[var(--accent)]">&#8226;</span>
			Combine <code>--format json</code> with <code>--branch main</code> in CI pipelines
		</div>
		<div class="docs-term-item">
			<span class="text-[var(--accent)]">&#8226;</span>
			Use <code>--incremental</code> for large codebases — only changed files are analyzed
		</div>
		<div class="docs-term-item">
			<span class="text-[var(--accent)]">&#8226;</span>
			Set <code>custom_prompt</code> to match your team's coding standards
		</div>
		<div class="docs-term-item">
			<span class="text-[var(--accent)]">&#8226;</span>
			Use <code>cora auth login</code> to store API keys securely instead of environment variables
		</div>
	</div>
</div>
</div>
