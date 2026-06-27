import * as universal from '../entries/pages/_layout.ts.js';

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.BZeUAe05.js","_app/immutable/chunks/Du04-Wio.js","_app/immutable/chunks/BuBrZOIh.js","_app/immutable/chunks/xihTtKlq.js","_app/immutable/chunks/DozEvg9T.js","_app/immutable/chunks/DyVSz01m.js"];
export const stylesheets = ["_app/immutable/assets/0.D7nPmqCL.css"];
export const fonts = [];
