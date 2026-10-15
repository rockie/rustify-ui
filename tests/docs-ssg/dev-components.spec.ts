import { expect, Page, test } from "@playwright/test";
import { ChildProcess, execFileSync, spawn } from "node:child_process";
import { copyFileSync, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { get, ClientRequest } from "node:http";
import { tmpdir } from "node:os";
import path from "node:path";

// This uses the real component fixture and both dev compiler
// targets. Installed templates and a first author's README trial are separate
// checks; the compiler pauses below observe actual in-progress dev states.
const repo = process.cwd();
const fixture = path.join(repo, "tests/docs-ssg/fixtures/components");
const bin = path.resolve(repo, process.env.RUSTIFY_DOCS_BIN || "target/docs-cli/rustify-docs");
const buildTimeout = 600_000;

type Timing = { profile: string; target: string | null; elapsed_ms: number; completed_at_ms: number; fresh_artifacts: number; compiled_artifacts: number; flags: string[]; success: boolean };
type Compile = { generation: string | null; completed_at_ms: number; host: Timing; wasm: Timing };
type State = { session: string; revision: number; sequence: number; building: boolean; phase: string; compile: Compile | null; good: number | null; ok: boolean; errors: number; warnings: number; diagnostics: { severity: string; rule: string; file: string | null; line: number | null; column: number | null; message: string; suggestion: string | null }[] };
type CompilerEvent = { phase: "start" | "end"; args: string[]; at: number; status?: number | null };
type Audit = { requests: string[]; failures: string[]; expectedStreamClosures: string[]; responses: { url: string; status: number; mime: string }[]; console: string[]; pageErrors: string[]; csp: string[] };
type Tree = { root: Element; nodes: Node[]; ids: [Element, string][]; source: Element; sourceNodes: Node[]; values: string[] };

declare global {
    interface Window {
        __dev_component_trees?: Tree[];
        __dev_component_document?: string;
        __dev_component_csp(value: string): void;
    }
}

test.beforeAll(() => {
    if (!process.env.RUSTIFY_DOCS_BIN) {
        test.setTimeout(buildTimeout);
        execFileSync("mbx", ["build", "--locked", "--profile", "docs-cli", "-p", "rustify-docs-cli"], {
            cwd: repo, stdio: "inherit", env: { ...process.env, CARGO_TARGET_DIR: path.join(repo, "target") },
        });
    }
});

class Site {
    readonly dir = mkdtempSync(path.join(tmpdir(), "rustify-docs-dev-components-"));
    readonly root = path.join(this.dir, "site");
    readonly compiler = path.join(this.dir, "compiler.cjs");
    readonly calls = path.join(this.dir, "compiler.jsonl");
    readonly lifecycle = path.join(this.dir, "compiler-lifecycle.jsonl");
    readonly gate = path.join(this.dir, "hold-build");
    readonly entered = path.join(this.dir, "build-entered");
    lock: Buffer;

    constructor() {
        for (const directory of ["src", "content", "static"]) cpSync(path.join(fixture, directory), path.join(this.root, directory), { recursive: true });
        for (const file of ["Cargo.toml", "Cargo.lock", "rustify-docs.toml", "theme.json"]) copyFileSync(path.join(fixture, file), path.join(this.root, file));
        let manifest = this.read("Cargo.toml");
        for (const library of ["rustify-ui", "rustify-components", "rustify-docs"]) {
            const old = `path = "../../../../crates/${library}"`;
            expect(manifest.split(old)).toHaveLength(2);
            manifest = manifest.replace(old, `path = ${JSON.stringify(path.join(repo, "crates", library))}`);
        }
        this.write("Cargo.toml", manifest);
        const pair = JSON.parse(execFileSync("python3", [
            path.join(repo, "scripts/docs-pages-prepare.py"),
            "--project", path.join(this.root, "Cargo.toml"), "--cli", bin,
        ], { encoding: "utf8" }));
        expect(pair.project).toBe(realpathSync(this.root));
        this.lock = readFileSync(path.join(this.root, "Cargo.lock"));
        writeFileSync(this.calls, "");
        writeFileSync(this.lifecycle, "");
        // The wrapper preserves the command, argv, output and exit status.
        // Its log and gate are siblings of the watched project, not inputs.
        writeFileSync(this.compiler, `#!${process.execPath}\nconst fs = require("node:fs");
const { spawnSync } = require("node:child_process");
const args = process.argv.slice(2);
fs.appendFileSync(${JSON.stringify(this.calls)}, JSON.stringify(args) + "\\n");
const record = value => fs.appendFileSync(${JSON.stringify(this.lifecycle)}, JSON.stringify({ args, at: Date.now(), ...value }) + "\\n");
if (args[0] === "build") record({ phase: "start" });
if (args[0] === "build" && fs.existsSync(${JSON.stringify(this.gate)})) {
    fs.writeFileSync(${JSON.stringify(this.entered)}, JSON.stringify(args));
    const wait = new Int32Array(new SharedArrayBuffer(4));
    while (fs.existsSync(${JSON.stringify(this.gate)})) Atomics.wait(wait, 0, 0, 20);
}
const result = spawnSync(${JSON.stringify(process.env.RUSTIFY_DOCS_CARGO || "mbx")}, args, { stdio: "inherit", env: process.env });
if (args[0] === "build") record({ phase: "end", status: result.status });
if (result.error) { console.error(result.error); process.exit(1); }
process.exit(result.status === null ? 1 : result.status);
`, { mode: 0o755 });
        this.hold();
    }

    read(file: string) { return readFileSync(path.join(this.root, file), "utf8"); }
    write(file: string, text: string) { writeFileSync(path.join(this.root, file), text); }
    edit(file: string, from: string, to: string) {
        const source = this.read(file);
        expect(source).toContain(from);
        this.write(file, source.replace(from, to));
    }
    commands(): string[][] { return readFileSync(this.calls, "utf8").trim().split("\n").filter(Boolean).map(line => JSON.parse(line)); }
    compilerEvents(): CompilerEvent[] { return readFileSync(this.lifecycle, "utf8").trim().split("\n").filter(Boolean).map(line => JSON.parse(line)); }
    hold() { rmSync(this.entered, { force: true }); writeFileSync(this.gate, "Hold the actual compiler until the browser observes the state.\n"); }
    release() { rmSync(this.gate, { force: true }); }
    remove() { rmSync(this.dir, { recursive: true, force: true }); }
}

class Dev {
    log = "";
    readonly states: State[] = [];
    readonly streamErrors: string[] = [];
    private events?: ClientRequest;
    private stopping = false;

    private constructor(readonly child: ChildProcess, readonly url: string) {}

    static async start(site: Site, root = site.root): Promise<Dev> {
        const child = spawn(bin, ["dev", "--port", "0"], {
            cwd: root, detached: true, env: { ...process.env, RUSTIFY_DOCS_CARGO: site.compiler },
        });
        let log = "";
        child.stderr!.on("data", chunk => { log += chunk; });
        const url = await new Promise<string>((resolve, reject) => {
            let out = "";
            child.stdout!.on("data", chunk => {
                out += chunk;
                const match = /^dev: (\S+)\n/m.exec(out);
                if (match) resolve(match[1]);
            });
            child.on("error", reject);
            child.on("exit", code => reject(new Error(`dev exited ${code}: ${out}${log}`)));
        });
        const server = new Dev(child, url);
        server.log = log;
        child.stderr!.on("data", chunk => { server.log += chunk; });
        const eventUrl = new URL("/__rustify-docs/events", url);
        eventUrl.searchParams.set("path", new URL("counter/", url).pathname);
        eventUrl.searchParams.set("good", "0");
        server.events = get(eventUrl, response => {
            if (response.statusCode !== 200) server.streamErrors.push(`SSE response ${response.statusCode}`);
            response.setEncoding("utf8");
            let pending = "";
            response.on("data", chunk => {
                pending += chunk;
                let end: number;
                while ((end = pending.indexOf("\n\n")) !== -1) {
                    const message = pending.slice(0, end);
                    pending = pending.slice(end + 2);
                    if (message.startsWith("event: state\n")) server.states.push(JSON.parse(message.slice("event: state\ndata: ".length)));
                }
            });
            response.on("error", error => { if (!server.stopping) server.streamErrors.push(String(error)); });
        });
        server.events.on("error", error => { if (!server.stopping) server.streamErrors.push(String(error)); });
        return server;
    }

    state(): State | undefined { return this.states.at(-1); }

    async stop() {
        this.stopping = true;
        this.events?.destroy();
        if (this.child.exitCode !== null || this.child.signalCode !== null) return;
        const exited = new Promise(resolve => this.child.once("exit", resolve));
        // The compiler wrapper and its actual compiler share this new process
        // group; a failed assertion must not leave a build running afterwards.
        process.kill(-this.child.pid!, "SIGTERM");
        await exited;
    }
}

let site: Site;
let server: Dev | undefined;
let audit: Audit;
type DependencyPackage = { id: string; name: string; version: string; manifest_path: string; source: string | null };
type DependencyNode = { id: string; features: string[]; deps: { name: string; pkg: string }[] };
type DependencyGraph = { mode: string; author: string; packages: DependencyPackage[]; resolve: { nodes: DependencyNode[] } };
let dependencyEvidence: { metadata: DependencyGraph[]; observations: unknown[] };

test.beforeEach(async ({ page }) => {
    site = new Site();
    server = undefined;
    dependencyEvidence = { metadata: [], observations: [] };
    audit = { requests: [], failures: [], expectedStreamClosures: [], responses: [], console: [], pageErrors: [], csp: [] };
    await page.exposeBinding("__dev_component_csp", (_, value: string) => audit.csp.push(value));
    await page.addInitScript(() => {
        document.addEventListener("securitypolicyviolation", event => window.__dev_component_csp(`${event.violatedDirective} ${event.blockedURI}`));
    });
    page.on("request", request => audit.requests.push(request.url()));
    page.on("requestfailed", request => {
        const message = `${request.url()} ${request.failure()?.errorText}`;
        if (new URL(request.url()).pathname === "/__rustify-docs/events" && request.failure()?.errorText === "net::ERR_ABORTED") audit.expectedStreamClosures.push(message);
        else audit.failures.push(message);
    });
    page.on("response", response => audit.responses.push({ url: response.url(), status: response.status(), mime: response.headers()["content-type"] || "" }));
    page.on("console", message => { if (message.type() === "error") audit.console.push(message.text()); });
    page.on("pageerror", error => audit.pageErrors.push(String(error)));
});

test.afterEach(async ({}, info) => {
    try {
        await info.attach("dev-component-browser-audit.json", { body: JSON.stringify(audit, null, 2), contentType: "application/json" });
        await info.attach("dev-component-states.json", { body: JSON.stringify(server?.states || [], null, 2), contentType: "application/json" });
        await info.attach("dev-component-compiler-commands.json", { body: JSON.stringify(site.commands(), null, 2), contentType: "application/json" });
        await info.attach("dev-component-compiler-lifecycle.json", { body: JSON.stringify(site.compilerEvents(), null, 2), contentType: "application/json" });
        await info.attach("dev-component-cli.log", { body: server?.log || "", contentType: "text/plain" });
        if (dependencyEvidence.metadata.length) {
            await info.attach("dev-component-dependencies.json", { body: JSON.stringify(dependencyEvidence, null, 2), contentType: "application/json" });
            await info.attach("dev-component-site-Cargo.toml", { body: site.read("Cargo.toml"), contentType: "text/plain" });
            if (existsSync(path.join(site.dir, "Cargo.lock.before-dev"))) {
                await info.attach("dev-component-lock-before-dev", { body: readFileSync(path.join(site.dir, "Cargo.lock.before-dev")), contentType: "text/plain" });
            }
            await info.attach("dev-component-lock-after-dev", { body: site.read("Cargo.lock"), contentType: "text/plain" });
        }
        const history = path.join(site.root, ".rustify-docs/logs/history");
        if (existsSync(history)) {
            const timings = readdirSync(history).map(directory => JSON.parse(readFileSync(path.join(history, directory, "timing.json"), "utf8")));
            await info.attach("dev-component-compile-history.json", { body: JSON.stringify(timings, null, 2), contentType: "application/json" });
        }
    } finally {
        try { await server?.stop(); } finally { site.remove(); }
    }
});

const islands = (page: Page) => page.locator("[data-rustify-island]");
const source = (page: Page) => page.locator(".rd-demo-source");
const values = (page: Page) => page.getByTestId("counter-value");

async function ready(page: Page, demoValue: string | string[] = "7") {
    await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(4, { timeout: buildTimeout });
    await expect(values(page)).toHaveText(["2", "2", "2", ...(Array.isArray(demoValue) ? demoValue : [demoValue])]);
    for (const island of await islands(page).all()) {
        const metadata = JSON.parse((await island.getAttribute("data-rustify-island-metadata"))!);
        const field = island.locator("fieldset[data-rd-island-controls]");
        await expect(field).toHaveJSProperty("disabled", false);
        await expect(field).toHaveJSProperty("inert", false);
        await expect(field.locator(":scope > [data-rustify-scope]")).toHaveAttribute("data-rustify-mounted", metadata.seed);
        await expect(field.locator("[style]")).toHaveCount(0);
    }
    expect(await page.evaluate(() => {
        const ids = [...document.querySelectorAll("[id]")].map(node => node.id);
        return new Set(ids).size === ids.length;
    })).toBe(true);
    await page.evaluate(() => document.fonts.ready);
}

async function sourceText(page: Page, original: string) {
    const fn = original.match(/(?:\/\/\/[^\r\n]*\r?\n)*pub fn counter_demo\([^)]*\)[\s\S]*?^\}(?:\r?\n|$)/m)?.[0];
    expect(fn).toBeTruthy();
    expect(await source(page).locator("template.rd-code-raw").evaluateAll(nodes => nodes.map(node => (node as HTMLTemplateElement).content.textContent))).toEqual([fn, original]);
}

async function captureTrees(page: Page) {
    await page.evaluate(() => {
        const nodes = (root: Node) => {
            const items: Node[] = [root];
            const walker = document.createTreeWalker(root, NodeFilter.SHOW_ALL);
            while (walker.nextNode()) items.push(walker.currentNode);
            return items;
        };
        const source = document.querySelector(".rd-demo-source")!;
        window.__dev_component_trees = [...document.querySelectorAll("[data-rustify-island]")].map(block => {
            const root = block.querySelector("fieldset[data-rd-island-controls]")!.firstElementChild!;
            return { root, nodes: nodes(root), ids: [...root.querySelectorAll("[id]")].map(node => [node, node.id] as [Element, string]), source, sourceNodes: nodes(source), values: [...root.querySelectorAll("input")].map(node => node.value) };
        });
        window.__dev_component_document = "the actual last good document";
    });
}

async function retained(page: Page) {
    const records = await page.evaluate(() => window.__dev_component_trees!.map(tree => ({
        nodes: tree.nodes.length, retained: tree.nodes.filter(node => node.isConnected && tree.root.contains(node)).length,
        root: [...document.querySelectorAll("fieldset[data-rd-island-controls]")].some(field => field.firstElementChild === tree.root),
        ids: tree.ids.every(([node, id]) => node.id === id),
        source: document.querySelector(".rd-demo-source") === tree.source,
        sourceNodes: tree.sourceNodes.every(node => node.isConnected && tree.source.contains(node)),
        values: [...tree.root.querySelectorAll("input")].map(node => node.value), originalValues: tree.values,
    })));
    expect(records).toHaveLength(4);
    for (const record of records) {
        expect(record.retained).toBe(record.nodes);
        expect(record.root && record.ids && record.source && record.sourceNodes).toBe(true);
        expect(record.values).toEqual(record.originalValues);
    }
    expect(await page.evaluate(() => window.__dev_component_document)).toBe("the actual last good document");
}

async function settled(after = -1, ok = true): Promise<State> {
    await expect.poll(() => {
        const state = server!.state();
        return !!state && state.revision > after && !state.building && state.ok === ok;
    }, { timeout: buildTimeout }).toBe(true);
    return server!.state()!;
}

function compilation(state: State) {
    expect(state.compile).not.toBeNull();
    const compiled = state.compile!;
    expect(compiled.completed_at_ms).toBe(Math.max(compiled.host.completed_at_ms, compiled.wasm.completed_at_ms));
    expect(compiled.host.profile).toBe("docs-host");
    expect(compiled.wasm.profile).toBe("docs-wasm");
    expect(compiled.host.target).toBeNull();
    expect(compiled.wasm.target).toBe("wasm32-unknown-unknown");
    for (const timing of [compiled.host, compiled.wasm]) {
        expect(timing.success).toBe(true);
        expect(timing.flags).toContain("--locked");
        expect(timing.elapsed_ms).toBeGreaterThanOrEqual(0);
        expect(timing.fresh_artifacts + timing.compiled_artifacts).toBeGreaterThan(0);
    }
    return compiled;
}

function serialBuilds() {
    const builds = site.commands().filter(args => args[0] === "build");
    expect(builds.length).toBeGreaterThanOrEqual(2);
    expect(builds.length % 2).toBe(0);
    const events = site.compilerEvents();
    expect(events).toHaveLength(builds.length * 2);
    for (let index = 0; index < builds.length; index++) {
        const args = builds[index];
        expect(args[args.indexOf("--profile") + 1]).toBe(index % 2 === 0 ? "docs-host" : "docs-wasm");
        const [start, end] = events.slice(index * 2, index * 2 + 2);
        expect([start.phase, end.phase]).toEqual(["start", "end"]);
        expect(start.args).toEqual(args);
        expect(end.args).toEqual(args);
        expect(end.at).toBeGreaterThanOrEqual(start.at);
        if (index > 0) expect(start.at).toBeGreaterThanOrEqual(events[index * 2 - 1].at);
    }
    return builds;
}

async function cleared(page: Page, state: State) {
    expect(state.ok && !state.building).toBe(true);
    expect(state.errors).toBe(0);
    expect(state.diagnostics.filter(diagnostic => diagnostic.severity === "error")).toEqual([]);
    await expect(page.locator(".rd-dev-panel")).toBeHidden();
    if (state.warnings) {
        await expect(page.locator(".rd-dev-marker")).toHaveText(`${state.warnings} warning${state.warnings === 1 ? "" : "s"}`);
    } else await expect(page.locator(".rd-dev")).toBeHidden();
}

async function newDocument(page: Page, file: string) {
    await page.waitForFunction(original => {
        const raw = document.querySelectorAll<HTMLTemplateElement>(".rd-demo-source template.rd-code-raw");
        return window.__dev_component_document === undefined && raw.length === 2 && raw[1].content.textContent === original;
    }, file, { timeout: 10_000 });
}

async function layouts(page: Page, state: string) {
    const viewport = page.viewportSize()!;
    const position = await page.evaluate(() => scrollY);
    const preference = await page.locator("html").getAttribute("data-theme") || "system";
    for (const variant of [
        { name: "desktop-light", mode: "light", width: 1280, height: 900 },
        { name: "desktop-dark", mode: "dark", width: 1280, height: 900 },
        { name: "narrow-light", mode: "light", width: 375, height: 812 },
    ]) {
        await page.setViewportSize({ width: variant.width, height: variant.height });
        await page.locator(`[data-rd-mode="${variant.mode}"]`).click();
        await page.evaluate(async () => {
            await document.fonts.ready;
            await Promise.allSettled(document.getAnimations().filter(animation => animation instanceof CSSTransition).map(animation => animation.finished));
            await new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
        });
        await expect(page.locator("html")).toHaveAttribute("data-theme", variant.mode);
        expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(variant.width);
        await test.info().attach(`${state}-${variant.name}.png`, { body: await page.screenshot(), contentType: "image/png" });
    }
    await page.setViewportSize(viewport);
    await page.locator(`[data-rd-mode="${preference}"]`).click();
    await page.evaluate(y => window.scrollTo(0, y), position);
}

async function healthy() {
    expect(audit.csp).toEqual([]);
    expect(audit.failures).toEqual([]);
    expect(audit.responses.filter(response => response.status >= 400)).toEqual([]);
    expect(audit.console).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    expect(server!.streamErrors).toEqual([]);
    expect(audit.requests.filter(url => new URL(url).origin !== new URL(server!.url).origin || /makepad/i.test(url))).toEqual([]);
    expect(readFileSync(path.join(site.root, "Cargo.lock"))).toEqual(site.lock);
    expect(existsSync(path.join(site.root, "dist"))).toBe(false);
    const sequences = server!.states.map(state => state.sequence);
    expect(sequences.every((value, index) => index === 0 || value > sequences[index - 1])).toBe(true);
    serialBuilds();
}

test("the address and first building shell precede the actual compilation, then content saves reuse its pair", async ({ page }) => {
    test.setTimeout(900_000);
    server = await Dev.start(site);
    await page.goto(new URL("counter/", server.url).href);
    await expect(page.getByRole("heading", { name: "Building your site", exact: true })).toBeVisible();
    await expect(islands(page)).toHaveCount(0);
    await expect.poll(() => existsSync(site.entered)).toBe(true);
    await expect.poll(() => server!.state()?.phase).toBe("host");
    expect(server.state()!.building).toBe(true);
    expect(server.state()!.good).toBeNull();
    await layouts(page, "first-building");

    let releaseWasm!: () => void;
    const heldWasm = new Promise<void>(resolve => { releaseWasm = resolve; });
    await page.route("**/assets/islands/**/app_bg.wasm", async route => { await heldWasm; await route.continue(); });
    site.release();
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("Registered counters", { timeout: buildTimeout });
    await expect(page.locator('[data-rustify-island][data-rd-island-state="loading"]')).toHaveCount(4);
    await captureTrees(page);
    await sourceText(page, site.read("src/demos.rs"));
    releaseWasm();
    await ready(page);
    await retained(page);
    const committed = await settled();
    const compiled = compilation(committed);
    const commands = site.commands();
    const builds = serialBuilds();
    const attempts = [...new Map(server.states.filter(state => state.compile).map(state => [state.compile!.completed_at_ms, state.compile!] as const)).values()];
    expect(attempts).toHaveLength(builds.length / 2);
    for (const [index, attempt] of attempts.entries()) {
        expect(attempt.host.flags).toEqual(builds[index * 2]);
        expect(attempt.wasm.flags).toEqual(builds[index * 2 + 1]);
    }
    expect(attempts.at(-1)).toEqual(compiled);
    expect(server.states.filter(state => state.good !== null).every(state => state.compile?.completed_at_ms === compiled.completed_at_ms)).toBe(true);
    await expect(page.locator(".rd-dev")).toHaveAttribute("data-revision", String(committed.revision));
    if (attempts.length > 1) {
        expect(server.log).toContain("dev: inputs changed; discarding the candidate and rebuilding");
        for (const discarded of attempts.slice(0, -1)) {
            expect(server.states.filter(state => state.compile?.completed_at_ms === discarded.completed_at_ms).every(state => state.good === null)).toBe(true);
        }
    }
    await page.getByTestId("counter-input").first().fill("20");
    await expect(values(page)).toHaveText(["20", "2", "2", "7"]);
    await page.getByTestId("counter-increment").first().click();
    await expect(values(page)).toHaveText(["21", "2", "2", "7"]);
    await cleared(page, committed);

    let previous = server.state()!.revision;
    site.edit("content/counter.md", "Each counter starts with the same props and child content.", "A Markdown save reuses the already compiled view and its source.");
    await expect(page.getByText("A Markdown save reuses the already compiled view and its source.", { exact: false })).toBeVisible();
    expect((await settled(previous)).compile).toEqual(compiled);
    expect(site.commands()).toEqual(commands);
    await ready(page);

    previous = server.state()!.revision;
    site.edit("rustify-docs.toml", 'name = "Component fixture"', 'name = "Component handbook"');
    await expect(page).toHaveTitle("Registered counters | Component handbook");
    expect((await settled(previous)).compile).toEqual(compiled);
    expect(site.commands()).toEqual(commands);
    await expect(page.locator('meta[property="og:site_name"]')).toHaveAttribute("content", "Component handbook");
    await ready(page);
    await sourceText(page, site.read("src/demos.rs"));

    previous = server.state()!.revision;
    const theme = JSON.parse(site.read("theme.json"));
    theme.styles.light.background = "#f2f7ff";
    site.write("theme.json", JSON.stringify(theme, null, 2) + "\n");
    await page.locator('[data-rd-mode="light"]').click();
    await expect(page.locator("body")).toHaveCSS("background-color", "rgb(242, 247, 255)");
    expect((await settled(previous)).compile).toEqual(compiled);
    expect(site.commands()).toEqual(commands);
    await ready(page);
    await sourceText(page, site.read("src/demos.rs"));
    const wasm = audit.responses.filter(response => response.url.endsWith("/app_bg.wasm"));
    expect(wasm.length).toBeGreaterThan(0);
    expect(new Set(wasm.map(response => response.url)).size).toBe(1);
    expect(wasm.every(response => response.status === 200 && response.mime.split(";")[0] === "application/wasm")).toBe(true);
    await healthy();
});

test("Rust saves keep the original interactive document until a complete new pair, and diagnostics recover at its scroll position", async ({ page }) => {
    test.setTimeout(900_000);
    server = await Dev.start(site);
    await page.goto(new URL("counter/", server.url).href);
    site.release();
    await ready(page);
    const firstState = await settled();
    const first = compilation(firstState);
    const original = site.read("src/demos.rs");
    const firstWasm = audit.requests.filter(url => url.endsWith("/app_bg.wasm")).at(-1)!;
    expect(firstWasm).toBeTruthy();
    await sourceText(page, original);
    await source(page).getByRole("tab", { name: "Full file", exact: true }).click();
    await page.getByTestId("counter-input").first().fill("30");
    await expect(values(page)).toHaveText(["30", "2", "2", "7"]);
    await captureTrees(page);

    const changed = original.replace("*value += 1", "*value += 3").replace('        7,\n        "Résumé', '        11,\n        "Résumé');
    expect(changed).not.toBe(original);
    site.hold();
    site.write("src/demos.rs", changed);
    await expect.poll(() => existsSync(site.entered)).toBe(true);
    await expect(page.locator(".rd-dev-marker")).toContainText("Building");
    await expect(page.locator(".rd-dev-marker")).toContainText("showing the last good version");
    await retained(page);
    await sourceText(page, original);
    await layouts(page, "rust-building");
    await page.getByTestId("counter-increment").first().click();
    await expect(values(page)).toHaveText(["31", "2", "2", "7"]);
    await source(page).getByRole("tab", { name: "Full file", exact: true }).click();
    const scroll = await page.evaluate(() => {
        const maximum = document.documentElement.scrollHeight - innerHeight;
        if (maximum < 300) throw new Error("the real source page must have room for a scroll-restoration check");
        const position = Math.min(500, maximum - 50);
        window.scrollTo(0, position);
        return position;
    });
    await expect.poll(() => page.evaluate(() => scrollY)).toBe(scroll);
    site.release();
    const secondState = await settled(firstState.revision);
    await newDocument(page, changed);
    await sourceText(page, changed);
    await ready(page, "11");
    await expect(source(page).getByRole("tab", { name: "Full file", exact: true })).toHaveAttribute("aria-selected", "true");
    await expect.poll(() => page.evaluate(() => scrollY)).toBe(scroll);
    expect(await page.evaluate(() => window.__dev_component_document)).toBeUndefined();
    const second = compilation(secondState);
    expect(second.completed_at_ms).toBeGreaterThan(first.completed_at_ms);
    expect(second.generation).not.toBe(first.generation);
    const secondWasm = audit.requests.filter(url => url.endsWith("/app_bg.wasm")).at(-1)!;
    expect(secondWasm).not.toBe(firstWasm);
    const group = secondWasm.slice(0, secondWasm.lastIndexOf("/") + 1);
    expect(audit.responses.some(response => response.url === `${group}app.js` && response.status === 200)).toBe(true);
    expect(audit.responses.some(response => response.url === secondWasm && response.status === 200 && response.mime.split(";")[0] === "application/wasm")).toBe(true);
    await page.getByTestId("counter-increment").first().click();
    await expect(values(page)).toHaveText(["5", "2", "2", "11"]);

    const bad = changed.replace("RwSignal::new(initial)", "RwSignal::new(initial + missing_counter_step)");
    const line = bad.slice(0, bad.indexOf("missing_counter_step")).split("\n").length;
    const column = bad.split("\n")[line - 1].indexOf("missing_counter_step") + 1;
    const previous = server.state()!.revision;
    await captureTrees(page);
    site.write("src/demos.rs", bad);
    const failed = await settled(previous, false);
    expect(failed.compile?.host.success).toBe(false);
    expect(failed.compile?.wasm.success).toBe(false);
    const errors = failed.diagnostics.filter(diagnostic => diagnostic.rule === "rust-compile" && diagnostic.file === "src/demos.rs" && diagnostic.line === line && diagnostic.column === column);
    expect(errors.length).toBeGreaterThan(0);
    const panel = page.locator(".rd-dev-panel");
    await expect(panel).toBeVisible();
    const fileGroup = panel.locator(".rd-dev-list > section").filter({ has: page.getByRole("heading", { name: "src/demos.rs", exact: true }) });
    await expect(fileGroup).toHaveCount(1);
    await expect(fileGroup.locator(".rd-dev-file")).toHaveText("src/demos.rs");
    await expect(fileGroup.locator('.rd-dev-item[data-severity="error"] .rd-dev-location')).toHaveText(Array(errors.length).fill(`${line}:${column}`));
    await expect(panel).toContainText("missing_counter_step");
    await expect(panel).toContainText("showing the last good version");
    await retained(page);
    await sourceText(page, changed);
    await layouts(page, "rust-error");
    await panel.getByRole("button", { name: "Hide problems" }).click();
    await expect(panel).toBeHidden();
    await expect(page.locator(".rd-dev-marker")).toBeVisible();
    await page.getByTestId("counter-increment").first().click();
    await expect(values(page)).toHaveText(["8", "2", "2", "11"]);
    await page.evaluate(y => window.scrollTo(0, y), scroll);

    const repaired = changed.replace("*value += 3", "*value += 5").replace('        11,\n        "Résumé', '        13,\n        "Résumé');
    site.write("src/demos.rs", repaired);
    const fixedState = await settled(failed.revision);
    await newDocument(page, repaired);
    await sourceText(page, repaired);
    await ready(page, "13");
    await cleared(page, fixedState);
    await expect.poll(() => page.evaluate(() => scrollY)).toBe(scroll);
    await expect(source(page).getByRole("tab", { name: "Full file", exact: true })).toHaveAttribute("aria-selected", "true");
    const fixed = compilation(fixedState);
    expect(fixed.completed_at_ms).toBeGreaterThan(second.completed_at_ms);
    await page.getByTestId("counter-increment").first().click();
    await expect(values(page)).toHaveText(["7", "2", "2", "13"]);
    await healthy();
});

function localPackage(directory: string, files: Record<string, string>) {
    for (const [file, text] of Object.entries(files)) {
        mkdirSync(path.dirname(path.join(site.dir, directory, file)), { recursive: true });
        site.write(path.join("..", directory, file), text);
    }
}

function lockLocalDependencies() {
    // Resolve each real feature separately, without building either target.
    // This happens before dev starts; --locked builds must not repair the lock.
    for (const feature of ["ssr", "hydrate"]) {
        const metadata = JSON.parse(execFileSync(process.env.RUSTIFY_DOCS_CARGO || "mbx", [
            "metadata", "--offline", "--format-version", "1", "--no-default-features", "--features", feature,
        ], { cwd: site.root, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 })) as { packages: DependencyPackage[]; resolve: { root: string; nodes: DependencyNode[] } };
        const author = metadata.resolve.root;
        const packages = metadata.packages.filter(pkg => pkg.id === author
            || (pkg.source === null && ["browser-palette", "payload"].includes(pkg.name))
            || ["rustify-ui", "rustify-components", "rustify-docs"].includes(pkg.name))
            .map(({ id, name, version, manifest_path, source }) => ({ id, name, version, manifest_path, source }));
        const ids = new Set(packages.map(pkg => pkg.id));
        const nodes = metadata.resolve.nodes.filter(node => ids.has(node.id)).map(({ id, features, deps }) => ({
            id, features, deps: deps.filter(dep => ids.has(dep.pkg)).map(({ name, pkg }) => ({ name, pkg })),
        }));
        dependencyEvidence.metadata.push({ mode: feature, author, packages, resolve: { nodes } });
    }
    copyFileSync(path.join(site.root, "Cargo.lock"), path.join(site.dir, "Cargo.lock.before-dev"));
    site.lock = readFileSync(path.join(site.dir, "Cargo.lock.before-dev"));
}

async function dependencySettled(previous: State, ok = true) {
    // A missing watch event must fail promptly, not consume the build budget.
    await expect.poll(() => {
        const state = server!.state()!;
        return state.revision > previous.revision || (state.sequence > previous.sequence && state.building);
    }, { timeout: 10_000 }).toBe(true);
    return settled(previous.revision, ok);
}

async function dependencySsr(page: Page, demoValues: string[]) {
    const response = await page.request.get(new URL("counter/", server!.url).href);
    expect(response.status()).toBe(200);
    const html = await response.text();
    const ssrValues = await page.evaluate(html => {
        const document = new DOMParser().parseFromString(html, "text/html");
        return [...document.querySelectorAll('[data-testid="counter-value"]')].map(node => node.textContent);
    }, html);
    expect(ssrValues).toEqual(["2", "2", "2", ...demoValues]);
    return ssrValues;
}

async function dependencyReady(page: Page, demoValues: string[]) {
    try {
        await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(4, { timeout: 30_000 });
        await ready(page, demoValues);
    } catch (error) {
        await test.info().attach("dev-dependency-readiness-failure.png", { body: await page.screenshot(), contentType: "image/png" });
        throw error;
    }
}

async function dependencySnapshot(page: Page, demoValues: string[]) {
    const ssrValues = await dependencySsr(page, demoValues);
    await page.goto(new URL("counter/", server!.url).href);
    await dependencyReady(page, demoValues);
    await sourceText(page, site.read("src/demos.rs"));
    const wasmUrl = audit.requests.filter(url => url.endsWith("/app_bg.wasm")).at(-1)!;
    expect(wasmUrl).toBeTruthy();
    const wasm = await page.request.get(wasmUrl);
    expect(wasm.status()).toBe(200);
    expect(wasm.headers()["content-type"].split(";")[0]).toBe("application/wasm");
    const snapshot = { ssrValues, wasmUrl, wasmSha256: createHash("sha256").update(await wasm.body()).digest("hex"), state: server!.state()! };
    dependencyEvidence.observations.push(snapshot);
    return snapshot;
}

function updatedDependency(previous: Awaited<ReturnType<typeof dependencySnapshot>>, next: Awaited<ReturnType<typeof dependencySnapshot>>) {
    const before = compilation(previous.state);
    const after = compilation(next.state);
    expect(before.generation).toBeTruthy();
    expect(after.generation).toBeTruthy();
    expect(after.generation).not.toBe(before.generation);
    expect(after.completed_at_ms).toBeGreaterThan(before.completed_at_ms);
    expect(next.wasmSha256).not.toBe(previous.wasmSha256);
    expect(next.wasmUrl).not.toBe(previous.wasmUrl);
}

test("hydrate-only dependency manifests invalidate wasm and behavior without compiling Markdown", async ({ page }) => {
    test.setTimeout(900_000);
    localPackage("browser-palette", {
        "Cargo.toml": `[package]\nname = "browser-palette"\nversion = "0.1.0"\nedition = "2021"\n\n[workspace]\n\n[features]\ndefault = []\nalternate = []\n`,
        "src/lib.rs": `pub fn value() -> i32 { if cfg!(feature = "alternate") { 27 } else { 17 } }\n`,
    });
    site.edit("Cargo.toml", 'hydrate = ["rustify-docs/hydrate"', 'hydrate = ["dep:browser-palette", "rustify-docs/hydrate"');
    site.edit("Cargo.toml", "[dependencies]\n", '[dependencies]\nbrowser-palette = { path = "../browser-palette", optional = true }\n');
    site.edit("src/demos.rs", '*value += 1', `{
                        #[cfg(feature = "hydrate")]
                        { *value = browser_palette::value(); }
                        #[cfg(not(feature = "hydrate"))]
                        { *value += 1; }
                    }`);
    // Both colors are in the unchanged site source, so only the dependency's
    // default feature determines the post-click value and its external class.
    site.edit("src/demos.rs", '<output data-testid="counter-value">', `<output data-testid="counter-value" class={move || match value.get() {
                    17 => "bg-[#123abc]",
                    27 => "bg-[#fedcba]",
                    _ => "",
                }}>`);
    const manifest = site.read("../browser-palette/Cargo.toml");
    const siteSource = site.read("src/demos.rs");
    dependencyEvidence.observations.push({ initialManifest: manifest, siteSource });
    lockLocalDependencies();
    for (const entry of dependencyEvidence.metadata) {
        const palette = entry.packages.find(pkg => pkg.name === "browser-palette");
        expect(!!palette && entry.resolve.nodes.some(node => node.id === palette.id)).toBe(entry.mode === "hydrate");
    }
    server = await Dev.start(site);
    site.release();
    await settled();
    await page.goto(new URL("counter/", server.url).href);
    await dependencyReady(page, ["7"]);
    const first = await dependencySnapshot(page, ["7"]);
    await page.getByTestId("counter-increment").last().click();
    await expect(values(page)).toHaveText(["2", "2", "2", "17"]);
    await expect(values(page).last()).toHaveCSS("background-color", "rgb(18, 58, 188)");
    await test.info().attach("dev-hydrate-only-initial.png", { body: await page.screenshot(), contentType: "image/png" });

    site.edit("../browser-palette/Cargo.toml", "default = []", 'default = ["alternate"]');
    const changedState = await dependencySettled(first.state);
    // Record the completion before asserting its generation: a stale cache
    // can observe this save but incorrectly reuse its already compiled pair.
    dependencyEvidence.observations.push({ changedManifest: site.read("../browser-palette/Cargo.toml"), changedState });
    const second = await dependencySnapshot(page, ["7"]);
    await page.getByTestId("counter-increment").last().click();
    await expect(values(page)).toHaveText(["2", "2", "2", "27"]);
    await expect(values(page).last()).toHaveCSS("background-color", "rgb(254, 220, 186)");
    updatedDependency(first, second);
    expect(site.read("src/demos.rs")).toBe(siteSource);
    expect(site.read("../browser-palette/Cargo.toml")).toBe(manifest.replace("default = []", 'default = ["alternate"]'));
    await cleared(page, changedState);
    await test.info().attach("dev-hydrate-only-alternate.png", { body: await page.screenshot(), contentType: "image/png" });

    const commands = site.commands();
    site.edit("content/counter.md", "Each counter starts with the same props and child content.", "Markdown still reuses the hydrate-only dependency pair.");
    const markdownState = await dependencySettled(changedState);
    await expect(page.getByText("Markdown still reuses the hydrate-only dependency pair.", { exact: false })).toBeVisible();
    await ready(page);
    expect(markdownState.compile).toEqual(changedState.compile);
    expect(site.commands()).toEqual(commands); // No metadata, helper Cargo or build calls.
    const markdown = await dependencySnapshot(page, ["7"]);
    expect(markdown.wasmSha256).toBe(second.wasmSha256);
    await page.getByTestId("counter-increment").last().click();
    await expect(values(page)).toHaveText(["2", "2", "2", "27"]);
    await expect(values(page).last()).toHaveCSS("background-color", "rgb(254, 220, 186)");
    await healthy();
});

test("renamed same-name build inputs track each dependency save, deletion and recovery", async ({ page }) => {
    test.setTimeout(900_000);
    for (const [directory, version, initial] of [["payload-v1", "1.0.0", "7"], ["payload-v2", "2.0.0", "19"]]) {
        localPackage(directory, {
            "Cargo.toml": `[package]\nname = "payload"\nversion = "${version}"\nedition = "2021"\nbuild = "build.rs"\n\n[workspace]\n`,
            "payload.txt": `${initial}\n`,
            "build.rs": `fn main() {
    println!("cargo:rerun-if-changed=payload.txt");
    let value: i32 = std::fs::read_to_string("payload.txt").expect("payload.txt must exist").trim().parse().unwrap();
    let output = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("payload.rs");
    std::fs::write(output, format!("pub fn value() -> i32 {{ {value} }}\\n")).unwrap();
}
`,
            "src/lib.rs": 'include!(concat!(env!("OUT_DIR"), "/payload.rs"));\n',
        });
    }
    site.edit("Cargo.toml", "[dependencies]\n", `[dependencies]
payload_v1 = { package = "payload", path = "../payload-v1" }
payload_v2 = { package = "payload", path = "../payload-v2" }
`);
    site.edit("src/demos.rs", `    counter(
        7,
        "Résumé — a real registered counter".to_string(),
        view! { <p>"This child is part of the same view on both targets."</p> }.into_any(),
    )`, `    view! {
        <div>
            {counter(payload_v1::value(), "Payload v1".to_string(), view! { <p>"First build input"</p> }.into_any())}
            {counter(payload_v2::value(), "Payload v2".to_string(), view! { <p>"Second build input"</p> }.into_any())}
        </div>
    }`);
    const siteSource = site.read("src/demos.rs");
    lockLocalDependencies();
    server = await Dev.start(site);
    site.release();
    await settled();
    await page.goto(new URL("counter/", server.url).href);
    await dependencyReady(page, ["7", "19"]);
    let current = await dependencySnapshot(page, ["7", "19"]);
    const expected = ["7", "19"];
    const clickBoth = async () => {
        for (const index of [3, 4]) await page.getByTestId("counter-increment").nth(index).click();
        await expect(values(page)).toHaveText(["2", "2", "2", ...expected.map(value => String(Number(value) + 1))]);
    };
    await clickBoth();
    for (const [index, directory, saved, restored] of [[0, "payload-v1", "14", "23"], [1, "payload-v2", "29", "37"]] as const) {
        const file = `../${directory}/payload.txt`;
        dependencyEvidence.observations.push({ save: file, value: saved, previous: current.state });
        site.write(file, `${saved}\n`);
        await dependencySettled(current.state);
        expected[index] = saved;
        const savedSnapshot = await dependencySnapshot(page, expected);
        updatedDependency(current, savedSnapshot);
        current = savedSnapshot;
        await clickBoth();
        await cleared(page, current.state);
        await captureTrees(page);

        const commands = site.commands().length;
        dependencyEvidence.observations.push({ delete: file, previous: current.state });
        rmSync(path.join(site.root, file));
        const failed = await dependencySettled(current.state, false);
        dependencyEvidence.observations.push({ deleted: file, failed });
        expect(failed.good).toBe(current.state.good);
        expect(failed.errors).toBeGreaterThan(0);
        expect(site.commands().length).toBeGreaterThan(commands);
        await expect(page.locator(".rd-dev-panel")).toContainText("payload.txt");
        await expect(page.locator(".rd-dev-panel")).toContainText("showing the last good version");
        await retained(page);
        await dependencySsr(page, expected);
        await expect(values(page)).toHaveText(["2", "2", "2", ...expected.map(value => String(Number(value) + 1))]);
        await page.getByTestId("counter-increment").nth(3 + index).click();
        await expect(values(page)).toHaveText(["2", "2", "2", ...expected.map((value, position) => String(Number(value) + (position === index ? 2 : 1)))]);
        const lastGood = await page.request.get(current.wasmUrl);
        expect(lastGood.status()).toBe(200);
        expect(createHash("sha256").update(await lastGood.body()).digest("hex")).toBe(current.wasmSha256);
        await test.info().attach(`dev-renamed-${directory}-deleted.png`, { body: await page.screenshot(), contentType: "image/png" });

        dependencyEvidence.observations.push({ restore: file, value: restored, previous: failed });
        site.write(file, `${restored}\n`);
        const recovered = await dependencySettled(failed);
        expected[index] = restored;
        const restoredSnapshot = await dependencySnapshot(page, expected);
        updatedDependency(current, restoredSnapshot);
        current = restoredSnapshot;
        await clickBoth();
        await cleared(page, recovered);
        await test.info().attach(`dev-renamed-${directory}-restored.png`, { body: await page.screenshot(), contentType: "image/png" });
    }
    expect(site.read("src/demos.rs")).toBe(siteSource);
    await healthy();
});

// The repairs the CLI tests assert, word for word, as the page shows them.
const builtinReference = "https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md#static-builtins";
type Repair = { rule: string; line: number; column: number; suggestion: string };

function lineOf(text: string, needle: string, offset = 0) {
    return text.split("\n").findIndex(line => line.includes(needle)) + 1 + offset;
}

function diagnosticProse(input: string) {
    const runs = [...input.matchAll(/`+/g)];
    let text = "";
    const codes: string[] = [];
    let start = 0;
    for (let i = 0; i < runs.length; i++) {
        const opening = runs[i];
        const j = runs.findIndex((closing, index) => index > i && closing[0].length === opening[0].length);
        if (j < 0) continue;
        let code = input.slice(opening.index! + opening[0].length, runs[j].index!);
        if (opening[0].length >= 2 && code.length >= 2 && code.startsWith(" ") && code.endsWith(" ")) code = code.slice(1, -1);
        text += input.slice(start, opening.index!) + code;
        codes.push(code);
        start = runs[j].index! + runs[j][0].length;
        i = j;
    }
    return { text: text + input.slice(start), codes };
}

async function expectRepairs(page: Page, state: State, file: string, repairs: Repair[]) {
    const order = (a: Repair, b: Repair) => a.line - b.line || a.column - b.column || a.rule.localeCompare(b.rule);
    const reported = state.diagnostics.filter(d => d.severity === "error" && d.file === file)
        .map(d => ({ rule: d.rule, line: d.line!, column: d.column!, suggestion: d.suggestion! }));
    expect(reported.sort(order)).toEqual([...repairs].sort(order));
    const group = page.locator(".rd-dev-panel .rd-dev-list > section").filter({ has: page.getByRole("heading", { name: file, exact: true }) });
    for (const repair of repairs) {
        const item = group.locator(".rd-dev-item")
            .filter({ has: page.locator(".rd-dev-location", { hasText: new RegExp(`^${repair.line}:${repair.column}$`) }) })
            .filter({ has: page.locator(".rd-dev-rule", { hasText: repair.rule }) });
        await expect(item).toHaveCount(1);
        // The panel shows the same text, formatting only its backticked names.
        const expected = diagnosticProse(repair.suggestion);
        expect(await item.locator(".rd-dev-suggestion").textContent()).toBe(expected.text);
        expect(await item.locator(".rd-dev-suggestion code").allTextContents()).toEqual(expected.codes);
    }
}

// Long candidate lists and the reference URL wrap inside the panel.
async function repairsFit(page: Page, label: string) {
    const viewport = page.viewportSize()!;
    for (const width of [375, 320]) {
        for (const mode of ["light", "dark"]) {
            await page.setViewportSize({ width, height: 812 });
            await page.locator(`[data-rd-mode="${mode}"]`).click();
            await expect(page.locator("html")).toHaveAttribute("data-theme", mode);
            const overflow = await page.evaluate(() => ({
                page: document.documentElement.scrollWidth - innerWidth,
                repairs: [...document.querySelectorAll<HTMLElement>(".rd-dev-suggestion")].map(node => node.scrollWidth - node.clientWidth),
            }));
            expect(overflow.page, `${width}px page`).toBeLessThanOrEqual(0);
            expect(overflow.repairs.every(extra => extra <= 0), `${width}px repairs ${overflow.repairs}`).toBe(true);
            await test.info().attach(`${label}-${width}-${mode}.png`, { body: await page.screenshot(), contentType: "image/png" });
        }
    }
    await page.setViewportSize(viewport);
    await page.locator('[data-rd-mode="system"]').click();
}

test("gaps embedded diagnostics: registered names, props and demos show the CLI repair over the last good page and clear after the fix", async ({ page }) => {
    test.setTimeout(900_000);
    server = await Dev.start(site);
    await page.goto(new URL("counter/", server.url).href);
    site.release();
    await ready(page);
    const good = await settled();
    const original = site.read("content/counter.md");
    await captureTrees(page);

    const cold = `${original}\n\`\`\`rustify-component\nname: MissingCounter\n\`\`\`\n\n\`\`\`rustify-component\nname: Counter\nprops:\n  unexpected: 1\n\`\`\`\n\n\`\`\`rustify-demo\nname: MissingDemo\n\`\`\`\n`;
    site.write("content/counter.md", cold);
    const failed = await settled(good.revision, false);
    const panel = page.locator(".rd-dev-panel");
    await expect(panel).toBeVisible();
    await expectRepairs(page, failed, "content/counter.md", [
        { rule: "unregistered-component", line: lineOf(cold, "name: MissingCounter", -1), column: 1, suggestion: "Use a component from `BrowserCounter`, `CardGrid`, `Counter`, `Steps`, `Tabs`; for an author component, enable `[rust]` and register its name in `site!`." },
        { rule: "unknown-property", line: lineOf(cold, "unexpected: 1"), column: 3, suggestion: "Use one of `initial` (`i32`; default expression `2`), `label` (`String`; default expression `\"Counter\".to_string()`) for `Counter`; rename or remove `unexpected`." },
        { rule: "unregistered-demo", line: lineOf(cold, "name: MissingDemo", -1), column: 1, suggestion: "Use a demo from `AsyncAfterAttachDemo`, `CatalogProxy`, `CounterDemo`, `FailureDemo`, `ThemeProbeDemo`; register demos in `site!`." },
    ]);
    expect(await page.evaluate(() => document.activeElement === document.body), "reading focus is not taken").toBe(true);
    await retained(page);
    await repairsFit(page, "gaps-registered");
    await panel.getByRole("button", { name: "Hide problems" }).click();
    const marker = page.locator(".rd-dev-marker");
    await expect(marker).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(panel.getByRole("button", { name: "Hide problems" })).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(marker).toBeFocused();

    // A refused type, after Chinese text on its own line, at the prop.
    const typed = `${original}\n\`\`\`rustify-component\nname: Counter\nchildren: |\n  wrong-type\nprops:\n  label: 中文标签\n  initial: wrong-type\n\`\`\`\n`;
    site.write("content/counter.md", typed);
    const typedState = await settled(failed.revision, false);
    await expect(marker).toBeVisible();
    await marker.click();
    await expect(panel).toBeVisible();
    await expectRepairs(page, typedState, "content/counter.md", [
        { rule: "invalid_props", line: lineOf(typed, "initial: wrong-type"), column: 3, suggestion: "Write `initial: 0` for declared type `i32` (default expression `2`). Reported props path: `initial`." },
    ]);
    await retained(page);
    await page.keyboard.press("Escape");
    await expect(marker).toBeFocused();

    site.write("content/counter.md", original);
    const fixed = await settled(typedState.revision);
    await cleared(page, fixed);
    // Gone with the errors, focus continues from the page; a remaining
    // warning keeps the marker, and focus on it.
    await expect(fixed.warnings ? marker : page.locator("main")).toBeFocused();
    await healthy();
});

test("gaps embedded diagnostics: without Rust the panel names the builtins and the empty demo registration and wraps the reference", async ({ page }) => {
    const plain = path.join(site.dir, "plain");
    mkdirSync(path.join(plain, "content"), { recursive: true });
    writeFileSync(path.join(plain, "rustify-docs.toml"), "schema_version = 1\n[site]\nname = 'Plain'\nurl = 'https://example.com/'\n");
    writeFileSync(path.join(plain, "content/index.md"), "---\ndescription: Home\n---\n# Home\n");
    const good = "---\ndescription: Counter\n---\n# Counter\n\nPlain text before the examples.\n";
    writeFileSync(path.join(plain, "content/counter.md"), good);
    server = await Dev.start(site, plain);
    await page.goto(new URL("counter/", server.url).href);
    const first = await settled();
    await expect(page.locator("h1")).toHaveText("Counter");

    const bad = `${good}\n\`\`\`rustify-component\nname: MissingWidget\n\`\`\`\n\n\`\`\`rustify-demo\nname: MissingDemo\n\`\`\`\n\n\`\`\`rustify-component\nname: Steps\n\`\`\`\n`;
    writeFileSync(path.join(plain, "content/counter.md"), bad);
    const failed = await settled(first.revision, false);
    const panel = page.locator(".rd-dev-panel");
    await expect(panel).toBeVisible();
    await expectRepairs(page, failed, "content/counter.md", [
        { rule: "unregistered-component", line: lineOf(bad, "name: MissingWidget", -1), column: 1, suggestion: "Use a component from `CardGrid`, `Steps`, `Tabs`; for an author component, enable `[rust]` and register its name in `site!`." },
        { rule: "unregistered-demo", line: lineOf(bad, "name: MissingDemo", -1), column: 1, suggestion: "Enable `[rust]` and register a demo in `site!` before using a `rustify-demo` block." },
        { rule: "embed-props", line: lineOf(bad, "name: Steps", -1), column: 1, suggestion: `Add or correct \`props\` using the declared fields \`items\` (\`Vec<StepSpec>\`; required); see ${builtinReference} for its object fields.` },
    ]);
    expect(await page.evaluate(() => document.activeElement === document.body), "reading focus is not taken").toBe(true);
    await expect(page.locator("article")).toContainText("Plain text before the examples.");
    await repairsFit(page, "gaps-builtins");
    await panel.getByRole("button", { name: "Hide problems" }).click();
    const marker = page.locator(".rd-dev-marker");
    await expect(marker).toBeFocused();

    writeFileSync(path.join(plain, "content/counter.md"), good);
    const fixed = await settled(failed.revision);
    await cleared(page, fixed);
    await expect(fixed.warnings ? marker : page.locator("main")).toBeFocused();
    expect(audit.csp).toEqual([]);
    expect(audit.failures).toEqual([]);
    expect(audit.console).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    expect(server.streamErrors).toEqual([]);
});

// Independent display expectations: never derive these from the prose reader.
async function exactProse(page: Page, state: State, rule: string, expected: { message: string; messageCodes: string[]; suggestion?: string; suggestionCodes?: string[] }) {
    const item = page.locator('.rd-dev-item').filter({ has: page.locator('.rd-dev-rule', { hasText: rule }) });
    await expect(item).toHaveCount(1);
    await expect.poll(() => item.locator('.rd-dev-message').textContent()).toBe(expected.message);
    expect(await item.locator('.rd-dev-message code').allTextContents()).toEqual(expected.messageCodes);
    if (expected.suggestion !== undefined) {
        expect(await item.locator('.rd-dev-suggestion').textContent()).toBe(expected.suggestion);
        expect(await item.locator('.rd-dev-suggestion code').allTextContents()).toEqual(expected.suggestionCodes);
    }
    await test.info().attach(`${rule}-prose.json`, { body: JSON.stringify({ diagnostic: state.diagnostics.find(d => d.rule === rule), expected, actual: await item.evaluate(node => ({ message: node.querySelector('.rd-dev-message')?.textContent, messageCodes: [...node.querySelectorAll('.rd-dev-message code')].map(code => code.textContent), suggestion: node.querySelector('.rd-dev-suggestion')?.textContent, suggestionCodes: [...node.querySelectorAll('.rd-dev-suggestion code')].map(code => code.textContent) })) }, null, 2), contentType: 'application/json' });
}

async function proseLayouts(page: Page, label: string) {
    for (const width of [1280, 375]) {
        for (const theme of ['light', 'dark']) {
            await page.setViewportSize({ width, height: 900 });
            await page.locator(`[data-rd-mode="${theme}"]`).click();
            await expect(page.locator('html')).toHaveAttribute('data-theme', theme);
            expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(0);
            const actual = await page.locator('.rd-dev-item').evaluateAll(items => items.map(item => ({ message: item.querySelector('.rd-dev-message')?.textContent, suggestion: item.querySelector('.rd-dev-suggestion')?.textContent, codes: [...item.querySelectorAll('code')].map(code => ({ text: code.textContent, font: getComputedStyle(code).fontFamily })), overflow: [...item.querySelectorAll<HTMLElement>('.rd-dev-message, .rd-dev-suggestion')].map(node => node.scrollWidth - node.clientWidth) })));
            expect(actual.every(item => item.overflow.every(extra => extra <= 0))).toBe(true);
            await test.info().attach(`${label}-${width}-${theme}.json`, { body: JSON.stringify({ width, theme, actual }, null, 2), contentType: 'application/json' });
            await test.info().attach(`${label}-${width}-${theme}.png`, { body: await page.screenshot(), contentType: 'image/png' });
        }
    }
}

test('gaps-2 code spans: plain author paths, prefixed anchors, YAML keys and empty language remain exact', async ({ page }) => {
    const plain = path.join(site.dir, 'plain');
    mkdirSync(path.join(plain, 'content'), { recursive: true });
    const config = "schema_version=1\n[site]\nname='Plain'\nurl='https://example.com/'\n";
    const good = '---\ndescription: Home\n---\n# Home\n';
    writeFileSync(path.join(plain, 'rustify-docs.toml'), config);
    writeFileSync(path.join(plain, 'content/index.md'), good);
    writeFileSync(path.join(plain, 'content/a`b`c.md'), '---\ndescription: Target\n---\n# Target\n');
    server = await Dev.start(site, plain);
    await page.goto(server.url);
    let state = await settled();
    writeFileSync(path.join(plain, 'content/index.md'), `${good}\n[Target](a%60b%60c.md#a%60b)\n`);
    state = await settled(state.revision, false);
    expect(state.diagnostics.find(d => d.rule === 'broken-anchor')?.message).toBe('`` content/a`b`c.md `` has no heading `` #a`b ``');
    await exactProse(page, state, 'broken-anchor', { message: 'content/a`b`c.md has no heading #a`b', messageCodes: ['content/a`b`c.md', '#a`b'] });
    await proseLayouts(page, 'gaps2-anchor');
    const hide = page.getByRole('button', { name: 'Hide problems' });
    const marker = page.locator('.rd-dev-marker');
    await hide.click();
    await expect(marker).toBeFocused();
    await page.keyboard.press('Enter');
    await expect(hide).toBeFocused();
    await page.keyboard.press('Escape');
    await expect(marker).toBeFocused();
    await marker.click();

    writeFileSync(path.join(plain, 'content/index.md'), `${good}\n\`\`\`rustify-component\nname: Steps\nun\`known: 1\n\`\`\`\n`);
    state = await settled(state.revision, false);
    await exactProse(page, state, 'embed-payload', { message: 'unknown field un`known; a demo takes only name, a component takes name, props and children', messageCodes: ['un`known', 'name', 'name', 'props', 'children'] });
    await proseLayouts(page, 'gaps2-yaml');

    writeFileSync(path.join(plain, 'rustify-docs.toml'), `${config}lang=''\n`);
    state = await settled(state.revision, false);
    await exactProse(page, state, 'config-lang', { message: ' is not a language tag', messageCodes: [''], suggestion: 'use a tag like en or zh-CN', suggestionCodes: ['en', 'zh-CN'] });
    await proseLayouts(page, 'gaps2-empty');
    writeFileSync(path.join(plain, 'content/index.md'), good);
    writeFileSync(path.join(plain, 'rustify-docs.toml'), config);
    state = await settled(state.revision);
    await cleared(page, state);
    expect(audit.csp).toEqual([]);
    expect(audit.failures).toEqual([]);
    expect(audit.console).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    expect(server.streamErrors).toEqual([]);
});

test('gaps-2 code spans: frontmatter language preserves author text resembling a YAML location', async ({ page }) => {
    const plain = path.join(site.dir, 'plain');
    mkdirSync(path.join(plain, 'content'), { recursive: true });
    const good = '---\ndescription: Home\n---\n# Home\n';
    writeFileSync(path.join(plain, 'rustify-docs.toml'), "schema_version=1\n[site]\nname='Plain'\nurl='https://example.com/'\n");
    writeFileSync(path.join(plain, 'content/index.md'), good);
    server = await Dev.start(site, plain);
    await page.goto(server.url);
    let state = await settled();
    const first = state;
    const lastGood = await (await page.request.get(server.url)).body();
    await page.evaluate(() => window.__dev_component_document = 'last good language page');
    for (const [label, value, encoded] of [
        ['ticks', 'a` at line `b', '`` a` at line `b ``'],
        ['plain', 'a at line b', '`a at line b`'],
    ]) {
        writeFileSync(path.join(plain, 'content/index.md'), `---\ndescription: Home\nlang: "${value}"\n---\n# Home\n`);
        state = await settled(state.revision, false);
        const diagnostic = state.diagnostics.find(d => d.rule === 'lang')!;
        expect(diagnostic).toMatchObject({ file: 'content/index.md', line: 3, column: 7, message: `lang: ${encoded} is not a language tag` });
        await exactProse(page, state, 'lang', { message: `lang: ${value} is not a language tag`, messageCodes: [value], suggestion: 'use a language tag such as en, zh-CN or fr', suggestionCodes: ['en', 'zh-CN', 'fr'] });
        expect(state.good).toBe(first.good);
        expect(await (await page.request.get(server.url)).body()).toEqual(lastGood);
        expect(await page.evaluate(() => window.__dev_component_document)).toBe('last good language page');
        await expect.poll(() => server!.log).toContain(diagnostic.message);
        await expect.poll(() => server!.log).toContain('content/index.md:3:7');
        await proseLayouts(page, `gaps2-frontmatter-${label}`);
        const hide = page.getByRole('button', { name: 'Hide problems' });
        const marker = page.locator('.rd-dev-marker');
        await hide.click();
        await expect(marker).toBeFocused();
        await page.keyboard.press('Enter');
        await expect(hide).toBeFocused();
        await page.keyboard.press('Escape');
        await expect(marker).toBeFocused();
        await marker.click();
    }
    writeFileSync(path.join(plain, 'content/index.md'), good);
    state = await settled(state.revision);
    await cleared(page, state);
    expect(audit.csp).toEqual([]);
    expect(audit.failures).toEqual([]);
    expect(audit.responses.filter(response => response.status >= 400)).toEqual([]);
    expect(audit.console).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    expect(server.streamErrors).toEqual([]);
});

test('gaps-2 code spans: defaults with literal backticks survive unknown props, type errors and external unmatched ticks', async ({ page }) => {
    test.setTimeout(900_000);
    site.edit('src/lib.rs', 'label: String = "Counter".to_string()', 'label: String = "a`b".to_string()');
    server = await Dev.start(site);
    await page.goto(new URL('counter/', server.url).href);
    site.release();
    await ready(page);
    let state = await settled();
    const original = site.read('content/counter.md');
    await captureTrees(page);
    site.write('content/counter.md', `${original}\n\`\`\`rustify-component\nname: Counter\nprops:\n  un\`known: 1\n\`\`\`\n`);
    state = await settled(state.revision, false);
    await exactProse(page, state, 'unknown-property', { message: 'component Counter has no property un`known', messageCodes: ['Counter', 'un`known'], suggestion: 'Use one of initial (i32; default expression 2), label (String; default expression "a`b".to_string()) for Counter; rename or remove un`known.', suggestionCodes: ['initial', 'i32', '2', 'label', 'String', '"a`b".to_string()', 'Counter', 'un`known'] });
    await retained(page);
    await proseLayouts(page, 'gaps2-default-unknown');
    site.write('content/counter.md', `${original}\n\`\`\`rustify-component\nname: Counter\nprops:\n  label: [wrong]\n\`\`\`\n`);
    state = await settled(state.revision, false);
    const typed = state.diagnostics.find(d => d.rule === 'invalid_props')!;
    expect(typed.suggestion).toBe('Write `label: "text"` for declared type `String` (default expression `` "a`b".to_string() ``). Reported props path: `label`.');
    await exactProse(page, state, 'invalid_props', { message: 'invalid type: sequence, expected a string', messageCodes: [], suggestion: 'Write label: "text" for declared type String (default expression "a`b".to_string()). Reported props path: label.', suggestionCodes: ['label: "text"', 'String', '"a`b".to_string()', 'label'] });
    await retained(page);
    await proseLayouts(page, 'gaps2-default-type');
    site.write('content/counter.md', `${original}\n\`\`\`rustify-component\nname: Counter\nprops:\n  initial: a\`b\n\`\`\`\n`);
    state = await settled(state.revision, false);
    const external = state.diagnostics.find(d => d.rule === 'invalid_props')!;
    expect(external.message).toContain('a`b');
    await exactProse(page, state, 'invalid_props', { message: 'invalid type: string "a`b", expected i32', messageCodes: [] });
    await proseLayouts(page, 'gaps2-external');
    await page.getByRole('button', { name: 'Hide problems' }).click();
    await expect(page.locator('.rd-dev-marker')).toBeFocused();
    site.write('content/counter.md', original);
    state = await settled(state.revision);
    await cleared(page, state);
    await healthy();
});

test('gaps-2 code spans: the real DOM reader preserves the handwritten whitespace and delimiter table', async ({ page }) => {
    const plain = path.join(site.dir, 'plain');
    mkdirSync(path.join(plain, 'content'), { recursive: true });
    writeFileSync(path.join(plain, 'rustify-docs.toml'), "schema_version=1\n[site]\nname='Plain'\nurl='https://example.com/'\n");
    writeFileSync(path.join(plain, 'content/index.md'), '---\ndescription: Home\n---\n# Home\n');
    server = await Dev.start(site, plain);
    const state = await settled();
    const cases: { encoded: string; text: string; codes: string[] }[] = [
        { encoded: '``  ``', text: '', codes: [''] },
        { encoded: '` `', text: ' ', codes: [' '] },
        { encoded: '`  `', text: '  ', codes: ['  '] },
        { encoded: '` a `', text: ' a ', codes: [' a '] },
        { encoded: '`a\nb`', text: 'a\nb', codes: ['a\nb'] },
        { encoded: '`a\r\nb`', text: 'a\r\nb', codes: ['a\r\nb'] },
        { encoded: '`a\\b`', text: 'a\\b', codes: ['a\\b'] },
        { encoded: '`Counter`', text: 'Counter', codes: ['Counter'] },
        { encoded: '`` a`b ``', text: 'a`b', codes: ['a`b'] },
        { encoded: '``` a``b ```', text: 'a``b', codes: ['a``b'] },
        { encoded: '`` `a` ``', text: '`a`', codes: ['`a`'] },
        { encoded: '`` ` ``', text: '`', codes: ['`'] },
        { encoded: '``` `` ```', text: '``', codes: ['``'] },
        { encoded: '``  `\r\n  ``', text: ' `\r\n ', codes: [' `\r\n '] },
        { encoded: '``    ``', text: '  ', codes: ['  '] },
        { encoded: '``a b``', text: 'a b', codes: ['a b'] },
        { encoded: '``\na\r\n``', text: '\na\r\n', codes: ['\na\r\n'] },
        { encoded: '``  `` then `` file/a`b `` and `en` / `zh-CN`', text: ' then file/a`b and en / zh-CN', codes: ['', 'file/a`b', 'en', 'zh-CN'] },
        { encoded: 'plain `unmatched and `` value ``', text: 'plain `unmatched and value', codes: ['value'] },
        { encoded: 'unknown field `a`b`', text: 'unknown field ab`', codes: ['a'] },
        { encoded: 'single a`b', text: 'single a`b', codes: [] },
        { encoded: '`<img src=x onerror=alert(1)>`', text: '<img src=x onerror=alert(1)>', codes: ['<img src=x onerror=alert(1)>'] },
    ];
    cases.forEach(({ encoded, text, codes }) => expect(diagnosticProse(encoded)).toEqual({ text, codes }));
    // This table isolates the DOM reader using a controlled SSE response.
    // The other gaps-2 tests exercise actual author input through the service.
    await page.route('**/__rustify-docs/events?**', route => route.fulfill({ contentType: 'text/event-stream', body: `event: state\ndata: ${JSON.stringify({ ...state, ok: false, errors: cases.length, warnings: 0, reload: false, diagnostics: cases.map((entry, i) => ({ severity: 'error', rule: `span-${i}`, file: 'table', line: i + 1, column: 1, message: entry.encoded, suggestion: entry.encoded })) })}\n\n` }));
    await page.goto(server.url);
    for (let i = 0; i < cases.length; i++) {
        const { text, codes } = cases[i];
        const item = page.locator('.rd-dev-item').nth(i);
        await expect.poll(() => item.locator('.rd-dev-message').textContent()).toBe(text);
        expect(await item.locator('.rd-dev-message code').allTextContents()).toEqual(codes);
        expect(await item.locator('.rd-dev-suggestion').textContent()).toBe(text);
        expect(await item.locator('.rd-dev-suggestion code').allTextContents()).toEqual(codes);
        expect(await item.locator('img').count()).toBe(0);
    }
    await test.info().attach('verbatim-table.json', { body: JSON.stringify(cases, null, 2), contentType: 'application/json' });
    await proseLayouts(page, 'gaps2-table');
    expect(audit.csp).toEqual([]);
    expect(audit.console).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
});
