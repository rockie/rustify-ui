import { boot, release_container, show_fatal, StartupError } from "./loader.js";

const container = "catalog";
const second_container = "catalog-second";
const status = document.getElementById("status");
const wasm_url = new URL("./component-catalog.wasm", import.meta.url);
let handle = null;
let second = null;
/// The loader handle of the instance that is running, so a failure notice can
/// offer to start another one in its place.
let live = null;

/// What one instance's death looks like on the page.
///
/// The mounted controls and their listeners live in the module that just
/// trapped, so they have to go with it. Removing the nodes is a JS-only path:
/// calling the application's dispose would re-enter that module.
const runtime_fatal = (error) => {
    if (handle !== null) {
        release_container(document.getElementById(container));
    }
    if (second !== null) {
        release_container(document.getElementById(second_container));
    }
    handle = null;
    second = null;
    const died = live;
    live = null;
    delete window.__component_catalog;
    status.dataset.status = "fatal";
    const again = died !== null && died.restarts < died.restart_limit;
    show_fatal(
        status,
        new StartupError(
            "RuntimeFatal",
            again
                ? `${error}; unsaved in-memory state is lost. Restart this instance, or reload the page.`
                : `${error}; unsaved in-memory state is lost. Reload the page: this instance has been restarted as often as it can be.`
        ),
        { restart: again ? () => relaunch(died) : null }
    );
};

async function relaunch(died) {
    status.dataset.status = "starting";
    status.textContent = "starting";
    try {
        const next = await died.restart({ on_fatal: runtime_fatal });
        if (next === null) {
            status.dataset.status = "fatal";
            show_fatal(status, new StartupError("RuntimeFatal", "no restarts left; reload the page"));
            return;
        }
        publish(next);
    } catch (error) {
        status.dataset.status = "failed";
        show_fatal(status, error);
    }
}

/// Publishes the page's handle on one live instance and mounts it.
function publish(started) {
    live = started;
    const { app, hooks, build } = started;
    app.catalog_identify(started.instance, build);
    window.__component_catalog = {
        hooks,
        mount() {
            handle = app.catalog_mount(container);
            return handle;
        },
        dispose() {
            if (handle === null) {
                return false;
            }
            const spent = app.catalog_dispose(handle);
            handle = null;
            return spent;
        },
        // A second scope on the same page, for the checks that need one
        // instance to be shown not to disturb another.
        mount_second() {
            second = app.catalog_mount(second_container);
            return second;
        },
        dispose_second() {
            if (second === null) {
                return false;
            }
            const spent = app.catalog_dispose(second);
            second = null;
            return spent;
        },
        snapshot() {
            return JSON.parse(app.catalog_snapshot());
        },
        diagnostics() {
            return JSON.parse(app.catalog_diagnostics());
        },
        live_regions() {
            return app.catalog_live_regions();
        },
        stats() {
            return hooks.runtime.stats();
        },
    };
    window.__component_catalog.mount();
    status.dataset.status = "ready";
    status.textContent = "ready";
}

boot({ wasm_url, on_fatal: runtime_fatal })
    .then(publish)
    .catch((error) => {
        status.dataset.status = "failed";
        show_fatal(status, error);
    });
