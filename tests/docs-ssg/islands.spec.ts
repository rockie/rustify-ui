import { expect, Locator, Page, Route, test as playwright } from "@playwright/test";
import { readFileSync } from "node:fs";
import { attachMotion, expectNormal, expectStill, installMotionProbe, motionTargets, sampleMotion } from "./motion";
import { atRest, attachScans, buildId, scan, Scan, serious } from "./a11y";

type RuntimeStats = { tasks: number; timers: number; observers: number; errors: number; aborted: boolean };
type Coordinator = { state(): string; failure(): string | null; stats(): RuntimeStats | null };
type Snapshot = {
    key: string;
    root: Element;
    original: Element;
    nodes: Node[];
    ids: [Element, string][];
    controls: { node: HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement; value: string; checked: boolean | null }[];
    source: Element | null;
    statusHeight: number;
    sourceTop: number | null;
};
type Audit = {
    requests: string[];
    failures: { url: string; error: string }[];
    responses: { url: string; status: number }[];
    console: { text: string; url: string }[];
    pageErrors: string[];
};

declare global {
    interface Window {
        __rustify_docs_islands?: Coordinator;
        __docs_island_csp: string[];
        __docs_island_probe?: { blocks: Snapshot[]; navigation: Element[] };
        __docs_init_probe?: { module: boolean; instance: boolean; attachment: boolean; runtime: boolean };
    }
}

async function watch(page: Page): Promise<Audit> {
    const audit: Audit = { requests: [], failures: [], responses: [], console: [], pageErrors: [] };
    await page.addInitScript(() => {
        window.__docs_island_csp = [];
        document.addEventListener("securitypolicyviolation", event => {
            window.__docs_island_csp.push(`${event.violatedDirective} ${event.blockedURI}`);
        });
    });
    page.on("request", request => audit.requests.push(request.url()));
    page.on("requestfailed", request => audit.failures.push({ url: request.url(), error: request.failure()?.errorText ?? "" }));
    page.on("response", response => {
        if (response.status() >= 400) audit.responses.push({ url: response.url(), status: response.status() });
    });
    page.on("console", message => {
        if (message.type() === "error") audit.console.push({ text: message.text(), url: message.location().url });
    });
    page.on("pageerror", error => audit.pageErrors.push(String(error)));
    return audit;
}

const test = playwright.extend<{ audit: Audit }>({
    audit: [async ({ page }, use, info) => {
        const audit = await watch(page);
        await use(audit);
        const browser = await page.evaluate(() => ({
            csp: window.__docs_island_csp,
            state: window.__rustify_docs_islands?.state(),
            failure: window.__rustify_docs_islands?.failure(),
            stats: window.__rustify_docs_islands?.stats(),
        }));
        await info.attach("island-browser-audit.json", { body: JSON.stringify({ ...audit, browser }, null, 2), contentType: "application/json" });
    }, { auto: true }],
});

const islands = (page: Page) => page.locator("[data-rustify-island]");
const canvas = (island: Locator) => island.locator("fieldset[data-rd-island-controls]");
const assetPattern = (file: string) => `**/assets/islands/**/${file}`;
const fixtureSource = () => readFileSync("tests/docs-ssg/fixtures/components/src/demos.rs", "utf8");
const url = (route: string) => new URL(route, String(test.info().project.use.baseURL)).href;

async function healthy(page: Page, audit: Audit) {
    expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
    expect(audit.failures).toEqual([]);
    expect(audit.responses).toEqual([]);
    expect(audit.console).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    const origin = new URL(url("./")).origin;
    expect(audit.requests.filter(request => new URL(request).origin !== origin)).toEqual([]);
}

async function ready(page: Page, count: number) {
    await expect(islands(page)).toHaveCount(count);
    await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(count);
    expect(await page.evaluate(() => window.__rustify_docs_islands?.state())).toBe("ready");
    for (const island of await islands(page).all()) {
        await expect(canvas(island)).toHaveJSProperty("inert", false);
        await expect(canvas(island)).toHaveJSProperty("disabled", false);
        await expect(canvas(island)).toHaveAttribute("aria-busy", "false");
        const metadata = JSON.parse((await island.getAttribute("data-rustify-island-metadata"))!);
        await expect(canvas(island).locator(":scope > [data-rustify-scope]")).toHaveAttribute("data-rustify-mounted", metadata.seed);
        await expect(island.locator("[data-rd-island-loading]")).toBeHidden();
        await expect(island.locator("[data-rd-island-error]")).toBeHidden();
        await expect(island.locator("[data-rd-island-reload]")).toBeHidden();
        await expect(island.locator("[data-rd-island-error]")).toHaveAttribute("aria-hidden", "true");
        await expect(island.locator("[data-rd-island-reload]")).toHaveAttribute("aria-hidden", "true");
        expect(await island.locator("[data-rd-island-reload]").evaluate(button => {
            (button as HTMLElement).focus();
            return document.activeElement === button;
        })).toBe(false);
    }
}

// Hold the real resource before WebAssembly runs; every assertion keeps the
// server's actual nodes and uses the normal loader once the route is released.
async function holdAsset(page: Page, file: string) {
    let release!: (fail?: boolean) => void;
    let observed!: (value: string) => void;
    const released = new Promise<boolean>(resolve => { release = (fail = false) => resolve(fail); });
    const seen = new Promise<string>(resolve => { observed = resolve; });
    const routeHandler = async (route: Route) => {
        observed(route.request().url());
        if (await released) {
            await route.fulfill({ status: 503, contentType: "text/plain", body: "Injected island asset failure" });
        } else {
            await route.continue();
        }
    };
    await page.route(assetPattern(file), routeHandler);
    return { seen, release, remove: () => page.unroute(assetPattern(file), routeHandler) };
}

async function captureOriginals(page: Page) {
    const before = await page.evaluate(() => {
        const blocks = [...document.querySelectorAll("[data-rustify-island]")].map(block => {
            const field = block.querySelector("fieldset[data-rd-island-controls]")!;
            const root = field.firstElementChild!;
            const nodes: Node[] = [root];
            const walker = document.createTreeWalker(root, NodeFilter.SHOW_ALL);
            while (walker.nextNode()) nodes.push(walker.currentNode);
            const source = block.querySelector(".rd-demo-source");
            const controls = [...root.querySelectorAll<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>("input,textarea,select")].map(node => ({
                node, value: node.value, checked: node instanceof HTMLInputElement ? node.checked : null,
            }));
            return {
                key: block.getAttribute("data-rustify-island")!, root, original: root.cloneNode(true) as Element,
                nodes, ids: [...root.querySelectorAll("[id]")].map(node => [node, node.id] as [Element, string]),
                controls, source,
                statusHeight: block.querySelector("[data-rd-island-status]")!.getBoundingClientRect().height,
                sourceTop: source ? source.getBoundingClientRect().top + scrollY : null,
            };
        });
        window.__docs_island_probe = { blocks, navigation: [...document.querySelectorAll("header a, .rd-header-nav a, .rd-sidebar a, .rd-pager a")] };
        return blocks.map(block => ({ key: block.key, nodes: block.nodes.length, html: block.original.outerHTML, values: block.controls.map(control => control.value) }));
    });
    await test.info().attach("original-ssr-nodes.json", { body: JSON.stringify(before, null, 2), contentType: "application/json" });
}

async function identity(page: Page) {
    const actual = await page.evaluate(() => window.__docs_island_probe!.blocks.map(snapshot => {
        const block = [...document.querySelectorAll("[data-rustify-island]")].find(block => block.getAttribute("data-rustify-island") === snapshot.key)!;
        const field = block.querySelector("fieldset[data-rd-island-controls]")!;
        return {
            key: snapshot.key, nodes: snapshot.nodes.length,
            sameRoot: field.firstElementChild === snapshot.root,
            retained: snapshot.nodes.filter(node => node.isConnected && snapshot.root.contains(node)).length,
            sameIds: snapshot.ids.every(([node, id]) => node.id === id),
            sameNativeValues: snapshot.controls.every(control => control.node.value === control.value && (!(control.node instanceof HTMLInputElement) || control.node.checked === control.checked)),
            sameSource: block.querySelector(".rd-demo-source") === snapshot.source,
            statusHeight: block.querySelector("[data-rd-island-status]")!.getBoundingClientRect().height,
            initialStatusHeight: snapshot.statusHeight,
            sourceTop: snapshot.source ? snapshot.source.getBoundingClientRect().top + scrollY : null,
            initialSourceTop: snapshot.sourceTop,
        };
    }));
    await test.info().attach("attached-node-identity.json", { body: JSON.stringify(actual, null, 2), contentType: "application/json" });
    for (const block of actual) {
        expect(block.sameRoot, block.key).toBe(true);
        expect(block.retained, block.key).toBe(block.nodes);
        expect(block.sameIds, block.key).toBe(true);
        expect(block.sameNativeValues, block.key).toBe(true);
        expect(block.sameSource, block.key).toBe(true);
        expect(block.statusHeight, block.key).toBe(block.initialStatusHeight);
        expect(block.sourceTop, block.key).toBe(block.initialSourceTop);
    }
    expect(await page.evaluate(() => {
        const ids = [...document.querySelectorAll("[id]")].map(node => node.id);
        return ids.length === new Set(ids).size;
    })).toBe(true);
}

async function openPaused(page: Page, route: string, file = "app_bg.wasm") {
    const held = await holdAsset(page, file);
    await page.goto(url(route), { waitUntil: "commit" });
    await held.seen;
    await expect(page.locator('[data-rustify-island][data-rd-island-state="loading"]')).toHaveCount(await islands(page).count());
    await captureOriginals(page);
    return held;
}

async function restored(page: Page, count: number) {
    await expect(page.locator('[data-rustify-island][data-rd-island-state="failed"]')).toHaveCount(count);
    expect(await page.evaluate(() => window.__rustify_docs_islands?.state())).toBe("failed");
    const result = await page.evaluate(() => ({
        blocks: window.__docs_island_probe!.blocks.map(snapshot => {
            const block = [...document.querySelectorAll("[data-rustify-island]")].find(block => block.getAttribute("data-rustify-island") === snapshot.key)!;
            return {
                key: snapshot.key,
                original: block.querySelector("fieldset[data-rd-island-controls]")!.firstElementChild!.isEqualNode(snapshot.original),
                sameSource: block.querySelector(".rd-demo-source") === snapshot.source,
            };
        }),
        navigation: window.__docs_island_probe!.navigation.every(node => node.isConnected),
    }));
    await test.info().attach("restored-ssr-trees.json", { body: JSON.stringify(result, null, 2), contentType: "application/json" });
    expect(result.navigation).toBe(true);
    for (const block of result.blocks) {
        expect(block.original, block.key).toBe(true);
        expect(block.sameSource, block.key).toBe(true);
    }
    for (const island of await islands(page).all()) {
        await expect(canvas(island)).toHaveJSProperty("disabled", true);
        await expect(canvas(island)).toHaveJSProperty("inert", true);
        await expect(canvas(island)).toHaveAttribute("aria-busy", "false");
        await expect(island.locator("[data-rd-island-status]")).toHaveAttribute("role", "alert");
        await expect(island.locator("[data-rd-island-error]")).toHaveText("This example could not start. Reload the page to try again.");
        await expect(island.locator("[data-rd-island-error]")).toBeVisible();
        await expect(island.locator("[data-rd-island-error]")).toHaveAttribute("aria-hidden", "false");
        await expect(island.locator("[data-rd-island-reload]")).toHaveAttribute("aria-hidden", "false");
        await expect(island.locator("[data-rd-island-reload]")).toBeEnabled();
        await expect(island.locator("[data-rd-island-loading]")).toBeHidden();
    }
}

async function sourceCopy(page: Page) {
    const source = page.locator(".rd-demo-source");
    await expect(source).toHaveCount(1);
    await expect(source.locator("[data-rd-source-path]")).toHaveText("src/demos.rs");
    const file = fixtureSource();
    const originalFunction = file.match(/(?:\/\/\/[^\r\n]*\r?\n)*pub fn counter_demo\(\)[\s\S]*?^\}(?:\r?\n|$)/m)?.[0];
    expect(originalFunction, "the fixture's direct function is present").toBeTruthy();
    const expected = [originalFunction!, file];
    const raw = await source.locator("template.rd-code-raw").evaluateAll(nodes => nodes.map(node => (node as HTMLTemplateElement).content.textContent));
    expect(raw).toEqual(expected);
    for (const [index, label] of ["Function", "Full file"].entries()) {
        const tab = source.getByRole("tab", { name: label, exact: true });
        await tab.click();
        await expect(tab).toHaveAttribute("aria-selected", "true");
        await expect(source.locator(`[data-rd-tab-panel="${index}"]`)).toBeVisible();
        await source.locator(".rd-code-tabs > [data-rd-copy]").click();
        expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(expected[index]);
        await expect(source.locator(".rd-code-tabs > [data-rd-copy]")).toHaveAttribute("data-state", "copied");
    }
    await source.getByRole("tab", { name: "Full file", exact: true }).press("Home");
    await expect(source.getByRole("tab", { name: "Function", exact: true })).toBeFocused();
    await expect(source.getByRole("tab", { name: "Function", exact: true })).toHaveAttribute("aria-selected", "true");
}

async function failedStartup(page: Page, audit: Audit, httpFailure?: { url: string; status: number }) {
    await restored(page, 4);
    await expect(page.getByTestId("counter-value")).toHaveText(["2", "2", "2", "7"]);
    expect(await page.evaluate(() => window.__rustify_docs_islands?.failure())).toBeTruthy();
    await sourceCopy(page);
    const navigation = page.locator(".rd-header-nav-wide").getByRole("link", { name: "Counters", exact: true });
    await expect(navigation).toHaveAttribute("href", new URL(url("counter/")).pathname);
    await navigation.focus();
    await expect(navigation).toBeFocused();
    expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
    expect(audit.console.filter(message => !/^Interactive examples failed/.test(message.text)
        && !(httpFailure && message.url === httpFailure.url && message.text.includes(String(httpFailure.status))))).toEqual([]);
    expect(audit.responses).toEqual(httpFailure ? [httpFailure] : []);
    expect(audit.failures.filter(request => request.url !== httpFailure?.url)).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
}

async function reloadCounters(page: Page) {
    const navigation = page.waitForNavigation({ waitUntil: "load" });
    await islands(page).first().getByRole("button", { name: "Reload page", exact: true }).click();
    await navigation;
    await ready(page, 4);
    await expect(page.getByTestId("counter-value")).toHaveText(["2", "2", "2", "7"]);
}

test("a page without interactive embeds loads no island JavaScript or wasm", async ({ page, audit }) => {
    await page.goto(url("./"));
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("Component fixture");
    await expect(islands(page)).toHaveCount(0);
    expect(audit.requests.filter(request => /\/assets\/islands\//.test(new URL(request).pathname) || new URL(request).pathname.endsWith(".wasm"))).toEqual([]);
    await healthy(page, audit);
});

test("successful island wasm has its native MIME type under the page's CSP", async ({ page, audit }) => {
    const wasmResponse = page.waitForResponse(response => /\/assets\/islands\/[^/]+\/app_bg\.wasm$/.test(new URL(response.url()).pathname));
    const documentResponse = await page.goto(url("counter/"));
    const wasm = await wasmResponse;
    await ready(page, 4);
    expect(wasm.status()).toBe(200);
    expect(wasm.headers()["content-type"].split(";")[0].trim()).toBe("application/wasm");
    const policy = "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'";
    await expect(page.locator('meta[http-equiv="Content-Security-Policy"]')).toHaveAttribute("content", policy);
    const isMeta = test.info().project.name === "components-meta";
    if (isMeta) {
        expect(documentResponse!.headers()["content-security-policy"]).toBeUndefined();
        expect(wasm.headers()["content-security-policy"]).toBeUndefined();
    } else {
        expect(documentResponse!.headers()["content-security-policy"]).toBe(`${policy}; frame-ancestors 'none'`);
    }
    await test.info().attach("wasm-mime-csp.json", {
        body: JSON.stringify({ project: test.info().project.name, url: wasm.url(), mime: wasm.headers()["content-type"], documentCsp: documentResponse!.headers()["content-security-policy"] ?? null, wasmCsp: wasm.headers()["content-security-policy"] ?? null, meta: policy }, null, 2),
        contentType: "application/json",
    });
    await islands(page).first().getByTestId("counter-increment").click();
    await expect(islands(page).first().getByTestId("counter-value")).toHaveText("3");
    await healthy(page, audit);
});

test("no script keeps real SSR views and both source panels readable with native controls disabled", async ({ browser }) => {
    const context = await browser.newContext({ javaScriptEnabled: false, reducedMotion: "reduce" });
    const page = await context.newPage();
    const audit = await watch(page);
    try {
        await page.goto(url("counter/"));
        await expect(islands(page)).toHaveCount(4);
        for (const [index, island] of (await islands(page).all()).entries()) {
            await expect(island).toHaveAttribute("data-rd-island-state", "no-script");
            await expect(canvas(island)).toHaveAttribute("disabled", "");
            await expect(canvas(island)).toHaveJSProperty("disabled", true);
            await expect(island.getByTestId("counter-increment")).toBeDisabled();
            await expect(island.getByTestId("counter-input")).toBeDisabled();
            await expect(island.getByTestId("counter-input")).toHaveValue(index === 3 ? "7" : "2");
            await expect(island.getByTestId("counter-value")).toHaveText(index === 3 ? "7" : "2");
            await expect(island.locator("noscript > span")).toHaveText("Enable JavaScript to interact");
            await expect(island.locator("noscript > span")).toBeVisible();
            await expect(island.locator("[data-rd-island-reload]")).toBeHidden();
        }
        await expect(page.locator(".rd-demo-source .rd-code")).toHaveCount(2);
        await expect(page.locator(".rd-demo-source .rd-code-tabs")).toBeHidden();
        for (const panel of await page.locator(".rd-demo-source .rd-code").all()) await expect(panel).toBeVisible();
        expect(await page.locator("template.rd-code-raw").last().evaluate(node => (node as HTMLTemplateElement).content.textContent)).toBe(fixtureSource());
        await page.goto(url("client-only/"));
        await expect(page.locator("[data-rustify-client-message]")).toHaveText("This counter starts in the browser.");
        await expect(page.getByTestId("counter-increment")).toHaveCount(0);
        expect((await page.locator("[data-rustify-client-only]").boundingBox())!.height).toBeGreaterThanOrEqual(192);
        await page.goto(url("proxy/"));
        await expect(page.locator("[data-category]")).toHaveCount(24);
        await expect(page.getByRole("textbox", { name: "Multiline text" })).toHaveValue("\nRésumé </textarea> & <line>\nCRLF\nCR\nlast");
        await expect(page.getByRole("textbox", { name: "Multiline text" })).toBeDisabled();
        expect(audit.requests.filter(request => new URL(request).pathname.endsWith(".wasm"))).toEqual([]);
        expect(audit.failures).toEqual([]);
        expect(audit.responses).toEqual([]);
        expect(audit.console).toEqual([]);
        expect(audit.pageErrors).toEqual([]);
        await test.info().attach("no-script-browser-audit.json", { body: JSON.stringify(audit, null, 2), contentType: "application/json" });
    } finally {
        await context.close();
    }
});

test("loading gates native and captured input while source controls remain usable", async ({ page, context, audit }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    const held = await openPaused(page, "counter/");
    try {
        const island = islands(page).first();
        await expect(canvas(island)).toHaveJSProperty("disabled", false);
        await expect(canvas(island)).toHaveJSProperty("inert", true);
        await expect(canvas(island)).toHaveAttribute("aria-busy", "true");
        await expect(island.locator("[data-rd-island-loading]")).toBeVisible();
        const result = await island.getByTestId("counter-increment").evaluate(button => {
            let delivered = 0;
            const names = ["click", "pointerdown", "keydown", "beforeinput", "input", "change", "submit"];
            for (const name of names) button.addEventListener(name, () => delivered++);
            const accepted = names.map(name => button.dispatchEvent(new Event(name, { bubbles: true, cancelable: true })));
            (button as HTMLElement).focus();
            return { accepted, delivered, focused: document.activeElement === button };
        });
        expect(result).toEqual({ accepted: Array(7).fill(false), delivered: 0, focused: false });
        const box = (await island.getByTestId("counter-increment").boundingBox())!;
        await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
        await expect(island.getByTestId("counter-value")).toHaveText("2");
        await sourceCopy(page);
        held.release();
        await ready(page, 4);
        await identity(page);
        await healthy(page, audit);
    } finally {
        held.release();
    }
});

test("attachment retains original SSR nodes and isolates three identical counters", async ({ page, audit }) => {
    const held = await openPaused(page, "counter/");
    held.release();
    await ready(page, 4);
    await identity(page);
    const counters = islands(page).filter({ has: page.locator('[data-counter]') });
    await expect(counters).toHaveCount(4);
    await counters.nth(0).getByTestId("counter-increment").click();
    await expect(counters.nth(0).getByTestId("counter-value")).toHaveText("3");
    await counters.nth(1).getByTestId("counter-input").fill("19");
    await expect(counters.nth(1).getByTestId("counter-value")).toHaveText("19");
    await counters.nth(2).getByTestId("counter-increment").focus();
    await page.keyboard.press("Enter");
    await expect(counters.nth(2).getByTestId("counter-value")).toHaveText("3");
    await expect(counters.nth(0).getByTestId("counter-value")).toHaveText("3");
    await expect(counters.nth(1).getByTestId("counter-value")).toHaveText("19");
    await expect(counters.nth(3).getByTestId("counter-value")).toHaveText("7");
    expect((await page.getByTestId("counter-input").all()).length).toBe(4);
    await healthy(page, audit);
});

test("an asynchronous action starts after attachment with supported hydration globals", async ({ page, audit }) => {
    const held = await openPaused(page, "async/");
    await expect(page.getByTestId("async-result")).toHaveText("Not loaded");
    held.release();
    await ready(page, 1);
    await identity(page);
    expect(await page.evaluate(() => ["__RESOLVED_RESOURCES", "__INCOMPLETE_CHUNKS", "__SERIALIZED_ERRORS"].map(name => Array.isArray((window as unknown as Record<string, unknown>)[name])))).toEqual([true, true, true]);
    await page.getByTestId("async-start").click();
    await expect(page.getByTestId("async-result")).toHaveText("Loaded after attachment");
    await healthy(page, audit);
});

test("client-only attachment preserves the placeholder and mounts typed props and child content", async ({ page, audit }) => {
    const held = await openPaused(page, "client-only/");
    const placeholder = page.locator("[data-rustify-client-only]");
    const original = await placeholder.elementHandle();
    const preserved = await page.evaluateHandle(() => {
        const snapshot = window.__docs_island_probe!.blocks[0];
        const message = snapshot.root.querySelector("[data-rustify-client-message]")!;
        return snapshot.nodes.filter(node => !message.contains(node));
    });
    const height = (await placeholder.boundingBox())!.height;
    await expect(page.locator("[data-rustify-client-message]")).toBeVisible();
    held.release();
    await ready(page, 1);
    expect(await original!.evaluate(node => node.isConnected)).toBe(true);
    expect(await preserved.evaluate(nodes => nodes.every(node => node.isConnected))).toBe(true);
    expect((await placeholder.boundingBox())!.height).toBeGreaterThanOrEqual(height);
    await expect(page.locator("[data-rustify-client-message]")).toHaveCount(0);
    const metadata = JSON.parse((await islands(page).first().getAttribute("data-rustify-island-metadata"))!);
    const host = page.locator("[data-rustify-client-host]");
    await expect(host).toHaveAttribute("data-rustify-scope", `${metadata.seed}-client`);
    await expect(host.locator(":scope > [data-rustify-content]")).toHaveCount(1);
    await expect(host.locator(":scope > [data-rustify-overlay]")).toHaveCount(1);
    await expect(page.getByTestId("counter-value")).toHaveText("2");
    await expect(page.locator("[data-rustify-children] strong")).toHaveText("browser-created child");
    await page.getByTestId("counter-increment").click();
    await expect(page.getByTestId("counter-value")).toHaveText("3");
    await healthy(page, audit);
});

test("static builtins need no script or wasm and retain every panel, native link and child block", async ({ browser }) => {
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();
    const audit = await watch(page);
    try {
        await page.goto(url("builtins/"));
        await expect(page.getByRole("heading", { level: 1 })).toHaveText("Static built-in components");
        await expect(islands(page)).toHaveCount(0);
        await expect(page.locator("html")).toHaveAttribute("data-rustify-scope", "");
        await expect(page.locator("body [data-rustify-scope], fieldset[data-rd-island-controls]")).toHaveCount(0);
        const groups = page.locator("[data-rd-static-tabs]");
        await expect(groups).toHaveCount(2);
        await expect(page.locator("[data-rd-static-panel]")).toHaveCount(4);
        for (const group of await groups.all()) {
            await expect(group.locator(":scope > [data-rd-static-tabbar]")).toBeHidden();
            await expect(group.locator(":scope > span[data-rustify-children]")).toHaveCount(1);
            for (const panel of await group.locator(":scope > [data-rd-static-panel]").all()) await expect(panel).toBeVisible();
        }
        await expect(groups.first().locator('[data-rd-static-panel="1"]')).toHaveAttribute("aria-label", "First group code");
        await expect(groups.first().locator('[data-rd-static-panel="1"] .rd-static-text')).toHaveText("A literal <tag> & raw value stays text.");
        await expect(groups.first().locator("tag")).toHaveCount(0);
        const children = groups.first().locator(":scope > span[data-rustify-children]");
        await expect(children.locator("strong")).toHaveText("checked Markdown");
        await expect(children.getByRole("link", { name: "registered counter", exact: true })).toHaveAttribute("href", new URL(url("counter/")).pathname);
        await expect(children.locator(".rd-code-pre code")).toHaveText('let text = "<tag> & raw";');
        await expect(page.locator(".rd-static-card-grid")).toHaveAttribute("data-columns", "2");
        await expect(page.locator(".rd-static-card-grid > ul > li")).toHaveCount(2);
        await expect(page.locator(".rd-static-card-grid").getByRole("link", { name: "Registered counters", exact: true })).toHaveAttribute("href", new URL(url("counter/")).pathname);
        await expect(page.locator(".rd-static-card-grid").getByRole("link", { name: "All categories", exact: true })).toHaveAttribute("href", new URL(url("proxy/")).pathname);
        await expect(page.locator(".rd-static-steps > ol > li .rd-static-step-title")).toHaveText(["Read the static example", "Open a counter"]);
        await expect(page.locator("[data-rustify-children]")).toHaveCount(4);
        for (const width of [1280, 375]) {
            await page.setViewportSize({ width, height: 800 });
            expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(0);
            await page.screenshot({ path: test.info().outputPath(`builtins-no-script-${width}.png`), fullPage: true });
        }
        expect(audit.requests.filter(request => /\/assets\/islands\//.test(new URL(request).pathname) || new URL(request).pathname.endsWith(".wasm"))).toEqual([]);
        expect(audit.failures).toEqual([]);
        expect(audit.responses).toEqual([]);
        expect(audit.console).toEqual([]);
        expect(audit.pageErrors).toEqual([]);
        await test.info().attach("builtin-no-script-browser-audit.json", { body: JSON.stringify(audit, null, 2), contentType: "application/json" });
    } finally {
        await context.close();
    }
});

test("static tabs use unique ARIA mappings and keyboard state stays local to each peer", async ({ page, context, audit }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    await page.goto(url("builtins/"));
    const groups = page.locator("[data-rd-static-tabs]");
    await expect(page.locator("[data-rd-static-tabs][data-rd-static-enhanced]")).toHaveCount(2);
    const panels = await page.locator("[data-rd-static-panel]").elementHandles();
    for (const group of await groups.all()) {
        await expect(group.locator(":scope > [data-rd-static-tabbar]")).toHaveAttribute("role", "tablist");
        const tabs = await group.locator("[data-rd-static-tab]").all();
        for (const [index, tab] of tabs.entries()) {
            const panel = group.locator(`[data-rd-static-panel="${index}"]`);
            await expect(tab).toHaveAttribute("role", "tab");
            await expect(tab).toHaveAttribute("aria-controls", (await panel.getAttribute("id"))!);
            await expect(panel).toHaveAttribute("role", "tabpanel");
            await expect(panel).toHaveAttribute("aria-labelledby", (await tab.getAttribute("id"))!);
            await expect(tab).toHaveJSProperty("tabIndex", index === 0 ? 0 : -1);
            await expect(panel).toHaveJSProperty("hidden", index !== 0);
        }
    }
    expect(await page.evaluate(() => {
        const ids = [...document.querySelectorAll("[id]")].map(node => node.id);
        return ids.length === new Set(ids).size;
    })).toBe(true);
    const first = groups.nth(0).locator("[data-rd-static-tab]");
    const second = groups.nth(1).locator("[data-rd-static-tab]");
    await first.nth(0).focus();
    await page.keyboard.press("ArrowRight");
    await expect(first.nth(1)).toBeFocused();
    await expect(first.nth(1)).toHaveAttribute("aria-selected", "true");
    await expect(groups.nth(0).locator('[data-rd-static-panel="1"]')).toBeVisible();
    await expect(second.nth(0)).toHaveAttribute("aria-selected", "true");
    await second.nth(0).focus();
    await page.keyboard.press("ArrowLeft");
    await expect(second.nth(1)).toBeFocused();
    await expect(second.nth(1)).toHaveAttribute("aria-selected", "true");
    await expect(first.nth(1)).toHaveAttribute("aria-selected", "true");
    await first.nth(1).press("Home");
    await expect(first.nth(0)).toBeFocused();
    await first.nth(0).press("End");
    await expect(first.nth(1)).toBeFocused();
    await first.nth(1).press("ArrowRight");
    await expect(first.nth(0)).toBeFocused();
    await expect(first.nth(0)).toHaveAttribute("aria-selected", "true");
    await expect(second.nth(1)).toHaveAttribute("aria-selected", "true");
    await groups.first().locator(".rd-code [data-rd-copy]").click();
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe('let text = "<tag> & raw";\n');
    await expect(first.nth(0)).toHaveAttribute("aria-selected", "true");
    await expect(second.nth(1)).toHaveAttribute("aria-selected", "true");
    for (const panel of panels) expect(await panel.evaluate(node => node.isConnected && document.getElementById(node.id) === node)).toBe(true);
    expect(audit.requests.filter(request => /\/assets\/islands\//.test(new URL(request).pathname) || new URL(request).pathname.endsWith(".wasm"))).toEqual([]);
    await page.locator(".rd-static-card-grid").getByRole("link", { name: "Registered counters", exact: true }).click();
    await expect(page).toHaveURL(url("counter/"));
    await ready(page, 4);
    await healthy(page, audit);
});

test("a real callback trap restores every initial island and keeps source, navigation and reload alive", async ({ page, context, audit }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    const held = await openPaused(page, "failure/");
    held.release();
    await ready(page, 2);
    await identity(page);
    await page.getByTestId("counter-increment").click();
    await expect(page.getByTestId("counter-value")).toHaveText("3");
    await page.getByTestId("failure-increment").click();
    await expect(page.getByTestId("failure-value")).toHaveText("1");
    await page.getByTestId("failure-crash").click();
    await restored(page, 2);
    await expect(page.getByTestId("counter-value")).toHaveText("2");
    await expect(page.getByTestId("failure-value")).toHaveText("0");
    expect(await page.evaluate(() => window.__rustify_docs_islands?.stats())).toMatchObject({ tasks: 0, timers: 0, observers: 0, aborted: true });
    await page.locator("[data-rd-search-open]").click();
    const search = page.locator("dialog[data-rd-search]");
    await search.getByRole("combobox").fill("Counter");
    await expect(search).toHaveAttribute("data-state", "results");
    await expect(search.getByRole("option").first()).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(page.locator("[data-rd-search-open]")).toBeFocused();
    const source = page.locator(".rd-demo-source");
    await source.getByRole("tab", { name: "Full file", exact: true }).click();
    await source.locator(".rd-code-tabs > [data-rd-copy]").click();
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(fixtureSource());
    await page.locator('.rd-header-nav-wide a[href]').filter({ hasText: "Counters" }).click();
    await expect(page).toHaveURL(url("counter/"));
    await ready(page, 4);
    expect(audit.pageErrors.length).toBeGreaterThanOrEqual(1);
    expect(audit.pageErrors.filter(error => !/^RuntimeError: unreachable$/.test(error))).toEqual([]);
    expect(audit.console.filter(message => !/^Interactive examples failed/.test(message.text) && !/docs fixture callback failure/.test(message.text))).toEqual([]);
    expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
    expect(audit.failures).toEqual([]);
    expect(audit.responses).toEqual([]);
    expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
});

for (const file of ["islands.js", "app.js", "app_bg.wasm"]) {
    test(`${file} load failure is terminal, preserves SSR and recovers only through Reload page`, async ({ page, audit }) => {
        const held = await openPaused(page, "counter/", file);
        const failed = [await held.seen];
        held.release(true);
        await restored(page, 4);
        await expect(page.getByTestId("counter-value")).toHaveText(["2", "2", "2", "7"]);
        expect(failed).toHaveLength(1);
        expect(await page.evaluate(() => window.__rustify_docs_islands?.failure())).toBeTruthy();
        await page.locator(".rd-demo-source").getByRole("tab", { name: "Full file", exact: true }).click();
        await expect(page.locator('.rd-demo-source [data-rd-tab-panel="1"]')).toBeVisible();
        expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
        expect(audit.console.filter(message => !/^Interactive examples failed/.test(message.text) && !(/503/.test(message.text) && failed.includes(message.url)))).toEqual([]);
        expect(audit.responses).toEqual([{ url: failed[0], status: 503 }]);
        expect(audit.failures.filter(request => !failed.includes(request.url))).toEqual([]);
        expect(audit.pageErrors).toEqual([]);
        expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
        await held.remove();
        const navigation = page.waitForNavigation({ waitUntil: "load" });
        await islands(page).first().getByRole("button", { name: "Reload page", exact: true }).click();
        await navigation;
        await ready(page, 4);
    });
}

test("an imported SDK snippet's real 404 restores SSR and leaves source, navigation and reload usable", async ({ page, context, audit }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    const held = await openPaused(page, "counter/");
    const glueUrl = new URL("app.js", await held.seen).href;
    const glue = await page.request.get(glueUrl);
    expect(glue.status()).toBe(200);
    const imports = [...(await glue.text()).matchAll(/\bfrom\s+['"]([^'"]+)['"]/g)].map(match => match[1]);
    const sdkImports = imports.filter(path => /\/snippets\/rustify-ui-[^/]+\/web\/dom-runtime\.js$/.test(path));
    expect(sdkImports).toHaveLength(1);
    const snippet = new URL(sdkImports[0], glueUrl).href;
    expect((await page.request.get(snippet)).status()).toBe(200);
    await test.info().attach("sdk-snippet-import.json", { body: JSON.stringify({ glue: glueUrl, import: sdkImports[0], resource: snippet }), contentType: "application/json" });
    const failSnippet = (route: Route) => route.fulfill({ status: 404, contentType: "text/plain", body: "Injected missing SDK snippet" });
    await page.route(snippet, failSnippet);
    held.release();
    await failedStartup(page, audit, { url: snippet, status: 404 });
    await page.unroute(snippet, failSnippet);
    await held.remove();
    await reloadCounters(page);
    expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
    expect(audit.responses).toEqual([{ url: snippet, status: 404 }]);
});

test("invalid wasm bytes in a successful HTTP response fail terminally before attachment", async ({ page, context, audit }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    let replaced = 0;
    const corruptWasm = async (route: Route) => {
        replaced++;
        await captureOriginals(page);
        await route.fulfill({ status: 200, contentType: "application/wasm", body: Buffer.from("Invalid wasm module bytes", "utf8") });
    };
    await page.route(assetPattern("app_bg.wasm"), corruptWasm);
    await page.goto(url("counter/"));
    await failedStartup(page, audit);
    expect(replaced).toBe(1);
    expect(await page.evaluate(() => window.__rustify_docs_islands?.failure())).toContain("CompileError");
    await page.unroute(assetPattern("app_bg.wasm"), corruptWasm);
    await reloadCounters(page);
    expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
    expect(audit.responses).toEqual([]);
});

test("a glue init exception after real wasm instantiation preserves SSR and recovers through reload", async ({ page, context, audit }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    let replaced = 0;
    const failInit = async (route: Route) => {
        const response = await route.fetch();
        expect(response.status()).toBe(200);
        const original = await response.text();
        const start = original.indexOf("async function __wbg_init(module_or_path) {");
        const end = original.indexOf("\nexport { initSync, __wbg_init as default };", start);
        expect(start).toBeGreaterThanOrEqual(0);
        expect(end).toBeGreaterThan(start);
        const init = original.slice(start, end);
        const finalizer = "return __wbg_finalize_init(instance, module);";
        expect(init.split(finalizer)).toHaveLength(2);
        const changed = init.replace(finalizer, `__wbg_finalize_init(instance, module);
    globalThis.__docs_init_probe = {
        module: module instanceof WebAssembly.Module,
        instance: instance instanceof WebAssembly.Instance,
        attachment: typeof instance.exports.docs_hydrate === 'function',
        runtime: typeof instance.exports.rustify_runtime_boot === 'function'
    };
    throw new Error('Injected glue initialization failure');`);
        replaced++;
        await captureOriginals(page);
        await route.fulfill({ response, body: original.slice(0, start) + changed + original.slice(end) });
    };
    await page.route(assetPattern("app.js"), failInit);
    await page.goto(url("counter/"));
    await failedStartup(page, audit);
    expect(replaced).toBe(1);
    expect(await page.evaluate(() => window.__docs_init_probe)).toEqual({ module: true, instance: true, attachment: true, runtime: true });
    expect(await page.evaluate(() => window.__rustify_docs_islands?.failure())).toBe("Error: Injected glue initialization failure");
    await page.unroute(assetPattern("app.js"), failInit);
    await reloadCounters(page);
    expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
    expect(audit.responses).toEqual([]);
});

test("invalid hydration metadata releases the live native host and restores untouched SSR", async ({ page, audit }) => {
    const held = await openPaused(page, "counter/");
    await islands(page).first().evaluate(block => {
        const metadata = JSON.parse(block.getAttribute("data-rustify-island-metadata")!);
        metadata.seed += "-wrong";
        block.setAttribute("data-rustify-island-metadata", JSON.stringify(metadata));
    });
    held.release();
    await restored(page, 4);
    expect(await page.evaluate(() => window.__rustify_docs_islands?.stats())).toMatchObject({ tasks: 0, timers: 0, observers: 0, aborted: true });
    expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
    expect(audit.console.filter(message => !/^Interactive examples failed/.test(message.text))).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    expect(audit.failures).toEqual([]);
    expect(audit.responses).toEqual([]);
    expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
});

test("unsupported initial hydration globals refuse startup before wasm runs", async ({ page, audit }) => {
    await page.addInitScript(() => { (window as unknown as Record<string, unknown>).__RESOLVED_RESOURCES = ["unsupported"]; });
    const held = await openPaused(page, "counter/", "islands.js");
    held.release();
    await restored(page, 4);
    expect(await page.evaluate(() => window.__rustify_docs_islands?.failure())).toContain("unsupported initial hydration data");
    expect(audit.requests.filter(request => new URL(request).pathname.endsWith(".wasm"))).toEqual([]);
    expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
    expect(audit.console.filter(message => !/^Interactive examples failed/.test(message.text))).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    expect(audit.failures).toEqual([]);
    expect(audit.responses).toEqual([]);
    expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
});

test("the startup deadline remains terminal when the real wasm completes late", async ({ page, audit }) => {
    // Playwright virtualizes the actual performance clock and timer callbacks;
    // this does not claim a thirty-second physical network measurement.
    await page.clock.install({ time: new Date("2026-01-01T00:00:00Z") });
    const held = await openPaused(page, "counter/");
    await page.clock.pauseAt((await page.evaluate(() => Date.now())) + 1000);
    const before = await page.evaluate(() => performance.now());
    await page.clock.runFor(30001);
    expect((await page.evaluate(() => performance.now())) - before).toBe(30001);
    await restored(page, 4);
    expect(await page.evaluate(() => window.__rustify_docs_islands?.failure())).toBe("Error: island startup deadline exceeded");
    held.release();
    await expect.poll(() => page.evaluate(() => window.__rustify_docs_islands?.stats()?.aborted)).toBe(true);
    await page.clock.runFor(100);
    await restored(page, 4);
    expect(await page.evaluate(() => window.__rustify_docs_islands?.stats())).toMatchObject({ tasks: 0, timers: 0, observers: 0, aborted: true });
    expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
    expect(audit.console.filter(message => !/^Interactive examples failed/.test(message.text))).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    expect(audit.failures).toEqual([]);
    expect(audit.responses).toEqual([]);
    expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
    await page.clock.resume();
});

test("the release proxy attaches every real category, then opens floating controls with focus return", async ({ page, audit }) => {
    const held = await openPaused(page, "proxy/");
    await expect(page.locator("[data-category]")).toHaveCount(24);
    held.release();
    await ready(page, 4);
    await identity(page);
    const category = (name: string) => page.locator(`[data-category="${name}"]`);
    await category("text field").getByRole("textbox", { name: "Editable text", exact: true }).fill("Attached text");
    await expect(category("text field").locator('[data-testid$="-text-result"]')).toHaveText("Attached text");
    await category("menu").getByRole("button", { name: "Open menu", exact: true }).click();
    await expect(page.getByRole("menu", { name: "Fixture actions" })).toBeVisible();
    await page.getByRole("menuitem", { name: "Copy", exact: true }).click();
    await expect(page.getByRole("menu", { name: "Fixture actions" })).toHaveCount(0);
    await expect(category("menu").getByRole("button", { name: "Open menu", exact: true })).toBeFocused();
    await category("dialog").getByRole("button", { name: "Open dialog", exact: true }).click();
    await expect(page.getByRole("dialog", { name: "Fixture dialog" })).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(page.getByRole("dialog", { name: "Fixture dialog" })).toHaveCount(0);
    await expect(category("dialog").getByRole("button", { name: "Open dialog", exact: true })).toBeFocused();
    const peers = page.locator('[data-counter]');
    await peers.first().getByTestId("counter-increment").click();
    await expect(peers.nth(0).getByTestId("counter-value")).toHaveText("3");
    await expect(peers.nth(1).getByTestId("counter-value")).toHaveText("2");
    await expect(peers.nth(2).getByTestId("counter-value")).toHaveText("2");
    await healthy(page, audit);
});

const proxyCategory = (page: Page, name: string) => page.locator(`[data-category="${name}"]`);

// Each step changes one control's state; the sample follows at once.
const proxySteps: [string, (page: Page) => Promise<void>][] = [
    ["hover button", page => proxyCategory(page, "button").locator('[data-name="Button"]').first().hover()],
    ["select tab", page => proxyCategory(page, "tabs").getByRole("tab", { name: "Detail", exact: true }).click()],
    ["toggle switch", page => proxyCategory(page, "switch").getByRole("switch").click()],
    ["check checkbox", page => proxyCategory(page, "checkbox").getByRole("checkbox").click()],
    ["choose radio", page => proxyCategory(page, "radio").getByRole("radio", { name: "South", exact: true }).click()],
    ["advance progress", page => proxyCategory(page, "progress").getByRole("button", { name: "Advance progress", exact: true }).click()],
    ["edit colour", async page => {
        const input = proxyCategory(page, "color field").getByRole("textbox");
        await input.fill("33669980");
        await input.press("Enter");
    }],
];

test("gaps reduced motion: under reduce the proxy's controls attach and change with no transition and a still spinner", async ({ page, audit }) => {
    await installMotionProbe(page);
    await page.emulateMedia({ reducedMotion: "reduce" });
    const held = await openPaused(page, "proxy/");
    const samples = [await sampleMotion(page, "loading")];
    held.release();
    await ready(page, 4);
    samples.push(await sampleMotion(page, "ready"));
    for (const [label, step] of proxySteps) {
        await step(page);
        samples.push(await sampleMotion(page, label));
    }
    await attachMotion(page, test.info(), "gaps-reduced-motion.json", samples);
    for (const target of Object.keys(motionTargets)) expect(samples[1].present[target], target).toBeGreaterThan(0);
    for (const sample of samples) expectStill(sample);
    await healthy(page, audit);
});

test("gaps reduced motion: without a preference the proxy keeps the SDK's durations and the spinner's animation", async ({ page, audit }) => {
    await installMotionProbe(page);
    await page.emulateMedia({ reducedMotion: "no-preference" });
    await page.goto(url("proxy/"));
    await ready(page, 4);
    const rest = await sampleMotion(page, "ready");
    await proxyCategory(page, "switch").getByRole("switch").click();
    const toggled = await sampleMotion(page, "toggle switch");
    await attachMotion(page, test.info(), "gaps-normal-motion.json", [rest, toggled]);
    for (const target of Object.keys(motionTargets)) expect(rest.present[target], target).toBeGreaterThan(0);
    expectNormal(rest);
    // Tailwind v4 moves the thumb with `translate`, which `transition-transform` covers.
    expect(toggled.transitions.filter(entry => entry.node.startsWith("span.") && entry.node.endsWith("in Switch") && entry.property === "translate").map(entry => entry.duration)).toEqual([150]);
    await healthy(page, audit);
});

for (const colorScheme of ["light", "dark"] as const) {
    for (const layout of [{ name: "375px", width: 375, scaleText: false }, { name: "200% reflow and text scaling", width: 640, scaleText: true }]) {
        test(`${layout.name} keeps loading, ready, failure and bounded source usable in ${colorScheme}`, async ({ page, audit }) => {
            await page.setViewportSize({ width: layout.width, height: 800 });
            await page.emulateMedia({ colorScheme, reducedMotion: "reduce" });
            const held = await openPaused(page, "failure/");
            if (layout.scaleText) {
                // The repository's zoom checks use half the CSS viewport and
                // twice the root text size, exercising both reflow and text.
                // Under reduce, scaling the text starts no transition at all;
                // the check comes before anything could wait one out.
                const transitions = await page.evaluate(async () => {
                    document.documentElement.style.fontSize = "200%";
                    const observed = document.getAnimations().filter((animation): animation is CSSTransition =>
                        animation instanceof CSSTransition && Number.isFinite(animation.effect?.getTiming().iterations))
                        .map(animation => ({ property: animation.transitionProperty, duration: animation.effect?.getTiming().duration }));
                    await new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
                    return observed;
                });
                await test.info().attach("font-scaling-transitions.json", { body: JSON.stringify(transitions, null, 2), contentType: "application/json" });
                expect(transitions).toEqual([]);
                await captureOriginals(page);
            }
            await page.evaluate(() => window.scrollTo(0, 0));
            await page.screenshot({ path: test.info().outputPath(`loading-${colorScheme}-${layout.width}.png`), fullPage: true });
            held.release();
            await ready(page, 2);
            await identity(page);
            const source = page.locator(".rd-demo-source");
            await source.getByRole("tab", { name: "Full file", exact: true }).click();
            const pre = source.locator('.rd-code:not([hidden]) .rd-code-pre');
            await pre.focus();
            await expect(pre).toBeFocused();
            const dimensions = await pre.evaluate(node => ({
                client: node.clientHeight, scroll: node.scrollHeight,
                limit: 24 * parseFloat(getComputedStyle(document.documentElement).fontSize),
            }));
            expect(dimensions.client).toBeLessThanOrEqual(dimensions.limit);
            expect(dimensions.scroll).toBeGreaterThan(dimensions.client);
            await page.getByTestId("failure-crash").click();
            await restored(page, 2);
            const status = await page.evaluate(() => window.__docs_island_probe!.blocks.map(snapshot => ({
                before: snapshot.statusHeight,
                after: document.querySelector(`[data-rustify-island="${snapshot.key}"] [data-rd-island-status]`)!.getBoundingClientRect().height,
                sourceBefore: snapshot.sourceTop,
                sourceAfter: snapshot.source ? snapshot.source.getBoundingClientRect().top + scrollY : null,
            })));
            for (const height of status) {
                expect(height.after).toBe(height.before);
                expect(height.sourceAfter).toBe(height.sourceBefore);
            }
            expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(0);
            await expect(source.getByRole("tab", { name: "Full file", exact: true })).toBeEnabled();
            await expect(islands(page).last().getByRole("button", { name: "Reload page", exact: true })).toBeInViewport();
            await page.evaluate(() => window.scrollTo(0, 0));
            await page.screenshot({ path: test.info().outputPath(`failure-${colorScheme}-${layout.width}.png`), fullPage: true });
            expect(audit.console.filter(message => !/^Interactive examples failed/.test(message.text) && !/docs fixture callback failure/.test(message.text))).toEqual([]);
            expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
            expect(audit.pageErrors.filter(error => !/^RuntimeError: unreachable$/.test(error))).toEqual([]);
            expect(audit.failures).toEqual([]);
            expect(audit.responses).toEqual([]);
            expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
        });
    }
}

for (const colorScheme of ["light", "dark"] as const) for (const width of [1280, 375]) {
    test(`gaps a11y: loading, ready, client-only and failed islands at rest have no serious or critical axe violation, ${colorScheme} ${width}px`, async ({ page, audit }) => {
        await page.setViewportSize({ width, height: 900 }); await page.emulateMedia({ colorScheme, reducedMotion: "no-preference" });
        const scans: Scan[] = [];
        const loading = page.locator('[data-rustify-island][data-rd-island-state="loading"]');
        let held = await openPaused(page, "counter/");
        await atRest(page);
        await expect(loading).toHaveCount(4);
        scans.push(await scan(page, "counter/", "loading"));
        held.release();
        await ready(page, 4);
        await atRest(page);
        await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(4);
        scans.push(await scan(page, "counter/", "ready"));
        await held.remove();

        held = await openPaused(page, "client-only/");
        await atRest(page);
        await expect(loading).toHaveCount(1);
        await expect(page.locator("[data-rustify-client-message]")).toBeVisible();
        scans.push(await scan(page, "client-only/", "client-only placeholder"));
        held.release();
        await ready(page, 1);
        await atRest(page);
        await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(1);
        scans.push(await scan(page, "client-only/", "ready"));
        await held.remove();

        // A real callback trap: every island returns to its initial view, failed.
        held = await openPaused(page, "failure/");
        held.release();
        await ready(page, 2);
        await page.getByTestId("failure-crash").click();
        await restored(page, 2);
        await atRest(page);
        await expect(page.locator('[data-rustify-island][data-rd-island-state="failed"]')).toHaveCount(2);
        scans.push(await scan(page, "failure/", "failed"));
        await held.remove();

        await attachScans(test.info(), `gaps-a11y-islands-${colorScheme}-${width}.json`, scans, await buildId(page, url("./")));
        expect(serious(scans)).toEqual([]);
        expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
        expect(audit.failures).toEqual([]);
        expect(audit.responses).toEqual([]);
        expect(audit.pageErrors.filter(error => !/^RuntimeError: unreachable$/.test(error))).toEqual([]);
        expect(audit.console.filter(message => !/^Interactive examples failed/.test(message.text) && !/docs fixture callback failure/.test(message.text))).toEqual([]);
        expect(audit.requests.filter(request => new URL(request).origin !== new URL(url("./")).origin)).toEqual([]);
    });
}
