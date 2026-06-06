<script lang="ts">
	import { Accordion } from "bits-ui";
	import { cn } from "$lib/utils.js";

	let { ref = $bindable(null), class: className = "", children, ...rest } = $props();
</script>

<Accordion.Item
	bind:ref
	class={cn("border-b border-[var(--border)]", className)}
	{...rest}
>
	{@render children()}
</Accordion.Item>
