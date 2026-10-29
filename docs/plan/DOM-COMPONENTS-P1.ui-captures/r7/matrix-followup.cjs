// UI review R7 capture for DOM-COMPONENTS-P1. Drives the real release builds
// served by `mbx xtask serve` (Studio :4181, catalog :4176) and records each
// screenshot with its cell, viewport, actions and page health (including CSP
// violations collected from each page before its context closes).
// Based on the R3 runner. The R3 supplementary frames (document restore entry,
// Rust integration middle, 390 import review middle, expanded complete upgrade
// JSON) are part of the standard runs here, and U1 · 200% adds the Colors page
// grid and a selected token's source block (target package, R4 capture change).
// Based on the R2 runner. The R2 supplementary frames (scrolled dialog ends,
// field tabs, lower base configuration, conflict panel end and what follows,
// dark caller override, menu/dialog inside the opposite boundary, inspector
// panel after an undeclared pick) are part of the standard runs here. Whole
// U7 fixture frames are page clips: the fixture root's box rounded outwards
// to whole CSS px plus 8px on each side (target package, R2 decision UI-97).
//
//   node docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/capture-runner.cjs [unit...]
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

const pageHealth = new WeakMap();
async function collectHealth(ctx) {
    for (const page of ctx.pages()) {
        const h = pageHealth.get(page);
        if (h) { h.url = page.url(); h.csp = await page.evaluate(() => window.__reviewCsp || []).catch(e => [String(e)]); }
    }
}
function watch(page, unit) {
    const entry = { unit, url: null, pageErrors: [], consoleErrors: [], failed: [], csp: [] };
    pageHealth.set(page, entry);
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

async function shot(page, unit, state, theme, interventions = "none", locator = null, attempt = "", clip = null) {
    await quiet(page);
    const vp = page.viewportSize();
    const dpr = await page.evaluate(() => window.devicePixelRatio);
    const file = `${unit}-${state}-${vp.width}-${theme}${attempt}.png`;
    if (fs.existsSync(path.join(OUT, file))) throw new Error(`refusing to replace prior capture ${file}`);
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
    fs.writeFileSync(path.join(OUT, "capture-progress.json"), JSON.stringify({ records, actions }, null, 2));
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

const cells={}; const originalShot=shot;shot=async(...args)=>{args[2]="matrix2-"+args[2];return originalShot(...args)};
cells.DARK1=async browser=>{const u='U1',ctx=await context(browser,1440,900),page=await ctx.newPage();watch(page,u);await ready(page,STUDIO);await graphitePink(page,u);await tid(page,'mode-dark').click();await tid(page,'locale-zh').click();await resetScroll(page);await shot(page,u,'zh-top','dark');await intoView(page,'studio-config');await shot(page,u,'zh-configured','dark');await tid(page,'locale-en').click();await tid(page,'preset-search').focus();await page.keyboard.press('Shift+Tab');await page.keyboard.press('Tab');await shot(page,u,'keyboard-01','dark');await tid(page,'studio-config-accent_color').focus();await page.keyboard.press('Shift+Tab');await page.keyboard.press('Tab');await shot(page,u,'keyboard-02','dark');await page.keyboard.press('Enter');await page.keyboard.press('ArrowDown');await shot(page,u,'keyboard-03','dark');await page.keyboard.press('Escape');await tid(page,'mode-dark').focus();await page.keyboard.press('Shift+Tab');await page.keyboard.press('Tab');await shot(page,u,'keyboard-04','dark');log(u,'matrix2: English configuration first, then Chinese dark top/config and real-keyboard dark focus frames; repairs failed locale-dependent selector in first matrix attempt');await collectHealth(ctx);await ctx.close()};
cells.LINK6=async browser=>{const u='U6';{
 const ctx=await context(browser,390,844),page=await ctx.newPage();watch(page,u);await ready(page,CATALOG);await page.evaluate(()=>window.__component_catalog.mount_layout_fixtures());const link=tid(page,'type-link');await link.scrollIntoViewIfNeeded();await link.hover();await shot(page,u,'links-hover','light');await page.mouse.move(0,0);await link.focus();await page.keyboard.press('Shift+Tab');await page.keyboard.press('Tab');await shot(page,u,'links-focus','light');log(u,'390 catalog: Link hover and real keyboard focus');await collectHealth(ctx);await ctx.close()}
 const ctx=await context(browser,1280,900),page=await ctx.newPage();watch(page,u);await ready(page,STUDIO);await tid(page,'renderer-dom').click();await tid(page,'mode-dark').click();const link=tid(page,'studio-preview').getByRole('link',{name:'Edit theme',exact:true});await link.scrollIntoViewIfNeeded();await shot(page,u,'link-studio','dark');await link.hover();await shot(page,u,'link-studio-hover','dark');await page.mouse.move(0,0);await link.focus();await page.keyboard.press('Shift+Tab');await page.keyboard.press('Tab');await shot(page,u,'link-studio-focus','dark');log(u,'1280 dark SDK Link in real Studio DOM preview; no product DOM/style injection');await collectHealth(ctx);await ctx.close()};
cells.MOBILE5=async browser=>{const u='U5',ctx=await context(browser,390,844),page=await ctx.newPage();watch(page,u);await ready(page,STUDIO);await tid(page,'mobile-preview').click();await tid(page,'renderer-dom').click();await tid(page,'inspector-toggle').click();const button=tid(page,'dom-preview').getByTestId('scene-cards').getByTestId('cards-submit');await button.scrollIntoViewIfNeeded();await button.click();await quiet(page);log(u,'390: inspect actual Create profile; product opens source in Edit tab');await tid(page,'mobile-preview').click();log(u,'390: return to Preview through actual tab after source selection');if(await tid(page,'inspector-toggle').getAttribute('aria-pressed')==='true')await tid(page,'inspector-toggle').click();await tid(page,'renderer-compare').click();for(let i=0;i<300;i++){const s=await snapshot(page);if(s.gpu_state==='Ready'&&s.drawn_revision===s.revision)break;await page.waitForTimeout(200)}await resetScroll(page);await shot(page,u,'gpu-compare','light');await page.locator('canvas').first().evaluate(n=>n.scrollIntoView({block:'start'}));await shot(page,u,'gpu-compare-end','light');await collectHealth(ctx);await ctx.close()};
(async()=>{const browser=await chromium.launch(),failures=[];for(const key of ['DARK1','LINK6'])try{await cells[key](browser);console.log(JSON.stringify({completed:key,shots:records.length}))}catch(e){failures.push({unit:key,error:String(e.stack||e)});console.log(String(e.stack||e))}await browser.close();const prev=JSON.parse(fs.readFileSync(path.join(OUT,'capture-log.json')));prev.runs.push({at:new Date().toISOString(),chromium:browser.version(),playwright:'1.63.0',units:['DARK1','LINK6'],records,health,actions,failures});fs.writeFileSync(path.join(OUT,'capture-log.json'),JSON.stringify(prev,null,2));fs.writeFileSync(path.join(OUT,'matrix-followup-log.json'),JSON.stringify({records,health,actions,failures},null,2));console.log(JSON.stringify({shots:records.length,failures}));})();
