<script lang="ts">
	import { onMount } from 'svelte';

	import HeroSection from '$lib/components/landing/HeroSection.svelte';
	import KpiStats from '$lib/components/landing/KpiStats.svelte';
	import LiveDemo from '$lib/components/landing/LiveDemo.svelte';
	import HowItWorks from '$lib/components/landing/HowItWorks.svelte';
	import FeatureGrid from '$lib/components/landing/FeatureGrid.svelte';
	import ComparisonTable from '$lib/components/landing/ComparisonTable.svelte';
	import QuickStart from '$lib/components/landing/QuickStart.svelte';

	onMount(() => {
		// Scroll reveal observer — applies 'revealed' class on intersection
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						entry.target.classList.add('revealed');
						observer.unobserve(entry.target);
					}
				});
			},
			{ threshold: 0.1 }
		);

	document.querySelectorAll('.scroll-reveal').forEach((el) => observer.observe(el));

	// Comparison table rows — stagger reveal
	document.querySelectorAll('.compare-table tbody tr').forEach((tr, i) => {
		(tr as HTMLElement).style.transitionDelay = `${i * 60}ms`;
		observer.observe(tr);
	});

	// Timeline numbers — pulse on reveal
	document.querySelectorAll('.timeline-number').forEach((el) => observer.observe(el));

	return () => {
		observer.disconnect();
	};
	});
</script>

<svelte:head>
	<title>cora — AI Code Review CLI</title>
	<meta name="description" content="cora is a CLI-first AI code reviewer. BYOK, zero config, runs in your terminal. Your code never leaves your machine." />
</svelte:head>

<div class="bg-[var(--background)]">
	<HeroSection />
	<KpiStats />
	<LiveDemo />
	<HowItWorks />
	<FeatureGrid />
	<ComparisonTable />
	<QuickStart />
</div>
