// The host a DOM-only instance runs under: the hooks the SDK's runtime
// binding calls, and nothing that draws. Shipped as a wasm-bindgen snippet of
// the SDK, so it always comes from the same release as the Rust that calls it.
//
// The module defines a factory and keeps nothing between calls: every
// instance on the page gets a host of its own, and a host that has failed
// takes nothing of another one's with it.

/// The attribute one page-level owner of the address bar writes on the root
/// element, as `<instance>:<scope>`. A trap runs no Rust destructor, so after
/// one the host is what takes it off.
const URL_OWNER_ATTRIBUTE = "data-rustify-url-owner";

export function create_dom_host(instance, protocol) {
    // Browser tasks the SDK asked for, by task id. The host owns them because
    // their callbacks are wasm code: a runtime that has failed drops them
    // instead of letting them re-enter a module nothing can trust.
    const deferred = new Map();
    // Timers for the tasks that asked to wait, so a failure can stop them
    // before they fire.
    const delays = new Map();
    // Size observers, by their release, so a failure disconnects every one.
    const observers = new Set();
    let next_task = 1;

    // Every page-level listener this instance takes out is registered with
    // this signal, on both sides of the boundary: the SDK through its
    // listener options, and the loader for its own. Aborting it is the only
    // removal a trap can perform.
    const controller = new AbortController();

    // A turn of the browser, taken through a message port rather than a timer:
    // nested `setTimeout(…, 0)` is clamped to 4 ms from the fifth level on,
    // and work that yields once a slice would spend its budget on the clamp.
    const turns = new MessageChannel();
    const run_task = (id) => {
        const callback = deferred.get(id);
        if (callback === undefined) {
            return;
        }
        deferred.delete(id);
        delays.delete(id);
        try {
            callback();
        } catch (error) {
            runtime.enter_fatal(error);
        }
    };
    turns.port1.onmessage = (event) => run_task(event.data);

    // Drops everything this host still holds for the instance. Shared by a
    // failure and by a start that was refused, which differ only in whether
    // the page is told.
    const release_all = () => {
        controller.abort();
        const root = document.documentElement;
        const owner = root.getAttribute(URL_OWNER_ATTRIBUTE);
        if (owner !== null && owner.startsWith(`${instance}:`)) {
            root.removeAttribute(URL_OWNER_ATTRIBUTE);
        }
        // Messages already posted still arrive; dropping the callbacks is
        // what keeps them from re-entering the module. A timer has not been
        // posted yet, so it is stopped outright.
        for (const timer of delays.values()) {
            clearTimeout(timer);
        }
        delays.clear();
        deferred.clear();
        for (const release of [...observers]) {
            release();
        }
        observers.clear();
        turns.port1.onmessage = null;
        turns.port1.close();
        turns.port2.close();
    };

    const runtime = {
        fatal: null,
        released: false,
        instance,
        signal: controller.signal,
        /// Called once, with the error that killed the instance, after
        /// everything above has been released. The loader sets it.
        on_fatal: null,
        // Bounded: a runtime that keeps failing must not grow an unbounded log.
        errors: [],
        record_error(message) {
            if (runtime.errors.length < 64) {
                runtime.errors.push(message);
            }
            console.error(`[rustify] ${message}`);
        },
        // What this host currently holds on the browser's side, so a page can
        // compare teardown against the baseline it started from.
        stats() {
            return {
                tasks: deferred.size,
                timers: delays.size,
                observers: observers.size,
                errors: runtime.errors.length,
                aborted: controller.signal.aborted,
            };
        },
        // The order is the contract. The instance is marked dead first, so
        // nothing below can call into it; the listeners go next, because
        // until they are gone an event arriving while the rest runs would
        // re-enter a module that cannot be trusted - and what the instance
        // registered to be released on abort is let go with them. The marks
        // on the page follow, then the tasks, timers and observers, and the
        // page hears last because it is its turn to show something.
        enter_fatal(error) {
            if (runtime.fatal || runtime.released) {
                return;
            }
            runtime.fatal = error;
            release_all();
            if (runtime.on_fatal) {
                runtime.on_fatal(error);
            } else {
                console.error(error);
            }
        },
        // Lets go of a host whose instance never started. Nothing is reported:
        // the loader is about to throw the reason to whoever asked for it.
        release() {
            if (runtime.fatal || runtime.released) {
                return;
            }
            runtime.released = true;
            release_all();
        },
    };

    const dead = () => runtime.fatal !== null || runtime.released;

    return {
        protocol,
        instance,
        runtime,
        signal: controller.signal,
        defer(callback) {
            if (dead()) {
                return;
            }
            const id = next_task;
            next_task += 1;
            deferred.set(id, callback);
            turns.port2.postMessage(id);
        },
        // The same, after a wait, owned by the host for the reason `defer` is.
        defer_after(callback, ms) {
            if (dead()) {
                return;
            }
            const id = next_task;
            next_task += 1;
            deferred.set(id, callback);
            delays.set(id, setTimeout(() => run_task(id), ms));
        },
        observe_resize(element, callback) {
            let active = true;
            const observer = new ResizeObserver(() => {
                if (!active || dead()) {
                    return;
                }
                try {
                    callback();
                } catch (error) {
                    runtime.enter_fatal(error);
                }
            });
            const release = () => {
                if (!active) {
                    return;
                }
                active = false;
                observer.disconnect();
                observers.delete(release);
            };
            if (dead()) {
                active = false;
            } else {
                observers.add(release);
                observer.observe(element);
            }
            return release;
        },
    };
}
