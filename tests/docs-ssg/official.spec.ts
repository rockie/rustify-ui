import { expect, Locator, Page, test as playwright } from "@playwright/test";
import { readFileSync } from "node:fs";
import path from "node:path";
import { attachMotion, expectNormal, expectStill, installMotionProbe, MotionSample, motionTargets, sampleMotion, seconds } from "./motion";
import { atRest, attachScans, buildId, scan, Scan, serious } from "./a11y";

// The official source, without a copied list of categories or pages. These
// assertions use the same files that root/subpath/meta builds must consume.
const config = readFileSync("rustify-docs.toml", "utf8");
const registry = readFileSync("site/src/lib.rs", "utf8");
const catalog = readFileSync("crates/rustify-components/src/catalog.rs", "utf8");
const names = new Map([...catalog.matchAll(/Self::(\w+) => "([^"]+)"/g)].map(match => [match[1], match[2]]));
const variants = [...catalog.slice(catalog.indexOf("pub const CATALOG:")).matchAll(/category: Category::(\w+)/g)].map(match => match[1]);
const declaredPages = [...registry.matchAll(/\w+ \{ route: "([^"]+)", title: "([^"]+)"/g)].map(match => ({ route: match[1], title: match[2] }));
const declarations = [...registry.matchAll(/(\w+Demo) => crate::demos::(\w+)::(\w+)/g)].map(match => ({ name: match[1], module: match[2], function: match[3] }));
const categories = variants.map(variant => {
    const name = names.get(variant)!;
    const route = `components/${name.replaceAll(" ", "-")}/`;
    const page = declaredPages.find(page => page.route === `/${route}`)!;
    const demo = declarations.find(demo => demo.name === `${variant}Demo`)!;
    if (!name || !page || !demo) throw new Error(`CATALOG ${variant} is missing its official page/demo`);
    const file = `site/src/demos/${demo.module}.rs`;
    const source = readFileSync(file, "utf8");
    const fn = source.match(new RegExp(`(?:\\/\\/\\/[^\\r\\n]*\\r?\\n)*pub fn ${demo.function}\\([^)]*\\)[\\s\\S]*?^\\}(?:\\r?\\n|$)`, "m"))?.[0];
    if (!fn) throw new Error(`missing source function ${demo.function} in ${file}`);
    return { name, route, title: page.title, demo: demo.name, file, source, function: fn };
});
const include = config.match(/\[content\][\s\S]*?include\s*=\s*\[([\s\S]*?)\]/)?.[1];
if (!include) throw new Error("official content.include must be explicit");
const guides = [...include.matchAll(/"([^"]+\.md)"/g)].map(match => {
    const file = `docs/${match[1]}`;
    const source = readFileSync(file, "utf8");
    const title = source.match(/^# ([^\r\n]+)$/m)?.[1];
    if (!title) throw new Error(`guide ${file} has no source H1`);
    return { file, route: `${path.basename(match[1], ".md")}/`, title };
});
const componentRoutes = config.match(/label = "Components"\s+routes = \[([\s\S]*?)\]/)?.[1];
const guideItems = config.match(/label = "Guides"\s+items = \[([\s\S]*?)\]/)?.[1];
if (!componentRoutes || !guideItems) throw new Error("official navigation groups are missing");
const configuredRoutes = [...componentRoutes.matchAll(/"([^"]+)"/g)].map(match => match[1]);
const configuredGuides = [...guideItems.matchAll(/"([^"]+)"/g)].map(match => `docs/${match[1]}`);
const readme = readFileSync("README.md", "utf8");
const introduction = readme.slice(readme.indexOf("\n", readme.indexOf("# ")) + 1, readme.indexOf("\n## Quick start")).trim();
const introParagraphs = introduction.split(/\n\s*\n/).map(paragraph => paragraph
    .replace(/\[([^\]]+)\]\([^)]*\)/g, "$1").replace(/`([^`]+)`/g, "$1").replace(/\s+/g, " ").trim());
const repoQuickstart = "https://github.com/rockie/rustify-ui/blob/main/README.md#quick-start";

type Audit = { requests: string[]; failures: string[]; console: string[]; pageErrors: string[]; csp: string[]; responses: { url: string; status: number; mime: string }[]; expected404: Set<string>; expectedConsole: string[] };
declare global { interface Window { __official_csp(value: string): void; __official_first_theme?: { mode: string | null; background: string }; } }

async function watch(page: Page): Promise<Audit> {
    const audit: Audit = { requests: [], failures: [], console: [], pageErrors: [], csp: [], responses: [], expected404: new Set(), expectedConsole: [] };
    await page.exposeBinding("__official_csp", (_, value: string) => audit.csp.push(value));
    await page.addInitScript(() => document.addEventListener("securitypolicyviolation", event => window.__official_csp(`${event.violatedDirective} ${event.blockedURI}`)));
    page.on("request", request => audit.requests.push(request.url()));
    page.on("requestfailed", request => audit.failures.push(`${request.url()} ${request.failure()?.errorText}`));
    page.on("pageerror", error => audit.pageErrors.push(String(error)));
    page.on("response", response => audit.responses.push({ url: response.url(), status: response.status(), mime: response.headers()["content-type"] || "" }));
    page.on("console", message => {
        if (message.type() !== "error") return;
        if (audit.expected404.has(message.location().url) && message.text() === "Failed to load resource: the server responded with a status of 404 (Not Found)") audit.expectedConsole.push(message.text());
        else audit.console.push(message.text());
    });
    return audit;
}

const test = playwright.extend<{ audit: Audit }>({ audit: [async ({ page }, use, info) => {
    const audit = await watch(page);
    try { await use(audit); }
    finally { await info.attach("official-browser-audit.json", { body: JSON.stringify({ ...audit, expected404: [...audit.expected404] }, null, 2), contentType: "application/json" }); }
}, { auto: true }] });

const url = (route: string) => new URL(route, String(test.info().project.use.baseURL)).href;
const base = () => new URL(url("./")).pathname;
const wasm = (audit: Audit) => audit.requests.filter(request => /\.wasm(?:[?#]|$)/.test(request));
const island = (page: Page) => page.locator("[data-rustify-island]");

async function healthy(page: Page, audit: Audit) {
    await page.evaluate(() => document.fonts.ready);
    expect(audit.failures).toEqual([]);
    expect(audit.console).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    expect(audit.csp).toEqual([]);
    expect(audit.responses.filter(response => response.status >= 400 && !(response.status === 404 && audit.expected404.has(response.url)))).toEqual([]);
    expect(audit.requests.filter(request => new URL(request).origin !== new URL(url("./")).origin || /makepad/i.test(request))).toEqual([]);
}

async function open(page: Page, route: string) {
    const response = await page.goto(url(route));
    expect(response?.status()).toBe(200);
    await expect(page.locator("html")).toHaveAttribute("lang", "en");
    const policy = await page.locator('meta[http-equiv="Content-Security-Policy"]').getAttribute("content");
    expect(policy).toContain("script-src");
    expect(policy).not.toContain("frame-ancestors");
    expect(policy).not.toContain("'unsafe-inline'");
    if (test.info().project.name.endsWith("-meta")) expect(response!.headers()["content-security-policy"]).toBeUndefined();
    else expect(response!.headers()["content-security-policy"]).toContain("frame-ancestors");
    return response;
}

async function attached(page: Page, name: string) {
    await expect(island(page)).toHaveCount(1);
    await expect(island(page)).toHaveAttribute("data-rd-island-state", "ready");
    const metadata = JSON.parse((await island(page).getAttribute("data-rustify-island-metadata"))!);
    expect(metadata.target).toEqual({ type: "demo", name });
    expect(metadata.environment.base).toBe(base());
    expect(metadata.environment.lang).toBe("en");
    const field = island(page).locator("fieldset[data-rd-island-controls]");
    await expect(field).toHaveJSProperty("disabled", false);
    await expect(field).toHaveJSProperty("inert", false);
    await expect(field.locator(":scope > [data-rustify-scope]")).toHaveAttribute("data-rustify-mounted", metadata.seed);
    await expect(field.locator("[style]")).toHaveCount(0);
    expect(await page.evaluate(() => { const ids = [...document.querySelectorAll("[id]")].map(node => node.id); return ids.length === new Set(ids).size; })).toBe(true);
    await page.evaluate(() => document.fonts.ready);
    return field;
}

async function sourceText(page: Page, entry: typeof categories[number]) {
    const source = page.locator(".rd-demo-source");
    await expect(source.locator("[data-rd-source-path]")).toHaveText(entry.file);
    expect(await source.locator("template.rd-code-raw").evaluateAll(nodes => nodes.map(node => (node as HTMLTemplateElement).content.textContent))).toEqual([entry.function, entry.source]);
    return source;
}

async function fits(page: Page) {
    expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth), "page-level horizontal overflow").toBeLessThanOrEqual(0);
}

async function exercise(page: Page, field: Locator, name: string) {
    const state = field.locator("[data-demo-state]");
    await expect(state).not.toBeEmpty();
    switch (name) {
        case "button":
            await expect(field.getByRole("button", { name: "Unavailable", exact: true })).toBeDisabled();
            await field.getByRole("button", { name: "Increment", exact: true }).focus(); await page.keyboard.press("Enter"); await expect(state).toHaveText("1"); break;
        case "label":
            await field.locator("label").click(); await expect(field.getByLabel("Project name", { exact: true })).toBeFocused();
            await field.getByLabel("Project name", { exact: true }).fill("Official reader"); await expect(state).toHaveText("Official reader"); break;
        case "link": {
            const link = field.getByRole("link", { name: "Jump to the destination", exact: true });
            const destination = await link.getAttribute("href"); await link.click(); await expect.poll(() => new URL(page.url()).hash).toBe(destination);
            await field.getByRole("button", { name: "Toggle availability", exact: true }).click(); await expect(state).toHaveText("Unavailable"); await expect(field.locator('a[data-name="Link"]')).toHaveAttribute("aria-disabled", "true"); await expect(field.locator('a[data-name="Link"]')).not.toHaveAttribute("href"); await expect(link).toHaveCount(0); break;
        }
        case "icon": await field.getByRole("button", { name: "Change icon", exact: true }).click(); await expect(state).toHaveText("Close"); await expect(field.getByRole("img", { name: "Not selected", exact: true })).toBeVisible(); break;
        case "text field": {
            const input = field.getByLabel("Project name", { exact: true }); const hint = await input.getAttribute("aria-describedby");
            expect(hint).toBeTruthy(); await expect(field.locator(`[id="${hint}"]`)).toHaveText("Changes appear below as you type.");
            await input.fill("A real edit — résumé"); await expect(state).toHaveText("A real edit — résumé"); break;
        }
        case "text area": await field.getByLabel("Notes", { exact: true }).fill("First line\nSecond line — résumé"); await expect(state).toHaveText("First line\nSecond line — résumé"); break;
        case "checkbox": await field.getByRole("checkbox", { name: "Include this item", exact: true }).press("Space"); await expect(state).toHaveText("Excluded"); break;
        case "radio": await expect(field.getByRole("radio", { name: "Custom (unavailable)", exact: true })).toBeDisabled(); await field.getByRole("radio", { name: "Compact", exact: true }).press("Space"); await expect(state).toHaveText("compact"); break;
        case "switch": await field.getByRole("switch", { name: "Enable notifications", exact: true }).press("Space"); await expect(state).toHaveText("Enabled"); break;
        case "select": {
            const trigger = field.getByRole("combobox", { name: "Language", exact: true }); await trigger.click();
            await field.getByRole("option", { name: "JavaScript", exact: true }).click(); await expect(state).toHaveText("javascript"); await expect(field.getByRole("listbox")).toBeHidden(); await expect(trigger).toBeFocused();
            await trigger.click(); await page.keyboard.press("Escape"); await expect(field.getByRole("listbox")).toBeHidden(); await expect(trigger).toBeFocused(); break;
        }
        case "slider": await field.getByRole("slider", { name: "Value", exact: true }).press("ArrowRight"); await expect(state).toHaveText("30"); break;
        case "progress": await field.getByRole("button", { name: "Advance", exact: true }).press("Enter"); await expect(state).toHaveText("50"); await expect(field.getByRole("progressbar", { name: "Example progress", exact: true })).toHaveAttribute("aria-valuenow", "50"); break;
        case "loading": await field.getByRole("button", { name: "Toggle loading", exact: true }).click(); await expect(state).toHaveText("Ready"); break;
        case "tooltip": {
            const trigger = field.getByRole("button", { name: "Hover or focus for help", exact: true }); await trigger.focus();
            await expect(state).toHaveText("Open"); const tooltip = field.getByRole("tooltip"); await expect(tooltip).toBeVisible(); await expect(trigger).toHaveAttribute("aria-describedby", await tooltip.getAttribute("id") as string);
            await page.keyboard.press("Escape"); await expect(state).toHaveText("Closed"); await expect(trigger).toBeFocused(); break;
        }
        case "menu": {
            const trigger = field.getByRole("button", { name: "Open menu", exact: true }); await trigger.click(); await expect(field.getByRole("menu", { name: "Example actions", exact: true })).toBeVisible();
            await page.keyboard.press("End"); const item = field.getByRole("menuitem", { name: "Rename", exact: true }); await expect(item).toBeFocused(); await page.keyboard.press("Enter");
            await expect(state).toHaveText("rename"); await expect(field.getByRole("menu")).toBeHidden(); await expect(trigger).toBeFocused();
            await trigger.click(); await page.keyboard.press("Escape"); await expect(field.getByRole("menu")).toBeHidden(); await expect(trigger).toBeFocused(); break;
        }
        case "dialog": {
            const trigger = field.getByRole("button", { name: "Open dialog", exact: true }); await trigger.click(); const dialog = field.getByRole("dialog", { name: "Confirm an action", exact: true }); await expect(dialog).toBeVisible();
            await page.keyboard.press("Tab"); expect(await dialog.evaluate(node => node.contains(document.activeElement))).toBe(true);
            await page.keyboard.press("Escape"); await expect(dialog).toBeHidden(); await expect(trigger).toBeFocused();
            await trigger.click(); await dialog.getByRole("button", { name: "Confirm", exact: true }).click(); await expect(state).toHaveText("Confirmed"); await expect(dialog).toBeHidden(); await expect(trigger).toBeFocused(); break;
        }
        case "tabs": await field.getByRole("tab", { name: "Overview", exact: true }).press("ArrowRight"); await expect(field.getByRole("tab", { name: "Details", exact: true })).toBeFocused(); await page.keyboard.press("Enter"); await expect(state).toHaveText("details"); await expect(field.getByRole("tabpanel", { name: "Details", exact: true })).toBeVisible(); break;
        case "scroll area": {
            await field.getByRole("button", { name: "Add item", exact: true }).click(); await expect(state).toHaveText("9");
            const area = field.getByRole("group", { name: "Example items", exact: true }); await area.focus(); await expect(area).toBeFocused(); await area.press("End"); await expect.poll(() => area.evaluate(node => node.scrollTop)).toBeGreaterThan(0); break;
        }
        case "data table": {
            const grid = field.getByRole("grid", { name: "Example components", exact: true }); const cell = grid.getByRole("gridcell").filter({ hasText: /^Button$/ }); await cell.focus(); await page.keyboard.press("Space"); await expect(state).toHaveText("Button"); await page.keyboard.press("Enter"); await expect(field.getByText("Activated: Button", { exact: true })).toBeVisible(); break;
        }
        case "tree": {
            const tree = field.getByRole("tree", { name: "Example documentation tree", exact: true }); const parent = tree.getByRole("treeitem").filter({ hasText: /Components$/ });
            await parent.focus(); await page.keyboard.press("ArrowDown"); const item = tree.getByRole("treeitem").filter({ hasText: /Button$/ }); await expect(item).toBeFocused(); await page.keyboard.press("Enter"); await expect(state).toHaveText("button");
            await page.keyboard.press("ArrowLeft"); await expect(parent).toBeFocused(); await page.keyboard.press("ArrowLeft"); await expect(parent).toHaveAttribute("aria-expanded", "false"); break;
        }
        case "toast": await field.getByRole("button", { name: "Show notification", exact: true }).click(); await expect(state).toHaveText("The example action completed."); await field.locator('[data-name="ToastDismiss"]').click(); await expect(state).toHaveText("None"); break;
        case "number field": await field.getByLabel("Amount", { exact: true }).press("ArrowUp"); await expect(state).toHaveText("13"); break;
        case "color field": await field.getByLabel("Color", { exact: true }).fill("ff000080"); await field.getByLabel("Color", { exact: true }).press("Enter"); await expect(state).toHaveText("#ff000080"); await expect(field.getByTestId("color-demo-swatch")).toHaveCSS("background-color", "rgba(255, 0, 0, 0.5)"); break;
        case "toggle group": {
            const toolbar = field.getByRole("toolbar", { name: "Text styles", exact: true }); await toolbar.getByRole("button", { name: "Bold", exact: true }).press("Space"); await expect(state).toHaveText("bold, italic"); await toolbar.getByRole("button", { name: "Bold", exact: true }).press("ArrowRight"); await expect(toolbar.getByRole("button", { name: "Italic", exact: true })).toBeFocused(); break;
        }
        default: throw new Error(`CATALOG ${name} needs an actual interaction probe`);
    }
}

test("home has one English title, the original README introduction and two first-viewport ordinary actions", async ({ page, audit }) => {
    expect(categories).toHaveLength(24); expect(guides).toHaveLength(12);
    expect(configuredRoutes).toEqual(categories.map(entry => `/${entry.route}`)); expect(configuredGuides).toEqual(guides.map(entry => entry.file));
    await page.setViewportSize({ width: 1280, height: 900 }); await open(page, "./");
    await expect(page.locator("h1")).toHaveCount(1); await expect(page.locator("h1")).toHaveText("Rustify UI");
    await expect(page.locator("[data-official-home]")).toHaveCount(1); await expect(page.locator(".rd-sidebar")).toHaveCount(0);
    expect((await page.locator(".rd-home-prose p").allTextContents()).map(text => text.replace(/\s+/g, " ").trim())).toEqual(introParagraphs);
    const quickstart = page.getByRole("navigation", { name: "Start reading", exact: true }).getByRole("link", { name: "Quick start", exact: true });
    const components = page.getByRole("navigation", { name: "Start reading", exact: true }).getByRole("link", { name: "Components", exact: true });
    await expect(quickstart).toHaveAttribute("href", new URL(url("quickstart/")).pathname); await expect(components).toHaveAttribute("href", new URL(url(categories[0].route)).pathname);
    for (const action of [quickstart, components]) { const box = (await action.boundingBox())!; expect(box.y).toBeGreaterThanOrEqual(0); expect(box.y + box.height).toBeLessThanOrEqual(900); }
    await expect(page.locator(".rd-home-prose").getByRole("link", { name: "SDK quick start", exact: true })).toHaveAttribute("href", new URL(url("quickstart/")).pathname);
    await expect(page.locator(".rd-home-prose").getByRole("link", { name: "repository quick start", exact: true })).toHaveAttribute("href", repoQuickstart);
    await expect(island(page)).toHaveCount(0); expect(wasm(audit)).toEqual([]); await healthy(page, audit);
    await quickstart.click(); await expect(page.locator("h1")).toHaveText(guides[0].title); expect(wasm(audit)).toEqual([]);
});

test("the ordinary sidebar contains the exact twelve guides and CATALOG pages in configured order", async ({ page, audit }) => {
    await open(page, categories[0].route); await attached(page, categories[0].demo);
    const links = page.locator(".rd-sidebar-wide a");
    expect(await links.evaluateAll(nodes => nodes.map(node => node.getAttribute("href")))).toEqual([...guides.map(entry => new URL(url(entry.route)).pathname), ...categories.map(entry => new URL(url(entry.route)).pathname)]);
    await expect(links).toHaveCount(36);
    await expect(page.locator(".rd-sidebar-wide a[aria-current=page]")).toHaveAttribute("href", new URL(url(categories[0].route)).pathname);
    await healthy(page, audit);
});

for (const guide of guides) test(`guide ${guide.route} is English, reachable and requests no wasm`, async ({ page, audit }) => {
    await open(page, guide.route); await expect(page.locator("h1")).toHaveCount(1); await expect(page.locator("h1")).toHaveText(guide.title);
    await expect(page.locator(".rd-sidebar-wide a[aria-current=page]")).toHaveAttribute("href", new URL(url(guide.route)).pathname);
    await expect(island(page)).toHaveCount(0); expect(wasm(audit)).toEqual([]); await healthy(page, audit);
});

for (const entry of categories) test(`CATALOG ${entry.name} uses its actual function, source and live control`, async ({ page, audit }) => {
    await open(page, entry.route); await expect(page.locator("h1")).toHaveCount(1); await expect(page.locator("h1")).toHaveText(entry.title);
    await expect(page.locator("[data-component-category]")).toHaveAttribute("data-component-category", entry.name);
    const field = await attached(page, entry.demo); const source = await sourceText(page, entry);
    await source.getByRole("tab", { name: "Full file", exact: true }).click(); await expect(source.getByRole("tab", { name: "Full file", exact: true })).toHaveAttribute("aria-selected", "true");
    await source.getByRole("tab", { name: "Function", exact: true }).click(); await expect(source.getByRole("tab", { name: "Function", exact: true })).toHaveAttribute("aria-selected", "true");
    await exercise(page, field, entry.name); expect(new Set(wasm(audit)).size).toBe(1);
    expect(audit.responses.filter(response => /\.wasm$/.test(response.url)).every(response => response.status === 200 && response.mime.split(";")[0] === "application/wasm")).toBe(true);
    await expect(page.locator("[data-component-category]").getByRole("link", { name: "component catalogue example", exact: true })).toHaveAttribute("href", "https://github.com/rockie/rustify-ui/tree/main/examples/component-catalog");
    await healthy(page, audit);
});

test("without script every official demo keeps its source and native disabled initial view", async ({ browser }) => {
    test.setTimeout(180_000); const context = await browser.newContext({ javaScriptEnabled: false, viewport: { width: 375, height: 812 } });
    const page = await context.newPage(); const audit = await watch(page);
    try {
        for (const entry of categories) {
            await open(page, entry.route); const field = island(page).locator("fieldset[data-rd-island-controls]"); await expect(field).toHaveJSProperty("disabled", true);
            await expect(island(page).locator("noscript span")).toBeVisible(); await expect(field.locator("[data-demo-state]")).not.toBeEmpty(); await sourceText(page, entry);
            expect(await field.locator("input,button,textarea,select").evaluateAll(nodes => nodes.every(node => node.matches(":disabled")))).toBe(true);
            await fits(page);
        }
        expect(wasm(audit)).toEqual([]); await healthy(page, audit);
        await test.info().attach("official-no-script-audit.json", { body: JSON.stringify({ ...audit, expected404: [] }, null, 2), contentType: "application/json" });
    } finally { await context.close(); }
});

for (const preference of ["light", "dark", "system"] as const) test(`the ${preference} first frame and mode controls persist across static and island pages`, async ({ page, audit }) => {
    await page.emulateMedia({ colorScheme: "dark" });
    await page.addInitScript(({ key, preference }) => {
        if (localStorage.getItem(key) === null) localStorage.setItem(key, preference); document.addEventListener("DOMContentLoaded", () => { window.__official_first_theme = { mode: document.documentElement.getAttribute("data-theme"), background: getComputedStyle(document.body).backgroundColor }; });
    }, { key: `rustify-docs:mode:${base()}`, preference });
    await open(page, "./"); await expect(page.locator("html")).toHaveAttribute("data-theme", preference);
    expect(await page.evaluate(() => window.__official_first_theme?.mode)).toBe(preference);
    const initial = await page.evaluate(() => window.__official_first_theme!.background);
    expect(await page.locator("body").evaluate(node => getComputedStyle(node).backgroundColor)).toBe(initial);
    const table = categories.find(entry => entry.name === "data table")!; await open(page, table.route); const field = await attached(page, table.demo);
    await page.evaluate(async () => { await Promise.allSettled(document.getAnimations().filter(animation => animation instanceof CSSTransition).map(animation => animation.finished)); });
    const colors = await page.locator("body").evaluate(node => ({ foreground: getComputedStyle(node).color, background: getComputedStyle(node).backgroundColor }));
    await expect(field.getByRole("grid", { name: "Example components", exact: true })).toHaveCSS("background-color", colors.background);
    await expect(field.getByRole("gridcell").first()).toHaveCSS("color", colors.foreground); expect(colors.foreground).not.toBe(colors.background);
    await open(page, "./");
    await page.locator('[data-rd-mode="light"]').press("Enter"); const light = await page.locator("body").evaluate(node => getComputedStyle(node).backgroundColor);
    await page.locator('[data-rd-mode="dark"]').press("Enter"); const dark = await page.locator("body").evaluate(node => getComputedStyle(node).backgroundColor); expect(light).not.toBe(dark);
    await page.getByRole("navigation", { name: "Start reading", exact: true }).getByRole("link", { name: "Components", exact: true }).click(); await attached(page, categories[0].demo);
    await expect(page.locator("html")).toHaveAttribute("data-theme", "dark"); await expect(page.locator('[data-rd-mode="dark"]')).toHaveAttribute("aria-pressed", "true"); await healthy(page, audit);
});

for (const layout of [{ name: "375px", width: 375, scaleText: false }, { name: "320px", width: 320, scaleText: false }, { name: "200% layout and text emulation", width: 640, scaleText: true }]) test(`${layout.name} retains reading controls, ordered narrow navigation and bounded long source`, async ({ page, audit }) => {
    await page.setViewportSize({ width: layout.width, height: 800 }); await page.emulateMedia({ reducedMotion: "reduce" });
    const entry = categories.find(entry => entry.name === "data table")!;
    for (const route of ["./", guides[0].route, entry.route]) {
        await open(page, route);
        // Under reduce, scaling the text starts no transition to wait out.
        if (layout.scaleText) expect(await page.evaluate(() => { document.documentElement.style.fontSize = "200%"; return document.getAnimations().filter(animation => animation instanceof CSSTransition).map(animation => (animation as CSSTransition).transitionProperty); })).toEqual([]);
        if (route === entry.route) {
            await attached(page, entry.demo); const source = await sourceText(page, entry); await source.getByRole("tab", { name: "Full file", exact: true }).click();
            const pre = source.locator('.rd-code:not([hidden]) .rd-code-pre'); await pre.focus(); await expect(pre).toBeFocused();
            const size = await pre.evaluate(node => ({ client: node.clientHeight, content: node.scrollHeight, limit: 24 * parseFloat(getComputedStyle(document.documentElement).fontSize) })); expect(size.client).toBeLessThanOrEqual(size.limit); expect(size.content).toBeGreaterThan(size.client);
        }
        if (route !== "./") {
            const details = page.locator(".rd-sidebar-narrow"); await details.locator(":scope > summary").click(); await expect(details).toHaveAttribute("open", "");
            expect(await details.locator("a").evaluateAll(nodes => nodes.map(node => node.getAttribute("href")))).toEqual([...guides.map(entry => new URL(url(entry.route)).pathname), ...categories.map(entry => new URL(url(entry.route)).pathname)]);
            await details.locator(":scope > summary").click();
        }
        await fits(page); await page.evaluate(() => document.fonts.ready);
    }
    await healthy(page, audit);
});

test("deep not-found retains same-base ordinary navigation", async ({ page, audit }) => {
    const missing = url("unavailable/deeper/page/"); audit.expected404.add(missing); const response = await page.goto(missing); expect(response?.status()).toBe(404);
    await expect(page.locator("h1")).toHaveText("Page not found"); await expect(page.locator("html")).toHaveAttribute("lang", "en"); await expect(island(page)).toHaveCount(0); expect(wasm(audit)).toEqual([]);
    const home = page.getByRole("contentinfo").getByRole("link", { name: "Back to the home page", exact: true }); await expect(home).toHaveAttribute("href", base()); await home.click(); await expect(page).toHaveURL(url("./")); await healthy(page, audit);
});

test("one bounded official home, category and guide capture batch records desktop light/dark and narrow reading", async ({ page, audit }) => {
    test.setTimeout(120_000); const entry = categories.find(entry => entry.name === "data table")!;
    for (const [kind, route] of [["home", "./"], ["category", entry.route], ["guide", guides[0].route]] as const) {
        for (const variant of [{ name: "desktop-light", width: 1280, height: 900, mode: "light" }, { name: "desktop-dark", width: 1280, height: 900, mode: "dark" }, { name: "narrow-light", width: 375, height: 812, mode: "light" }]) {
            await page.setViewportSize({ width: variant.width, height: variant.height }); await open(page, route);
            if (kind === "category") await attached(page, entry.demo);
            await page.locator(`[data-rd-mode="${variant.mode}"]`).click(); await page.evaluate(async () => { await document.fonts.ready; await Promise.allSettled(document.getAnimations().filter(animation => animation instanceof CSSTransition).map(animation => animation.finished)); window.scrollTo(0, 0); });
            await fits(page);
            // One nine-image review batch; every host still exercises these states.
            if (test.info().project.name === "official-subpath") await test.info().attach(`official-${kind}-${variant.name}.png`, { body: await page.screenshot(), contentType: "image/png" });
        }
    }
    await healthy(page, audit);
});

// The categories whose demos hold the controls the reduced-motion contract names.
const motionCategories = ["button", "tabs", "switch", "checkbox", "radio", "progress", "color field", "loading"];

test("gaps reduced motion: under reduce the default theme's controls attach and change with no transition and a still spinner", async ({ page, audit }) => {
    await installMotionProbe(page); await page.emulateMedia({ reducedMotion: "reduce" });
    const samples: MotionSample[] = [];
    for (const name of motionCategories) {
        const entry = categories.find(entry => entry.name === name)!; await open(page, entry.route); const field = await attached(page, entry.demo);
        samples.push(await sampleMotion(page, `${name} ready`));
        if (name === "button") { await field.locator('[data-name="Button"]').first().hover(); samples.push(await sampleMotion(page, "button hover")); }
        await exercise(page, field, name); samples.push(await sampleMotion(page, `${name} changed`));
    }
    await attachMotion(page, test.info(), "gaps-reduced-motion.json", samples);
    for (const target of Object.keys(motionTargets)) expect(samples.some(sample => sample.present[target] > 0), target).toBe(true);
    for (const sample of samples) expectStill(sample);
    await healthy(page, audit);
});

test("gaps reduced motion: without a preference the default theme keeps 150ms transitions and a turning spinner", async ({ page, audit }) => {
    await installMotionProbe(page); await page.emulateMedia({ reducedMotion: "no-preference" });
    const samples: MotionSample[] = [];
    for (const name of ["switch", "loading"]) {
        const entry = categories.find(entry => entry.name === name)!; await open(page, entry.route); const field = await attached(page, entry.demo);
        samples.push(await sampleMotion(page, `${name} ready`));
        if (name === "switch") { await field.getByRole("switch").press("Space"); samples.push(await sampleMotion(page, "switch toggled")); }
    }
    await attachMotion(page, test.info(), "gaps-normal-motion.json", samples);
    for (const sample of samples) expectNormal(sample);
    expect(samples[0].tokens.length).toBeGreaterThan(0); expect(samples[0].tokens.map(seconds).every(value => value === 0.15)).toBe(true);
    expect(samples[1].transitions.filter(entry => entry.node.startsWith("span.") && entry.node.endsWith("in Switch") && entry.property === "translate").map(entry => entry.duration)).toEqual([150]);
    expect(samples[2].spinners).toEqual([{ name: "rui-spin", running: 1 }]);
    await healthy(page, audit);
});

// Every registered route (home and the 24 categories), the Markdown quick
// start and a deep missing address, as the projected registry lists them.
const gapsRoutes = [...declaredPages.map(page => page.route.slice(1)), "quickstart/", "__gaps_missing__/deeper/"];

for (const colorScheme of ["light", "dark"] as const) for (const width of [1280, 375]) {
    test(`gaps a11y: every official route at rest has no serious or critical axe violation, ${colorScheme} ${width}px`, async ({ page, audit }) => {
        test.setTimeout(600_000);
        expect(gapsRoutes).toHaveLength(27);
        await page.setViewportSize({ width, height: 900 }); await page.emulateMedia({ colorScheme, reducedMotion: "no-preference" });
        const scans: Scan[] = [];
        for (const route of gapsRoutes) {
            const missing = route.startsWith("__gaps_missing__");
            if (missing) audit.expected404.add(url(route));
            const response = await page.goto(url(route)); expect(response!.status(), route).toBe(missing ? 404 : 200);
            const count = await island(page).count();
            const ready = page.locator('[data-rustify-island][data-rd-island-state="ready"]');
            if (count) await expect(ready).toHaveCount(count);
            await atRest(page);
            if (count) await expect(ready).toHaveCount(count);
            scans.push(await scan(page, route, count ? "ready" : "static"));
        }
        await attachScans(test.info(), `gaps-a11y-official-${colorScheme}-${width}.json`, scans, await buildId(page, url("./")));
        expect(serious(scans)).toEqual([]);
        await healthy(page, audit);
    });
}
