import { boot } from "./loader.js";

const status = document.getElementById("runtime-status");
try {
  const live = await boot({
    wasm_url: new URL("./theme_export_consumer.wasm", import.meta.url),
    on_fatal(error) {
      status.dataset.status = "failed";
      status.textContent = "Consumer trapped: " + error;
      console.error(error);
    },
  });
  live.app.theme_export_mount();
  window.__theme_export_consumer = live.app;
} catch (error) {
  status.dataset.status = "failed";
  status.textContent = "Consumer failed: " + error;
  console.error(error);
}
