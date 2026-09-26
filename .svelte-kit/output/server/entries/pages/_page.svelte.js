import "../../chunks/index-server.js";
import { _ as escape_html, g as attr, r as ensure_array_like, t as attr_class } from "../../chunks/server.js";
import "@tauri-apps/api/core";
import "@tauri-apps/plugin-notification";
//#region src/routes/+page.svelte
function _page($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const W = 180;
		const H = 320;
		let fps = 0;
		let log = [];
		Array.from({ length: 40 }, (_, i) => [
			i * 67 % W,
			i * 37 % 140,
			i % 3
		]);
		$$renderer.push(`<main class="svelte-1uha8ag"><canvas${attr("width", W)}${attr("height", H)} class="svelte-1uha8ag"></canvas> <div class="hud svelte-1uha8ag"><span${attr_class("fps svelte-1uha8ag", void 0, { "bad": false })}>${escape_html(fps)} fps</span> <label class="svelte-1uha8ag">hour <input type="range" min="0" max="23.9" step="0.5"${attr("value", (/* @__PURE__ */ new Date()).getHours())} class="svelte-1uha8ag"/></label> <div class="row svelte-1uha8ag"><button class="svelte-1uha8ag">Keychain</button> <button class="svelte-1uha8ag">Notify</button></div> <!--[-->`);
		const each_array = ensure_array_like(log);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let line = each_array[$$index];
			$$renderer.push(`<p class="svelte-1uha8ag">${escape_html(line)}</p>`);
		}
		$$renderer.push(`<!--]--></div></main>`);
	});
}
//#endregion
export { _page as default };
