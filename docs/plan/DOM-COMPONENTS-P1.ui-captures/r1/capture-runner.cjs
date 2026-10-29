// UI review R1 capture for DOM-COMPONENTS-P1. Drives the real release builds
// served by `mbx xtask serve` (Studio :4181, catalog :4176) and records each
// screenshot with its cell, viewport, actions and page health.
//
//   node docs/plan/DOM-COMPONENTS-P1.ui-captures/r1/capture-runner.cjs [unit...]
const { chromium } = require("@playwright/test");
const fs = require("node:fs");
const path = require("node:path");

const OUT = __dirname;
const STUDIO = "http://127.0.0.1:4181/";
const CATALOG = "http://127.0.0.1:4176/";
const only = process.argv.slice(2);
const records = [];
const health = [];
const actions = {};

function log(unit, text) { (actions[unit] ||= []).push(`${new Date().toISOString()} ${text}`); }

async function context(browser, width, height, extra = {}) {
    return browser.newContext({ viewport: { width, height }, deviceScaleFactor: 1, reducedMotion: "reduce",
        colorScheme: "light", locale: "en-US", acceptDownloads: true, ...extra });
}

function watch(page, unit) {
    const entry = { unit, url: null, pageErrors: [], consoleErrors: [], failed: [], csp: [] };
    page.on("pageerror", e => entry.pageErrors.push(e.message));
    page.on("console", m => { if (m.type() === "error") entry.consoleErrors.push(m.text()); });
    page.on("requestfailed", r => entry.failed.push(`${r.url()} ${r.failure()?.errorText}`));
    page.on("response", r => { if (r.status() >= 400) entry.failed.push(`${r.status()} ${r.url()}`); });
    page.addInitScript(() => {
        window.__reviewCsp = [];
        document.addEventListener("securitypolicyviolation", e => window.__reviewCsp.push(e.violatedDirective));
    });
    health.push(entry);
    return entry;
}

async function quiet(page) {
    await page.evaluate(() => document.fonts.ready);
    await page.waitForLoadState("networkidle").catch(() => {});
    await page.evaluate(() => new Promise(resolve => {
        let previous = performance.now(), since = previous;
        const deadline = previous + 15000;
        const tick = () => {
            const now = performance.now();
            if (now - previous > 100) since = now;
            previous = now;
            if (now - since >= 500 || now > deadline) resolve(); else requestAnimationFrame(tick);
        };
        requestAnimationFrame(tick);
    }));
}

async function ready(page, url) {
    await page.goto(url);
    await page.locator("#status[data-status=ready], [data-testid=status][data-status=ready]").first().waitFor({ state: "attached", timeout: 90000 });
    await quiet(page);
}

async function shot(page, unit, state, theme, interventions = "none", locator = null, attempt = "") {
    await quiet(page);
    const vp = page.viewportSize();
    const file = `${unit}-${state}-${vp.width}-${theme}${attempt}.png`;
    if (locator) await locator.screenshot({ path: path.join(OUT, file), animations: "disabled" });
    else await page.screenshot({ path: path.join(OUT, file), animations: "disabled" });
    records.push({ file, unit, state, viewport: `${vp.width}x${vp.height}`, dpr: 1, theme, url: page.url(),
        time: new Date().toISOString(), kind: locator ? "element" : "viewport", interventions });
    log(unit, `screenshot ${file}`);
    return file;
}

const tid = (page, id) => page.getByTestId(id);
async function select(page, id, label) {
    await tid(page, id).click();
    await tid(page, `${id}-list`).getByRole("option", { name: label, exact: true }).click();
}
async function color(page, token, value) {
    await tid(page, "editor-colors").click();
    await tid(page, "color-search").fill("");
    await tid(page, `color-token-${token}`).click();
    await tid(page, "color-value").fill(value);
    await tid(page, "color-value").press("Enter");
}
async function resetScroll(page) {
    await page.evaluate(() => {
        for (const s of [".studio-previews", "#studio-editor", ".studio-editor-content"]) document.querySelector(s)?.scrollTo(0, 0);
        window.scrollTo(0, 0);
    });
}
async function intoView(page, id) {
    await tid(page, id).first().evaluate(node => node.scrollIntoView({ block: "start" }));
}
async function graphitePink(page, unit) {
    await tid(page, "preset-search").fill("Graphite");
    await select(page, "preset-select", "Built-in · Graphite");
    await select(page, "studio-config-accent_color", "pink");
    await select(page, "studio-config-gray_color", "Auto");
    log(unit, "preset search 'Graphite' → Built-in · Graphite → brand pink → gray Auto");
}
const snapshot = page => page.evaluate(() => window.__theme_studio.snapshot());
async function idle(page) {
    for (let i = 0; i < 200; i++) {
        const s = await snapshot(page);
        if (s.storage.permission === "Ready" && !s.storage.paused && s.storage.pending === 0 && s.storage.acknowledged?.epoch === s.editor_epoch) return;
        await page.waitForTimeout(100);
    }
    throw new Error("storage never became idle");
}
async function tabTo(page, id, max = 120) {
    for (let i = 1; i <= max; i++) {
        await page.keyboard.press("Tab");
        if (await page.evaluate(id => document.activeElement?.closest(`[data-testid="${id}"]`) !== null || document.activeElement?.dataset?.testid === id, id)) return i;
    }
    throw new Error(`Tab never reached ${id}`);
}

const cells = {
    async U1(browser) {
        const u = "U1";
        for (const [w, h] of [[1440, 900], [768, 1024]]) {
            const ctx = await context(browser, w, h); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); log(u, `open ${STUDIO} at ${w}x${h}`);
            await shot(page, u, "default", "light");
            await graphitePink(page, u);
            await intoView(page, "studio-config");
            await shot(page, u, "configured", "light");
            if (w === 1440) {
                await tid(page, "mode-dark").click(); log(u, "click mode Dark");
                await shot(page, u, "configured", "dark");
                await resetScroll(page);
                await shot(page, u, "top", "dark");
                await tid(page, "mode-light").click(); await tid(page, "locale-zh").click(); log(u, "click mode Light, click 中文");
                await resetScroll(page);
                await shot(page, u, "zh-top", "light");
                await intoView(page, "studio-config");
                await shot(page, u, "zh-configured", "light");
            }
            await ctx.close();
        }
        {
            const ctx = await context(browser, 390, 844); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); log(u, "open at 390x844");
            await shot(page, u, "edit", "light");
            await graphitePink(page, u); await intoView(page, "studio-config");
            await shot(page, u, "configured", "light");
            await tid(page, "mobile-preview").click(); log(u, "click Preview tab"); await resetScroll(page);
            await shot(page, u, "preview", "light");
            await ctx.close();
        }
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO);
            await page.setViewportSize({ width: 720, height: 900 });
            await page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
            log(u, "open at 1440x900, then viewport 720x900 and root font-size 200% (WCAG reflow technique)");
            await shot(page, u, "zoom200", "light");
            await tid(page, "editor-colors").focus(); await page.keyboard.press("End"); log(u, "focus Colors tab, press End");
            await tid(page, "theme-export").focus(); await page.keyboard.press("Enter"); log(u, "focus Export, press Enter");
            await shot(page, u, "zoom200-export", "light");
            await ctx.close();
        }
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); log(u, "keyboard walk from document start at 1440x900");
            let n = await tabTo(page, "preset-search"); log(u, `Tab ×${n} → preset search focused`);
            await shot(page, u, "keyboard-01", "light");
            n = await tabTo(page, "studio-config-accent_color"); log(u, `Tab ×${n} → brand color select focused`);
            await shot(page, u, "keyboard-02", "light");
            await page.keyboard.press("Enter"); await page.keyboard.press("ArrowDown"); log(u, "Enter, ArrowDown on brand select");
            await shot(page, u, "keyboard-03", "light");
            await page.keyboard.press("Escape"); log(u, "Escape");
            n = await tabTo(page, "mode-dark"); log(u, `Tab ×${n} → mode Dark focused`);
            await shot(page, u, "keyboard-04", "light");
            await ctx.close();
        }
    },

    async U2(browser) {
        const u = "U2";
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await graphitePink(page, u);
            await tid(page, "editor-colors").click(); await tid(page, "color-search").fill("");
            await tid(page, "color-token-primary").click(); log(u, "Colors tab → token primary");
            await intoView(page, "color-editor");
            await shot(page, u, "source-overridden", "light");
            await tid(page, "source-kind").first().evaluate(n => n.scrollIntoView({ block: "center" }));
            await shot(page, u, "source-overridden-detail", "light");
            await tid(page, "source-clear").first().click(); log(u, "click single-token release (source-clear) on primary");
            await shot(page, u, "token-released", "light");
            await intoView(page, "studio-config");
            await tid(page, "studio-config-accent_color-reset").click(); log(u, "click brand group reset");
            await shot(page, u, "group-reset", "light");
            await tid(page, "color-token-ring").click(); log(u, "Colors tab → token ring");
            await tid(page, "source-kind").first().evaluate(n => n.scrollIntoView({ block: "center" }));
            await shot(page, u, "source-generated", "light");
            await tid(page, "editor-controls").click(); log(u, "click Fonts & other tab"); await resetScroll(page);
            await intoView(page, "editor-controls");
            await shot(page, u, "controls", "light");
            await tid(page, "edit-radius").fill("abc"); await tid(page, "edit-radius").press("Enter"); log(u, "radius ← 'abc', Enter");
            await tid(page, "edit-radius").evaluate(n => n.scrollIntoView({ block: "center" }));
            await shot(page, u, "field-error", "light");
            await ctx.close();
        }
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO);
            await color(page, "background", "#ffffff"); await color(page, "foreground", "#ffffff");
            log(u, "background ← #ffffff, foreground ← #ffffff");
            await page.locator(".studio-diagnostics > summary").click(); log(u, "open diagnostics");
            await page.locator(".studio-diagnostics").evaluate(n => n.scrollIntoView({ block: "start" }));
            await shot(page, u, "low-contrast", "light");
            await ctx.close();
        }
        {
            const ctx = await context(browser, 390, 844); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await graphitePink(page, u);
            await tid(page, "editor-colors").click(); await tid(page, "color-token-primary").click(); log(u, "Colors tab → primary at 390");
            await tid(page, "source-kind").first().evaluate(n => n.scrollIntoView({ block: "center" }));
            await shot(page, u, "source-overridden", "light");
            await ctx.close();
        }
    },

    async U3(browser) {
        const u = "U3";
        for (const [w, h] of [[1440, 900], [390, 844]]) {
            const ctx = await context(browser, w, h); const page = await ctx.newPage(); watch(page, u);
            await ctx.grantPermissions(["clipboard-read", "clipboard-write"], { origin: STUDIO.slice(0, -1) });
            await ready(page, STUDIO);
            const values = (await snapshot(page)).values;
            values.light.primary = "#7c3aed"; values.dark.primary = "#a78bfa";
            const legacy = JSON.stringify({ schema_version: 1, id: "review-legacy", name: "Imported legacy 旧主题", styles: values }, null, 2);
            await tid(page, "theme-import").click(); log(u, `open Import at ${w}`);
            await tid(page, "import-source").fill(legacy); await tid(page, "import-review").click(); log(u, "paste valid v1 JSON (primary changed), click Review");
            await shot(page, u, "import-review", "light");
            if (w === 1440) {
                await page.getByTestId("import-apply").evaluate(n => n.scrollIntoView({ block: "end" }));
                await shot(page, u, "import-review-end", "light");
                await tid(page, "import-source").fill('{"schema_version":999}'); await tid(page, "import-review").click(); log(u, "replace with {schema_version:999}, click Review");
                await shot(page, u, "import-error", "light");
            }
            await page.keyboard.press("Escape"); log(u, "Escape");
            await graphitePink(page, u);
            await tid(page, "theme-export").click(); log(u, "open Export");
            await shot(page, u, "export-json", "light");
            if (w === 1440) {
                await select(page, "export-kind", "Compatible JSON v1 · lossy"); log(u, "export kind → Compatible JSON v1 · lossy");
                await shot(page, u, "export-v1", "light");
                await select(page, "export-kind", "Tailwind v4 CSS"); log(u, "export kind → Tailwind v4 CSS");
                await shot(page, u, "export-css", "light");
                await select(page, "export-kind", "Rust integration"); log(u, "export kind → Rust integration");
                await shot(page, u, "export-rust", "light");
                await tid(page, "export-copy").click(); log(u, "click Copy (clipboard permission granted)");
                await page.waitForTimeout(300);
                await shot(page, u, "export-copied", "light");
            }
            await ctx.close();
        }
    },

    async U4(browser) {
        const u = "U4";
        const v2 = "rustify-ui.theme-studio.v2", v1 = "rustify-ui.theme-studio.v1";
        async function seed(page) {
            await ready(page, STUDIO);
            const before = await snapshot(page);
            const doc = { schema_version: 1, id: "legacy-draft", name: "旧草稿", styles: before.values };
            const saved = [{ ...structuredClone(doc), id: "legacy-one", name: "旧主题一" }, { ...structuredClone(doc), id: "legacy-two", name: "Legacy two" }];
            const text = JSON.stringify({ envelope_version: 1, draft: doc, saved_themes: saved, preferences: { ...before.preferences, renderer: "dom" } }, null, 2) + "\n";
            await page.evaluate(({ v1, v2, text }) => { window.__theme_studio.dispose(); localStorage.removeItem(v2); localStorage.setItem(v1, text); }, { v1, v2, text });
            await page.reload(); await page.getByTestId("status").and(page.locator("[data-status=ready]")).waitFor({ state: "attached", timeout: 90000 }); await quiet(page);
        }
        for (const [w, h] of [[1440, 900], [390, 844]]) {
            const ctx = await context(browser, w, h); const page = await ctx.newPage(); watch(page, u);
            await seed(page); log(u, `seed only ${v1} (legacy draft + two named themes), reload at ${w}`);
            await resetScroll(page);
            await shot(page, u, "upgrade-pending", "light", "localStorage v1 seeded before reload");
            if (w === 1440) {
                await tid(page, "upgrade-review-open").click(); log(u, "click upgrade Review");
                await shot(page, u, "upgrade-review", "light", "localStorage v1 seeded");
                await tid(page, "upgrade-cancel").click(); log(u, "click Cancel upgrade");
                await color(page, "primary", "#234567"); log(u, "edit primary ← #234567");
                await tid(page, "theme-save").click(); await tid(page, "theme-name").fill("Memory only"); await tid(page, "theme-name-confirm").click();
                log(u, "Save locally, name 'Memory only', confirm");
                await resetScroll(page);
                await shot(page, u, "upgrade-cancelled", "light", "localStorage v1 seeded");
                await tid(page, "upgrade-review-open").click(); log(u, "click upgrade Review again");
                await tid(page, "upgrade-confirm").click(); log(u, "click upgrade confirm");
                await page.waitForTimeout(1500); await resetScroll(page);
                await shot(page, u, "upgraded", "light", "localStorage v1 seeded");
            }
            await ctx.close();
        }
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "renderer-dom").click();
            await color(page, "primary", "#123456"); await idle(page); log(u, "primary ← #123456, wait for idle save");
            await page.evaluate(() => { window.__set = Storage.prototype.setItem; Storage.prototype.setItem = function () { throw new DOMException("Injected full device", "QuotaExceededError"); }; });
            log(u, "inject: Storage.prototype.setItem throws QuotaExceededError");
            await tid(page, "theme-save").click(); await tid(page, "theme-name").fill("Retained name"); await tid(page, "theme-name-confirm").click();
            log(u, "Save locally, name 'Retained name', confirm");
            await tid(page, "storage-issue").first().waitFor({ timeout: 10000 }).catch(() => {});
            await resetScroll(page);
            await shot(page, u, "quota-failure", "light", "setItem throws QuotaExceededError");
            await tid(page, "storage-operations").locator("summary").click(); log(u, "open pending / recent operations");
            await shot(page, u, "quota-operations", "light", "setItem throws QuotaExceededError");
            await ctx.close();
        }
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "renderer-dom").click();
            await color(page, "primary", "#123456"); await idle(page); log(u, "tab A: primary ← #123456, idle");
            const other = await ctx.newPage(); watch(other, u); await ready(other, STUDIO);
            await color(other, "primary", "#654321"); await idle(other); log(u, "tab B (same origin): primary ← #654321, idle");
            await page.bringToFront();
            await tid(page, "storage-conflict").waitFor({ timeout: 15000 });
            await resetScroll(page);
            await shot(page, u, "conflict", "light", "second same-origin tab wrote v2");
            await tid(page, "storage-conflict").evaluate(n => n.scrollIntoView({ block: "start" }));
            await shot(page, u, "conflict-detail", "light", "second same-origin tab wrote v2");
            await ctx.close();
        }
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await page.addInitScript(() => Object.defineProperty(navigator, "locks", { value: undefined, configurable: true }));
            await ready(page, STUDIO); log(u, "init script removes navigator.locks; open");
            await color(page, "primary", "#789abc");
            await tid(page, "theme-save").click(); await tid(page, "theme-name").fill("Not persisted"); await tid(page, "theme-name-confirm").click();
            log(u, "primary ← #789abc; Save locally 'Not persisted'");
            await resetScroll(page);
            await shot(page, u, "no-locks", "light", "navigator.locks removed before load");
            await ctx.close();
        }
    },

    async U5(browser) {
        const u = "U5";
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "renderer-dom").click(); log(u, "renderer DOM"); await resetScroll(page);
            await shot(page, u, "dom", "light");
            await tid(page, "mode-dark").click(); log(u, "mode Dark"); await resetScroll(page);
            await shot(page, u, "dom", "dark");
            await tid(page, "mode-light").click(); log(u, "mode Light");
            const preview = tid(page, "studio-preview");
            await preview.getByTestId("open-preview-menu").click(); log(u, "open preview menu");
            await shot(page, u, "menu", "light");
            await page.keyboard.press("Escape");
            await preview.getByTestId("open-preview-dialog").click(); log(u, "open preview dialog");
            await shot(page, u, "dialog", "light");
            await preview.getByTestId("preview-dialog-close").click().catch(() => page.keyboard.press("Escape"));
            await tid(page, "compare-scope").locator("summary").first().click(); await tid(page, "compare-dark").click();
            log(u, "open comparison scope, set its mode Dark");
            await tid(page, "compare-scope").evaluate(n => n.scrollIntoView({ block: "start" }));
            await shot(page, u, "compare-nested", "light");
            await resetScroll(page);
            await tid(page, "inspector-toggle").click(); log(u, "click Inspect");
            await preview.getByTestId("preview-primary").click(); log(u, "click preview primary button");
            await shot(page, u, "inspector", "light");
            await tid(page, "inspector-toggle").click();
            await tid(page, "renderer-compare").click(); log(u, "renderer DOM + GPU");
            for (let i = 0; i < 300; i++) { const s = await snapshot(page); if (s.gpu_state === "Ready" && s.drawn_revision === s.revision) break; await page.waitForTimeout(200); }
            await resetScroll(page);
            await shot(page, u, "gpu-compare", "light");
            await ctx.close();
        }
        {
            const ctx = await context(browser, 390, 844); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "mobile-preview").click(); log(u, "390: click Preview tab");
            await resetScroll(page);
            await shot(page, u, "preview", "light");
            await tid(page, "studio-preview").getByTestId("open-preview-dialog").click(); log(u, "390: open preview dialog");
            await shot(page, u, "dialog", "light");
            await ctx.close();
        }
    },

    // Re-capture of the Inspector: the first attempt clicked a preview node
    // that is not a declared sample, so no selection was on screen.
    async U5b(browser) {
        const u = "U5";
        const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
        await ready(page, STUDIO); await tid(page, "renderer-dom").click();
        await tid(page, "inspector-toggle").click(); log(u, "a2: click Inspect");
        const cards = tid(page, "dom-preview").getByTestId("scene-cards");
        const card = cards.getByTestId("cards-form");
        await card.scrollIntoViewIfNeeded(); await card.hover({ position: { x: 8, y: 8 } }); log(u, "a2: hover card corner");
        await shot(page, u, "inspector-card", "light");
        await cards.getByTestId("cards-submit").hover(); log(u, "a2: hover Create profile button");
        await shot(page, u, "inspector", "light", "none", null, "-a2");
        await cards.getByTestId("cards-submit").click(); log(u, "a2: click Create profile button while inspecting");
        await shot(page, u, "inspector-picked", "light");
        await ctx.close();
    },

    // Supplementary frames for cells the R1 judges reported they could not
    // read: lower dialog regions, closed option lists, hover-tinted focus
    // frames and the Inspector on an undeclared node.
    async SUP(browser) {
        const v1 = "rustify-ui.theme-studio.v1", v2 = "rustify-ui.theme-studio.v2";
        {
            const u = "U3";
            const ctx = await context(browser, 390, 844); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO);
            const values = (await snapshot(page)).values;
            values.light.primary = "#7c3aed"; values.dark.primary = "#a78bfa";
            await tid(page, "theme-import").click();
            await tid(page, "import-source").fill(JSON.stringify({ schema_version: 1, id: "review-legacy", name: "Imported legacy 旧主题", styles: values }, null, 2));
            await tid(page, "import-review").click(); log(u, "SUP 390: Import, paste valid v1, Review");
            await tid(page, "import-apply").evaluate(n => n.scrollIntoView({ block: "end" })); log(u, "SUP 390: scroll dialog to Apply");
            await shot(page, u, "import-review-end", "light");
            await ctx.close();
        }
        {
            const u = "U3";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await graphitePink(page, u);
            await tid(page, "theme-export").click(); await select(page, "export-kind", "Tailwind v4 CSS"); log(u, "SUP: Export → Tailwind v4 CSS");
            await tid(page, "export-profile").click(); log(u, "SUP: open CSS profile list");
            await shot(page, u, "export-css-profile-open", "light");
            await page.keyboard.press("Escape");
            await tid(page, "export-color-format").click(); log(u, "SUP: open color notation list");
            await shot(page, u, "export-css-notation-open", "light");
            await page.keyboard.press("Escape");
            await select(page, "export-kind", "Rust integration"); log(u, "SUP: export kind → Rust integration");
            await tid(page, "export-source").evaluate(n => { n.scrollTop = n.scrollHeight; }); log(u, "SUP: scroll exported code to its end");
            await shot(page, u, "export-rust-end", "light");
            await ctx.close();
        }
        {
            const u = "U4";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO);
            const before = await snapshot(page);
            const doc = { schema_version: 1, id: "legacy-draft", name: "旧草稿", styles: before.values };
            const saved = [{ ...structuredClone(doc), id: "legacy-one", name: "旧主题一" }, { ...structuredClone(doc), id: "legacy-two", name: "Legacy two" }];
            const text = JSON.stringify({ envelope_version: 1, draft: doc, saved_themes: saved, preferences: { ...before.preferences, renderer: "dom" } }, null, 2) + "\n";
            await page.evaluate(({ v1, v2, text }) => { window.__theme_studio.dispose(); localStorage.removeItem(v2); localStorage.setItem(v1, text); }, { v1, v2, text });
            await page.reload(); await page.getByTestId("status").and(page.locator("[data-status=ready]")).waitFor({ state: "attached", timeout: 90000 }); await quiet(page);
            log(u, "SUP: seed only v1 (draft + two named themes), reload");
            await tid(page, "upgrade-review-open").click(); log(u, "SUP: click upgrade Review");
            await tid(page, "upgrade-confirm").evaluate(n => n.scrollIntoView({ block: "center" })); log(u, "SUP: scroll editor to the upgrade confirm button");
            await shot(page, u, "upgrade-review-end", "light", "localStorage v1 seeded");
            await ctx.close();
        }
        {
            const u = "U4";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "renderer-dom").click();
            await color(page, "primary", "#123456"); await idle(page);
            const other = await ctx.newPage(); watch(other, u); await ready(other, STUDIO);
            await color(other, "primary", "#654321"); await idle(other); log(u, "SUP: tab A #123456, tab B #654321");
            await page.bringToFront(); await tid(page, "storage-conflict").waitFor({ timeout: 15000 });
            await tid(page, "storage-external-raw").locator("summary").click(); log(u, "SUP: expand View exact stored text");
            await tid(page, "storage-external-raw").evaluate(n => n.scrollIntoView({ block: "start" }));
            await shot(page, u, "conflict-raw", "light", "second same-origin tab wrote v2");
            await ctx.close();
        }
        {
            const u = "U4";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await page.addInitScript(() => Object.defineProperty(navigator, "locks", { value: undefined, configurable: true }));
            await ready(page, STUDIO); log(u, "SUP: init script removes navigator.locks; open, no action");
            await resetScroll(page);
            await shot(page, u, "no-locks-initial", "light", "navigator.locks removed before load");
            await ctx.close();
        }
        {
            const u = "U5";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "renderer-dom").click();
            await tid(page, "inspector-toggle").click(); log(u, "SUP: click Inspect"); await page.waitForTimeout(800);
            const create = tid(page, "studio-preview").getByTestId("preview-primary");
            await create.scrollIntoViewIfNeeded(); await create.hover(); log(u, "SUP: wait 800 ms, hover preview primary (Create) button");
            await shot(page, u, "inspector-undeclared-hover", "light");
            await create.click(); await page.waitForTimeout(500); log(u, "SUP: click it while inspecting, wait 500 ms");
            await shot(page, u, "inspector-undeclared", "light");
            await ctx.close();
        }
        {
            const u = "U6";
            const ctx = await context(browser, 1280, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, CATALOG);
            await page.evaluate(() => window.__component_catalog.mount_button_fixtures());
            await tid(page, "button-default").waitFor(); await quiet(page);
            await page.mouse.move(0, 0); log(u, "SUP: mount_button_fixtures(), park pointer at (0,0)");
            await tid(page, "button-ghost-4").focus(); await page.keyboard.press("Tab"); log(u, "SUP: focus ghost 4, Tab → Default action");
            await tid(page, "button-default").evaluate(n => n.scrollIntoView({ block: "center" }));
            await shot(page, u, "focus", "light", "none", null, "-a2");
            for (let i = 0; i < 4; i++) await page.keyboard.press("Tab"); log(u, "SUP: Tab ×4");
            await shot(page, u, "focus-02", "light", "none", null, "-a2");
            await tid(page, "button-scale-110").click(); await page.mouse.move(0, 0); log(u, "SUP: click Scale 110%, park pointer");
            await shot(page, u, "scale110", "light", "none", tid(page, "button-matrix"), "-a2");
            await ctx.close();
        }
        {
            const u = "U7";
            const ctx = await context(browser, 1280, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, CATALOG);
            await page.evaluate(() => window.__component_catalog.mount_layout_fixtures());
            await tid(page, "layout-typography").waitFor(); await quiet(page);
            await tid(page, "layout-invalid").click(); await page.mouse.move(0, 0); log(u, "SUP: click Invalid prop, park pointer");
            await shot(page, u, "invalid", "light", "none", tid(page, "catalog-layout-fixture"), "-a2");
            await ctx.close();
        }
    },

    async U6(browser) {
        const u = "U6";
        for (const [w, h] of [[1280, 900], [390, 844]]) {
            const ctx = await context(browser, w, h); const page = await ctx.newPage(); watch(page, u);
            await ready(page, CATALOG);
            await page.evaluate(() => window.__component_catalog.mount_button_fixtures());
            await tid(page, "button-default").waitFor(); await quiet(page);
            log(u, `catalog ${w}: mount_button_fixtures()`);
            const body = tid(page, "button-fixture-body");
            await shot(page, u, "fixture", "light", "none", body);
            if (w === 1280) {
                await tid(page, "button-mode-dark").click(); log(u, "click fixture Dark");
                await shot(page, u, "fixture", "dark", "none", body);
                await tid(page, "button-mode-light").click(); log(u, "click fixture Light");
                await tid(page, "button-toggle-loading").click(); log(u, "click Toggle loading");
                await tid(page, "button-native-form").evaluate(n => n.scrollIntoView({ block: "center" }));
                await shot(page, u, "loading", "light", "none", tid(page, "button-native-form"));
                await tid(page, "button-toggle-loading").click(); log(u, "click Toggle loading (off)");
                await tid(page, "button-ghost-4").focus(); await page.keyboard.press("Tab"); log(u, "focus ghost 4, press Tab → Default action");
                await tid(page, "button-default").evaluate(n => n.scrollIntoView({ block: "center" }));
                await shot(page, u, "focus", "light");
                await page.keyboard.press("Tab"); await page.keyboard.press("Tab"); await page.keyboard.press("Tab"); await page.keyboard.press("Tab");
                log(u, "Tab ×4 more");
                await shot(page, u, "focus-02", "light");
                await tid(page, "button-native-form").evaluate(n => n.scrollIntoView({ block: "center" }));
                await tid(page, "button-native-default").focus(); for (let i = 0; i < 3; i++) await page.keyboard.press("Tab");
                log(u, "focus Ordinary action, Tab ×3 → Unavailable request");
                await shot(page, u, "focus-unavailable", "light");
                await tid(page, "button-scale-110").click(); log(u, "click Scale 110%");
                await shot(page, u, "scale110", "light", "none", tid(page, "button-matrix"));
                await tid(page, "button-scale-100").click(); await tid(page, "button-theme-full").click(); log(u, "Scale 100%, Theme full radius");
                await shot(page, u, "radius-full", "light", "none", body);
                await tid(page, "button-panel-chinese").click(); log(u, "ThemePanel 中文");
                await shot(page, u, "panel-zh", "light", "none", tid(page, "button-panel-fixture"));
            }
            await ctx.close();
        }
        for (const [w, h] of [[1280, 900], [390, 844]]) {
            const ctx = await context(browser, w, h); const page = await ctx.newPage(); watch(page, u);
            await ready(page, CATALOG);
            await page.evaluate(() => window.__component_catalog.mount_layout_fixtures());
            await tid(page, "type-link").waitFor(); await quiet(page);
            log(u, `catalog ${w}: mount_layout_fixtures() for Link`);
            await tid(page, "type-link").evaluate(n => n.scrollIntoView({ block: "center" }));
            await shot(page, u, "links", "light");
            if (w === 1280) {
                await tid(page, "type-link").hover(); log(u, "hover type-link");
                await shot(page, u, "links-hover", "light");
                await tid(page, "type-link").focus(); log(u, "focus type-link");
                await shot(page, u, "links-focus", "light");
            }
            await ctx.close();
        }
    },

    async U7(browser) {
        const u = "U7";
        for (const [w, h] of [[1280, 900], [390, 844]]) {
            const ctx = await context(browser, w, h); const page = await ctx.newPage(); watch(page, u);
            await ready(page, CATALOG);
            await page.evaluate(() => window.__component_catalog.mount_layout_fixtures());
            await tid(page, "layout-typography").waitFor(); await quiet(page);
            log(u, `catalog ${w}: mount_layout_fixtures()`);
            const root = tid(page, "catalog-layout-fixture");
            await shot(page, u, "fixture", "light", "none", root);
            if (w === 1280) {
                await tid(page, "layout-scale-110").click(); log(u, "click scale 110");
                await shot(page, u, "scale110", "light", "none", root);
                await tid(page, "layout-scale-95").click(); log(u, "click scale 95");
                await shot(page, u, "scale95", "light", "none", root);
                await tid(page, "layout-scale-100").click(); await tid(page, "layout-invalid").click(); log(u, "scale 100, click invalid");
                await tid(page, "layout-invalid-prop").evaluate(n => n.scrollIntoView({ block: "center" }));
                await shot(page, u, "invalid", "light");
            }
            await ctx.close();
        }
    },
};

(async () => {
    const browser = await chromium.launch();
    const version = browser.version();
    const units = only.length ? only : Object.keys(cells);
    const failures = [];
    for (const unit of units) {
        try { await cells[unit](browser); }
        catch (error) { failures.push({ unit, error: String(error.stack || error) }); log(unit, `FAILED ${error.message}`); }
    }
    await browser.close();
    const prev = fs.existsSync(path.join(OUT, "capture-log.json")) ? JSON.parse(fs.readFileSync(path.join(OUT, "capture-log.json"), "utf8")) : { runs: [] };
    prev.runs.push({ at: new Date().toISOString(), chromium: version, playwright: "1.63.0", units, records, health, actions, failures });
    fs.writeFileSync(path.join(OUT, "capture-log.json"), JSON.stringify(prev, null, 2));
    console.log(JSON.stringify({ chromium: version, shots: records.length, failures }, null, 2));
})();
