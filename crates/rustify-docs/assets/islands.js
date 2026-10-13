import { boot } from "./dom-loader.js";

const coordinator = window.__rustify_docs_islands;

function hydrationGlobals() {
  for (const name of ["__RESOLVED_RESOURCES", "__INCOMPLETE_CHUNKS", "__SERIALIZED_ERRORS"]) {
    if (globalThis[name] === undefined) globalThis[name] = [];
    if (!Array.isArray(globalThis[name]) || globalThis[name].length !== 0) {
      throw new Error(`unsupported initial hydration data: ${name}`);
    }
  }
}

function attached(block) {
  const metadata = JSON.parse(block.getAttribute("data-rustify-island-metadata"));
  const canvas = block.querySelector("fieldset[data-rd-island-controls]");
  const root = canvas?.firstElementChild;
  if (!root || root.getAttribute("data-rustify-mounted") !== metadata.seed) {
    throw new Error(`island did not attach: ${metadata.key}`);
  }
  if (block.getAttribute("data-rd-island-client-only") === "true") {
    const host = root.querySelector("[data-rustify-client-host]");
    if (!host || host.getAttribute("data-rustify-scope") !== `${metadata.seed}-client`
        || !host.querySelector("[data-rustify-content]") || !host.querySelector("[data-rustify-overlay]")
        || root.querySelector("[data-rustify-client-message]")) {
      throw new Error(`client-only example did not mount: ${metadata.key}`);
    }
  }
}

try {
  if (!coordinator) throw new Error("island input coordinator is missing");
  const blocks = await coordinator.parsed;
  if (blocks.length && !coordinator.terminal()) {
    hydrationGlobals();
    const started = await boot({
      glue_url: new URL("./app.js", import.meta.url).href,
      wasm_url: new URL("./app_bg.wasm", import.meta.url).href,
      on_fatal: (error) => coordinator.fail(error),
    });
    if (coordinator.bind(started)) {
      if (typeof started.app.docs_hydrate !== "function") throw new Error("island attachment entry is missing");
      await started.app.docs_hydrate();
      await new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve)));
      if (!coordinator.terminal()) {
        for (const block of blocks) attached(block);
        coordinator.complete();
      }
    }
  }
} catch (error) {
  if (coordinator) coordinator.fail(error);
  else console.error("Interactive examples could not start", error);
}
