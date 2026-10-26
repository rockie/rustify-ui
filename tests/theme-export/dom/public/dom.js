import { boot, release_container, show_fatal } from "./rustify-dom-loader.js";

const status = document.getElementById("dom-status");
const roots = ["dom-snippet", "dom-json", "dom-preview-light", "dom-preview-dark"];
const hostStyle = () => {
  const style = getComputedStyle(document.getElementById("dom-host"));
  return [style.fontFamily, style.fontSize, style.color, style.backgroundColor];
};
const before = hostStyle();
try {
  const live = await boot({
    glue_url: new URL("./pkg/app.js", import.meta.url),
    wasm_url: new URL("./pkg/app_bg.wasm", import.meta.url),
    on_fatal(error) {
      roots.forEach(id => release_container(document.getElementById(id)));
      status.dataset.status = "failed";
      show_fatal(status, error);
    },
  });
  live.app.dom_export_mount(new URL("./", import.meta.url).pathname);
  window.__theme_export_dom = { app: live.app, before, hostStyle };
  status.dataset.status = "ready";
  status.textContent = "Mounted the exported Rust and JSON with the default DOM SDK.";
} catch (error) {
  status.dataset.status = "failed";
  show_fatal(status, error);
}
