import { boot, show_fatal, StartupError } from "./loader.js";
import * as noUiSlider from "./vendor/nouislider/nouislider.min.mjs";

// The one fixed-version third-party DOM component this release verifies:
// noUiSlider 15.8.1, vendored into the checkout and recorded in
// sources.lock.json. The shim is thin on purpose - the initialization and the
// teardown being checked are the component's own - and it counts what it holds
// so a rebuild can be inspected rather than assumed clean.
const third_party = {
    name: "nouislider",
    version: "15.8.1",
    created: 0,
    destroyed: 0,
    live: new Map(),
    create(element, start, min, max, step, on_update) {
        if (third_party.live.has(element)) {
            throw new Error("that element already carries a slider");
        }
        noUiSlider.create(element, {
            start: [start],
            step,
            range: { min, max },
            connect: [true, false],
        });
        const listener = (values) => on_update(Number(values[0]));
        element.noUiSlider.on("update", listener);
        third_party.live.set(element, listener);
        third_party.created += 1;
        return true;
    },
    destroy(element) {
        if (!third_party.live.has(element)) {
            return false;
        }
        // The component's own teardown: it removes the nodes it made and the
        // handlers it registered.
        element.noUiSlider.destroy();
        third_party.live.delete(element);
        third_party.destroyed += 1;
        return true;
    },
    set(element, value) {
        const slider = element.noUiSlider;
        if (!slider) {
            return false;
        }
        if (Number(slider.get()) === value) {
            return true;
        }
        slider.set(value);
        return true;
    },
    stats() {
        return {
            created: third_party.created,
            destroyed: third_party.destroyed,
            live: third_party.live.size,
            // What the document holds, not what the shim believes it holds.
            targets: document.querySelectorAll(".noUi-target").length,
            handles: document.querySelectorAll(".noUi-handle").length,
        };
    },
};
window.__rustify_third_party = third_party;

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
            // A second scope on the same page, for the checks that need one
            // instance to be shown not to disturb another.
            mount_into(container_id) {
                const host = document.createElement("div");
                host.id = container_id;
                document.querySelector("main").append(host);
                return app.workbench_mount(container_id);
            },
            dispose_handle(id, container_id) {
                const spent = app.workbench_dispose(id);
                document.getElementById(container_id)?.remove();
                return spent;
            },
            snapshot() {
                return JSON.parse(app.workbench_snapshot());
            },
            start_load(delay_ms, outcome) {
                return app.workbench_start_load(delay_ms, outcome);
            },
            lookup_object(id) {
                return app.workbench_lookup_object(id);
            },
            // R25's wait: bounded at five seconds, and the only outcome the
            // deadline can produce is a timeout - found and disposed are
            // answers the application already has.
            async await_object(id, timeout_ms = 5000) {
                const deadline = performance.now() + Math.min(timeout_ms, 5000);
                for (;;) {
                    const outcome = app.workbench_lookup_object(id);
                    if (outcome !== "not_found") {
                        return outcome;
                    }
                    if (performance.now() >= deadline) {
                        return "timeout";
                    }
                    await new Promise((resolve) => setTimeout(resolve, 50));
                }
            },
            inject_duplicate_id() {
                return app.workbench_inject_duplicate_id();
            },
            close_on_next_action() {
                app.workbench_close_on_next_action();
            },
            set_third_party(present) {
                return app.workbench_set_third_party(present);
            },
            // Moves the third-party component the way a user would, from
            // outside the application: whatever it reports is the component's
            // own doing.
            nudge_third_party(value) {
                let moved = 0;
                for (const element of third_party.live.keys()) {
                    element.noUiSlider.set(value);
                    moved += 1;
                }
                return moved;
            },
            third_party: third_party.stats,
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
