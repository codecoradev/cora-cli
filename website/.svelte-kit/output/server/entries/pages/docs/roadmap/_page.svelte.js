import { $ as attr, c as ensure_array_like, l as head, m as stringify, tt as escape_html } from "../../../../chunks/dev.js";
//#region src/routes/docs/roadmap/+page.svelte
function _page($$renderer) {
	head("5gq709", $$renderer, ($$renderer) => {
		$$renderer.title(($$renderer) => {
			$$renderer.push(`<title>Roadmap — cora Docs</title>`);
		});
	});
	$$renderer.push(`<h1 class="text-3xl font-bold mb-6">Roadmap</h1> <p class="text-[var(--muted-foreground)] mb-10">Demand-gated — we build what people actually need. Track progress on <a href="https://github.com/codecoradev/cora-cli/issues" target="_blank" rel="noopener" class="text-[var(--accent)] hover:underline">GitHub Issues</a>.</p> <div class="space-y-10"><section><h2 class="text-xl font-semibold mb-4 flex items-center gap-2"><span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.1.5</span> Initial Release</h2> <div class="space-y-3"><!--[-->`);
	const each_array = ensure_array_like([
		{
			title: "Basic diff review with OpenAI",
			issue: 90
		},
		{
			title: "JSON response repair & unicode handling",
			issue: 89
		},
		{
			title: "CLI interface with review command",
			issue: 90
		}
	]);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		$$renderer.push(`<a${attr("href", `https://github.com/codecoradev/cora-cli/issues/${stringify(item.issue)}`)} target="_blank" rel="noopener" class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"><div class="flex items-center gap-3"><span class="text-xs text-[var(--muted-foreground)] font-mono">#${escape_html(item.issue)}</span> <span class="text-sm">${escape_html(item.title)}</span></div> <span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span></a>`);
	}
	$$renderer.push(`<!--]--></div></section> <section><h2 class="text-xl font-semibold mb-4 flex items-center gap-2"><span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.1.6</span> Custom Prompts &amp; Path Injection</h2> <div class="space-y-3"><!--[-->`);
	const each_array_1 = ensure_array_like([
		{
			title: "Enhanced default system prompts",
			issue: 95
		},
		{
			title: "Custom system prompt via .cora.yaml config",
			issue: 94
		},
		{
			title: "Inject valid file paths into system prompt",
			issue: 93
		},
		{
			title: "JSON object response format (opt-in)",
			issue: 92
		}
	]);
	for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
		let item = each_array_1[$$index_1];
		$$renderer.push(`<a${attr("href", `https://github.com/codecoradev/cora-cli/issues/${stringify(item.issue)}`)} target="_blank" rel="noopener" class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"><div class="flex items-center gap-3"><span class="text-xs text-[var(--muted-foreground)] font-mono">#${escape_html(item.issue)}</span> <span class="text-sm">${escape_html(item.title)}</span></div> <span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span></a>`);
	}
	$$renderer.push(`<!--]--></div></section> <section><h2 class="text-xl font-semibold mb-4 flex items-center gap-2"><span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.1.7</span> Deterministic &amp; Reliable</h2> <div class="space-y-3"><!--[-->`);
	const each_array_2 = ensure_array_like([
		{
			title: "Deterministic reviews — temperature=0",
			issue: 98
		},
		{
			title: "Non-deterministic output bug fix",
			issue: 97
		},
		{
			title: "HTTP timeout + connection pooling",
			issue: 99
		},
		{
			title: "Diff-hash caching for repeat reviews",
			issue: 100
		},
		{
			title: "Configurable max_tokens",
			issue: 101
		}
	]);
	for (let $$index_2 = 0, $$length = each_array_2.length; $$index_2 < $$length; $$index_2++) {
		let item = each_array_2[$$index_2];
		$$renderer.push(`<a${attr("href", `https://github.com/codecoradev/cora-cli/issues/${stringify(item.issue)}`)} target="_blank" rel="noopener" class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"><div class="flex items-center gap-3"><span class="text-xs text-[var(--muted-foreground)] font-mono">#${escape_html(item.issue)}</span> <span class="text-sm">${escape_html(item.title)}</span></div> <span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span></a>`);
	}
	$$renderer.push(`<!--]--></div></section> <section><h2 class="text-xl font-semibold mb-4 flex items-center gap-2"><span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.2.0</span> Multi-Provider &amp; SARIF</h2> <div class="space-y-3"><!--[-->`);
	const each_array_3 = ensure_array_like([
		{
			title: "BYOK — Anthropic, Groq, Ollama support",
			issue: 106
		},
		{
			title: "SARIF output format",
			issue: 106
		},
		{
			title: "Branch review mode",
			issue: 106
		},
		{
			title: "Output footer watermark",
			issue: 106
		}
	]);
	for (let $$index_3 = 0, $$length = each_array_3.length; $$index_3 < $$length; $$index_3++) {
		let item = each_array_3[$$index_3];
		$$renderer.push(`<a${attr("href", `https://github.com/codecoradev/cora-cli/issues/${stringify(item.issue)}`)} target="_blank" rel="noopener" class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"><div class="flex items-center gap-3"><span class="text-xs text-[var(--muted-foreground)] font-mono">#${escape_html(item.issue)}</span> <span class="text-sm">${escape_html(item.title)}</span></div> <span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span></a>`);
	}
	$$renderer.push(`<!--]--></div></section> <section><h2 class="text-xl font-semibold mb-4 flex items-center gap-2"><span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.3</span> Progress &amp; CI Hardening</h2> <div class="space-y-3"><!--[-->`);
	const each_array_4 = ensure_array_like([
		{
			title: "Static analysis context injection (reduce false positives)",
			issue: 140
		},
		{
			title: "--progress flag for machine-readable output",
			issue: 108
		},
		{
			title: "Composite action crash fix (KeyError)",
			issue: 102
		},
		{
			title: "Config validate command",
			issue: 88
		}
	]);
	for (let $$index_4 = 0, $$length = each_array_4.length; $$index_4 < $$length; $$index_4++) {
		let item = each_array_4[$$index_4];
		$$renderer.push(`<a${attr("href", `https://github.com/codecoradev/cora-cli/issues/${stringify(item.issue)}`)} target="_blank" rel="noopener" class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"><div class="flex items-center gap-3"><span class="text-xs text-[var(--muted-foreground)] font-mono">#${escape_html(item.issue)}</span> <span class="text-sm">${escape_html(item.title)}</span></div> <span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span></a>`);
	}
	$$renderer.push(`<!--]--></div></section> <section><h2 class="text-xl font-semibold mb-4 flex items-center gap-2"><span class="px-2 py-0.5 rounded text-xs font-semibold bg-green-500/20 text-green-400">v0.4</span> Deterministic Engine Pipeline</h2> <div class="space-y-3"><!--[-->`);
	const each_array_5 = ensure_array_like([
		{
			title: "Deterministic rule engine — 12 built-in rules",
			issue: 116
		},
		{
			title: "File bundling — parallel per-bundle review",
			issue: 115
		},
		{
			title: "AST-based cross-file dependency extraction",
			issue: 114
		},
		{
			title: "Hunk header regex panic fix + 5MB diff support",
			issue: 159
		}
	]);
	for (let $$index_5 = 0, $$length = each_array_5.length; $$index_5 < $$length; $$index_5++) {
		let item = each_array_5[$$index_5];
		$$renderer.push(`<a${attr("href", `https://github.com/codecoradev/cora-cli/issues/${stringify(item.issue)}`)} target="_blank" rel="noopener" class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"><div class="flex items-center gap-3"><span class="text-xs text-[var(--muted-foreground)] font-mono">#${escape_html(item.issue)}</span> <span class="text-sm">${escape_html(item.title)}</span></div> <span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span></a>`);
	}
	$$renderer.push(`<!--]--></div></section> <section><h2 class="text-xl font-semibold mb-4 flex items-center gap-2"><span class="px-2 py-0.5 rounded text-xs font-semibold bg-blue-500/20 text-blue-400">v0.5</span> Install &amp; Distribution</h2> <div class="space-y-3"><!--[-->`);
	const each_array_6 = ensure_array_like([
		{
			title: "Easy install — Homebrew tap & install script",
			issue: 151,
			status: "done"
		},
		{
			title: "CI gate mode — block PRs on review findings",
			issue: 149,
			status: "done"
		},
		{
			title: "Website redesign — landing page + docs",
			issue: 163,
			status: "done"
		},
		{
			title: "Interactive auth login with tiered provider selection",
			issue: 172,
			status: "done"
		},
		{
			title: "README overhaul — market-facing copy",
			issue: 162,
			status: "planned"
		}
	]);
	for (let $$index_6 = 0, $$length = each_array_6.length; $$index_6 < $$length; $$index_6++) {
		let item = each_array_6[$$index_6];
		$$renderer.push(`<a${attr("href", `https://github.com/codecoradev/cora-cli/issues/${stringify(item.issue)}`)} target="_blank" rel="noopener" class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"><div class="flex items-center gap-3"><span class="text-xs text-[var(--muted-foreground)] font-mono">#${escape_html(item.issue)}</span> <span class="text-sm">${escape_html(item.title)}</span></div> `);
		if (item.status === "done") {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<span class="text-xs px-2 py-0.5 rounded-full bg-green-500/20 text-green-400">✓ Done</span>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<span class="text-xs px-2 py-0.5 rounded-full bg-yellow-500/20 text-yellow-400">◎ Planned</span>`);
		}
		$$renderer.push(`<!--]--></a>`);
	}
	$$renderer.push(`<!--]--></div></section> <section><h2 class="text-xl font-semibold mb-4 flex items-center gap-2"><span class="px-2 py-0.5 rounded text-xs font-semibold bg-purple-500/20 text-purple-400">Future</span> What's Next</h2> <div class="space-y-3"><!--[-->`);
	const each_array_7 = ensure_array_like([
		{
			title: "Lightweight agent follow-up — 1 capped tool-call",
			issue: 117
		},
		{
			title: "`cora gain` — local productivity stats + viral sharing",
			issue: 161
		},
		{
			title: "GitHub App backend MVP in Rust (Axum)",
			issue: 132
		},
		{
			title: "Publish cora-review as GitHub Marketplace action",
			issue: 47
		}
	]);
	for (let $$index_7 = 0, $$length = each_array_7.length; $$index_7 < $$length; $$index_7++) {
		let item = each_array_7[$$index_7];
		$$renderer.push(`<a${attr("href", `https://github.com/codecoradev/cora-cli/issues/${stringify(item.issue)}`)} target="_blank" rel="noopener" class="flex items-center justify-between px-4 py-3 rounded-lg border border-[var(--border)] hover:border-[var(--accent)] transition-colors"><div class="flex items-center gap-3"><span class="text-xs text-[var(--muted-foreground)] font-mono">#${escape_html(item.issue)}</span> <span class="text-sm">${escape_html(item.title)}</span></div> <span class="text-xs px-2 py-0.5 rounded-full bg-purple-500/20 text-purple-400">→ Planned</span></a>`);
	}
	$$renderer.push(`<!--]--></div></section></div>`);
}
//#endregion
export { _page as default };
