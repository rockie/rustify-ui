import { boot, show_fatal, StartupError } from "./loader.js";

const container = "workbench";
const status = document.getElementById("status");
let handle = null;

const runtime_fatal = (error) => {
    // The mounted controls and their listeners live in the module that just
    // trapped, so they have to go with it. Removing the nodes is a JS-only
    // path: calling the application's dispose would re-enter that module.
    if (handle !== null) {
        document.getElementById(container)?.replaceChildren();
    }
    handle = null;
    delete window.__property_workbench;
    status.dataset.status = "fatal";
    show_fatal(
        status,
        new StartupError("RuntimeFatal", `${error}; reload the page, unsaved in-memory state is lost`)
    );
};

boot({ wasm_url: new URL("./property-workbench.wasm", import.meta.url), on_fatal: runtime_fatal })
    .then(({ app, hooks }) => {
        window.__property_workbench = {
            hooks,
            mount() {
                handle = app.workbench_mount(container);
                return handle;
            },
            dispose() {
                if (handle === null) {
                    return false;
                }
                const spent = app.workbench_dispose(handle);
                handle = null;
                return spent;
            },
            live_regions() {
                return app.workbench_live_regions();
            },
            snapshot() {
                return JSON.parse(app.workbench_snapshot());
            },
            inject_duplicate_id() {
                return app.workbench_inject_duplicate_id();
            },
            close_on_next_action() {
                app.workbench_close_on_next_action();
            },
            stats() {
                return hooks.runtime.stats();
            },
        };
        window.__property_workbench.mount();
        status.dataset.status = "ready";
        status.textContent = "ready";
    })
    .catch((error) => {
        status.dataset.status = "failed";
        show_fatal(status, error);
    });
