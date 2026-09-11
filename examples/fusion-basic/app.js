import { boot, release_container, show_fatal, StartupError } from "./loader.js";

const status = document.getElementById("status");
const wasm_url = new URL("./fusion-basic.wasm", import.meta.url);

/// Every instance running on this page, by its number. A second instance is a
/// second wasm instance - its own memory, its own runtime, its own diagnostics
/// - and the page reaches it the same way it reaches the first.
const instances = {};
window.__fusion_instances = instances;

/// Starts one application instance and publishes the page's handle on it.
///
/// `create` is given the instance's own failure notice and returns the loader
/// handle, or `null` when the slot may not be restarted again. The notice can
/// fire from inside that call - before the handle it belongs to exists - so it
/// is reached through a slot the instance fills in once it can.
async function start(create) {
    const slot = {};
    const created = create((error) => slot.notify?.(error));
    if (created === null) {
        return null;
    }
    const handle = await created;
    const { app, hooks, build, instance } = handle;
    const mounted = new Map();
    let dead = null;

    app.fusion_basic_identify(instance, build);
    const api = {
        hooks,
        instance,
        /// The wasm instance itself, so a test can hold a weak reference to it
        /// and ask whether a dead instance's memory ever comes back.
        wasm: handle.wasm,
        restarts: handle.restarts,
        restart_limit: handle.restart_limit,
        mount(container_id) {
            const id = app.fusion_basic_mount(container_id);
            mounted.set(container_id, id);
            return id;
        },
        // The two routing fixtures. `mount_owner` is expected to fail when one
        // is already mounted, and the caller is meant to see it.
        mount_owner(container_id) {
            const id = app.fusion_basic_mount_owner(container_id);
            mounted.set(container_id, id);
            return id;
        },
        mount_guest(container_id) {
            const id = app.fusion_basic_mount_guest(container_id);
            mounted.set(container_id, id);
            return id;
        },
        routes() {
            return app.fusion_basic_routes();
        },
        set_guard(on) {
            return app.fusion_basic_set_guard(on);
        },
        mount_geometry(container_id) {
            const id = app.fusion_basic_geometry_mount(container_id);
            mounted.set(container_id, id);
            return id;
        },
        geometry() {
            return JSON.parse(app.fusion_basic_geometry());
        },
        /// A scope whose only control traps inside its own event handler.
        mount_trap(container_id) {
            const id = app.fusion_basic_mount_trap(container_id);
            mounted.set(container_id, id);
            return id;
        },
        /// Traps this instance from an export call.
        trap() {
            app.fusion_basic_trap();
        },
        dispose(container_id) {
            const id = mounted.get(container_id);
            if (id === undefined) {
                return false;
            }
            mounted.delete(container_id);
            return app.fusion_basic_dispose(id);
        },
        diagnostics() {
            return JSON.parse(app.fusion_basic_diagnostics());
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
        /// What killed this instance, or `null` while it is alive.
        fatal() {
            return dead;
        },
        /// A second application instance on this page, mounting the trap
        /// fixture into `container_id`. Resolves to its instance number.
        async boot_instance(container_id) {
            const next = await start((on_fatal) => boot({ wasm_url, on_fatal }));
            next.mount_trap(container_id);
            return next.instance;
        },
        /// A fresh instance in this slot, mounting the trap fixture again.
        /// `null` once the slot has been restarted as often as it may be.
        async restart(container_id) {
            const next = await start((on_fatal) => handle.restart({ on_fatal }));
            if (next === null) {
                return null;
            }
            next.mount_trap(container_id);
            return next.instance;
        },
    };

    // The mounted controls and their listeners live in the module that just
    // trapped, so they have to go with it. Removing the nodes is a JS-only
    // path: calling the application's dispose would re-enter that module.
    slot.notify = (error) => {
        dead = String(error);
        for (const container_id of mounted.keys()) {
            release_container(document.getElementById(container_id));
        }
        mounted.clear();
        if (instance !== 1) {
            return;
        }
        delete window.__fusion_basic;
        status.dataset.status = "fatal";
        show_fatal(
            status,
            new StartupError("RuntimeFatal", `${error}; reload the page, unsaved in-memory state is lost`)
        );
    };

    instances[instance] = api;
    return api;
}

start((on_fatal) => boot({ wasm_url, on_fatal }))
    .then((api) => {
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
