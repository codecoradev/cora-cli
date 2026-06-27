
// this file is generated — do not edit it


declare module "svelte/elements" {
	export interface HTMLAttributes<T> {
		'data-sveltekit-keepfocus'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-noscroll'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-preload-code'?:
			| true
			| ''
			| 'eager'
			| 'viewport'
			| 'hover'
			| 'tap'
			| 'off'
			| undefined
			| null;
		'data-sveltekit-preload-data'?: true | '' | 'hover' | 'tap' | 'off' | undefined | null;
		'data-sveltekit-reload'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-replacestate'?: true | '' | 'off' | undefined | null;
	}
}

export {};


declare module "$app/types" {
	type MatcherParam<M> = M extends (param : string) => param is (infer U extends string) ? U : string;

	export interface AppTypes {
		RouteId(): "/" | "/docs" | "/docs/changelog" | "/docs/cli-reference" | "/docs/configuration" | "/docs/examples" | "/docs/getting-started" | "/docs/installation" | "/docs/providers" | "/docs/roadmap" | "/docs/usage";
		RouteParams(): {
			
		};
		LayoutParams(): {
			"/": Record<string, never>;
			"/docs": Record<string, never>;
			"/docs/changelog": Record<string, never>;
			"/docs/cli-reference": Record<string, never>;
			"/docs/configuration": Record<string, never>;
			"/docs/examples": Record<string, never>;
			"/docs/getting-started": Record<string, never>;
			"/docs/installation": Record<string, never>;
			"/docs/providers": Record<string, never>;
			"/docs/roadmap": Record<string, never>;
			"/docs/usage": Record<string, never>
		};
		Pathname(): "/" | "/docs" | "/docs/changelog" | "/docs/cli-reference" | "/docs/configuration" | "/docs/examples" | "/docs/getting-started" | "/docs/installation" | "/docs/providers" | "/docs/roadmap" | "/docs/usage";
		ResolvedPathname(): `${"" | `/${string}`}${ReturnType<AppTypes['Pathname']>}`;
		Asset(): "/favicon.png" | "/og.png" | "/robots.txt" | string & {};
	}
}