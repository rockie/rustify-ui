// UI review R3 second follow-up: every axe colour-contrast node in the catalog
// button fixture (with text), and real screenshots of the elements measured past
// their column edge at 720x900 + root 200% (Colors tab pills and source block,
// preview "Create · 0" / "Disabled").
//
//   node docs/plan/DOM-COMPONENTS-P1.ui-captures/r3/machine-check-followup2.cjs
const { chromium } = require("@playwright/test");
const AxeBuilder = require("@axe-core/playwright").default;
const fs = require("node:fs");
const path = require("node:path");
const STUDIO = "http://127.0.0.1:4181/";
const CATALOG = "http://127.0.0.1:4176/";
const out = { at: new Date().toISOString(), axeButtons: [], shots: [] };
async function open(browser, url, w, h) {
    const ctx = await browser.newContext({ viewport: { width: w, height: h }, deviceScaleFactor: 1, reducedMotion: "reduce", locale: "en-US" });
    const page = await ctx.newPage();
    await page.goto(url);
    await page.locator("[data-testid=status][data-status=ready], #status[data-status=ready]").first().waitFor({ state: "attached", timeout: 90000 });
    await page.evaluate(() => document.fonts.ready); await page.waitForTimeout(500);
    return { ctx, page };
}
(async () => {
    const browser = await chromium.launch();
    {
        const s = await open(browser, CATALOG, 1280, 900);
        await s.page.evaluate(() => window.__component_catalog.mount_button_fixtures());
        await s.page.getByTestId("button-default").waitFor();
        const r = await new AxeBuilder({ page: s.page }).withTags(["wcag2aa"]).include("[data-testid=catalog-component-fixture]").analyze();
        for (const v of r.violations) for (const n of v.nodes) {
            const text = await s.page.locator(n.target.join(" ")).first().evaluate(el => {
                const b = el.closest("button,[data-testid]"); return { text: el.textContent.trim().slice(0, 30), testid: b?.dataset?.testid ?? null, variant: b?.dataset?.variant ?? null, color: b?.dataset?.color ?? null };
            }).catch(() => null);
            out.axeButtons.push({ id: v.id, target: n.target.join(" "), ...text, summary: (n.failureSummary || "").split("\n")[1]?.slice(0, 120) });
        }
        await s.ctx.close();
    }
    {
        const s = await open(browser, STUDIO, 1440, 900);
        await s.page.setViewportSize({ width: 720, height: 900 });
        await s.page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
        await s.page.waitForTimeout(500);
        await s.page.getByTestId("editor-colors").click(); await s.page.getByTestId("color-search").fill("");
        await s.page.getByTestId("color-token-primary").click(); await s.page.waitForTimeout(400); await s.page.mouse.move(0, 0);
        for (const [label, sel] of [["pill", ".studio-editor-content, #studio-editor"], ["effective", "#studio-editor"]]) {
            const target = label === "pill" ? s.page.getByText("Overridden · Light").first() : s.page.locator("dt", { hasText: "Effective value" }).first();
            await target.evaluate(n => n.scrollIntoView({ block: "center" }));
            await s.page.waitForTimeout(300);
            const file = `MC-root200-${label}-720-light.png`;
            await s.page.screenshot({ path: path.join(__dirname, file) });
            out.shots.push({ file, url: s.page.url(), viewport: "720x900 root 200%", time: new Date().toISOString(),
                box: await target.evaluate(n => { const r = n.getBoundingClientRect(), e = document.querySelector("#studio-editor").getBoundingClientRect(); return { left: r.left, right: r.right, top: r.top, editorRight: e.left + document.querySelector("#studio-editor").clientWidth }; }) });
        }
        await s.ctx.close();
    }
    {
        const s = await open(browser, STUDIO, 1440, 900);
        await s.page.setViewportSize({ width: 720, height: 900 });
        await s.page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
        await s.page.getByTestId("renderer-dom").click(); await s.page.waitForTimeout(500); await s.page.mouse.move(0, 0);
        for (const text of ["Create · 0", "Disabled"]) {
            const t = s.page.locator(".studio-previews").getByText(text, { exact: true }).first();
            await t.evaluate(n => n.scrollIntoView({ block: "center" })); await s.page.waitForTimeout(300);
            const file = `MC-root200-preview-${text.startsWith("Create") ? "create" : "disabled"}-720-light.png`;
            await s.page.screenshot({ path: path.join(__dirname, file) });
            out.shots.push({ file, url: s.page.url(), viewport: "720x900 root 200%, renderer DOM", time: new Date().toISOString(),
                box: await t.evaluate(n => { const r = n.getBoundingClientRect(); let p = n.parentElement, clip = null; while (p) { const cs = getComputedStyle(p); if (/(hidden|clip|auto|scroll)/.test(cs.overflowX)) { const pr = p.getBoundingClientRect(); clip = { el: p.tagName.toLowerCase() + (p.className && typeof p.className === "string" ? "." + p.className.split(" ")[0] : ""), right: pr.left + p.clientWidth }; break; } p = p.parentElement; } return { left: r.left, right: r.right, top: r.top, nearestClip: clip }; }) });
        }
        await s.ctx.close();
    }
    await browser.close();
    fs.writeFileSync(path.join(__dirname, "machine-check-followup2.json"), JSON.stringify(out, null, 2));
    console.log("written");
})();
