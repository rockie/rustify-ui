import { boot, show_fatal, StartupError } from "./loader.js";

const container = "catalog";
const second_container = "catalog-second";
const status = document.getElementById("status");
let handle = null;
let second = null;

const runtime_fatal = (error) => {
    // The mounted controls and their listeners live in the module that just
    // trapped, so they have to go with it. Removing the nodes is a JS-only
    // path: calling the application's dispose would re-enter that module.
    if (handle !== null) {
        document.getElementById(container)?.replaceChildren();
    }
    if (second !== null) {
        document.getElementById(second_container)?.replaceChildren();
    }
    handle = null;
    second = null;
    delete window.__component_catalog;
    status.dataset.status = "fatal";
    show_fatal(
        status,
        new StartupError("RuntimeFatal", `${error}; reload the page, unsaved in-memory state is lost`)
    );
};

boot({ wasm_url: new URL("./component-catalog.wasm", import.meta.url), on_fatal: runtime_fatal })
    .then(({ app, hooks, build }) => {
        app.catalog_identify(1, build);
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
            // Makes the samples page behave as though the font that covers
            // more than Latin never arrived, and lets it arrive again. The
            // real path is a resource failure, which the deployment build
            // exercises against a real broken file; this is what lets one page
            // show the mark and then take it away.
            block_font(blocked) {
                return app.catalog_block_font(blocked);
            },
            stats() {
                return hooks.runtime.stats();
            },
        };
        window.__component_catalog.mount();
        status.dataset.status = "ready";
        status.textContent = "ready";
    })
    .catch((error) => {
        status.dataset.status = "failed";
        show_fatal(status, error);
    });
