<svelte:head>
	<title>CLI Reference — cora docs</title>
	<meta name="description" content="Complete CLI reference for cora - AI code review tool commands, flags, and options." />
</svelte:head>

<h1 class="text-3xl md:text-4xl font-bold mb-2">CLI Reference</h1>
<p class="text-[var(--color-text-muted)] mb-10">Complete command reference for the cora CLI.</p>

<!-- Global Flags -->
<section class="mb-12">
	<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">$</span> Global Flags
	</h2>
	<div class="rounded-xl border border-[var(--color-border)] overflow-hidden">
		<table class="compare-table">
			<thead>
				<tr class="bg-[var(--color-surface)]">
					<th class="w-1/3">Flag</th>
					<th>Description</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td><code class="cmd-flag">--config</code> <code class="text-[var(--color-text-dim)]">&lt;path&gt;</code></td>
					<td>Override config file path (default: <code>.cora.yaml</code>)</td>
				</tr>
				<tr>
					<td><code class="cmd-flag">--json</code></td>
					<td>Output results as JSON</td>
				</tr>
				<tr>
					<td><code class="cmd-flag">--verbose</code></td>
					<td>Enable debug logging</td>
				</tr>
			</tbody>
		</table>
	</div>
</section>

<!-- Commands -->
<section class="mb-12">
	<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">⚡</span> Commands
	</h2>
	<div class="rounded-xl border border-[var(--color-border)] overflow-hidden">
		<table class="compare-table">
			<thead>
				<tr class="bg-[var(--color-surface)]">
					<th class="w-1/3">Command</th>
					<th>Description</th>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td><code class="cmd-highlight">cora init</code></td>
					<td>Create <code>.cora.yaml</code> config file</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora review --staged</code></td>
					<td>Review staged git changes (default)</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora review --branch</code> <code class="text-[var(--color-text-dim)]">&lt;name&gt;</code></td>
					<td>Compare current branch against target</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora review --diff</code> <code class="text-[var(--color-text-dim)]">&lt;base&gt;..&lt;head&gt;</code></td>
					<td>Review specific diff range</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora review --file</code> <code class="text-[var(--color-text-dim)]">&lt;path&gt;</code></td>
					<td>Review single file</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora scan</code> <code class="cmd-flag">[--incremental]</code></td>
					<td>Full project scan</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora hook install</code></td>
					<td>Install pre-commit hook</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora hook uninstall</code></td>
					<td>Remove pre-commit hook</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora auth login</code></td>
					<td>Save API key to <code>~/.cora/config.toml</code></td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora auth status</code></td>
					<td>Check current auth status</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora providers</code></td>
					<td>List supported AI providers</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora completion</code> <code class="text-[var(--color-text-dim)]">&lt;shell&gt;</code></td>
					<td>Generate shell completions (bash/zsh/fish/powershell)</td>
				</tr>
				<tr>
					<td><code class="cmd-highlight">cora upload-sarif</code> <code class="text-[var(--color-text-dim)]">&lt;file&gt;</code></td>
					<td>Upload SARIF to GitHub Code Scanning</td>
				</tr>
			</tbody>
		</table>
	</div>
</section>

<!-- Quick examples -->
<section>
	<h2 class="text-xl font-semibold mb-4 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">💡</span> Quick Examples
	</h2>
	<div class="space-y-4">
		<div class="terminal-block">
			<span class="cmd-comment"># Review staged changes (what's about to be committed)</span><br/>
			<span class="cmd-comment">$</span> <span class="cmd-highlight">cora review</span> <span class="cmd-flag">--staged</span>
		</div>
		<div class="terminal-block">
			<span class="cmd-comment"># Compare your feature branch against main</span><br/>
			<span class="cmd-comment">$</span> <span class="cmd-highlight">cora review</span> <span class="cmd-flag">--branch</span> <span class="cmd-string">main</span>
		</div>
		<div class="terminal-block">
			<span class="cmd-comment"># Full project scan with incremental caching</span><br/>
			<span class="cmd-comment">$</span> <span class="cmd-highlight">cora scan</span> <span class="cmd-flag">--incremental</span>
		</div>
		<div class="terminal-block">
			<span class="cmd-comment"># Install pre-commit hook</span><br/>
			<span class="cmd-comment">$</span> <span class="cmd-highlight">cora hook</span> <span class="cmd-string">install</span>
		</div>
	</div>
</section>
