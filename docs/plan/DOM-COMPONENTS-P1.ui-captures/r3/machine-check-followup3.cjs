// UI review R3 third follow-up: the fine-adjustment Colors grid and its source
// block at 720x900 + root 200% (main-agent check). Clean frames with the editor
// column scrolled vertically only; measurements of clipped/overlapping parts.
//
//   node docs/plan/DOM-COMPONENTS-P1.ui-captures/r3/machine-check-followup3.cjs
const { chromium } = require("@playwright/test");
const fs = require("node:fs");
const path = require("node:path");
const STUDIO = "http://127.0.0.1:4181/";
const out = { at: new Date().toISOString(), steps: [], frames: [] };
(async () => {
    const browser = await chromium.launch();
    const ctx = await browser.newContext({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1, reducedMotion: "reduce", locale: "en-US" });
    const page = await ctx.newPage();
    await page.goto(STUDIO);
    await page.locator("[data-testid=status][data-status=ready]").first().waitFor({ state: "attached", timeout: 90000 });
    await page.evaluate(() => document.fonts.ready);
    await page.setViewportSize({ width: 720, height: 900 });
    await page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
    out.steps.push("open 1440x900, then 720x900 and root font-size 200%");
    await page.getByTestId("editor-colors").click(); await page.getByTestId("color-search").fill("");
    out.steps.push("click Colors tab, clear search");
    await page.waitForTimeout(500); await page.mouse.move(0, 0);
    const editor = page.locator("#studio-editor");
    const measure = () => page.evaluate(() => {
        const ed = document.querySelector("#studio-editor"), er = ed.getBoundingClientRect(), edRight = er.left + ed.clientWidth;
        const cells = [...document.querySelectorAll('[data-testid^="color-token-"]')].slice(0, 4).map(cell => {
            const r = cell.getBoundingClientRect();
            const kids = [...cell.querySelectorAll("*")].filter(n => n.children.length === 0 && n.textContent.trim());
            return { testid: cell.dataset.testid, cell: [Math.round(r.left), Math.round(r.right), Math.round(r.top), Math.round(r.bottom)],
                parts: kids.map(n => { const b = n.getBoundingClientRect(), cs = getComputedStyle(n); return { text: n.textContent.trim().slice(0, 24), left: Math.round(b.left), right: Math.round(b.right), lines: Math.round(b.height / parseFloat(cs.lineHeight || 1)) || null, scrollW: n.scrollWidth, clientW: n.clientWidth }; }) };
        });
        return { editor: { left: Math.round(er.left), visibleRight: Math.round(edRight), scrollWidth: ed.scrollWidth, clientWidth: ed.clientWidth, scrollLeft: ed.scrollLeft, overflowX: getComputedStyle(ed).overflowX }, cells };
    });
    await page.getByTestId("color-token-background").first().evaluate(n => { const ed = document.querySelector("#studio-editor"); ed.scrollTop += n.getBoundingClientRect().top - 380; });
    out.steps.push("scroll the editor column vertically so the first token cells sit below the header (scrollLeft untouched)");
    await page.waitForTimeout(300);
    out.grid = await measure();
    await page.screenshot({ path: path.join(__dirname, "MC-root200-colors-grid-720-light.png") });
    out.frames.push({ file: "MC-root200-colors-grid-720-light.png", time: new Date().toISOString() });
    await page.getByTestId("color-token-primary").first().click();
    out.steps.push("click token primary");
    await page.waitForTimeout(400); await page.mouse.move(0, 0);
    out.source = await page.evaluate(() => {
        const ed = document.querySelector("#studio-editor"), er = ed.getBoundingClientRect(), edRight = er.left + ed.clientWidth;
        const dts = [...ed.querySelectorAll("dt")].slice(0, 6);
        return { editorVisibleRight: Math.round(edRight), scrollLeft: ed.scrollLeft, pairs: dts.map(dt => { const dd = dt.nextElementSibling; const a = dt.getBoundingClientRect(), b = dd?.getBoundingClientRect();
            return { dt: dt.textContent.trim(), dtBox: [Math.round(a.left), Math.round(a.right), Math.round(a.top)], dd: dd?.textContent.trim().slice(0, 30), ddBox: b && [Math.round(b.left), Math.round(b.right), Math.round(b.top)] }; }) };
    });
    const dt = page.locator("#studio-editor dt", { hasText: "Effective value" }).first();
    await dt.evaluate(n => { const ed = document.querySelector("#studio-editor"); ed.scrollTop += n.getBoundingClientRect().top - 450; });
    out.steps.push("scroll the editor column vertically to the source block (scrollLeft untouched)");
    await page.waitForTimeout(300);
    await page.screenshot({ path: path.join(__dirname, "MC-root200-source-720-light.png") });
    out.frames.push({ file: "MC-root200-source-720-light.png", time: new Date().toISOString() });
    out.scrollLeftAtEnd = await editor.evaluate(n => n.scrollLeft);
    await browser.close();
    fs.writeFileSync(path.join(__dirname, "machine-check-followup3.json"), JSON.stringify(out, null, 2));
    console.log(JSON.stringify(out, null, 1).slice(0, 6000));
})();
