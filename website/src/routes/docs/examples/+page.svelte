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
	<title>Examples — cora docs</title>
	<meta name="description" content="Practical examples for using cora - quick review, CI, pre-commit hooks, SARIF, and more." />
</svelte:head>

<h1 class="scroll-reveal" style="font-size: 2rem; font-weight: 600; letter-spacing: -0.02em; margin-bottom: 0.5rem;">Examples</h1>
<p class="scroll-reveal" style="color: var(--text-secondary); margin-bottom: 2.5rem;">Practical examples to get you started with cora.</p>

<!-- Quick Review -->
<section class="docs-section scroll-reveal">
	<h2 style="display: flex; align-items: center; gap: 0.5rem;">
		<span style="color: var(--accent); font-family: 'JetBrains Mono', monospace; font-size: 0.875rem;">01</span>
		Quick Review
	</h2>
	<p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 0.75rem;">Review your staged changes before committing.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<span class="syntax-cmd">$</span> <span class="syntax-flag">CORA_API_KEY</span>=<span class="syntax-string">sk-xxx</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span>
		</div>
	</div>
</section>

<!-- Branch Comparison -->
<section class="docs-section scroll-reveal">
	<h2 style="display: flex; align-items: center; gap: 0.5rem;">
		<span style="color: var(--accent); font-family: 'JetBrains Mono', monospace; font-size: 0.875rem;">02</span>
		Branch Comparison
	</h2>
	<p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 0.75rem;">Compare your current branch against main.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--branch</span> <span class="syntax-string">main</span>
		</div>
	</div>
</section>

<!-- Full Project Scan -->
<section class="docs-section scroll-reveal">
	<h2 style="display: flex; align-items: center; gap: 0.5rem;">
		<span style="color: var(--accent); font-family: 'JetBrains Mono', monospace; font-size: 0.875rem;">03</span>
		Full Project Scan
	</h2>
	<p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 0.75rem;">Scan your entire project for issues.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora scan</span>
		</div>
	</div>
</section>

<!-- Incremental Scan -->
<section class="docs-section scroll-reveal">
	<h2 style="display: flex; align-items: center; gap: 0.5rem;">
		<span style="color: var(--accent); font-family: 'JetBrains Mono', monospace; font-size: 0.875rem;">04</span>
		Incremental Scan
	</h2>
	<p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 0.75rem;">Only scan files that changed since the last scan.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora scan</span> <span class="syntax-flag">--incremental</span>
		</div>
	</div>
</section>

<!-- Streaming -->
<section class="docs-section scroll-reveal">
	<h2 style="display: flex; align-items: center; gap: 0.5rem;">
		<span style="color: var(--accent); font-family: 'JetBrains Mono', monospace; font-size: 0.875rem;">05</span>
		Streaming Output
	</h2>
	<p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 0.75rem;">Stream results as they come in from the LLM.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span> <span class="syntax-flag">--stream</span>
		</div>
	</div>
</section>

<!-- GitHub Actions -->
<section class="docs-section scroll-reveal">
	<h2 style="display: flex; align-items: center; gap: 0.5rem;">
		<span style="color: var(--accent); font-family: 'JetBrains Mono', monospace; font-size: 0.875rem;">06</span>
		GitHub Actions CI
	</h2>
	<p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 0.75rem;">Add cora to your CI pipeline.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
			<span class="terminal-title">.github/workflows/cora-review.yml</span>
		</div>
		<div class="terminal-body">
<pre class="whitespace-pre"><span class="syntax-highlight">name:</span> <span class="syntax-string">Code Review</span>

<span class="syntax-highlight">on:</span>
  <span class="syntax-flag">pull_request:</span>
    <span class="syntax-flag">branches:</span> [<span class="syntax-string">main</span>]

<span class="syntax-highlight">jobs:</span>
  <span class="syntax-flag">review:</span>
    <span class="syntax-flag">runs-on:</span> <span class="syntax-string">ubuntu-latest</span>
    <span class="syntax-flag">steps:</span>
      <span style="color: var(--text-secondary);">- uses:</span> <span class="syntax-string">actions/checkout@v4</span>
      <span style="color: var(--text-secondary);">- name:</span> <span class="syntax-string">Install cora</span>
        <span class="syntax-flag">run:</span> <span class="syntax-string">cargo install cora</span>
      <span style="color: var(--text-secondary);">- name:</span> <span class="syntax-string">Run AI code review</span>
        <span class="syntax-flag">env:</span>
          {@html '<span class="syntax-flag">CORA_API_KEY:</span> <span class="syntax-string">${{ secrets.CORA_API_KEY }}</span>'}
          <span class="syntax-flag">CORA_PROVIDER:</span> <span class="syntax-string">openai</span>
        <span class="syntax-flag">run:</span> <span class="syntax-string">cora review --branch main --fail-on error</span></pre>
		</div>
	</div>
</section>

<!-- Pre-commit Hook -->
<section class="docs-section scroll-reveal">
	<h2 style="display: flex; align-items: center; gap: 0.5rem;">
		<span style="color: var(--accent); font-family: 'JetBrains Mono', monospace; font-size: 0.875rem;">07</span>
		Pre-commit Hook
	</h2>
	<p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 0.75rem;">Install once, then every commit gets reviewed automatically.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<span class="syntax-comment"># Install the hook</span><br/>
			<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora hook</span> <span class="syntax-string">install</span><br/><br/>
			<span class="syntax-comment"># Now just commit normally &mdash; cora reviews automatically</span><br/>
			<span class="syntax-cmd">$</span> <span class="syntax-highlight">git</span> <span class="syntax-string">commit</span> <span class="syntax-flag">-m</span> <span class="syntax-string">"fix: handle edge case in parser"</span><br/>
			<span style="color: var(--text-tertiary);">cora pre-commit hook running...</span><br/>
			<span class="syntax-success">No issues found &mdash; commit allowed</span>
		</div>
	</div>
</section>

<!-- SARIF Upload -->
<section class="docs-section scroll-reveal">
	<h2 style="display: flex; align-items: center; gap: 0.5rem;">
		<span style="color: var(--accent); font-family: 'JetBrains Mono', monospace; font-size: 0.875rem;">08</span>
		SARIF Upload
	</h2>
	<p style="color: var(--text-secondary); font-size: 0.875rem; margin-bottom: 0.75rem;">Generate SARIF output and upload to GitHub Code Scanning.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<span class="syntax-comment"># Generate SARIF report and upload</span><br/>
			<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span> <span class="syntax-flag">--output</span> <span class="syntax-string">sarif</span> <span style="color: var(--text-tertiary);">&gt;</span> <span class="syntax-string">results.sarif</span> <span style="color: var(--text-tertiary);">&amp;&amp;</span> \<br/>
			&nbsp;&nbsp;<span class="syntax-highlight">cora upload-sarif</span> <span class="syntax-string">results.sarif</span><br/><br/>
			<span class="syntax-success">Uploaded 3 findings to GitHub Code Scanning</span>
		</div>
	</div>
</section>
