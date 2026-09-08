import { boot, show_fatal, StartupError } from "./loader.js";

const status = document.getElementById("status");
let handle = null;

const runtime_fatal = (error) => {
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
                handle = app.workbench_mount("workbench");
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
