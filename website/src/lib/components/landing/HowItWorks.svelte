<script lang="ts">
	import StepCard from './StepCard.svelte';
	import TerminalBlock from './TerminalBlock.svelte';
	import { ArrowRight, Lightbulb, CheckCircle } from '@lucide/svelte';
</script>

<!-- ====== S4 — HOW IT WORKS ====== -->
<section class="section section-compact">
	<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
		How it works
	</h2>
	<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)] max-w-[32rem] mx-auto">
		Three steps from code to confidence.
	</p>

	<div class="flex flex-col md:flex-row items-stretch mt-10 gap-6">
		<StepCard number="01" icon={ArrowRight} title="Write code" description="Push your changes as normal. cora only sees your diff." />

		<!-- Connector -->
		<div class="connect-line hidden md:flex scroll-reveal">
			<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
		</div>

		<StepCard number="02" icon={Lightbulb} title="Review with AI" description="cora analyzes your diff with the LLM of your choice." delayMs={100} />

		<!-- Connector -->
		<div class="connect-line hidden md:flex scroll-reveal [transition-delay:100ms]">
			<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5l7 7-7 7"/></svg>
		</div>

		<StepCard number="03" icon={CheckCircle} title="Ship with confidence" description="Merge clean, production-ready code. Every time." delayMs={200} />
	</div>
</section>
