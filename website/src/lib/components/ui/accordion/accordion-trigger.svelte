<script lang="ts">
	import { Accordion } from "bits-ui";
	import { ChevronDown } from "@lucide/svelte";
	import { cn } from "$lib/utils.js";

	let { ref = $bindable(null), class: className = "", children, ...rest } = $props();
</script>

<Accordion.Header class="flex" {...rest}>
	<Accordion.Trigger
		bind:ref
		class={cn(
			"flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline text-[var(--foreground)] [&[data-state=open]>svg]:rotate-180",
			className
		)}
	>
		{@render children()}
		<ChevronDown class="h-4 w-4 shrink-0 transition-transform duration-200" />
	</Accordion.Trigger>
</Accordion.Header>
