<script lang="ts">
	import { Accordion } from "bits-ui";

	let { children, ...rest }: any = $props();
</script>

<Accordion.Root
	class="w-full"
	{...rest}
>
	{@render children()}
</Accordion.Root>
