import { boot } from "./rustify-dom-loader.js";

const status = document.getElementById("startup-status");
try {
    const runtime = await boot({ glue_url: "./pkg/app.js", wasm_url: "./pkg/app_bg.wasm" });
    runtime.app.public_panel_mount("app");
    status.hidden = true;
    document.documentElement.dataset.ready = "true";
} catch (error) {
    status.textContent = `Delivery note could not start: ${error}`;
}
