import { boot, release_container, show_fatal, StartupError } from "./loader.js";

const status = document.getElementById("status");
const wasm_url = new URL("./fusion-basic.wasm", import.meta.url);

/// Every instance running on this page, by its number. A second instance is a
/// second wasm instance - its own memory, its own runtime, its own diagnostics
/// - and the page reaches it the same way it reaches the first.
const instances = {};
window.__fusion_instances = instances;

/// Which instance the page itself is showing.
///
/// Not always the first one: a restart puts a new instance in its place, and
/// from then on that is the one whose death the page's status line is about.
/// An extra instance a test booted alongside it is nobody's news but the
/// test's.
let showing = 1;

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
    // What this instance has mounted, by container: the scope's own handle and
    // which fixture it is. The fixture is kept because a replacement instance
    // has to put back what this one was showing, and "a scope was here" does
    // not say which one.
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
            mounted.set(container_id, { id, fixture: "mount" });
            return id;
        },
        /// The smallest complete application, which is what this page shows.
        b0(container_id) {
            const id = app.fusion_basic_b0_mount(container_id);
            mounted.set(container_id, { id, fixture: "b0" });
            return id;
        },
        /// Its one state, its region's state, and where the region drew each
        /// of its twenty controls.
        b0_state() {
            return JSON.parse(app.fusion_basic_b0());
        },
        /// The names of those controls, in drawing order.
        b0_controls() {
            return JSON.parse(app.fusion_basic_b0_controls());
        },
        /// How many of the application's own components are alive here. Zero
        /// after the last scope has gone, or something of it is still held.
        live_components() {
            return app.fusion_basic_live_components();
        },
        // The two routing fixtures. `mount_owner` is expected to fail when one
        // is already mounted, and the caller is meant to see it.
        mount_owner(container_id) {
            const id = app.fusion_basic_mount_owner(container_id);
            mounted.set(container_id, { id, fixture: "mount_owner" });
            return id;
        },
        mount_guest(container_id) {
            const id = app.fusion_basic_mount_guest(container_id);
            mounted.set(container_id, { id, fixture: "mount_guest" });
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
            mounted.set(container_id, { id, fixture: "mount_geometry" });
            return id;
        },
        geometry() {
            return JSON.parse(app.fusion_basic_geometry());
        },
        /// A scope whose only control traps inside its own event handler.
        mount_trap(container_id) {
            const id = app.fusion_basic_mount_trap(container_id);
            mounted.set(container_id, { id, fixture: "mount_trap" });
            return id;
        },
        /// Traps this instance from an export call.
        trap() {
            app.fusion_basic_trap();
        },
        /// Traps this instance from inside a task the host is running. The
        /// call itself returns; the failure arrives a turn later.
        trap_deferred() {
            app.fusion_basic_trap_deferred();
        },
        /// What a JS-side simulation of a trap reaches. It is the same door
        /// the three real ones come through, which is the point of having it.
        simulate_trap(message = "simulated trap") {
            hooks.runtime.enter_fatal(new Error(message));
        },
        dispose(container_id) {
            const scope = mounted.get(container_id);
            if (scope === undefined) {
                return false;
            }
            mounted.delete(container_id);
            return app.fusion_basic_dispose(scope.id);
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
        /// A second application instance on this page, mounting `fixture`
        /// into `container_id`. Resolves to its instance number.
        ///
        /// Which fixture matters: an instance that is going to be asked
        /// whether it still works needs something to work, and one that is
        /// going to be killed needs a way to die.
        async boot_instance(container_id, fixture = "mount_trap") {
            const next = await start((on_fatal) => boot({ wasm_url, on_fatal }));
            next[fixture](container_id);
            return next.instance;
        },
        /// The second long-lived instance of the B4 load.
        ///
        /// B4 is two instances, each mounting and unmounting one scope of two
        /// regions per round; the instances themselves are booted once and
        /// live for the whole measurement, because a restarted instance
        /// leaves a linear memory behind and that is a different measurement.
        async boot_second_instance(container_id = "instance-two") {
            const existing = instances[2];
            if (existing !== undefined && existing.fatal() === null) {
                return existing.instance;
            }
            const next = await start((on_fatal) => boot({ wasm_url, on_fatal }));
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
    //
    // The notice belongs to the instance the page is showing. A second
    // instance dying is not news the page's own status line should carry: the
    // test that booted it is what asks about it.
    slot.notify = (error) => {
        dead = String(error);
        const was_showing = [...mounted].map(([container_id, scope]) => [
            container_id,
            scope.fixture,
        ]);
        for (const [container_id] of was_showing) {
            release_container(document.getElementById(container_id));
        }
        mounted.clear();
        if (instance !== showing) {
            return;
        }
        delete window.__fusion_basic;
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
            { restart: again ? () => relaunch(handle, was_showing) : null }
        );
    };

    instances[instance] = api;
    return api;
}

/// Starts a replacement for the instance that died and puts back what it had
/// mounted, so the page a person is looking at comes back rather than an empty
/// one they have to know how to refill.
async function relaunch(died, was_showing) {
    status.dataset.status = "starting";
    status.textContent = "starting";
    const next = await start((on_fatal) => died.restart({ on_fatal }));
    if (next === null) {
        status.dataset.status = "fatal";
        show_fatal(status, new StartupError("RuntimeFatal", "no restarts left; reload the page"));
        return;
    }
    for (const [container_id, fixture] of was_showing) {
        next[fixture](container_id);
    }
    showing = next.instance;
    window.__fusion_basic = next;
    status.dataset.status = "ready";
    status.textContent = "ready";
}

start((on_fatal) => boot({ wasm_url, on_fatal }))
    .then((api) => {
        api.b0("b0");
        window.__fusion_basic = api;
        status.dataset.status = "ready";
        status.textContent = "ready";
    })
    .catch((error) => {
        status.dataset.status = "failed";
        show_fatal(status, error);
    });
