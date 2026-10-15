import { CDPSession, expect, Locator, Page, test as playwright } from "@playwright/test";
import { createHash } from "node:crypto";
import { readFileSync, readdirSync, statSync } from "node:fs";
import { atRest, attachScans, buildId, scan, Scan, serious } from "./a11y";
import { capture, firstFontFamily, grantClipboard } from "./browser";

type ThemeMode = "light" | "dark";
type FontResponse = { url: string; mime: string; bytes: Buffer; sha256: string };
type Audit = { requests: string[]; errors: string[]; fonts: FontResponse[]; pending: Promise<void>[]; document404: Set<string>; expectedConsole: string[]; detach(): Promise<void> };
type Original = { key: string; root: Element; nodes: Node[]; ids: [Element, string][]; source: Element; sourceTop: number; height: number };

declare global {
    interface Window {
        __custom_csp: string[];
        __custom_originals: Original[];
    }
}

const theme = JSON.parse(readFileSync("tests/docs-ssg/fixtures/components/theme.json", "utf8"));
const route = (value: string) => new URL(value, String(test.info().project.use.baseURL)).href;
const base = () => new URL(route("./")).pathname;
const islands = (page: Page) => page.locator("[data-rustify-island]");
const rgb = (hex: string) => `rgb(${[1, 3, 5].map(offset => parseInt(hex.slice(offset, offset + 2), 16)).join(", ")})`;
const slots = ["header", "footer", "sidebar_top", "sidebar_bottom", "before_content", "after_content"];
const sample = "Hello — résumé, 中文, 🙂";

async function watch(page: Page): Promise<Audit> {
    let capture: CDPSession | undefined;
    const audit: Audit = { requests: [], errors: [], fonts: [], pending: [], document404: new Set(), expectedConsole: [], detach: async () => {
        await Promise.all(audit.pending);
        if (capture) { await capture.detach(); capture = undefined; }
    } };
    const chromium = page.context().browser()?.browserType().name() === "chromium";
    if (chromium) {
        const directory = "crates/rustify-docs-cli/assets/fonts";
        const sizes = readdirSync(directory).filter(file => file.endsWith(".ttf")).map(file => statSync(`${directory}/${file}`).size);
        const cdp = capture = await page.context().newCDPSession(page);
        // Own response retention, with measured font bytes plus the fixture's
        // 11 MiB wasm and sub-1 MiB document. HTTP caching is left unchanged.
        await cdp.send("Network.enable", {
            maxResourceBufferSize: Math.max(...sizes) * 2 + 1024 * 1024,
            maxTotalBufferSize: sizes.reduce((sum, size) => sum + size, 0) * 2 + 24 * 1024 * 1024,
        });
        const responses = new Map<string, { url: string; mime: string }>();
        cdp.on("Network.responseReceived", ({ requestId, response }) => {
            if (/\/assets\/fonts\/[^/]+\.ttf$/.test(new URL(response.url).pathname)) responses.set(requestId, { url: response.url, mime: response.mimeType });
        });
        cdp.on("Network.loadingFinished", ({ requestId }) => {
            const response = responses.get(requestId);
            if (!response) return;
            responses.delete(requestId);
            audit.pending.push(cdp.send("Network.getResponseBody", { requestId }).then(body => {
                const bytes = Buffer.from(body.body, body.base64Encoded ? "base64" : "utf8");
                audit.fonts.push({ ...response, bytes, sha256: createHash("sha256").update(bytes).digest("hex") });
            }, error => { audit.errors.push(`${response.url} ${String(error)}`); }));
        });
    }
    await page.addInitScript(() => {
        window.__custom_csp = [];
        document.addEventListener("securitypolicyviolation", event => window.__custom_csp.push(`${event.violatedDirective} ${event.blockedURI}`));
    });
    page.on("request", request => audit.requests.push(request.url()));
    page.on("requestfailed", request => audit.errors.push(`${request.url()} ${request.failure()?.errorText}`));
    page.on("console", message => {
        if (message.type() !== "error") return;
        const url = message.location().url;
        if (audit.document404.has(url) && message.text() === "Failed to load resource: the server responded with a status of 404 (Not Found)") {
            audit.expectedConsole.push(`${url}: ${message.text()}`);
        } else audit.errors.push(message.text());
    });
    page.on("pageerror", error => audit.errors.push(String(error)));
    page.on("response", response => {
        if (response.status() >= 400 && response.request().resourceType() !== "document") audit.errors.push(`${response.url()} ${response.status()}`);
        if (!chromium && /\/assets\/fonts\/[^/]+\.ttf$/.test(new URL(response.url()).pathname)) {
            audit.pending.push(response.body().then(bytes => {
                audit.fonts.push({ url: response.url(), mime: response.headers()["content-type"], bytes, sha256: createHash("sha256").update(bytes).digest("hex") });
            }, error => { audit.errors.push(`${response.url()} ${String(error)}`); }));
        }
    });
    return audit;
}

const test = playwright.extend<{ audit: Audit }>({
    audit: [async ({ page }, use, info) => {
        const audit = await watch(page);
        try {
            await use(audit);
            await flushFonts(page, audit);
            await info.attach("custom-browser-audit.json", {
                body: JSON.stringify({ requests: audit.requests, errors: audit.errors, expectedConsole: audit.expectedConsole, fonts: audit.fonts.map(({ bytes, ...font }) => ({ ...font, bytes: bytes.length })), csp: await page.evaluate(() => window.__custom_csp) }, null, 2),
                contentType: "application/json",
            });
        } finally { await audit.detach(); }
    }, { auto: true }],
});

async function flushFonts(page: Page, audit: Audit) {
    await page.evaluate(() => document.fonts.ready);
    await Promise.all(audit.pending);
}

async function healthy(page: Page, audit: Audit) {
    await flushFonts(page, audit);
    expect(audit.errors).toEqual([]);
    expect(await page.evaluate(() => window.__custom_csp)).toEqual([]);
    expect(audit.requests.filter(request => /makepad/i.test(request))).toEqual([]);
    expect(audit.requests.filter(request => new URL(request).origin !== new URL(route("./")).origin)).toEqual([]);
}

async function ready(page: Page, count = 4) {
    await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(count);
    for (const island of await islands(page).all()) {
        const metadata = JSON.parse((await island.getAttribute("data-rustify-island-metadata"))!);
        expect(metadata.environment.theme.document).toEqual(theme);
        await expect(island.locator("fieldset[data-rd-island-controls]")).toHaveJSProperty("inert", false);
        await expect(island.locator("fieldset[data-rd-island-controls] > [data-rustify-scope]")).toHaveAttribute("data-rustify-mounted", metadata.seed);
    }
}

async function contextSlots(page: Page, contextRoute: string, missing = false) {
    for (const name of [...slots, ...(missing ? ["not_found"] : [])]) {
        const slot = page.locator(`[data-slot="${name}"]`);
        await expect(slot).toHaveCount(1);
        await expect(slot).toHaveAttribute("data-context-route", contextRoute);
        await expect(slot).toHaveAttribute("data-context-lang", "en");
        await expect(slot).toHaveAttribute("data-context-base", base());
    }
    if (!missing) await expect(page.locator('[data-slot="not_found"]')).toHaveCount(0);
    await expect(page.getByRole("banner").locator('[data-slot="header"]')).toHaveText("Reading laboratory");
    await expect(page.getByRole("contentinfo").locator('[data-slot="footer"]')).toHaveText("Keep reading");
    await expect(page.locator('.rd-sidebar [data-slot="sidebar_top"]')).toHaveCount(1);
    await expect(page.locator('.rd-sidebar [data-slot="sidebar_bottom"]')).toHaveCount(1);
    await expect(page.locator("[data-rd-mode-switch]")).toBeVisible();
    await expect(page.locator(".rd-header-nav-wide").getByRole("link", { name: "Counters", exact: true })).toHaveAttribute("href", new URL(route("counter/")).pathname);
}

async function pausedCustom(page: Page) {
    let release!: () => void;
    let observed!: () => void;
    const released = new Promise<void>(resolve => { release = resolve; });
    const seen = new Promise<void>(resolve => { observed = resolve; });
    await page.route("**/assets/islands/**/app_bg.wasm", async request => {
        observed();
        await released;
        await request.continue();
    });
    await page.goto(route("custom/"), { waitUntil: "commit" });
    await seen;
    await expect(page.locator('[data-rustify-island][data-rd-island-state="loading"]')).toHaveCount(4);
    await page.evaluate(() => document.fonts.ready);
    await page.evaluate(() => {
        window.__custom_originals = [...document.querySelectorAll("[data-rustify-island]")].map(block => {
            const root = block.querySelector("fieldset[data-rd-island-controls]")!.firstElementChild!;
            const walker = document.createTreeWalker(root, NodeFilter.SHOW_ALL);
            const nodes: Node[] = [root];
            while (walker.nextNode()) nodes.push(walker.currentNode);
            const source = block.querySelector(".rd-demo-source")!;
            return { key: block.getAttribute("data-rustify-island")!, root, nodes, ids: [...root.querySelectorAll("[id]")].map(node => [node, node.id] as [Element, string]), source, sourceTop: source.getBoundingClientRect().top + scrollY, height: block.querySelector("[data-rd-island-status]")!.getBoundingClientRect().height };
        });
    });
    return release;
}

async function retainOriginals(page: Page) {
    const result = await page.evaluate(() => window.__custom_originals.map(original => {
        const block = [...document.querySelectorAll("[data-rustify-island]")].find(block => block.getAttribute("data-rustify-island") === original.key)!;
        return { key: original.key, count: original.nodes.length, retained: original.nodes.filter(node => node.isConnected && original.root.contains(node)).length, root: block.querySelector("fieldset[data-rd-island-controls]")!.firstElementChild === original.root, ids: original.ids.every(([node, id]) => node.id === id), source: block.querySelector(".rd-demo-source") === original.source, sourceTop: original.source.getBoundingClientRect().top + scrollY, beforeTop: original.sourceTop, height: block.querySelector("[data-rd-island-status]")!.getBoundingClientRect().height, beforeHeight: original.height };
    }));
    await test.info().attach("custom-island-node-identity.json", { body: JSON.stringify(result, null, 2), contentType: "application/json" });
    for (const item of result) {
        expect(item.retained, item.key).toBe(item.count);
        expect(item.root && item.ids && item.source, item.key).toBe(true);
        expect(item.sourceTop, item.key).toBe(item.beforeTop);
        expect(item.height, item.key).toBe(item.beforeHeight);
    }
    expect(await page.evaluate(() => {
        const ids = [...document.querySelectorAll("[id]")].map(node => node.id);
        return ids.length === new Set(ids).size;
    })).toBe(true);
}

async function sourceText(source: Locator, file: string, name: string) {
    const original = readFileSync(file, "utf8");
    const fn = original.match(new RegExp(`(?:\\/\\/\\/[^\\r\\n]*\\r?\\n)*pub fn ${name}\\([^)]*\\)[\\s\\S]*?^\\}(?:\\r?\\n|$)`, "m"))?.[0];
    expect(fn).toBeTruthy();
    expect(await source.locator("template.rd-code-raw").evaluateAll(nodes => nodes.map(node => (node as HTMLTemplateElement).content.textContent))).toEqual([fn, original]);
}

async function themeValues(page: Page, mode: ThemeMode) {
    const values = theme.styles[mode];
    await expect(page.locator("body")).toHaveCSS("background-color", rgb(values.background));
    await expect(page.locator("body")).toHaveCSS("color", rgb(values.foreground));
    const surface = page.locator("[data-theme-surface]");
    await expect(surface).toHaveCSS("background-color", rgb(values.card));
    await expect(surface).toHaveCSS("color", rgb(values["card-foreground"]));
    await expect(surface).toHaveCSS("border-top-left-radius", `${parseFloat(values.radius) - 2}px`);
    for (const [slot, family] of [["sans", "Noto Sans"], ["serif", "Noto Serif"], ["mono", "JetBrains Mono"]]) {
        expect(await page.locator(`[data-font-${slot}]`).evaluate(firstFontFamily)).toBe(family);
    }
}

test("registered routes keep declared titles, optional description and ordered ordinary navigation", async ({ page, audit }) => {
    await page.goto(route("custom/"));
    await ready(page);
    await expect(page).toHaveTitle("Custom page | Component fixture");
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("Custom page");
    await expect(page.locator('meta[name="description"]')).not.toHaveAttribute("content", "");
    const navigation = page.locator(".rd-sidebar-wide");
    const custom = navigation.getByRole("link", { name: "Custom page", exact: true });
    const plain = navigation.getByRole("link", { name: "Static custom page", exact: true });
    expect(await custom.evaluate((node, href) => [...node.closest("nav")!.querySelectorAll("a")].filter(link => [node.getAttribute("href"), href].includes(link.getAttribute("href"))).map(link => link.textContent), new URL(route("plain/")).pathname)).toEqual(["Custom page", "Static custom page"]);
    await expect(custom).toHaveAttribute("aria-current", "page");
    await page.getByRole("link", { name: "Jump to details", exact: true }).click();
    await expect(page.locator("#custom-details")).toBeInViewport();
    await flushFonts(page, audit);
    await plain.click();
    await expect(page).toHaveURL(route("plain/"));
    await expect(page).toHaveTitle("Static custom page | Component fixture");
    await expect(page.locator('meta[name="description"]')).toHaveAttribute("content", "");
    await expect(islands(page)).toHaveCount(0);
    await flushFonts(page, audit);
    await page.getByRole("link", { name: "Custom page anchor", exact: true }).click();
    await expect(page).toHaveURL(route("custom/#custom-details"));
    await expect(page.locator("#custom-details")).toBeInViewport();
    await flushFonts(page, audit);
    await page.getByRole("link", { name: "Registered counter anchor", exact: true }).click();
    await expect(page).toHaveURL(route("counter/#three-independent-instances"));
    await expect(page.locator("#three-independent-instances")).toBeInViewport();
    await healthy(page, audit);
});

test("all seven slots share canonical context across Markdown, custom pages and a deep static 404", async ({ page, audit }) => {
    for (const [path, context, count] of [["counter/", "/counter/", 4], ["custom/", "/custom/", 4], ["plain/", "/plain/", 0], ["missing/a/b/", "/404/", 0]] as const) {
        await flushFonts(page, audit);
        if (context === "/404/") audit.document404.add(route(path));
        const response = await page.goto(route(path));
        expect(response!.status()).toBe(context === "/404/" ? 404 : 200);
        await contextSlots(page, context, context === "/404/");
        await expect(islands(page)).toHaveCount(count);
        if (count) await ready(page, count);
        await expect(page.getByRole("banner")).toHaveCount(1);
        await expect(page.getByRole("main")).toHaveCount(1);
        await expect(page.getByRole("contentinfo")).toHaveCount(1);
        await expect(page.locator('[data-slot="before_content"] [data-rustify-island]')).toHaveCount(context === "/custom/" ? 1 : 0);
        if (context === "/404/") {
            await expect(page.getByRole("heading", { level: 1 })).toHaveText("Page not found");
            await flushFonts(page, audit);
            await capture(page, { path: test.info().outputPath("custom-deep-404.png"), fullPage: true });
            await page.getByRole("link", { name: "return home", exact: true }).click();
            await expect(page).toHaveURL(route("./"));
        }
    }
    await healthy(page, audit);
});

test("nested page and slot demos retain actual SSR nodes, source and distinct identities", async ({ page, context, audit }) => {
    await grantClipboard(context);
    const release = await pausedCustom(page);
    const metadata = await islands(page).evaluateAll(nodes => nodes.map(node => JSON.parse(node.getAttribute("data-rustify-island-metadata")!)));
    expect(metadata.map(item => item.target.name)).toEqual(["CounterDemo", "CounterDemo", "CounterDemo", "ThemeProbeDemo"]);
    expect(new Set(metadata.map(item => item.key)).size).toBe(4);
    expect(new Set(metadata.map(item => item.seed)).size).toBe(4);
    await expect(page.getByTestId("counter-value")).toHaveText(["7", "7", "7"]);
    await expect(page.locator("[style]")).toHaveCount(0);
    release();
    await ready(page);
    await retainOriginals(page);
    const counters = islands(page).filter({ has: page.locator("[data-counter]") });
    await counters.nth(0).getByTestId("counter-increment").click();
    await counters.nth(1).getByTestId("counter-input").fill("21");
    await expect(page.getByTestId("counter-value")).toHaveText(["8", "21", "7"]);
    for (const island of await counters.all()) {
        await sourceText(island.locator(".rd-demo-source"), "tests/docs-ssg/fixtures/components/src/demos.rs", "counter_demo");
        await island.getByRole("tab", { name: "Full file", exact: true }).click();
        await island.locator(".rd-code-tabs > [data-rd-copy]").click();
        expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(readFileSync("tests/docs-ssg/fixtures/components/src/demos.rs", "utf8"));
    }
    const probe = islands(page).filter({ has: page.locator("[data-theme-probe]") });
    await sourceText(probe.locator(".rd-demo-source"), "tests/docs-ssg/fixtures/components/src/custom.rs", "theme_probe_demo");
    await expect(page.locator("[data-rustify-scope] [style]")).toHaveCount(0);
    await healthy(page, audit);
});

for (const [scheme, preference, effective] of [["light", "light", "light"], ["dark", "dark", "dark"], ["light", "system", "light"], ["dark", "system", "dark"]] as const) {
    test(`${preference} first frame uses the document theme with a ${scheme} system before wasm`, async ({ page, audit }) => {
        await page.emulateMedia({ colorScheme: scheme });
        await page.addInitScript(({ base, preference }) => localStorage.setItem(`rustify-docs:mode:${base}`, preference), { base: base(), preference });
        const release = await pausedCustom(page);
        await expect(page.locator("html")).toHaveAttribute("data-theme", preference);
        await themeValues(page, effective);
        await capture(page, { path: test.info().outputPath(`first-frame-${preference}-${scheme}.png`), fullPage: true });
        release();
        await ready(page);
        await retainOriginals(page);
        await themeValues(page, effective);
        await expect(page.getByTestId("theme-probe-context")).toHaveAttribute("data-theme-document", theme.id);
        await expect(page.getByTestId("theme-probe-context")).toHaveAttribute("data-theme-mode", effective);
        await expect(page.getByTestId("theme-probe-context")).toHaveAttribute("data-theme-radius", String(parseFloat(theme.styles[effective].radius)));
        await expect(page.getByTestId("counter-value")).toHaveText(["7", "7", "7"]);
        await healthy(page, audit);
    });
}

test("live mode changes reach the theme context and real floating controls, including Rust-only classes", async ({ page, audit }) => {
    await page.goto(route("custom/"));
    await ready(page);
    await expect(page.getByTestId("theme-probe-future")).toHaveCount(0);
    expect(await page.locator("[class]").evaluateAll(nodes => nodes.some(node => node.classList.contains("w-[73px]") || node.classList.contains("bg-emerald-600")))).toBe(false);
    for (const mode of ["light", "dark"] as const) {
        await page.locator(`[data-rd-mode="${mode}"]`).click();
        await themeValues(page, mode);
        await expect(page.getByTestId("theme-probe-context")).toHaveAttribute("data-theme-mode", mode);
        await page.getByTestId("theme-probe-menu-trigger").focus();
        await page.getByTestId("theme-probe-menu-trigger").press("Enter");
        const menu = page.getByTestId("theme-probe-menu");
        await expect(menu).toHaveCSS("background-color", rgb(theme.styles[mode].popover));
        await expect(menu).toHaveCSS("border-top-left-radius", `${parseFloat(theme.styles[mode].radius) - 2}px`);
        expect(await menu.evaluate(firstFontFamily)).toBe("Noto Sans");
        await menu.getByRole("menuitem", { name: "Choose résumé 中文 🙂", exact: true }).press("Enter");
        await expect(page.getByTestId("theme-probe-menu-value")).toHaveText("choose");
        await expect(page.getByTestId("theme-probe-menu-trigger")).toBeFocused();
        await page.getByTestId("theme-probe-dialog-trigger").focus();
        await page.getByTestId("theme-probe-dialog-trigger").press("Enter");
        const dialog = page.getByRole("dialog", { name: "Theme probe dialog", exact: true });
        const panel = dialog.locator('[data-name="Dialog"]');
        await expect(panel).toHaveCSS("background-color", rgb(theme.styles[mode].popover));
        await expect(panel).toHaveCSS("color", rgb(theme.styles[mode]["popover-foreground"]));
        await expect(panel).toHaveCSS("border-top-left-radius", theme.styles[mode].radius);
        expect(await panel.evaluate(firstFontFamily)).toBe("Noto Sans");
        await page.keyboard.press("Escape");
        await expect(dialog).toHaveCount(0);
        await expect(page.getByTestId("theme-probe-dialog-trigger")).toBeFocused();
    }
    await page.getByTestId("theme-probe-reveal").click();
    const future = page.getByTestId("theme-probe-future");
    await expect(future).toHaveCSS("width", "73px");
    await expect(future).toHaveCSS("height", "31px");
    await expect(future).toHaveCSS("border-top-left-radius", "22px");
    await expect(future).toHaveClass(/bg-\[url\(static\/future\.svg\)\]/);
    const background = await future.evaluate(node => getComputedStyle(node).backgroundImage);
    const source = readFileSync("tests/docs-ssg/fixtures/components/static/future.svg");
    const resource = new URL(`assets/files/future.${createHash("sha256").update(source).digest("hex").slice(0, 10)}.svg`, route("./"));
    expect(background).toContain(resource.href);
    expect((await (await page.request.get(resource.href)).body()).equals(source)).toBe(true);
    expect(await future.evaluate(node => {
        const canvas = document.createElement("canvas");
        canvas.width = canvas.height = 1;
        const context = canvas.getContext("2d")!;
        const pixel = (color: string) => {
            context.clearRect(0, 0, 1, 1);
            context.fillStyle = color;
            context.fillRect(0, 0, 1, 1);
            return [...context.getImageData(0, 0, 1, 1).data];
        };
        return JSON.stringify(pixel(getComputedStyle(node).backgroundColor)) === JSON.stringify(pixel("oklch(59.6% 0.145 163.225)"));
    })).toBe(true);
    await expect(page.locator("[style]")).toHaveCount(0);
    await healthy(page, audit);
});

test("the actual browser font responses match the CLI's packaged bytes and neutral digests", async ({ page, audit }) => {
    await page.goto(route("custom/"));
    await ready(page);
    await page.evaluate(async sample => {
        await Promise.all(["Noto Sans", "Noto Serif", "JetBrains Mono"].map(family => document.fonts.load(`16px "${family}"`, sample)));
        await document.fonts.ready;
    }, sample);
    await Promise.all(audit.pending);
    const filenames = ["NotoSans-Regular.ttf", "NotoSerif-Regular.ttf", "jetbrains_mono_variable.ttf", "LXGWWenKaiRegular.ttf", "NotoColorEmoji.ttf"];
    const expected = new Map(filenames.map(file => {
        const bytes = readFileSync(`crates/rustify-docs-cli/assets/fonts/${file}`);
        return [createHash("sha256").update(bytes).digest("hex"), { file, bytes }] as const;
    }));
    const observed = new Set<string>();
    for (const font of audit.fonts) {
        const packaged = expected.get(font.sha256);
        expect(packaged, font.url).toBeDefined();
        expect(font.bytes.equals(packaged!.bytes), font.url).toBe(true);
        expect(new URL(font.url).pathname).toBe(`${base()}assets/fonts/${font.sha256}.ttf`);
        expect(font.mime.split(";")[0]).toBe("font/ttf");
        observed.add(font.sha256);
    }
    expect([...observed].sort()).toEqual([...expected.keys()].sort());
    for (const stylesheet of await page.locator('link[rel="stylesheet"]').evaluateAll(nodes => nodes.map(node => (node as HTMLLinkElement).href))) {
        const css = await (await page.request.get(stylesheet)).text();
        expect(css).not.toContain("makepad_widgets/resources");
    }
    await healthy(page, audit);
});

test("custom images and CSS backgrounds enter the same published resource graph", async ({ page, audit }) => {
    await page.goto(route("custom/"));
    await ready(page);
    const image = page.locator("[data-custom-image]");
    await expect(image).toHaveJSProperty("complete", true);
    await expect(image).toHaveJSProperty("naturalWidth", 120);
    const resource = (await image.getAttribute("src"))!;
    expect(resource).toMatch(new RegExp(`^${base()}assets/files/[^/]+\\.svg$`));
    const source = readFileSync("tests/docs-ssg/fixtures/components/static/custom.svg");
    expect((await (await page.request.get(resource)).body()).equals(source)).toBe(true);
    const background = await page.locator("[data-custom-background]").evaluate(node => getComputedStyle(node).backgroundImage);
    expect(background).toContain(new URL(resource, route("./")).href);
    await healthy(page, audit);
});

test("the plain custom page and its slots remain static and load no island resources", async ({ page, audit }) => {
    await page.goto(route("plain/"));
    await contextSlots(page, "/plain/");
    await expect(islands(page)).toHaveCount(0);
    expect(audit.requests.filter(request => /\/assets\/islands\//.test(request) || request.endsWith(".wasm"))).toEqual([]);
    await healthy(page, audit);
});

test("no script preserves custom views, sources, navigation and theme at 375px and 200% reflow", async ({ browser }) => {
    // 640 CSS pixels at DPR 2 emulate a 1280-pixel window at 200% layout zoom.
    // Scale every CSS pixel, including author controls with explicit px sizes.
    let controlFontPixels = 0;
    for (const [width, height, scale] of [[375, 800, 1], [640, 400, 2]] as const) {
        const context = await browser.newContext({ javaScriptEnabled: false, colorScheme: "dark", viewport: { width, height }, deviceScaleFactor: scale });
        const page = await context.newPage();
        const audit = await watch(page);
        try {
            await page.goto(route("custom/"));
            await page.evaluate(() => document.fonts.ready);
            await expect(islands(page)).toHaveCount(4);
            await expect(page.getByTestId("counter-value")).toHaveText(["7", "7", "7"]);
            for (const island of await islands(page).all()) {
                await expect(island.locator("fieldset")).toHaveJSProperty("disabled", true);
                await expect(island.locator("noscript > span")).toHaveText("Enable JavaScript to interact");
                for (const button of await island.locator("fieldset button").all()) await expect(button).toBeDisabled();
                for (const panel of await island.locator(".rd-code").all()) await expect(panel).toBeVisible();
            }
            await expect(page.getByTestId("theme-probe-future")).toHaveCount(0);
            await expect(page.getByRole("dialog")).toHaveCount(0);
            await expect(page.getByRole("menu")).toHaveCount(0);
            await expect(page.locator("[data-font-sans]")).toHaveCSS("background-color", rgb(theme.styles.dark.card));
            expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(0);
            const zoom = await page.evaluate(() => ({
                viewport: innerWidth, dpr: devicePixelRatio,
                bodyFont: getComputedStyle(document.body).fontSize,
                controlFont: getComputedStyle(document.querySelector('[data-testid="counter-input"]')!).fontSize,
            }));
            expect(zoom.viewport).toBe(width);
            expect(zoom.dpr).toBe(scale);
            if (scale === 1) controlFontPixels = parseFloat(zoom.controlFont);
            expect(controlFontPixels).toBeGreaterThan(0);
            expect(parseFloat(zoom.controlFont) * scale).toBe(controlFontPixels * scale);
            await test.info().attach(`custom-no-script-zoom-${width}.json`, { body: JSON.stringify({ ...zoom, kind: "layout zoom emulation", physicalControlFontPixels: parseFloat(zoom.controlFont) * scale }), contentType: "application/json" });
            await capture(page, { path: test.info().outputPath(`custom-no-script-${width}.png`), fullPage: true });
            await flushFonts(page, audit);
            await page.locator("[data-custom-page]").getByRole("link", { name: "Static custom page", exact: true }).click();
            await expect(page).toHaveURL(route("plain/#plain-details"));
            await expect(islands(page)).toHaveCount(0);
            expect(audit.requests.filter(request => /\/assets\/islands\//.test(request) || request.endsWith(".wasm"))).toEqual([]);
            await flushFonts(page, audit);
            expect(audit.errors).toEqual([]);
            await test.info().attach(`custom-no-script-audit-${width}.json`, { body: JSON.stringify({ requests: audit.requests, errors: audit.errors }, null, 2), contentType: "application/json" });
        } finally {
            await audit.detach();
            await context.close();
        }
    }
});

for (const colorScheme of ["light", "dark"] as const) for (const width of [1280, 375]) {
    test(`gaps a11y: the custom page, the plain page and a deep 404 at rest have no serious or critical axe violation, ${colorScheme} ${width}px`, async ({ page, audit }) => {
        await page.setViewportSize({ width, height: 900 }); await page.emulateMedia({ colorScheme, reducedMotion: "no-preference" });
        const scans: Scan[] = [];
        for (const [path, count, state] of [["custom/", 4, "ready"], ["plain/", 0, "static"], ["missing/a/b/", 0, "static 404"]] as const) {
            const missing = path.startsWith("missing/");
            if (missing) audit.document404.add(route(path));
            const response = await page.goto(route(path));
            expect(response!.status()).toBe(missing ? 404 : 200);
            await expect(islands(page)).toHaveCount(count);
            if (count) await ready(page, count);
            await atRest(page);
            if (count) await ready(page, count);
            scans.push(await scan(page, path, state));
        }
        await attachScans(test.info(), `gaps-a11y-custom-${colorScheme}-${width}.json`, scans, await buildId(page, route("./")));
        expect(serious(scans)).toEqual([]);
        await healthy(page, audit);
    });
}
