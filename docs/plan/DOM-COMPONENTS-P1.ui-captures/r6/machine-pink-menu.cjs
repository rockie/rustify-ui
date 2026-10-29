// UI review R6 capture for DOM-COMPONENTS-P1. Drives the real release builds
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
//   node docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/capture-runner.cjs [unit...]
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


(async()=>{const browser=await chromium.launch(),probes=[],failures=[];
for(const[w,h,dark]of[[768,1024,false],[390,844,false],[1440,900,true]]){const ctx=await context(browser,w,h),page=await ctx.newPage();watch(page,'UI-217');try{await ready(page,STUDIO);await graphitePink(page,'UI-217');if(dark)await tid(page,'mode-dark').click();await tid(page,'studio-config-accent_color').focus();await page.keyboard.press('Shift+Tab');await page.keyboard.press('Tab');await page.keyboard.press('Enter');await page.keyboard.press('ArrowDown');await quiet(page);
const data=await tid(page,'studio-config-accent_color-list').evaluate(n=>{const b=n.getBoundingClientRect(),active=document.activeElement,id=active?.getAttribute('aria-activedescendant'),box=e=>{const r=e.getBoundingClientRect();return{x:r.x,y:r.y,width:r.width,height:r.height}};return{list:box(n),scrollTop:n.scrollTop,scrollHeight:n.scrollHeight,clientHeight:n.clientHeight,focus:{tag:active?.tagName,testid:active?.getAttribute('data-testid'),activeDescendant:id,outline:getComputedStyle(active).outline},options:[...n.querySelectorAll('[role=option]')].map(e=>({id:e.id,text:e.textContent,attrs:Object.fromEntries([...e.attributes].filter(a=>a.name.startsWith('aria-')||a.name.startsWith('data-')).map(a=>[a.name,a.value])),box:box(e),background:getComputedStyle(e).backgroundColor,insideList:e.getBoundingClientRect().top>=b.top&&e.getBoundingClientRect().bottom<=b.bottom})),activeDescendantBox:id&&document.getElementById(id)?box(document.getElementById(id)):null};});await shot(page,'MC','pink-menu',dark?'dark':'light');await page.keyboard.press('Enter');await quiet(page);probes.push({viewport:w+'x'+h,theme:dark?'dark':'light',before:'Pink',...data,afterEnter:await tid(page,'studio-config-accent_color').innerText()});}catch(e){failures.push({viewport:w,error:String(e.stack||e)});}await collectHealth(ctx);await ctx.close();}
await browser.close();fs.writeFileSync(path.join(OUT,'machine-pink-menu.json'),JSON.stringify({tool:'Playwright 1.63.0',chromium:browser.version(),steps:'Graphite -> Pink -> Auto; focus by Shift+Tab/Tab; Enter; ArrowDown; screenshot; Enter confirmation',probes,records,health,actions,failures},null,2));console.log(JSON.stringify({shots:records.length,failures,summary:probes.map(p=>({viewport:p.viewport,theme:p.theme,focus:p.focus,scrollTop:p.scrollTop,active:p.options.filter(o=>o.attrs['data-highlighted']||o.attrs['data-active']||o.id===p.focus.activeDescendant||o.background!=='rgba(0, 0, 0, 0)'),afterEnter:p.afterEnter}))}));})();
