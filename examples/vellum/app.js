import { boot, release_container, show_fatal, StartupError } from "./loader.js";

const status = document.getElementById("status");
const wasm_url = new URL("./vellum.wasm", import.meta.url);
let handle = null;
let live = null;
let readyFrame = null;
let mountGeneration = 0;
// Where the page was loaded, and what `window` and `navigator` held once it
// was first ready: `reset()` returns the page to both.
const entry = location.href;
let firstLoad = null;

// Fatal cleanup must stay outside the failed Wasm instance. This owns only
// native resource teardown; document reads, writes and serialization are Rust.
window.__vellumAbortResource = (signal, resource, kind) => {
    const close = () => {
        try {
            if (kind === "raf") cancelAnimationFrame(resource);
            else if (kind === "font") document.fonts.delete(resource);
            else if (kind === "transaction") resource.abort();
            else if (kind === "database") resource.close();
            else if (resource.readyState === "done") resource.result?.close();
        } catch { /* Already completed/closed resources have nothing left to stop. */ }
    };
    const lateOpen = () => { if (signal.aborted) close(); };
    if (kind === "open") resource.addEventListener("success", lateOpen, { once: true });
    signal.addEventListener("abort", close, { once: true });
    if (signal.aborted) close();
    return () => {
        signal.removeEventListener("abort", close);
        if (kind === "open") resource.removeEventListener("success", lateOpen);
    };
};

// Everything a remount reads back: the autosaved document in IndexedDB, and
// the options, welcome flag and fallback document in localStorage.
async function forgetStorage() {
    localStorage.clear();
    sessionStorage.clear();
    const names = new Set(["vellum-editor", ...(await indexedDB.databases()).map(database => database.name)]);
    // A connection still closing only delays a delete; it completes once it has.
    await Promise.all([...names].map(name => new Promise((resolve, reject) => {
        const request = indexedDB.deleteDatabase(name);
        request.onsuccess = resolve;
        request.onerror = () => reject(request.error);
    })));
}

function forgetAdded(target, kept) {
    for (const key of Reflect.ownKeys(target)) {
        if (!kept.has(key)) Reflect.deleteProperty(target, key);
    }
}

function stopReadyCheck() {
    if (readyFrame !== null) cancelAnimationFrame(readyFrame);
    readyFrame = null;
    mountGeneration++;
}

const runtime_fatal = (error) => {
    stopReadyCheck();
    release_container(document.getElementById("vellum"));
    handle = null;
    const died = live;
    live = null;
    delete window.vellum;
    delete window.__vellum;
    status.dataset.status = "fatal";
    show_fatal(status, new StartupError("RuntimeFatal", `${error}; unsaved changes were lost. Restart Vellum or reload the page.`), {
        restart: died && died.restarts < died.restart_limit ? () => relaunch(died) : null,
    });
};

async function relaunch(died) {
    status.dataset.status = "starting";
    try {
        const next = await died.restart({ on_fatal: runtime_fatal });
        if (!next) throw new StartupError("RuntimeFatal", "No restarts left; reload the page.");
        publish(next);
    } catch (error) {
        status.dataset.status = "failed";
        show_fatal(status, error);
    }
}

function publish(started) {
    stopReadyCheck();
    live = started;
    const { app, hooks, build, instance } = started;
    app.vellum_identify(instance, build);
    const snapshot = () => JSON.parse(app.vellum_snapshot());
    const command = (name, value = null) => JSON.parse(app.vellum_command(name, JSON.stringify(value)));
    const asyncCommand = async (name, value = null) => JSON.parse(await app.vellum_async_command(name, JSON.stringify(value)));
    const waitReady = () => {
        const generation = mountGeneration;
        status.dataset.status = "starting";
        const check = () => {
            readyFrame = null;
            if (live !== started || handle === null || mountGeneration !== generation) return;
            if (snapshot().ready) {
                firstLoad ??= { window: new Set(Reflect.ownKeys(window)), navigator: new Set(Reflect.ownKeys(navigator)) };
                status.dataset.status = "ready";
                status.textContent = "ready";
            } else {
                readyFrame = requestAnimationFrame(check);
            }
        };
        readyFrame = requestAnimationFrame(check);
    };
    const doc = {
        get data() { return snapshot().doc.data; },
        get nodes() { return snapshot().doc.nodes; },
        get page() { return snapshot().doc.page; },
        get revision() { return snapshot().doc.revision; },
        get(id) { return snapshot().doc.nodes.find(node => node.id === id); },
        world(id) { return JSON.parse(app.vellum_world(id)); },
        serialize() { return app.vellum_serialize(); },
    };
    // The page as a first load has it, without loading it again: nothing a
    // remount would read back from storage, nothing a caller added to the
    // page, and a new editor. Resolves once that editor is ready and has
    // drawn its scene. Creating the region again is cheap here; keeping it
    // would mean resetting every piece of editor state by hand.
    const reset = async () => {
        window.__vellum.dispose();
        await forgetStorage();
        // Only faces added from script go; the document keeps its stylesheets' own.
        for (const face of [...document.fonts]) document.fonts.delete(face);
        if (firstLoad) {
            forgetAdded(window, firstLoad.window);
            forgetAdded(navigator, firstLoad.navigator);
        }
        hooks.runtime.errors.length = 0;
        if (location.href !== entry) history.replaceState(null, "", entry);
        window.__vellum.mount();
        await new Promise((resolve, reject) => {
            const check = () => {
                if (live !== started) return reject(new Error("Vellum stopped before its reset finished"));
                const state = handle !== null && status.dataset.status === "ready" ? snapshot() : null;
                if (state?.ready && (state.renderer.gpuError !== null || state.renderer.instanceCount > 0)) resolve();
                else requestAnimationFrame(check);
            };
            requestAnimationFrame(check);
        });
    };
    window.__vellum = {
        hooks, instance, reset,
        mount(container_id = "vellum") { stopReadyCheck(); handle = app.vellum_mount(container_id); waitReady(); return handle; },
        dispose() { stopReadyCheck(); if (handle === null) return false; const disposed = app.vellum_dispose(handle); handle = null; return disposed; },
        snapshot,
        diagnostics() { return JSON.parse(app.vellum_diagnostics()); },
        live_regions() { return app.vellum_live_regions(); },
        errors() { return hooks.runtime.errors; },
        stats() { return hooks.runtime.stats(); },
        rasterStats() { return JSON.parse(app.vellum_raster_stats()); },
        replaceTextExternally(id, text) { return app.vellum_replace_text_externally(JSON.stringify({ id, text })); },
    };
    window.vellum = {
        version: "0.1.0", doc,
        get ready() { return snapshot().ready; },
        get state() { const state = snapshot().state; state.selection = new Set(state.selection); return state; },
        get options() { return snapshot().options; },
        get renderer() { return { ...snapshot().renderer, exportCanvas(ids, scale = 1) { return app.vellum_export_canvas(JSON.stringify(ids), scale); } }; },
        select(ids) { return command("select", ids); },
        fit(ids = null) { return command("fit", ids); },
        zoomAt(factor, x, y) { return command("zoomAt", { factor, x, y }); },
        setTool(tool) { return command("setTool", tool); },
        setProperty(prop, value) { return command("setProperty", { prop, value }); },
        transaction(label, patches) { return command("transaction", { label, patches }); },
        createAtCenter(type, props = {}) { return command("createAtCenter", { type, props }); },
        textLayout(id) { return JSON.parse(app.vellum_text_layout(id)); },
        importDocument(file) { return app.vellum_import_document(file); },
        importImage(file, location = null) { return app.vellum_import_image(file, JSON.stringify(location)); },
        importFont(font) { return font instanceof File ? app.vellum_import_font_file(font) : asyncCommand("importFont", font); },
        save() { return asyncCommand("save"); },
        doExport(format, scale = 2, ids = null) { return app.vellum_do_export(format, scale, JSON.stringify(ids)); },
        loadStoredFonts() { return asyncCommand("loadStoredFonts"); },
        fontReady() { return asyncCommand("fontReady"); },
        setAsset(asset) { return asyncCommand("setAsset", asset); },
        insertAsset(kind) { return command("insertAsset", kind); },
        instantiate(id) { return command("instantiate", id); },
        switchPage(id) { return command("switchPage", id); },
        setTheme(theme) { return command("setTheme", theme); },
        parse(text) { return command("parse", text); },
        exportSVG(ids = null) { return command("exportSVG", ids); },
        render() { return command("render"); },
        actions: new Proxy({}, { get: (_, name) => (...args) => command("action", { name: String(name), value: args[0] ?? null }) }),
    };
    window.__vellum.mount();
}

boot({ wasm_url, on_fatal: runtime_fatal }).then(publish).catch(error => {
    status.dataset.status = "failed";
    show_fatal(status, error);
});
