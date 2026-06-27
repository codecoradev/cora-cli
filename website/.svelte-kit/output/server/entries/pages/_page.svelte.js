import { r as tick } from "../../chunks/index-server.js";
import { $ as attr, _t as getContext, a as bind_props, c as ensure_array_like, et as clsx$1, f as spread_props, g as html, i as attributes, l as head, m as stringify, mt as run, n as attr_class, nt as ATTACHMENT_KEY, o as derived, r as attr_style, tt as escape_html, u as props_id, vt as hasContext, yt as setContext } from "../../chunks/dev.js";
import { i as on } from "../../chunks/legacy-client.js";
import { r as createSubscriber, t as Icon } from "../../chunks/Icon.js";
import { clsx } from "clsx";
import parse from "style-to-object";
import { twMerge } from "tailwind-merge";
//#region node_modules/@lucide/svelte/dist/icons/arrow-right.svelte
function Arrow_right($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "arrow-right" },
		props,
		{ iconNode: [["path", { "d": "M5 12h14" }], ["path", { "d": "m12 5 7 7-7 7" }]] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/chevron-down.svelte
function Chevron_down($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "chevron-down" },
		props,
		{ iconNode: [["path", { "d": "m6 9 6 6 6-6" }]] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/circle-check-big.svelte
function Circle_check_big($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "circle-check-big" },
		props,
		{ iconNode: [["path", { "d": "M21.801 10A10 10 0 1 1 17 3.335" }], ["path", { "d": "m9 11 3 3L22 4" }]] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/copy.svelte
function Copy($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "copy" },
		props,
		{ iconNode: [["rect", {
			"width": "14",
			"height": "14",
			"x": "8",
			"y": "8",
			"rx": "2",
			"ry": "2"
		}], ["path", { "d": "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" }]] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/eye.svelte
function Eye($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "eye" },
		props,
		{ iconNode: [["path", { "d": "M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0" }], ["circle", {
			"cx": "12",
			"cy": "12",
			"r": "3"
		}]] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/file-check.svelte
function File_check($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "file-check" },
		props,
		{ iconNode: [
			["path", { "d": "M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z" }],
			["path", { "d": "M14 2v5a1 1 0 0 0 1 1h5" }],
			["path", { "d": "m9 15 2 2 4-4" }]
		] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/git-branch.svelte
function Git_branch($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "git-branch" },
		props,
		{ iconNode: [
			["path", { "d": "M15 6a9 9 0 0 0-9 9V3" }],
			["circle", {
				"cx": "18",
				"cy": "6",
				"r": "3"
			}],
			["circle", {
				"cx": "6",
				"cy": "18",
				"r": "3"
			}]
		] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/key-round.svelte
function Key_round($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "key-round" },
		props,
		{ iconNode: [["path", { "d": "M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z" }], ["circle", {
			"cx": "16.5",
			"cy": "7.5",
			"r": ".5",
			"fill": "currentColor"
		}]] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/layers.svelte
function Layers($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "layers" },
		props,
		{ iconNode: [
			["path", { "d": "M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z" }],
			["path", { "d": "M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 12" }],
			["path", { "d": "M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 17" }]
		] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/lightbulb.svelte
function Lightbulb($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "lightbulb" },
		props,
		{ iconNode: [
			["path", { "d": "M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5" }],
			["path", { "d": "M9 18h6" }],
			["path", { "d": "M10 22h4" }]
		] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/lock.svelte
function Lock($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "lock" },
		props,
		{ iconNode: [["rect", {
			"width": "18",
			"height": "11",
			"x": "3",
			"y": "11",
			"rx": "2",
			"ry": "2"
		}], ["path", { "d": "M7 11V7a5 5 0 0 1 10 0v4" }]] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/scan-search.svelte
function Scan_search($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "scan-search" },
		props,
		{ iconNode: [
			["path", { "d": "M3 7V5a2 2 0 0 1 2-2h2" }],
			["path", { "d": "M17 3h2a2 2 0 0 1 2 2v2" }],
			["path", { "d": "M21 17v2a2 2 0 0 1-2 2h-2" }],
			["path", { "d": "M7 21H5a2 2 0 0 1-2-2v-2" }],
			["circle", {
				"cx": "12",
				"cy": "12",
				"r": "3"
			}],
			["path", { "d": "m16 16-1.9-1.9" }]
		] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/shield-check.svelte
function Shield_check($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "shield-check" },
		props,
		{ iconNode: [["path", { "d": "M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" }], ["path", { "d": "m9 12 2 2 4-4" }]] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/shield.svelte
function Shield($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "shield" },
		props,
		{ iconNode: [["path", { "d": "M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" }]] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/star.svelte
function Star($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "star" },
		props,
		{ iconNode: [["path", { "d": "M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z" }]] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/upload.svelte
function Upload($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "upload" },
		props,
		{ iconNode: [
			["path", { "d": "M12 3v12" }],
			["path", { "d": "m17 8-5-5-5 5" }],
			["path", { "d": "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }]
		] }
	]));
}
//#endregion
//#region node_modules/@lucide/svelte/dist/icons/zap.svelte
function Zap($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	Icon($$renderer, spread_props([
		{ name: "zap" },
		props,
		{ iconNode: [["path", { "d": "M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z" }]] }
	]));
}
//#endregion
//#region src/lib/utils.ts
function cn(...inputs) {
	return twMerge(clsx(inputs));
}
//#endregion
//#region src/lib/components/landing/TerminalBlock.svelte
function TerminalBlock($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { title = "Terminal", showCopy = false, copyText, children, class: className, $$slots, $$events, ...restProps } = $$props;
		let copyClicked = false;
		$$renderer.push(`<div${attributes({
			class: clsx$1(cn("terminal relative", className)),
			...restProps
		})}><div class="terminal-header"><span class="terminal-dot terminal-dot-red"></span> <span class="terminal-dot terminal-dot-yellow"></span> <span class="terminal-dot terminal-dot-green"></span> <span class="terminal-title">${escape_html(title)}</span></div> <div class="terminal-body relative">`);
		children($$renderer);
		$$renderer.push(`<!----> `);
		if (showCopy) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<button${attr_class("copy-btn", void 0, { "copied": copyClicked })} aria-label="Copy command">`);
			$$renderer.push("<!--[-1-->");
			Copy($$renderer, { size: 14 });
			$$renderer.push(`<!--]--></button>`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></div></div>`);
	});
}
//#endregion
//#region src/lib/components/landing/HeroSection.svelte
function HeroSection($$renderer) {
	$$renderer.push(`<section class="section section-hero relative flex items-center justify-center min-h-[calc(100vh-3.5rem)] overflow-hidden"><div class="hero-glow animate-glow-pulse w-[500px] h-[500px] bg-[var(--accent)] opacity-[0.03] dark:opacity-[0.06] top-[10%] left-[20%]"></div> <div class="hero-glow animate-glow-pulse w-[400px] h-[400px] bg-[var(--accent)] opacity-[0.02] dark:opacity-[0.04] bottom-[10%] right-[15%]" style="animation-delay: 2s;"></div> <div class="absolute inset-0 grid-bg pointer-events-none"></div> <div class="relative z-10 max-w-3xl mx-auto text-center px-4"><div class="animate-fade-in-up mb-6"><span class="accent-badge"><span class="badge-dot"></span> Open source · BYOK · Zero config</span></div> <h1 class="animate-fade-in-up delay-100 text-4xl sm:text-5xl md:text-6xl font-bold -tracking-tight leading-[1.1]">AI code review <span class="hero-gradient">in your terminal</span></h1> <p class="animate-fade-in-up delay-200 mt-6 text-lg text-[var(--muted-foreground)] max-w-xl mx-auto leading-relaxed">cora catches bugs, security issues, and code smells before they land in your PR.
			Bring your own API key. Your code never leaves your machine.</p> <div class="animate-fade-in-up delay-300 mt-8 flex flex-col sm:flex-row items-center justify-center gap-3"><a href="/docs/getting-started" class="btn-primary group">Get Started `);
	Arrow_right($$renderer, { class: "w-4 h-4 transition-transform group-hover:translate-x-1" });
	$$renderer.push(`<!----></a> <a href="#demo-terminal" class="btn-ghost">See it in action</a></div> <div class="animate-fade-in-up delay-400 mt-8 max-w-md mx-auto">`);
	TerminalBlock($$renderer, {
		title: "install",
		children: ($$renderer) => {
			$$renderer.push(`<div><span class="syntax-cmd">cargo</span> <span class="syntax-flag">install</span> cora-cli</div> <div class="mt-2"><span class="syntax-cmd">cora</span> <span class="syntax-flag">auth login</span></div> <div><span class="syntax-cmd">cora</span> <span class="syntax-flag">review</span> <span class="syntax-highlight">--staged</span></div>`);
		},
		$$slots: { default: true }
	});
	$$renderer.push(`<!----></div> <div class="animate-fade-in-up delay-500 mt-10 flex items-center justify-center gap-6 text-xs text-[var(--muted-foreground)]"><span class="flex items-center gap-1.5">`);
	Shield($$renderer, { class: "w-3.5 h-3.5" });
	$$renderer.push(`<!----> Local-first</span> <span class="flex items-center gap-1.5">`);
	Zap($$renderer, { class: "w-3.5 h-3.5" });
	$$renderer.push(`<!----> 5 LLM providers</span> <span class="flex items-center gap-1.5">`);
	Eye($$renderer, { class: "w-3.5 h-3.5" });
	$$renderer.push(`<!----> CI/CD ready</span></div></div></section>`);
}
//#endregion
//#region src/lib/components/landing/KpiStats.svelte
function KpiStats($$renderer) {
	$$renderer.push(`<section class="section section-compact"><p class="text-center mb-10 scroll-reveal text-xs font-medium text-[var(--muted-foreground)] uppercase tracking-widest">Trusted by developers who ship fast</p> <div class="grid grid-cols-1 md:grid-cols-3 gap-4 max-w-3xl mx-auto"><div class="glass-card text-center scroll-reveal py-8"><div class="text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">5</div> <div class="text-sm text-[var(--muted-foreground)] mt-2">AI Providers</div></div> <div class="glass-card text-center scroll-reveal py-8 [transition-delay:100ms]"><div class="text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">&lt; 3s</div> <div class="text-sm text-[var(--muted-foreground)] mt-2">Review Time</div></div> <div class="glass-card text-center scroll-reveal py-8 [transition-delay:200ms]"><div class="text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">Zero</div> <div class="text-sm text-[var(--muted-foreground)] mt-2">Config Required</div></div></div></section>`);
}
//#endregion
//#region src/lib/components/landing/LiveDemo.svelte
function LiveDemo($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let terminalLines = [];
		const terminalOutput = [
			{
				text: "$ cora review --staged",
				color: "var(--muted-foreground)"
			},
			{
				text: "",
				color: ""
			},
			{
				text: "Analyzing 3 files...",
				color: "var(--muted-foreground)"
			},
			{
				text: "✓ src/auth/login.ts — 2 issues found",
				color: "var(--success)"
			},
			{
				text: "  ⚠ Line 42: Potential SQL injection",
				color: "var(--warning)"
			},
			{
				text: "  ⚠ Line 87: Hardcoded secret",
				color: "var(--warning)"
			},
			{
				text: "✓ src/utils/parser.ts — clean",
				color: "var(--muted-foreground)"
			},
			{
				text: "✓ src/api/routes.ts — 1 issue found",
				color: "var(--success)"
			},
			{
				text: "  ✗ Line 23: Missing error handling",
				color: "var(--destructive)"
			},
			{
				text: "",
				color: ""
			},
			{
				text: "3 issues found in 3 files",
				color: "var(--foreground)"
			}
		];
		$$renderer.push(`<section class="section section-tall" id="demo-terminal"><h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">See it in action</h2> <p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)] max-w-[32rem] mx-auto">Run cora against staged changes. Results in seconds, not minutes.</p> <div class="max-w-2xl mx-auto mt-10 scroll-reveal">`);
		TerminalBlock($$renderer, {
			title: "cora \\u2014 review",
			children: ($$renderer) => {
				$$renderer.push(`<!--[-->`);
				const each_array = ensure_array_like(terminalLines);
				for (let i = 0, $$length = each_array.length; i < $$length; i++) {
					let line = each_array[i];
					$$renderer.push(`<div class="min-h-[1.45em]"${attr_style(`color: ${stringify(terminalOutput[i]?.color || "var(--foreground)")};`)}>${escape_html(line)}</div>`);
				}
				$$renderer.push(`<!--]--> `);
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<!--]-->`);
			},
			$$slots: { default: true }
		});
		$$renderer.push(`<!----></div></section>`);
	});
}
//#endregion
//#region src/lib/components/landing/StepCard.svelte
function StepCard($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { number, icon, title, description, class: className, delayMs = 0 } = $$props;
		$$renderer.push(`<div${attr_class(clsx$1(cn("glass-card flex-1 text-center scroll-reveal", className)))}${attr_style("", { "transition-delay": delayMs ? `${delayMs}ms` : void 0 })}><div class="text-2xl font-bold text-[var(--accent)] -tracking-tight font-mono opacity-50">${escape_html(number)}</div> <div class="flex justify-center mt-4"><icon${attr("size", 24)} stroke="var(--accent)" fill="none"></icon></div> <h3 class="mt-4 text-xl font-semibold text-[var(--foreground)] -tracking-tight leading-snug">${escape_html(title)}</h3> <p class="mt-2 text-sm text-[var(--muted-foreground)]">${escape_html(description)}</p></div>`);
	});
}
//#endregion
//#region src/lib/components/landing/HowItWorks.svelte
function HowItWorks($$renderer) {
	$$renderer.push(`<section class="section section-compact"><h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">How it works</h2> <p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)] max-w-[32rem] mx-auto">Three steps from code to confidence.</p> <div class="flex flex-col md:flex-row items-stretch mt-10 gap-6">`);
	StepCard($$renderer, {
		number: "01",
		icon: Arrow_right,
		title: "Write code",
		description: "Push your changes as normal. cora only sees your diff."
	});
	$$renderer.push(`<!----> <div class="connect-line hidden md:flex scroll-reveal"><svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"></path><path d="M12 5l7 7-7 7"></path></svg></div> `);
	StepCard($$renderer, {
		number: "02",
		icon: Lightbulb,
		title: "Review with AI",
		description: "cora analyzes your diff with the LLM of your choice.",
		delayMs: 100
	});
	$$renderer.push(`<!----> <div class="connect-line hidden md:flex scroll-reveal [transition-delay:100ms]"><svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"></path><path d="M12 5l7 7-7 7"></path></svg></div> `);
	StepCard($$renderer, {
		number: "03",
		icon: Circle_check_big,
		title: "Ship with confidence",
		description: "Merge clean, production-ready code. Every time.",
		delayMs: 200
	});
	$$renderer.push(`<!----></div></section>`);
}
//#endregion
//#region src/lib/components/landing/FeatureCard.svelte
function FeatureCard($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { icon, title, accent, description, class: className, delayMs = 0 } = $$props;
		$$renderer.push(`<div${attr_class(clsx$1(cn("glass-card scroll-reveal", className)))}${attr_style("", { "transition-delay": delayMs ? `${delayMs}ms` : void 0 })}><div class="feature-icon"><icon${attr("size", 24)} stroke="currentColor" fill="none"></icon></div> <h3 class="mt-4 text-lg font-semibold text-[var(--foreground)]">${escape_html(title)}</h3> <p class="mt-2 text-sm text-[var(--accent)]">${escape_html(accent)}</p> <p class="mt-2 text-sm text-[var(--muted-foreground)] leading-relaxed">${html(description)}</p></div>`);
	});
}
//#endregion
//#region src/lib/components/landing/FeatureGrid.svelte
function FeatureGrid($$renderer) {
	$$renderer.push(`<section class="section section-tall"><h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">Built for developers who value control</h2> <p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)]">Everything you need, nothing you don't.</p> <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-10">`);
	FeatureCard($$renderer, {
		icon: Scan_search,
		title: "AI Code Review",
		accent: "Diff, branch, or full scan",
		description: "Three review modes: staged diff, branch comparison, or full project scan. LLM-powered analysis catches bugs, security issues, and style violations."
	});
	$$renderer.push(`<!----> `);
	FeatureCard($$renderer, {
		icon: Key_round,
		title: "Bring Your Own Key",
		accent: "No subscriptions, no lock-in",
		description: "Uses YOUR OpenAI, Anthropic, Groq, Ollama, or Z.AI API key. No data stored on our servers. You control the model, you control the cost.",
		delayMs: 100
	});
	$$renderer.push(`<!----> `);
	FeatureCard($$renderer, {
		icon: Git_branch,
		title: "Pre-commit Hooks",
		accent: "Review before you push",
		description: "Install once. Every commit gets reviewed automatically. Block bad code from entering your branch before it ships.",
		delayMs: 200
	});
	$$renderer.push(`<!----> `);
	FeatureCard($$renderer, {
		icon: Layers,
		title: "Incremental Scan",
		accent: "Only scan what changed",
		description: "SHA256 content hash cache. First scan indexes your codebase. Subsequent scans only review new or modified files.",
		delayMs: 300
	});
	$$renderer.push(`<!----> `);
	FeatureCard($$renderer, {
		icon: Shield_check,
		title: "SARIF Output",
		accent: "GitHub Code Scanning",
		description: "Upload review findings directly to GitHub's Security tab. Track issues across PRs. Works with any CI/CD pipeline.",
		delayMs: 400
	});
	$$renderer.push(`<!----> `);
	FeatureCard($$renderer, {
		icon: Lock,
		title: "Fully Private",
		accent: "Your code stays yours",
		description: "Runs entirely on your machine. No cloud, no telemetry, no data leaving your network. Perfect for Gitea and air-gapped environments.",
		delayMs: 500
	});
	$$renderer.push(`<!----> `);
	FeatureCard($$renderer, {
		icon: Lightbulb,
		title: "Deterministic Reviews",
		accent: "Same diff, same issues",
		description: "Temperature defaults to 0. Identical diffs always produce identical findings — perfect for CI reproducibility.",
		delayMs: 600
	});
	$$renderer.push(`<!----> `);
	FeatureCard($$renderer, {
		icon: Upload,
		title: "Smart Caching",
		accent: "Never review the same diff twice",
		description: "Reviews are cached by diff hash in <code class=\"text-[var(--accent)]\">~/.cache/cora/reviews/</code>. Re-reviewing an unchanged diff returns cached results instantly. Use <code class=\"text-[var(--accent)]\">--no-cache</code> to bypass.",
		delayMs: 700
	});
	$$renderer.push(`<!----> `);
	FeatureCard($$renderer, {
		icon: File_check,
		title: "Custom Prompts & Anti-Hallucination",
		accent: "Control the review, trust the output",
		description: "Override system prompts for review and scan. File path injection and post-parse filtering ensure the LLM only reports issues that exist in your actual diff.",
		delayMs: 800
	});
	$$renderer.push(`<!----></div></section>`);
}
//#endregion
//#region src/lib/components/landing/ComparisonTable.svelte
function ComparisonTable($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let revealed = false;
		const features = [
			{
				name: "BYOK",
				cora: true,
				coderabbit: false,
				copilot: false,
				sonarqube: null
			},
			{
				name: "Self-hosted",
				cora: true,
				coderabbit: false,
				copilot: false,
				sonarqube: true
			},
			{
				name: "Gitea / Forgejo",
				cora: true,
				coderabbit: false,
				copilot: false,
				sonarqube: true
			},
			{
				name: "CLI",
				cora: true,
				coderabbit: false,
				copilot: false,
				sonarqube: false
			},
			{
				name: "Pre-commit hooks",
				cora: true,
				coderabbit: false,
				copilot: false,
				sonarqube: false
			},
			{
				name: "SARIF output",
				cora: true,
				coderabbit: true,
				copilot: true,
				sonarqube: true
			},
			{
				name: "Cost",
				cora: "Free + API",
				coderabbit: "$12–39/mo",
				copilot: "$10–39/mo",
				sonarqube: "Free / $150+"
			},
			{
				name: "License",
				cora: "MIT",
				coderabbit: "Apache 2.0",
				copilot: "Proprietary",
				sonarqube: "LGPL"
			}
		];
		const competitors = [
			{
				key: "coderabbit",
				label: "CodeRabbit"
			},
			{
				key: "copilot",
				label: "Copilot"
			},
			{
				key: "sonarqube",
				label: "SonarQube"
			}
		];
		function cellIcon(value) {
			if (value === true) return "check";
			if (value === false) return "cross";
			if (value === null) return "dash";
			return "text";
		}
		$$renderer.push(`<section class="section section-compact"><h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">Why developers choose cora</h2> <p class="text-center mt-3 scroll-reveal text-sm text-[var(--muted-foreground)]">Compared to popular code review tools.</p> <div class="hidden md:block scroll-reveal mt-10 max-w-[56rem] mx-auto"><div class="compare-glass-card svelte-14kdxof"><table class="compare-table w-full svelte-14kdxof"><thead class="svelte-14kdxof"><tr><th class="text-left svelte-14kdxof">Feature</th><th class="text-center cora-highlight-col svelte-14kdxof"><span class="cora-badge svelte-14kdxof">cora</span></th><th class="text-center svelte-14kdxof">CodeRabbit</th><th class="text-center svelte-14kdxof">Copilot</th><th class="text-center svelte-14kdxof">SonarQube</th></tr></thead><tbody class="svelte-14kdxof"><!--[-->`);
		const each_array = ensure_array_like(features);
		for (let i = 0, $$length = each_array.length; i < $$length; i++) {
			let feat = each_array[i];
			$$renderer.push(`<tr${attr_class("compare-row svelte-14kdxof", void 0, { "revealed": revealed })}${attr_style(`transition-delay: ${stringify(i * 60)}ms`)}><td class="font-medium text-[var(--foreground)] svelte-14kdxof">${escape_html(feat.name)}</td><td class="text-center cora-highlight-col svelte-14kdxof">`);
			if (cellIcon(feat.cora) === "check") {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<span class="check-badge svelte-14kdxof">✓</span>`);
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<span class="font-semibold text-[var(--accent)]">${escape_html(feat.cora)}</span>`);
			}
			$$renderer.push(`<!--]--></td><td class="text-center svelte-14kdxof">`);
			if (cellIcon(feat[competitors[0].key]) === "check") {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<span class="check-muted svelte-14kdxof">✓</span>`);
			} else if (cellIcon(feat[competitors[0].key]) === "cross") {
				$$renderer.push("<!--[1-->");
				$$renderer.push(`<span class="cross-badge svelte-14kdxof">✗</span>`);
			} else if (cellIcon(feat[competitors[0].key]) === "dash") {
				$$renderer.push("<!--[2-->");
				$$renderer.push(`<span class="dash-muted svelte-14kdxof">—</span>`);
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<span class="text-sm text-[var(--muted-foreground)]">${escape_html(feat[competitors[0].key])}</span>`);
			}
			$$renderer.push(`<!--]--></td><td class="text-center svelte-14kdxof">`);
			if (cellIcon(feat[competitors[1].key]) === "check") {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<span class="check-muted svelte-14kdxof">✓</span>`);
			} else if (cellIcon(feat[competitors[1].key]) === "cross") {
				$$renderer.push("<!--[1-->");
				$$renderer.push(`<span class="cross-badge svelte-14kdxof">✗</span>`);
			} else if (cellIcon(feat[competitors[1].key]) === "dash") {
				$$renderer.push("<!--[2-->");
				$$renderer.push(`<span class="dash-muted svelte-14kdxof">—</span>`);
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<span class="text-sm text-[var(--muted-foreground)]">${escape_html(feat[competitors[1].key])}</span>`);
			}
			$$renderer.push(`<!--]--></td><td class="text-center svelte-14kdxof">`);
			if (cellIcon(feat[competitors[2].key]) === "check") {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<span class="check-muted svelte-14kdxof">✓</span>`);
			} else if (cellIcon(feat[competitors[2].key]) === "cross") {
				$$renderer.push("<!--[1-->");
				$$renderer.push(`<span class="cross-badge svelte-14kdxof">✗</span>`);
			} else if (cellIcon(feat[competitors[2].key]) === "dash") {
				$$renderer.push("<!--[2-->");
				$$renderer.push(`<span class="dash-muted svelte-14kdxof">—</span>`);
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<span class="text-sm text-[var(--muted-foreground)]">${escape_html(feat[competitors[2].key])}</span>`);
			}
			$$renderer.push(`<!--]--></td></tr>`);
		}
		$$renderer.push(`<!--]--></tbody></table></div></div> <div class="md:hidden mt-8 space-y-3 scroll-reveal"><!--[-->`);
		const each_array_1 = ensure_array_like(features);
		for (let i = 0, $$length = each_array_1.length; i < $$length; i++) {
			let feat = each_array_1[i];
			$$renderer.push(`<div${attr_class("compare-card svelte-14kdxof", void 0, { "revealed": revealed })}${attr_style(`transition-delay: ${stringify(i * 60)}ms`)}><div class="compare-card-header svelte-14kdxof"><span class="font-semibold text-[var(--foreground)] text-sm">${escape_html(feat.name)}</span></div> <div class="compare-card-body svelte-14kdxof"><div class="compare-card-cora svelte-14kdxof"><span class="text-xs font-bold uppercase tracking-wider text-[var(--accent)] opacity-70">cora</span> `);
			if (cellIcon(feat.cora) === "check") {
				$$renderer.push("<!--[0-->");
				$$renderer.push(`<span class="check-badge svelte-14kdxof">✓</span>`);
			} else {
				$$renderer.push("<!--[-1-->");
				$$renderer.push(`<span class="font-semibold text-[var(--accent)] text-sm">${escape_html(feat.cora)}</span>`);
			}
			$$renderer.push(`<!--]--></div> <!--[-->`);
			const each_array_2 = ensure_array_like(competitors);
			for (let $$index_1 = 0, $$length = each_array_2.length; $$index_1 < $$length; $$index_1++) {
				let comp = each_array_2[$$index_1];
				$$renderer.push(`<div class="compare-card-comp svelte-14kdxof"><span class="text-xs uppercase tracking-wider text-[var(--muted-foreground)] opacity-70">${escape_html(comp.label)}</span> `);
				if (cellIcon(feat[comp.key]) === "check") {
					$$renderer.push("<!--[0-->");
					$$renderer.push(`<span class="check-muted text-sm svelte-14kdxof">✓</span>`);
				} else if (cellIcon(feat[comp.key]) === "cross") {
					$$renderer.push("<!--[1-->");
					$$renderer.push(`<span class="cross-badge text-sm svelte-14kdxof">✗</span>`);
				} else if (cellIcon(feat[comp.key]) === "dash") {
					$$renderer.push("<!--[2-->");
					$$renderer.push(`<span class="dash-muted text-sm svelte-14kdxof">—</span>`);
				} else {
					$$renderer.push("<!--[-1-->");
					$$renderer.push(`<span class="text-sm text-[var(--muted-foreground)]">${escape_html(feat[comp.key])}</span>`);
				}
				$$renderer.push(`<!--]--></div>`);
			}
			$$renderer.push(`<!--]--></div></div>`);
		}
		$$renderer.push(`<!--]--></div></section>`);
	});
}
//#endregion
//#region node_modules/svelte-toolbelt/dist/utils/is.js
function isFunction(value) {
	return typeof value === "function";
}
function isObject(value) {
	return value !== null && typeof value === "object";
}
var CLASS_VALUE_PRIMITIVE_TYPES = [
	"string",
	"number",
	"bigint",
	"boolean"
];
function isClassValue(value) {
	if (value === null || value === void 0) return true;
	if (CLASS_VALUE_PRIMITIVE_TYPES.includes(typeof value)) return true;
	if (Array.isArray(value)) return value.every((item) => isClassValue(item));
	if (typeof value === "object") {
		if (Object.getPrototypeOf(value) !== Object.prototype) return false;
		return true;
	}
	return false;
}
//#endregion
//#region node_modules/svelte-toolbelt/dist/box/box-extras.svelte.js
var BoxSymbol = Symbol("box");
var isWritableSymbol = Symbol("is-writable");
function boxWith(getter, setter) {
	const derived$1 = derived(getter);
	if (setter) return {
		[BoxSymbol]: true,
		[isWritableSymbol]: true,
		get current() {
			return derived$1();
		},
		set current(v) {
			setter(v);
		}
	};
	return {
		[BoxSymbol]: true,
		get current() {
			return getter();
		}
	};
}
/**
* @returns Whether the value is a Box
*
* @see {@link https://runed.dev/docs/functions/box}
*/
function isBox(value) {
	return isObject(value) && BoxSymbol in value;
}
/**
* @returns Whether the value is a WritableBox
*
* @see {@link https://runed.dev/docs/functions/box}
*/
function isWritableBox(value) {
	return isBox(value) && isWritableSymbol in value;
}
function boxFrom(value) {
	if (isBox(value)) return value;
	if (isFunction(value)) return boxWith(value);
	return simpleBox(value);
}
/**
* Function that gets an object of boxes, and returns an object of reactive values
*
* @example
* const count = box(0)
* const flat = box.flatten({ count, double: box.with(() => count.current) })
* // type of flat is { count: number, readonly double: number }
*
* @see {@link https://runed.dev/docs/functions/box}
*/
function boxFlatten(boxes) {
	return Object.entries(boxes).reduce((acc, [key, b]) => {
		if (!isBox(b)) return Object.assign(acc, { [key]: b });
		if (isWritableBox(b)) Object.defineProperty(acc, key, {
			get() {
				return b.current;
			},
			set(v) {
				b.current = v;
			}
		});
		else Object.defineProperty(acc, key, { get() {
			return b.current;
		} });
		return acc;
	}, {});
}
/**
* Function that converts a box to a readonly box.
*
* @example
* const count = box(0) // WritableBox<number>
* const countReadonly = box.readonly(count) // ReadableBox<number>
*
* @see {@link https://runed.dev/docs/functions/box}
*/
function toReadonlyBox(b) {
	if (!isWritableBox(b)) return b;
	return {
		[BoxSymbol]: true,
		get current() {
			return b.current;
		}
	};
}
function simpleBox(initialValue) {
	let current = initialValue;
	return {
		[BoxSymbol]: true,
		[isWritableSymbol]: true,
		get current() {
			return current;
		},
		set current(v) {
			current = v;
		}
	};
}
//#endregion
//#region node_modules/svelte-toolbelt/dist/box/box.svelte.js
function box(initialValue) {
	let current = initialValue;
	return {
		[BoxSymbol]: true,
		[isWritableSymbol]: true,
		get current() {
			return current;
		},
		set current(v) {
			current = v;
		}
	};
}
box.from = boxFrom;
box.with = boxWith;
box.flatten = boxFlatten;
box.readonly = toReadonlyBox;
box.isBox = isBox;
box.isWritableBox = isWritableBox;
//#endregion
//#region node_modules/svelte-toolbelt/dist/utils/compose-handlers.js
/**
* Composes event handlers into a single function that can be called with an event.
* If the previous handler cancels the event using `event.preventDefault()`, the handlers
* that follow will not be called.
*/
function composeHandlers(...handlers) {
	return function(e) {
		for (const handler of handlers) {
			if (!handler) continue;
			if (e.defaultPrevented) return;
			if (typeof handler === "function") handler.call(this, e);
			else handler.current?.call(this, e);
		}
	};
}
//#endregion
//#region node_modules/svelte-toolbelt/dist/utils/strings.js
var NUMBER_CHAR_RE = /\d/;
var STR_SPLITTERS = [
	"-",
	"_",
	"/",
	"."
];
function isUppercase(char = "") {
	if (NUMBER_CHAR_RE.test(char)) return void 0;
	return char !== char.toLowerCase();
}
function splitByCase(str) {
	const parts = [];
	let buff = "";
	let previousUpper;
	let previousSplitter;
	for (const char of str) {
		const isSplitter = STR_SPLITTERS.includes(char);
		if (isSplitter === true) {
			parts.push(buff);
			buff = "";
			previousUpper = void 0;
			continue;
		}
		const isUpper = isUppercase(char);
		if (previousSplitter === false) {
			if (previousUpper === false && isUpper === true) {
				parts.push(buff);
				buff = char;
				previousUpper = isUpper;
				continue;
			}
			if (previousUpper === true && isUpper === false && buff.length > 1) {
				const lastChar = buff.at(-1);
				parts.push(buff.slice(0, Math.max(0, buff.length - 1)));
				buff = lastChar + char;
				previousUpper = isUpper;
				continue;
			}
		}
		buff += char;
		previousUpper = isUpper;
		previousSplitter = isSplitter;
	}
	parts.push(buff);
	return parts;
}
function pascalCase(str) {
	if (!str) return "";
	return splitByCase(str).map((p) => upperFirst(p)).join("");
}
function camelCase(str) {
	return lowerFirst(pascalCase(str || ""));
}
function upperFirst(str) {
	return str ? str[0].toUpperCase() + str.slice(1) : "";
}
function lowerFirst(str) {
	return str ? str[0].toLowerCase() + str.slice(1) : "";
}
//#endregion
//#region node_modules/svelte-toolbelt/dist/utils/css-to-style-obj.js
function cssToStyleObj(css) {
	if (!css) return {};
	const styleObj = {};
	function iterator(name, value) {
		if (name.startsWith("-moz-") || name.startsWith("-webkit-") || name.startsWith("-ms-") || name.startsWith("-o-")) {
			styleObj[pascalCase(name)] = value;
			return;
		}
		if (name.startsWith("--")) {
			styleObj[name] = value;
			return;
		}
		styleObj[camelCase(name)] = value;
	}
	parse(css, iterator);
	return styleObj;
}
//#endregion
//#region node_modules/svelte-toolbelt/dist/utils/execute-callbacks.js
/**
* Executes an array of callback functions with the same arguments.
* @template T The types of the arguments that the callback functions take.
* @param callbacks array of callback functions to execute.
* @returns A new function that executes all of the original callback functions with the same arguments.
*/
function executeCallbacks(...callbacks) {
	return (...args) => {
		for (const callback of callbacks) if (typeof callback === "function") callback(...args);
	};
}
//#endregion
//#region node_modules/svelte-toolbelt/dist/utils/style-to-css.js
function createParser(matcher, replacer) {
	const regex = RegExp(matcher, "g");
	return (str) => {
		if (typeof str !== "string") throw new TypeError(`expected an argument of type string, but got ${typeof str}`);
		if (!str.match(regex)) return str;
		return str.replace(regex, replacer);
	};
}
var camelToKebab = createParser(/[A-Z]/, (match) => `-${match.toLowerCase()}`);
function styleToCSS(styleObj) {
	if (!styleObj || typeof styleObj !== "object" || Array.isArray(styleObj)) throw new TypeError(`expected an argument of type object, but got ${typeof styleObj}`);
	return Object.keys(styleObj).map((property) => `${camelToKebab(property)}: ${styleObj[property]};`).join("\n");
}
//#endregion
//#region node_modules/svelte-toolbelt/dist/utils/style.js
function styleToString(style = {}) {
	return styleToCSS(style).replace("\n", " ");
}
var EVENT_LIST_SET = new Set([
	"onabort",
	"onanimationcancel",
	"onanimationend",
	"onanimationiteration",
	"onanimationstart",
	"onauxclick",
	"onbeforeinput",
	"onbeforetoggle",
	"onblur",
	"oncancel",
	"oncanplay",
	"oncanplaythrough",
	"onchange",
	"onclick",
	"onclose",
	"oncompositionend",
	"oncompositionstart",
	"oncompositionupdate",
	"oncontextlost",
	"oncontextmenu",
	"oncontextrestored",
	"oncopy",
	"oncuechange",
	"oncut",
	"ondblclick",
	"ondrag",
	"ondragend",
	"ondragenter",
	"ondragleave",
	"ondragover",
	"ondragstart",
	"ondrop",
	"ondurationchange",
	"onemptied",
	"onended",
	"onerror",
	"onfocus",
	"onfocusin",
	"onfocusout",
	"onformdata",
	"ongotpointercapture",
	"oninput",
	"oninvalid",
	"onkeydown",
	"onkeypress",
	"onkeyup",
	"onload",
	"onloadeddata",
	"onloadedmetadata",
	"onloadstart",
	"onlostpointercapture",
	"onmousedown",
	"onmouseenter",
	"onmouseleave",
	"onmousemove",
	"onmouseout",
	"onmouseover",
	"onmouseup",
	"onpaste",
	"onpause",
	"onplay",
	"onplaying",
	"onpointercancel",
	"onpointerdown",
	"onpointerenter",
	"onpointerleave",
	"onpointermove",
	"onpointerout",
	"onpointerover",
	"onpointerup",
	"onprogress",
	"onratechange",
	"onreset",
	"onresize",
	"onscroll",
	"onscrollend",
	"onsecuritypolicyviolation",
	"onseeked",
	"onseeking",
	"onselect",
	"onselectionchange",
	"onselectstart",
	"onslotchange",
	"onstalled",
	"onsubmit",
	"onsuspend",
	"ontimeupdate",
	"ontoggle",
	"ontouchcancel",
	"ontouchend",
	"ontouchmove",
	"ontouchstart",
	"ontransitioncancel",
	"ontransitionend",
	"ontransitionrun",
	"ontransitionstart",
	"onvolumechange",
	"onwaiting",
	"onwebkitanimationend",
	"onwebkitanimationiteration",
	"onwebkitanimationstart",
	"onwebkittransitionend",
	"onwheel"
]);
//#endregion
//#region node_modules/svelte-toolbelt/dist/utils/merge-props.js
/**
* Modified from https://github.com/adobe/react-spectrum/blob/main/packages/%40react-aria/utils/src/mergeProps.ts (see NOTICE.txt for source)
*/
function isEventHandler(key) {
	return EVENT_LIST_SET.has(key);
}
/**
* Given a list of prop objects, merges them into a single object.
* - Automatically composes event handlers (e.g. `onclick`, `oninput`, etc.)
* - Chains regular functions with the same name so they are called in order
* - Merges class strings with `clsx`
* - Merges style objects and converts them to strings
* - Handles a bug with Svelte where setting the `hidden` attribute to `false` doesn't remove it
* - Overrides other values with the last one
*/
function mergeProps(...args) {
	const result = { ...args[0] };
	for (let i = 1; i < args.length; i++) {
		const props = args[i];
		if (!props) continue;
		for (const key of Object.keys(props)) {
			const a = result[key];
			const b = props[key];
			const aIsFunction = typeof a === "function";
			const bIsFunction = typeof b === "function";
			if (aIsFunction && typeof bIsFunction && isEventHandler(key)) result[key] = composeHandlers(a, b);
			else if (aIsFunction && bIsFunction) result[key] = executeCallbacks(a, b);
			else if (key === "class") {
				const aIsClassValue = isClassValue(a);
				const bIsClassValue = isClassValue(b);
				if (aIsClassValue && bIsClassValue) result[key] = clsx(a, b);
				else if (aIsClassValue) result[key] = clsx(a);
				else if (bIsClassValue) result[key] = clsx(b);
			} else if (key === "style") {
				const aIsObject = typeof a === "object";
				const bIsObject = typeof b === "object";
				const aIsString = typeof a === "string";
				const bIsString = typeof b === "string";
				if (aIsObject && bIsObject) result[key] = {
					...a,
					...b
				};
				else if (aIsObject && bIsString) {
					const parsedStyle = cssToStyleObj(b);
					result[key] = {
						...a,
						...parsedStyle
					};
				} else if (aIsString && bIsObject) result[key] = {
					...cssToStyleObj(a),
					...b
				};
				else if (aIsString && bIsString) {
					const parsedStyleA = cssToStyleObj(a);
					const parsedStyleB = cssToStyleObj(b);
					result[key] = {
						...parsedStyleA,
						...parsedStyleB
					};
				} else if (aIsObject) result[key] = a;
				else if (bIsObject) result[key] = b;
				else if (aIsString) result[key] = a;
				else if (bIsString) result[key] = b;
			} else result[key] = b !== void 0 ? b : a;
		}
		for (const key of Object.getOwnPropertySymbols(props)) {
			const a = result[key];
			const b = props[key];
			result[key] = b !== void 0 ? b : a;
		}
	}
	if (typeof result.style === "object") result.style = styleToString(result.style).replaceAll("\n", " ");
	if (result.hidden === false) {
		result.hidden = void 0;
		delete result.hidden;
	}
	if (result.disabled === false) {
		result.disabled = void 0;
		delete result.disabled;
	}
	return result;
}
//#endregion
//#region node_modules/runed/dist/internal/configurable-globals.js
var defaultWindow = void 0;
//#endregion
//#region node_modules/runed/dist/internal/utils/dom.js
/**
* Handles getting the active element in a document or shadow root.
* If the active element is within a shadow root, it will traverse the shadow root
* to find the active element.
* If not, it will return the active element in the document.
*
* @param document A document or shadow root to get the active element from.
* @returns The active element in the document or shadow root.
*/
function getActiveElement(document) {
	let activeElement = document.activeElement;
	while (activeElement?.shadowRoot) {
		const node = activeElement.shadowRoot.activeElement;
		if (node === activeElement) break;
		else activeElement = node;
	}
	return activeElement;
}
//#endregion
//#region node_modules/runed/dist/utilities/active-element/active-element.svelte.js
var ActiveElement = class {
	#document;
	#subscribe;
	constructor(options = {}) {
		const { window = defaultWindow, document = window?.document } = options;
		if (window === void 0) return;
		this.#document = document;
		this.#subscribe = createSubscriber((update) => {
			const cleanupFocusIn = on(window, "focusin", update);
			const cleanupFocusOut = on(window, "focusout", update);
			return () => {
				cleanupFocusIn();
				cleanupFocusOut();
			};
		});
	}
	get current() {
		this.#subscribe?.();
		if (!this.#document) return null;
		return getActiveElement(this.#document);
	}
};
new ActiveElement();
//#endregion
//#region node_modules/runed/dist/utilities/context/context.js
var Context = class {
	#name;
	#key;
	/**
	* @param name The name of the context.
	* This is used for generating the context key and error messages.
	*/
	constructor(name) {
		this.#name = name;
		this.#key = Symbol(name);
	}
	/**
	* The key used to get and set the context.
	*
	* It is not recommended to use this value directly.
	* Instead, use the methods provided by this class.
	*/
	get key() {
		return this.#key;
	}
	/**
	* Checks whether this has been set in the context of a parent component.
	*
	* Must be called during component initialisation.
	*/
	exists() {
		return hasContext(this.#key);
	}
	/**
	* Retrieves the context that belongs to the closest parent component.
	*
	* Must be called during component initialisation.
	*
	* @throws An error if the context does not exist.
	*/
	get() {
		const context = getContext(this.#key);
		if (context === void 0) throw new Error(`Context "${this.#name}" not found`);
		return context;
	}
	/**
	* Retrieves the context that belongs to the closest parent component,
	* or the given fallback value if the context does not exist.
	*
	* Must be called during component initialisation.
	*/
	getOr(fallback) {
		const context = getContext(this.#key);
		if (context === void 0) return fallback;
		return context;
	}
	/**
	* Associates the given value with the current component and returns it.
	*
	* Must be called during component initialisation.
	*/
	set(context) {
		return setContext(this.#key, context);
	}
};
//#endregion
//#region node_modules/runed/dist/utilities/watch/watch.svelte.js
function runWatcher(sources, flush, effect, options = {}) {
	const { lazy = false } = options;
}
function watch(sources, effect, options) {
	runWatcher(sources, "post", effect, options);
}
function watchPre(sources, effect, options) {
	runWatcher(sources, "pre", effect, options);
}
watch.pre = watchPre;
function watchOnce(source, effect) {}
function watchOncePre(source, effect) {}
watchOnce.pre = watchOncePre;
//#endregion
//#region node_modules/runed/dist/utilities/resource/resource.svelte.js
function debounce(fn, delay) {
	let timeoutId;
	let lastResolve = null;
	return (...args) => {
		return new Promise((resolve) => {
			if (lastResolve) lastResolve(void 0);
			lastResolve = resolve;
			clearTimeout(timeoutId);
			timeoutId = setTimeout(async () => {
				const result = await fn(...args);
				if (lastResolve) {
					lastResolve(result);
					lastResolve = null;
				}
			}, delay);
		});
	};
}
function throttle(fn, delay) {
	let lastRun = 0;
	let lastPromise = null;
	return (...args) => {
		const now = Date.now();
		if (lastRun && now - lastRun < delay) return lastPromise ?? Promise.resolve(void 0);
		lastRun = now;
		lastPromise = fn(...args);
		return lastPromise;
	};
}
function runResource(source, fetcher, options = {}, effectFn) {
	const { lazy = false, once = false, initialValue, debounce: debounceTime, throttle: throttleTime } = options;
	let current = initialValue;
	let loading = false;
	let error = void 0;
	let cleanupFns = [];
	const runCleanup = () => {
		cleanupFns.forEach((fn) => fn());
		cleanupFns = [];
	};
	const onCleanup = (fn) => {
		cleanupFns = [...cleanupFns, fn];
	};
	const baseFetcher = async (value, previousValue, refetching = false) => {
		try {
			loading = true;
			error = void 0;
			runCleanup();
			const controller = new AbortController();
			onCleanup(() => controller.abort());
			const result = await fetcher(value, previousValue, {
				data: current,
				refetching,
				onCleanup,
				signal: controller.signal
			});
			current = result;
			return result;
		} catch (e) {
			if (!(e instanceof DOMException && e.name === "AbortError")) error = e;
			return;
		} finally {
			loading = false;
		}
	};
	const runFetcher = debounceTime ? debounce(baseFetcher, debounceTime) : throttleTime ? throttle(baseFetcher, throttleTime) : baseFetcher;
	const sources = Array.isArray(source) ? source : [source];
	let prevValues;
	effectFn((values, previousValues) => {
		if (once && prevValues) return;
		prevValues = values;
		runFetcher(Array.isArray(source) ? values : values[0], Array.isArray(source) ? previousValues : previousValues?.[0]);
	}, { lazy });
	return {
		get current() {
			return current;
		},
		get loading() {
			return loading;
		},
		get error() {
			return error;
		},
		mutate: (value) => {
			current = value;
		},
		refetch: (info) => {
			const values = sources.map((s) => s());
			return runFetcher(Array.isArray(source) ? values : values[0], Array.isArray(source) ? values : values[0], info ?? true);
		}
	};
}
function resource(source, fetcher, options) {
	return runResource(source, fetcher, options, (fn, options) => {
		const sources = Array.isArray(source) ? source : [source];
		const getters = () => sources.map((s) => s());
		watch(getters, (values, previousValues) => {
			fn(values, previousValues ?? []);
		}, options);
	});
}
function resourcePre(source, fetcher, options) {
	return runResource(source, fetcher, options, (fn, options) => {
		const sources = Array.isArray(source) ? source : [source];
		const getter = () => sources.map((s) => s());
		watch.pre(getter, (values, previousValues) => {
			fn(values, previousValues ?? []);
		}, options);
	});
}
resource.pre = resourcePre;
//#endregion
//#region node_modules/svelte-toolbelt/dist/utils/after-tick.js
function afterTick(fn) {
	(/* @__PURE__ */ tick()).then(fn);
}
if (typeof HTMLElement === "function");
//#endregion
//#region node_modules/svelte/src/attachments/index.js
/**
* Creates an object key that will be recognised as an attachment when the object is spread onto an element,
* as a programmatic alternative to using `{@attach ...}`. This can be useful for library authors, though
* is generally not needed when building an app.
*
* ```svelte
* <script>
* 	import { createAttachmentKey } from 'svelte/attachments';
*
* 	const props = {
* 		class: 'cool',
* 		onclick: () => alert('clicked'),
* 		[createAttachmentKey()]: (node) => {
* 			node.textContent = 'attached!';
* 		}
* 	};
* <\/script>
*
* <button {...props}>click me</button>
* ```
* @since 5.29
*/
function createAttachmentKey() {
	return Symbol(ATTACHMENT_KEY);
}
//#endregion
//#region node_modules/svelte-toolbelt/dist/utils/attach-ref.js
/**
* Creates a Svelte Attachment that attaches a DOM element to a ref.
* The ref can be either a WritableBox or a callback function.
*
* @param ref - Either a WritableBox to store the element in, or a callback function that receives the element
* @param onChange - Optional callback that fires when the ref changes
* @returns An object with a spreadable attachment key that should be spread onto the element
*
* @example
* // Using with WritableBox
* const ref = box<HTMLDivElement | null>(null);
* <div {...attachRef(ref)}>Content</div>
*
* @example
* // Using with callback
* <div {...attachRef((node) => myNode = node)}>Content</div>
*
* @example
* // Using with onChange
* <div {...attachRef(ref, (node) => console.log(node))}>Content</div>
*/
function attachRef(ref, onChange) {
	return { [createAttachmentKey()]: (node) => {
		if (isBox(ref)) {
			ref.current = node;
			run(() => onChange?.(node));
			return () => {
				if ("isConnected" in node && node.isConnected) return;
				ref.current = null;
				onChange?.(null);
			};
		}
		ref(node);
		run(() => onChange?.(node));
		return () => {
			if ("isConnected" in node && node.isConnected) return;
			ref(null);
			onChange?.(null);
		};
	} };
}
//#endregion
//#region node_modules/bits-ui/dist/internal/attrs.js
function boolToStr(condition) {
	return condition ? "true" : "false";
}
function boolToEmptyStrOrUndef(condition) {
	return condition ? "" : void 0;
}
function getDataOpenClosed(condition) {
	return condition ? "open" : "closed";
}
function getDataTransitionAttrs(state) {
	if (state === "starting") return { "data-starting-style": "" };
	if (state === "ending") return { "data-ending-style": "" };
	return {};
}
var BitsAttrs = class {
	#variant;
	#prefix;
	attrs;
	constructor(config) {
		this.#variant = config.getVariant ? config.getVariant() : null;
		this.#prefix = this.#variant ? `data-${this.#variant}-` : `data-${config.component}-`;
		this.getAttr = this.getAttr.bind(this);
		this.selector = this.selector.bind(this);
		this.attrs = Object.fromEntries(config.parts.map((part) => [part, this.getAttr(part)]));
	}
	getAttr(part, variantOverride) {
		if (variantOverride) return `data-${variantOverride}-${part}`;
		return `${this.#prefix}${part}`;
	}
	selector(part, variantOverride) {
		return `[${this.getAttr(part, variantOverride)}]`;
	}
};
function createBitsAttrs(config) {
	const bitsAttrs = new BitsAttrs(config);
	return {
		...bitsAttrs.attrs,
		selector: bitsAttrs.selector,
		getAttr: bitsAttrs.getAttr
	};
}
//#endregion
//#region node_modules/bits-ui/dist/internal/kbd-constants.js
var ARROW_DOWN = "ArrowDown";
var ARROW_LEFT = "ArrowLeft";
var ARROW_RIGHT = "ArrowRight";
var ARROW_UP = "ArrowUp";
var HOME = "Home";
var PAGE_DOWN = "PageDown";
var PAGE_UP = "PageUp";
//#endregion
//#region node_modules/bits-ui/dist/internal/locale.js
/**
* Detects the text direction in the element.
* @returns {Direction} The text direction ('ltr' for left-to-right or 'rtl' for right-to-left).
*/
function getElemDirection(elem) {
	return window.getComputedStyle(elem).getPropertyValue("direction");
}
//#endregion
//#region node_modules/bits-ui/dist/internal/get-directional-keys.js
var FIRST_KEYS = [
	ARROW_DOWN,
	PAGE_UP,
	HOME
];
var LAST_KEYS = [
	ARROW_UP,
	PAGE_DOWN,
	"End"
];
[...FIRST_KEYS, ...LAST_KEYS];
/**
* A utility function that returns the next key based on the direction and orientation.
*/
function getNextKey(dir = "ltr", orientation = "horizontal") {
	return {
		horizontal: dir === "rtl" ? ARROW_LEFT : ARROW_RIGHT,
		vertical: ARROW_DOWN
	}[orientation];
}
/**
* A utility function that returns the previous key based on the direction and orientation.
*/
function getPrevKey(dir = "ltr", orientation = "horizontal") {
	return {
		horizontal: dir === "rtl" ? ARROW_RIGHT : ARROW_LEFT,
		vertical: ARROW_UP
	}[orientation];
}
/**
* A utility function that returns the next and previous keys based on the direction
* and orientation.
*/
function getDirectionalKeys(dir = "ltr", orientation = "horizontal") {
	if (!["ltr", "rtl"].includes(dir)) dir = "ltr";
	if (!["horizontal", "vertical"].includes(orientation)) orientation = "horizontal";
	return {
		nextKey: getNextKey(dir, orientation),
		prevKey: getPrevKey(dir, orientation)
	};
}
//#endregion
//#region node_modules/bits-ui/dist/internal/is.js
var isBrowser = typeof document !== "undefined";
getIsIOS();
function getIsIOS() {
	return isBrowser && window?.navigator?.userAgent && (/iP(ad|hone|od)/.test(window.navigator.userAgent) || window?.navigator?.maxTouchPoints > 2 && /iPad|Macintosh/.test(window?.navigator.userAgent));
}
function isHTMLElement(element) {
	return element instanceof HTMLElement;
}
//#endregion
//#region node_modules/bits-ui/dist/internal/roving-focus-group.js
var RovingFocusGroup = class {
	#opts;
	#currentTabStopId = box(null);
	constructor(opts) {
		this.#opts = opts;
	}
	getCandidateNodes() {
		return [];
	}
	focusFirstCandidate() {
		const items = this.getCandidateNodes();
		if (!items.length) return;
		items[0]?.focus();
	}
	handleKeydown(node, e, both = false) {
		const rootNode = this.#opts.rootNode.current;
		if (!rootNode || !node) return;
		const items = this.getCandidateNodes();
		if (!items.length) return;
		const currentIndex = items.indexOf(node);
		const { nextKey, prevKey } = getDirectionalKeys(getElemDirection(rootNode), this.#opts.orientation.current);
		const loop = this.#opts.loop.current;
		const keyToIndex = {
			[nextKey]: currentIndex + 1,
			[prevKey]: currentIndex - 1,
			[HOME]: 0,
			["End"]: items.length - 1
		};
		if (both) {
			const altNextKey = nextKey === "ArrowDown" ? ARROW_RIGHT : ARROW_DOWN;
			const altPrevKey = prevKey === "ArrowUp" ? ARROW_LEFT : ARROW_UP;
			keyToIndex[altNextKey] = currentIndex + 1;
			keyToIndex[altPrevKey] = currentIndex - 1;
		}
		let itemIndex = keyToIndex[e.key];
		if (itemIndex === void 0) return;
		e.preventDefault();
		if (itemIndex < 0 && loop) itemIndex = items.length - 1;
		else if (itemIndex === items.length && loop) itemIndex = 0;
		const itemToFocus = items[itemIndex];
		if (!itemToFocus) return;
		itemToFocus.focus();
		this.#currentTabStopId.current = itemToFocus.id;
		this.#opts.onCandidateFocus?.(itemToFocus);
		return itemToFocus;
	}
	getTabIndex(node) {
		const items = this.getCandidateNodes();
		const anyActive = this.#currentTabStopId.current !== null;
		if (node && !anyActive && items[0] === node) {
			this.#currentTabStopId.current = node.id;
			return 0;
		} else if (node?.id === this.#currentTabStopId.current) return 0;
		return -1;
	}
	setCurrentTabStopId(id) {
		this.#currentTabStopId.current = id;
	}
	focusCurrentTabStop() {
		const currentTabStopId = this.#currentTabStopId.current;
		if (!currentTabStopId) return;
		const currentTabStop = this.#opts.rootNode.current?.querySelector(`#${currentTabStopId}`);
		if (!currentTabStop || !isHTMLElement(currentTabStop)) return;
		currentTabStop.focus();
	}
};
//#endregion
//#region node_modules/bits-ui/dist/internal/animations-complete.js
var AnimationsComplete = class {
	#opts;
	#currentFrame = null;
	#observer = null;
	#runId = 0;
	constructor(opts) {
		this.#opts = opts;
	}
	#cleanup() {
		if (this.#currentFrame !== null) {
			window.cancelAnimationFrame(this.#currentFrame);
			this.#currentFrame = null;
		}
		this.#observer?.disconnect();
		this.#observer = null;
		this.#runId++;
	}
	run(fn) {
		this.#cleanup();
		const node = this.#opts.ref.current;
		if (!node) return;
		if (typeof node.getAnimations !== "function") {
			this.#executeCallback(fn);
			return;
		}
		const runId = this.#runId;
		const executeIfCurrent = () => {
			if (runId !== this.#runId) return;
			this.#executeCallback(fn);
		};
		const waitForAnimations = () => {
			if (runId !== this.#runId) return;
			const animations = node.getAnimations();
			if (animations.length === 0) {
				executeIfCurrent();
				return;
			}
			Promise.all(animations.map((animation) => animation.finished)).then(() => {
				executeIfCurrent();
			}).catch(() => {
				if (runId !== this.#runId) return;
				if (node.getAnimations().some((animation) => animation.pending || animation.playState !== "finished")) {
					waitForAnimations();
					return;
				}
				executeIfCurrent();
			});
		};
		const requestWaitForAnimations = () => {
			this.#currentFrame = window.requestAnimationFrame(() => {
				this.#currentFrame = null;
				waitForAnimations();
			});
		};
		if (!this.#opts.afterTick.current) {
			requestWaitForAnimations();
			return;
		}
		this.#currentFrame = window.requestAnimationFrame(() => {
			this.#currentFrame = null;
			const startingStyleAttr = "data-starting-style";
			if (!node.hasAttribute(startingStyleAttr)) {
				requestWaitForAnimations();
				return;
			}
			this.#observer = new MutationObserver(() => {
				if (runId !== this.#runId) return;
				if (node.hasAttribute(startingStyleAttr)) return;
				this.#observer?.disconnect();
				this.#observer = null;
				requestWaitForAnimations();
			});
			this.#observer.observe(node, {
				attributes: true,
				attributeFilter: [startingStyleAttr]
			});
		});
	}
	#executeCallback(fn) {
		const execute = () => {
			fn();
		};
		if (this.#opts.afterTick) afterTick(execute);
		else execute();
	}
};
//#endregion
//#region node_modules/bits-ui/dist/internal/presence-manager.svelte.js
var PresenceManager = class {
	#opts;
	#enabled;
	#afterAnimations;
	#shouldRender = false;
	#transitionStatus = void 0;
	#hasMounted = false;
	#transitionFrame = null;
	constructor(opts) {
		this.#opts = opts;
		this.#shouldRender = opts.open.current;
		this.#enabled = opts.enabled ?? true;
		this.#afterAnimations = new AnimationsComplete({
			ref: this.#opts.ref,
			afterTick: this.#opts.open
		});
		watch(() => this.#opts.open.current, (isOpen) => {
			if (!this.#hasMounted) {
				this.#hasMounted = true;
				return;
			}
			this.#clearTransitionFrame();
			if (!isOpen && this.#opts.shouldSkipExitAnimation?.()) {
				this.#shouldRender = false;
				this.#transitionStatus = void 0;
				this.#opts.onComplete?.();
				return;
			}
			if (isOpen) this.#shouldRender = true;
			this.#transitionStatus = isOpen ? "starting" : "ending";
			if (isOpen) this.#transitionFrame = window.requestAnimationFrame(() => {
				this.#transitionFrame = null;
				if (this.#opts.open.current) this.#transitionStatus = void 0;
			});
			if (!this.#enabled) {
				if (!isOpen) this.#shouldRender = false;
				this.#transitionStatus = void 0;
				this.#opts.onComplete?.();
				return;
			}
			this.#afterAnimations.run(() => {
				if (isOpen === this.#opts.open.current) {
					if (!this.#opts.open.current) this.#shouldRender = false;
					this.#transitionStatus = void 0;
					this.#opts.onComplete?.();
				}
			});
		});
	}
	get shouldRender() {
		return this.#shouldRender;
	}
	get transitionStatus() {
		return this.#transitionStatus;
	}
	#clearTransitionFrame() {
		if (this.#transitionFrame === null) return;
		window.cancelAnimationFrame(this.#transitionFrame);
		this.#transitionFrame = null;
	}
};
//#endregion
//#region node_modules/bits-ui/dist/bits/accordion/accordion.svelte.js
var accordionAttrs = createBitsAttrs({
	component: "accordion",
	parts: [
		"root",
		"trigger",
		"content",
		"item",
		"header"
	]
});
var AccordionRootContext = new Context("Accordion.Root");
var AccordionItemContext = new Context("Accordion.Item");
var AccordionBaseState = class {
	opts;
	rovingFocusGroup;
	attachment;
	constructor(opts) {
		this.opts = opts;
		this.rovingFocusGroup = new RovingFocusGroup({
			rootNode: this.opts.ref,
			candidateAttr: accordionAttrs.trigger,
			loop: this.opts.loop,
			orientation: this.opts.orientation
		});
		this.attachment = attachRef(this.opts.ref);
	}
	#props = derived(() => ({
		id: this.opts.id.current,
		"data-orientation": this.opts.orientation.current,
		"data-disabled": boolToEmptyStrOrUndef(this.opts.disabled.current),
		[accordionAttrs.root]: "",
		...this.attachment
	}));
	get props() {
		return this.#props();
	}
	set props($$value) {
		return this.#props($$value);
	}
};
var AccordionSingleState = class extends AccordionBaseState {
	opts;
	isMulti = false;
	constructor(opts) {
		super(opts);
		this.opts = opts;
		this.includesItem = this.includesItem.bind(this);
		this.toggleItem = this.toggleItem.bind(this);
	}
	includesItem(item) {
		return this.opts.value.current === item;
	}
	toggleItem(item) {
		this.opts.value.current = this.includesItem(item) ? "" : item;
	}
};
var AccordionMultiState = class extends AccordionBaseState {
	#value;
	isMulti = true;
	constructor(props) {
		super(props);
		this.#value = props.value;
		this.includesItem = this.includesItem.bind(this);
		this.toggleItem = this.toggleItem.bind(this);
	}
	includesItem(item) {
		return this.#value.current.includes(item);
	}
	toggleItem(item) {
		this.#value.current = this.includesItem(item) ? this.#value.current.filter((v) => v !== item) : [...this.#value.current, item];
	}
};
var AccordionRootState = class {
	static create(props) {
		const { type, ...rest } = props;
		const rootState = type === "single" ? new AccordionSingleState(rest) : new AccordionMultiState(rest);
		return AccordionRootContext.set(rootState);
	}
};
var AccordionItemState = class AccordionItemState {
	static create(props) {
		return AccordionItemContext.set(new AccordionItemState({
			...props,
			rootState: AccordionRootContext.get()
		}));
	}
	opts;
	root;
	#isActive = derived(() => this.root.includesItem(this.opts.value.current));
	get isActive() {
		return this.#isActive();
	}
	set isActive($$value) {
		return this.#isActive($$value);
	}
	#isDisabled = derived(() => this.opts.disabled.current || this.root.opts.disabled.current);
	get isDisabled() {
		return this.#isDisabled();
	}
	set isDisabled($$value) {
		return this.#isDisabled($$value);
	}
	attachment;
	contentNode = null;
	contentPresence;
	constructor(opts) {
		this.opts = opts;
		this.root = opts.rootState;
		this.updateValue = this.updateValue.bind(this);
		this.attachment = attachRef(this.opts.ref);
		this.contentPresence = new PresenceManager({
			ref: boxWith(() => this.contentNode),
			open: boxWith(() => this.isActive)
		});
	}
	updateValue() {
		this.root.toggleItem(this.opts.value.current);
	}
	#props = derived(() => ({
		id: this.opts.id.current,
		"data-state": getDataOpenClosed(this.isActive),
		"data-disabled": boolToEmptyStrOrUndef(this.isDisabled),
		"data-orientation": this.root.opts.orientation.current,
		[accordionAttrs.item]: "",
		...this.attachment
	}));
	get props() {
		return this.#props();
	}
	set props($$value) {
		return this.#props($$value);
	}
};
var AccordionTriggerState = class AccordionTriggerState {
	opts;
	itemState;
	#root;
	#isDisabled = derived(() => this.opts.disabled.current || this.itemState.opts.disabled.current || this.#root.opts.disabled.current);
	attachment;
	constructor(opts, itemState) {
		this.opts = opts;
		this.itemState = itemState;
		this.#root = itemState.root;
		this.onclick = this.onclick.bind(this);
		this.onkeydown = this.onkeydown.bind(this);
		this.attachment = attachRef(this.opts.ref);
	}
	static create(props) {
		return new AccordionTriggerState(props, AccordionItemContext.get());
	}
	onclick(e) {
		if (this.#isDisabled() || e.button !== 0) {
			e.preventDefault();
			return;
		}
		this.itemState.updateValue();
	}
	onkeydown(e) {
		if (this.#isDisabled()) return;
		if (e.key === " " || e.key === "Enter") {
			e.preventDefault();
			this.itemState.updateValue();
			return;
		}
		this.#root.rovingFocusGroup.handleKeydown(this.opts.ref.current, e);
	}
	#props = derived(() => ({
		id: this.opts.id.current,
		disabled: this.#isDisabled(),
		"aria-expanded": boolToStr(this.itemState.isActive),
		"aria-disabled": boolToStr(this.#isDisabled()),
		"data-disabled": boolToEmptyStrOrUndef(this.#isDisabled()),
		"data-state": getDataOpenClosed(this.itemState.isActive),
		"data-orientation": this.#root.opts.orientation.current,
		[accordionAttrs.trigger]: "",
		tabindex: this.opts.tabindex.current,
		onclick: this.onclick,
		onkeydown: this.onkeydown,
		...this.attachment
	}));
	get props() {
		return this.#props();
	}
	set props($$value) {
		return this.#props($$value);
	}
};
var AccordionContentState = class AccordionContentState {
	opts;
	item;
	attachment;
	#originalStyles = void 0;
	#isMountAnimationPrevented = false;
	#dimensions = {
		width: 0,
		height: 0
	};
	#open = derived(() => {
		if (this.opts.hiddenUntilFound.current) return this.item.isActive;
		return this.opts.forceMount.current || this.item.isActive;
	});
	get open() {
		return this.#open();
	}
	set open($$value) {
		return this.#open($$value);
	}
	constructor(opts, item) {
		this.opts = opts;
		this.item = item;
		this.#isMountAnimationPrevented = this.item.isActive;
		this.attachment = attachRef(this.opts.ref, (v) => this.item.contentNode = v);
		watch.pre([() => this.opts.ref.current, () => this.opts.hiddenUntilFound.current], ([node, hiddenUntilFound]) => {
			if (!node || !hiddenUntilFound) return;
			const handleBeforeMatch = () => {
				if (this.item.isActive) return;
				requestAnimationFrame(() => {
					this.item.updateValue();
				});
			};
			return on(node, "beforematch", handleBeforeMatch);
		});
		watch([() => this.open, () => this.opts.ref.current], this.#updateDimensions);
	}
	static create(props) {
		return new AccordionContentState(props, AccordionItemContext.get());
	}
	#updateDimensions = ([_, node]) => {
		if (!node) return;
		afterTick(() => {
			const element = this.opts.ref.current;
			if (!element) return;
			this.#originalStyles ??= {
				transitionDuration: element.style.transitionDuration,
				animationName: element.style.animationName
			};
			element.style.transitionDuration = "0s";
			element.style.animationName = "none";
			const rect = element.getBoundingClientRect();
			this.#dimensions = {
				width: rect.width,
				height: rect.height
			};
			if (!this.#isMountAnimationPrevented && this.#originalStyles) {
				element.style.transitionDuration = this.#originalStyles.transitionDuration;
				element.style.animationName = this.#originalStyles.animationName;
			}
		});
	};
	get shouldRender() {
		return this.item.contentPresence.shouldRender;
	}
	#snippetProps = derived(() => ({ open: this.item.isActive }));
	get snippetProps() {
		return this.#snippetProps();
	}
	set snippetProps($$value) {
		return this.#snippetProps($$value);
	}
	#props = derived(() => ({
		id: this.opts.id.current,
		"data-state": getDataOpenClosed(this.item.isActive),
		...getDataTransitionAttrs(this.item.contentPresence.transitionStatus),
		"data-disabled": boolToEmptyStrOrUndef(this.item.isDisabled),
		"data-orientation": this.item.root.opts.orientation.current,
		[accordionAttrs.content]: "",
		style: {
			"--bits-accordion-content-height": `${this.#dimensions.height}px`,
			"--bits-accordion-content-width": `${this.#dimensions.width}px`
		},
		hidden: this.opts.hiddenUntilFound.current && !this.item.isActive ? "until-found" : void 0,
		...this.opts.hiddenUntilFound.current && !this.shouldRender ? {} : { hidden: this.opts.hiddenUntilFound.current ? !this.shouldRender : this.opts.forceMount.current ? void 0 : !this.shouldRender },
		...this.attachment
	}));
	get props() {
		return this.#props();
	}
	set props($$value) {
		return this.#props($$value);
	}
};
var AccordionHeaderState = class AccordionHeaderState {
	opts;
	item;
	attachment;
	constructor(opts, item) {
		this.opts = opts;
		this.item = item;
		this.attachment = attachRef(this.opts.ref);
	}
	static create(props) {
		return new AccordionHeaderState(props, AccordionItemContext.get());
	}
	#props = derived(() => ({
		id: this.opts.id.current,
		role: "heading",
		"aria-level": this.opts.level.current,
		"data-heading-level": this.opts.level.current,
		"data-state": getDataOpenClosed(this.item.isActive),
		"data-orientation": this.item.root.opts.orientation.current,
		[accordionAttrs.header]: "",
		...this.attachment
	}));
	get props() {
		return this.#props();
	}
	set props($$value) {
		return this.#props($$value);
	}
};
//#endregion
//#region node_modules/bits-ui/dist/internal/noop.js
/**
* A no operation function (does nothing)
*/
function noop() {}
//#endregion
//#region node_modules/bits-ui/dist/internal/create-id.js
function createId(prefixOrUid, uid) {
	if (uid === void 0) return `bits-${prefixOrUid}`;
	return `bits-${prefixOrUid}-${uid}`;
}
//#endregion
//#region node_modules/bits-ui/dist/bits/accordion/components/accordion.svelte
function Accordion($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const uid = props_id($$renderer);
		let { disabled = false, children, child, type, value = void 0, ref = null, id = createId(uid), onValueChange = noop, loop = true, orientation = "vertical", $$slots, $$events, ...restProps } = $$props;
		function handleDefaultValue() {
			if (value !== void 0) return;
			value = type === "single" ? "" : [];
		}
		handleDefaultValue();
		watch.pre(() => value, () => {
			handleDefaultValue();
		});
		const rootState = AccordionRootState.create({
			type,
			value: boxWith(() => value, (v) => {
				value = v;
				onValueChange(v);
			}),
			id: boxWith(() => id),
			disabled: boxWith(() => disabled),
			loop: boxWith(() => loop),
			orientation: boxWith(() => orientation),
			ref: boxWith(() => ref, (v) => ref = v)
		});
		const mergedProps = derived(() => mergeProps(restProps, rootState.props));
		if (child) {
			$$renderer.push("<!--[0-->");
			child($$renderer, { props: mergedProps() });
			$$renderer.push(`<!---->`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div${attributes({ ...mergedProps() })}>`);
			children?.($$renderer);
			$$renderer.push(`<!----></div>`);
		}
		$$renderer.push(`<!--]-->`);
		bind_props($$props, {
			value,
			ref
		});
	});
}
//#endregion
//#region node_modules/bits-ui/dist/bits/accordion/components/accordion-item.svelte
function Accordion_item$1($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const defaultId = createId(props_id($$renderer));
		let { id = defaultId, disabled = false, value = defaultId, children, child, ref = null, $$slots, $$events, ...restProps } = $$props;
		const itemState = AccordionItemState.create({
			value: boxWith(() => value),
			disabled: boxWith(() => disabled),
			id: boxWith(() => id),
			ref: boxWith(() => ref, (v) => ref = v)
		});
		const mergedProps = derived(() => mergeProps(restProps, itemState.props));
		if (child) {
			$$renderer.push("<!--[0-->");
			child($$renderer, { props: mergedProps() });
			$$renderer.push(`<!---->`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div${attributes({ ...mergedProps() })}>`);
			children?.($$renderer);
			$$renderer.push(`<!----></div>`);
		}
		$$renderer.push(`<!--]-->`);
		bind_props($$props, { ref });
	});
}
//#endregion
//#region node_modules/bits-ui/dist/bits/accordion/components/accordion-header.svelte
function Accordion_header($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const uid = props_id($$renderer);
		let { id = createId(uid), level = 2, children, child, ref = null, $$slots, $$events, ...restProps } = $$props;
		const headerState = AccordionHeaderState.create({
			id: boxWith(() => id),
			level: boxWith(() => level),
			ref: boxWith(() => ref, (v) => ref = v)
		});
		const mergedProps = derived(() => mergeProps(restProps, headerState.props));
		if (child) {
			$$renderer.push("<!--[0-->");
			child($$renderer, { props: mergedProps() });
			$$renderer.push(`<!---->`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div${attributes({ ...mergedProps() })}>`);
			children?.($$renderer);
			$$renderer.push(`<!----></div>`);
		}
		$$renderer.push(`<!--]-->`);
		bind_props($$props, { ref });
	});
}
//#endregion
//#region node_modules/bits-ui/dist/bits/accordion/components/accordion-trigger.svelte
function Accordion_trigger$1($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const uid = props_id($$renderer);
		let { disabled = false, ref = null, id = createId(uid), tabindex = 0, children, child, $$slots, $$events, ...restProps } = $$props;
		const triggerState = AccordionTriggerState.create({
			disabled: boxWith(() => disabled),
			id: boxWith(() => id),
			tabindex: boxWith(() => tabindex ?? 0),
			ref: boxWith(() => ref, (v) => ref = v)
		});
		const mergedProps = derived(() => mergeProps(restProps, triggerState.props));
		if (child) {
			$$renderer.push("<!--[0-->");
			child($$renderer, { props: mergedProps() });
			$$renderer.push(`<!---->`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<button${attributes({
				type: "button",
				...mergedProps()
			})}>`);
			children?.($$renderer);
			$$renderer.push(`<!----></button>`);
		}
		$$renderer.push(`<!--]-->`);
		bind_props($$props, { ref });
	});
}
//#endregion
//#region node_modules/bits-ui/dist/bits/accordion/components/accordion-content.svelte
function Accordion_content$1($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const uid = props_id($$renderer);
		let { child, ref = null, id = createId(uid), forceMount = false, children, hiddenUntilFound = false, $$slots, $$events, ...restProps } = $$props;
		const contentState = AccordionContentState.create({
			forceMount: boxWith(() => forceMount),
			id: boxWith(() => id),
			ref: boxWith(() => ref, (v) => ref = v),
			hiddenUntilFound: boxWith(() => hiddenUntilFound)
		});
		const mergedProps = derived(() => mergeProps(restProps, contentState.props));
		if (child) {
			$$renderer.push("<!--[0-->");
			child($$renderer, {
				props: mergedProps(),
				...contentState.snippetProps
			});
			$$renderer.push(`<!---->`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<div${attributes({ ...mergedProps() })}>`);
			children?.($$renderer);
			$$renderer.push(`<!----></div>`);
		}
		$$renderer.push(`<!--]-->`);
		bind_props($$props, { ref });
	});
}
//#endregion
//#region src/lib/components/ui/accordion/accordion.svelte
function Accordion_1($$renderer, $$props) {
	let { children, $$slots, $$events, ...rest } = $$props;
	if (Accordion) {
		$$renderer.push("<!--[-->");
		Accordion($$renderer, spread_props([
			{ class: "w-full" },
			rest,
			{
				children: ($$renderer) => {
					children($$renderer);
					$$renderer.push(`<!---->`);
				},
				$$slots: { default: true }
			}
		]));
		$$renderer.push("<!--]-->");
	} else {
		$$renderer.push("<!--[!-->");
		$$renderer.push("<!--]-->");
	}
}
//#endregion
//#region src/lib/components/ui/accordion/accordion-item.svelte
function Accordion_item($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { ref = null, class: className = "", children, $$slots, $$events, ...rest } = $$props;
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			if (Accordion_item$1) {
				$$renderer.push("<!--[-->");
				Accordion_item$1($$renderer, spread_props([
					{ class: cn("border-b border-[var(--border)]", className) },
					rest,
					{
						get ref() {
							return ref;
						},
						set ref($$value) {
							ref = $$value;
							$$settled = false;
						},
						children: ($$renderer) => {
							children($$renderer);
							$$renderer.push(`<!---->`);
						},
						$$slots: { default: true }
					}
				]));
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
		bind_props($$props, { ref });
	});
}
//#endregion
//#region src/lib/components/ui/accordion/accordion-trigger.svelte
function Accordion_trigger($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { ref = null, class: className = "", children, $$slots, $$events, ...rest } = $$props;
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			if (Accordion_header) {
				$$renderer.push("<!--[-->");
				Accordion_header($$renderer, spread_props([
					{ class: "flex" },
					rest,
					{
						children: ($$renderer) => {
							if (Accordion_trigger$1) {
								$$renderer.push("<!--[-->");
								Accordion_trigger$1($$renderer, {
									class: cn("flex flex-1 items-center justify-between py-4 font-medium transition-all hover:underline text-[var(--foreground)] [&[data-state=open]>svg]:rotate-180", className),
									get ref() {
										return ref;
									},
									set ref($$value) {
										ref = $$value;
										$$settled = false;
									},
									children: ($$renderer) => {
										children($$renderer);
										$$renderer.push(`<!----> `);
										Chevron_down($$renderer, { class: "h-4 w-4 shrink-0 transition-transform duration-200" });
										$$renderer.push(`<!---->`);
									},
									$$slots: { default: true }
								});
								$$renderer.push("<!--]-->");
							} else {
								$$renderer.push("<!--[!-->");
								$$renderer.push("<!--]-->");
							}
						},
						$$slots: { default: true }
					}
				]));
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
		bind_props($$props, { ref });
	});
}
//#endregion
//#region src/lib/components/ui/accordion/accordion-content.svelte
function Accordion_content($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { ref = null, class: className = "", children, $$slots, $$events, ...rest } = $$props;
		let $$settled = true;
		let $$inner_renderer;
		function $$render_inner($$renderer) {
			if (Accordion_content$1) {
				$$renderer.push("<!--[-->");
				Accordion_content$1($$renderer, spread_props([
					{ class: cn("overflow-hidden text-sm transition-all data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down", className) },
					rest,
					{
						get ref() {
							return ref;
						},
						set ref($$value) {
							ref = $$value;
							$$settled = false;
						},
						children: ($$renderer) => {
							$$renderer.push(`<div class="pb-4 pt-0">`);
							children($$renderer);
							$$renderer.push(`<!----></div>`);
						},
						$$slots: { default: true }
					}
				]));
				$$renderer.push("<!--]-->");
			} else {
				$$renderer.push("<!--[!-->");
				$$renderer.push("<!--]-->");
			}
		}
		do {
			$$settled = true;
			$$inner_renderer = $$renderer.copy();
			$$render_inner($$inner_renderer);
		} while (!$$settled);
		$$renderer.subsume($$inner_renderer);
		bind_props($$props, { ref });
	});
}
//#endregion
//#region src/lib/components/landing/FaqSection.svelte
function FaqSection($$renderer) {
	const faqList = [
		{
			question: "What does \"BYOK\" mean?",
			answer: "Bring Your Own Key — you use your own LLM API key (OpenAI, Anthropic, Groq, Ollama, or Z.AI). cora never charges for AI usage. Your API costs depend on your provider.",
			value: "byok"
		},
		{
			question: "Does cora send my code to third parties?",
			answer: "No. Your code is sent only to the LLM provider you configure. cora itself has no backend, no telemetry, no analytics. Everything runs locally in your terminal.",
			value: "privacy"
		},
		{
			question: "Which LLM providers are supported?",
			answer: "OpenAI (GPT-4o, etc.), Anthropic (Claude), Groq (Llama), Ollama (local models), and Z.AI (GLM). You can also add custom providers with any OpenAI-compatible API.",
			value: "providers"
		},
		{
			question: "How is cora different from GitHub Copilot Code Review?",
			answer: "copa is a CLI tool — it runs in your terminal, not in GitHub. You choose your own LLM provider and model. It works with any git workflow, any hosting platform, and any CI/CD pipeline.",
			value: "comparison"
		},
		{
			question: "Can I use cora in CI/CD?",
			answer: "Yes. cora works in GitHub Actions, GitLab CI, and any CI that supports CLI tools. Use the --staged or --base flags for automated reviews on PRs.",
			value: "ci"
		},
		{
			question: "What languages does cora support?",
			answer: "cora reviews any text-based code. Since it uses LLMs for analysis, it understands virtually all programming languages. The quality of review depends on the model you choose.",
			value: "languages"
		},
		{
			question: "Is cora free?",
			answer: "cora is open source and free to use. You only pay for your own LLM API usage through your provider. With Ollama, you can run it completely free with local models.",
			value: "pricing"
		},
		{
			question: "How do I configure review rules?",
			answer: "Create a .cora.yaml file in your project root. You can configure focus areas, ignore patterns, severity thresholds, and hook settings. See the Configuration docs for details.",
			value: "config"
		}
	];
	$$renderer.push(`<section class="section section-compact"><h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">Frequently asked questions</h2> <p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)] max-w-[32rem] mx-auto">Everything you need to know about using cora.</p> <div class="max-w-2xl mx-auto mt-10 scroll-reveal">`);
	Accordion_1($$renderer, {
		type: "single",
		collapsible: true,
		children: ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			const each_array = ensure_array_like(faqList);
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let item = each_array[$$index];
				Accordion_item($$renderer, {
					value: item.value,
					children: ($$renderer) => {
						Accordion_trigger($$renderer, {
							class: "text-left text-[var(--foreground)] hover:no-underline",
							children: ($$renderer) => {
								$$renderer.push(`<!---->${escape_html(item.question)}`);
							},
							$$slots: { default: true }
						});
						$$renderer.push(`<!----> `);
						Accordion_content($$renderer, {
							class: "text-[var(--muted-foreground)]",
							children: ($$renderer) => {
								$$renderer.push(`<!---->${escape_html(item.answer)}`);
							},
							$$slots: { default: true }
						});
						$$renderer.push(`<!---->`);
					},
					$$slots: { default: true }
				});
			}
			$$renderer.push(`<!--]-->`);
		},
		$$slots: { default: true }
	});
	$$renderer.push(`<!----></div></section>`);
}
//#endregion
//#region src/lib/components/landing/TimelineStep.svelte
function TimelineStep($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { number, title, description, terminalCode, descriptionClass = "text-[var(--muted-foreground)]", class: className, delayMs = 0 } = $$props;
		$$renderer.push(`<div${attr_class(clsx$1(cn("timeline-step scroll-reveal", className)))}${attr_style("", { "transition-delay": delayMs ? `${delayMs}ms` : void 0 })}><div class="timeline-number">${escape_html(number)}</div> <div><h3 class="text-lg font-semibold text-[var(--foreground)] -tracking-tight leading-snug">${escape_html(title)}</h3> <p${attr_class(clsx$1(cn("mt-1 mb-4 text-sm", descriptionClass)))}>${escape_html(description)}</p> `);
		if (terminalCode) {
			$$renderer.push("<!--[0-->");
			terminalCode($$renderer);
			$$renderer.push(`<!---->`);
		} else $$renderer.push("<!--[-1-->");
		$$renderer.push(`<!--]--></div></div>`);
	});
}
//#endregion
//#region src/lib/components/landing/QuickStart.svelte
function QuickStart($$renderer) {
	$$renderer.push(`<section class="section section-tall" id="quick-start"><h2 class="text-center scroll-reveal text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">Start in 30 seconds</h2> <p class="text-center mt-4 scroll-reveal text-sm text-[var(--muted-foreground)]">No account required. No subscription. No cloud.</p> <div class="flex flex-col mt-10 max-w-[40rem] mx-auto gap-6">`);
	{
		function terminalCode($$renderer) {
			TerminalBlock($$renderer, {
				children: ($$renderer) => {
					$$renderer.push(`<span class="syntax-cmd">$</span> <span class="syntax-highlight">curl -fsSL</span> <span class="syntax-string">https://cora.dev/install</span> <span class="syntax-highlight">|</span> <span class="syntax-string">sh</span>`);
				},
				$$slots: { default: true }
			});
		}
		TimelineStep($$renderer, {
			number: 1,
			title: "Install",
			description: "Single binary, no dependencies.",
			terminalCode,
			$$slots: { terminalCode: true }
		});
	}
	$$renderer.push(`<!----> `);
	{
		function terminalCode($$renderer) {
			TerminalBlock($$renderer, {
				children: ($$renderer) => {
					$$renderer.push(`<span class="syntax-cmd">$</span> <span class="syntax-highlight">export</span> <span class="syntax-flag">OPENAI_API_KEY</span>=<span class="syntax-string">"sk-..."</span>`);
				},
				$$slots: { default: true }
			});
		}
		TimelineStep($$renderer, {
			number: 2,
			title: "Set API key",
			description: "Use your existing OpenAI or Anthropic key.",
			delayMs: 100,
			terminalCode,
			$$slots: { terminalCode: true }
		});
	}
	$$renderer.push(`<!----> `);
	{
		function terminalCode($$renderer) {
			TerminalBlock($$renderer, {
				children: ($$renderer) => {
					$$renderer.push(`<span class="syntax-cmd">$</span> <span class="syntax-flag">CORA_API_KEY</span>=<span class="syntax-string">key</span> <span class="syntax-highlight">cora review</span> <span class="syntax-flag">--staged</span>`);
				},
				$$slots: { default: true }
			});
		}
		TimelineStep($$renderer, {
			number: 3,
			title: "Review",
			description: "Review your staged changes.",
			delayMs: 200,
			terminalCode,
			$$slots: { terminalCode: true }
		});
	}
	$$renderer.push(`<!----> `);
	TimelineStep($$renderer, {
		number: 4,
		title: "Done",
		description: "That's it. No account. No subscription.",
		descriptionClass: "text-[var(--success)]",
		delayMs: 300
	});
	$$renderer.push(`<!----></div> <div class="text-center mt-20 scroll-reveal"><h2 class="text-2xl md:text-3xl font-bold text-[var(--foreground)] -tracking-tight leading-tight">Ready to ship better code?</h2> <p class="mt-3 text-sm text-[var(--muted-foreground)]">No account. No subscription. No cloud.</p> <div class="flex flex-wrap justify-center gap-4 items-center mt-8"><a href="/docs" class="btn-primary">`);
	Arrow_right($$renderer, { size: 18 });
	$$renderer.push(`<!----> Get Started</a> <a href="https://github.com/codecoradev/cora-cli" target="_blank" rel="noopener" class="btn-ghost">`);
	Star($$renderer, { size: 18 });
	$$renderer.push(`<!----> Star on GitHub</a></div></div> <footer class="border-t border-[var(--border)] mt-20 py-8"><div class="max-w-6xl mx-auto flex flex-col md:flex-row items-center justify-between gap-4"><div><span class="text-sm font-semibold text-[var(--foreground)]">cora</span> <span class="text-xs text-[var(--muted-foreground)] ml-3">MIT License</span></div> <div class="flex items-center gap-6"><a href="https://github.com/codecoradev/cora-cli" target="_blank" rel="noopener" class="text-sm text-[var(--muted-foreground)] no-underline transition-colors duration-200 min-h-11 inline-flex items-center hover:text-[var(--foreground)]">GitHub</a> <a href="/docs" class="text-sm text-[var(--muted-foreground)] no-underline transition-colors duration-200 min-h-11 inline-flex items-center hover:text-[var(--foreground)]">Docs</a></div></div></footer></section>`);
}
//#endregion
//#region src/routes/+page.svelte
function _page($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		head("1uha8ag", $$renderer, ($$renderer) => {
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>cora — AI Code Review CLI</title>`);
			});
			$$renderer.push(`<meta name="description" content="cora is a CLI-first AI code reviewer. BYOK, zero config, runs in your terminal. Your code never leaves your machine."/>`);
		});
		$$renderer.push(`<div class="bg-[var(--background)]">`);
		HeroSection($$renderer, {});
		$$renderer.push(`<!----> `);
		KpiStats($$renderer, {});
		$$renderer.push(`<!----> `);
		LiveDemo($$renderer, {});
		$$renderer.push(`<!----> `);
		HowItWorks($$renderer, {});
		$$renderer.push(`<!----> `);
		FeatureGrid($$renderer, {});
		$$renderer.push(`<!----> `);
		ComparisonTable($$renderer, {});
		$$renderer.push(`<!----> `);
		FaqSection($$renderer, {});
		$$renderer.push(`<!----> `);
		QuickStart($$renderer, {});
		$$renderer.push(`<!----></div>`);
	});
}
//#endregion
export { _page as default };
