<script lang="ts">
	import TerminalBlock from './TerminalBlock.svelte';

	// Terminal typing animation state (kept here since it's section-specific)
	let terminalLines = $state<string[]>([]);
	let terminalComplete = $state(false);
	let demoStarted = $state(false);
	let typeInterval: ReturnType<typeof setInterval> | null = null;

	const terminalOutput = [
		{ text: '$ cora review --staged', color: 'var(--muted-foreground)' },
		{ text: '', color: '' },
		{ text: 'Analyzing 3 files...', color: 'var(--muted-foreground)' },
		{ text: '\u2713 src/auth/login.ts \u2014 2 issues found', color: 'var(--success)' },
		{ text: '  \u26A0 Line 42: Potential SQL injection', color: 'var(--warning)' },
		{ text: '  \u26A0 Line 87: Hardcoded secret', color: 'var(--warning)' },
		{ text: '\u2713 src/utils/parser.ts \u2014 clean', color: 'var(--muted-foreground)' },
		{ text: '\u2713 src/api/routes.ts \u2014 1 issue found', color: 'var(--success)' },
		{ text: '  \u2717 Line 23: Missing error handling', color: 'var(--destructive)' },
		{ text: '', color: '' },
		{ text: '3 issues found in 3 files', color: 'var(--foreground)' },
	];

	let node: HTMLElement | undefined = $state();

	$effect(() => {
		if (!node) return;
		const demoObserver = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting && !demoStarted) {
						demoStarted = true;
						let lineIndex = 0;
						typeInterval = setInterval(() => {
							if (lineIndex < terminalOutput.length) {
								terminalLines = [...terminalLines, terminalOutput[lineIndex].text];
								lineIndex++;
							} else {
								terminalComplete = true;
								if (typeInterval) clearInterval(typeInterval);
							}
						}, 200);
					}
				});
			},
			{ threshold: 0.3 }
		);

		demoObserver.observe(node);
		return () => {
			demoObserver.disconnect();
			if (typeInterval) clearInterval(typeInterval);
		};
	});
</script>

<!-- ====== S3 — LIVE TERMINAL DEMO ====== -->
<section class="section section-tall" id="demo-terminal" bind:this={node}>
	<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
		See it in action
	</h2>
	<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)] max-w-[32rem] mx-auto">
		Run cora against staged changes. Results in seconds, not minutes.
	</p>

	<div class="max-w-2xl mx-auto mt-10 scroll-reveal">
		<TerminalBlock title="cora \u2014 review">
			{#each terminalLines as line, i}
				<div class="min-h-[1.45em]" style="color: {terminalOutput[i]?.color || 'var(--foreground)'};">{line}</div>
			{/each}
			{#if terminalComplete}
				<span class="typing-cursor"></span>
			{/if}
		</TerminalBlock>
	</div>
</section>
