import init, * as facade from "./worker/style.js";

const ready = init({ module_or_path: new URL("./worker/style_bg.wasm", import.meta.url) });
self.onmessage = async event => {
  const { requestId, candidates } = event.data;
  try {
    await ready;
    self.postMessage({ requestId, candidates: JSON.parse(facade.transport_probe(JSON.stringify(candidates))),
      protocolVersion: 1, scannerEnabled: typeof facade.scan_content === "function",
      sharedMemory: typeof SharedArrayBuffer !== "undefined" });
  } catch (error) {
    self.postMessage({ requestId, error: String(error) });
  }
};
