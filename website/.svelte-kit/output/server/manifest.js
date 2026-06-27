export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png","og.png","robots.txt"]),
	mimeTypes: {".png":"image/png",".txt":"text/plain"},
	_: {
		client: {start:"_app/immutable/entry/start.CmvPVbW8.js",app:"_app/immutable/entry/app.DyiyvRxF.js",imports:["_app/immutable/entry/start.CmvPVbW8.js","_app/immutable/chunks/DyVSz01m.js","_app/immutable/chunks/Du04-Wio.js","_app/immutable/entry/app.DyiyvRxF.js","_app/immutable/chunks/Du04-Wio.js","_app/immutable/chunks/kNaey6uv.js","_app/immutable/chunks/xihTtKlq.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js'))
		],
		remotes: {
			
		},
		routes: [
			
		],
		prerendered_routes: new Set(["/","/docs/changelog","/docs/cli-reference","/docs/configuration","/docs/examples","/docs/getting-started","/docs/installation","/docs/providers","/docs/roadmap","/docs/usage"]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
