<script lang="ts">
	import TimelineStep from './TimelineStep.svelte';
	import TerminalBlock from './TerminalBlock.svelte';
	import { ArrowRight, Star } from '@lucide/svelte';
</script>

<!-- ====== S7 — QUICK START + CTA + FOOTER ====== -->
<section class="section section-tall" id="quick-start">
	<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
		Start in 30 seconds
	</h2>
	<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)]">
		No account required. No subscription. No cloud.
	</p>

	<div class="flex flex-col mt-10 max-w-[40rem] mx-auto gap-6">
		<!-- Step 1 -->
		<TimelineStep number={1} title="Install" description="Single binary, no dependencies.">
			{#snippet terminalCode()}
				<TerminalBlock>
					<span class="syntax-cmd">$</span> <span class="syntax-highlight">curl -fsSL</span> <span class="syntax-string">https://cora.dev/install</span> <span class="syntax-highlight">|</span> <span class="syntax-string">sh</span>
				</TerminalBlock>
			{/snippet}
		</TimelineStep>

		<!-- Step 2 -->
		<TimelineStep number={2} title="Set API key" description="Use your existing OpenAI or Anthropic key." delayMs={100}>
			{#snippet terminalCode()}
				<TerminalBlock>
					<span class="syntax-cmd">$</span> <span class="syntax-highlight">export</span> <span class="syntax-flag">OPENAI_API_KEY</span>=<span class="syntax-string">"sk-..."</span>
				</TerminalBlock>
			{/snippet}
		</TimelineStep>

		<!-- Step 3 -->
		<TimelineStep number={3} title="Review" description="Review your staged changes." delayMs={200}>
			{#snippet terminalCode()}
				<TerminalBlock>
					<span class="syntax-cmd">$</span> <span class="syntax-flag">CORA_API_KEY</span>=<span class="syntax-string">key</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span>
				</TerminalBlock>
			{/snippet}
		</TimelineStep>

		<!-- Step 4 -->
		<TimelineStep number={4} title="Done" description="That's it. No account. No subscription." descriptionClass="text-[var(--success)]" delayMs={300} />
	</div>

	<!-- CTA -->
	<div class="text-center mt-20 scroll-reveal">
		<h2 class="text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
			Ready to ship better code?
		</h2>
		<p class="mt-3 text-sm text-[var(--muted-foreground)]">
			No account. No subscription. No cloud.
		</p>
		<div class="flex flex-wrap justify-center gap-4 items-center mt-8">
			<a href="/docs" class="btn-primary">
				<ArrowRight size={18} />
				Get Started
			</a>
			<a href="https://github.com/codecoradev/cora-cli" target="_blank" rel="noopener" class="btn-ghost">
				<Star size={18} />
				Star on GitHub
			</a>
		</div>
	</div>

	<!-- Footer -->
	<footer class="border-t border-[var(--border)] mt-20 py-8">
		<div class="max-w-6xl mx-auto flex flex-col md:flex-row items-center justify-between gap-4">
			<div>
				<span class="text-sm font-semibold text-[var(--foreground)]">cora</span>
				<span class="text-xs text-[var(--muted-foreground)] ml-3">MIT License</span>
			</div>
			<div class="flex items-center gap-6">
				<a href="https://github.com/codecoradev/cora-cli" target="_blank" rel="noopener" class="text-sm text-[var(--muted-foreground)] no-underline transition-colors duration-200 min-h-11 inline-flex items-center hover:text-[var(--foreground)]">GitHub</a>
				<a href="/docs" class="text-sm text-[var(--muted-foreground)] no-underline transition-colors duration-200 min-h-11 inline-flex items-center hover:text-[var(--foreground)]">Docs</a>
			</div>
		</div>
	</footer>
</section>
