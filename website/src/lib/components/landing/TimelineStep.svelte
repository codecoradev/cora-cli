<script lang="ts">
	import { cn } from '$lib/utils.js';
	import type { Snippet } from 'svelte';

	let {
		number,
		title,
		description,
		terminalCode,
		descriptionClass = 'text-[var(--muted-foreground)]',
		class: className,
		delayMs = 0,
	}: {
		number: number;
		title: string;
		description: string;
		terminalCode?: Snippet;
		descriptionClass?: string;
		class?: string;
		delayMs?: number;
	} = $props();
</script>

<div class={cn('timeline-step scroll-reveal', className)} style:transition-delay={delayMs ? `${delayMs}ms` : undefined}>
	<div class="timeline-number">{number}</div>
	<div>
		<h3 class="text-lg font-semibold text-[var(--foreground)] -tracking-tight leading-snug">{title}</h3>
		<p class={cn('mt-1 mb-4 text-sm', descriptionClass)}>{description}</p>
		{#if terminalCode}
			{@render terminalCode()}
		{/if}
	</div>
</div>
