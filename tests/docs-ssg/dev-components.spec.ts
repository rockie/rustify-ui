import { expect, Page, test } from "@playwright/test";
import { ChildProcess, execFileSync, spawn } from "node:child_process";
import { copyFileSync, cpSync, existsSync, mkdtempSync, readFileSync, readdirSync, realpathSync, rmSync, writeFileSync } from "node:fs";
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
type State = { session: string; revision: number; sequence: number; building: boolean; phase: string; compile: Compile | null; good: number | null; ok: boolean; errors: number; warnings: number; diagnostics: { severity: string; rule: string; file: string | null; line: number | null; column: number | null; message: string }[] };
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
    readonly lock: Buffer;

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

    static async start(site: Site): Promise<Dev> {
        const child = spawn(bin, ["dev", "--port", "0"], {
            cwd: site.root, detached: true, env: { ...process.env, RUSTIFY_DOCS_CARGO: site.compiler },
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

test.beforeEach(async ({ page }) => {
    site = new Site();
    server = undefined;
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

async function ready(page: Page, demoValue = "7") {
    await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(4, { timeout: buildTimeout });
    await expect(values(page)).toHaveText(["2", "2", "2", demoValue]);
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
