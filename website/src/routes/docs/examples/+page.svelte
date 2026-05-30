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
	<title>Examples — cora docs</title>
	<meta name="description" content="Practical examples for using cora - quick review, CI, pre-commit hooks, SARIF, and more." />
</svelte:head>

<div class="docs-content">
<h1 class="scroll-reveal">Examples</h1>
<p class="scroll-reveal">Practical examples to get you started with cora.</p>

<!-- Quick Review -->
<section class="docs-section scroll-reveal">
	<h2 class="flex items-center gap-2">
		<span class="text-[var(--accent)] font-mono text-sm">01</span>
		Quick Review
	</h2>
	<p class="text-sm text-[var(--muted-foreground)] mb-3">Review your staged changes before committing.</p>
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
	<h2 class="flex items-center gap-2">
		<span class="text-[var(--accent)] font-mono text-sm">02</span>
		Branch Comparison
	</h2>
	<p class="text-sm text-[var(--muted-foreground)] mb-3">Compare your current branch against main.</p>
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
	<h2 class="flex items-center gap-2">
		<span class="text-[var(--accent)] font-mono text-sm">03</span>
		Full Project Scan
	</h2>
	<p class="text-sm text-[var(--muted-foreground)] mb-3">Scan your entire project for issues.</p>
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
	<h2 class="flex items-center gap-2">
		<span class="text-[var(--accent)] font-mono text-sm">04</span>
		Incremental Scan
	</h2>
	<p class="text-sm text-[var(--muted-foreground)] mb-3">Only scan files that changed since the last scan.</p>
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
	<h2 class="flex items-center gap-2">
		<span class="text-[var(--accent)] font-mono text-sm">05</span>
		Streaming Output
	</h2>
	<p class="text-sm text-[var(--muted-foreground)] mb-3">Stream results as they come in from the LLM.</p>
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
	<h2 class="flex items-center gap-2">
		<span class="text-[var(--accent)] font-mono text-sm">06</span>
		GitHub Actions CI
	</h2>
	<p class="text-sm text-[var(--muted-foreground)] mb-3">Add cora to your CI pipeline.</p>
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
      <span class="syntax-cmd">- uses:</span> <span class="syntax-string">actions/checkout@v4</span>
      <span class="syntax-cmd">- name:</span> <span class="syntax-string">Install cora</span>
        <span class="syntax-flag">run:</span> <span class="syntax-string">cargo install cora</span>
      <span class="syntax-cmd">- name:</span> <span class="syntax-string">Run AI code review</span>
        <span class="syntax-flag">env:</span>
          {@html '<span class="syntax-flag">CORA_API_KEY:</span> <span class="syntax-string">${{ secrets.CORA_API_KEY }}</span>'}
          <span class="syntax-flag">CORA_PROVIDER:</span> <span class="syntax-string">openai</span>
        <span class="syntax-flag">run:</span> <span class="syntax-string">cora review --branch main --fail-on error</span></pre>
		</div>
	</div>
</section>

<!-- Pre-commit Hook -->
<section class="docs-section scroll-reveal">
	<h2 class="flex items-center gap-2">
		<span class="text-[var(--accent)] font-mono text-sm">07</span>
		Pre-commit Hook
	</h2>
	<p class="text-sm text-[var(--muted-foreground)] mb-3">Install once, then every commit gets reviewed automatically.</p>
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
			<span class="syntax-cmd">cora pre-commit hook running...</span><br/>
			<span class="syntax-success">No issues found &mdash; commit allowed</span>
		</div>
	</div>
</section>

<!-- SARIF Upload -->
<section class="docs-section scroll-reveal">
	<h2 class="flex items-center gap-2">
		<span class="text-[var(--accent)] font-mono text-sm">08</span>
		SARIF Upload
	</h2>
	<p class="text-sm text-[var(--muted-foreground)] mb-3">Generate SARIF output and upload to GitHub Code Scanning.</p>
	<div class="docs-terminal">
		<div class="terminal-bar">
			<span class="terminal-dot terminal-dot-red"></span>
			<span class="terminal-dot terminal-dot-yellow"></span>
			<span class="terminal-dot terminal-dot-green"></span>
		</div>
		<div class="terminal-body">
			<span class="syntax-comment"># Generate SARIF report and upload</span><br/>
			<span class="syntax-cmd">$</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span> <span class="syntax-flag">--output</span> <span class="syntax-string">sarif</span> <span class="syntax-cmd">&gt;</span> <span class="syntax-string">results.sarif</span> <span class="syntax-cmd">&amp;&amp;</span> \<br/>
			&nbsp;&nbsp;<span class="syntax-highlight">cora upload-sarif</span> <span class="syntax-string">results.sarif</span><br/><br/>
			<span class="syntax-success">Uploaded 3 findings to GitHub Code Scanning</span>
		</div>
	</div>
</section>
</div>
