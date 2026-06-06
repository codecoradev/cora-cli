<script lang="ts">
	import { cn } from '$lib/utils.js';
	import type { Snippet } from 'svelte';
	import { Check, Copy } from '@lucide/svelte';

	let {
		title = 'Terminal',
		showCopy = false,
		copyText,
		children,
		class: className,
		...restProps
	}: {
		title?: string;
		showCopy?: boolean;
		copyText?: string;
		children: Snippet;
		class?: string;
	} & Record<string, unknown> = $props();

	let copyClicked = $state(false);

	async function copyToClipboard() {
		if (copyClicked) return;
		try {
			const text = copyText ?? '';
			if (text) {
				await navigator.clipboard.writeText(text);
				copyClicked = true;
				setTimeout(() => { copyClicked = false; }, 2000);
			}
		} catch { /* clipboard not available */ }
	}
</script>

<div class={cn('terminal relative', className)} {...restProps}>
	<div class="terminal-header">
		<span class="terminal-dot terminal-dot-red"></span>
		<span class="terminal-dot terminal-dot-yellow"></span>
		<span class="terminal-dot terminal-dot-green"></span>
		<span class="terminal-title">{title}</span>
	</div>
	<div class="terminal-body relative">
		{@render children()}
		{#if showCopy}
			<button
				class="copy-btn"
				onclick={copyClicked ? undefined : copyToClipboard}
				class:copied={copyClicked}
				aria-label="Copy command"
			>
				{#if copyClicked}
					<Check size={14} />
				{:else}
					<Copy size={14} />
				{/if}
			</button>
		{/if}
	</div>
</div>
