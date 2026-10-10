// Starts a Rustify UI application that draws no GPU regions.
//
// `boot()` instantiates one wasm instance from the wasm-bindgen output the
// page names, checks that the instance speaks the runtime protocol this
// loader was written for, creates its host and starts it. Only then does the
// page call the application's own start: a scope mounted without a host has
// nothing to end its listeners when the instance fails, so the SDK refuses one.
//
// Written out by the SDK itself (`rustify_ui::runtime::DOM_LOADER_JS`), so it
// is always the release of the Rust it boots.

/// The protocol of the hooks this loader hands over. The SDK's own constant
/// is compared with it before anything is created.
const PROTOCOL = 1;

/// How many times one slot may be restarted.
///
/// Every restart evaluates a fresh copy of the generated glue under a new URL,
/// and a module record lives as long as the document: the dead instance's
/// linear memory is never returned. Restarting is therefore bounded, and the
/// page reload is what is left after that.
const RESTART_LIMIT = 3;

/// What an application must export for this loader to start it. All three
/// come from the SDK; a build without them is not a DOM build of it.
const REQUIRED = ["rustify_runtime_protocol", "rustify_dom_create_host", "rustify_runtime_boot"];

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

/// How many stack frames a trap keeps, at least.
///
/// The one thing in an uncaught trap that names its instance is the glue frame
/// the browser called into wasm through, and V8 keeps ten frames by default.
/// A panic spends eight of them between the handler and the abort, and every
/// instance of one application shares its wasm URL, so a handler a few calls
/// deep traps with a stack that names no instance at all. Raised, never
/// lowered, and only where the engine has the setting.
const STACK_FRAMES = 200;
if (typeof Error.stackTraceLimit === "number" && Error.stackTraceLimit < STACK_FRAMES) {
    Error.stackTraceLimit = STACK_FRAMES;
}

/// Compiled modules by wasm URL. A second instance of the same application
/// shares the compilation; what it does not share is linear memory.
const modules = new Map();
let next_instance = 1;

function compile(wasm_url) {
    const key = String(wasm_url);
    let module = modules.get(key);
    if (module === undefined) {
        module = WebAssembly.compileStreaming(fetch(wasm_url));
        // A failed fetch must not stay cached: the next boot asks again.
        module.catch(() => modules.delete(key));
        modules.set(key, module);
    }
    return module;
}

/// The glue's URL for `instance`.
///
/// The glue holds its instance in a module-level binding and its initialiser
/// returns early when that binding is set, so a second `init` of the same
/// module record is a no-op rather than a second instance. A different URL is
/// a different module record; the query string is the only part that differs,
/// so every copy is the same file to the HTTP cache.
function glue_for(glue_url, instance) {
    const url = new URL(glue_url);
    if (instance !== 1) {
        url.searchParams.set("instance", String(instance));
    }
    return url.href;
}

/// Wraps the application's exports so that a trap inside one is seen by the
/// instance it happened in, and so that a dead instance is never called.
///
/// An `Err` returned by a Rust function arrives as an ordinary exception and
/// is not a trap.
function call_boundary(glue, instance) {
    const wrapped = {};
    for (const name of Object.keys(glue)) {
        const value = glue[name];
        if (typeof value !== "function" || name === "default") {
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

/// Loads one instance of the application and starts its runtime.
///
/// `glue_url` and `wasm_url` are the JS and wasm wasm-bindgen wrote
/// (`--target web`); relative ones resolve against this loader, which is
/// deployed beside them. Resolves to the application's exports once the
/// runtime has started. `on_fatal` is called if that instance traps; every
/// scope in it is dead by then, and the other instances on the page are not
/// affected.
export async function boot({ glue_url, wasm_url, on_fatal } = {}) {
    if (!glue_url || !wasm_url) {
        throw new StartupError("InvalidBootOptions", "boot() needs both glue_url and wasm_url");
    }
    const number = next_instance;
    next_instance += 1;
    const glue = new URL(glue_url, import.meta.url).href;
    const wasm = new URL(wasm_url, import.meta.url).href;
    return start(glue, wasm, on_fatal, number, 0);
}

async function start(glue_url, wasm_url, on_fatal, number, restarts) {
    const module = await compile(wasm_url);
    const url = glue_for(glue_url, number);
    const glue = await import(url);
    await glue.default({ module_or_path: module });

    const missing = REQUIRED.filter((name) => typeof glue[name] !== "function");
    if (missing.length > 0) {
        throw new StartupError(
            "BuildContractMismatch",
            `${missing.join(", ")} missing from the wasm: build it with the Rustify UI SDK, without the gpu feature`
        );
    }
    const protocol = glue.rustify_runtime_protocol();
    if (protocol !== PROTOCOL) {
        throw new StartupError(
            "BuildContractMismatch",
            `this loader speaks runtime protocol ${PROTOCOL} and the wasm ${protocol}; deploy both from one build`
        );
    }

    const hooks = glue.rustify_dom_create_host(number);
    const instance = { number, fatal: null, hooks, restarts };
    const refuse = (error) => {
        hooks.runtime.release();
        const refused = new StartupError("BuildContractMismatch", error?.message ?? String(error));
        // What the page can check: nothing of the refused instance is left.
        refused.signal = hooks.signal;
        return refused;
    };
    if (hooks.protocol !== PROTOCOL || hooks.instance !== number) {
        throw refuse(new Error(`the host was created for protocol ${hooks.protocol}, instance ${hooks.instance}`));
    }
    hooks.runtime.on_fatal = (error) => {
        instance.fatal = error;
        if (on_fatal) {
            on_fatal(error);
        } else {
            console.error(error);
        }
    };

    // A trap inside a DOM event handler leaves wasm through the browser's own
    // dispatch, so no code of ours is on the stack to catch it. What is left
    // is the uncaught error, and the only thing in it that names an instance
    // is the glue frame in its stack: each instance has its own glue URL.
    const belongs = (error) =>
        error instanceof WebAssembly.RuntimeError && String(error.stack ?? "").includes(`${url}:`);
    const listening = { signal: hooks.signal };
    window.addEventListener(
        "error",
        (event) => {
            if (!instance.fatal && belongs(event.error)) {
                hooks.runtime.enter_fatal(event.error);
            }
        },
        listening
    );
    window.addEventListener(
        "unhandledrejection",
        (event) => {
            if (!instance.fatal && belongs(event.reason)) {
                hooks.runtime.enter_fatal(event.reason);
            }
        },
        listening
    );

    try {
        glue.rustify_runtime_boot(hooks);
    } catch (error) {
        throw refuse(error);
    }

    return {
        app: call_boundary(glue, instance),
        hooks,
        instance: number,
        glue_url: url,
        /// Aborted when this instance fails. Page-level listeners registered
        /// with it are gone by the time `on_fatal` is called.
        signal: hooks.signal,
        restarts,
        /// A fresh instance in place of this one, or `null` once the slot has
        /// been restarted as often as it may be. The dead instance's memory
        /// stays with the document either way.
        restart({ on_fatal: next_fatal } = {}) {
            if (restarts >= RESTART_LIMIT) {
                return null;
            }
            const next = next_instance;
            next_instance += 1;
            return start(glue_url, wasm_url, next_fatal ?? on_fatal, next, restarts + 1);
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
    container.removeAttribute("data-rustify-theme-owner");
    container.removeAttribute("data-rustify-theme-runtime");
    container.removeAttribute("tabindex");
}

/// Shows a failure notice inside `container`, and the one thing a person can
/// still do about it.
///
/// Built from nothing but the DOM: it is shown when the runtime could not
/// start, or has just died, so it must not depend on wasm being alive. When
/// `restart` is given the notice offers it - the caller decides whether the
/// slot has a restart left, because the loader refuses one it does not. The
/// text says what was lost; it never says anything was saved.
export function show_fatal(container, error, { restart } = {}) {
    const box = document.createElement("div");
    box.className = "rustify-fatal";
    box.setAttribute("role", "alert");
    const kind = error && error.kind ? error.kind : "RuntimeFatal";
    const text = document.createElement("p");
    text.dataset.testid = "fatal-text";
    text.textContent = `${kind}: ${error && error.message ? error.message : String(error)}`;
    box.append(text);
    if (typeof restart === "function") {
        const button = document.createElement("button");
        button.type = "button";
        button.dataset.testid = "fatal-restart";
        button.textContent = "restart this instance";
        // Once: a second press while the first restart is still loading would
        // spend a restart of the slot on an instance nobody asked for.
        button.addEventListener("click", () => {
            button.disabled = true;
            restart();
        });
        box.append(button);
    }
    const reload = document.createElement("button");
    reload.type = "button";
    reload.dataset.testid = "fatal-reload";
    reload.textContent = "reload the page";
    reload.addEventListener("click", () => window.location.reload());
    box.append(reload);
    container.replaceChildren(box);
}
