// Default-theme demo blocks in their loading and failed states, which the
// gaps plan scanned only under the components fixture's imported theme.
// Loading: the wasm request is held unanswered. Failed: the wasm request is
// refused, so the loader restores the initial view with its failure notice.
// node state-axe.mjs <official site origin with base> <output json>
import { chromium } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { writeFile } from "node:fs/promises";

const [origin, output] = process.argv.slice(2);
const routes = ["components/button/", "components/tabs/"];
const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
const report = { origin, browser: browser.version(), tags: ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"], scans: [] };
try {
  for (const state of ["loading", "failed"]) for (const colorScheme of ["light", "dark"]) for (const width of [1280, 375]) {
    const context = await browser.newContext({ viewport: { width, height: 900 }, colorScheme, reducedMotion: "no-preference" });
    const held = [];
    await context.route(/\.wasm(\?|$)/, route => state === "failed" ? route.abort("failed") : held.push(route));
    const page = await context.newPage();
    const errors = [];
    page.on("pageerror", e => errors.push(String(e)));
    try {
      for (const route of routes) {
        await page.goto(new URL(route, origin).href);
        const islands = page.locator("[data-rustify-island]");
        const count = await islands.count();
        await page.waitForFunction(([want, n]) => {
          const all = [...document.querySelectorAll("[data-rustify-island]")];
          return all.length === n && all.every(node => node.getAttribute("data-rd-island-state") === want);
        }, [state, count], { timeout: 30000 });
        await page.evaluate(() => document.fonts.ready);
        // Let finite transitions end (normal preference), as the plan's steady scans do.
        await page.evaluate(() => Promise.all(document.getAnimations().filter(a => a.effect?.getTiming().iterations !== Infinity).map(a => a.finished.catch(() => null))));
        const states = await islands.evaluateAll(nodes => nodes.map(node => node.getAttribute("data-rd-island-state")));
        const notice = await page.locator("[data-rustify-island]").first().evaluate(node => node.innerText.slice(0, 400));
        const scan = await new AxeBuilder({ page }).withTags(report.tags).analyze();
        report.scans.push({
          state, colorScheme, width, route, islands: count, states,
          notice,
          violations: scan.violations.map(v => ({ id: v.id, impact: v.impact, nodes: v.nodes.map(n => ({ target: n.target, summary: n.failureSummary })) })),
          incomplete: scan.incomplete.map(v => ({ id: v.id, nodes: v.nodes.length })),
        });
      }
      report.pageErrors = (report.pageErrors || []).concat(errors.map(e => ({ state, colorScheme, width, error: e })));
    } finally {
      for (const route of held) await route.abort("failed").catch(() => null);
      await context.close();
    }
  }
} finally { await browser.close(); }
report.summary = {
  scans: report.scans.length,
  states: [...new Set(report.scans.map(s => `${s.state}:${s.states.join("/")}`))],
  serious_or_critical: report.scans.flatMap(s => s.violations.filter(v => v.impact === "serious" || v.impact === "critical").map(v => `${s.state} ${s.colorScheme} ${s.width} ${s.route} ${v.id}`)),
  violations: report.scans.reduce((n, s) => n + s.violations.length, 0),
  incomplete: [...new Set(report.scans.flatMap(s => s.incomplete.map(i => i.id)))],
};
await writeFile(output, JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify(report.summary, null, 2));
if (report.summary.serious_or_critical.length) process.exitCode = 1;
