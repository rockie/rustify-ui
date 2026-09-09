import { boot, show_fatal, StartupError } from "./loader.js";

const status = document.getElementById("status");
const handles = new Map();

const runtime_fatal = (error) => {
    // The mounted controls and their listeners live in the module that just
    // trapped, so they have to go with it. Removing the nodes is a JS-only
    // path: calling the application's dispose would re-enter that module.
    for (const container_id of handles.keys()) {
        document.getElementById(container_id)?.replaceChildren();
    }
    handles.clear();
    delete window.__fusion_basic;
    status.dataset.status = "fatal";
    show_fatal(
        status,
        new StartupError("RuntimeFatal", `${error}; reload the page, unsaved in-memory state is lost`)
    );
};

boot({ wasm_url: new URL("./fusion-basic.wasm", import.meta.url), on_fatal: runtime_fatal })
    .then(({ app, hooks }) => {
        const api = {
            hooks,
            mount(container_id) {
                const handle = app.fusion_basic_mount(container_id);
                handles.set(container_id, handle);
                return handle;
            },
            mount_geometry(container_id) {
                const handle = app.fusion_basic_geometry_mount(container_id);
                handles.set(container_id, handle);
                return handle;
            },
            geometry() {
                return JSON.parse(app.fusion_basic_geometry());
            },
            dispose(container_id) {
                const handle = handles.get(container_id);
                if (handle === undefined) {
                    return false;
                }
                handles.delete(container_id);
                return app.fusion_basic_dispose(handle);
            },
            live_regions() {
                return app.fusion_basic_live_regions();
            },
            errors() {
                return hooks.runtime.errors;
            },
            stats() {
                return hooks.runtime.stats();
            },
            region_states() {
                return JSON.parse(app.fusion_basic_region_states());
            },
        };
        api.mount("scope-a");
        api.mount("scope-b");
        window.__fusion_basic = api;
        status.dataset.status = "ready";
        status.textContent = "ready";
    })
    .catch((error) => {
        status.dataset.status = "failed";
        show_fatal(status, error);
    });
