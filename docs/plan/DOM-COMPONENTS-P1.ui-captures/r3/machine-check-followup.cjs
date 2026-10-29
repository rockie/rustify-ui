// UI review R3 follow-up checks on the live pages: what overflows the editor and
// preview columns at 720x900 + root 200%, Link focus after its transition, and
// the colour steps behind the axe Code findings (UI-156).
//
//   node docs/plan/DOM-COMPONENTS-P1.ui-captures/r3/machine-check-followup.cjs
const { chromium } = require("@playwright/test");
const fs = require("node:fs");
const path = require("node:path");
const STUDIO = "http://127.0.0.1:4181/";
const CATALOG = "http://127.0.0.1:4176/";
const out = { at: new Date().toISOString() };
async function open(browser, url, w, h) {
    const ctx = await browser.newContext({ viewport: { width: w, height: h }, deviceScaleFactor: 1, reducedMotion: "reduce", locale: "en-US" });
    const page = await ctx.newPage();
    await page.goto(url);
    await page.locator("[data-testid=status][data-status=ready], #status[data-status=ready]").first().waitFor({ state: "attached", timeout: 90000 });
    await page.evaluate(() => document.fonts.ready); await page.waitForTimeout(500);
    return { ctx, page };
}
const clipped = page => page.evaluate(() => {
    const res = {};
    for (const sel of ["#studio-editor", ".studio-previews"]) {
        const box = document.querySelector(sel); const br = box.getBoundingClientRect();
        const right = br.left + box.clientWidth;
        res[sel] = { clientWidth: box.clientWidth, scrollWidth: box.scrollWidth, overflowX: getComputedStyle(box).overflowX,
            items: [...box.querySelectorAll("*")].filter(n => { const r = n.getBoundingClientRect(); return r.width > 0 && r.height > 0 && r.right > right + 1; })
                .filter(n => ![...n.children].some(c => c.getBoundingClientRect().right > right + 1))
                .slice(0, 12).map(n => { const r = n.getBoundingClientRect(); return { el: `${n.tagName.toLowerCase()}${n.dataset.testid ? `[${n.dataset.testid}]` : ""}`,
                    text: (n.textContent || "").trim().slice(0, 40), left: Math.round(r.left), right: Math.round(r.right), top: Math.round(r.top), clipRight: Math.round(right) }; }) };
    }
    return res;
});
(async () => {
    const browser = await chromium.launch();
    {
        const s = await open(browser, STUDIO, 1440, 900);
        await s.page.setViewportSize({ width: 720, height: 900 });
        await s.page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
        await s.page.waitForTimeout(500);
        out.root200 = await clipped(s.page);
        await s.page.evaluate(() => document.querySelector("#studio-editor").scrollTo(0, 1e6));
        out.root200EditorEnd = await clipped(s.page);
        await s.ctx.close();
    }
    {
        const s = await open(browser, CATALOG, 1280, 900);
        await s.page.evaluate(() => window.__component_catalog.mount_layout_fixtures());
        await s.page.getByTestId("type-link").waitFor(); await s.page.mouse.move(0, 0);
        out.linkFocus = [];
        for (const id of ["type-link", "type-long-link"]) {
            await s.page.getByTestId(id).focus(); await s.page.keyboard.press("Shift+Tab"); await s.page.keyboard.press("Tab");
            await s.page.waitForTimeout(1000);
            out.linkFocus.push(await s.page.evaluate(() => { const el = document.activeElement, cs = getComputedStyle(el);
                return { id: el.dataset.testid, focusVisible: el.matches(":focus-visible"), outline: `${cs.outlineStyle} ${cs.outlineWidth} ${cs.outlineColor}`, offset: cs.outlineOffset,
                    radius: cs.borderTopLeftRadius, decoration: cs.textDecorationLine, transition: cs.transitionProperty + " " + cs.transitionDuration }; }));
        }
        await s.ctx.close();
    }
    const codeSteps = page => page.evaluate(() => {
        const probe = (host, value) => { const n = document.createElement("span"); n.style.color = value; host.appendChild(n); const v = getComputedStyle(n).color; n.remove(); return v; };
        return [...document.querySelectorAll(".rustify-code")].map(el => {
            const cs = getComputedStyle(el), host = el.parentElement;
            const step = name => { const raw = cs.getPropertyValue(name).trim(); return raw ? probe(host, raw) : null; };
            return { text: el.textContent.trim().slice(0, 24), variant: el.dataset.variant || null, color: el.dataset.color || null, hc: el.hasAttribute("data-high-contrast"),
                fg: cs.color, bg: cs.backgroundColor, a11: step("--rustify-component-color-a11"), a3: step("--rustify-component-color-a3"),
                a9: step("--rustify-component-color-a9"), contrast: step("--rustify-component-color-contrast"), primary: step("--primary"), primaryFg: step("--primary-foreground") };
        });
    });
    {
        const s = await open(browser, STUDIO, 1440, 900);
        await s.page.getByTestId("renderer-dom").click(); await s.page.waitForTimeout(300);
        out.studioCode = await codeSteps(s.page);
        await s.ctx.close();
    }
    {
        const s = await open(browser, CATALOG, 1280, 900);
        await s.page.evaluate(() => window.__component_catalog.mount_layout_fixtures());
        await s.page.getByTestId("type-code-matrix").waitFor();
        out.catalogCode = await codeSteps(s.page);
        await s.ctx.close();
    }
    await browser.close();
    fs.writeFileSync(path.join(__dirname, "machine-check-followup.json"), JSON.stringify(out, null, 2));
    console.log("written");
})();
