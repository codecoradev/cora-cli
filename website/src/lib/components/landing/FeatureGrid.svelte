<script lang="ts">
import FeatureCard from './FeatureCard.svelte';
import {
	ScanSearch,
	KeyRound,
	GitBranch,
	Layers,
	ShieldCheck,
	Lock,
	Lightbulb,
	Upload,
	FileCheck
} from '@lucide/svelte';
</script>

<!-- ====== S5 — FEATURES ====== -->
<section class="section section-tall">
	<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
		Built for developers who value control
	</h2>
	<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)]">
		Everything you need, nothing you don't.
	</p>

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-10">
		<FeatureCard
			icon={ScanSearch}
			title="AI Code Review"
			accent="Diff, branch, or full scan"
			description="Three review modes: staged diff, branch comparison, or full project scan. LLM-powered analysis catches bugs, security issues, and style violations."
		/>

		<FeatureCard
			icon={KeyRound}
			title="Bring Your Own Key"
			accent="No subscriptions, no lock-in"
			description="Uses YOUR OpenAI, Anthropic, Groq, Ollama, or Z.AI API key. No data stored on our servers. You control the model, you control the cost."
			delayMs={100}
		/>

		<FeatureCard
			icon={GitBranch}
			title="Pre-commit Hooks"
			accent="Review before you push"
			description="Install once. Every commit gets reviewed automatically. Block bad code from entering your branch before it ships."
			delayMs={200}
		/>

		<FeatureCard
			icon={Layers}
			title="Incremental Scan"
			accent="Only scan what changed"
			description="SHA256 content hash cache. First scan indexes your codebase. Subsequent scans only review new or modified files."
			delayMs={300}
		/>

		<FeatureCard
			icon={ShieldCheck}
			title="SARIF Output"
			accent="GitHub Code Scanning"
			description="Upload review findings directly to GitHub's Security tab. Track issues across PRs. Works with any CI/CD pipeline."
			delayMs={400}
		/>

		<FeatureCard
			icon={Lock}
			title="Fully Private"
			accent="Your code stays yours"
			description="Runs entirely on your machine. No cloud, no telemetry, no data leaving your network. Perfect for Gitea and air-gapped environments."
			delayMs={500}
		/>

		<FeatureCard
			icon={Lightbulb}
			title="Deterministic Reviews"
			accent="Same diff, same issues"
			description="Temperature defaults to 0. Identical diffs always produce identical findings — perfect for CI reproducibility."
			delayMs={600}
		/>

		<FeatureCard
			icon={Upload}
			title="Smart Caching"
			accent="Never review the same diff twice"
			description="Reviews are cached by diff hash in <code class=&quot;text-[var(--accent)]&quot;>~/.cache/cora/reviews/</code>. Re-reviewing an unchanged diff returns cached results instantly. Use <code class=&quot;text-[var(--accent)]&quot;>--no-cache</code> to bypass."
			delayMs={700}
		/>

		<FeatureCard
			icon={FileCheck}
			title="Custom Prompts &amp; Anti-Hallucination"
			accent="Control the review, trust the output"
			description="Override system prompts for review and scan. File path injection and post-parse filtering ensure the LLM only reports issues that exist in your actual diff."
			delayMs={800}
		/>
	</div>
</section>
