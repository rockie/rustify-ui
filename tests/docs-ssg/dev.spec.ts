import AxeBuilder from "@axe-core/playwright";
import { expect, Page, test } from "@playwright/test";
import { ChildProcess, execFileSync, spawn } from "node:child_process";
import { cpSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import path from "node:path";

/// `rustify-docs dev` with a browser open on it: a save reloads the pages it
/// changed in place and no others; a failed build leaves the last good page
/// readable under a corner panel that lists the problems, hides to a marker
/// and clears itself on the fix; nothing built yet is a page that says so; a
/// restarted dev brings open pages up to date; a panic in a build is one
/// more problem. Each test runs its own `dev` on a copy of the content
/// fixture, with the binary `--profile docs-cli` builds (as installed).

const repo = process.cwd();
const fixture = path.join(repo, "tests/docs-ssg/fixtures/content");
const bin = path.resolve(repo, process.env.RUSTIFY_DOCS_BIN || "target/docs-cli/rustify-docs");

test.beforeAll(() => {
    if (!process.env.RUSTIFY_DOCS_BIN) {
        test.setTimeout(600_000);
        execFileSync("mbx", ["build", "--locked", "--profile", "docs-cli", "-p", "rustify-docs-cli"], {
            cwd: repo,
            stdio: "inherit",
            env: { ...process.env, CARGO_TARGET_DIR: path.join(repo, "target") },
        });
    }
});

/// A copy of the content fixture, removed when the test ends.
class Site {
    readonly dir = mkdtempSync(path.join(tmpdir(), "rustify-docs-dev-"));
    readonly root = path.join(this.dir, "site");

    constructor() {
        cpSync(fixture, this.root, { recursive: true });
    }

    read(file: string): string {
        return readFileSync(path.join(this.root, file), "utf8");
    }

    write(file: string, text: string): void {
        writeFileSync(path.join(this.root, file), text);
    }

    /// Replaces `from` with `to`, asserting it was there.
    edit(file: string, from: string, to: string): void {
        const text = this.read(file);
        expect(text, `${from} in ${file}`).toContain(from);
        this.write(file, text.replace(from, to));
    }

    remove(): void {
        rmSync(this.dir, { recursive: true, force: true });
    }
}

/// A running `dev`.
class Dev {
    log = "";

    private constructor(
        readonly child: ChildProcess,
        readonly url: string,
    ) {}

    static start(site: Site, args: string[] = ["--port", "0"], env: Record<string, string> = {}): Promise<Dev> {
        const child = spawn(bin, ["dev", ...args], { cwd: site.root, env: { ...process.env, ...env } });
        let log = "";
        child.stderr!.on("data", (chunk) => (log += chunk));
        return new Promise((resolve, reject) => {
            let out = "";
            child.stdout!.on("data", (chunk) => {
                out += chunk;
                const match = /^dev: (\S+)\n/m.exec(out);
                if (match) {
                    const dev = new Dev(child, match[1]);
                    child.stderr!.on("data", (more) => (dev.log += more));
                    dev.log = log;
                    resolve(dev);
                }
            });
            child.on("exit", (code) => reject(new Error(`dev exited ${code}: ${out}${log}`)));
        });
    }

    async stop(): Promise<void> {
        if (this.child.exitCode !== null || this.child.signalCode !== null) return;
        const exited = new Promise((resolve) => this.child.on("exit", resolve));
        this.child.kill();
        await exited;
    }
}

async function freePort(): Promise<number> {
    return new Promise((resolve) => {
        const server = createServer();
        server.listen(0, "127.0.0.1", () => {
            const port = (server.address() as { port: number }).port;
            server.close(() => resolve(port));
        });
    });
}

let site: Site;
const running: Dev[] = [];
const initialStates: unknown[] = [];
const expectedStreamClosures: string[] = [];

/// The address precedes the first build; these tests need its terminal result.
async function firstResult(url: string): Promise<void> {
    const events = new URL("/__rustify-docs/events", url);
    events.searchParams.set("path", new URL(url).pathname);
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), 10_000);
    try {
        const response = await fetch(events, { signal: controller.signal });
        if (response.status !== 200 || !response.body) throw new Error(`dev SSE returned ${response.status}`);
        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        let pending = "";
        while (true) {
            const { value, done } = await reader.read();
            if (done) throw new Error("dev SSE closed before the first build result");
            pending += decoder.decode(value, { stream: true });
            let end: number;
            while ((end = pending.indexOf("\n\n")) !== -1) {
                const message = pending.slice(0, end);
                pending = pending.slice(end + 2);
                if (!message.startsWith("event: state\ndata: ")) continue;
                const state = JSON.parse(message.slice("event: state\ndata: ".length));
                initialStates.push({ url, state });
                if (state.revision > 0 && state.building === false) return;
            }
        }
    } finally {
        clearTimeout(timeout);
        controller.abort();
    }
}

function watchRequestFailures(page: Page, errors: string[]): void {
    page.on("requestfailed", (request) => {
        const error = request.failure()?.errorText;
        const message = `${request.url()}: ${error}`;
        if (new URL(request.url()).pathname === "/__rustify-docs/events" && error === "net::ERR_ABORTED") expectedStreamClosures.push(message);
        else errors.push(message);
    });
}

async function dev(args?: string[], env?: Record<string, string>): Promise<Dev> {
    const started = await Dev.start(site, args, env);
    running.push(started);
    await firstResult(started.url);
    return started;
}

test.beforeEach(() => {
    site = new Site();
    initialStates.length = 0;
    expectedStreamClosures.length = 0;
});

test.afterEach(async () => {
    await test.info().attach("dev-initial-states.json", { body: JSON.stringify(initialStates, null, 2), contentType: "application/json" });
    await test.info().attach("dev-expected-stream-closures.json", { body: JSON.stringify(expectedStreamClosures, null, 2), contentType: "application/json" });
    while (running.length) await running.pop()!.stop();
    site.remove();
});

/// The build result the page's panel last took in.
async function seen(page: Page): Promise<number> {
    return Number(await page.locator(".rd-dev").getAttribute("data-revision"));
}

async function nextResult(page: Page, after: number): Promise<void> {
    await expect.poll(() => seen(page)).toBeGreaterThan(after);
}

/// Marks the document, so a reload shows as the mark's absence.
async function mark(page: Page): Promise<void> {
    await page.evaluate(() => ((window as unknown as { __kept: boolean }).__kept = true));
}

async function kept(page: Page): Promise<boolean> {
    return page.evaluate(() => (window as unknown as { __kept?: boolean }).__kept === true);
}

const BROKEN = "[Broken](missing.md)";

test("semantic diagnostics locate the source, preserve the home, and clear on repair", async ({ page }) => {
    const original = site.read("content/index.md");
    const server = await dev();
    await page.goto(server.url);
    await mark(page);
    for (const [source, rule, location] of [
        ["---\ntitle: Home\nlayout: home\nlang: english\n---\n# Home\n", "lang", "4:7"],
        ["---\ntitle: Home\nlayout: home\n---\nIntro\n\n# Home\n", "multiple-h1", "7:1"],
        ["---\ntitle: Home\nlayout: home\n---\n<div><h1>Home</h1></div>\n", "multiple-h1", "5:6"],
        ["---\ntitle: Home\nlayout: home\n---\n<H1>Home</H1>\n", "multiple-h1", "5:1"],
    ]) {
        site.write("content/index.md", source);
        const panel = page.getByRole("dialog");
        await expect(panel).toBeVisible();
        await expect(panel).toContainText(rule);
        await expect(panel.locator(".rd-dev-location")).toHaveText(location);
        await expect(panel.locator(".rd-dev-suggestion")).not.toBeEmpty();
        await expect(page.locator("h1")).toHaveText("Fixture Docs");
        expect(await kept(page)).toBe(true);
        site.write("content/index.md", original);
        await expect(panel).toBeHidden();
    }
});

test("a new site is readable at once: the home page and its two pages", async ({ page }) => {
    const target = path.join(site.dir, "new-site");
    execFileSync(bin, ["new", target]);
    const child = spawn(bin, ["dev", "--port", "0"], { cwd: target });
    const url = await new Promise<string>((resolve, reject) => {
        let out = "";
        child.stdout!.on("data", (chunk) => {
            out += chunk;
            const match = /^dev: (\S+)\n/m.exec(out);
            if (match) resolve(match[1]);
        });
        child.on("exit", (code) => reject(new Error(`dev exited ${code}`)));
    });
    try {
        await firstResult(url);
        await page.goto(url);
        await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
        await page.locator(".rd-action-primary").click();
        await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
        const first = page.url();
        const sidebar = page.locator(".rd-sidebar-wide a:not([aria-current])").first();
        await sidebar.click();
        await expect(page).not.toHaveURL(first);
        await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
        // The template has no `site.url` yet: a warning, which waits behind
        // a marker rather than opening the panel.
        const marker = page.getByRole("button", { name: "1 warning" });
        await expect(marker).toBeVisible();
        await expect(page.getByRole("dialog")).toBeHidden();
        await marker.click();
        await expect(page.getByRole("dialog")).toContainText("rustify-docs.toml");
    } finally {
        child.kill();
    }
});

test("a save reloads the page it changed in place and leaves the others", async ({ context }) => {
    const server = await dev();
    const setup = await context.newPage();
    await setup.goto(`${server.url}guide/setup/`);
    await setup.getByRole("tab", { name: "Config" }).first().click();
    await setup.evaluate(() => window.scrollTo(0, 600));
    await expect.poll(() => setup.evaluate(() => window.scrollY)).toBe(600);
    await mark(setup);
    const api = await context.newPage();
    await api.goto(`${server.url}reference/api/`);
    await expect(api.locator(".rd-dev")).toHaveAttribute("data-revision", /\d+/);
    await mark(api);
    const before = await seen(api);

    site.edit("content/guide/setup.md", "Once installed, the same choice applies here:", "Once installed, the same choice still applies here:");
    await expect(setup.getByText("the same choice still applies here")).toBeVisible();
    expect(await kept(setup), "the changed page reloaded").toBe(false);
    await expect.poll(() => setup.evaluate(() => window.scrollY)).toBe(600);
    await expect(setup.getByRole("tab", { name: "Config" }).first()).toHaveAttribute("aria-selected", "true");
    await nextResult(api, before);
    expect(await kept(api), "an unchanged page stayed").toBe(true);

    // The configuration changes every page.
    site.edit("rustify-docs.toml", "Fixture Docs", "Fixture Handbook");
    await expect(api.locator(".rd-brand")).toContainText("Fixture Handbook");
    expect(await kept(api)).toBe(false);
});

test("a real symlink target save refreshes its image and survives deletion and recovery", async ({ context }) => {
    mkdirSync(path.join(site.root, "brand"));
    mkdirSync(path.join(site.root, "images"));
    const svg = (fill: string) => `<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40"><rect width="40" height="40" fill="${fill}"/></svg>`;
    site.write("brand/real.svg", svg("red"));
    symlinkSync("../brand/real.svg", path.join(site.root, "images/mark.svg"));
    site.edit("content/guide/index.md", "Read the setup page first.", "Read the setup page first. ![Mark](../../images/mark.svg)");
    const server = await dev();
    const page = await context.newPage();
    const errors: string[] = [];
    page.on("pageerror", error => errors.push(error.message));
    await page.goto(`${server.url}guide/`);
    const image = page.getByRole("img", { name: "Mark", exact: true });
    const original = await image.getAttribute("src");
    const api = await context.newPage();
    await api.goto(`${server.url}reference/api/`);
    await expect(api.locator(".rd-dev")).toHaveAttribute("data-revision", /\d+/);
    await mark(api);
    const before = await seen(api);
    site.write("brand/replacement.svg", svg("blue"));
    renameSync(path.join(site.root, "brand/replacement.svg"), path.join(site.root, "brand/real.svg"));
    await expect(image).not.toHaveAttribute("src", original!);
    const blue = await image.getAttribute("src");
    expect(await (await page.request.get(new URL(blue!, page.url()).href)).text()).toContain('fill="blue"');
    await nextResult(api, before);
    expect(await kept(api), "the page without the image stays mounted").toBe(true);
    await mark(page);
    rmSync(path.join(site.root, "brand/real.svg"));
    await expect(page.getByRole("dialog")).toContainText("missing-image");
    expect(await kept(page), "deletion keeps the last good page").toBe(true);
    site.write("brand/real.svg", svg("green"));
    await expect(image).not.toHaveAttribute("src", blue!);
    await expect(page.getByRole("dialog")).toBeHidden();
    const green = await image.getAttribute("src");
    expect(await (await page.request.get(new URL(green!, page.url()).href)).text()).toContain('fill="green"');
    expect(await kept(api), "recovery still leaves the unrelated page mounted").toBe(true);
    expect(errors).toEqual([]);
});

for (const [alias, url] of [["content/manual.pdf", "../manual.pdf"], ["static/manual.pdf", "/manual.pdf"]]) {
    test(`a snapshot attachment in ${alias} recovers when only its target is restored`, async ({ context }) => {
        mkdirSync(path.join(site.root, "brand"));
        site.write("brand/real.pdf", "original attachment");
        symlinkSync("../brand/real.pdf", path.join(site.root, alias));
        site.edit("content/guide/index.md", "Then write a page.", `Then write a page.\n\n[Download](${url})`);
        const server = await dev();
        const page = await context.newPage();
        const errors: string[] = [];
        page.on("pageerror", (error) => errors.push(error.message));
        page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
        page.on("response", (response) => { if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`); });
        await page.goto(`${server.url}guide/`);
        const link = page.getByRole("link", { name: "Download", exact: true });
        const original = await link.getAttribute("href");
        const api = await context.newPage();
        await api.goto(`${server.url}reference/api/`);
        await expect(api.locator(".rd-dev")).toHaveAttribute("data-revision", /\d+/);
        await mark(api);
        await mark(page);
        rmSync(path.join(site.root, "brand/real.pdf"));
        await expect(page.getByRole("dialog")).toContainText("broken-link");
        expect(await kept(page), "deletion leaves the last good page readable").toBe(true);
        const failed = await seen(page);
        site.write("brand/real.pdf", "restored attachment");
        await expect(link).not.toHaveAttribute("href", original!);
        await expect(page.getByRole("dialog")).toBeHidden();
        expect(await (await page.request.get(new URL((await link.getAttribute("href"))!, page.url()).href)).text()).toBe("restored attachment");
        await nextResult(api, failed);
        expect(await kept(api), "the unrelated page remains mounted").toBe(true);
        expect(errors).toEqual([]);
    });
}

test("arbitrary CSS resources keep numeric entity delimiters and compiler URL decoding", async ({ page }) => {
    mkdirSync(path.join(site.root, "static/assets"));
    mkdirSync(path.join(site.root, "content/="));
    const svg = '<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40"><rect width="40" height="40" fill="red"/></svg>';
    site.write("static/assets/bg one.svg", svg);
    site.write("static/assets/bg_one.svg", svg.replace("red", "blue"));
    site.write("content/=/bg.svg", svg);
    const classes = [
        "[background-image:URL('/assets/bg_one.svg')]",
        "[background-image:url('/assets/bg_one.svg')]",
        "[background-image:URL('/assets/bg\\_one.svg')]",
        "[background-image:url('/assets/bg\\_one.svg')]",
        "[background-image:URL('/assets/bg%20one.svg')]",
        "[background-image:url('/assets/bg%20one.svg')]",
        "[background-image:url&#40=/bg.svg)]",
        "[background-image:url&#40;=/bg.svg)]",
        "[background-image:url(&#34=/bg.svg&#34)]",
        "[background-image:url(&#34;=/bg.svg&#34;)]",
    ];
    site.write("content/index.md", `---\ndescription: CSS resources\n---\n# Resources\n\n${classes.map((value, i) => `<div id="resource-${i}" class="w-10 h-10 ${value}">Background ${i}</div>`).join("\n")}\n`);
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(error.message));
    page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
    page.on("response", (response) => { if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`); });
    const server = await dev();
    await page.goto(server.url);
    await expect(page.getByRole("heading", { name: "Resources", exact: true })).toBeVisible();
    for (let i = 0; i < classes.length; i++) {
        const background = await page.locator(`#resource-${i}`).evaluate((element) => getComputedStyle(element).backgroundImage);
        expect(background, classes[i]).not.toBe("none");
        const url = background.match(/^url\("(.*)"\)$/)![1];
        const response = await page.request.get(url);
        expect(response.ok(), classes[i]).toBe(true);
        expect(await response.text()).toContain(`fill="${[1, 2, 3].includes(i) ? "blue" : "red"}"`);
    }
    expect(errors).toEqual([]);
    await test.info().attach("resources.png", { body: await page.screenshot(), contentType: "image/png" });
});

test("template contents are rejected as anchors while their holder and ordinary IDs navigate", async ({ page }) => {
    site.write("rustify-docs.toml", `${site.read("rustify-docs.toml")}\n[[nav.groups]]\nlabel = "Anchors"\nitems = ["index.md", "other.md"]\n`);
    const body = '---\ndescription: Template anchors\n---\n# Templates\n\n<template id="holder"><span id="ghost">Hidden</span><template id="nested"><b id="deep">Hidden</b></template></template>\n\n<p id="visible">Visible</p>\n\n[Holder](#holder) [Visible](#visible) [Other holder](other.md#holder)\n';
    site.write("content/index.md", `${body}\n[Ghost](#ghost) [Deep](other.md#deep)\n`);
    site.write("content/other.md", body);
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(error.message));
    const server = await dev();
    await page.goto(server.url);
    await expect(page.getByRole("region", { name: "2 errors · nothing has built yet" })).toContainText("broken-anchor");
    site.write("content/index.md", body);
    await expect(page.getByRole("heading", { name: "Templates", exact: true })).toBeVisible();
    expect(await page.evaluate(() => ({
        holder: document.getElementById("holder")?.tagName,
        ghost: document.getElementById("ghost"),
        nested: document.getElementById("nested"),
        deep: document.getElementById("deep"),
        stored: document.querySelector<HTMLTemplateElement>("#holder")!.content.getElementById("ghost")?.tagName,
    }))).toEqual({ holder: "TEMPLATE", ghost: null, nested: null, deep: null, stored: "SPAN" });
    await page.getByRole("link", { name: "Visible", exact: true }).click();
    expect(new URL(page.url()).hash).toBe("#visible");
    await page.getByRole("link", { name: "Holder", exact: true }).click();
    expect(new URL(page.url()).hash).toBe("#holder");
    await page.getByRole("link", { name: "Other holder", exact: true }).click();
    await expect(page).toHaveURL(/\/other\/#holder$/);
    expect(await page.locator("#holder").count()).toBe(1);
    expect(errors).toEqual([]);
});

for (const base of ["/", "/docs/"]) {
    test(`body anchors at ${base} navigate on the same and other pages and recover from ghost links`, async ({ page }) => {
        site.edit("rustify-docs.toml", 'url = "https://example.com/docs/"', `url = "https://example.com${base}"`);
        site.edit("rustify-docs.toml", 'base = "/docs/"', `base = "${base}"`);
        site.write("rustify-docs.toml", `${site.read("rustify-docs.toml")}\n[[nav.groups]]\nlabel = "Anchors"\nitems = ["index.md", "other.md"]\n`);
        const body = '---\ndescription: Body anchor\n---\n# Body anchor\n\n<body id="target">\n\nText\n\n</body>\n\n<template id="holder"><body id="ghost"><span id="hidden">Hidden</span></template>\n\n[Target](#target) [Other target](other.md#target)\n';
        site.write("content/index.md", body);
        site.write("content/other.md", body);
        const errors: string[] = [];
        page.on("pageerror", (error) => errors.push(error.message));
        page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
        watchRequestFailures(page, errors);
        const server = await dev();
        await page.goto(server.url);
        await expect(page.getByRole("heading", { name: "Body anchor", exact: true })).toBeVisible();
        expect(await page.evaluate(() => ({
            target: document.getElementById("target") === document.body,
            ghost: document.getElementById("ghost"),
            hidden: document.getElementById("hidden"),
            stored: document.querySelector<HTMLTemplateElement>("#holder")!.content.getElementById("hidden")?.tagName,
        }))).toEqual({ target: true, ghost: null, hidden: null, stored: "SPAN" });
        await page.locator("article").getByRole("link", { name: "Target", exact: true }).click();
        expect(new URL(page.url()).hash).toBe("#target");
        await page.locator("article").getByRole("link", { name: "Other target", exact: true }).click();
        await expect(page).toHaveURL(new RegExp(`${base}other/#target$`));
        expect(await page.locator("body").getAttribute("id")).toBe("target");
        site.write("content/index.md", `${body}\n[Ghost](#ghost) [Other ghost](other.md#ghost)\n`);
        await expect(page.getByRole("dialog", { name: "2 errors · showing the last good version" })).toContainText("broken-anchor");
        site.write("content/index.md", body);
        await expect(page.getByRole("dialog", { name: /errors ·/ })).toHaveCount(0);
        expect(errors).toEqual([]);
        await test.info().attach("body-anchors.png", { body: await page.screenshot(), contentType: "image/png" });
    });
}

for (const [name, markup, ids] of [
    ["self-closing SVG template", '<svg><template id="svg-holder"/></svg>', ["svg-holder"]],
    ["paired SVG template", '<svg><template id="svg-holder"><circle id="circle" r="10"/></template></svg>', ["svg-holder", "circle"]],
    ["SVG inside an HTML template", '<template id="holder"><svg><template id="ghost"/></svg></template>', ["holder"]],
] as const) {
    test(`${name} preserves document anchors and cross-page navigation`, async ({ page }) => {
        site.write("rustify-docs.toml", `${site.read("rustify-docs.toml")}\n[[nav.groups]]\nlabel = "Anchors"\nitems = ["index.md", "other.md"]\n`);
        const body = `---\ndescription: Foreign anchors\n---\n# Foreign anchors\n\n${markup}\n\n## Visible\n\n[Visible](#visible) [Other visible](other.md#visible)\n\n${ids.map((id) => `[${id}](#${id}) [Other ${id}](other.md#${id})`).join("\n")}\n`;
        site.write("content/index.md", body);
        site.write("content/other.md", body);
        const errors: string[] = [];
        page.on("pageerror", (error) => errors.push(error.message));
        page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
        watchRequestFailures(page, errors);
        const server = await dev();
        await page.goto(server.url);
        await expect(page.getByRole("heading", { name: "Visible", exact: true })).toBeVisible();
        expect(await page.locator("article [id]").evaluateAll((nodes) => nodes.map((node) => node.id))).toEqual(["foreign-anchors", ...ids, "visible"]);
        if (ids.some((id) => id === "svg-holder")) {
            expect(await page.locator("#svg-holder").evaluate((node) => ({ namespace: node.namespaceURI, content: "content" in node }))).toEqual({ namespace: "http://www.w3.org/2000/svg", content: false });
        }
        await page.locator("article").getByRole("link", { name: "Visible", exact: true }).click();
        expect(new URL(page.url()).hash).toBe("#visible");
        await page.locator("article").getByRole("link", { name: `Other ${ids[0]}`, exact: true }).click();
        await expect(page).toHaveURL(new RegExp(`/other/#${ids[0]}$`));
        expect(await page.locator(`[id="${ids[0]}"]`).count()).toBe(1);
        expect(errors).toEqual([]);
        await test.info().attach("foreign-anchors.png", { body: await page.screenshot(), contentType: "image/png" });
    });
}

test("HTML templates at foreign integration points recover from ghost links", async ({ page }) => {
    site.write("rustify-docs.toml", `${site.read("rustify-docs.toml")}\n[[nav.groups]]\nlabel = "Anchors"\nitems = ["index.md", "other.md"]\n`);
    const contexts = [
        ["<svg><foreignObject>", "</foreignObject></svg>"],
        ["<svg><desc>", "</desc></svg>"],
        ["<svg><title>", "</title></svg>"],
        ["<math><mtext>", "</mtext></math>"],
        ['<math><annotation-xml encoding="text/html">', "</annotation-xml></math>"],
    ];
    const body = `---\ndescription: Integration anchors\n---\n# Integration anchors\n\n${contexts.map(([open, close], i) => `${open}<template id="holder-${i}"><svg><template><circle id="ghost-${i}"/></template></svg></template>${close}`).join("\n\n")}\n\n## Visible\n\n[Visible](#visible) [Other holder](other.md#holder-0)\n`;
    const invalid = contexts.map((_, i) => `[Ghost ${i}](#ghost-${i}) [Other ghost ${i}](other.md#ghost-${i})`).join("\n");
    site.write("content/index.md", `${body}\n${invalid}\n`);
    site.write("content/other.md", body);
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(error.message));
    const server = await dev();
    await page.goto(server.url);
    await expect(page.getByRole("region", { name: "10 errors · nothing has built yet" })).toContainText("broken-anchor");
    site.write("content/index.md", body);
    await expect(page.getByRole("heading", { name: "Visible", exact: true })).toBeVisible();
    for (let i = 0; i < contexts.length; i++) {
        expect(await page.locator(`#holder-${i}`).evaluate((node) => ({ namespace: node.namespaceURI, stored: (node as HTMLTemplateElement).content.querySelector("circle")?.id }))).toEqual({ namespace: "http://www.w3.org/1999/xhtml", stored: `ghost-${i}` });
        expect(await page.locator(`#ghost-${i}`).count()).toBe(0);
    }
    await page.locator("article").getByRole("link", { name: "Visible", exact: true }).click();
    expect(new URL(page.url()).hash).toBe("#visible");
    await page.locator("article").getByRole("link", { name: "Other holder", exact: true }).click();
    await expect(page).toHaveURL(/\/other\/#holder-0$/);
    expect(errors).toEqual([]);
});

test("a failed build keeps the page readable under a panel that hides to a marker and clears on the fix", async ({ page }) => {
    site.write("content/guide/setup.md", `${site.read("content/guide/setup.md")}\n\n## Content\n\nThe author's heading keeps its own anchor.\n`);
    const server = await dev();
    await page.goto(`${server.url}guide/setup/`);
    await expect(page.locator("h2#content")).toHaveText("Content");
    await expect(page.locator("main")).toHaveAttribute("id", "content-1");
    await mark(page);
    const before = await seen(page);
    site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);

    const panel = page.getByRole("dialog", { name: "1 error · showing the last good version" });
    await expect(panel).toBeVisible();
    await expect(panel).toContainText("content/guide/setup.md");
    await expect(panel.locator(".rd-dev-location")).toHaveText(/^\d+:\d+$/);
    await expect(panel).toContainText("broken-link");
    expect(await kept(page), "the last good page stays").toBe(true);
    await expect(page.locator("article").getByText("Broken", { exact: true })).toHaveCount(0);
    expect(await page.evaluate(() => document.activeElement === document.body), "focus stays where it was").toBe(true);
    await expect(page.getByRole("status").filter({ hasText: "1 error" })).toHaveCount(1);

    await panel.getByRole("button", { name: "Hide problems" }).click();
    await expect(panel).toBeHidden();
    const marker = page.getByRole("button", { name: "1 error · showing the last good version" });
    await expect(marker).toBeVisible();
    await expect(marker).toBeFocused();
    await expect(marker).toHaveAttribute("aria-expanded", "false");

    // The same mistake saved again: a new result, the marker stays.
    const failed = await seen(page);
    site.write("content/guide/setup.md", `${site.read("content/guide/setup.md")}\n`);
    await nextResult(page, failed);
    await expect(marker).toBeVisible();
    await expect(panel).toBeHidden();

    await page.keyboard.press("Enter");
    await expect(panel).toBeVisible();
    await expect(panel.getByRole("button", { name: "Hide problems" })).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(marker).toBeFocused();

    site.edit("content/guide/setup.md", `${BROKEN} `, "");
    await expect(marker).toBeHidden();
    await expect(panel).toBeHidden();
    // Focus was on the marker: it continues from the page's content.
    await expect(page.locator("main")).toBeFocused();
    await expect(page.getByRole("status").filter({ hasText: "Problems fixed" })).toHaveCount(1);
    expect(await kept(page), "the fixed page was the last good one").toBe(true);
    expect(before).toBeGreaterThan(0);
});

test("while nothing has built, every page says so, and becomes the site once it builds", async ({ page }) => {
    site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);
    const server = await dev();
    const response = await page.goto(`${server.url}guide/setup/`);
    expect(response?.status()).toBe(500);
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("The site does not build yet");
    // The problems are the page: listed in its column, with nothing to hide them.
    const list = page.getByRole("region", { name: "1 error · nothing has built yet" });
    await expect(list).toBeVisible();
    await expect(page.locator("main .rd-dev")).toBeVisible();
    await expect(list.getByRole("button")).toHaveCount(0);
    await expect(list.locator(".rd-dev-code")).toHaveText("content/guide/missing.md");
    await list.locator(".rd-dev-list").focus();
    await page.keyboard.press("Escape");
    await expect(list).toBeVisible();
    site.edit("content/guide/setup.md", `${BROKEN} `, "");
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("Setup");
    await expect(page.locator(".rd-dev")).toBeHidden();
});

test("a restarted dev brings an open page up to date", async ({ page }) => {
    const port = String(await freePort());
    const first = await dev(["--port", port]);
    await page.goto(`${first.url}guide/setup/`);
    await expect(page.locator(".rd-dev")).toHaveAttribute("data-revision", /\d+/);
    await first.stop();
    site.edit("content/guide/setup.md", "Once installed, the same choice applies here:", "After a restart, the same choice applies here:");
    await dev(["--port", port]);
    await expect(page.getByText("After a restart")).toBeVisible({ timeout: 15_000 });
});

test("a panic in a build is one more problem, and the next build is served", async ({ page }) => {
    const server = await dev(["--port", "0"], { RUSTIFY_DOCS_FAULT: "panic:PANIC-HERE" });
    await page.goto(`${server.url}guide/setup/`);
    await mark(page);
    site.edit("content/guide/setup.md", "Once installed,", "PANIC-HERE Once installed,");
    const panel = page.getByRole("dialog", { name: "1 error · showing the last good version" });
    await expect(panel).toContainText("internal");
    await expect(panel).toContainText("stopped inside rustify-docs");
    await expect(panel.locator(".rd-dev-suggestion")).toContainText("save again");
    expect(await kept(page)).toBe(true);
    site.edit("content/guide/setup.md", "PANIC-HERE Once installed,", "Recovered. Once installed,");
    await expect(page.getByText("Recovered. Once installed")).toBeVisible();
    await expect(page.locator(".rd-dev")).toBeHidden();
    expect(server.log).toContain("stopped inside rustify-docs");
});

test("the panel speaks the page's language", async ({ page }) => {
    const server = await dev();
    await page.goto(`${server.url}reference/api/`);
    site.edit("content/reference/api.md", "返回[指南](../guide/index.md)。", `返回[指南](../guide/index.md)。${BROKEN}`);
    const panel = page.getByRole("dialog", { name: "1 个错误 · 显示上一个可用版本" });
    await expect(panel).toBeVisible();
    await expect(page.locator(".rd-dev")).toHaveAttribute("lang", "zh-CN");
    await expect(panel.getByRole("button", { name: "收起问题" })).toBeVisible();
});

test("on a phone the panel is a sheet along the bottom and nothing scrolls sideways", async ({ browser }) => {
    const server = await dev();
    for (const width of [375, 320]) {
        const context = await browser.newContext({ viewport: { width, height: 640 }, reducedMotion: "reduce" });
        const page = await context.newPage();
        await page.goto(`${server.url}guide/setup/`);
        if (width === 375) {
            site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);
        }
        const panel = page.getByRole("dialog");
        await expect(panel).toBeVisible();
        const box = (await panel.boundingBox())!;
        expect(box.x).toBe(0);
        expect(box.width).toBe(width);
        expect(box.y + box.height).toBeCloseTo(640, 0);
        expect(box.height).toBeLessThanOrEqual(320);
        expect(await page.evaluate(() => getComputedStyle(document.querySelector(".rd-dev-panel")!).animationName)).toBe("none");
        expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
        await context.close();
    }
});

test("axe finds no serious or critical problem with the panel open or hidden, in both modes", async ({ browser }) => {
    const server = await dev();
    site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);
    const found: string[] = [];
    for (const scheme of ["light", "dark"] as const) {
        const context = await browser.newContext({ colorScheme: scheme });
        const page = await context.newPage();
        await page.goto(`${server.url}guide/setup/`);
        const panel = page.getByRole("dialog");
        await expect(panel).toBeVisible();
        for (const state of ["open", "hidden"]) {
            if (state === "hidden") await panel.getByRole("button").click();
            // Colours are measured once the panel's entrance has finished.
            await page.waitForFunction(() => document.getAnimations().every((animation) => animation.playState === "finished"));
            const { violations } = await new AxeBuilder({ page })
                .include(".rd-dev")
                .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
                .analyze();
            for (const violation of violations) {
                for (const node of violation.nodes) {
                    const detail = (node.any[0]?.message || node.failureSummary || "").replace(/\s+/g, " ");
                    found.push(`${scheme} ${state} ${violation.impact} ${violation.id}: ${node.target.join(" ")}: ${detail}`);
                }
            }
        }
        await context.close();
    }
    await test.info().attach("axe-violations.txt", { body: found.join("\n") || "none", contentType: "text/plain" });
    expect(found.filter((line) => / (serious|critical) /.test(line))).toEqual([]);
});
