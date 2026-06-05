<script lang="ts">
	import TerminalBlock from './TerminalBlock.svelte';
	import { ArrowRight } from '@lucide/svelte';
</script>

<!-- ====== S1 — HERO ====== -->
<section class="section section-hero relative flex items-center justify-center min-h-[calc(100vh-3.5rem)]">
	<!-- Subtle radial gradient glow -->
	<div class="absolute inset-0 pointer-events-none" style="background: radial-gradient(ellipse 50% 40% at 50% 40%, oklch(0.65 0.22 270 / 0.04), transparent);"></div>

	<div class="relative z-10 max-w-3xl mx-auto text-center">
		<!-- Badge -->
		<div class="animate-fade-in-up">
			<span class="accent-badge">
				<span class="badge-dot"></span>
				AI Code Review for Developers
			</span>
		</div>

		<!-- Headline -->
		<h1 class="mt-8 mb-0 animate-fade-in-up delay-100 text-4xl sm:text-5xl md:text-6xl lg:text-7xl font-bold text-[var(--foreground)] leading-none -tracking-tighter">
			Review code.<br />
			<span class="bg-gradient-to-br from-[oklch(0.65_0.22_270)] to-[oklch(0.7_0.15_240)] bg-clip-text text-transparent">Ship faster.</span>
		</h1>

		<!-- Subtitle -->
		<p class="max-w-xl mx-auto mt-6 animate-fade-in-up delay-200 text-base sm:text-lg text-[var(--muted-foreground)] leading-normal sm:leading-relaxed -tracking-tight">
			cora catches bugs, security issues, and style violations before they merge. CLI-first. BYOK. Runs in your terminal.
		</p>

		<!-- Install Terminal -->
		<div class="max-w-lg w-full mx-auto mt-10 animate-fade-in-up delay-300">
			<TerminalBlock title="Terminal" showCopy copyText="curl -fsSL https://cora.dev/install | sh">
				<span class="syntax-cmd">$</span>
				<span class="syntax-highlight"> curl -fsSL</span> <span class="syntax-string">https://cora.dev/install</span> <span class="syntax-highlight">|</span> <span class="syntax-string">sh</span>
				<span class="typing-cursor"></span>
			</TerminalBlock>
		</div>

		<!-- CTAs -->
		<div class="flex flex-wrap justify-center gap-4 items-center mt-8 animate-fade-in-up delay-400">
			<a href="#quick-start" class="btn-primary">
				<ArrowRight size={18} />
				Get Started
			</a>
			<a href="https://github.com/codecoradev/cora-cli" target="_blank" rel="noopener" class="btn-ghost">
				<svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>
				View on GitHub
			</a>
		</div>

		<!-- Bottom text -->
		<p class="mt-6 animate-fade-in-up delay-500 text-xs text-[var(--muted-foreground)] tracking-wide">
			MIT License &middot; No account &middot; OpenAI &middot; Anthropic &middot; Groq &middot; Ollama
		</p>
	</div>
</section>
