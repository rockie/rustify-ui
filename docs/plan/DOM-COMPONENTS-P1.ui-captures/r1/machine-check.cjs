// UI review R1 machine checks, run on the live release pages after judging:
// horizontal overflow, axe-core scan, CSP events and a real-key keyboard walk.
//
//   node docs/plan/DOM-COMPONENTS-P1.ui-captures/r1/machine-check.cjs
const { chromium } = require("@playwright/test");
const AxeBuilder = require("@axe-core/playwright").default;
const fs = require("node:fs");
const path = require("node:path");

const STUDIO = "http://127.0.0.1:4181/";
const CATALOG = "http://127.0.0.1:4176/";
const out = { at: new Date().toISOString(), overflow: [], axe: [], csp: [], keyboard: [], console: [] };

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

(async () => {
    const browser = await chromium.launch();
    // Overflow and axe across Studio viewports.
    for (const [w, h, label] of [[1440, 900, "studio-1440"], [768, 1024, "studio-768"], [390, 844, "studio-390-edit"]]) {
        const s = await open(browser, STUDIO, w, h);
        await overflow(s.page, label, ["#studio-editor", ".studio-previews"]);
        await axe(s.page, label);
        if (w === 390) {
            await s.page.getByTestId("mobile-preview").click(); await s.page.waitForTimeout(300);
            await overflow(s.page, "studio-390-preview", [".studio-previews"]);
            await axe(s.page, "studio-390-preview");
            await s.page.getByTestId("mobile-edit").click(); await s.page.waitForTimeout(300);
        }
        await s.page.getByTestId("theme-export").click(); await s.page.waitForTimeout(300);
        const dialog = await s.page.getByTestId("data-tools-dialog").first().boundingBox().catch(() => null);
        out.overflow.push({ label: `${label}-export-dialog`, dialog, viewport: { w, h } });
        await axe(s.page, `${label}-export-dialog`);
        await finish(label, s);
    }
    {
        const s = await open(browser, STUDIO, 1440, 900);
        await s.page.setViewportSize({ width: 720, height: 900 });
        await s.page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
        await s.page.waitForTimeout(500);
        await overflow(s.page, "studio-720-root200", ["#studio-editor", ".studio-previews"]);
        await finish("studio-720-root200", s);
    }
    // Keyboard walk on the Studio main path.
    {
        const s = await open(browser, STUDIO, 1440, 900);
        const walk = [];
        for (let i = 1; i <= 70; i++) { await s.page.keyboard.press("Tab"); walk.push({ step: i, ...(await focusInfo(s.page)) }); }
        out.keyboard.push({ label: "studio-tab-walk-70", walk });
        await s.page.getByTestId("theme-export").focus(); await s.page.keyboard.press("Enter"); await s.page.waitForTimeout(300);
        const trap = [];
        for (let i = 1; i <= 25; i++) { await s.page.keyboard.press("Tab"); trap.push({ step: i, ...(await focusInfo(s.page)) }); }
        await s.page.keyboard.press("Escape"); await s.page.waitForTimeout(300);
        const after = await focusInfo(s.page);
        const stillOpen = await s.page.getByTestId("export-source").isVisible().catch(() => false);
        out.keyboard.push({ label: "studio-export-dialog-trap", trap, afterEscape: after, dialogStillVisible: stillOpen });
        await s.page.getByTestId("theme-import").focus(); await s.page.keyboard.press("Enter"); await s.page.waitForTimeout(300);
        const trap2 = [];
        for (let i = 1; i <= 15; i++) { await s.page.keyboard.press("Tab"); trap2.push({ step: i, ...(await focusInfo(s.page)) }); }
        await s.page.keyboard.press("Escape"); await s.page.waitForTimeout(300);
        out.keyboard.push({ label: "studio-import-dialog-trap", trap: trap2, afterEscape: await focusInfo(s.page) });
        // Brand select: open with Enter, move with ArrowDown, commit with Enter.
        await s.page.getByTestId("studio-config-accent_color").focus();
        const before = await s.page.getByTestId("studio-config-accent_color").innerText();
        await s.page.keyboard.press("Enter"); await s.page.keyboard.press("ArrowDown"); await s.page.keyboard.press("ArrowDown"); await s.page.keyboard.press("Enter");
        await s.page.waitForTimeout(300);
        out.keyboard.push({ label: "studio-brand-select-keyboard", before, after: await s.page.getByTestId("studio-config-accent_color").innerText(), focus: await focusInfo(s.page) });
        await finish("studio-keyboard", s);
    }
    // Catalog fixtures.
    for (const [w, h] of [[1280, 900], [390, 844]]) {
        const s = await open(browser, CATALOG, w, h);
        await s.page.evaluate(() => window.__component_catalog.mount_button_fixtures());
        await s.page.getByTestId("button-default").waitFor();
        await overflow(s.page, `catalog-buttons-${w}`, ["[data-testid=button-fixture-body]"]);
        await axe(s.page, `catalog-buttons-${w}`, "[data-testid=catalog-component-fixture]");
        if (w === 1280) {
            await s.page.getByTestId("button-classic-1").focus();
            const walk = [await focusInfo(s.page)];
            for (let i = 1; i <= 60; i++) { await s.page.keyboard.press("Tab"); walk.push({ step: i, ...(await focusInfo(s.page)) }); }
            out.keyboard.push({ label: "catalog-buttons-tab-walk", walk });
        }
        await s.page.evaluate(() => { window.__component_catalog.dispose_button_fixtures(); window.__component_catalog.mount_layout_fixtures(); });
        await s.page.getByTestId("layout-typography").waitFor();
        await overflow(s.page, `catalog-layout-${w}`, ["[data-testid=catalog-layout-fixture]"]);
        await axe(s.page, `catalog-layout-${w}`, "[data-testid=catalog-layout-fixture]");
        await finish(`catalog-${w}`, s);
    }
    await browser.close();
    out.chromium = "153.0.8010.12";
    fs.writeFileSync(path.join(__dirname, "machine-check.json"), JSON.stringify(out, null, 2));
    console.log("written");
})();
