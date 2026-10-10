import { boot, release_container, show_fatal, StartupError } from "./rustify-dom-loader.js";

// The wasm-bindgen output, deployed beside the loader.
const glue_url = "./pkg/app.js";
const wasm_url = "./pkg/app_bg.wasm";

const status = document.getElementById("status");

/// Every instance on this page, by its number.
const instances = {};
/// Which instances have had a release run on abort, in the order they ran.
const released = [];
window.__dom_instances = instances;

/// Which instance the page itself is showing. A restart puts a new one in
/// that place; an instance a test booted beside it is the test's business.
let showing = 1;

/// Starts one instance with the application mounted in `container_id`, and
/// publishes its handle. `create` is given the instance's failure notice and
/// returns the loader's promise, or `null` when the slot may not restart.
async function start(container_id, url_owner, create) {
    const slot = {};
    const created = create((error) => slot.notify?.(error));
    if (created === null) {
        return null;
    }
    const handle = await created;
    const { app, hooks, instance } = handle;
    let dead = null;
    app.dom_consumer_identify(instance);
    const scope = app.dom_consumer_mount(container_id, url_owner);

    const api = {
        instance,
        restarts: handle.restarts,
        restart_limit: handle.restart_limit,
        glue_url: handle.glue_url,
        fatal: () => dead,
        stats: () => hooks.runtime.stats(),
        aborted: () => hooks.signal.aborted,
        state: () => JSON.parse(app.dom_consumer_state(container_id) ?? "null"),
        diagnostics: () => JSON.parse(app.dom_consumer_diagnostics()),
        clipboard_available: () => app.dom_consumer_clipboard_available(),
        tooltip: (open) => app.dom_consumer_tooltip(container_id, open),
        hold_release: () => app.dom_consumer_release_on_abort(() => released.push(instance)),
        trap: () => app.dom_consumer_trap(),
        trap_deferred: () => app.dom_consumer_trap_deferred(),
        simulate_trap: (message = "simulated trap") => hooks.runtime.enter_fatal(new Error(message)),
        dispose: () => app.dom_consumer_dispose(scope),
        /// A fresh instance in this slot, or `null` once the slot has been
        /// restarted as often as it may be.
        restart: async () => {
            const next = await start(container_id, url_owner, (on_fatal) => handle.restart({ on_fatal }));
            return next === null ? null : next.instance;
        },
    };

    // What the dead instance mounted lives in the module that trapped, so it
    // goes with it - through the DOM only: its dispose would re-enter it.
    slot.notify = (error) => {
        dead = String(error);
        release_container(document.getElementById(container_id));
        if (instance !== showing) {
            return;
        }
        status.dataset.status = "fatal";
        const again = handle.restarts < handle.restart_limit;
        show_fatal(
            status,
            new StartupError(
                "RuntimeFatal",
                again
                    ? `${error}; unsaved in-memory state is lost. Restart this instance, or reload the page.`
                    : `${error}; unsaved in-memory state is lost. Reload the page: this instance has been restarted as often as it can be.`
            ),
            { restart: again ? () => api.restart() : null }
        );
    };

    instances[instance] = api;
    if (container_id === "instance-one") {
        showing = instance;
        status.dataset.status = "ready";
        status.replaceChildren();
    }
    return api;
}

window.__dom_consumer = {
    instances,
    released,
    /// A further instance with the application mounted in `container_id`.
    async boot_instance(container_id = "instance-two", url_owner = false) {
        const next = await start(container_id, url_owner, (on_fatal) => boot({ glue_url, wasm_url, on_fatal }));
        return next.instance;
    },
};

try {
    await start("instance-one", true, (on_fatal) => boot({ glue_url, wasm_url, on_fatal }));
} catch (error) {
    status.dataset.status = "startup-failed";
    show_fatal(status, error);
}
