<script lang="ts">
	import { Root, Item, Trigger, Content } from '$lib/components/ui/accordion';

	interface FAQItem {
		question: string;
		answer: string;
		value: string;
	}

	const faqList: FAQItem[] = [
		{
			question: 'What does "BYOK" mean?',
			answer: 'Bring Your Own Key — you use your own LLM API key (OpenAI, Anthropic, Groq, Ollama, or Z.AI). cora never charges for AI usage. Your API costs depend on your provider.',
			value: 'byok'
		},
		{
			question: 'Does cora send my code to third parties?',
			answer: 'No. Your code is sent only to the LLM provider you configure. cora itself has no backend, no telemetry, no analytics. Everything runs locally in your terminal.',
			value: 'privacy'
		},
		{
			question: 'Which LLM providers are supported?',
			answer: 'OpenAI (GPT-4o, etc.), Anthropic (Claude), Groq (Llama), Ollama (local models), and Z.AI (GLM). You can also add custom providers with any OpenAI-compatible API.',
			value: 'providers'
		},
		{
			question: 'How is cora different from GitHub Copilot Code Review?',
			answer: 'copa is a CLI tool — it runs in your terminal, not in GitHub. You choose your own LLM provider and model. It works with any git workflow, any hosting platform, and any CI/CD pipeline.',
			value: 'comparison'
		},
		{
			question: 'Can I use cora in CI/CD?',
			answer: 'Yes. cora works in GitHub Actions, GitLab CI, and any CI that supports CLI tools. Use the --staged or --base flags for automated reviews on PRs.',
			value: 'ci'
		},
		{
			question: 'What languages does cora support?',
			answer: 'cora reviews any text-based code. Since it uses LLMs for analysis, it understands virtually all programming languages. The quality of review depends on the model you choose.',
			value: 'languages'
		},
		{
			question: 'Is cora free?',
			answer: 'cora is open source and free to use. You only pay for your own LLM API usage through your provider. With Ollama, you can run it completely free with local models.',
			value: 'pricing'
		},
		{
			question: 'How do I configure review rules?',
			answer: 'Create a .cora.yaml file in your project root. You can configure focus areas, ignore patterns, severity thresholds, and hook settings. See the Configuration docs for details.',
			value: 'config'
		}
	];
</script>

<!-- ====== FAQ ====== -->
<section class="section section-compact">
	<h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">
		Frequently asked questions
	</h2>
	<p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)] max-w-[32rem] mx-auto">
		Everything you need to know about using cora.
	</p>

	<div class="max-w-2xl mx-auto mt-10 scroll-reveal">
		<Root type="single" collapsible>
			{#each faqList as item}
				<Item value={item.value}>
					<Trigger class="text-left text-[var(--foreground)] hover:no-underline">
						{item.question}
					</Trigger>
					<Content class="text-[var(--muted-foreground)]">
						{item.answer}
					</Content>
				</Item>
			{/each}
		</Root>
	</div>
</section>
