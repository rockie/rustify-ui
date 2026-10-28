// UI review R4 machine checks, run on the live release pages after judging:
// horizontal overflow, axe-core scan, CSP events, a real-key keyboard walk,
// the language switch hit targets, B2 focus styles per variant, loaded font
// faces, scroll regions at 200%, the Code colour steps behind UI-156, and the
// 200% Colors page, renderer switch, export Output and DOM preview width
// (UI-192, UI-157, UI-158 and the R3 preview-width lead). Based on R3.
//
//   node docs/plan/DOM-COMPONENTS-P1.ui-captures/r4/machine-check.cjs
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
    // 200% follow-ups: Colors page with primary selected (UI-192), renderer
    // switch (UI-157), export Output (UI-158), DOM preview width (R3 lead).
    {
        const s = await open(browser, STUDIO, 1440, 900);
        await s.page.setViewportSize({ width: 720, height: 900 });
        await s.page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
        await s.page.waitForTimeout(500);
        const rect = n => { if (!n) return null; const r = n.getBoundingClientRect(); return { left: Math.round(r.left), right: Math.round(r.right), top: Math.round(r.top), bottom: Math.round(r.bottom), width: Math.round(r.width) }; };
        const renderer = await s.page.evaluate(rect => {
            const r = eval(rect);
            const ids = ["renderer-dom", "renderer-gpu", "renderer-compare"];
            const items = ids.map(id => ({ id, ...r(document.querySelector(`[data-testid="${id}"]`)) }));
            const pre = document.querySelector(".studio-previews");
            return { viewport: innerWidth, items, previews: { scrollWidth: pre.scrollWidth, clientWidth: pre.clientWidth, rect: r(pre) } };
        }, rect.toString());
        await s.page.getByTestId("renderer-dom").click(); await s.page.waitForTimeout(400);
        const preview = await s.page.evaluate(rect => {
            const r = eval(rect);
            const surf = [...document.querySelectorAll("section.preview-surface, .preview-surface")];
            const pre = document.querySelector(".studio-previews");
            return { surfaces: surf.map(n => ({ scrollWidth: n.scrollWidth, clientWidth: n.clientWidth, scrollLeft: n.scrollLeft, overflowX: getComputedStyle(n).overflowX, rect: r(n) })),
                previews: { scrollWidth: pre.scrollWidth, clientWidth: pre.clientWidth, scrollLeft: pre.scrollLeft },
                beyond: [...pre.querySelectorAll("button, a, input, select, [role=button]")].filter(n => { const b = n.getBoundingClientRect(); return b.width > 0 && b.right > pre.getBoundingClientRect().right + 1; })
                    .slice(0, 12).map(n => ({ text: (n.getAttribute("aria-label") || n.textContent || "").trim().slice(0, 30), testid: n.dataset.testid || null, ...r(n) })) };
        }, rect.toString());
        await s.page.getByTestId("editor-colors").click(); await s.page.waitForTimeout(300);
        const grid = await s.page.evaluate(rect => {
            const r = eval(rect);
            const ed = document.querySelector("#studio-editor");
            const cells = [...document.querySelectorAll('[data-testid^="color-token-"]')].slice(0, 6).map(n => ({ id: n.dataset.testid, ...r(n), scrollWidth: n.scrollWidth, clientWidth: n.clientWidth }));
            return { editor: { scrollWidth: ed.scrollWidth, clientWidth: ed.clientWidth, scrollLeft: ed.scrollLeft, rect: r(ed) }, cells };
        }, rect.toString());
        await s.page.getByTestId("color-token-primary").click(); await s.page.waitForTimeout(400);
        const source = await s.page.evaluate(rect => {
            const r = eval(rect);
            const ed = document.querySelector("#studio-editor");
            const eff = document.querySelector('[data-testid="source-effective"]');
            const src = document.querySelector('[data-testid="token-source"]');
            const clear = document.querySelector('[data-testid="source-clear"]');
            const wide = [...(src ? src.querySelectorAll("*") : [])].filter(n => n.scrollWidth > n.clientWidth + 1 && getComputedStyle(n).overflowX === "visible").slice(0, 8).map(n => `${n.tagName.toLowerCase()}${n.dataset.testid ? `[${n.dataset.testid}]` : ""} ${n.scrollWidth}/${n.clientWidth}`);
            return { editor: { scrollWidth: ed.scrollWidth, clientWidth: ed.clientWidth, scrollLeft: ed.scrollLeft, rect: r(ed) },
                effective: eff && { text: eff.textContent.trim().slice(0, 40), ...r(eff) }, source: r(src), clear: clear && { text: clear.textContent.trim(), ...r(clear) }, wideChildren: wide };
        }, rect.toString());
        await s.page.getByTestId("theme-export").click(); await s.page.waitForTimeout(400);
        const output = await s.page.getByTestId("export-kind").evaluate(n => { const b = n.getBoundingClientRect(); return { text: n.textContent.trim(), width: Math.round(b.width), scrollWidth: n.scrollWidth, clientWidth: n.clientWidth, right: Math.round(b.right), viewport: innerWidth }; });
        out.zoom200.push({ renderer, preview, grid, source, output });
        await finish("studio-720-root200-followups", s);
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
    // Language switch hit targets (UI-99): box and what is hit at its centre.
    {
        const s = await open(browser, STUDIO, 1440, 900);
        for (const state of ["en", "zh"]) {
            if (state === "zh") { await s.page.getByTestId("locale-zh").click(); await s.page.waitForTimeout(300); }
            out.targets.push({ label: `studio-locale-${state}`, items: await s.page.evaluate(() => ["locale-en", "locale-zh"].map(id => {
                const el = document.querySelector(`[data-testid="${id}"]`); const r = el.getBoundingClientRect();
                const hit = document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2);
                return { id, x: r.left, y: r.top, w: r.width, h: r.height, text: el.textContent.trim(), centreHits: hit?.closest("[data-testid]")?.dataset.testid ?? null };
            })) });
        }
        await finish("studio-locale", s);
    }
    // B2 focus styles: reach each control with the keyboard and read its outline.
    {
        const s = await open(browser, CATALOG, 1280, 900);
        await s.page.evaluate(() => window.__component_catalog.mount_button_fixtures());
        await s.page.getByTestId("button-default").waitFor(); await s.page.mouse.move(0, 0);
        const read = async (id, mode) => {
            await s.page.getByTestId(id).focus(); await s.page.keyboard.press("Shift+Tab"); await s.page.keyboard.press("Tab");
            out.focusStyles.push({ id, mode, ...(await s.page.evaluate(id => {
                const el = document.activeElement, cs = getComputedStyle(el);
                return { focused: el.dataset.testid, focusVisible: el.matches(":focus-visible"), outlineStyle: cs.outlineStyle, outlineWidth: cs.outlineWidth,
                    outlineColor: cs.outlineColor, outlineOffset: cs.outlineOffset, borderRadius: cs.borderTopLeftRadius, boxShadow: cs.boxShadow.slice(0, 80),
                    textDecorationLine: cs.textDecorationLine, opacity: cs.opacity, background: cs.backgroundColor, color: cs.color, expected: id };
            }, id)) });
        };
        const ids = ["classic", "solid", "soft", "surface", "outline", "ghost-offset", "ghost"].map(v => `button-${v}-2`)
            .concat(["button-color-red", "button-color-gray", "button-high-contrast", "button-icon", "button-native-unavailable"]);
        for (const id of ids) await read(id, "light");
        await s.page.getByTestId("button-mode-dark").click(); await s.page.mouse.move(0, 0);
        for (const id of ids.slice(0, 7)) await read(id, "dark");
        await s.page.evaluate(() => { window.__component_catalog.dispose_button_fixtures(); window.__component_catalog.mount_layout_fixtures(); });
        await s.page.getByTestId("type-link").waitFor();
        for (const id of ["type-link", "type-long-link"]) await read(id, "light");
        await finish("catalog-focus-styles", s);
    }
    // Typography weights (UI-88/89, Text weight rows): computed style and loaded faces.
    {
        const s = await open(browser, CATALOG, 1280, 900);
        await s.page.evaluate(() => window.__component_catalog.mount_layout_fixtures());
        await s.page.getByTestId("type-heading-matrix").waitFor(); await s.page.evaluate(() => document.fonts.ready);
        out.fonts.push(await s.page.evaluate(() => {
            const style = n => { const cs = getComputedStyle(n); return { tag: n.tagName.toLowerCase(), text: n.textContent.trim().slice(0, 40), weight: cs.fontWeight, family: cs.fontFamily.slice(0, 90), size: cs.fontSize, synthesis: cs.fontSynthesisWeight || cs.fontSynthesis }; };
            const weightRows = [...document.querySelectorAll('[data-testid="type-text-matrix"] *')].filter(n => n.children.length === 0 && n.textContent.trim().startsWith("Type weight gives"));
            const heading = document.querySelector('[data-testid="type-heading-matrix"] h1, [data-testid="type-heading-matrix"] h2, [data-testid="type-heading-matrix"] h3');
            const strong = document.querySelector('[data-testid="type-code-matrix"] strong') || document.querySelector('[data-testid="type-strong"]');
            const bodyFamily = getComputedStyle(weightRows[0] || document.body).fontFamily.split(",")[0].replace(/["']/g, "").trim();
            return {
                weightRows: weightRows.map(style), heading: heading && style(heading), strong: strong && style(strong),
                faces: [...document.fonts].map(f => ({ family: f.family.replace(/["']/g, ""), weight: f.weight, style: f.style, status: f.status })),
                check: Object.fromEntries([300, 400, 500, 600, 700].map(w => [w, document.fonts.check(`${w} 16px "${bodyFamily}"`)])), bodyFamily,
            };
        }));
        await finish("catalog-fonts", s);
    }
    // Studio header at 720x900 + root 200% (U1): is the editor header a clipped scroller, and does the wheel scroll it?
    {
        const s = await open(browser, STUDIO, 1440, 900);
        await s.page.setViewportSize({ width: 720, height: 900 });
        await s.page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
        await s.page.waitForTimeout(500);
        const probe = () => s.page.evaluate(() => {
            const editor = document.querySelector("#studio-editor");
            const res = [];
            for (const n of editor.querySelectorAll("*")) {
                const cs = getComputedStyle(n);
                if (cs.position === "sticky" || (n.scrollHeight > n.clientHeight + 1 && /(auto|scroll|hidden)/.test(cs.overflowY))) {
                    const r = n.getBoundingClientRect();
                    res.push({ el: `${n.tagName.toLowerCase()}${n.dataset.testid ? `[${n.dataset.testid}]` : ""}${typeof n.className === "string" && n.className ? "." + n.className.trim().split(/\s+/).slice(0, 2).join(".") : ""}`,
                        position: cs.position, overflowY: cs.overflowY, maxHeight: cs.maxHeight, top: Math.round(r.top), bottom: Math.round(r.bottom), scrollTop: n.scrollTop, scrollHeight: n.scrollHeight, clientHeight: n.clientHeight });
                }
            }
            const pm = [...editor.querySelectorAll("*")].find(n => n.children.length === 0 && n.textContent.trim() === "Preview mode");
            const pr = pm?.getBoundingClientRect();
            return { editorScrollTop: editor.scrollTop, candidates: res.slice(0, 8), previewModeLabel: pr && { top: Math.round(pr.top), bottom: Math.round(pr.bottom) } };
        });
        const before = await probe();
        await s.page.mouse.move(150, 300); await s.page.mouse.wheel(0, 200); await s.page.waitForTimeout(400);
        const afterWheelOnHeader = await probe();
        await s.page.mouse.move(150, 700); await s.page.mouse.wheel(0, 400); await s.page.waitForTimeout(400);
        const afterWheelBelow = await probe();
        out.header200.push({ before, afterWheelOnHeader, afterWheelBelow });
        await finish("studio-header-200", s);
    }
    // UI-156: which colour steps the low-contrast Code samples resolve to.
    const codeSteps = page => page.evaluate(() => {
        const probe = (host, value, prop) => { const n = document.createElement("span"); n.style[prop] = value; host.appendChild(n); const v = getComputedStyle(n)[prop]; n.remove(); return v; };
        return [...document.querySelectorAll(".rustify-code")].map(el => {
            const cs = getComputedStyle(el), host = el.parentElement;
            const step = name => { const raw = cs.getPropertyValue(name).trim(); return raw ? probe(host, raw, "color") : null; };
            return { text: el.textContent.trim().slice(0, 30), variant: el.dataset.variant || null, color: el.dataset.color || null, highContrast: el.hasAttribute("data-high-contrast"),
                computedColor: cs.color, computedBackground: cs.backgroundColor,
                a11: step("--rustify-component-color-a11"), a3: step("--rustify-component-color-a3"), a9: step("--rustify-component-color-a9"),
                contrast: step("--rustify-component-color-contrast"), primary: step("--primary"), primaryForeground: step("--primary-foreground") };
        });
    });
    {
        const s = await open(browser, CATALOG, 1280, 900);
        await s.page.evaluate(() => window.__component_catalog.mount_layout_fixtures());
        await s.page.getByTestId("type-code-matrix").waitFor();
        out.codeSteps.push({ label: "catalog-layout", items: await codeSteps(s.page) });
        await finish("catalog-code-steps", s);
    }
    {
        const s = await open(browser, STUDIO, 1440, 900);
        await s.page.getByTestId("renderer-dom").click(); await s.page.waitForTimeout(300);
        out.codeSteps.push({ label: "studio-preview", items: await codeSteps(s.page) });
        await axe(s.page, "studio-1440-dom-preview");
        await finish("studio-code-steps", s);
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
    out.chromium = browser.version();
    fs.writeFileSync(path.join(__dirname, "machine-check.json"), JSON.stringify(out, null, 2));
    console.log("written");
})();
