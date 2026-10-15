import { expect, Locator, Page, Response, Route, test as playwright } from "@playwright/test";
import { readFileSync } from "node:fs";

type Metadata = { key: string; seed: string; target: { type: string; name: string }; environment: { lang: string; base: string } };
type Original = { key: string; root: Element; nodes: Node[]; ids: [Element, string][]; source: Element | null; controls: { node: HTMLInputElement; value: string; checked: boolean }[] };
type ProbeWindow = Window & {
    __i18n_csp?: string[];
    __i18n_originals?: Original[];
    __rustify_docs_islands?: { parsed: Promise<Element[]>; state(): string; failure(): string | null; stats(): unknown };
};
type Audit = {
    requests: string[];
    failures: string[];
    responses: { url: string; status: number; document: boolean }[];
    console: { text: string; url: string }[];
    pageErrors: string[];
    document404: Set<string>;
};

const test = playwright.extend<{ audit: Audit }>({
    audit: [async ({ page }, use, info) => {
        const audit = await watch(page);
        await use(audit);
        const browser = await page.evaluate(() => {
            const probe = window as ProbeWindow;
            return { csp: probe.__i18n_csp ?? [], state: probe.__rustify_docs_islands?.state(), failure: probe.__rustify_docs_islands?.failure() };
        });
        await info.attach("i18n-islands-audit.json", { body: JSON.stringify({ ...audit, document404: [...audit.document404], browser }, null, 2), contentType: "application/json" });
    }, { auto: true }],
});

const url = (path: string) => new URL(path, String(test.info().project.use.baseURL)).href;
const base = () => new URL(url("./")).pathname;
const metaHost = () => test.info().project.name === "components-i18n-meta";
const islands = (page: Page) => page.locator("[data-rustify-island]");
const canvas = (island: Locator) => island.locator("fieldset[data-rd-island-controls]");
const policy = "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'";
const frenchNotice = "Cette page n’est pas encore traduite. Le contenu de la langue par défaut est affiché.";

async function watch(page: Page): Promise<Audit> {
    const audit: Audit = { requests: [], failures: [], responses: [], console: [], pageErrors: [], document404: new Set() };
    await page.addInitScript(() => {
        (window as ProbeWindow).__i18n_csp = [];
        document.addEventListener("securitypolicyviolation", event => (window as ProbeWindow).__i18n_csp!.push(`${event.violatedDirective} ${event.blockedURI}`));
    });
    page.on("request", request => audit.requests.push(request.url()));
    page.on("requestfailed", request => audit.failures.push(`${request.url()} ${request.failure()?.errorText}`));
    page.on("response", response => {
        if (response.status() >= 400) audit.responses.push({ url: response.url(), status: response.status(), document: response.request().resourceType() === "document" });
    });
    page.on("console", message => {
        if (message.type() === "error") audit.console.push({ text: message.text(), url: message.location().url });
    });
    page.on("pageerror", error => audit.pageErrors.push(String(error)));
    return audit;
}

async function healthy(page: Page, audit: Audit) {
    expect(await page.evaluate(() => (window as ProbeWindow).__i18n_csp ?? [])).toEqual([]);
    expect(audit.failures).toEqual([]);
    expect(audit.responses.filter(response => !(response.document && response.status === 404 && audit.document404.has(response.url)))).toEqual([]);
    expect(audit.console.filter(message => !(audit.document404.has(message.url)
        && message.text === "Failed to load resource: the server responded with a status of 404 (Not Found)"))).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    expect(audit.requests.filter(request => new URL(request).origin !== new URL(url("./")).origin)).toEqual([]);
    for (const request of audit.requests.filter(request => /\/assets\//.test(new URL(request).pathname))) {
        expect(new URL(request).pathname.startsWith(`${base()}assets/`), request).toBe(true);
    }
}

async function documentPolicy(page: Page, response: Response | null, status: number) {
    expect(response).not.toBeNull();
    expect(response!.status()).toBe(status);
    await expect(page.locator('meta[http-equiv="Content-Security-Policy"]')).toHaveAttribute("content", policy);
    if (metaHost()) expect(response!.headers()["content-security-policy"]).toBeUndefined();
    else expect(response!.headers()["content-security-policy"]).toBe(`${policy}; frame-ancestors 'none'`);
}

async function metadata(page: Page, lang: string, count = 2) {
    await expect(islands(page)).toHaveCount(count);
    const records = await islands(page).evaluateAll(blocks => blocks.map(block => {
        const data = JSON.parse(block.getAttribute("data-rustify-island-metadata")!) as Metadata;
        const root = block.querySelector("fieldset[data-rd-island-controls]")!.firstElementChild!;
        return { data, key: block.getAttribute("data-rustify-island"), scope: root.getAttribute("data-rustify-scope"), mounted: root.getAttribute("data-rustify-mounted") };
    }));
    expect(new Set(records.map(record => record.data.key)).size).toBe(count);
    expect(new Set(records.map(record => record.data.seed)).size).toBe(count);
    expect(records.map(record => record.data.target.name).sort()).toEqual(["CounterDemo", "ThemeProbeDemo"].sort());
    for (const record of records) {
        expect(record.key).toBe(record.data.key);
        expect(record.scope).toBe(record.data.seed);
        expect(record.data.environment.lang).toBe(lang);
        expect(record.data.environment.base).toBe(base());
        if (record.mounted !== null) expect(record.mounted).toBe(record.data.seed);
    }
    return records;
}

async function captureOriginals(page: Page) {
    const summary = await page.evaluate(() => {
        const originals = [...document.querySelectorAll("[data-rustify-island]")].map(block => {
            const root = block.querySelector("fieldset[data-rd-island-controls]")!.firstElementChild!;
            const nodes: Node[] = [root];
            const walker = document.createTreeWalker(root, NodeFilter.SHOW_ALL);
            while (walker.nextNode()) nodes.push(walker.currentNode);
            return {
                key: block.getAttribute("data-rustify-island")!, root, nodes,
                ids: [...root.querySelectorAll("[id]")].map(node => [node, node.id] as [Element, string]),
                source: block.querySelector(".rd-demo-source"),
                controls: [...root.querySelectorAll<HTMLInputElement>("input")].map(node => ({ node, value: node.value, checked: node.checked })),
            };
        });
        (window as ProbeWindow).__i18n_originals = originals;
        return originals.map(original => ({ key: original.key, nodes: original.nodes.length, html: original.root.outerHTML }));
    });
    await test.info().attach("i18n-original-ssr.json", { body: JSON.stringify(summary, null, 2), contentType: "application/json" });
}

async function identity(page: Page) {
    const result = await page.evaluate(() => (window as ProbeWindow).__i18n_originals!.map(original => {
        const block = [...document.querySelectorAll("[data-rustify-island]")].find(block => block.getAttribute("data-rustify-island") === original.key)!;
        return {
            key: original.key,
            sameRoot: block.querySelector("fieldset[data-rd-island-controls]")!.firstElementChild === original.root,
            nodes: original.nodes.length,
            retained: original.nodes.filter(node => node.isConnected && original.root.contains(node)).length,
            sameIds: original.ids.every(([node, id]) => node.id === id),
            sameValues: original.controls.every(control => control.node.value === control.value && control.node.checked === control.checked),
            sameSource: block.querySelector(".rd-demo-source") === original.source,
        };
    }));
    await test.info().attach("i18n-attached-identity.json", { body: JSON.stringify(result, null, 2), contentType: "application/json" });
    for (const block of result) {
        expect(block.sameRoot, block.key).toBe(true);
        expect(block.retained, block.key).toBe(block.nodes);
        expect(block.sameIds, block.key).toBe(true);
        expect(block.sameValues, block.key).toBe(true);
        expect(block.sameSource, block.key).toBe(true);
    }
    expect(await page.locator("[id]").evaluateAll(nodes => nodes.length === new Set(nodes.map(node => node.id)).size)).toBe(true);
}

async function holdAsset(page: Page, file: string) {
    const pattern = `**/assets/islands/**/${file}`;
    let release!: () => void;
    let observed!: () => void;
    const released = new Promise<void>(resolve => { release = resolve; });
    const seen = new Promise<void>(resolve => { observed = resolve; });
    const handler = async (route: Route) => {
        observed();
        await released;
        await route.continue();
    };
    await page.route(pattern, handler);
    return { seen, release, remove: () => page.unroute(pattern, handler) };
}

// The entry module is held first, so parsed can be inspected before boot.
// The real wasm is held next to retain the server's nodes until attachment.
async function openPaused(page: Page, audit: Audit, path: string, lang: string, notFound = false) {
    const module = await holdAsset(page, "islands.js");
    const wasm = await holdAsset(page, "app_bg.wasm");
    const start = audit.requests.length;
    if (notFound) audit.document404.add(url(path));
    try {
        const response = await page.goto(url(path), { waitUntil: "commit" });
        await module.seen;
        const parsed = await page.evaluate(async () => {
            const coordinator = (window as ProbeWindow).__rustify_docs_islands!;
            const blocks = await coordinator.parsed;
            return {
                lang: document.documentElement.lang, state: coordinator.state(), stats: coordinator.stats(),
                templates: document.querySelectorAll("template[data-rd-not-found-prefix]").length,
                connected: blocks.every(block => block.isConnected),
                keys: blocks.map(block => block.getAttribute("data-rustify-island")),
                active: [...document.querySelectorAll("[data-rustify-island]")].map(block => block.getAttribute("data-rustify-island")),
                langs: blocks.map(block => JSON.parse(block.getAttribute("data-rustify-island-metadata")!).environment.lang),
                mounted: document.querySelectorAll("[data-rustify-mounted]").length,
            };
        });
        expect(parsed).toEqual({ lang, state: "loading", stats: null, templates: 0, connected: true, keys: parsed.active, active: parsed.active, langs: [lang, lang], mounted: 0 });
        expect(parsed.active).toHaveLength(2);
        expect(new Set(parsed.keys).size).toBe(2);
        expect(audit.requests.slice(start).filter(request => /\/(?:app\.js|app_bg\.wasm)$/.test(new URL(request).pathname))).toEqual([]);
        await test.info().attach("i18n-before-boot.json", { body: JSON.stringify(parsed, null, 2), contentType: "application/json" });
        await documentPolicy(page, response, notFound ? 404 : 200);
        if (notFound && metaHost()) {
            const raw = await response!.text();
            expect(raw).toMatch(/<html\b[^>]*\blang="en"/);
            expect(raw).toContain('data-rd-not-found-prefix="zh"');
            expect(raw).toContain('data-rd-not-found-prefix="fr"');
        }
        await metadata(page, lang);
        await expect(page.locator('[data-rustify-island][data-rd-island-state="loading"]')).toHaveCount(2);
        for (const island of await islands(page).all()) {
            await expect(canvas(island)).toHaveJSProperty("inert", true);
            await expect(canvas(island)).toHaveAttribute("aria-busy", "true");
            await expect(canvas(island)).toHaveJSProperty("disabled", false);
            const gated = await canvas(island).locator("button").first().evaluate(button => {
                let delivered = false;
                button.addEventListener("click", () => { delivered = true; });
                const accepted = button.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
                return { accepted, delivered };
            });
            expect(gated).toEqual({ accepted: false, delivered: false });
        }
        await captureOriginals(page);
        module.release();
        await wasm.seen;
        await module.remove();
        return { ...wasm, start };
    } catch (error) {
        module.release();
        wasm.release();
        throw error;
    }
}

async function ready(page: Page, audit: Audit, lang: string, start: number) {
    await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(2);
    expect(await page.evaluate(() => (window as ProbeWindow).__rustify_docs_islands?.state())).toBe("ready");
    for (const island of await islands(page).all()) {
        await expect(canvas(island)).toHaveJSProperty("inert", false);
        await expect(canvas(island)).toHaveJSProperty("disabled", false);
        await expect(canvas(island)).toHaveAttribute("aria-busy", "false");
        const data = JSON.parse((await island.getAttribute("data-rustify-island-metadata"))!) as Metadata;
        await expect(canvas(island).locator(":scope > [data-rustify-scope]")).toHaveAttribute("data-rustify-mounted", data.seed);
    }
    await metadata(page, lang);
    for (const file of ["islands.js", "app.js", "app_bg.wasm"]) {
        expect(audit.requests.slice(start).filter(request => new URL(request).pathname.endsWith(`/${file}`)), file).toHaveLength(1);
    }
}

async function interact(page: Page, lang: string) {
    await expect(page.getByTestId("counter-value")).toHaveText("7");
    await page.getByTestId("counter-increment").click();
    await expect(page.getByTestId("counter-value")).toHaveText("8");
    await page.getByTestId("theme-probe-dialog-trigger").click();
    const dialog = page.getByRole("dialog", { name: "Theme probe dialog", exact: true });
    await expect(dialog).toBeVisible();
    await dialog.getByRole("button", { name: lang === "zh" ? "关闭" : "close", exact: true }).click();
    await expect(dialog).toHaveCount(0);
    await expect(page.getByTestId("theme-probe-dialog-trigger")).toBeFocused();
}

async function localeContext(page: Page, lang: string) {
    await expect(page.locator("html")).toHaveAttribute("lang", lang);
    const marker = page.locator("[data-locale-page]");
    await expect(marker).toHaveCount(1);
    await expect(marker).toHaveAttribute("data-context-route", "/locale/");
    await expect(marker).toHaveAttribute("data-context-lang", lang);
    await expect(marker).toHaveAttribute("data-context-base", base());
    await expect(marker.locator(":scope > p")).toHaveText(lang === "zh" ? "中文语言页面" : "English locale page");
    await expect(page.locator("article.rd-prose")).toHaveAttribute("lang", lang === "fr" ? "en" : lang);
    await expect(page.locator("[data-rd-language-switch] summary")).toHaveText(lang === "zh" ? "中文" : lang === "fr" ? "Français" : "English");
}

async function sourceReadable(page: Page) {
    for (const island of await islands(page).all()) {
        const counter = await island.locator("[data-counter]").count() === 1;
        const file = counter ? "demos.rs" : "custom.rs";
        const source = island.locator(".rd-demo-source");
        await expect(source.locator("[data-rd-source-path]")).toHaveText(`src/${file}`);
        await expect(source.locator(".rd-code")).toHaveCount(2);
        for (const panel of await source.locator(".rd-code").all()) await expect(panel).toBeVisible();
        expect(await source.locator("template.rd-code-raw").last().evaluate(node => (node as HTMLTemplateElement).content.textContent))
            .toBe(readFileSync(`tests/docs-ssg/fixtures/components/src/${file}`, "utf8"));
    }
}

for (const lang of ["en", "zh", "fr"]) {
    const path = lang === "en" ? "locale/" : `${lang}/locale/`;
    test(`${path} retains SSR nodes and its target language through real island attachment`, async ({ page, audit }) => {
        const held = await openPaused(page, audit, path, lang);
        try {
            await localeContext(page, lang);
            if (lang === "fr") {
                await expect(page.locator(".rd-title")).toHaveAttribute("lang", "en");
                await expect(page.locator(".rd-untranslated")).toHaveText(frenchNotice);
                await expect(page.locator(".rd-untranslated")).toHaveAttribute("lang", "fr");
                for (const island of await islands(page).all()) {
                    await expect(island.locator("[data-rd-island-loading]")).toHaveText("Chargement de l’exemple…");
                    await expect(island.locator("[data-rd-island-loading]")).toHaveAttribute("lang", "fr");
                    await expect(island.locator("[data-rd-island-error]")).toHaveAttribute("lang", "en");
                    await expect(island.locator("[data-rd-island-reload]")).toHaveAttribute("lang", "en");
                    const copy = island.locator(".rd-code [data-rd-copy]").first();
                    await expect(copy).toHaveAttribute("data-copy", "Copier");
                    await expect(copy).toHaveAttribute("data-copy-lang", "fr");
                    await expect(copy).toHaveAttribute("data-copied-lang", "fr");
                    await expect(copy).toHaveAttribute("data-copy-failed-lang", "en");
                    await expect(island.locator(".rd-code-title").first()).toHaveText("Fonction");
                    await expect(island.locator(".rd-code-title").first()).toHaveAttribute("lang", "fr");
                    await expect(island.locator(".rd-code-title").last()).toHaveText("Full file");
                }
            } else await expect(page.locator(".rd-untranslated")).toHaveCount(0);
            held.release();
            await ready(page, audit, lang, held.start);
            await identity(page);
            await interact(page, lang);
            await healthy(page, audit);
        } finally { held.release(); await held.remove(); }
    });
}

test("the language menu switches between the two real registered versions of the same page", async ({ page, audit }) => {
    await page.goto(url("locale/#shared-section"));
    await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(2);
    for (const lang of ["zh", "en"]) {
        const switcher = page.locator("[data-rd-language-switch]");
        await switcher.locator("summary").click();
        const link = switcher.locator(`a[hreflang="${lang}"]`);
        await expect(link).toHaveAttribute("href", new URL(url(lang === "en" ? "locale/" : "zh/locale/")).pathname);
        await link.click();
        await expect(page).toHaveURL(url(`${lang === "en" ? "locale/" : "zh/locale/"}#shared-section`));
        await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(2);
        await localeContext(page, lang);
        await metadata(page, lang);
        await expect(page.locator(`[data-rd-language-switch] a[hreflang="${lang}"]`)).toHaveAttribute("aria-current", "page");
        await interact(page, lang);
    }
    await healthy(page, audit);
});

test("the Chinese shell keeps an untranslated static registered page readable without loading islands", async ({ page, audit }) => {
    const response = await page.goto(url("zh/plain/"));
    await documentPolicy(page, response, 200);
    await expect(page.locator("html")).toHaveAttribute("lang", "zh");
    await expect(page.locator("article.rd-prose")).toHaveAttribute("lang", "en");
    await expect(page.locator("[data-plain-page]")).toContainText("This custom page contains ordinary reading content.");
    await expect(page.locator(".rd-untranslated")).toHaveAttribute("lang", "zh");
    await expect(page.locator(".rd-untranslated")).toBeVisible();
    await expect(page.locator('[data-slot="before_content"]')).toHaveAttribute("data-context-route", "/plain/");
    await expect(page.locator('[data-slot="before_content"]')).toHaveAttribute("data-context-lang", "zh");
    await expect(islands(page)).toHaveCount(0);
    expect(audit.requests.filter(request => /\/assets\/islands\/|\.wasm$/.test(new URL(request).pathname))).toEqual([]);
    await healthy(page, audit);
});

for (const lang of ["zh", "fr"]) {
    test(`${lang} deep 404 selects one active slot tree before parsed and boot, with real HTTP 404`, async ({ page, audit }) => {
        const path = `${lang}/missing/a/b/`;
        const held = await openPaused(page, audit, path, lang, true);
        try {
            await expect(page.locator(".rd-title")).toHaveText(lang === "zh" ? "页面不存在" : "Page introuvable");
            for (const slot of ["before_content", "not_found"]) {
                const container = page.locator(`[data-slot="${slot}"]`);
                await expect(container).toHaveCount(1);
                await expect(container).toHaveAttribute("data-context-route", "/404/");
                await expect(container).toHaveAttribute("data-context-lang", lang);
                await expect(container).toHaveAttribute("data-context-base", base());
                await expect(container.locator("[data-rustify-island]")).toHaveCount(1);
            }
            await expect(page.locator('[data-slot="before_content"] [data-counter]')).toHaveCount(1);
            await expect(page.locator('[data-slot="not_found"] [data-theme-probe]')).toHaveCount(1);
            held.release();
            await ready(page, audit, lang, held.start);
            await identity(page);
            await interact(page, lang);
            await expect(page.locator("template[data-rd-not-found-prefix]")).toHaveCount(0);
            await healthy(page, audit);
        } finally { held.release(); await held.remove(); }
    });
}

test("without scripts the locale pages, source and language homes stay readable and island controls stay disabled", async ({ browser }) => {
    const context = await browser.newContext({ javaScriptEnabled: false, reducedMotion: "reduce" });
    const page = await context.newPage();
    const audit = await watch(page);
    try {
        for (const requested of ["en", "zh", "fr"]) {
            const path = requested === "en" ? "locale/" : `${requested}/locale/`;
            const response = await page.goto(url(path));
            await documentPolicy(page, response, 200);
            await localeContext(page, requested);
            await metadata(page, requested);
            await sourceReadable(page);
            for (const island of await islands(page).all()) {
                await expect(island).toHaveAttribute("data-rd-island-state", "no-script");
                await expect(canvas(island)).toHaveJSProperty("disabled", true);
                await expect(island.locator("noscript > span")).toBeVisible();
                await expect(island.locator("[data-rd-island-reload]")).toBeHidden();
                await expect(island.locator(".rd-code-tabs")).toBeHidden();
            }
            await expect(page.getByTestId("counter-increment")).toBeDisabled();
            await expect(page.getByTestId("counter-input")).toBeDisabled();
            await expect(page.getByTestId("theme-probe-dialog-trigger")).toBeDisabled();
            await expect(page.getByTestId("counter-value")).toHaveText("7");
        }
        await page.goto(url("zh/plain/"));
        await expect(page.locator("[data-plain-page]")).toContainText("Readable without scripts — 中文, 🙂");
        await expect(page.locator("article.rd-prose")).toHaveAttribute("lang", "en");
        for (const requested of ["zh", "fr"]) {
            const path = `${requested}/missing/a/b/`;
            audit.document404.add(url(path));
            const response = await page.goto(url(path));
            await documentPolicy(page, response, 404);
            const lang = metaHost() ? "en" : requested;
            await expect(page.locator("html")).toHaveAttribute("lang", lang);
            await expect(page.locator(".rd-title")).toHaveText(lang === "en" ? "Page not found" : lang === "zh" ? "页面不存在" : "Page introuvable");
            await metadata(page, lang);
            for (const slot of ["before_content", "not_found"]) {
                const container = page.locator(`[data-slot="${slot}"]`);
                await expect(container).toHaveCount(1);
                await expect(container).toHaveAttribute("data-context-route", "/404/");
                await expect(container).toHaveAttribute("data-context-lang", lang);
                await expect(container.locator("[data-rustify-island]")).toHaveCount(1);
            }
            for (const home of ["en", "zh", "fr"]) {
                const link = page.locator(`.rd-available-languages a[hreflang="${home}"]`);
                await expect(link).toHaveAttribute("href", new URL(url(home === "en" ? "./" : `${home}/`)).pathname);
                await expect(link).toBeVisible();
            }
            await sourceReadable(page);
            for (const island of await islands(page).all()) {
                await expect(island).toHaveAttribute("data-rd-island-state", "no-script");
                await expect(canvas(island)).toHaveJSProperty("disabled", true);
                await expect(island.locator("noscript > span")).toBeVisible();
            }
            await expect(page.getByTestId("counter-increment")).toBeDisabled();
            await expect(page.getByTestId("theme-probe-dialog-trigger")).toBeDisabled();
            const templates = await page.locator("template[data-rd-not-found-prefix]").evaluateAll(nodes => nodes.map(node => {
                const template = node as HTMLTemplateElement;
                return { lang: template.getAttribute("data-rd-not-found-lang"), islands: template.content.querySelectorAll("[data-rustify-island]").length, mounted: template.content.querySelectorAll("[data-rustify-mounted]").length };
            }));
            expect(templates).toEqual(metaHost() ? [{ lang: "fr", islands: 2, mounted: 0 }, { lang: "zh", islands: 2, mounted: 0 }] : []);
        }
        expect(audit.requests.filter(request => /\/assets\/islands\/|\.wasm$/.test(new URL(request).pathname))).toEqual([]);
        await healthy(page, audit);
        await test.info().attach("i18n-islands-no-script-audit.json", { body: JSON.stringify({ ...audit, document404: [...audit.document404] }, null, 2), contentType: "application/json" });
    } finally { await context.close(); }
});
