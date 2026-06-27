import { $ as attr, c as ensure_array_like, h as unsubscribe_stores, l as head, n as attr_class, p as store_get, tt as escape_html } from "../../../chunks/dev.js";
import { t as page } from "../../../chunks/stores.js";
//#region src/routes/docs/+layout.svelte
function _layout($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let { children } = $$props;
		const navLinks = [
			{
				href: "/docs/getting-started",
				label: "Getting Started"
			},
			{
				href: "/docs/installation",
				label: "Installation"
			},
			{
				href: "/docs/usage",
				label: "Usage"
			},
			{
				href: "/docs/configuration",
				label: "Configuration"
			},
			{
				href: "/docs/providers",
				label: "Providers"
			},
			{
				href: "/docs/examples",
				label: "Examples"
			},
			{
				href: "/docs/roadmap",
				label: "Roadmap"
			},
			{
				href: "/docs/cli-reference",
				label: "CLI Reference"
			},
			{
				href: "/docs/changelog",
				label: "Changelog"
			}
		];
		head("1bpnej", $$renderer, ($$renderer) => {
			$$renderer.title(($$renderer) => {
				$$renderer.push(`<title>Docs — cora</title>`);
			});
		});
		$$renderer.push(`<div class="min-h-screen flex bg-[var(--background)]"><aside class="docs-sidebar hidden lg:block"><div class="p-6"><a href="/" class="flex items-center gap-2 mb-8 text-sm text-[var(--muted-foreground)] no-underline transition-colors duration-200 min-h-11 hover:text-[var(--foreground)]"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 12H5M12 19l-7-7 7-7"></path></svg> Back to Home</a> <div class="mb-6"><a href="/docs" class="text-lg font-semibold text-[var(--accent)] no-underline -tracking-tight leading-tight">cora docs</a></div> <nav class="flex flex-col gap-1"><!--[-->`);
		const each_array = ensure_array_like(navLinks);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let link = each_array[$$index];
			$$renderer.push(`<a${attr("href", link.href)}${attr_class("docs-sidebar-link", void 0, { "active": store_get($$store_subs ??= {}, "$page", page).url.pathname === link.href })}>${escape_html(link.label)}</a>`);
		}
		$$renderer.push(`<!--]--></nav></div></aside> <nav class="mobile-docs-nav"><a href="/" class="text-[var(--muted-foreground)]" aria-label="Back to Home"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 12H5M12 19l-7-7 7-7"></path></svg></a> <!--[-->`);
		const each_array_1 = ensure_array_like(navLinks);
		for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
			let link = each_array_1[$$index_1];
			$$renderer.push(`<a${attr("href", link.href)}${attr_class("", void 0, { "active": store_get($$store_subs ??= {}, "$page", page).url.pathname === link.href })}>${escape_html(link.label)}</a>`);
		}
		$$renderer.push(`<!--]--></nav> <main class="flex-1 min-w-0"><div class="max-w-3xl mx-auto px-8 py-12">`);
		children($$renderer);
		$$renderer.push(`<!----></div></main></div>`);
		if ($$store_subs) unsubscribe_stores($$store_subs);
	});
}
//#endregion
export { _layout as default };
