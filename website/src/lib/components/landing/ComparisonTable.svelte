<!-- ====== S6 — COMPARISON TABLE ====== -->
<script lang="ts">
	import { onMount } from 'svelte';

	let revealed = $state(false);
	let sectionEl: HTMLElement;

	onMount(() => {
		const observer = new IntersectionObserver(
			([entry]) => {
				if (entry.isIntersecting) {
					revealed = true;
					observer.disconnect();
				}
			},
			{ threshold: 0.15 }
		);
		if (sectionEl) observer.observe(sectionEl);
		return () => observer.disconnect();
	});

	type CellValue = boolean | string | null;

	interface Feature {
		name: string;
		[key: string]: CellValue;
	}

	const features: Feature[] = [
		{ name: 'BYOK', cora: true, coderabbit: false, copilot: false, sonarqube: null },
		{ name: 'Self-hosted', cora: true, coderabbit: false, copilot: false, sonarqube: true },
		{ name: 'Gitea / Forgejo', cora: true, coderabbit: false, copilot: false, sonarqube: true },
		{ name: 'CLI', cora: true, coderabbit: false, copilot: false, sonarqube: false },
		{ name: 'Pre-commit hooks', cora: true, coderabbit: false, copilot: false, sonarqube: false },
		{ name: 'SARIF output', cora: true, coderabbit: true, copilot: true, sonarqube: true },
		{ name: 'Cost', cora: 'Free + API', coderabbit: '$12–39/mo', copilot: '$10–39/mo', sonarqube: 'Free / $150+' },
		{ name: 'License', cora: 'MIT', coderabbit: 'Apache 2.0', copilot: 'Proprietary', sonarqube: 'LGPL' }
	];

	const competitors = [
		{ key: 'coderabbit', label: 'CodeRabbit' },
		{ key: 'copilot', label: 'Copilot' },
		{ key: 'sonarqube', label: 'SonarQube' }
	];

	function cellIcon(value: CellValue): string {
		if (value === true) return 'check';
		if (value === false) return 'cross';
		if (value === null) return 'dash';
		return 'text';
	}
</script>

<section bind:this={sectionEl} class="section section-compact">
	<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
		Why developers choose cora
	</h2>
	<p class="text-center mt-3 scroll-reveal text-sm text-[var(--muted-foreground)]">
		Compared to popular code review tools.
	</p>

	<!-- ── Desktop table (md+) ── -->
	<div class="hidden md:block scroll-reveal mt-10 max-w-[56rem] mx-auto">
		<div class="compare-glass-card">
			<table class="compare-table w-full">
				<thead>
					<tr>
						<th class="text-left">Feature</th>
						<th class="text-center cora-highlight-col">
							<span class="cora-badge">cora</span>
						</th>
						<th class="text-center">CodeRabbit</th>
						<th class="text-center">Copilot</th>
						<th class="text-center">SonarQube</th>
					</tr>
				</thead>
				<tbody>
					{#each features as feat, i}
						<tr class="compare-row" class:revealed={revealed} style="transition-delay: {i * 60}ms">
							<td class="font-medium text-[var(--foreground)]">{feat.name}</td>
							<td class="text-center cora-highlight-col">
								{#if cellIcon(feat.cora) === 'check'}
									<span class="check-badge">✓</span>
								{:else}
									<span class="font-semibold text-[var(--accent)]">{feat.cora}</span>
								{/if}
							</td>
							<td class="text-center">
								{#if cellIcon(feat[competitors[0].key]) === 'check'}
									<span class="check-muted">✓</span>
								{:else if cellIcon(feat[competitors[0].key]) === 'cross'}
									<span class="cross-badge">✗</span>
								{:else if cellIcon(feat[competitors[0].key]) === 'dash'}
									<span class="dash-muted">—</span>
								{:else}
									<span class="text-sm text-[var(--muted-foreground)]">{feat[competitors[0].key]}</span>
								{/if}
							</td>
							<td class="text-center">
								{#if cellIcon(feat[competitors[1].key]) === 'check'}
									<span class="check-muted">✓</span>
								{:else if cellIcon(feat[competitors[1].key]) === 'cross'}
									<span class="cross-badge">✗</span>
								{:else if cellIcon(feat[competitors[1].key]) === 'dash'}
									<span class="dash-muted">—</span>
								{:else}
									<span class="text-sm text-[var(--muted-foreground)]">{feat[competitors[1].key]}</span>
								{/if}
							</td>
							<td class="text-center">
								{#if cellIcon(feat[competitors[2].key]) === 'check'}
									<span class="check-muted">✓</span>
								{:else if cellIcon(feat[competitors[2].key]) === 'cross'}
									<span class="cross-badge">✗</span>
								{:else if cellIcon(feat[competitors[2].key]) === 'dash'}
									<span class="dash-muted">—</span>
								{:else}
									<span class="text-sm text-[var(--muted-foreground)]">{feat[competitors[2].key]}</span>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>

	<!-- ── Mobile card layout ── -->
	<div class="md:hidden mt-8 space-y-3 scroll-reveal">
		{#each features as feat, i}
			<div class="compare-card" class:revealed={revealed} style="transition-delay: {i * 60}ms">
				<div class="compare-card-header">
					<span class="font-semibold text-[var(--foreground)] text-sm">{feat.name}</span>
				</div>
				<div class="compare-card-body">
					<!-- cora -->
					<div class="compare-card-cora">
						<span class="text-xs font-bold uppercase tracking-wider text-[var(--accent)] opacity-70">cora</span>
						{#if cellIcon(feat.cora) === 'check'}
							<span class="check-badge">✓</span>
						{:else}
							<span class="font-semibold text-[var(--accent)] text-sm">{feat.cora}</span>
						{/if}
					</div>
					<!-- competitors -->
					{#each competitors as comp}
						<div class="compare-card-comp">
							<span class="text-xs uppercase tracking-wider text-[var(--muted-foreground)] opacity-70">{comp.label}</span>
							{#if cellIcon(feat[comp.key]) === 'check'}
								<span class="check-muted text-sm">✓</span>
							{:else if cellIcon(feat[comp.key]) === 'cross'}
								<span class="cross-badge text-sm">✗</span>
							{:else if cellIcon(feat[comp.key]) === 'dash'}
								<span class="dash-muted text-sm">—</span>
							{:else}
								<span class="text-sm text-[var(--muted-foreground)]">{feat[comp.key]}</span>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</section>

<style>
	/* ── Desktop table ── */
	.compare-glass-card {
		border-radius: 16px;
		border: 1px solid var(--border);
		background: oklch(0.99 0.002 270 / 0.6);
		backdrop-filter: blur(8px);
		overflow: hidden;
		box-shadow: 0 4px 24px -4px oklch(0.4 0.02 270 / 0.08);
	}

	:global(.dark) .compare-glass-card {
		background: oklch(0.16 0.015 270 / 0.6);
		box-shadow: 0 4px 24px -4px oklch(0.1 0.02 270 / 0.2);
	}

	.compare-table {
		border-collapse: collapse;
	}

	.compare-table th,
	.compare-table td {
		padding: 14px 16px;
		font-size: 14px;
		border-bottom: 1px solid var(--border);
		white-space: nowrap;
	}

	.compare-table thead th {
		font-size: 13px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--muted-foreground);
		padding: 16px;
		border-bottom: 2px solid var(--border);
	}

	.compare-table tbody tr:last-child td {
		border-bottom: none;
	}

	.cora-highlight-col {
		background: oklch(0.55 0.22 270 / 0.05);
	}

	:global(.dark) .cora-highlight-col {
		background: oklch(0.65 0.22 270 / 0.08);
	}

	.cora-badge {
		display: inline-flex;
		align-items: center;
		padding: 2px 10px;
		border-radius: 6px;
		font-weight: 700;
		font-size: 13px;
		background: oklch(0.55 0.22 270 / 0.12);
		color: oklch(0.55 0.22 270);
	}

	:global(.dark) .cora-badge {
		background: oklch(0.65 0.22 270 / 0.18);
		color: oklch(0.75 0.2 270);
	}

	/* ── Badges ── */
	.check-badge {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 26px;
		height: 26px;
		border-radius: 8px;
		font-weight: 700;
		font-size: 14px;
		background: oklch(0.6 0.18 150 / 0.12);
		color: oklch(0.5 0.18 150);
	}

	:global(.dark) .check-badge {
		background: oklch(0.6 0.18 150 / 0.18);
		color: oklch(0.7 0.15 150);
	}

	.check-muted {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 26px;
		height: 26px;
		border-radius: 8px;
		font-size: 14px;
		color: var(--muted-foreground);
	}

	.cross-badge {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 26px;
		height: 26px;
		border-radius: 8px;
		font-weight: 600;
		font-size: 14px;
		color: oklch(0.55 0.15 25);
		background: oklch(0.55 0.15 25 / 0.08);
	}

	:global(.dark) .cross-badge {
		color: oklch(0.6 0.15 25);
		background: oklch(0.6 0.15 25 / 0.12);
	}

	.dash-muted {
		color: var(--muted-foreground);
		opacity: 0.5;
	}

	/* ── Desktop row animation ── */
	.compare-row {
		opacity: 0;
		transform: translateY(8px);
		transition: opacity 0.4s ease-out, transform 0.4s ease-out;
	}

	.compare-row.revealed {
		opacity: 1;
		transform: translateY(0);
	}

	/* ── Mobile card layout ── */
	.compare-card {
		border-radius: 12px;
		border: 1px solid var(--border);
		background: oklch(0.99 0.002 270 / 0.6);
		overflow: hidden;
		opacity: 0;
		transform: translateY(8px);
		transition: opacity 0.4s ease-out, transform 0.4s ease-out;
	}

	.compare-card.revealed {
		opacity: 1;
		transform: translateY(0);
	}

	:global(.dark) .compare-card {
		background: oklch(0.16 0.015 270 / 0.6);
	}

	.compare-card-header {
		padding: 10px 14px;
		border-bottom: 1px solid var(--border);
		background: oklch(0.96 0.003 270);
	}

	:global(.dark) .compare-card-header {
		background: oklch(0.14 0.012 270);
	}

	.compare-card-body {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr 1fr;
		gap: 0;
	}

	.compare-card-cora {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
		padding: 10px 8px;
		background: oklch(0.55 0.22 270 / 0.05);
		border-right: 1px solid var(--border);
	}

	:global(.dark) .compare-card-cora {
		background: oklch(0.65 0.22 270 / 0.08);
	}

	.compare-card-comp {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
		padding: 10px 8px;
		border-right: 1px solid var(--border);
	}

	.compare-card-comp:last-child {
		border-right: none;
	}
</style>
