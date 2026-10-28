// Author apply capture for UI review R3, DOM-COMPONENTS-P1. Drives the real release builds
// served by `mbx xtask serve` (Studio :4181, catalog :4176) and records each
// screenshot with its cell, viewport, actions and page health.
// Based on the R2 runner. The R2 supplementary frames (scrolled dialog ends,
// field tabs, lower base configuration, conflict panel end and what follows,
// dark caller override, menu/dialog inside the opposite boundary, inspector
// panel after an undeclared pick) are part of the standard runs here. Whole
// U7 fixture frames are page clips: the fixture root's box rounded outwards
// to whole CSS px plus 8px on each side (target package, R2 decision UI-97).
//
//   RUSTIFY_APPLY_OUT=<new-output-directory> node docs/plan/DOM-COMPONENTS-P1.ui-captures/r3-apply/capture.cjs [unit...]
const { chromium } = require("@playwright/test");
const fs = require("node:fs");
const path = require("node:path");

const OUT = process.env.RUSTIFY_APPLY_OUT;
if (!OUT) throw new Error("Set RUSTIFY_APPLY_OUT to a new author-capture directory.");
fs.mkdirSync(OUT, { recursive: true });
const STUDIO = "http://127.0.0.1:4181/";
const CATALOG = "http://127.0.0.1:4176/";
const only = process.argv.slice(2);
const records = [];
const health = [];
const pageHealth=new WeakMap();
async function collectHealth(ctx){for(const page of ctx.pages()){const h=pageHealth.get(page);if(h){h.url=page.url();h.csp=await page.evaluate(()=>window.__reviewCsp||[]).catch(error=>[String(error)]);}}}
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
    pageHealth.set(page,entry);
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

async function shot(page, unit, state, theme, interventions = "none", locator = null, attempt = "", clip = null) {
    await quiet(page);
    const vp = page.viewportSize();
    const dpr = await page.evaluate(() => window.devicePixelRatio);
    const file = `${unit}-${state}-${vp.width}-${theme}${attempt}.png`;
    let kind = locator ? "element" : clip ? "clip" : "viewport";
    if (locator && await locator.getAttribute("data-testid") === "catalog-layout-fixture") {
        // Page clip in document coordinates: outward-rounded root box + 8px.
        const box = await locator.evaluate(n => { const r = n.getBoundingClientRect();
            return { left: r.left + scrollX, top: r.top + scrollY, right: r.right + scrollX, bottom: r.bottom + scrollY }; });
        const x = Math.max(0, Math.floor(box.left) - 8), y = Math.max(0, Math.floor(box.top) - 8);
        const pageClip = { x, y, width: Math.ceil(box.right) + 8 - x, height: Math.ceil(box.bottom) + 8 - y };
        await page.screenshot({ path: path.join(OUT, file), animations: "disabled", fullPage: true, clip: pageClip });
        kind = `page-clip ${JSON.stringify(pageClip)} root ${JSON.stringify(box)}`;
    } else if (locator) await locator.screenshot({ path: path.join(OUT, file), animations: "disabled" });
    else await page.screenshot({ path: path.join(OUT, file), animations: "disabled", ...(clip ? { clip } : {}) });
    records.push({ file, unit, state, viewport: `${vp.width}x${vp.height}`, dpr, theme, url: page.url(),
        time: new Date().toISOString(), kind, interventions,
        focused: await page.evaluate(() => document.activeElement?.closest("[data-testid]")?.dataset.testid ?? null) });
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
    await select(page, "studio-config-accent_color", "Pink");
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

const applyChecks={};
const cells = {
    async APPLY(browser) {
        for (const [w,h] of [[1440,900],[768,1024],[390,844]]) {
            const ctx=await context(browser,w,h),page=await ctx.newPage();watch(page,"APPLY");
            await ready(page,STUDIO);await graphitePink(page,"APPLY");
            await tid(page,"editor-colors").click();await tid(page,"color-token-primary").click();
            await quiet(page);
            applyChecks[`source-${w}`]=await page.evaluate(()=>{
                const box=s=>{const r=document.querySelector(s).getBoundingClientRect();return {x:r.x,y:r.y,w:r.width,h:r.height,bottom:r.bottom};};
                return {token:box('[data-testid="color-token-primary"]'),source:box('[data-testid="token-source"]'),clear:box('[data-testid="source-clear"]'),editor:box('#studio-editor'),scrollWidth:document.querySelector('#studio-editor').scrollWidth,clientWidth:document.querySelector('#studio-editor').clientWidth};
            });
            await shot(page,"APPLY","selection-visible","light");
            if(w===1440) {
                await intoView(page,"studio-config");
                applyChecks.groupGaps=await page.locator('[data-name="ThemeOverrideCount"]').evaluateAll(nodes=>nodes.map(n=>({text:n.textContent,gap:n.nextElementSibling.getBoundingClientRect().top-n.getBoundingClientRect().bottom,color:getComputedStyle(n).color})));
                await tid(page,"group-override-tokens").locator("summary").click();
                await intoView(page,"group-override-tokens");await shot(page,"APPLY","group-tokens","light");
                const reset=tid(page,"color-reset-primary");await reset.focus();await page.keyboard.press("Shift+Tab");await page.keyboard.press("Tab");
                applyChecks.reset={label:await reset.getAttribute("aria-label"),title:await reset.getAttribute("title")};
                await shot(page,"APPLY","reset-focus","light");
                await page.keyboard.press("Escape");await resetScroll(page);await tid(page,"renderer-dom").click();
                const widths=()=>page.locator('[data-testid="preview-toolbar"] button').evaluateAll(nodes=>nodes.map(n=>({id:n.dataset.testid,w:n.getBoundingClientRect().width,x:n.getBoundingClientRect().x})));
                const before=await widths();await tid(page,"inspector-toggle").click();await quiet(page);const after=await widths();
                applyChecks.toggleWidths={before,after};
            }
            await collectHealth(ctx); await ctx.close();
        }
        const ctx=await context(browser,1440,900),page=await ctx.newPage();watch(page,"APPLY");await ready(page,STUDIO);
        await page.setViewportSize({width:720,height:900});await page.evaluate(()=>document.documentElement.style.fontSize="200%");
        await tid(page,"editor-colors").click();await tid(page,"color-token-background").evaluate(n=>n.scrollIntoView({block:"start"}));await page.mouse.move(0,0);
        await shot(page,"APPLY","zoom200-grid","light");await tid(page,"color-token-primary").click(); await quiet(page);
        await tid(page,"token-source").evaluate(n=>n.scrollIntoView({block:"start"}));await quiet(page);
        await shot(page,"APPLY","zoom200-source","light");
        applyChecks.zoom200=await page.evaluate(()=>{
            const ed=document.querySelector('#studio-editor'),pre=document.querySelector('.studio-previews'),bar=document.querySelector('.studio-editor-top'),value=document.querySelector('[data-testid="source-effective"]');
            const r=value.getBoundingClientRect();return {editor:{scrollWidth:ed.scrollWidth,clientWidth:ed.clientWidth,scrollLeft:ed.scrollLeft},preview:{scrollWidth:pre.scrollWidth,clientWidth:pre.clientWidth},toolbarHeight:bar.getBoundingClientRect().height,toolbarRatio:bar.getBoundingClientRect().height/innerHeight,effective:{text:value.textContent,left:r.left,right:r.right}};
        });
        await tid(page,"theme-export").click();await quiet(page);applyChecks.zoom200.output=await tid(page,"export-kind").evaluate(n=>({text:n.textContent,width:n.getBoundingClientRect().width,scrollWidth:n.scrollWidth,clientWidth:n.clientWidth}));
        await shot(page,"APPLY","zoom200-export","light");await collectHealth(ctx); await ctx.close();
        fs.writeFileSync(path.join(OUT,"apply-checks.json"),JSON.stringify(applyChecks,null,2));
    },
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
            await collectHealth(ctx); await ctx.close();
        }
        {
            const ctx = await context(browser, 390, 844); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); log(u, "open at 390x844");
            await shot(page, u, "edit", "light");
            await graphitePink(page, u); await intoView(page, "studio-config");
            await shot(page, u, "configured", "light");
            await tid(page, "mobile-preview").click(); log(u, "click Preview tab"); await resetScroll(page);
            await shot(page, u, "preview", "light");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO);
            await page.setViewportSize({ width: 720, height: 900 });
            await page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
            log(u, "open at 1440x900, then viewport 720x900 and root font-size 200% (WCAG reflow technique)");
            await shot(page, u, "zoom200", "light");
            await tid(page, "editor-colors").click();
            await tid(page, "color-token-primary").click();
            await shot(page, u, "zoom200-colors", "light");
            await tid(page, "editor-colors").focus(); await page.keyboard.press("End"); log(u, "focus Colors tab, press End");
            await tid(page, "theme-export").focus(); await page.keyboard.press("Enter"); log(u, "focus Export, press Enter");
            await shot(page, u, "zoom200-export", "light");
            await collectHealth(ctx); await ctx.close();
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
            await collectHealth(ctx); await ctx.close();
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
            await collectHealth(ctx); await ctx.close();
        }
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO);
            await color(page, "background", "#ffffff"); await color(page, "foreground", "#ffffff");
            log(u, "background ← #ffffff, foreground ← #ffffff");
            await page.locator(".studio-diagnostics > summary").click(); log(u, "open diagnostics");
            await page.locator(".studio-diagnostics").evaluate(n => n.scrollIntoView({ block: "start" }));
            await shot(page, u, "low-contrast", "light");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const ctx = await context(browser, 390, 844); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await graphitePink(page, u);
            await tid(page, "editor-colors").click(); await tid(page, "color-token-primary").click(); log(u, "Colors tab → primary at 390");
            await tid(page, "source-kind").first().evaluate(n => n.scrollIntoView({ block: "center" }));
            await shot(page, u, "source-overridden", "light");
            await collectHealth(ctx); await ctx.close();
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
            await page.getByTestId("import-apply").evaluate(n => n.scrollIntoView({ block: "end" })); log(u, "scroll dialog to Apply");
            await shot(page, u, "import-review-end", "light");
            if (w === 1440) {
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
                await select(page, "export-kind", "CSS · resolved values"); log(u, "export kind → CSS · resolved values");
                await shot(page, u, "export-css", "light");
                await tid(page, "export-profile").click(); log(u, "open CSS profile list");
                await shot(page, u, "export-css-profile-open", "light");
                await page.keyboard.press("Escape"); log(u, "Escape (close list)");
                await tid(page, "export-color-format").click(); log(u, "open color notation list");
                await shot(page, u, "export-css-notation-open", "light");
                await page.keyboard.press("Escape"); log(u, "Escape (close list)");
                await select(page, "export-kind", "Rust integration"); log(u, "export kind → Rust integration");
                await shot(page, u, "export-rust", "light");
                await tid(page, "export-copy").click(); log(u, "click Copy (clipboard permission granted)");
                await page.waitForTimeout(300);
                await shot(page, u, "export-copied", "light");
                await tid(page, "export-source").evaluate(n => { n.scrollTop = n.scrollHeight; }); log(u, "scroll exported code to its end");
                await shot(page, u, "export-rust-end", "light");
            }
            await collectHealth(ctx); await ctx.close();
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
                await tid(page, "upgrade-confirm").evaluate(n => n.scrollIntoView({ block: "center" })); log(u, "scroll to the upgrade confirm button");
                await shot(page, u, "upgrade-review-end", "light", "localStorage v1 seeded");
                await tid(page, "upgrade-cancel").click(); log(u, "click Cancel upgrade");
                await color(page, "primary", "#234567"); log(u, "edit primary ← #234567");
                await tid(page, "theme-save").click(); await tid(page, "theme-name").fill("Memory only"); await tid(page, "theme-name-confirm").click();
                log(u, "Save locally, name 'Memory only', confirm");
                await resetScroll(page);
                await shot(page, u, "upgrade-cancelled", "light", "localStorage v1 seeded");
                const downloadEvent=page.waitForEvent("download");
                await tid(page,"storage-export").click();
                const file=await downloadEvent;
                const downloaded=JSON.parse(fs.readFileSync(await file.path(),"utf8"));
                if(downloaded.overrides.light.primary!=="#234567") throw new Error("cancelled current-theme download lost the memory edit");
                log(u,`Download current theme actually downloaded ${file.suggestedFilename()}; primary ${downloaded.overrides.light.primary}`);

                await tid(page, "upgrade-review-open").click(); log(u, "click upgrade Review again");
                await tid(page, "upgrade-confirm").click(); log(u, "click upgrade confirm");
                await page.waitForTimeout(1500); await resetScroll(page);
                await shot(page, u, "upgraded", "light", "localStorage v1 seeded");
            }
            await collectHealth(ctx); await ctx.close();
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
            await collectHealth(ctx); await ctx.close();
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
            await tid(page, "storage-external-raw").locator("summary").click(); log(u, "expand View exact stored text");
            await tid(page, "storage-external-raw").evaluate(n => n.scrollIntoView({ block: "start" }));
            await shot(page, u, "conflict-raw", "light", "second same-origin tab wrote v2");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await page.addInitScript(() => Object.defineProperty(navigator, "locks", { value: undefined, configurable: true }));
            await ready(page, STUDIO); log(u, "init script removes navigator.locks; open, no action");
            await resetScroll(page);
            await shot(page, u, "no-locks-initial", "light", "navigator.locks removed before load");
            await color(page, "primary", "#789abc");
            await tid(page, "theme-save").click(); await tid(page, "theme-name").fill("Not persisted"); await tid(page, "theme-name-confirm").click();
            log(u, "primary ← #789abc; Save locally 'Not persisted'");
            await resetScroll(page);
            await shot(page, u, "no-locks", "light", "navigator.locks removed before load");
            await collectHealth(ctx); await ctx.close();
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
            await select(preview, "local-appearance", "Opposite to root"); log(u, "local boundary appearance → Opposite to root");
            await preview.getByTestId("local-card").scrollIntoViewIfNeeded();
            await shot(page, u, "local-opposite", "light");
            await select(preview, "local-appearance", "Inherit root"); log(u, "local boundary appearance → Inherit root");
            await resetScroll(page);
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
            await tid(page, "renderer-compare").click(); log(u, "renderer DOM + GPU");
            for (let i = 0; i < 300; i++) { const s = await snapshot(page); if (s.gpu_state === "Ready" && s.drawn_revision === s.revision) break; await page.waitForTimeout(200); }
            await resetScroll(page);
            await shot(page, u, "gpu-compare", "light");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "renderer-dom").click();
            await tid(page, "inspector-toggle").click(); log(u, "click Inspect"); await page.waitForTimeout(800);
            const cards = tid(page, "dom-preview").getByTestId("scene-cards");
            const card = cards.getByTestId("cards-form");
            await card.scrollIntoViewIfNeeded(); await card.hover({ position: { x: 8, y: 8 } }); log(u, "wait 800 ms, hover card corner");
            await shot(page, u, "inspector-card", "light");
            await cards.getByTestId("cards-submit").hover(); log(u, "hover Create profile button");
            await shot(page, u, "inspector", "light");
            await cards.getByTestId("cards-submit").click(); log(u, "click Create profile button while inspecting");
            await shot(page, u, "inspector-picked", "light");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "renderer-dom").click();
            await tid(page, "inspector-toggle").click(); log(u, "click Inspect"); await page.waitForTimeout(800);
            const create = tid(page, "studio-preview").getByTestId("preview-primary");
            await create.scrollIntoViewIfNeeded(); await create.hover(); log(u, "wait 800 ms, hover preview primary (Create) button");
            await shot(page, u, "inspector-undeclared-hover", "light");
            await create.click(); await page.waitForTimeout(500); log(u, "click it while inspecting, wait 500 ms");
            await shot(page, u, "inspector-undeclared", "light");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const ctx = await context(browser, 390, 844); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "mobile-preview").click(); log(u, "390: click Preview tab");
            await resetScroll(page);
            await shot(page, u, "preview", "light");
            await tid(page, "studio-preview").getByTestId("open-preview-dialog").click(); log(u, "390: open preview dialog");
            await shot(page, u, "dialog", "light");
            await collectHealth(ctx); await ctx.close();
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
                await tid(page, "button-mode-dark").click(); await page.mouse.move(0, 0); log(u, "click fixture Dark, park pointer at (0,0)");
                await shot(page, u, "fixture", "dark", "none", body);
                await tid(page, "button-mode-light").click(); log(u, "click fixture Light");
                await tid(page, "button-toggle-loading").click(); await page.mouse.move(0, 0); log(u, "click Toggle loading, park pointer");
                await tid(page, "button-native-form").evaluate(n => n.scrollIntoView({ block: "center" }));
                await shot(page, u, "loading", "light", "none", tid(page, "button-native-form"));
                await tid(page, "button-toggle-loading").click(); await page.mouse.move(0, 0); log(u, "click Toggle loading (off), park pointer");
                await tid(page, "button-ghost-4").focus(); await page.keyboard.press("Tab"); log(u, "focus ghost 4, press Tab → Default action");
                await tid(page, "button-default").evaluate(n => n.scrollIntoView({ block: "center" }));
                await shot(page, u, "focus", "light");
                for (let i = 0; i < 4; i++) await page.keyboard.press("Tab");
                log(u, "Tab ×4 more");
                await shot(page, u, "focus-02", "light");
                await tid(page, "button-native-form").evaluate(n => n.scrollIntoView({ block: "center" }));
                await tid(page, "button-native-default").focus(); for (let i = 0; i < 3; i++) await page.keyboard.press("Tab");
                log(u, "focus Ordinary action, Tab ×3 → Unavailable request");
                await shot(page, u, "focus-unavailable", "light");
                await tid(page, "button-scale-110").click(); await page.mouse.move(0, 0); log(u, "click Scale 110%, park pointer");
                await shot(page, u, "scale110", "light", "none", tid(page, "button-matrix"));
                await tid(page, "button-scale-100").click(); await tid(page, "button-theme-full").click(); await page.mouse.move(0, 0); log(u, "Scale 100%, Theme full radius, park pointer");
                await shot(page, u, "radius-full", "light", "none", body);
                await tid(page, "button-panel-chinese").click(); await page.mouse.move(0, 0); log(u, "ThemePanel 中文, park pointer");
                await shot(page, u, "panel-zh", "light", "none", tid(page, "button-panel-fixture"));
            }
            await collectHealth(ctx); await ctx.close();
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
                await page.mouse.move(0, 0);
                await tid(page, "type-link").focus(); await page.keyboard.press("Shift+Tab"); await page.keyboard.press("Tab");
                log(u, "park pointer; focus type-link, Shift+Tab, Tab (keyboard focus on type-link)");
                await shot(page, u, "links-focus", "light");
            }
            await collectHealth(ctx); await ctx.close();
        }
    },

    // B2 focus close-ups: every variant at size 2, a component colour, the
    // IconButton and the Link, each reached with the keyboard, at DPR 2.
    async U6F(browser) {
        const u = "U6";
        const ctx = await context(browser, 1280, 900, { deviceScaleFactor: 2 }); const page = await ctx.newPage(); watch(page, u);
        await ready(page, CATALOG);
        await page.evaluate(() => window.__component_catalog.mount_button_fixtures());
        await tid(page, "button-default").waitFor(); await quiet(page); await page.mouse.move(0, 0);
        log(u, "DPR 2 context at 1280x900: mount_button_fixtures(), park pointer");
        async function closeUp(id, state, theme = "light") {
            await tid(page, id).focus(); await page.keyboard.press("Shift+Tab"); await page.keyboard.press("Tab");
            await tid(page, id).evaluate(n => n.scrollIntoView({ block: "center" }));
            const focused = await page.evaluate(() => document.activeElement?.dataset?.testid ?? document.activeElement?.closest("[data-testid]")?.dataset.testid);
            if (focused !== id) throw new Error(`focus landed on ${focused}, not ${id}`);
            const box = await tid(page, id).boundingBox();
            const pad = 24;
            const clip = { x: Math.max(0, box.x - pad), y: Math.max(0, box.y - pad), width: box.width + pad * 2, height: box.height + pad * 2 };
            log(u, `keyboard focus ${id} (focus, Shift+Tab, Tab); clip ±${pad}px`);
            await shot(page, u, state, theme, "none", null, "-dpr2", clip);
        }
        for (const variant of ["classic", "solid", "soft", "surface", "outline", "ghost-offset", "ghost"]) await closeUp(`button-${variant}-2`, `focus-${variant}`);
        await closeUp("button-color-red", "focus-color-red");
        await closeUp("button-high-contrast", "focus-high-contrast");
        await closeUp("button-icon", "focus-icon");
        await tid(page, "button-mode-dark").click(); await page.mouse.move(0, 0); log(u, "click fixture Dark, park pointer");
        for (const variant of ["solid", "soft", "outline"]) await closeUp(`button-${variant}-2`, `focus-${variant}`, "dark");
        await collectHealth(ctx); await ctx.close();
        const ctx2 = await context(browser, 1280, 900, { deviceScaleFactor: 2 }); const page2 = await ctx2.newPage(); watch(page2, u);
        await ready(page2, CATALOG);
        await page2.evaluate(() => window.__component_catalog.mount_layout_fixtures());
        await tid(page2, "type-link").waitFor(); await quiet(page2); await page2.mouse.move(0, 0);
        log(u, "DPR 2 context at 1280x900: mount_layout_fixtures(), park pointer");
        await tid(page2, "type-link").focus(); await page2.keyboard.press("Shift+Tab"); await page2.keyboard.press("Tab");
        await tid(page2, "type-link").evaluate(n => n.scrollIntoView({ block: "center" }));
        const box = await tid(page2, "type-link").boundingBox();
        log(u, "keyboard focus type-link (focus, Shift+Tab, Tab); clip ±24px");
        await shot(page2, u, "focus-link", "light", "none", null, "-dpr2", { x: box.x - 24, y: box.y - 24, width: box.width + 48, height: box.height + 48 });
        await collectHealth(ctx2); await ctx2.close();
    },

    async U7(browser) {
        const u = "U7";
        const sections = ["type-heading-matrix", "type-text-matrix", "type-code-matrix", "type-kbd-matrix", "type-blockquote-matrix",
            "type-wrap-matrix", "layout-container-matrix", "layout-inset-demonstration", "layout-reset-comparison"];
        for (const [w, h] of [[1280, 900], [390, 844]]) {
            const ctx = await context(browser, w, h); const page = await ctx.newPage(); watch(page, u);
            await ready(page, CATALOG);
            await page.evaluate(() => window.__component_catalog.mount_layout_fixtures());
            await tid(page, "layout-typography").waitFor(); await quiet(page);
            log(u, `catalog ${w}: mount_layout_fixtures()`);
            const root = tid(page, "catalog-layout-fixture");
            await shot(page, u, "fixture", "light", "none", root);
            for (const id of sections) { log(u, `section element ${id}`); await shot(page, u, id, "light", "none", tid(page, id)); }
            if (w === 1280) {
                await tid(page, "layout-scale-110").click(); await page.mouse.move(0, 0); log(u, "click scale 110, park pointer");
                await shot(page, u, "scale110", "light", "none", root);
                await shot(page, u, "scale110-heading", "light", "none", tid(page, "type-heading-matrix"));
                await tid(page, "layout-scale-95").click(); await page.mouse.move(0, 0); log(u, "click scale 95, park pointer");
                await shot(page, u, "scale95", "light", "none", root);
                await shot(page, u, "scale95-heading", "light", "none", tid(page, "type-heading-matrix"));
                await tid(page, "layout-scale-100").click(); await tid(page, "layout-invalid").click(); await page.mouse.move(0, 0); log(u, "scale 100, click Invalid prop, park pointer");
                await shot(page, u, "invalid", "light", "none", root);
                await tid(page, "layout-invalid-prop").evaluate(n => n.scrollIntoView({ block: "center" }));
                await shot(page, u, "invalid-detail", "light");
            }
            await collectHealth(ctx); await ctx.close();
        }
    },
    // Frames R2 added as supplements, now part of every round.
    async EXTRA(browser) {
        // Scroll every clipped scroller inside the element (and the element) to its end.
        async function scrollEnds(page, unit, id) {
            const moved = await page.getByTestId(id).first().evaluate(root => {
                const out = [];
                for (const n of [root, ...root.querySelectorAll("*")]) {
                    const oy = getComputedStyle(n).overflowY;
                    if (n.scrollHeight > n.clientHeight + 1 && oy !== "visible") {
                        const from = n.scrollTop; n.scrollTop = n.scrollHeight;
                        out.push(`${n.tagName.toLowerCase()}${n.dataset.testid ? `[${n.dataset.testid}]` : ""}${typeof n.className === "string" && n.className ? "." + n.className.trim().split(/\s+/).slice(0, 2).join(".") : ""} overflow-y:${oy} scrollTop ${from}→${n.scrollTop}`);
                    }
                }
                return out;
            });
            log(unit, `scroll clipped scrollers inside ${id} to their end: ${moved.join("; ") || "none"}`);
        }
        {
            const u = "U1";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO);
            await page.setViewportSize({ width: 720, height: 900 });
            await page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
            log(u, "EXTRA: open at 1440x900, then viewport 720x900 and root font-size 200%");
            await tid(page, "editor-colors").focus(); await page.keyboard.press("End"); log(u, "EXTRA: focus Colors tab, press End");
            await tid(page, "theme-export").focus(); await page.keyboard.press("Enter"); log(u, "EXTRA: focus Export, press Enter");
            await scrollEnds(page, u, "data-tools-dialog");
            await shot(page, u, "zoom200-export-end", "light");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const u = "U1";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); log(u, "EXTRA: open at 1440x900");
            await tid(page, "editor-colors").evaluate(n => n.scrollIntoView({ block: "center" })); await page.mouse.move(0, 0);
            log(u, "EXTRA: scroll field tabs (Colors / Typography & other) to the middle of the editor, park pointer");
            await shot(page, u, "field-tabs", "light");
            await tid(page, "editor-colors").focus(); await page.keyboard.press("Shift+Tab"); await page.keyboard.press("Tab");
            log(u, "EXTRA: keyboard focus on Colors tab (focus, Shift+Tab, Tab)");
            await shot(page, u, "field-tabs-focus", "light");
            await graphitePink(page, u);
            await tid(page, "studio-config-scaling").evaluate(n => n.scrollIntoView({ block: "end" })); await page.mouse.move(0, 0);
            log(u, "EXTRA: scroll base configuration down to Scaling, park pointer");
            await shot(page, u, "configured-lower", "light");
            await collectHealth(ctx); await ctx.close();
        }
        for (const [w, h] of [[1440, 900], [390, 844]]) {
            const u = "U3";
            const ctx = await context(browser, w, h); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO);
            const values = (await snapshot(page)).values;
            values.light.primary = "#7c3aed"; values.dark.primary = "#a78bfa";
            await tid(page, "theme-import").click();
            await tid(page, "import-source").fill(JSON.stringify({ schema_version: 1, id: "review-legacy", name: "Imported legacy 旧主题", styles: values }, null, 2));
            await tid(page, "import-review").click(); log(u, `EXTRA ${w}: Import, paste valid v1 (primary changed), Review`);
            await scrollEnds(page, u, "data-tools-dialog");
            await shot(page, u, "import-review-bottom", "light");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const u = "U4";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "renderer-dom").click();
            await color(page, "primary", "#123456"); await idle(page);
            const other = await ctx.newPage(); watch(other, u); await ready(other, STUDIO);
            await color(other, "primary", "#654321"); await idle(other); log(u, "EXTRA: tab A #123456, tab B #654321");
            await page.bringToFront(); await tid(page, "storage-conflict").waitFor({ timeout: 15000 });
            await tid(page, "storage-conflict").evaluate(n => n.scrollIntoView({ block: "end" }));
            log(u, "EXTRA: scroll so the bottom edge of the conflict panel meets the viewport bottom");
            await shot(page, u, "conflict-detail-end", "light", "second same-origin tab wrote v2");
            const moved = await page.evaluate(() => {
                let s = document.querySelector('[data-testid="storage-conflict"]').parentElement;
                while (s && !(s.scrollHeight > s.clientHeight + 1 && /(auto|scroll)/.test(getComputedStyle(s).overflowY))) s = s.parentElement;
                s = s || document.scrollingElement; const from = s.scrollTop; s.scrollBy(0, 300);
                return `${s.tagName.toLowerCase()}${s.id ? "#" + s.id : ""} scrollTop ${from}→${s.scrollTop}`;
            });
            log(u, `EXTRA: scroll the conflict panel's scrolling ancestor 300px further (${moved})`);
            await shot(page, u, "conflict-after", "light", "second same-origin tab wrote v2");
            const diff = page.getByTestId("storage-conflict-diff");
            if (await diff.count()) {
                await diff.locator("summary").first().click(); log(u, "EXTRA: open 'Draft differences' disclosure in the conflict panel");
                await diff.evaluate(n => n.scrollIntoView({ block: "start" }));
                await shot(page, u, "conflict-diff", "light", "second same-origin tab wrote v2");
            } else log(u, "EXTRA: no storage-conflict-diff disclosure present");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const u = "U5";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "renderer-dom").click();
            await tid(page, "mode-dark").click(); await page.mouse.move(0, 0); log(u, "EXTRA: renderer DOM, mode Dark, park pointer");
            await tid(page, "studio-preview").getByTestId("class-override").evaluate(n => n.scrollIntoView({ block: "center" }));
            log(u, "EXTRA: scroll preview to Caller class override");
            await shot(page, u, "dom-caller", "dark");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const u = "U5";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "renderer-dom").click();
            const preview = tid(page, "studio-preview");
            await select(preview, "local-appearance", "Opposite to root"); log(u, "EXTRA: local boundary appearance → Opposite to root");
            await preview.getByTestId("open-preview-menu").click(); log(u, "EXTRA: open Actions menu inside the local boundary");
            await shot(page, u, "local-opposite-menu", "light");
            await page.keyboard.press("Escape"); log(u, "EXTRA: Escape");
            await preview.getByTestId("open-preview-dialog").click(); log(u, "EXTRA: Open dialog inside the local boundary");
            await shot(page, u, "local-opposite-dialog", "light");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const u = "U5";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await tid(page, "renderer-dom").click();
            await tid(page, "inspector-toggle").click(); log(u, "EXTRA: click Inspect"); await page.waitForTimeout(800);
            const create = tid(page, "studio-preview").getByTestId("preview-primary");
            await create.scrollIntoViewIfNeeded(); await create.click(); await page.waitForTimeout(500);
            log(u, "EXTRA: wait 800 ms, click preview primary (Create) button while inspecting, wait 500 ms");
            await tid(page, "inspector-panel").evaluate(n => n.scrollIntoView({ block: "start" }));
            log(u, "EXTRA: scroll the inspector panel into view");
            await shot(page, u, "inspector-undeclared-panel", "light");
            await collectHealth(ctx); await ctx.close();
        }
    },
    // Supplementary frames for in-matrix cells the R3 judges reported they
    // could not read: the document restore entry (U2), the middle of the Rust
    // integration text and of the 390 import review (U3), and the expanded
    // complete migration JSON in the upgrade review (U4).
    async SUP(browser) {
        {
            const u = "U2";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await graphitePink(page, u);
            await resetScroll(page);
            await tid(page, "theme-reset").evaluate(n => n.scrollIntoView({ block: "center" })); await page.mouse.move(0, 0);
            log(u, "SUP: scroll the editor so the document restore entry (theme-reset) is in view, park pointer");
            await shot(page, u, "restore", "light");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const u = "U3";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO); await graphitePink(page, u);
            await tid(page, "theme-export").click(); await select(page, "export-kind", "Rust integration"); log(u, "SUP: open Export, kind → Rust integration");
            const moved = await tid(page, "export-source").evaluate(n => {
                const text = n.value ?? n.textContent; const at = text.indexOf(".woff2");
                if (at < 0) return "no .woff2 in exported text";
                const line = text.slice(0, at).split("\n").length, lines = text.split("\n").length;
                n.scrollTop = Math.max(0, n.scrollHeight * (line - 3) / lines);
                return `first .woff2 on line ${line} of ${lines}; scrollTop ${n.scrollTop}`;
            });
            log(u, `SUP: scroll the exported code to the first font file (${moved})`);
            await shot(page, u, "export-rust-mid", "light");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const u = "U3";
            const ctx = await context(browser, 390, 844); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO);
            const values = (await snapshot(page)).values;
            values.light.primary = "#7c3aed"; values.dark.primary = "#a78bfa";
            await tid(page, "theme-import").click();
            await tid(page, "import-source").fill(JSON.stringify({ schema_version: 1, id: "review-legacy", name: "Imported legacy 旧主题", styles: values }, null, 2));
            await tid(page, "import-review").click(); log(u, "SUP 390: Import, paste valid v1 (primary changed), Review");
            const target = tid(page, "data-tools-dialog").getByText(/^Identity/).first();
            await target.evaluate(n => n.scrollIntoView({ block: "center" }));
            log(u, "SUP 390: scroll the dialog so the Identity group is in the middle of the viewport");
            await shot(page, u, "import-review-mid", "light");
            await collectHealth(ctx); await ctx.close();
        }
        {
            const u = "U4";
            const v2 = "rustify-ui.theme-studio.v2", v1 = "rustify-ui.theme-studio.v1";
            const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
            await ready(page, STUDIO);
            const before = await snapshot(page);
            const doc = { schema_version: 1, id: "legacy-draft", name: "旧草稿", styles: before.values };
            const saved = [{ ...structuredClone(doc), id: "legacy-one", name: "旧主题一" }, { ...structuredClone(doc), id: "legacy-two", name: "Legacy two" }];
            const text = JSON.stringify({ envelope_version: 1, draft: doc, saved_themes: saved, preferences: { ...before.preferences, renderer: "dom" } }, null, 2) + "\n";
            await page.evaluate(({ v1, v2, text }) => { window.__theme_studio.dispose(); localStorage.removeItem(v2); localStorage.setItem(v1, text); }, { v1, v2, text });
            await page.reload(); await page.getByTestId("status").and(page.locator("[data-status=ready]")).waitFor({ state: "attached", timeout: 90000 }); await quiet(page);
            log(u, `SUP: seed only ${v1} (legacy draft + two named themes), reload`);
            await tid(page, "upgrade-review-open").click(); log(u, "SUP: click upgrade Review");
            const summary = page.getByText("Complete library upgrade JSON", { exact: true }).first();
            await summary.click(); log(u, "SUP: expand Complete library upgrade JSON");
            await summary.evaluate(n => n.scrollIntoView({ block: "start" })); await page.mouse.move(0, 0);
            await shot(page, u, "upgrade-review-json", "light", "localStorage v1 seeded");
            await collectHealth(ctx); await ctx.close();
        }
    },
    // Retake of the Rust middle frame: the exported text has no ".woff2", so the
    // first SUP frame never scrolled. Scroll the code to its vertical middle.
    async SUP2(browser) {
        const u = "U3";
        const ctx = await context(browser, 1440, 900); const page = await ctx.newPage(); watch(page, u);
        await ready(page, STUDIO); await graphitePink(page, u);
        await tid(page, "theme-export").click(); await select(page, "export-kind", "Rust integration"); log(u, "SUP2: open Export, kind → Rust integration");
        const moved = await tid(page, "export-source").evaluate(n => { n.scrollTop = (n.scrollHeight - n.clientHeight) / 2; return `scrollTop ${n.scrollTop} of ${n.scrollHeight - n.clientHeight}`; });
        log(u, `SUP2: scroll the exported code to its vertical middle (${moved})`);
        await shot(page, u, "export-rust-mid", "light", "none", null, "-a2");
        await collectHealth(ctx); await ctx.close();
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
