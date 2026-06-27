import "../../../chunks/dev.js";
import { t as goto } from "../../../chunks/client.js";
//#region src/routes/docs/+page.svelte
function _page($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		goto("/docs/getting-started", { replaceState: true });
	});
}
//#endregion
export { _page as default };
