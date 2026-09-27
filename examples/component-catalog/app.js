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

const frame = () => new Promise((resolve) => requestAnimationFrame(resolve));

/// Resolves once the page shows what a first load shows.
///
/// The application's effects run after the task that set them off, and the
/// region reports where it drew a control a frame after drawing it; a page
/// that was left on the samples page also has that page's region to lose. The
/// first category is one the region draws, so its control is reported too.
async function settled(catalog, timeout_ms = 60_000) {
    const deadline = performance.now() + timeout_ms;
    for (;;) {
        await frame();
        const now = catalog.snapshot();
        if (
            now.region === "ready" &&
            now.button !== null &&
            now.control !== null &&
            catalog.live_regions() === 1
        ) {
            return;
        }
        if (performance.now() > deadline) {
            throw new Error(`the page did not settle after a reset: ${JSON.stringify(now)}`);
        }
    }
}

/// Publishes the page's handle on one live instance and mounts it.
function publish(started) {
    live = started;
    const { app, hooks, build } = started;
    app.catalog_identify(started.instance, build);
    const catalog = {
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
        // The page as it loads, without loading it again, for checks that
        // share one page.
        //
        // What a load costs is starting the region: every new WebGL context
        // has a software rasteriser compile its shaders, which takes seconds,
        // and a remount pays that again. So the scope and its region stay and
        // the application puts its own state back. Only what a caller did to
        // the page itself is undone the other way: a second scope it added
        // is disposed, and a main scope it disposed is mounted again.
        async reset() {
            catalog.dispose_second();
            if (handle === null) {
                catalog.mount();
            } else {
                app.catalog_reset(handle);
            }
            await settled(catalog);
            // After the application has closed its layers, since a layer that
            // closes hands focus back to whatever opened it.
            document.activeElement?.blur();
            window.scrollTo(0, 0);
            // What the caller provoked is not part of a first load.
            hooks.runtime.errors.length = 0;
        },
    };
    window.__component_catalog = catalog;
    catalog.mount();
    status.dataset.status = "ready";
    status.textContent = "ready";
}

boot({ wasm_url, on_fatal: runtime_fatal })
    .then(publish)
    .catch((error) => {
        status.dataset.status = "failed";
        show_fatal(status, error);
    });
