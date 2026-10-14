import { boot, release_container } from "./rustify-dom-loader.js";

// No cross-build resource serialization is used by this deterministic view.
globalThis.__RESOLVED_RESOURCES = [];
globalThis.__INCOMPLETE_CHUNKS = [];
globalThis.__SERIALIZED_ERRORS = [];

const scopes = ["catalog-a", "catalog-b", "catalog-c"];
const mode = document.body.dataset.harnessMode;
const packageName = mode === "csr" ? "csr/pkg" : "hydrate/pkg";
const status = document.getElementById("status");
const rootOf = (scope) => document.querySelector(`[data-harness-root="${scope}"] > div`);
const handles = new Map();
const captures = new Map();
const originals = new Map(scopes.map((scope) => [scope, rootOf(scope).cloneNode(true)]));
let app;
let runtime;
let fatal = null;

const runtimeAttributes = new Set([
    "data-rustify-mounted",
    "data-rustify-instance",
    "data-rustify-theme-runtime",
    "data-rustify-theme-owner",
    "data-rustify-geometry-scope",
]);

// Tachys 0.2.19 emits these runtime placeholders. Inspect cloned live DOM
// before serializing: reparsing a textarea's serialized empty comment would
// turn it into authored RCDATA text and corrupt the comparison.
const frameworkComments = new Set(["", "!", "<() />"]);

function normalizedOuterHTML(element, removeFrameworkComments) {
    const clone = element.cloneNode(true);
    if (removeFrameworkComments) {
        const walker = document.createTreeWalker(clone, NodeFilter.SHOW_COMMENT);
        const markers = [];
        while (walker.nextNode()) {
            if (frameworkComments.has(walker.currentNode.data)) markers.push(walker.currentNode);
        }
        for (const marker of markers) marker.remove();
    }
    for (const node of [clone, ...clone.querySelectorAll("*")]) {
        const attributes = [...node.attributes]
            .filter((attribute) => !runtimeAttributes.has(attribute.name))
            .map((attribute) => [attribute.name, attribute.value])
            .sort(([a], [b]) => a.localeCompare(b));
        for (const attribute of [...node.attributes]) node.removeAttribute(attribute.name);
        for (const [name, value] of attributes) node.setAttribute(name, value);
    }
    return clone.outerHTML;
}

function tree(scope, removeFrameworkComments) {
    const root = rootOf(scope);
    // SDK transport metadata belongs to the root. These are the two complete
    // authored trees; values, IDs, ARIA, classes, styles and text are preserved.
    return [...root.children].map((element) => normalizedOuterHTML(element, removeFrameworkComments));
}

const content = (scope) => tree(scope, true);
const rawContent = (scope) => tree(scope, false);

function capture(scope) {
    const root = rootOf(scope);
    const walker = document.createTreeWalker(root, NodeFilter.SHOW_ALL);
    const nodes = [root];
    while (walker.nextNode()) nodes.push(walker.currentNode);
    captures.set(scope, {
        root,
        nodes,
        ids: [...root.querySelectorAll("[id]")].map((node) => [node, node.id]),
        html: content(scope),
    });
    return { html: content(scope), rawHtml: rawContent(scope), nodes: nodes.length, elements: nodes.filter((node) => node.nodeType === Node.ELEMENT_NODE).length };
}

function identity(scope) {
    const before = captures.get(scope);
    const root = rootOf(scope);
    return {
        root: root === before.root,
        nodes: before.nodes.length,
        retained: before.nodes.filter((node) => node.isConnected && root.contains(node)).length,
        ids: before.ids.every(([node, id]) => node.id === id),
    };
}

function start(scope) {
    const handle = app.harness_attach(rootOf(scope), scope);
    handles.set(scope, handle);
    return handle;
}

function reject(kind) {
    const root = document.getElementById("rejection-root").content.firstElementChild.cloneNode(true);
    if (kind !== "detached") document.body.append(root);
    if (kind === "marker") root.removeAttribute("data-rustify-ssr");
    if (kind === "content") root.querySelector("[data-rustify-content]").remove();
    if (kind === "overlay") root.querySelector("[data-rustify-overlay]").remove();
    if (kind === "extra") root.append(document.createElement("div"));
    if (kind === "scope") root.setAttribute("data-rustify-scope", "wrong");
    if (kind === "seed") root.setAttribute("data-rustify-seed", "wrong");
    if (kind === "base") root.setAttribute("data-rustify-base", "/wrong/");
    const before = root.outerHTML;
    const children = [...root.childNodes];
    let error = null;
    try {
        app.harness_attach(root, "rejected");
    } catch (caught) {
        error = String(caught);
    }
    const after = root.outerHTML;
    const sameNodes = children.every((node, index) => root.childNodes[index] === node);
    root.remove();
    return { error, before, after, sameNodes };
}

function failed(error) {
    fatal = String(error);
    for (const scope of scopes) {
        const root = rootOf(scope);
        release_container(root);
        const original = originals.get(scope);
        // Only JavaScript can clean a failed instance and restore its static
        // initial state. Never call its Rust destructors after a trap.
        root.replaceWith(original.cloneNode(true));
    }
    status.dataset.status = "failed";
    status.textContent = "The shared runtime failed. Reload to restore interaction.";
}

try {
    if (mode === "preboot") {
        app = await import(`./${packageName}/app.js?without-runtime`);
        await app.default({ module_or_path: `./${packageName}/app_bg.wasm` });
    } else {
        const started = await boot({
            glue_url: `./${packageName}/app.js`,
            wasm_url: `./${packageName}/app_bg.wasm`,
            on_fatal: failed,
        });
        app = started.app;
        runtime = started.hooks.runtime;
        app.harness_identify(started.instance);
    }
    window.__hydration = {
        mode,
        scopes,
        capture,
        identity,
        content,
        rawContent,
        start,
        startAll() {
            const immediate = {};
            for (const scope of scopes) {
                start(scope);
                immediate[scope] = rawContent(scope);
            }
            status.dataset.status = "attached";
            status.textContent = "All three scopes are interactive.";
            return immediate;
        },
        attempt(scope) {
            try { return { handle: start(scope), error: null }; }
            catch (error) { return { handle: null, error: String(error) }; }
        },
        state: (scope) => JSON.parse(app.harness_state(scope) ?? "null"),
        diagnostics: () => JSON.parse(app.harness_diagnostics()),
        stats: () => runtime?.stats() ?? null,
        dispose(scope) {
            const handle = handles.get(scope);
            if (handle === undefined) return false;
            const disposed = app.harness_dispose(handle);
            handles.delete(scope);
            return disposed;
        },
        asynchronous: (scope) => app.harness_async(scope),
        reject,
        fatal: () => fatal,
        trap: () => app.harness_trap_deferred(),
        geometryMount: (root, seed) => app.harness_geometry_mount(root, seed),
        geometryDispose: (handle) => app.harness_dispose(handle),
        peers: {},
        async bootPeer(slot, seed) {
            const shell = document.createElement("section");
            shell.className = "harness-scope";
            shell.dataset.harnessPeer = slot;
            const peerRoot = document.createElement("div");
            shell.append(peerRoot);
            document.querySelector("main").append(shell);
            let dead = null;
            const started = await boot({
                glue_url: "./csr/pkg/app.js",
                wasm_url: "./csr/pkg/app_bg.wasm",
                on_fatal(error) {
                    dead = String(error);
                    release_container(peerRoot);
                },
            });
            started.app.harness_identify(started.instance);
            const handle = started.app.harness_attach(peerRoot, seed);
            const peer = {
                state: () => JSON.parse(started.app.harness_state(seed) ?? "null"),
                stats: () => started.hooks.runtime.stats(),
                dispose: () => started.app.harness_dispose(handle),
                trap: () => started.app.harness_trap_deferred(),
                fatal: () => dead,
            };
            window.__hydration.peers[slot] = peer;
            return started.instance;
        },
    };
    document.getElementById("attach").addEventListener("click", () => window.__hydration.startAll());
    status.dataset.status = "runtime-ready";
    status.textContent = mode === "preboot" ? "Wasm loaded without a runtime." : "Runtime ready; static nodes are untouched.";
} catch (error) {
    status.dataset.status = "startup-failed";
    status.textContent = String(error);
}
