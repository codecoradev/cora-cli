<script lang="ts">
	import { cn } from '$lib/utils.js';

	let {
		icon,
		title,
		accent,
		description,
		class: className,
		delayMs = 0,
	}: {
		icon: typeof import('@lucide/svelte').Icon;
		title: string;
		accent: string;
		description: string;
		class?: string;
		delayMs?: number;
	} = $props();
</script>

<div class={cn('glass-card scroll-reveal', className)} style:transition-delay={delayMs ? `${delayMs}ms` : undefined}>
	<div class="feature-icon">
		<icon size={24} stroke="currentColor" fill="none"></icon>
	</div>
	<h3 class="mt-4 text-lg font-semibold text-[var(--foreground)]">{title}</h3>
	<p class="mt-2 text-sm text-[var(--accent)]">{accent}</p>
	<p class="mt-2 text-sm text-[var(--muted-foreground)] leading-relaxed">{@html description}</p>
</div>
