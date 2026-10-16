import { expect, Page, test as playwright } from "@playwright/test";
import { atRest, attachScans, buildId, scan, Scan, serious } from "./a11y";

const test = playwright.extend<{ searchAudit: unknown }>({ searchAudit: [async ({ page }, use, info) => {
    const audit = { requests: [] as string[], failed: [] as string[], errors: [] as string[], csp: [] as string[], pageErrors: [] as string[] };
    const recovering = info.title.includes("recovery");
    const searchAsset = (value: string) => /\/assets\/search(?:-worker)?\.[^/]+\.(?:js|json)(?:[?#]|$)/.test(value);
    await page.exposeFunction("searchAuditCsp", (value: string) => audit.csp.push(value));
    await page.addInitScript(() => document.addEventListener("securitypolicyviolation", event => (window as any).searchAuditCsp(`${event.violatedDirective} ${event.blockedURI}`)));
    page.on("request", request => audit.requests.push(request.url()));
    page.on("requestfailed", request => { if (!(recovering && searchAsset(request.url()))) audit.failed.push(`${request.url()} ${request.failure()?.errorText}`); });
    page.on("pageerror", error => audit.pageErrors.push(String(error)));
    page.on("console", message => {
        if (message.type() !== "error") return;
        if (message.text().startsWith("Failed to load resource:") && ((recovering && searchAsset(message.location().url)) || message.location().url.includes("__search_missing__"))) return;
        audit.errors.push(`${message.location().url} ${message.text()}`);
    });
    try { await use(audit); }
    finally {
        await info.attach("search-browser-audit.json", { body: JSON.stringify(audit, null, 2), contentType: "application/json" });
        expect(audit.csp).toEqual([]);
        expect(audit.pageErrors).toEqual([]);
        expect(audit.failed).toEqual([]);
        expect(audit.errors).toEqual([]);
        const target = new URL(String(info.project.use.baseURL));
        expect(audit.requests.filter(value => { const url = new URL(value); return url.origin !== target.origin || !url.pathname.startsWith(target.pathname) || /makepad|\.wasm(?:[?#]|$)/i.test(value); })).toEqual([]);
    }
}, { auto: true }] });

const base = () => String(test.info().project.use.baseURL);
const url = (path: string) => new URL(path, base()).href;
const dialog = (page: Page) => page.locator("dialog[data-rd-search]");
const input = (page: Page) => dialog(page).getByRole("combobox");
const open = async (page: Page, path = "") => {
    await page.goto(url(path));
    await page.locator("[data-rd-search-open]").click();
    await expect(input(page)).toBeFocused();
};

for (const fault of ["HTTP", "network", "module-shape"]) test(`recovery loads a fresh module URL after first ${fault} failure and retains the input`, async ({ page }) => {
    let failed = false;
    const modules: string[] = [];
    await page.route(/\/assets\/search\.[a-f0-9]+\.js(?:\?|$)/, async route => {
        modules.push(route.request().url());
        if (!failed) {
            failed = true;
            if (fault === "network") await route.abort("failed");
            else if (fault === "module-shape") await route.fulfill({ contentType: "text/javascript", body: "export default 0;" });
            else await route.fulfill({ status: 503, body: "unavailable", contentType: "text/javascript" });
        }
        else await route.continue();
    });
    const csp: string[] = [];
    await page.exposeFunction("searchCsp", (value: string) => csp.push(value));
    await page.addInitScript(() => document.addEventListener("securitypolicyviolation", event => (window as any).searchCsp(event.violatedDirective)));
    await open(page);
    await input(page).fill("CustomOnlyToken");
    await expect(dialog(page)).toHaveAttribute("data-state", "error");
    await expect(dialog(page).getByRole("alert")).toContainText("try");
    if (fault === "HTTP" && test.info().project.name === "docs-search-subpath") await page.screenshot({ path: test.info().outputPath("search-error-1280.png"), fullPage: true });
    await dialog(page).getByRole("button", { name: "Retry", exact: true }).click();
    await expect(dialog(page)).toHaveAttribute("data-state", "results");
    await expect(input(page)).toHaveValue("CustomOnlyToken");
    await expect(dialog(page).getByRole("option")).toHaveCount(1);
    expect(modules).toHaveLength(2);
    expect(new URL(modules[0]).search).toBe("");
    expect(new URL(modules[1]).searchParams.get("rd_search_attempt")).toBe("1");
    expect(new URL(modules[1]).pathname).toBe(new URL(modules[0]).pathname);
    await input(page).fill("ReachableCustomToken");
    await expect(dialog(page).getByRole("option")).toHaveAttribute("href", /#custom-section$/);
    expect(csp).toEqual([]);
});

test("recovery keeps the latest input through delayed JSON and ignores closed sessions", async ({ page }) => {
    let release!: () => void;
    const held = new Promise<void>(resolve => release = resolve);
    let jsonRequests = 0;
    await page.context().route(/\/assets\/search\.en\.[a-f0-9]+\.json$/, async route => {
        jsonRequests++;
        if (jsonRequests === 1) await held;
        await route.continue();
    });
    await open(page);
    await input(page).fill("OldQuery");
    await input(page).fill("CustomOnlyToken");
    await expect(dialog(page)).toHaveAttribute("data-state", "loading");
    await expect.poll(() => jsonRequests).toBe(1);
    if (test.info().project.name === "docs-search-subpath") await page.screenshot({ path: test.info().outputPath("search-loading-1280.png"), fullPage: true });
    await page.keyboard.press("Escape");
    await expect(page.locator("[data-rd-search-open]")).toBeFocused();
    release();
    await page.locator("[data-rd-search-open]").click();
    await expect(dialog(page)).toHaveAttribute("data-state", "results");
    await expect(input(page)).toHaveValue("CustomOnlyToken");
    await expect(dialog(page).getByRole("option")).toHaveCount(1);
    expect(jsonRequests).toBe(2);
});

test("recovery retries JSON failures with a new Worker and the successful module", async ({ page }) => {
    let indexRequests = 0;
    let modules = 0;
    page.on("request", request => { if (/\/search\.[a-f0-9]+\.js(?:\?|$)/.test(request.url())) modules++; });
    await page.context().route(/\/assets\/search\.en\.[a-f0-9]+\.json$/, async route => {
        indexRequests++;
        if (indexRequests === 1) await route.fulfill({ json: { schema_version: 99, pages: [] } });
        else await route.continue();
    });
    await open(page);
    await input(page).fill("CustomOnlyToken");
    await expect(dialog(page)).toHaveAttribute("data-state", "error");
    await dialog(page).getByRole("button", { name: "Retry", exact: true }).click();
    await expect(dialog(page)).toHaveAttribute("data-state", "results");
    expect(indexRequests).toBe(2);
    expect(modules).toBe(1);
    await expect(input(page)).toHaveValue("CustomOnlyToken");
});

test("recovery leaves a timed-out import inert after retry succeeds", async ({ page }) => {
    await page.clock.install();
    let release!: () => void;
    const held = new Promise<void>(resolve => release = resolve);
    let moduleRequests = 0;
    let workers = 0;
    page.on("worker", () => workers++);
    await page.route(/\/assets\/search\.[a-f0-9]+\.js(?:\?|$)/, async route => {
        moduleRequests++;
        if (moduleRequests === 1) await held;
        await route.continue();
    });
    await open(page);
    await input(page).fill("CustomOnlyToken");
    await expect.poll(() => moduleRequests).toBe(1);
    await page.clock.fastForward(10_001);
    await expect(dialog(page)).toHaveAttribute("data-state", "error");
    await dialog(page).getByRole("button", { name: "Retry", exact: true }).click();
    await expect(dialog(page)).toHaveAttribute("data-state", "results");
    expect(workers).toBe(1);
    release();
    await expect.poll(() => moduleRequests).toBe(2);
    await input(page).fill("ReachableCustomToken");
    await expect(dialog(page).getByRole("option")).toHaveAttribute("href", /#custom-section$/);
    expect(workers).toBe(1);
});

test("recovery abandons an import closed before load and opens a fresh session", async ({ page }) => {
    let release!: () => void;
    const held = new Promise<void>(resolve => release = resolve);
    const urls: string[] = [];
    let workers = 0;
    page.on("worker", () => workers++);
    await page.route(/\/assets\/search\.[a-f0-9]+\.js(?:\?|$)/, async route => {
        urls.push(route.request().url());
        if (urls.length === 1) await held;
        await route.continue();
    });
    await open(page);
    await input(page).fill("CustomOnlyToken");
    await expect.poll(() => urls.length).toBe(1);
    await page.keyboard.press("Escape");
    await expect(page.locator("[data-rd-search-open]")).toBeFocused();
    await page.locator("[data-rd-search-open]").click();
    await expect(dialog(page)).toHaveAttribute("data-state", "results");
    release();
    expect(new URL(urls[1]).searchParams.get("rd_search_attempt")).toBe("1");
    await input(page).fill("ReachableCustomToken");
    await expect(dialog(page).getByRole("option")).toHaveAttribute("href", /#custom-section$/);
    expect(workers).toBe(1);
});

for (const fault of ["error", "messageerror", "timeout", "query-timeout"] as const) {
    test(`recovery handles Worker ${fault} and keeps input for a successful retry`, async ({ page }) => {
        await page.clock.install();
        await page.addInitScript(fault => {
            const Native = Worker;
            let first = true;
            window.Worker = class extends Native {
                postMessage(message: any) {
                    if (first && message.type === (fault === "query-timeout" ? "query" : "init")) {
                        first = false;
                        (window as any).searchFaultTriggered = true;
                        if (fault === "error" || fault === "messageerror") queueMicrotask(() => this.dispatchEvent(new Event(fault, { cancelable: true })));
                        return;
                    }
                    super.postMessage(message);
                }
            } as typeof Worker;
        }, fault);
        await open(page);
        await input(page).fill("CustomOnlyToken");
        if (fault === "timeout" || fault === "query-timeout") {
            await expect.poll(() => page.evaluate(() => (window as any).searchFaultTriggered)).toBe(true);
            await page.clock.fastForward(10_001);
        }
        await expect(dialog(page)).toHaveAttribute("data-state", "error");
        await dialog(page).getByRole("button", { name: "Retry", exact: true }).click();
        await expect(dialog(page)).toHaveAttribute("data-state", "results");
        await expect(input(page)).toHaveValue("CustomOnlyToken");
    });
}

test("recovery ignores old query and instance messages and defers composing input", async ({ page }) => {
    await page.addInitScript(() => {
        const Native = Worker;
        (window as any).searchMessages = [];
        window.Worker = class extends Native {
            constructor(url: string | URL, options?: WorkerOptions) { super(url, options); (window as any).searchWorker = this; }
            postMessage(message: any) { (window as any).searchMessages.push(message); super.postMessage(message); }
        } as typeof Worker;
    });
    await open(page);
    await expect(dialog(page)).toHaveAttribute("data-state", "idle");
    await input(page).dispatchEvent("compositionstart");
    await input(page).fill("OldQuery");
    await input(page).fill("CustomOnlyToken");
    expect(await page.evaluate(() => (window as any).searchMessages.filter((message: any) => message.type === "query").length)).toBe(0);
    await input(page).dispatchEvent("compositionend");
    await expect(dialog(page)).toHaveAttribute("data-state", "results");
    await page.evaluate(() => {
        const state = window as any;
        const latest = state.searchMessages.filter((message: any) => message.type === "query").at(-1);
        for (const identity of [{ instance: latest.instance, id: latest.id - 1 }, { instance: latest.instance - 1, id: latest.id }]) {
            state.searchWorker.dispatchEvent(new MessageEvent("message", { data: { type: "results", ...identity, query: latest.query, offset: 0, total: 0, has_more: false, groups: [] } }));
        }
    });
    await expect(dialog(page)).toHaveAttribute("data-state", "results");
    await expect(dialog(page).getByRole("option")).toHaveCount(1);
});

for (const phase of ["ready", "module-loading"]) test(`recovery settles a same-turn close and reopen during ${phase}`, async ({ page }) => {
    let release!: () => void;
    const held = new Promise<void>(resolve => release = resolve);
    const imports: string[] = [];
    if (phase === "module-loading") await page.route(/\/assets\/search\.[a-f0-9]+\.js(?:\?|$)/, async route => {
        imports.push(route.request().url());
        if (imports.length === 1) await held;
        await route.continue();
    });
    try {
        await open(page);
        if (phase === "ready") {
            await input(page).fill("CustomOnlyToken");
            await expect(dialog(page)).toHaveAttribute("data-state", "results");
        } else await expect.poll(() => imports.length).toBe(1);
        await page.evaluate(() => {
            const dialog = document.querySelector<HTMLDialogElement>("dialog[data-rd-search]")!;
            dialog.close();
            document.querySelector<HTMLButtonElement>("[data-rd-search-open]")!.click();
            const input = dialog.querySelector<HTMLInputElement>("input")!;
            input.value = "ReachableCustomToken";
            input.dispatchEvent(new InputEvent("input", { bubbles: true }));
        });
        await expect(dialog(page)).toHaveAttribute("data-state", "results");
        await expect(dialog(page).getByRole("option")).toHaveCount(1);
        await expect(dialog(page).getByRole("option")).toHaveAttribute("href", /#custom-section$/);
        if (phase === "module-loading") expect(new URL(imports[1]).searchParams.get("rd_search_attempt")).toBe("1");
        release();
        await input(page).fill("CustomOnlyToken");
        await expect(dialog(page)).toHaveAttribute("data-state", "results");
        await expect(dialog(page).getByRole("option")).toHaveCount(1);
    } finally { release(); }
});

test("search is lazy, focuses synchronously and distinguishes idle, results and no matches", async ({ page }) => {
    const requests: string[] = [];
    page.on("request", request => requests.push(request.url()));
    await page.goto(url("page-0001/"));
    expect(requests.filter(value => /\/assets\/search/.test(value))).toEqual([]);
    const focus = await page.evaluate(() => {
        document.querySelector<HTMLButtonElement>("[data-rd-search-open]")!.click();
        return document.activeElement?.matches("[data-rd-search-input]");
    });
    expect(focus).toBe(true);
    await expect(dialog(page)).toHaveAttribute("data-state", "idle");
    await input(page).fill("ThemeScope");
    await expect(dialog(page)).toHaveAttribute("data-state", "results");
    await expect(dialog(page).getByRole("option").first().locator(".rd-search-title")).toHaveText("ThemeScope reference");
    await expect(dialog(page).getByRole("option").first().locator("mark").first()).toHaveText("ThemeScope");
    await input(page).fill("AbsolutelyNoMatchingWordHere");
    await expect(dialog(page)).toHaveAttribute("data-state", "empty");
    await expect(dialog(page).getByRole("option")).toHaveCount(0);
    await expect(dialog(page).locator("[data-rd-search-retry]")).toBeHidden();
    await input(page).fill("  ");
    await expect(dialog(page)).toHaveAttribute("data-state", "idle");
    await expect(input(page)).toHaveAttribute("aria-expanded", "false");
    expect(requests.filter(value => /\/assets\/search\.[a-f0-9]+\.js/.test(value))).toHaveLength(1);
    expect(requests.filter(value => /\/assets\/search-worker\.[a-f0-9]+\.js/.test(value))).toHaveLength(1);
    expect(requests.filter(value => /\/assets\/search\.en\.[a-f0-9]+\.json/.test(value))).toHaveLength(1);
});

test("keyboard selection, more results and ordinary links retain the full page count", async ({ page }) => {
    await page.goto(url(""));
    const trigger = page.locator(".rd-brand");
    await trigger.focus();
    await page.keyboard.press("Control+k");
    await expect(input(page)).toBeFocused();
    await input(page).fill("SearchCorpusToken");
    await expect(dialog(page).getByRole("option")).toHaveCount(20);
    await expect(dialog(page).locator("[data-rd-search-count]")).toHaveText("98 results");
    const initial = await input(page).getAttribute("aria-activedescendant");
    await page.keyboard.press("ArrowDown");
    const selected = await input(page).getAttribute("aria-activedescendant");
    expect(selected).not.toBe(initial);
    await expect(page.locator(`[id='${selected}']`)).toHaveAttribute("aria-selected", "true");
    const requests: string[] = [];
    page.context().on("request", request => requests.push(request.url()));
    await page.keyboard.press("Tab");
    await expect(dialog(page).getByRole("button", { name: "Show more", exact: true })).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(dialog(page).getByRole("option")).toHaveCount(40);
    await atRest(page);
    const more = dialog(page).getByRole("button", { name: "Show more", exact: true });
    await expect(more).toBeFocused();
    expect(await more.evaluate(node => {
        const style = getComputedStyle(node);
        return style.outlineStyle !== "none" && parseFloat(style.outlineWidth) >= 2;
    })).toBe(true);
    await page.keyboard.press("Shift+Tab");
    await expect(input(page)).toBeFocused();
    await page.keyboard.press("Tab");
    await expect(more).toBeFocused();
    for (const total of [60, 80, 98]) {
        await page.keyboard.press("Enter");
        await expect(dialog(page).getByRole("option")).toHaveCount(total);
        await atRest(page);
        await expect(total === 98 ? input(page) : more).toBeFocused();
        await expect(input(page)).toHaveAttribute("aria-activedescendant", selected!);
    }
    await expect(more).toBeHidden();
    await expect(input(page)).toHaveAttribute("aria-activedescendant", selected!);
    await expect(dialog(page).locator("[data-rd-search-count]")).toHaveText("98 results");
    expect(requests).toEqual([]);
    await page.keyboard.press("Escape");
    await expect(trigger).toBeFocused();
    await page.keyboard.press("Meta+k");
    await input(page).fill("ReachableCustomToken");
    await expect(dialog(page).getByRole("option")).toHaveCount(1);
    await expect(dialog(page).getByRole("option")).toHaveAttribute("href", /#custom-section$/);
    await page.keyboard.press("Enter");
    await expect(page).toHaveURL(url("#custom-section"));
    await expect(dialog(page)).not.toBeVisible();
    await expect(page.locator("h2#custom-section")).toBeVisible();
});

test("search totals use singular and plural labels without changing the live count", async ({ page }) => {
    await open(page);
    for (const [query, total] of [["CustomOnlyToken", "1 result"], ["ThemeScope", "2 results"], ["AbsolutelyNoMatchingWordHere", "0 results"]]) {
        await input(page).fill(query);
        await expect(dialog(page).locator("[data-rd-search-count]")).toHaveText(total);
    }
});

test("Chinese true content and fallback search retain the target language and source labels", async ({ page }) => {
    await open(page, "zh/page-0001/");
    await expect(dialog(page)).toHaveAttribute("data-rd-search-lang", "zh");
    await expect(input(page)).toHaveAccessibleName("搜索文档");
    await input(page).fill("主题切换");
    await expect(dialog(page)).toHaveAttribute("data-state", "results");
    await expect(dialog(page).getByRole("option").first().locator(".rd-search-title")).toHaveText("主题切换");
    await input(page).fill("FallbackOnlyToken");
    const result = dialog(page).getByRole("option");
    await expect(result).toHaveCount(1);
    await expect(dialog(page).locator("[data-rd-search-count]")).toHaveText("1 个结果");
    await expect(result).toHaveAttribute("href", /\/zh\/page-0051\/$/);
    await expect(result.locator(".rd-search-title")).toHaveAttribute("lang", "en");
    await expect(result.locator(".rd-search-badge")).toHaveText("zh");
    await expect(result.locator(".rd-search-source")).toContainText("en");
    await page.keyboard.press("Enter");
    await expect(page).toHaveURL(url("zh/page-0051/"));
    await expect(page.locator("html")).toHaveAttribute("lang", "zh");
    await expect(page.locator(".rd-untranslated")).toBeVisible();
    for (const scheme of ["light", "dark"] as const) {
        await page.emulateMedia({ colorScheme: scheme });
        await page.locator(`[data-rd-mode="${scheme}"]`).click();
        await expect(page.locator(".rd-untranslated")).toHaveCSS("background-color", scheme === "light" ? "rgb(242, 244, 247)" : "rgb(22, 27, 38)");
    }
});

test("a deep 404 searches its selected language under header and meta CSP", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await open(page, "zh/__search_missing__/deep/");
    await expect(dialog(page)).toHaveAttribute("data-rd-search-lang", "zh");
    await input(page).fill("主题切换");
    await expect(dialog(page).getByRole("option").first().locator(".rd-search-title")).toHaveText("主题切换");
    if (test.info().project.name === "docs-search-subpath") await page.screenshot({ path: test.info().outputPath("search-404-375.png"), fullPage: true });
    await page.keyboard.press("Escape");
    await expect(page.locator("[data-rd-search-open]")).toBeFocused();
    const policy = await page.locator('meta[http-equiv="Content-Security-Policy"]').getAttribute("content");
    expect(policy).toContain("worker-src 'self'");
    expect(policy).not.toContain("'unsafe-inline'");
});

test("without scripts the document, source language and ordinary navigation remain readable", async ({ browser }) => {
    const context = await browser.newContext({ javaScriptEnabled: false, viewport: { width: 375, height: 812 } });
    try {
        const page = await context.newPage();
        const requests: string[] = [];
        page.on("request", request => requests.push(request.url()));
        await page.goto(url("zh/page-0001/"));
        await expect(page.getByRole("heading", { name: "主题切换", exact: true })).toBeVisible();
        await expect(page.locator("[data-rd-search-open]")).toBeDisabled();
        await expect(page.getByText("启用 JavaScript 后可搜索。你仍可通过导航链接阅读文档。", { exact: true })).toBeVisible();
        await page.locator(".rd-sidebar-narrow summary").click();
        await page.locator(".rd-sidebar-narrow a[href$='/zh/page-0002/']").click();
        await expect(page).toHaveURL(url("zh/page-0002/"));
        expect(requests.filter(value => /\/assets\/search|\.wasm(?:[?#]|$)/.test(value))).toEqual([]);
        if (test.info().project.name === "docs-search-subpath") {
            await page.goto(url("zh/"));
            await page.screenshot({ path: test.info().outputPath("search-no-script-375.png"), fullPage: true });
        }
    } finally { await context.close(); }
});

test("result text and query text cannot introduce HTML or external navigation", async ({ page }) => {
    await page.context().route(/\/assets\/search\.en\.[a-f0-9]+\.json$/, async route => {
        const response = await route.fetch();
        const data = await response.json();
        const item = data.pages[0];
        item.title = '<img src=x onerror="window.searchInjected=true"> InjectionSentinel';
        item.sections = [{ anchor: null, path: [item.title], text: '<script>InjectionSentinel</script> & original text' }];
        data.pages = [item];
        await route.fulfill({ json: data });
    });
    await open(page);
    await input(page).fill("InjectionSentinel");
    const result = dialog(page).getByRole("option");
    await expect(result).toHaveCount(1);
    await expect(result.locator(".rd-search-title")).toContainText("<img");
    await expect(result.locator("img,script")).toHaveCount(0);
    expect(await page.evaluate(() => (window as any).searchInjected)).toBeUndefined();
    await input(page).fill("<img src=x onerror=alert(1)>");
    await expect(dialog(page)).toHaveAttribute("data-state", "empty");
    await expect(dialog(page).locator("img,script")).toHaveCount(0);
});

test("light, dark, narrow and text reflow states retain focus, contrast and visible controls", async ({ page }) => {
    test.setTimeout(180_000);
    const scans: Scan[] = [];
    const id = await buildId(page, base());
    for (const scheme of ["light", "dark"] as const) for (const width of [1280, 375, 320]) {
        await page.setViewportSize({ width, height: width === 1280 ? 900 : 812 });
        await page.emulateMedia({ colorScheme: scheme, reducedMotion: "reduce" });
        await open(page, "zh/");
        await input(page).fill("theme");
        await expect(dialog(page)).toHaveAttribute("data-state", "results");
        await atRest(page);
        expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
        const bounds = await dialog(page).boundingBox();
        expect(bounds!.width).toBe(Math.min(576, width - 32));
        expect(bounds!.x).toBeGreaterThanOrEqual(0);
        expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(width);
        await expect(input(page)).toBeFocused();
        expect(await input(page).evaluate(node => { const style = getComputedStyle(node); return style.outlineStyle !== "none" && parseFloat(style.outlineWidth) >= 2; })).toBe(true);
        scans.push(await scan(page, "zh/", `search-${scheme}-${width}`));
        if (test.info().project.name === "docs-search-subpath") await page.screenshot({ path: test.info().outputPath(`search-${scheme}-${width}.png`), fullPage: true });
        await page.keyboard.press("Escape");
    }
    await page.setViewportSize({ width: 640, height: 812 });
    await page.emulateMedia({ colorScheme: "light", reducedMotion: "reduce" });
    await open(page, "zh/");
    await page.evaluate(() => document.documentElement.style.fontSize = "200%");
    await input(page).fill("theme");
    await expect(dialog(page)).toHaveAttribute("data-state", "results");
    await expect(dialog(page).getByRole("button", { name: "关闭", exact: true })).toBeInViewport();
    await expect(input(page)).toBeInViewport();
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
    scans.push(await scan(page, "zh/", "search-200-percent-layout-text-emulation"));
    if (test.info().project.name === "docs-search-subpath") await page.screenshot({ path: test.info().outputPath("search-text-200.png"), fullPage: true });
    await attachScans(test.info(), "search-a11y.json", scans, id);
    expect(serious(scans)).toEqual([]);
});
