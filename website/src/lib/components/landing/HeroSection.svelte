<script lang="ts">
	import TerminalBlock from './TerminalBlock.svelte';
	import { ArrowRight, Shield, Zap, Eye } from '@lucide/svelte';
</script>

<!-- ====== S1 — HERO ====== -->
<section class="section section-hero relative flex items-center justify-center min-h-[calc(100vh-3.5rem)] overflow-hidden">
	<!-- Animated glow orbs -->
	<div class="hero-glow animate-glow-pulse w-[500px] h-[500px] bg-[var(--accent)] opacity-[0.03] dark:opacity-[0.06] top-[10%] left-[20%]"></div>
	<div class="hero-glow animate-glow-pulse w-[400px] h-[400px] bg-[var(--accent)] opacity-[0.02] dark:opacity-[0.04] bottom-[10%] right-[15%]" style="animation-delay: 2s;"></div>

	<!-- Grid background -->
	<div class="absolute inset-0 grid-bg pointer-events-none"></div>

	<div class="relative z-10 max-w-3xl mx-auto text-center px-4">
		<!-- Badge -->
		<div class="animate-fade-in-up mb-6">
			<span class="accent-badge">
				<span class="badge-dot"></span>
				Open source · BYOK · Zero config
			</span>
		</div>

		<!-- Headline -->
		<h1 class="animate-fade-in-up delay-100 text-4xl sm:text-5xl md:text-6xl font-bold -tracking-tight leading-[1.1]">
			AI code review
			<span class="hero-gradient">in your terminal</span>
		</h1>

		<!-- Subtitle -->
		<p class="animate-fade-in-up delay-200 mt-6 text-lg text-[var(--muted-foreground)] max-w-xl mx-auto leading-relaxed">
			cora catches bugs, security issues, and code smells before they land in your PR.
			Bring your own API key. Your code never leaves your machine.
		</p>

		<!-- CTAs -->
		<div class="animate-fade-in-up delay-300 mt-8 flex flex-col sm:flex-row items-center justify-center gap-3">
			<a href="/docs/getting-started" class="btn-primary group">
				Get Started
				<ArrowRight class="w-4 h-4 transition-transform group-hover:translate-x-1" />
			</a>
			<a href="#demo-terminal" class="btn-ghost">
				See it in action
			</a>
		</div>

		<!-- Quick install -->
		<div class="animate-fade-in-up delay-400 mt-8 max-w-md mx-auto">
			<TerminalBlock title="install">
				<div><span class="syntax-cmd">cargo</span> <span class="syntax-flag">install</span> cora-cli</div>
				<div class="mt-2"><span class="syntax-cmd">cora</span> <span class="syntax-flag">auth login</span></div>
				<div><span class="syntax-cmd">cora</span> <span class="syntax-flag">review</span> <span class="syntax-highlight">--staged</span></div>
			</TerminalBlock>
		</div>

		<!-- Trust signals -->
		<div class="animate-fade-in-up delay-500 mt-10 flex items-center justify-center gap-6 text-xs text-[var(--muted-foreground)]">
			<span class="flex items-center gap-1.5"><Shield class="w-3.5 h-3.5" /> Local-first</span>
			<span class="flex items-center gap-1.5"><Zap class="w-3.5 h-3.5" /> 5 LLM providers</span>
			<span class="flex items-center gap-1.5"><Eye class="w-3.5 h-3.5" /> CI/CD ready</span>
		</div>
	</div>
</section>
