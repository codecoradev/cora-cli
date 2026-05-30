<svelte:head>
	<title>Examples — cora docs</title>
	<meta name="description" content="Practical examples for using cora - quick review, CI, pre-commit hooks, SARIF, and more." />
</svelte:head>

<h1 class="text-3xl md:text-4xl font-bold mb-2">Examples</h1>
<p class="text-[var(--color-text-muted)] mb-10">Practical examples to get you started with cora.</p>

<!-- Quick Review -->
<section class="mb-10">
	<h2 class="text-xl font-semibold mb-3 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">1.</span> Quick Review
	</h2>
	<p class="text-[var(--color-text-muted)] text-sm mb-3">Review your staged changes before committing.</p>
	<div class="terminal-block">
		<span class="cmd-comment">$</span> <span class="cmd-highlight">CORA_API_KEY</span>=<span class="cmd-string">sk-xxx</span> <span class="cmd-highlight">cora review</span> <span class="cmd-flag">--staged</span>
	</div>
</section>

<!-- Branch Comparison -->
<section class="mb-10">
	<h2 class="text-xl font-semibold mb-3 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">2.</span> Branch Comparison
	</h2>
	<p class="text-[var(--color-text-muted)] text-sm mb-3">Compare your current branch against main.</p>
	<div class="terminal-block">
		<span class="cmd-comment">$</span> <span class="cmd-highlight">cora review</span> <span class="cmd-flag">--branch</span> <span class="cmd-string">main</span>
	</div>
</section>

<!-- Full Project Scan -->
<section class="mb-10">
	<h2 class="text-xl font-semibold mb-3 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">3.</span> Full Project Scan
	</h2>
	<p class="text-[var(--color-text-muted)] text-sm mb-3">Scan your entire project for issues.</p>
	<div class="terminal-block">
		<span class="cmd-comment">$</span> <span class="cmd-highlight">cora scan</span>
	</div>
</section>

<!-- Incremental Scan -->
<section class="mb-10">
	<h2 class="text-xl font-semibold mb-3 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">4.</span> Incremental Scan
	</h2>
	<p class="text-[var(--color-text-muted)] text-sm mb-3">Only scan files that changed since the last scan.</p>
	<div class="terminal-block">
		<span class="cmd-comment">$</span> <span class="cmd-highlight">cora scan</span> <span class="cmd-flag">--incremental</span>
	</div>
</section>

<!-- Streaming -->
<section class="mb-10">
	<h2 class="text-xl font-semibold mb-3 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">5.</span> Streaming Output
	</h2>
	<p class="text-[var(--color-text-muted)] text-sm mb-3">Stream results as they come in from the LLM.</p>
	<div class="terminal-block">
		<span class="cmd-comment">$</span> <span class="cmd-highlight">cora review</span> <span class="cmd-flag">--staged</span> <span class="cmd-flag">--stream</span>
	</div>
</section>

<!-- GitHub Actions -->
<section class="mb-10">
	<h2 class="text-xl font-semibold mb-3 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">6.</span> GitHub Actions CI
	</h2>
	<p class="text-[var(--color-text-muted)] text-sm mb-3">Add cora to your CI pipeline.</p>
	<div class="terminal-block">
<pre class="whitespace-pre text-sm"><span class="cmd-comment"># .github/workflows/cora-review.yml</span>
<span class="cmd-highlight">name:</span> <span class="cmd-string">Code Review</span>

<span class="cmd-highlight">on:</span>
  <span class="cmd-flag">pull_request:</span>
    <span class="cmd-flag">branches:</span> [<span class="cmd-string">main</span>]

<span class="cmd-highlight">jobs:</span>
  <span class="cmd-flag">review:</span>
    <span class="cmd-flag">runs-on:</span> <span class="cmd-string">ubuntu-latest</span>
    <span class="cmd-flag">steps:</span>
      <span class="text-[var(--color-text)]">- uses:</span> <span class="cmd-string">actions/checkout@v4</span>

      <span class="text-[var(--color-text)]">- name:</span> <span class="cmd-string">Install cora</span>
        <span class="cmd-flag">run:</span> <span class="cmd-string">cargo install cora</span>

      <span class="text-[var(--color-text)]">- name:</span> <span class="cmd-string">Run AI code review</span>
        <span class="cmd-flag">env:</span>
          {@html '<span class="cmd-flag">CORA_API_KEY:</span> <span class="cmd-string">${{ secrets.CORA_API_KEY }}</span>'}
          <span class="cmd-flag">CORA_PROVIDER:</span> <span class="cmd-string">openai</span>
        <span class="cmd-flag">run:</span> <span class="cmd-string">cora review --branch main --fail-on error</span></pre>
	</div>
</section>

<!-- Pre-commit Hook -->
<section class="mb-10">
	<h2 class="text-xl font-semibold mb-3 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">7.</span> Pre-commit Hook
	</h2>
	<p class="text-[var(--color-text-muted)] text-sm mb-3">Install once, then every commit gets reviewed automatically.</p>
	<div class="terminal-block">
		<span class="cmd-comment"># Install the hook</span><br/>
		<span class="cmd-comment">$</span> <span class="cmd-highlight">cora hook</span> <span class="cmd-string">install</span><br/><br/>
		<span class="cmd-comment"># Now just commit normally — cora reviews automatically</span><br/>
		<span class="cmd-comment">$</span> <span class="cmd-highlight">git</span> <span class="cmd-string">commit</span> <span class="cmd-flag">-m</span> <span class="cmd-string">"fix: handle edge case in parser"</span><br/>
		<span class="text-[var(--color-text-muted)]">🪝 cora pre-commit hook running...</span><br/>
		<span class="text-[var(--color-success)]">✓ No issues found — commit allowed</span>
	</div>
</section>

<!-- SARIF Upload -->
<section>
	<h2 class="text-xl font-semibold mb-3 flex items-center gap-2">
		<span class="text-[var(--color-accent)]">8.</span> SARIF Upload
	</h2>
	<p class="text-[var(--color-text-muted)] text-sm mb-3">Generate SARIF output and upload to GitHub Code Scanning.</p>
	<div class="terminal-block">
		<span class="cmd-comment"># Generate SARIF report and upload</span><br/>
		<span class="cmd-comment">$</span> <span class="cmd-highlight">cora review</span> <span class="cmd-flag">--staged</span> <span class="cmd-flag">--output</span> <span class="cmd-string">sarif</span> <span class="text-[var(--color-text-muted)]">></span> <span class="cmd-string">results.sarif</span> <span class="text-[var(--color-text-muted)]">&&</span> \<br/>
		&nbsp;&nbsp;<span class="cmd-highlight">cora upload-sarif</span> <span class="cmd-string">results.sarif</span><br/><br/>
		<span class="text-[var(--color-success)]">✓ Uploaded 3 findings to GitHub Code Scanning</span>
	</div>
</section>
