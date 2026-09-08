import { init_env, ToWasmMsg, FromWasmMsg } from "./makepad_wasm_bridge/wasm_bridge.js";
import init, * as app from "./bindgen.js";
import { create_message_classes, SCHEMA_HASH } from "./rustify_makepad/message_bridge.js";
import { create_host_hooks } from "./rustify_makepad/embedded.js";

export class StartupError extends Error {
    constructor(kind, message) {
        super(message);
        this.kind = kind;
    }
}

// Loads the wasm built next to this file and wires the Makepad host into it.
// Resolves to the application's exported functions once the runtime is ready.
// `on_fatal` is called if the module traps later on; every mount in this
// runtime is dead by then and only a reload brings it back.
export async function boot({ wasm_url, on_fatal }) {
    const env = {};
    const set_wasm = init_env(env);
    const module = await WebAssembly.compileStreaming(fetch(wasm_url));
    const wasm = await init({ module_or_path: module }, env);
    set_wasm(wasm);
    wasm._memory = wasm.exports.memory;
    wasm._module = module;
    wasm._has_thread_support =
        typeof SharedArrayBuffer !== "undefined" && wasm._memory.buffer instanceof SharedArrayBuffer;

    const live_hash = String(wasm.exports.wasm_js_message_bridge_hash());
    if (live_hash !== SCHEMA_HASH) {
        throw new StartupError(
            "BuildContractMismatch",
            `message bridge ${SCHEMA_HASH} does not match wasm ${live_hash}; redeploy matching assets`
        );
    }

    const msg_class = create_message_classes(ToWasmMsg, FromWasmMsg);
    const hooks = create_host_hooks(wasm, msg_class, on_fatal);
    app.rustify_makepad_boot(hooks);
    return { app, wasm, hooks };
}

// Shows a static failure notice inside `container`; used when the runtime
// cannot start at all, so the message never depends on wasm being alive.
export function show_fatal(container, error) {
    const box = document.createElement("div");
    box.className = "rustify-fatal";
    box.setAttribute("role", "alert");
    const kind = error && error.kind ? error.kind : "RuntimeFatal";
    box.textContent = `${kind}: ${error && error.message ? error.message : String(error)}`;
    container.replaceChildren(box);
}
