import { boot, release_container, show_fatal, StartupError } from "./loader.js";
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
// Where the page was loaded, before the application moved the address to the
// object it selected. A reset starts from here the way the load did.
const first_address = location.pathname + location.search;
// One WEBGL_lose_context per canvas, taken while the context still hands
// extensions out, with the context it belongs to.
const lose_context_handles = new Map();
// The containers `mount_into` made, so a reset can take away what a check
// added to the page.
const added = new Set();
const wasm_url = new URL("./property-workbench.wasm", import.meta.url);
let handle = null;
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
    handle = null;
    const died = live;
    live = null;
    delete window.__property_workbench;
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

const next_task = () => new Promise((resolve) => setTimeout(resolve, 0));

/// Waits for `done()`, polling. The application's effects run as microtasks,
/// so one task is enough for them; a region's start or rebuild is not.
async function until(done, what, timeout_ms = 60_000) {
    const deadline = performance.now() + timeout_ms;
    while (!done()) {
        if (performance.now() > deadline) {
            throw new Error(`reset: ${what} did not happen within ${timeout_ms} ms`);
        }
        await new Promise((resolve) => setTimeout(resolve, 20));
    }
}

/// Publishes the page's handle on one live instance and mounts it.
function publish(started) {
    live = started;
    const { app, hooks, build, base } = started;
    app.workbench_set_base(base);
    app.workbench_identify(started.instance, build);
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
            added.add(host);
            return app.workbench_mount(container_id);
        },
        // Mounts over a container that is already taken, which is a
        // refusal the SDK records. Nothing is created either way, so this
        // can be driven as hard as a test needs.
        mount_over(container_id) {
            try {
                app.workbench_mount(container_id);
                return "mounted";
            } catch (error) {
                return String(error);
            }
        },
        dispose_handle(id, container_id) {
            const spent = app.workbench_dispose(id);
            const host = document.getElementById(container_id);
            added.delete(host);
            host?.remove();
            return spent;
        },
        snapshot() {
            return JSON.parse(app.workbench_snapshot());
        },
        start_load(delay_ms, outcome) {
            return app.workbench_start_load(delay_ms, outcome);
        },
        // The validations the form is waiting on, oldest first, and the
        // two answers a test holds open: one per check, one per save.
        form_checks() {
            return JSON.parse(app.workbench_form_checks());
        },
        resolve_check(index, ok) {
            return app.workbench_resolve_check(index, ok);
        },
        resolve_save(ok) {
            return app.workbench_resolve_save(ok);
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
        diagnostics() {
            return JSON.parse(app.workbench_diagnostics());
        },
        set_diagnostics(on) {
            return app.workbench_set_diagnostics(on);
        },
        // Takes the GL context away from a region's canvas the way the
        // browser does when it reclaims one. The extension is the only
        // honest way to produce a real loss.
        lose_context(test_id) {
            const canvas = document.querySelector(`[data-testid="${test_id}"]`);
            const gl = canvas?.getContext("webgl2");
            const ext = gl?.getExtension("WEBGL_lose_context");
            if (!ext) {
                return false;
            }
            // Kept: a lost context hands out no extensions, so the only
            // way to ask for the restore is an object taken beforehand.
            lose_context_handles.set(test_id, { gl, ext });
            ext.loseContext();
            return true;
        },
        // A real loss is followed by the browser's own restore; the
        // extension makes that step explicit so a test can drive it.
        restore_context(test_id) {
            const lost = lose_context_handles.get(test_id);
            if (!lost) {
                return false;
            }
            lost.ext.restoreContext();
            return true;
        },
        // Puts the page back the way it loaded, without loading it again.
        //
        // Loading is not what a check costs here; starting the region is:
        // every new WebGL context has the software rasteriser compile its
        // shaders, and mounting the scope again makes a new context. So the
        // scope and its region stay, and the application gives its state the
        // values it started with. Only a check that closed the scope pays for
        // a start, because the scope has to be mounted again.
        //
        // What is not put back: the diagnostic record and the runtime's
        // counters, which have no reset, the history entries a check pushed,
        // which cannot be removed, and what the region keeps to itself, such
        // as how far its group list is scrolled.
        async reset() {
            // Left focused, a text control commits when it loses focus. That
            // has to land in the state being thrown away, not the one put back.
            document.activeElement?.blur?.();
            getSelection()?.removeAllRanges();
            for (const { gl, ext } of lose_context_handles.values()) {
                if (gl.isContextLost()) {
                    ext.restoreContext();
                }
            }
            lose_context_handles.clear();

            // Handles start at one, so a page whose scope was disposed asks
            // for none of them to be kept.
            const mounted = app.workbench_reset_page(handle ?? 0);
            for (const host of added) {
                host.remove();
            }
            added.clear();
            if (mounted) {
                // The third-party component went out in the first half; it
                // comes back in the second, once its teardown has run.
                await next_task();
                app.workbench_reset_scope(first_address);
            } else {
                // Closed by the check, or by an action it armed: mounted
                // again at the address the page loaded at, which is what the
                // scope reads its selection from.
                history.replaceState(history.state, "", first_address);
                handle = app.workbench_mount(container);
            }
            // The scope's effects - the address following the selection, the
            // snapshot - run as microtasks after the calls above.
            await next_task();
            await until(() => third_party.live.size === 1, "the third-party component");
            await until(
                () => JSON.parse(app.workbench_snapshot()).region === "ready",
                "a ready region"
            );
            // Counted from the load, when the one component there is had just
            // been made.
            third_party.created = third_party.live.size;
            third_party.destroyed = 0;
            hooks.runtime.errors.length = 0;
            // Again: a layer that closed above hands the keyboard back to
            // whatever opened it when nothing else has it.
            document.activeElement?.blur?.();
            // A check that zoomed the page set the root font size.
            document.documentElement.removeAttribute("style");
            window.scrollTo(0, 0);
            for (const element of document.querySelectorAll("main *")) {
                if (element.scrollTop !== 0 || element.scrollLeft !== 0) {
                    element.scrollTop = 0;
                    element.scrollLeft = 0;
                }
            }
        },
    };
    window.__property_workbench.mount();
    status.dataset.status = "ready";
    status.textContent = "ready";
}

boot({ wasm_url, on_fatal: runtime_fatal })
    .then(publish)
    .catch((error) => {
        status.dataset.status = "failed";
        show_fatal(status, error);
    });