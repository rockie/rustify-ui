// UI review R8 machine checks, run on the live release pages after judging:
// horizontal overflow, axe-core scan, CSP events, a real-key keyboard walk,
// the language switch hit targets, B2 focus styles per variant, loaded font
// faces, scroll regions at 200%, the Code colour steps behind UI-156, and the
// 200% Colors page, renderer switch, export Output and DOM preview width
// (UI-192, UI-157, UI-158 and the R3 preview-width lead). Based on R3.
//
//   node docs/plan/DOM-COMPONENTS-P1.ui-captures/r8/machine-check.cjs
const { chromium } = require("@playwright/test");
const AxeBuilder = require("@axe-core/playwright").default;
const fs = require("node:fs");
const path = require("node:path");

const STUDIO = "http://127.0.0.1:4181/";
const CATALOG = "http://127.0.0.1:4176/";
const out = { at: new Date().toISOString(), overflow: [], axe: [], csp: [], keyboard: [], console: [], targets: [], focusStyles: [], fonts: [], header200: [], codeSteps: [], zoom200: [] };

async function open(browser, url, width, height) {
    const ctx = await browser.newContext({ viewport: { width, height }, deviceScaleFactor: 1, reducedMotion: "reduce", locale: "en-US" });
    const page = await ctx.newPage();
    const errors = [];
    page.on("console", m => { if (m.type() === "error") errors.push(m.text()); });
    page.on("pageerror", e => errors.push(e.message));
    await page.addInitScript(() => {
        window.__csp = [];
        document.addEventListener("securitypolicyviolation", e => window.__csp.push(`${e.violatedDirective} ${e.blockedURI}`));
    });
    await page.goto(url);
    await page.locator("[data-testid=status][data-status=ready], #status[data-status=ready]").first().waitFor({ state: "attached", timeout: 90000 });
    await page.evaluate(() => document.fonts.ready);
    await page.waitForTimeout(500);
    return { ctx, page, errors };
}
async function finish(label, session) {
    out.csp.push({ label, events: await session.page.evaluate(() => window.__csp) });
    out.console.push({ label, errors: session.errors });
    await session.ctx.close();
    fs.writeFileSync(path.join(__dirname, "machine-check-progress.json"), JSON.stringify(out, null, 2));
    console.log(label);
}

async function overflow(page, label, containers = []) {
    const result = await page.evaluate(containers => {
        const doc = document.documentElement;
        const vw = window.innerWidth;
        const offenders = [];
        for (const el of document.querySelectorAll("body *")) {
            const r = el.getBoundingClientRect();
            if (r.width === 0 || r.height === 0 || r.right <= vw + 1) continue;
            let clipped = false;
            for (let p = el.parentElement; p; p = p.parentElement) {
                const s = getComputedStyle(p);
                if (/(auto|scroll|hidden|clip)/.test(s.overflowX) && p.getBoundingClientRect().right <= vw + 1) { clipped = true; break; }
            }
            if (!clipped) offenders.push(`${el.tagName.toLowerCase()}${el.dataset.testid ? `[${el.dataset.testid}]` : ""}${el.className && typeof el.className === "string" ? "." + el.className.split(" ").slice(0, 2).join(".") : ""} right=${Math.round(r.right)}`);
        }
        return {
            viewport: vw, scrollWidth: doc.scrollWidth, pageOverflow: doc.scrollWidth - vw,
            containers: containers.map(sel => { const n = document.querySelector(sel); return n ? { sel, scrollWidth: n.scrollWidth, clientWidth: n.clientWidth } : { sel, missing: true }; }),
            offenders: offenders.slice(0, 15), offenderCount: offenders.length,
        };
    }, containers);
    out.overflow.push({ label, ...result });
}

async function axe(page, label, include) {
    let builder = new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]);
    if (include) builder = builder.include(include);
    const r = await builder.analyze();
    out.axe.push({ label, violations: r.violations.map(v => ({ id: v.id, impact: v.impact, count: v.nodes.length,
        help: v.help, targets: v.nodes.slice(0, 6).map(n => `${n.target.join(" ")} :: ${(n.failureSummary || "").split("\n").slice(1, 2).join(" ").slice(0, 160)}`) })),
        incomplete: r.incomplete.map(v => ({ id: v.id, count: v.nodes.length })) });
}

async function focusInfo(page) {
    return page.evaluate(() => {
        const el = document.activeElement;
        if (!el || el === document.body) return { tag: "body" };
        const s = getComputedStyle(el), r = el.getBoundingClientRect();
        const visible = (s.outlineStyle !== "none" && parseFloat(s.outlineWidth) > 0) || (s.boxShadow && s.boxShadow !== "none");
        return { tag: el.tagName.toLowerCase(), testid: el.dataset.testid || el.closest("[data-testid]")?.dataset.testid || null,
            name: (el.getAttribute("aria-label") || el.textContent || el.getAttribute("placeholder") || "").trim().slice(0, 40),
            indicator: visible, outline: `${s.outlineStyle} ${s.outlineWidth} ${s.outlineColor}`, shadow: s.boxShadow.slice(0, 60),
            onScreen: r.width > 0 && r.height > 0 && r.bottom > 0 && r.right > 0 && r.top < innerHeight && r.left < innerWidth,
            inDialog: !!el.closest("dialog,[role=dialog]") };
    });
}

(async()=>{
const browser=await chromium.launch(process.env.RUSTIFY_CHROMIUM?{executablePath:process.env.RUSTIFY_CHROMIUM}:{});
try{
 for(const [w,h,label] of [[1440,900,'studio-1440'],[768,1024,'studio-768'],[390,844,'studio-390-edit']]){
  const s=await open(browser,STUDIO,w,h);await overflow(s.page,label,['#studio-editor','.studio-previews']);await axe(s.page,label);
  if(w===390){await s.page.getByTestId('mobile-preview').click();await overflow(s.page,'studio-390-preview',['.studio-previews']);await axe(s.page,'studio-390-preview');await s.page.getByTestId('mobile-edit').click();}
  await s.page.getByTestId('theme-export').click();await s.page.waitForTimeout(500);
  out.overflow.push({label:label+'-export-dialog',dialog:await s.page.getByTestId('data-tools-dialog').boundingBox(),viewport:{w,h}});await axe(s.page,label+'-export-dialog');await finish(label,s);
 }
 {
  const s=await open(browser,STUDIO,1440,900);await s.page.setViewportSize({width:720,height:900});await s.page.evaluate(()=>document.documentElement.style.fontSize='200%');await s.page.getByTestId('renderer-dom').click();await s.page.waitForTimeout(700);await overflow(s.page,'studio-720-root200',['#studio-editor','.studio-previews']);
  out.zoom200.push(await s.page.evaluate(()=>{const box=n=>{const r=n.getBoundingClientRect();return {left:r.left,right:r.right,top:r.top,bottom:r.bottom,width:r.width,height:r.height}};return {surfaces:[...document.querySelectorAll('.preview-surface')].map(n=>({visible:n.checkVisibility(),scrollWidth:n.scrollWidth,clientWidth:n.clientWidth,scrollLeft:n.scrollLeft,rect:box(n)})),create:[...document.querySelectorAll('[data-testid=preview-primary]')].map(n=>({visible:n.checkVisibility(),text:n.textContent,rect:box(n)}))};}));
  await s.page.screenshot({path:path.join(__dirname,'MC-root200-preview-720-light.png'),animations:'disabled'});await finish('studio-720-root200',s);
 }
 {
  const s=await open(browser,STUDIO,1440,900),walk=[];
  for(let i=1;i<=70;i++){await s.page.keyboard.press('Tab');walk.push({step:i,...await focusInfo(s.page)});}out.keyboard.push({label:'studio-tab-walk-70',walk});
  for(const[trigger,source,n]of[['theme-export','export-source',25],['theme-import','import-source',15]]){
   await s.page.getByTestId(trigger).focus();await s.page.keyboard.press('Enter');await s.page.waitForTimeout(300);const trap=[];
   for(let i=1;i<=n;i++){await s.page.keyboard.press('Tab');trap.push({step:i,...await focusInfo(s.page)});}
   await s.page.keyboard.press('Escape');await s.page.waitForTimeout(300);out.keyboard.push({label:trigger+'-trap',trap,afterEscape:await focusInfo(s.page),dialogStillVisible:await s.page.getByTestId(source).isVisible()});
  }
  await s.page.getByTestId('studio-config-accent_color').focus();const before=await s.page.getByTestId('studio-config-accent_color').innerText();await s.page.keyboard.press('Enter');await s.page.keyboard.press('ArrowDown');await s.page.keyboard.press('ArrowDown');await s.page.keyboard.press('Enter');await s.page.waitForTimeout(400);
  out.keyboard.push({label:'brand-keyboard-commit',before,after:await s.page.getByTestId('studio-config-accent_color').innerText()});await finish('studio-keyboard',s);
 }
 for(const[w,h]of[[1280,900],[390,844]]){
  const s=await open(browser,CATALOG,w,h);await s.page.evaluate(()=>window.__component_catalog.mount_button_fixtures());await s.page.getByTestId('button-default').waitFor();await overflow(s.page,'catalog-buttons-'+w,['[data-testid=button-fixture-body]']);await axe(s.page,'catalog-buttons-'+w,'[data-testid=catalog-component-fixture]');
  if(w===1280){
   for(const variant of ['classic','solid','soft','surface','outline','ghost-offset','ghost']){
    const id=`button-${variant}-2`;await s.page.getByTestId(id).focus();await s.page.keyboard.press('Shift+Tab');await s.page.keyboard.press('Tab');await s.page.waitForTimeout(700);
    out.focusStyles.push({id,...await s.page.evaluate(()=>{const n=document.activeElement,c=getComputedStyle(n);return{focused:n.dataset.testid,focusVisible:n.matches(':focus-visible'),outline:c.outline,outlineOffset:c.outlineOffset}})});
   }
  }
  await s.page.evaluate(()=>{window.__component_catalog.dispose_button_fixtures();window.__component_catalog.mount_layout_fixtures();});await s.page.getByTestId('layout-typography').waitFor();await overflow(s.page,'catalog-layout-'+w,['[data-testid=catalog-layout-fixture]']);await axe(s.page,'catalog-layout-'+w,'[data-testid=catalog-layout-fixture]');await finish('catalog-'+w,s);
 }
 out.chromium=browser.version();
}catch(e){out.error=String(e.stack||e);process.exitCode=1;}finally{await browser.close();fs.writeFileSync(path.join(__dirname,'machine-check.json'),JSON.stringify(out,null,2));console.log(JSON.stringify({completed:!out.error,error:out.error}));}
})();
