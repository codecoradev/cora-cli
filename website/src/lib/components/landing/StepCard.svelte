<script lang="ts">
	import { cn } from '$lib/utils.js';

	let {
		number,
		icon,
		title,
		description,
		class: className,
		delayMs = 0,
	}: {
		number: string;
		icon: typeof import('lucide-svelte').default;
		title: string;
		description: string;
		class?: string;
		delayMs?: number;
	} = $props();
</script>

<div class={cn('glass-card flex-1 text-center scroll-reveal', className)} style:transition-delay={delayMs ? `${delayMs}ms` : undefined}>
	<div class="text-2xl font-bold text-[var(--accent)] -tracking-tight font-mono opacity-50">{number}</div>
	<div class="flex justify-center mt-4">
		<icon size={24} stroke="var(--accent)" fill="none"></icon>
	</div>
	<h3 class="mt-4 text-xl font-semibold text-[var(--foreground)] -tracking-tight leading-snug">{title}</h3>
	<p class="mt-2 text-sm text-[var(--muted-foreground)]">{description}</p>
</div>
