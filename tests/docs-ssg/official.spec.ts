import { expect, Locator, Page, test as playwright } from "@playwright/test";
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { attachMotion, expectNormal, expectStill, installMotionProbe, MotionSample, motionTargets, sampleMotion, seconds } from "./motion";
import { atRest, attachScans, buildId, scan, Scan, serious } from "./a11y";
import { EVIDENCE } from "../tier";
import { capture, fitsWidth, nativeZoom, NATIVE_MISSING, NATIVE_SKIP } from "../native-zoom";

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
// The site's own showcase directory (§2): every released family with its
// shareable place and the registered demo that renders it.
const showcaseSource = readFileSync("site/src/showcase.rs", "utf8");
const showcaseEntries = [...showcaseSource.matchAll(/ShowcaseEntry\s*\{\s*group:\s*"([^"]+)",\s*route:\s*"([^"]+)",\s*anchor:\s*"([^"]+)",\s*title:\s*"([^"]+)",\s*demo:\s*"([^"]+)"/g)].map(match => ({ group: match[1], route: match[2], anchor: match[3], title: match[4], demo: match[5] }));
const showcaseGroups = [...showcaseSource.matchAll(/\("([a-z]+)", "([A-Za-z]+)"\)/g)].map(match => ({ slug: match[1], label: match[2] }));
for (const entry of showcaseEntries) {
    if (!declarations.some(demo => demo.name === entry.demo)) throw new Error(`SHOWCASE ${entry.title} references unregistered demo ${entry.demo}`);
}
const exploreRoutes = ["/colors/", "/primitives/", "/studio/"];
const componentNav = [...showcaseGroups.map(group => `/components/${group.slug}/`), ...categories.map(entry => `/${entry.route}`)];
const indexedPages = 15 + 24 + 9 + 3 + 1;
const include = config.match(/\[content\][\s\S]*?include\s*=\s*\[([\s\S]*?)\]/)?.[1];
if (!include) throw new Error("official content.include must be explicit");
// A source's own frontmatter description; a missing one fails only the assertions that read it.
const frontDescription = (source: string) => source.match(/^---\r?\n([\s\S]*?)\r?\n---\r?\n/)?.[1].match(/^description: ([^\r\n]+)$/m)?.[1];
const guides = [...include.matchAll(/"([^"]+\.md)"/g)].filter(match => !match[1].startsWith("zh/")).map(match => {
    const file = `docs/${match[1]}`;
    const source = readFileSync(file, "utf8");
    const title = source.match(/^# ([^\r\n]+)$/m)?.[1];
    if (!title) throw new Error(`guide ${file} has no source H1`);
    return { file, route: `${path.basename(match[1], ".md")}/`, title, description: frontDescription(source) };
});
const zhQuickstartDescription = frontDescription(readFileSync("docs/zh/quickstart.md", "utf8"));
// The escaping llms.txt applies to titles and descriptions.
const llmsText = (value: string) => value.replace(/[\\[\]<>*_`]/g, "\\$&");
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
        // HTTP/2 omits the reason phrase; keep the expected document URL guard.
        const missing = /^Failed to load resource: the server responded with a status of 404 \((?:Not Found)?\)$/.test(message.text());
        if (audit.expected404.has(message.location().url) && missing) audit.expectedConsole.push(message.text());
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

async function open(page: Page, route: string, lang = "en") {
    const response = await page.goto(url(route));
    expect(response?.status()).toBe(200);
    await expect(page.locator("html")).toHaveAttribute("lang", lang);
    const policy = await page.locator('meta[http-equiv="Content-Security-Policy"]').getAttribute("content");
    expect(policy).toContain("script-src");
    expect(policy).not.toContain("frame-ancestors");
    expect(policy).not.toContain("'unsafe-inline'");
    if (test.info().project.name.endsWith("-meta")) expect(response!.headers()["content-security-policy"]).toBeUndefined();
    else expect(response!.headers()["content-security-policy"]).toContain("frame-ancestors");
    return response;
}

async function attached(page: Page, name: string, lang = "en") {
    await expect(island(page)).toHaveCount(1);
    await expect(island(page)).toHaveAttribute("data-rd-island-state", "ready");
    const metadata = JSON.parse((await island(page).getAttribute("data-rustify-island-metadata"))!);
    expect(metadata.target).toEqual({ type: "demo", name });
    expect(metadata.environment.base).toBe(base());
    expect(metadata.environment.lang).toBe(lang);
    const field = island(page).locator("fieldset[data-rd-island-controls]");
    await expect(field).toHaveJSProperty("disabled", false);
    await expect(field).toHaveJSProperty("inert", false);
    await expect(field.locator(":scope > [data-rustify-scope]")).toHaveAttribute("data-rustify-mounted", metadata.seed);
    await expect(field.locator("[style]:not([data-rustify-notifications])")).toHaveCount(0);
    expect(await page.evaluate(() => { const ids = [...document.querySelectorAll("[id]")].map(node => node.id); return ids.length === new Set(ids).size; })).toBe(true);
    await page.evaluate(() => document.fonts.ready);
    return field;
}

// A page can host several named islands; wait for all of them, pick one.
async function attachedDemo(page: Page, name: string, lang = "en") {
    const all = island(page);
    const total = await all.count();
    expect(total, `island ${name}`).toBeGreaterThan(0);
    for (let index = 0; index < total; index++) await expect(all.nth(index)).toHaveAttribute("data-rd-island-state", "ready");
    const names = await all.evaluateAll(nodes => nodes.map(node => { const parsed = JSON.parse(node.getAttribute("data-rustify-island-metadata") || "null"); return parsed?.target?.name ?? ""; }));
    const index = names.indexOf(name);
    expect(index, `island ${name} on the page`).toBeGreaterThanOrEqual(0);
    const metadata = JSON.parse((await all.nth(index).getAttribute("data-rustify-island-metadata"))!);
    expect(metadata.environment.base).toBe(base());
    expect(metadata.environment.lang).toBe(lang);
    const field = all.nth(index).locator("fieldset[data-rd-island-controls]");
    await expect(field).toHaveJSProperty("disabled", false);
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
        case "checkbox": {
            const parent = field.getByRole("checkbox", { name: "Include every package", exact: true });
            await expect(parent).toHaveJSProperty("indeterminate", true); await expect(state).toHaveText("mixed · docs, components");
            await parent.press("Space"); await expect(state).toHaveText("all · docs, dom, components"); await expect(field.getByRole("checkbox", { name: "DOM SDK", exact: true })).toBeChecked();
            await field.getByRole("checkbox", { name: "Docs", exact: true }).press("Space"); await expect(parent).toHaveJSProperty("indeterminate", true); await expect(state).toHaveText("mixed · dom, components"); break;
        }
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
            await expect(field.getByRole("menuitem", { name: "Copy", exact: true })).toBeFocused();
            await page.keyboard.press("End"); const item = field.getByRole("menuitem", { name: "Rename", exact: true }); await expect(item).toBeFocused(); await page.keyboard.press("Enter");
            await expect(state).toHaveText("rename"); await expect(field.getByRole("menu")).toBeHidden(); await expect(trigger).toBeFocused();
            await trigger.click(); await expect(field.getByRole("menuitem", { name: "Copy", exact: true })).toBeFocused(); await page.keyboard.press("Escape"); await expect(field.getByRole("menu")).toBeHidden(); await expect(trigger).toBeFocused(); break;
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
    expect(categories).toHaveLength(24); expect(guides).toHaveLength(15);
    expect(configuredRoutes).toEqual(componentNav); expect(configuredGuides).toEqual(guides.map(entry => entry.file));
    await page.setViewportSize({ width: 1280, height: 900 }); await open(page, "./");
    await expect(page.locator("h1")).toHaveCount(1); await expect(page.locator("h1")).toHaveText("Rustify UI");
    await expect(page.locator("[data-official-home]")).toHaveCount(1); await expect(page.locator(".rd-sidebar")).toHaveCount(0);
    expect((await page.locator(".rd-home-prose p").allTextContents()).map(text => text.replace(/\s+/g, " ").trim())).toEqual(introParagraphs);
    const quickstart = page.getByRole("navigation", { name: "Start reading", exact: true }).getByRole("link", { name: "Quick start", exact: true });
    const components = page.getByRole("navigation", { name: "Start reading", exact: true }).getByRole("link", { name: "Components", exact: true });
    await expect(quickstart).toHaveAttribute("href", new URL(url("quickstart/")).pathname); await expect(components).toHaveAttribute("href", new URL(url("components/")).pathname);
    for (const action of [quickstart, components]) { const box = (await action.boundingBox())!; expect(box.y).toBeGreaterThanOrEqual(0); expect(box.y + box.height).toBeLessThanOrEqual(900); }
    await expect(page.locator(".rd-home-prose").getByRole("link", { name: "SDK quick start", exact: true })).toHaveAttribute("href", new URL(url("quickstart/")).pathname);
    await expect(page.locator(".rd-home-prose").getByRole("link", { name: "repository quick start", exact: true })).toHaveAttribute("href", repoQuickstart);
    await attachedDemo(page, "ButtonDemo");
    await attachedDemo(page, "SegmentedControlDemo");
    expect(new Set(wasm(audit)).size).toBe(1); await healthy(page, audit);
    await quickstart.click(); await expect(page.locator("h1")).toHaveText(guides[0].title); expect(new Set(wasm(audit)).size).toBe(1);
});

test("official Chinese home and quick start are real translations with local search and AI discovery", async ({ page, request, audit }) => {
    await page.setViewportSize({ width: 1280, height: 900 });
    await open(page, "./");
    await page.locator(".rd-language-switch summary").click();
    await page.locator('.rd-language-switch a[hreflang="zh"]').click();
    await expect(page).toHaveURL(url("zh/"));
    await expect(page.locator("html")).toHaveAttribute("lang", "zh");
    await expect(page.locator(".rd-tagline")).toContainText("实验性 Rust SDK");
    await expect(page.locator(".rd-untranslated")).toHaveCount(0);
    if (test.info().project.name === "official-subpath") {
        await page.screenshot({ path: test.info().outputPath("official-zh-home-1280.png"), fullPage: true });
        await page.setViewportSize({ width: 375, height: 812 });
        await page.screenshot({ path: test.info().outputPath("official-zh-home-375.png"), fullPage: true });
        await page.setViewportSize({ width: 1280, height: 900 });
    }
    await page.getByRole("navigation", { name: "开始阅读", exact: true }).getByRole("link", { name: "快速开始", exact: true }).click();
    await expect(page).toHaveURL(url("zh/quickstart/"));
    await expect(page.locator("main article").first()).toHaveAttribute("lang", "zh");
    await expect(page.locator("#a-dom-application-without-gpu-regions")).toContainText("DOM 应用");
    await page.locator("[data-rd-search-open]").click();
    const dialog = page.locator("dialog[data-rd-search]");
    await dialog.getByRole("combobox").fill("准备工具");
    await expect(dialog.getByRole("option").first().locator(".rd-search-title")).toHaveText("快速开始");
    await expect(dialog.getByRole("option").first()).toHaveAttribute("href", /\/zh\/quickstart\/#prerequisites$/);
    const indexURL = await dialog.getAttribute("data-rd-search-index");
    const data = await (await request.get(new URL(indexURL!, url("./")).href)).json();
    expect(data.pages).toHaveLength(indexedPages);
    expect(data.pages.filter((page: any) => page.is_fallback)).toHaveLength(indexedPages - 2);
    await page.keyboard.press("Enter");
    await expect(dialog).toBeHidden();
    await expect(page).toHaveURL(url("zh/quickstart/#prerequisites"));
    for (const [path, expected] of [["llms.txt", "/rustify-ui/quickstart/"], ["zh/llms.txt", "/rustify-ui/zh/quickstart/"]] as const) {
        const response = await request.get(url(path));
        expect(response.status()).toBe(200);
        expect(response.headers()["content-type"]).toContain("text/plain");
        const text = await response.text();
        expect(text.split("\n").filter(line => line.startsWith("- ["))).toHaveLength(indexedPages);
        const target = expected.replace("/rustify-ui/", base());
        expect(text).toContain(`https://rockie.github.io${target}`);
        if (path.startsWith("zh/")) {
            expect(text).toContain("尚未翻译");
            expect(text).toContain(`https://rockie.github.io${base()}components/button/`);
            expect(text).not.toContain(`https://rockie.github.io${base()}zh/components/button/`);
        }
        const lines = text.split("\n");
        for (const guide of guides) {
            const translated = path.startsWith("zh/") && guide.route === "quickstart/";
            const link = `](https://rockie.github.io${base()}${translated ? "zh/" : ""}${guide.route}): `;
            const line = lines.find(line => line.startsWith("- [") && line.includes(link));
            expect(line, `${path} lists ${guide.file}`).toBeDefined();
            const [label, description] = [line!.slice(3, line!.indexOf(link)), line!.slice(line!.indexOf(link) + link.length)];
            if (translated) {
                expect(zhQuickstartDescription, "docs/zh/quickstart.md declares a description").toBeDefined();
                expect(description).toBe(llmsText(zhQuickstartDescription!));
                continue;
            }
            expect(guide.description, `${guide.file} declares a description`).toBeDefined();
            expect(description, `${path}: ${guide.file} uses its frontmatter description`).toBe(llmsText(guide.description!));
            if (path.startsWith("zh/")) {
                expect(label.startsWith(`${llmsText(guide.title)} (`), label).toBe(true);
                expect(label).toContain("尚未翻译");
            } else expect(label).toBe(llmsText(guide.title));
        }
    }
    await healthy(page, audit);
});

test("all Chinese fallback categories retain real DOM demos, source and target-language controls", async ({ page, audit }) => {
    test.setTimeout(240_000);
    for (const entry of categories) {
        await open(page, `zh/${entry.route}`, "zh");
        await expect(page.locator(".rd-untranslated")).toBeVisible();
        const field = await attached(page, entry.demo, "zh");
        await sourceText(page, entry);
        await expect(page.locator(".rd-demo-source").getByRole("tab", { name: "完整文件", exact: true })).toHaveCount(1);
        const metadata = JSON.parse((await island(page).getAttribute("data-rustify-island-metadata"))!);
        expect(metadata.environment.lang).toBe("zh");
        await exercise(page, field, entry.name);
    }
    await healthy(page, audit);
});

test("the ordinary sidebar contains the exact fifteen guides and CATALOG pages in configured order", async ({ page, audit }) => {
    await open(page, categories[0].route); await attached(page, categories[0].demo);
    const links = page.locator(".rd-sidebar-wide a");
    expect(await links.evaluateAll(nodes => nodes.map(node => node.getAttribute("href")))).toEqual([...exploreRoutes.map(route => new URL(url(route.slice(1))).pathname), ...guides.map(entry => new URL(url(entry.route)).pathname), ...componentNav.map(route => new URL(url(route.slice(1))).pathname)]);
    await expect(links).toHaveCount(exploreRoutes.length + guides.length + componentNav.length);
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
    await open(page, categories[0].route); await attached(page, categories[0].demo);
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
            expect(await details.locator("a").evaluateAll(nodes => nodes.map(node => node.getAttribute("href")))).toEqual([...exploreRoutes.map(route => new URL(url(route.slice(1))).pathname), ...guides.map(entry => new URL(url(entry.route)).pathname), ...componentNav.map(route => new URL(url(route.slice(1))).pathname)]);
            await details.locator(":scope > summary").click();
        }
        await fits(page); await page.evaluate(() => document.fonts.ready);
    }
    await healthy(page, audit);
});

test("search section targets stay below a wrapped reading header", async ({ page, audit }) => {
    await page.emulateMedia({ reducedMotion: "reduce" });
    for (const [width, lang] of [[320, "en"], [320, "zh"], [375, "zh"], [1280, "zh"]] as const) {
        await page.setViewportSize({ width, height: 800 });
        await open(page, `${lang === "zh" ? "zh/" : ""}quickstart/`, lang);
        await page.locator("[data-rd-search-open]").click();
        await page.getByRole("combobox").fill("ThemeScope");
        await page.getByRole("option").filter({ hasText: "Architecture" }).click();
        await expect(page).toHaveURL(/\/architecture\/#versioned-themes$/);
        await atRest(page);
        const bounds = await page.locator("#versioned-themes").evaluate(target => ({
            target: target.getBoundingClientRect().top,
            header: document.querySelector(".rd-header-bar")!.parentElement!.getBoundingClientRect().bottom,
        }));
        await test.info().attach(`search-target-${width}-${lang}.json`, { body: JSON.stringify(bounds), contentType: "application/json" });
        expect(bounds.target, `visible section target at ${width}px in ${lang}`).toBeGreaterThanOrEqual(bounds.header);
        await fits(page);
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
    expect(samples[1].transitions.filter(entry => entry.node.startsWith("span.") && entry.node.endsWith("in Switch") && entry.property === "transform").map(entry => entry.duration)).toEqual([150]);
    expect(samples[2].spinners).toHaveLength(8);
    for (const leaf of samples[2].spinners) expect(leaf).toEqual({ name: "rustify-spinner-fade", running: 1 });
    await healthy(page, audit);
});

test("native 200%: home, narrow navigation, tree and data table demos and mode switch fit the zoomed window", { tag: EVIDENCE }, async ({ playwright }, info) => {
    test.skip(NATIVE_MISSING, NATIVE_SKIP);
    test.setTimeout(180_000);
    const zoom = await nativeZoom(playwright.chromium, info);
    try {
        const page = zoom.page, audit = await watch(page);
        await open(page, "./"); await expect(page.locator("h1")).toHaveText("Rustify UI"); await fitsWidth(page);
        // The home page has no sidebar; its own Components action leads into the docs.
        await page.getByRole("navigation", { name: "Start reading", exact: true }).getByRole("link", { name: "Components", exact: true }).click();
        await expect(page.locator("h1")).toHaveText("Component catalogue"); await fitsWidth(page);
        const visited: { route: string; tableScroll?: { scrollWidth: number; clientWidth: number } | null }[] = [];
        for (const name of ["tree", "data table"]) {
            const entry = categories.find(entry => entry.name === name)!;
            const narrow = page.locator(".rd-sidebar-narrow");
            await expect(page.locator(".rd-sidebar-wide")).toBeHidden();
            await narrow.locator("summary").click(); await expect(narrow).toHaveAttribute("open", "");
            await narrow.locator(`a[href="${new URL(url(entry.route)).pathname}"]`).click();
            await expect(page).toHaveURL(url(entry.route)); await expect(page.locator("h1")).toHaveText(entry.title);
            const field = await attached(page, entry.demo); await exercise(page, field, name); await fitsWidth(page);
            const tableScroll = name === "data table" ? await field.getByRole("grid").evaluate(node => {
                const scroller = [node, ...node.querySelectorAll("*"), ...(node.parentElement ? [node.parentElement] : [])].find(child => child.scrollWidth > child.clientWidth + 1 && /auto|scroll/.test(getComputedStyle(child).overflowX));
                return scroller ? { scrollWidth: scroller.scrollWidth, clientWidth: scroller.clientWidth } : null;
            }) : undefined;
            visited.push({ route: entry.route, tableScroll });
            await capture(page, `docs/validation/dom-components-p5/native-200-${info.project.name}-${name.replace(" ", "-")}.png`);
        }
        const light = await page.locator("body").evaluate(node => getComputedStyle(node).backgroundColor);
        await page.locator('[data-rd-mode="dark"]').click(); await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
        expect(await page.locator("body").evaluate(node => getComputedStyle(node).backgroundColor)).not.toBe(light); await fitsWidth(page);
        await healthy(page, audit);
        writeFileSync(`docs/validation/dom-components-p5/native-200-${info.project.name}.json`, JSON.stringify({ environment: zoom.environment, visited }, null, 2));
    } finally { await zoom.close(); }
});

// V-14: the showcase groups carry the new families next to the old pages.
const groupPages = showcaseGroups.map(group => ({ ...group, route: `components/${group.slug}/`, entries: showcaseEntries.filter(entry => entry.group === group.slug) }));

test("V-14 every showcase group page renders its inline entries and links the old detail pages", async ({ page, audit }) => {
    test.setTimeout(300_000);
    expect(groupPages).toHaveLength(9);
    for (const group of groupPages) {
        await open(page, group.route);
        await expect(page.locator("h1")).toHaveText(group.label);
        await expect(page.locator(`[data-showcase-category="${group.slug}"]`)).toBeVisible();
        const inline = group.entries.filter(entry => entry.route === `/${group.route}`);
        const linked = group.entries.filter(entry => entry.route !== `/${group.route}`);
        expect(inline.length).toBeGreaterThan(0);
        for (const entry of inline) {
            await expect(page.locator(`[data-showcase-entry="${entry.anchor}"]`)).toBeVisible();
            const field = await attachedDemo(page, entry.demo);
            await expect(field.locator("[data-demo-state]")).not.toBeEmpty();
        }
        for (const entry of linked) await expect(page.locator("ul.rd-showcase-links").getByRole("link", { name: entry.title, exact: true })).toHaveAttribute("href", new RegExp(`#${entry.anchor}$`));
        await healthy(page, audit);
    }
});

test("V-14 the upgraded button page shows the full matrix with working state toggles", async ({ page, audit }) => {
    await open(page, "components/button/");
    const field = await attached(page, "ButtonDemo");
    const increment = field.getByRole("button", { name: "Increment", exact: true });
    const state = field.locator("[data-demo-state]");
    await expect(field.getByRole("button", { name: "Unavailable", exact: true })).toBeDisabled();
    await increment.focus(); await page.keyboard.press("Enter"); await expect(state).toHaveText("1");
    const solid = await increment.evaluate(node => getComputedStyle(node).backgroundColor);
    await field.getByLabel("Variant").selectOption("Outline");
    await expect.poll(() => increment.evaluate(node => getComputedStyle(node).backgroundColor)).not.toBe(solid);
    await field.getByLabel("Size").selectOption({ index: 3 });
    await field.getByLabel("Disabled").check(); await expect(increment).toBeDisabled();
    await page.keyboard.press("Enter"); await expect(state).toHaveText("1");
    await field.getByLabel("High contrast").check(); await expect(state).toHaveText("1");
    await field.getByLabel("Disabled").uncheck(); await field.getByLabel("Loading").check();
    await expect(increment).toBeDisabled();
    await field.getByLabel("Loading").uncheck(); await increment.click(); await expect(state).toHaveText("2");
    await healthy(page, audit);
});

test("V-14 the menubar entry executes commands by keyboard", async ({ page, audit }) => {
    await open(page, "components/menus/");
    const field = await attachedDemo(page, "MenubarDemo");
    const state = field.locator("[data-demo-state]");
    await field.getByRole("menuitem", { name: "File", exact: true }).focus();
    await page.keyboard.press("Enter");
    const menu = field.getByRole("menu", { name: "File commands", exact: true });
    await expect(menu).toBeVisible();
    await expect(menu.getByRole("menuitem", { name: "New project", exact: true })).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(state).toContainText("New project");
    await expect(menu).toBeHidden();
    await field.getByRole("menuitem", { name: "View", exact: true }).focus();
    await page.keyboard.press("ArrowDown");
    await expect(field.getByRole("menu", { name: "View commands", exact: true }).getByRole("menuitem", { name: "Zen mode (unavailable)", exact: true })).toBeDisabled();
    await page.keyboard.press("Escape");
    await healthy(page, audit);
});

// What a reader sees after operating a sample, not the state text alone.
const box = async (locator: Locator) => (await locator.boundingBox())!;

test("V-14 interaction samples change the controls they drive", async ({ page, audit }) => {
    await open(page, "components/button/");
    const button = await attached(page, "ButtonDemo");
    const increment = button.getByRole("button", { name: "Increment", exact: true });
    const accent = await increment.evaluate(node => getComputedStyle(node).backgroundColor);
    await button.getByLabel("Color").selectOption("crimson");
    await expect(increment).toHaveAttribute("data-color", "crimson");
    await expect.poll(() => increment.evaluate(node => getComputedStyle(node).backgroundColor)).not.toBe(accent);
    await increment.click(); await expect(button.locator("[data-demo-state]")).toHaveText("1");

    await open(page, "components/interaction/");
    const cards = await attachedDemo(page, "CheckboxCardsDemo");
    const mail = cards.getByRole("checkbox", { name: /Email updates/ });
    const small = await box(mail);
    await cards.getByLabel("Size").selectOption({ index: 2 });
    await expect.poll(async () => (await box(mail)).height).toBeGreaterThan(small.height);
    await cards.getByRole("checkbox", { name: /Product news/ }).click();
    await cards.getByRole("button", { name: "Submit the form", exact: true }).click();
    await expect(cards.getByText("Form data: channel=mail&channel=news")).toBeVisible();
    const radios = await attachedDemo(page, "RadioCardsDemo");
    await radios.getByRole("radio", { name: /Starter/ }).click();
    await radios.getByRole("button", { name: "Submit the form", exact: true }).click();
    await expect(radios.getByText("Form data: plan=starter")).toBeVisible();

    const range = await attachedDemo(page, "RangeSliderDemo");
    const state = range.locator("[data-demo-state]");
    const high = range.getByRole("slider", { name: "High edge" }).first();
    await expect(state).toHaveText("20 – 60");
    await high.press("ArrowRight"); await expect(state).toHaveText("20 – 65");
    for (let press = 0; press < 3; press++) await high.press("ArrowRight");
    await expect(state).toHaveText("20 – 80"); await expect(range.getByText("Committed: 20 – 80")).toBeVisible();
    await high.press("ArrowRight");
    await expect(range.getByText(/^Rejected 20 – 85/)).toBeVisible(); await expect(state).toHaveText("20 – 80");
    await expect(high).toHaveAttribute("aria-valuenow", "80");
    const low = range.getByRole("slider", { name: "Low edge" }).first();
    for (let press = 0; press < 12; press++) await low.press("ArrowRight");
    await expect(state).toHaveText("70 – 80");
    const vertical = range.getByRole("slider", { name: "High edge" }).nth(1);
    await expect(vertical).toHaveAttribute("aria-orientation", "vertical");
    await expect(vertical).toHaveAttribute("aria-valuenow", "80");

    const parts = await attachedDemo(page, "SelectPartsDemo");
    const trigger = parts.getByRole("combobox", { name: "Release", exact: true });
    await trigger.click();
    const list = page.getByRole("listbox");
    await expect(list.getByRole("group", { name: "Stable" })).toBeVisible();
    await expect(list.getByRole("group", { name: "Preview" }).getByRole("option")).toHaveCount(6);
    await expect(list.getByRole("option", { name: "Release 0.11.0", exact: true })).toHaveAttribute("aria-disabled", "true");
    await list.getByRole("option", { name: "Release 0.8.0", exact: true }).click();
    await expect(trigger).toContainText("Release 0.8.0"); await expect(trigger).toBeFocused();
    await parts.getByRole("button", { name: "Clear the release", exact: true }).click();
    await expect(parts.locator("[data-demo-state]")).toHaveText("none"); await expect(trigger).toContainText("Choose a release");

    const cascade = await attachedDemo(page, "SelectGroupedDemo");
    await cascade.getByRole("combobox", { name: "Region", exact: true }).click();
    await page.getByRole("option", { name: "Asia-Pacific", exact: true }).click();
    const city = cascade.getByRole("combobox", { name: "City", exact: true });
    await expect(city).toContainText("Osaka");
    await city.click();
    await expect(page.getByRole("listbox").getByRole("option")).toHaveText(["Osaka", "Taipei"]);
    await page.getByRole("option", { name: "Taipei", exact: true }).click();
    await expect(cascade.locator("[data-demo-state]")).toHaveText("apac · taipei");
    await healthy(page, audit);
});

test("V-14 overlay samples open, close and report distinct results", async ({ page, audit }) => {
    await open(page, "components/overlays/");
    const alert = await attachedDemo(page, "AlertDialogDemo");
    const outcome = alert.locator("[data-demo-state]");
    const remove = alert.getByRole("button", { name: "Delete a workspace", exact: true });
    const dialog = page.getByRole("alertdialog", { name: "Delete this workspace?", exact: true });
    await expect(dialog).toBeHidden();
    await remove.click(); await expect(dialog).toBeVisible();
    await page.keyboard.press("Escape"); await expect(dialog).toBeHidden();
    await expect(outcome).toHaveText("Kept the workspace"); await expect(remove).toBeFocused();
    await remove.click(); await dialog.getByRole("button", { name: "Cancel", exact: true }).click();
    await expect(dialog).toBeHidden(); await expect(outcome).toHaveText("Kept the workspace");
    await remove.click(); await dialog.getByRole("button", { name: "Delete", exact: true }).click();
    await expect(dialog).toBeHidden(); await expect(outcome).toHaveText("Workspace deleted");

    const popover = await attachedDemo(page, "PopoverDemo");
    const trigger = popover.getByRole("button", { name: "Choose a density", exact: true });
    const panel = page.getByRole("dialog", { name: "Density", exact: true });
    await expect(panel).toBeHidden();
    await trigger.click(); await expect(panel).toBeVisible();
    await panel.getByRole("radio", { name: "Compact", exact: true }).check();
    await expect(popover.locator("[data-demo-state]")).toHaveText("compact");
    await page.keyboard.press("Escape"); await expect(panel).toBeHidden(); await expect(trigger).toBeFocused();
    await trigger.click(); await panel.getByRole("button", { name: "Done", exact: true }).click(); await expect(panel).toBeHidden();
    await healthy(page, audit);
});

test("V-14 navigation, content, layout, typography and extension samples show their effect", async ({ page, audit }) => {
    test.setTimeout(240_000);
    await open(page, "components/navigation/");
    const accordion = await attachedDemo(page, "AccordionDemo");
    const first = accordion.getByRole("button", { name: "What stays local?", exact: true });
    const second = accordion.getByRole("button", { name: "Does it work without scripts?", exact: true });
    await expect(first).toBeEnabled(); await expect(first).toHaveAttribute("aria-expanded", "true");
    await expect(accordion.getByText("Every sample value stays")).toBeVisible();
    await expect(accordion.getByText("The panels render in static HTML")).toBeHidden();
    await first.focus(); await page.keyboard.press("ArrowDown"); await expect(second).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(accordion.getByText("The panels render in static HTML")).toBeVisible();
    await expect(accordion.getByText("Every sample value stays")).toBeHidden();
    await expect(accordion.locator("[data-demo-state]")).toHaveText("second");
    await second.click(); await expect(accordion.locator("[data-demo-state]")).toHaveText("none");

    await open(page, "components/content/");
    const table = await attachedDemo(page, "TableDemo");
    await expect(table.getByRole("rowheader", { name: "rustify-colors", exact: true })).toBeVisible();
    const scroll = await attachedDemo(page, "ScrollAreaPartsDemo");
    const viewport = scroll.locator('[data-rustify-part="scroll-area-viewport"]');
    await expect.poll(() => viewport.evaluate(node => [node.scrollWidth > node.clientWidth, node.scrollHeight > node.clientHeight])).toEqual([true, true]);
    await expect(scroll.locator('[data-rustify-part="scroll-area-corner"]')).toBeVisible();

    await open(page, "components/layout/");
    const inset = await attachedDemo(page, "InsetDemo");
    const card = inset.locator('[data-name="Box"]');
    const band = inset.locator('[data-name="Inset"]');
    for (const step of ["2", "6"]) {
        await inset.getByLabel("Container padding step").selectOption(step);
        await expect.poll(async () => { const [outer, inner] = [await box(card), await box(band)]; return [Math.round(inner.x - outer.x), Math.round(outer.x + outer.width - inner.x - inner.width), Math.round(inner.y - outer.y)]; }).toEqual([1, 1, 1]);
    }
    const layout = await attachedDemo(page, "LayoutDemo");
    const layoutBox = layout.locator('[data-name="Box"]');
    const padding = await layoutBox.evaluate(node => getComputedStyle(node).paddingLeft);
    await layout.getByLabel("Grid columns").selectOption("4");
    await expect.poll(() => layoutBox.evaluate(node => getComputedStyle(node).paddingLeft)).not.toBe(padding);

    await open(page, "components/typography/");
    const text = await attachedDemo(page, "TextControlsDemo");
    const sample = text.getByText("The same text follows the size");
    const left = (await sample.evaluate(node => { const range = document.createRange(); range.selectNodeContents(node); return range.getBoundingClientRect().x; }));
    await text.getByLabel("Align").selectOption("Right");
    await expect.poll(() => sample.evaluate(node => { const range = document.createRange(); range.selectNodeContents(node); return range.getBoundingClientRect().x; })).toBeGreaterThan(left + 20);

    await open(page, "components/data-table/");
    const grid = (await attached(page, "DataTableDemo")).getByRole("grid", { name: "Example components", exact: true });
    const names = () => grid.getByRole("gridcell").evaluateAll(cells => cells.filter(cell => cell.getAttribute("aria-colindex") === "1" && !cell.classList.contains("hidden")).map(cell => cell.textContent));
    expect(await names()).toEqual(["Button", "Text field", "Data table", "Dialog"]);
    await grid.getByRole("button", { name: /Component/ }).click();
    await expect.poll(names).toEqual(["Button", "Data table", "Dialog", "Text field"]);
    await grid.getByRole("button", { name: /Component/ }).click();
    await expect.poll(names).toEqual(["Text field", "Dialog", "Data table", "Button"]);
    await grid.getByRole("gridcell").filter({ hasText: /^Dialog$/ }).focus(); await page.keyboard.press("Space");
    await expect(page.locator("[data-demo-state]")).toHaveText("Dialog");
    await page.keyboard.press("Enter"); await expect(page.getByText("Activated: Dialog", { exact: true })).toBeVisible();

    await open(page, "components/extensions/");
    const splitter = await attachedDemo(page, "SplitterDemo");
    const panels = splitter.locator('[data-name="Grid"] > div:not([data-rustify-part])');
    const widths = () => panels.evaluateAll(nodes => nodes.map(node => Math.round(node.getBoundingClientRect().width)));
    expect(await widths()).toEqual([96, 96, 96]);
    const divider = splitter.getByRole("separator", { name: "Resize panels 1", exact: true });
    const [sources, divide] = [await box(panels.first()), await box(divider)];
    expect(Math.abs(divide.x + divide.width / 2 - (sources.x + sources.width))).toBeLessThanOrEqual(2);
    await divider.press("ArrowRight"); await expect.poll(widths).toEqual([112, 80, 96]);
    // One pointer sequence in the page, as tests/browser/p2-workspace.spec.ts drags its dividers.
    await divider.evaluate(node => {
        const box = node.getBoundingClientRect(); const from = box.left + box.width / 2;
        for (const [type, x] of [["pointerdown", from], ["pointermove", from - 32], ["pointerup", from - 32]] as const)
            node.dispatchEvent(new PointerEvent(type, { bubbles: true, clientX: x, clientY: box.top + box.height / 2, pointerId: 1, isPrimary: true }));
    });
    await expect.poll(widths).toEqual([80, 112, 96]);
    await splitter.getByRole("button", { name: "Reset columns", exact: true }).click(); await expect.poll(widths).toEqual([96, 96, 96]);
    await healthy(page, audit);
});

test("V-15 from the home preview through Colors, Primitives and into the composed public Studio", async ({ page, audit, request }) => {
    test.setTimeout(480_000);
    await page.setViewportSize({ width: 1280, height: 900 });
    await open(page, "./");
    const preview = await attachedDemo(page, "ButtonDemo");
    await attachedDemo(page, "SegmentedControlDemo");
    await preview.getByRole("button", { name: "Increment", exact: true }).click();
    await expect(preview.locator("[data-demo-state]")).toHaveText("1");
    await page.getByRole("link", { name: "Colors", exact: true }).first().click();
    await expect(page).toHaveURL(url("colors/"));
    const colorField = await attachedDemo(page, "ColorsDemo");
    await colorField.getByLabel("Family").selectOption("mint");
    await expect(colorField.locator("[data-demo-state]")).toContainText("mint · light · solid · srgb");
    await colorField.getByLabel("Mode").selectOption("dark");
    await expect(colorField.locator("[data-demo-state]")).toContainText("mint · dark");
    await expect(colorField.locator("svg[aria-hidden='true'] rect").first()).toHaveAttribute("fill", /^#/);
    await colorField.getByLabel("Style").selectOption("alpha");
    await colorField.getByLabel("Display P3").check();
    await colorField.getByRole("link", { name: "mint-dark-alpha-p3.css", exact: true }).click();
    await expect.poll(() => new URL(page.url()).hash).toBe("#css-mint-dark-alpha-p3");
    const matching = page.locator("#css-mint-dark-alpha-p3");
    await expect(matching).toHaveAttribute("download", "");
    const css = await request.get(await matching.evaluate(node => (node as HTMLAnchorElement).href));
    expect(css.status()).toBe(200);
    expect(css.headers()["content-type"]).toContain("text/css");
    expect(await css.text()).toContain("color(display-p3");
    await expect(colorField.getByText("white-alpha", { exact: true }).locator("xpath=..").locator("li")).toHaveCount(12);
    for (const label of ["light alpha P3", "dark alpha"]) await expect(page.locator("ul.rd-color-downloads").getByRole("link", { name: label, exact: true })).toHaveCount(31);
    const black = page.locator("#css-black-alpha-p3");
    await expect(black).toHaveText("alpha P3");
    expect((await request.get(await black.evaluate(node => (node as HTMLAnchorElement).href))).headers()["content-type"]).toContain("text/css");
    await page.getByRole("link", { name: "Primitives", exact: true }).first().click();
    await expect(page).toHaveURL(url("primitives/"));
    const raw = await attachedDemo(page, "PrimitivesDemo");
    const rawBox = raw.getByRole("checkbox", { name: "Raw subscribe", exact: true });
    const themedBox = raw.getByRole("checkbox", { name: "Themed subscribe", exact: true });
    await expect(rawBox).toHaveJSProperty("tagName", "INPUT");
    await rawBox.check(); await expect(raw.locator("[data-demo-state]")).toHaveText("checked"); await expect(themedBox).toBeChecked();
    await themedBox.uncheck(); await expect(rawBox).not.toBeChecked(); await expect(raw.locator("[data-demo-state]")).toHaveText("unchecked");
    await expect(raw.locator('[data-rustify-part="visually-hidden"]')).toHaveText(" to release notes");
    const direction = await attachedDemo(page, "DirectionDemo");
    const slider = direction.getByRole("slider", { name: "Amount", exact: true });
    await slider.press("ArrowRight"); await expect(direction.locator("[data-demo-state]")).toHaveText("ltr · amount 40");
    await direction.getByRole("button").click();
    await expect(direction.locator('[data-rustify-part="direction-provider"]')).toHaveAttribute("dir", "rtl");
    await slider.press("ArrowRight"); await expect(direction.locator("[data-demo-state]")).toHaveText("rtl · amount 30");
    await page.getByRole("link", { name: "Theme Studio", exact: true }).first().click();
    await expect(page.locator("h1")).toHaveText("Theme Studio");
    await page.getByRole("link", { name: "Open Theme Studio", exact: true }).click();
    await expect(page).toHaveURL(new URL("theme-studio/", url("./")).href);
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: 60_000 });
    const manifest = await (await request.get(new URL("theme-studio/build-manifest.json", url("./")).href)).json();
    expect(manifest).toMatchObject({ example: "theme-studio", profile: "release", base: `${base()}theme-studio/` });
    await page.getByTestId("renderer-dom").click();
    await page.getByTestId("preview-scene").click();
    await page.getByTestId("preview-scene-list").getByRole("option", { name: "Components", exact: true }).click();
    await expect(page.getByTestId("dom-preview").getByTestId("component-gallery")).toBeVisible();
    await page.getByTestId("inspector-toggle").click();
    await expect(page.getByTestId("inspector-panel")).toBeVisible();
    await expect(page.locator("a.studio-back")).toHaveAttribute("href", "../");
    expect(audit.requests.some(address => new URL(address).pathname.includes("theme-studio/"))).toBe(true);
    expect(audit.requests.filter(address => { const resource = new URL(address); return resource.protocol !== "data:" && resource.protocol !== "blob:" && (resource.origin !== new URL(url("./")).origin || !resource.pathname.startsWith(base())); })).toEqual([]);
    expect(audit.failures).toEqual([]); expect(audit.pageErrors).toEqual([]); expect(audit.console).toEqual([]); expect(audit.csp).toEqual([]);
});

// Every registered route (home and the 24 categories), the Markdown quick
// start and a deep missing address, as the projected registry lists them.
const gapsRoutes = [...declaredPages.map(page => page.route.slice(1)), "quickstart/", "__gaps_missing__/deeper/"];

for (const colorScheme of ["light", "dark"] as const) for (const width of [1280, 375]) {
    test(`gaps a11y: every official route at rest has no serious or critical axe violation, ${colorScheme} ${width}px`, async ({ page, audit }) => {
        test.setTimeout(600_000);
        expect(gapsRoutes).toHaveLength(declaredPages.length + 2);
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
