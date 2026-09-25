// Computed-style snapshot of every component-catalog page, light and dark.
//
//   node docs/validation/dx/probes/computed-style.mjs snapshot <url> <out.json>
//   node docs/validation/dx/probes/computed-style.mjs compare <before.json> <after.json>
//
// `snapshot` walks every `nav-*` page of a served catalogue build in both
// themes and records `getComputedStyle` for every element inside
// `[data-rustify-scope]`, keyed by its DOM path, with transitions read at their end
// and endless animations at their start, so neither timing nor a spinner is a
// difference. `compare` prints every
// (page, theme, element, property) whose value changed and exits 1 if any did.
// What changes between the two runs is class names only, so the DOM paths
// line up and any difference is a style the class change did not keep.

import { chromium, devices } from "@playwright/test";
import { readFileSync, writeFileSync } from "node:fs";

const [mode, a, b] = process.argv.slice(2);

async function snapshot(url, out) {
    const browser = await chromium.launch();
    const context = await browser.newContext({ ...devices["Desktop Chrome"], reducedMotion: "reduce" });
    const page = await context.newPage();
    await page.goto(url);
    await page.waitForSelector('[data-testid="status"][data-status="ready"]', { timeout: 60_000 });
    const navs = await page.evaluate(() =>
        [...document.querySelectorAll('[data-testid^="nav-"]')].map((e) => e.getAttribute("data-testid"))
    );
    const result = {};
    for (const theme of ["light", "dark"]) {
        for (const nav of navs) {
            await page.getByTestId(nav).first().click();
            const current = await page.evaluate(() => window.__component_catalog.snapshot().theme);
            if (current !== theme) {
                await page.getByTestId("toggle-theme").first().click();
                await page.waitForFunction((t) => window.__component_catalog.snapshot().theme === t, theme);
            }
            // Hover and focus are the pointer's and the keyboard's, not the
            // classes': park both before reading.
            await page.mouse.move(0, 0);
            await page.evaluate(() => document.activeElement?.blur());
            await page.evaluate(() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r))));
            result[`${nav}|${theme}`] = await page.evaluate(() => {
                // A transition is read at its end, which does not depend on
                // when it started; an endless animation at its start.
                for (const animation of document.getAnimations()) {
                    const endless = animation.effect?.getComputedTiming().endTime === Infinity;
                    if (endless) {
                        animation.pause();
                        animation.currentTime = 0;
                    } else {
                        animation.finish();
                    }
                }
                const path = (element) => {
                    const parts = [];
                    for (let e = element; e && e !== document.body; e = e.parentElement) {
                        const index = e.parentElement ? [...e.parentElement.children].indexOf(e) : 0;
                        parts.push(`${e.tagName.toLowerCase()}:${index}`);
                    }
                    return parts.reverse().join("/");
                };
                const out = {};
                for (const scope of document.querySelectorAll("[data-rustify-scope]")) {
                    for (const element of [scope, ...scope.querySelectorAll("*")]) {
                        const style = getComputedStyle(element);
                        const values = {};
                        for (let i = 0; i < style.length; i++) {
                            const name = style[i];
                            values[name] = style.getPropertyValue(name);
                        }
                        out[path(element)] = values;
                    }
                }
                return out;
            });
        }
    }
    await browser.close();
    writeFileSync(out, JSON.stringify(result));
    const elements = Object.values(result).reduce((n, page) => n + Object.keys(page).length, 0);
    console.log(`${Object.keys(result).length} page states, ${elements} element snapshots -> ${out}`);
}

function compare(beforePath, afterPath) {
    const before = JSON.parse(readFileSync(beforePath, "utf8"));
    const after = JSON.parse(readFileSync(afterPath, "utf8"));
    let differences = 0;
    let checked = 0;
    const report = (line) => {
        differences += 1;
        if (differences <= 200) console.log(line);
    };
    for (const key of new Set([...Object.keys(before), ...Object.keys(after)])) {
        const left = before[key] ?? {};
        const right = after[key] ?? {};
        for (const element of new Set([...Object.keys(left), ...Object.keys(right)])) {
            if (!(element in left) || !(element in right)) {
                report(`${key} ${element}: present only ${element in left ? "before" : "after"}`);
                continue;
            }
            for (const property of new Set([...Object.keys(left[element]), ...Object.keys(right[element])])) {
                checked += 1;
                if (left[element][property] !== right[element][property]) {
                    report(`${key} ${element} ${property}: ${left[element][property]} -> ${right[element][property]}`);
                }
            }
        }
    }
    console.log(`${checked} values compared, ${differences} differ`);
    process.exit(differences === 0 ? 0 : 1);
}

if (mode === "snapshot") await snapshot(a, b);
else if (mode === "compare") compare(a, b);
else {
    console.error("usage: computed-style.mjs snapshot <url> <out.json> | compare <before.json> <after.json>");
    process.exit(2);
}
