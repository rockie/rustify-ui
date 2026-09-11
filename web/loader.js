import { init_env, ToWasmMsg, FromWasmMsg } from "./makepad_wasm_bridge/wasm_bridge.js";
import init, * as first_glue from "./bindgen.js";
import { create_message_classes, SCHEMA_HASH } from "./rustify_makepad/message_bridge.js";
import { create_host_hooks } from "./rustify_makepad/embedded.js";

export class StartupError extends Error {
    constructor(kind, message) {
        super(message);
        this.kind = kind;
    }
}

/// Thrown when an export is called on an instance that has already trapped.
/// Nothing in that module can be trusted afterwards, so the call does not
/// happen and the caller hears why rather than getting a second trap.
export class InstanceDead extends Error {
    constructor(name) {
        super(`${name}: this instance has already failed`);
        this.kind = "InstanceDead";
    }
}

/// How many times one slot may be restarted.
///
/// Every restart evaluates a fresh copy of the generated glue under a new URL,
/// and a module record lives as long as the document: the dead instance's
/// linear memory is never returned. Restarting is therefore bounded, and the
/// page reload is what is left after that.
const RESTART_LIMIT = 3;

/// Compiled modules by wasm URL. A second instance of the same application
/// shares the compilation rather than fetching and compiling the module
/// again; what it does not share is linear memory.
const modules = new Map();
let message_classes = null;
let next_instance = 1;

function compile(wasm_url) {
    const key = String(wasm_url);
    let module = modules.get(key);
    if (module === undefined) {
        module = WebAssembly.compileStreaming(fetch(wasm_url));
        modules.set(key, module);
    }
    return module;
}

/// Evaluates a copy of the generated glue for `instance`.
///
/// The glue holds its instance in a module-level binding and its initialiser
/// returns early when that binding is set, so a second `init` of the same
/// module record is a no-op rather than a second instance. A different URL is
/// a different module record, which is what makes two instances possible at
/// all; the query string is the only part that may differ, so both copies are
/// the same file to the HTTP cache.
async function load_glue(instance) {
    if (instance === 1) {
        return { glue: first_glue, init, url: new URL("./bindgen.js", import.meta.url).href };
    }
    const url = new URL(`./bindgen.js?instance=${instance}`, import.meta.url).href;
    const glue = await import(url);
    return { glue, init: glue.default, url };
}

/// Wraps the application's exports so that a trap inside one is seen by the
/// instance it happened in.
///
/// An export called from the page reaches wasm outside every host entry point:
/// the runtime hears nothing about it, and the caller gets a `RuntimeError`
/// from a module that is now dead but still being used. The boundary reports
/// it first and rethrows, so the caller still sees the failure it caused.
/// An `Err` returned by a Rust function arrives as an ordinary exception and
/// is not a trap.
function call_boundary(glue, instance) {
    const wrapped = {};
    for (const name of Object.keys(glue)) {
        const value = glue[name];
        if (typeof value !== "function") {
            wrapped[name] = value;
            continue;
        }
        wrapped[name] = (...args) => {
            if (instance.fatal || instance.hooks.runtime.fatal) {
                throw new InstanceDead(name);
            }
            try {
                return value(...args);
            } catch (error) {
                if (error instanceof WebAssembly.RuntimeError) {
                    instance.hooks.runtime.enter_fatal(error);
                }
                throw error;
            }
        };
    }
    return wrapped;
}

/// Loads the wasm built next to this file and wires the Makepad host into it.
/// Resolves to the application's exported functions once the runtime is ready.
///
/// One call is one application instance: its own linear memory, its own host
/// hooks and its own diagnostics. `on_fatal` is called if that instance traps;
/// every mount in it is dead by then, and the other instances on the page are
/// not affected.
export async function boot({ wasm_url, on_fatal }) {
    const number = next_instance;
    next_instance += 1;
    return start(wasm_url, on_fatal, number, 0);
}

async function start(wasm_url, on_fatal, number, restarts) {
    // The path this build was made for. It is a property of the build - the
    // page names its own files absolutely under it - so it is read from the
    // build rather than guessed from the address bar, which at a deep link
    // says nothing about where the application starts.
    const manifest_url = new URL("./build-manifest.json", wasm_url);
    const base = await fetch(manifest_url)
        .then((response) => (response.ok ? response.json() : null))
        .then((manifest) => manifest?.base ?? "/")
        .catch(() => "/");
    // The region resolves its resources against this rather than against the
    // address bar, which an application that routes is changing. One page is
    // one deployment, so one global is the right shape for it.
    window.makepad_resource_base = base;
    const env = {};
    const set_wasm = init_env(env);
    const module = await compile(wasm_url);
    const { glue, init: init_glue, url: glue_url } = await load_glue(number);
    const wasm = await init_glue({ module_or_path: module }, env);
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

    message_classes ??= create_message_classes(ToWasmMsg, FromWasmMsg);
    // Page-level resources this instance takes out, so a trap can release
    // them from here: a Rust destructor does not run after one.
    const controller = new AbortController();
    const instance = { number, fatal: null, hooks: null, restarts };
    const hooks = create_host_hooks(wasm, message_classes, (error) => {
        instance.fatal = error;
        controller.abort();
        window.removeEventListener("error", attribute);
        window.removeEventListener("unhandledrejection", attribute_rejection);
        if (on_fatal) {
            on_fatal(error);
        } else {
            console.error(error);
        }
    });
    instance.hooks = hooks;

    // A trap inside a DOM event handler leaves wasm through the browser's own
    // dispatch, so no code of ours is on the stack to catch it. What is left
    // is the uncaught error, and the only thing in it that names an instance
    // is the glue frame in its stack: each instance has its own glue URL.
    const belongs = (error) =>
        error instanceof WebAssembly.RuntimeError && String(error.stack ?? "").includes(`${glue_url}:`);
    const attribute = (event) => {
        if (!instance.fatal && belongs(event.error)) {
            hooks.runtime.enter_fatal(event.error);
        }
    };
    const attribute_rejection = (event) => {
        if (!instance.fatal && belongs(event.reason)) {
            hooks.runtime.enter_fatal(event.reason);
        }
    };
    window.addEventListener("error", attribute);
    window.addEventListener("unhandledrejection", attribute_rejection);

    glue.rustify_makepad_boot(hooks);
    // The build the runtime came from, so a diagnostic can be matched to the
    // source it was produced by. The bridge hash is the one identifier the
    // page and the wasm have already agreed on.
    return {
        app: call_boundary(glue, instance),
        wasm,
        hooks,
        build: SCHEMA_HASH,
        base,
        instance: number,
        glue_url,
        /// Aborted when this instance fails. Page-level listeners registered
        /// with it are gone by the time `on_fatal` is called.
        signal: controller.signal,
        restarts,
        /// A fresh instance in place of this one, or `null` once the slot has
        /// been restarted as often as it may be. The dead instance's memory
        /// stays with the document either way. The notice belongs to the new
        /// instance, so a caller that keeps one per instance passes its own.
        restart({ on_fatal: next_fatal } = {}) {
            if (restarts >= RESTART_LIMIT) {
                return null;
            }
            const next = next_instance;
            next_instance += 1;
            return start(wasm_url, next_fatal ?? on_fatal, next, restarts + 1);
        },
        restart_limit: RESTART_LIMIT,
    };
}

/// Releases a container a dead instance had mounted into.
///
/// The SDK writes its own marks on a container it mounts into and removes them
/// when the scope is disposed - but a trap runs no Rust destructor, so after
/// one the marks are still there and the next instance would be told the
/// container is taken. Nothing in the dead module may be called to tidy this
/// up, so the page's own half does it.
export function release_container(container) {
    if (!container) {
        return;
    }
    container.replaceChildren();
    container.removeAttribute("data-rustify-scope");
    container.removeAttribute("tabindex");
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
