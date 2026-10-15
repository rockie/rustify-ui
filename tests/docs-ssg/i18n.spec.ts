import { expect, Locator, Page, test as playwright } from "@playwright/test";
import { atRest, attachScans, buildId, scan, Scan, serious } from "./a11y";
import { expectStill, installMotionProbe, sampleMotion } from "./motion";

// The content-only fixture, served by the installed C in the three
// docs-i18n projects. Narrow desktop viewports are responsive checks;
// native 200% zoom, real phones and spoken language remain separate gates.
const STRICT = "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";
const META = STRICT.replace("; frame-ancestors 'none'", "");
const languages = [
    { lang: "en", prefix: "", label: "English", title: "Set up your reading space", missing: "Page not found" },
    { lang: "zh", prefix: "zh/", label: "中文", title: "设置阅读空间", missing: "页面不存在" },
    { lang: "fr", prefix: "fr/", label: "Français", title: "Set up your reading space", missing: "Page introuvable" },
] as const;

type Audit = {
    requests: string[];
    failures: string[];
    console: string[];
    pageErrors: string[];
    csp: string[];
    responses: { url: string; status: number; mainDocument: boolean }[];
    expected404: Set<string>;
    expectedConsole: string[];
};

declare global {
    interface Window { __i18nCsp(value: string): void; }
}

const url = (path: string) => new URL(path, String(playwright.info().project.use.baseURL)).href;
const base = () => new URL(url("./")).pathname;
const metaHost = () => playwright.info().project.name === "docs-i18n-meta";
const menu = (page: Page) => page.locator("details.rd-language-switch");

async function watch(page: Page): Promise<Audit> {
    const audit: Audit = { requests: [], failures: [], console: [], pageErrors: [], csp: [], responses: [], expected404: new Set(), expectedConsole: [] };
    await page.exposeBinding("__i18nCsp", (_, value: string) => audit.csp.push(value));
    await page.addInitScript(() => document.addEventListener("securitypolicyviolation", event => window.__i18nCsp(`${event.violatedDirective} ${event.blockedURI}`)));
    page.on("request", request => audit.requests.push(request.url()));
    page.on("requestfailed", request => audit.failures.push(`${request.url()} ${request.failure()?.errorText}`));
    page.on("pageerror", error => audit.pageErrors.push(String(error)));
    page.on("response", response => audit.responses.push({
        url: response.url(), status: response.status(),
        mainDocument: response.request().isNavigationRequest() && response.frame() === page.mainFrame(),
    }));
    page.on("console", message => {
        if (message.type() !== "error") return;
        if (audit.expected404.has(message.location().url) && message.text() === "Failed to load resource: the server responded with a status of 404 (Not Found)") {
            audit.expectedConsole.push(`${message.location().url}: ${message.text()}`);
        } else audit.console.push(message.text());
    });
    return audit;
}

async function attachAudit(audit: Audit, name = "i18n-browser-audit.json") {
    await playwright.info().attach(name, {
        body: JSON.stringify({ ...audit, expected404: [...audit.expected404] }, null, 2),
        contentType: "application/json",
    });
}

async function healthy(page: Page, audit: Audit) {
    await page.evaluate(() => document.fonts.ready);
    expect(audit.failures).toEqual([]);
    expect(audit.console).toEqual([]);
    expect(audit.pageErrors).toEqual([]);
    expect(audit.csp).toEqual([]);
    expect(audit.responses.filter(response => response.status >= 400
        && !(response.status === 404 && response.mainDocument && audit.expected404.has(response.url)))).toEqual([]);
    for (const request of audit.requests) {
        const target = new URL(request);
        expect(target.origin, request).toBe(new URL(url("./")).origin);
        expect(target.pathname.startsWith(base()), request).toBe(true);
    }
    expect(audit.requests.filter(request => /makepad|\.wasm(?:[?#]|$)/i.test(request))).toEqual([]);
}

const test = playwright.extend<{ audit: Audit }>({
    audit: async ({ page }, use) => {
        const audit = await watch(page);
        try {
            await use(audit);
            await healthy(page, audit);
        } finally { await attachAudit(audit); }
    },
});

async function open(page: Page, path: string) {
    const response = await page.goto(url(path));
    expect(response?.status(), path).toBe(200);
    await expect(page.locator("html")).toHaveAttribute("data-rd-base", base());
    await expect(page.locator('meta[http-equiv="Content-Security-Policy"]')).toHaveAttribute("content", META);
    if (metaHost()) expect(response!.headers()["content-security-policy"]).toBeUndefined();
    else expect(response!.headers()["content-security-policy"]).toBe(STRICT);
    await expect(page.locator("[data-rustify-island]")).toHaveCount(0);
    return response!;
}

async function switchTo(page: Page, lang: string) {
    if (await menu(page).getAttribute("open") === null) await menu(page).locator("summary").click();
    await menu(page).locator(`a[hreflang="${lang}"]`).click();
}

async function fits(page: Page) {
    expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth), "page-level horizontal overflow").toBeLessThanOrEqual(0);
}

async function visibleFocus(target: Locator) {
    await expect(target).toBeFocused();
    expect(await target.evaluate(node => {
        const style = getComputedStyle(node);
        return style.outlineStyle !== "none" && parseFloat(style.outlineWidth) >= 2;
    }), "keyboard focus has a visible outline").toBe(true);
}

test("same-page languages keep navigation local and identify the real source", async ({ page, audit }) => {
    let sourceImage = "";
    for (const language of languages) {
        await open(page, `${language.prefix}guide/setup/`);
        await expect(page.locator("html")).toHaveAttribute("lang", language.lang);
        await expect(page.locator("article")).toHaveAttribute("lang", language.lang === "zh" ? "zh" : "en");
        await expect(page.getByRole("heading", { level: 1 })).toHaveText(language.title);
        expect(await page.getByRole("heading", { level: 1 }).evaluate(node => node.closest("[lang]")?.getAttribute("lang"))).toBe(language.lang === "zh" ? "zh" : "en");
        await expect(menu(page).locator("summary")).toHaveText(language.label);
        await menu(page).locator("summary").click();
        await expect(menu(page).locator("a[hreflang]")).toHaveCount(3);
        for (const target of languages) {
            const link = menu(page).locator(`a[hreflang="${target.lang}"]`);
            await expect(link).toHaveAttribute("href", `${base()}${target.prefix}guide/setup/`);
            await expect(link).toHaveAttribute("lang", target.lang);
            await expect(link.locator(".rd-language-fallback")).toHaveCount(target.lang === "fr" ? 1 : 0);
        }
        await expect(menu(page).locator('a[aria-current="page"]')).toHaveAttribute("hreflang", language.lang);
        await page.keyboard.press("Escape");
        await expect(page.locator(".rd-untranslated")).toHaveCount(language.lang === "fr" ? 1 : 0);
        if (language.lang === "fr") {
            await expect(page.locator("article > h1 + .rd-untranslated")).toHaveAttribute("lang", "fr");
        }
        const current = page.locator('.rd-sidebar-wide a[aria-current="page"]');
        await expect(current).toHaveText(language.title);
        await expect(current).toHaveAttribute("lang", language.lang === "zh" ? "zh" : "en");
        for (const selector of [".rd-sidebar-wide a", ".rd-header-nav a", ".rd-pager a", ".rd-site-footer a"]) {
            const hrefs = await page.locator(selector).evaluateAll(nodes => nodes.map(node => node.getAttribute("href")));
            expect(hrefs.length, selector).toBeGreaterThan(0);
            expect(hrefs.every(href => href?.startsWith(`${base()}${language.prefix}`)), `${language.lang}: ${selector}`).toBe(true);
        }
        if (language.lang !== "zh") {
            const image = page.locator("article img");
            await expect(image).toBeVisible();
            expect(await image.evaluate(node => (node as HTMLImageElement).naturalWidth)).toBeGreaterThan(0);
            const src = await image.getAttribute("src");
            if (language.lang === "en") sourceImage = src!;
            else expect(src).toBe(sourceImage);
        }
        await page.locator('.rd-pager a').last().click();
        await expect(page).toHaveURL(url(`${language.prefix}guide/second/`));
        await expect(page.locator("html")).toHaveAttribute("lang", language.lang);
        await expect(page.locator("article")).toHaveAttribute("lang", "en");
        await expect(page.locator(".rd-untranslated")).toHaveCount(language.lang === "en" ? 0 : 1);
        await page.locator("article").getByRole("link", { name: "Return to setup", exact: true }).click();
        await expect(page).toHaveURL(url(`${language.prefix}guide/setup/#shared-section`));
        await expect(page.locator("#shared-section")).toBeInViewport();
    }
    expect(audit.responses.filter(response => response.status !== 200)).toEqual([]);
});

test("ordinary language links retain only an anchor present in the destination", async ({ page, audit }) => {
    await open(page, "guide/setup/#shared-section");
    await switchTo(page, "zh");
    await expect(page).toHaveURL(url("zh/guide/setup/#shared-section"));
    await expect(page.locator("#shared-section")).toBeInViewport();
    await switchTo(page, "fr");
    await expect(page).toHaveURL(url("fr/guide/setup/#shared-section"));
    await expect(page.locator("#shared-section")).toBeInViewport();
    await open(page, "guide/setup/#english-heading");
    await switchTo(page, "zh");
    await expect(page).toHaveURL(url("zh/guide/setup/"));
    await expect(page.getByRole("heading", { level: 1 })).toBeInViewport();
    const chineseHeading = await page.locator("article h2", { hasText: "中文标题" }).getAttribute("id");
    expect(chineseHeading).toBeTruthy();
    await page.goto(url(`zh/guide/setup/#${encodeURIComponent(chineseHeading!)}`));
    await switchTo(page, "en");
    await expect(page).toHaveURL(url("guide/setup/"));
    await expect(page.getByRole("heading", { level: 1 })).toBeInViewport();
    expect(audit.failures).toEqual([]);
});

test("canonical, hreflang and the one sitemap list real versions and exclude fallback indexing", async ({ page, request, audit }) => {
    await open(page, "");
    const publicRoot = new URL((await page.locator('link[rel="canonical"]').getAttribute("href"))!);
    expect(publicRoot.protocol).toMatch(/^https?:$/);
    expect(publicRoot.pathname).toBe(base());
    const real = ["", "guide/setup/", "guide/second/", "faq/", "zh/", "zh/guide/setup/", "fr/"];
    for (const language of languages) {
        for (const logical of ["", "guide/setup/", "guide/second/", "faq/"]) {
            const path = `${language.prefix}${logical}`;
            await open(page, path);
            const fallback = !real.includes(path);
            const source = fallback ? logical : path;
            await expect(page.locator('link[rel="canonical"]')).toHaveAttribute("href", new URL(source, publicRoot).href);
            await expect(page.locator('meta[name="robots"][content="noindex"]')).toHaveCount(fallback ? 1 : 0);
            const alternates = await page.locator('head link[rel="alternate"][hreflang]').evaluateAll(nodes => nodes.map(node => ({
                lang: node.getAttribute("hreflang"), href: node.getAttribute("href"),
            })).sort((a, b) => a.lang!.localeCompare(b.lang!)));
            const versions = languages.filter(target => real.includes(`${target.prefix}${logical}`));
            expect(alternates).toEqual(versions.map(target => ({ lang: target.lang, href: new URL(`${target.prefix}${logical}`, publicRoot).href })).sort((a, b) => a.lang.localeCompare(b.lang)));
        }
    }
    const sitemap = await request.get(url("sitemap.xml"));
    expect(sitemap.status()).toBe(200);
    const locations = await page.evaluate(xml => [...new DOMParser().parseFromString(xml, "application/xml").querySelectorAll("loc")].map(node => node.textContent).sort(), await sitemap.text());
    expect(locations).toEqual(real.map(path => new URL(path, publicRoot).href).sort());
    const robots = await request.get(url("robots.txt"));
    expect(robots.status()).toBe(200);
    expect(await robots.text()).toContain(`Sitemap: ${new URL("sitemap.xml", publicRoot).href}`);
    expect(audit.csp).toEqual([]);
});

test("partial French copy labels announce their own language without changing the article", async ({ page, context, browserName, audit }) => {
    test.skip(browserName !== "chromium", "clipboard permissions are a Chromium automation feature");
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    await open(page, "fr/guide/setup/");
    const copy = page.locator("figure.rd-code [data-rd-copy]");
    await expect(copy).toBeVisible();
    await expect(copy).toHaveAttribute("lang", "fr");
    await expect(copy).toContainText("Copier");
    await expect(copy).toHaveAttribute("data-copied-lang", "en");
    await copy.click();
    await expect(copy).toHaveAttribute("data-state", "copied");
    await expect(copy).toContainText("Copied");
    await expect(copy).toHaveAttribute("lang", "en");
    await expect(page.locator("#rd-status")).toHaveText("Copied");
    await expect(page.locator("#rd-status")).toHaveAttribute("lang", "en");
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe('let greeting = "Résumé — 中文 — 🙂";\nprintln!("{greeting}");\n');
    await expect(page.locator("html")).toHaveAttribute("lang", "fr");
    await expect(page.locator("article")).toHaveAttribute("lang", "en");
    expect(audit.pageErrors).toEqual([]);
});

test("deep 404s select one active language tree with the same policy and a way home", async ({ page, audit }) => {
    for (const [path, lang] of [["missing/a/b/", "en"], ["zh/missing/a/b/", "zh"], ["fr/missing/a/b/", "fr"], ["zh-prefix/missing/", "en"]] as const) {
        audit.expected404.add(url(path));
        const response = await page.goto(url(path));
        expect(response?.status()).toBe(404);
        const raw = await response!.text();
        const servedLang = metaHost() ? "en" : lang;
        expect(raw).toContain(`<html lang="${servedLang}"`);
        if (metaHost()) {
            expect(raw).toContain('template data-rd-not-found-prefix="zh"');
            expect(raw).toContain('data-rd-not-found-prefix="fr"');
        }
        const selected = languages.find(language => language.lang === lang)!;
        await expect(page.locator("html")).toHaveAttribute("lang", lang);
        await expect(page).toHaveTitle(`${selected.missing} | Language fixture`);
        await expect(page.getByRole("heading", { level: 1 })).toHaveText(selected.missing);
        await expect(page.getByRole("banner")).toHaveCount(1);
        await expect(page.getByRole("main")).toHaveCount(1);
        await expect(page.getByRole("contentinfo")).toHaveCount(1);
        await expect(page.locator("template[data-rd-not-found-prefix]")).toHaveCount(0);
        await expect(page.locator('meta[name="robots"]')).toHaveAttribute("content", "noindex");
        await expect(page.locator('head link[rel="canonical"], head link[hreflang]')).toHaveCount(0);
        await expect(page.locator('meta[http-equiv="Content-Security-Policy"]')).toHaveAttribute("content", META);
        if (metaHost()) expect(response!.headers()["content-security-policy"]).toBeUndefined();
        else expect(response!.headers()["content-security-policy"]).toBe(STRICT);
        expect(await page.evaluate(() => {
            const ids = [...document.querySelectorAll("[id]")].map(node => node.id);
            return ids.length === new Set(ids).size;
        })).toBe(true);
        for (const language of languages) {
            await expect(page.locator(`.rd-available-languages a[hreflang="${language.lang}"]`)).toHaveAttribute("href", `${base()}${language.prefix}`);
        }
        const home = page.locator("main article > p a");
        await expect(home).toHaveAttribute("href", `${base()}${selected.prefix}`);
        await page.screenshot({ path: test.info().outputPath(`i18n-404-${path.startsWith("zh-prefix") ? "prefix-boundary" : lang}.png`), fullPage: true });
        await home.click();
        await expect(page).toHaveURL(url(selected.prefix));
        await expect(page.locator("html")).toHaveAttribute("lang", lang);
    }
});

test("reading, language links and 404 recovery remain usable without scripts", async ({ browser }) => {
    const context = await browser.newContext({ javaScriptEnabled: false, reducedMotion: "reduce" });
    const page = await context.newPage();
    const audit = await watch(page);
    try {
        await open(page, "zh/guide/setup/");
        await expect(page.locator("html")).toHaveAttribute("lang", "zh");
        await expect(page.getByRole("heading", { level: 1 })).toHaveText("设置阅读空间");
        await switchTo(page, "fr");
        await expect(page).toHaveURL(url("fr/guide/setup/"));
        await expect(page.locator("article")).toHaveAttribute("lang", "en");
        await expect(page.locator(".rd-untranslated")).toBeVisible();
        await expect(page.locator("article code").first()).toContainText("Résumé — 中文 — 🙂");
        await expect(page.locator("[data-rd-copy]")).toBeHidden();
        await page.locator("article").getByRole("link", { name: "Continue", exact: true }).click();
        await expect(page).toHaveURL(url("fr/guide/second/"));
        for (const requested of ["zh", "fr"] as const) {
            const path = `${requested}/missing/deep/`;
            audit.expected404.add(url(path));
            const response = await page.goto(url(path));
            expect(response?.status()).toBe(404);
            const active = languages.find(language => language.lang === (metaHost() ? "en" : requested))!;
            await expect(page.locator("html")).toHaveAttribute("lang", active.lang);
            await expect(page.getByRole("heading", { level: 1 })).toHaveText(active.missing);
            await expect(page.locator('meta[http-equiv="Content-Security-Policy"]')).toHaveAttribute("content", META);
            for (const language of languages) {
                await expect(page.locator(`.rd-available-languages a[hreflang="${language.lang}"]`)).toHaveAttribute("href", `${base()}${language.prefix}`);
            }
            await page.locator(`.rd-available-languages a[hreflang="${requested}"]`).click();
            await expect(page).toHaveURL(url(`${requested}/`));
            await expect(page.locator("html")).toHaveAttribute("lang", requested);
        }
        await healthy(page, audit);
    } finally {
        await attachAudit(audit, "i18n-no-script-audit.json");
        await context.close();
    }
});

for (const scheme of ["light", "dark"] as const) {
    test(`language controls remain keyboard reachable at desktop, 375 and 320 px in ${scheme} with reduced motion`, async ({ page, audit }) => {
        await page.emulateMedia({ colorScheme: scheme, reducedMotion: "reduce" });
        await installMotionProbe(page);
        const scans: Scan[] = [];
        for (const width of [1280, 375, 320]) {
            await page.setViewportSize({ width, height: 800 });
            await open(page, "fr/guide/setup/");
            await page.locator(`[data-rd-mode="${scheme}"]`).click();
            await expect(page.locator("html")).toHaveAttribute("data-theme", scheme);
            await expect(page.locator("body")).toHaveCSS("background-color", scheme === "dark" ? "rgb(12, 17, 29)" : "rgb(249, 250, 251)");
            await fits(page);
            const summary = menu(page).locator("summary");
            await summary.focus();
            await page.keyboard.press("Enter");
            await expect(menu(page)).toHaveAttribute("open", "");
            await page.keyboard.press("Tab");
            await visibleFocus(menu(page).locator('a[hreflang="en"]'));
            await page.keyboard.press("Tab");
            await visibleFocus(menu(page).locator('a[hreflang="zh"]'));
            await page.keyboard.press("Escape");
            await expect(menu(page)).not.toHaveAttribute("open", "");
            await visibleFocus(summary);
            await page.keyboard.press("Enter");
            for (const language of languages) {
                const link = menu(page).locator(`a[hreflang="${language.lang}"]`);
                await link.scrollIntoViewIfNeeded();
                await expect(link).toBeInViewport();
                const bounds = (await link.boundingBox())!;
                expect(bounds.x).toBeGreaterThanOrEqual(0);
                expect(bounds.x + bounds.width).toBeLessThanOrEqual(width);
            }
            await atRest(page);
            scans.push(await scan(page, "fr/guide/setup/", `language-open-${width}`));
            await page.screenshot({ path: test.info().outputPath(`i18n-${scheme}-${width}.png`), fullPage: true });
            await page.keyboard.press("Escape");
            await summary.focus();
            await page.keyboard.press("Enter");
            await page.keyboard.press("Tab");
            await page.keyboard.press("Tab");
            await page.keyboard.press("Enter");
            await expect(page).toHaveURL(url("zh/guide/setup/"));
            await expect(page.locator("html")).toHaveAttribute("data-theme", scheme);
            await expect(page.locator("html")).toHaveAttribute("lang", "zh");
            if (width < 768) {
                const sidebar = page.locator("details.rd-sidebar-narrow");
                await sidebar.locator("summary").focus();
                await page.keyboard.press("Enter");
                await expect(sidebar.getByRole("link", { name: "Continue reading", exact: true })).toBeVisible();
                await page.keyboard.press("Escape");
                await expect(sidebar).not.toHaveAttribute("open", "");
                await visibleFocus(sidebar.locator("summary"));
            }
            expect(await page.evaluate(() => getComputedStyle(document.documentElement).scrollBehavior)).toBe("auto");
            expectStill(await sampleMotion(page, `${scheme}-${width}`));
            await fits(page);
            if (width === 375) {
                const missing = "fr/missing/mobile/";
                audit.expected404.add(url(missing));
                const response = await page.goto(url(missing));
                expect(response?.status()).toBe(404);
                await expect(page.locator("html")).toHaveAttribute("lang", "fr");
                await expect(page.locator("html")).toHaveAttribute("data-theme", scheme);
                await expect(page.getByRole("heading", { level: 1 })).toHaveText("Page introuvable");
                await fits(page);
                await atRest(page);
                scans.push(await scan(page, missing, "mobile-404"));
                await page.screenshot({ path: test.info().outputPath(`i18n-404-${scheme}-375.png`), fullPage: true });
            }
        }
        await attachScans(test.info(), `i18n-${scheme}-a11y.json`, scans, await buildId(page, url("./")));
        expect(serious(scans)).toEqual([]);
        expect(audit.csp).toEqual([]);
    });
}
