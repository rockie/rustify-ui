// UI review R5 follow-up 2 on the live pages: (a) Link focus style read 1s
// after keyboard focus (the 0.15s transition otherwise gives mid values);
// (b) 720x900 + root 200%, renderer DOM: the DOM preview surface at its
// default horizontal scroll, scrolled vertically only (no scrollIntoView, so
// the surface's own scrollLeft stays 0), photographed where "Create · 0" sits.
//
//   node docs/plan/DOM-COMPONENTS-P1.ui-captures/r5/machine-check-followup2.cjs
const { chromium } = require("@playwright/test");
const fs = require("node:fs");
const path = require("node:path");
const STUDIO = "http://127.0.0.1:4181/", CATALOG = "http://127.0.0.1:4176/";

async function open(browser, url, w, h) {
    const ctx = await browser.newContext({ viewport: { width: w, height: h }, deviceScaleFactor: 1, reducedMotion: "reduce", locale: "en-US" });
    const page = await ctx.newPage();
    await page.goto(url);
    await page.locator("[data-testid=status][data-status=ready], #status[data-status=ready]").first().waitFor({ state: "attached", timeout: 90000 });
    await page.evaluate(() => document.fonts.ready);
    return { ctx, page };
}
(async () => {
    const browser = await chromium.launch(process.env.RUSTIFY_CHROMIUM ? { executablePath: process.env.RUSTIFY_CHROMIUM } : {});
    const out = { at: new Date().toISOString(), linkFocus: [], preview200: null };
    {
        const { ctx, page } = await open(browser, CATALOG, 1280, 900);
        await page.evaluate(() => window.__component_catalog.mount_layout_fixtures());
        await page.getByTestId("type-link").waitFor(); await page.mouse.move(0, 0);
        for (const id of ["type-link", "type-long-link"]) {
            await page.getByTestId(id).focus(); await page.keyboard.press("Shift+Tab"); await page.keyboard.press("Tab");
            await page.waitForTimeout(1000);
            out.linkFocus.push(await page.evaluate(id => { const el = document.activeElement, cs = getComputedStyle(el);
                return { id, focused: el.dataset.testid, focusVisible: el.matches(":focus-visible"), outline: `${cs.outlineStyle} ${cs.outlineWidth} ${cs.outlineColor}`, offset: cs.outlineOffset, radius: cs.borderTopLeftRadius, underline: cs.textDecorationLine }; }, id));
        }
        await ctx.close();
    }
    {
        const { ctx, page } = await open(browser, STUDIO, 1440, 900);
        await page.setViewportSize({ width: 720, height: 900 });
        await page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
        await page.getByTestId("renderer-dom").click(); await page.waitForTimeout(500);
        out.preview200 = await page.evaluate(() => {
            const pre = document.querySelector(".studio-previews");
            const btn = document.querySelector('[data-testid="dom-preview"] [data-testid="preview-primary"]') || document.querySelector('[data-testid="preview-primary"]');
            const surf = btn.closest(".preview-surface");
            const before = { previewsScrollTop: pre.scrollTop, surfaceScrollLeft: surf.scrollLeft };
            const b = btn.getBoundingClientRect(), p = pre.getBoundingClientRect();
            pre.scrollTop += b.top - p.top - 150;
            const r = btn.getBoundingClientRect(), s = surf.getBoundingClientRect();
            return { before, after: { previewsScrollTop: pre.scrollTop, surfaceScrollLeft: surf.scrollLeft },
                surface: { left: Math.round(s.left), right: Math.round(s.right), scrollWidth: surf.scrollWidth, clientWidth: surf.clientWidth, overflowX: getComputedStyle(surf).overflowX },
                button: { text: btn.textContent.trim(), left: Math.round(r.left), right: Math.round(r.right), top: Math.round(r.top), bottom: Math.round(r.bottom) },
                visibleWidth: Math.max(0, Math.min(r.right, s.right) - Math.max(r.left, s.left)) };
        });
        await page.mouse.move(0, 0); await page.waitForTimeout(400);
        await page.screenshot({ path: path.join(__dirname, "MC-root200-preview-clip-720-light.png"), animations: "disabled" });
        out.preview200.screenshot = "MC-root200-preview-clip-720-light.png";
        await ctx.close();
    }
    await browser.close();
    fs.writeFileSync(path.join(__dirname, "machine-check-followup2.json"), JSON.stringify(out, null, 2));
    console.log(JSON.stringify(out, null, 2));
})();
