// UI review R4 follow-up on the live Studio page: the preview dialog opened
// inside the local boundary (U5 judge's P1 claim) — computed colours and size
// of its Confirm button against the boundary's own primary, for "Opposite to
// root" and "Inherit root", plus the theme scope the dialog is rendered in.
//
//   node docs/plan/DOM-COMPONENTS-P1.ui-captures/r4/machine-check-followup.cjs
const { chromium } = require("@playwright/test");
const fs = require("node:fs");
const path = require("node:path");
const STUDIO = "http://127.0.0.1:4181/";

(async () => {
    const browser = await chromium.launch();
    const ctx = await browser.newContext({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1, reducedMotion: "reduce", locale: "en-US" });
    const page = await ctx.newPage();
    const errors = [];
    page.on("console", m => { if (m.type() === "error") errors.push(m.text()); });
    page.on("pageerror", e => errors.push(e.message));
    await page.goto(STUDIO);
    await page.locator("[data-testid=status][data-status=ready], #status[data-status=ready]").first().waitFor({ state: "attached", timeout: 90000 });
    await page.evaluate(() => document.fonts.ready);
    await page.getByTestId("renderer-dom").click();
    const preview = page.getByTestId("studio-preview");
    const out = { at: new Date().toISOString(), cases: [], errors };
    for (const choice of ["Opposite to root", "Inherit root"]) {
        await preview.getByTestId("local-appearance").click();
        await preview.getByTestId("local-appearance-list").getByRole("option", { name: choice, exact: true }).click();
        await page.waitForTimeout(300);
        await preview.getByTestId("open-preview-dialog").click();
        await page.waitForTimeout(400);
        out.cases.push({ choice, ...(await page.evaluate(() => {
            const scopeOf = n => { const s = n.closest("[data-theme], [data-appearance], .dark, .light, [class*=rustify-theme], [class*=theme]"); return s ? { tag: s.tagName.toLowerCase(), testid: s.dataset.testid || null, cls: (typeof s.className === "string" ? s.className : "").slice(0, 120), dataTheme: s.getAttribute("data-theme"), dataAppearance: s.getAttribute("data-appearance") } : null; };
            const dialog = [...document.querySelectorAll("[role=dialog], dialog")].find(d => d.getBoundingClientRect().width > 0 && /follows the theme/.test(d.textContent));
            const confirm = dialog && [...dialog.querySelectorAll("button")].find(b => b.textContent.trim() === "Confirm");
            const boundary = document.querySelector('[data-testid="local-card"]') || document.querySelector('[data-testid="studio-preview"] [data-testid*="local"]');
            const action = boundary && [...boundary.querySelectorAll("button")].find(b => b.textContent.trim() === "Actions");
            const cs = n => n && (s => ({ background: s.backgroundColor, color: s.color, primary: s.getPropertyValue("--primary").trim(), primaryForeground: s.getPropertyValue("--primary-foreground").trim(), popover: s.getPropertyValue("--popover").trim(), height: Math.round(n.getBoundingClientRect().height * 10) / 10, width: Math.round(n.getBoundingClientRect().width * 10) / 10 }))(getComputedStyle(n));
            return { dialogFound: !!dialog, dialogScope: dialog && scopeOf(dialog), dialogSurface: dialog && getComputedStyle(dialog).backgroundColor,
                dialogParent: dialog && `${dialog.parentElement.tagName.toLowerCase()}${dialog.parentElement.id ? "#" + dialog.parentElement.id : ""}`,
                confirm: cs(confirm), confirmScope: confirm && scopeOf(confirm), boundary: cs(boundary), boundaryScope: boundary && scopeOf(boundary), boundaryActions: cs(action) };
        })) });
        await page.keyboard.press("Escape"); await page.waitForTimeout(300);
    }
    await browser.close();
    fs.writeFileSync(path.join(__dirname, "machine-check-followup.json"), JSON.stringify(out, null, 2));
    console.log(JSON.stringify(out, null, 2));
})();
