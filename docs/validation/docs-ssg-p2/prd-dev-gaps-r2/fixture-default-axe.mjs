// The components fixture built with the default theme (its imported theme
// removed), so that loading, client-only, ready and failed demo blocks are
// scanned in the theme the PRD's AA scope names. The gaps plan scanned these
// states only under the fixture's imported theme.
// node fixture-default-axe.mjs <origin> <output json>
import { chromium } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { writeFile } from "node:fs/promises";

const [origin, output] = process.argv.slice(2);
const tags = ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"];
const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
const report = { origin, browser: browser.version(), tags, scans: [], console: [], pageErrors: [] };
const states = page => page.locator("[data-rustify-island]").evaluateAll(nodes => nodes.map(n => n.getAttribute("data-rd-island-state")));
const all = (page, want) => page.waitForFunction(want => {
  const nodes = [...document.querySelectorAll("[data-rustify-island]")];
  return nodes.length > 0 && nodes.every(n => n.getAttribute("data-rd-island-state") === want);
}, want, { timeout: 60000 });
const rest = page => page.evaluate(async () => {
  await document.fonts.ready;
  await Promise.all(document.getAnimations().filter(a => a.effect?.getTiming().iterations !== Infinity).map(a => a.finished.catch(() => null)));
});
try {
  for (const colorScheme of ["light", "dark"]) for (const width of [1280, 375]) {
    const context = await browser.newContext({ viewport: { width, height: 900 }, colorScheme, reducedMotion: "no-preference" });
    let held = [];
    let hold = true;
    await context.route(/\.wasm(\?|$)/, route => hold ? held.push(route) : route.continue());
    const page = await context.newPage();
    page.on("console", m => { if (m.type() === "error") report.console.push({ colorScheme, width, text: m.text() }); });
    page.on("pageerror", e => report.pageErrors.push({ colorScheme, width, error: String(e) }));
    const scan = async (route, label) => {
      await rest(page);
      const result = await new AxeBuilder({ page }).withTags(tags).analyze();
      report.scans.push({ colorScheme, width, route, label, states: await states(page),
        violations: result.violations.map(v => ({ id: v.id, impact: v.impact, nodes: v.nodes.map(n => n.target) })),
        incomplete: result.incomplete.map(v => ({ id: v.id, nodes: v.nodes.length })) });
    };
    const release = async () => { hold = false; for (const r of held) await r.continue(); held = []; };
    try {
      for (const [route, paused] of [["counter/", "loading"], ["client-only/", "client-only placeholder"]]) {
        hold = true;
        await page.goto(new URL(route, origin).href);
        await all(page, "loading");
        if (route === "client-only/") await page.locator("[data-rustify-client-message]").waitFor();
        await scan(route, paused);
        await release();
        await all(page, "ready");
        await scan(route, "ready");
      }
      hold = false;
      await page.goto(new URL("failure/", origin).href);
      await all(page, "ready");
      await page.getByTestId("failure-crash").click();
      await all(page, "failed");
      await scan("failure/", "failed");
    } finally {
      for (const r of held) await r.abort().catch(() => null);
      await context.close();
    }
  }
} finally { await browser.close(); }
report.summary = {
  scans: report.scans.length,
  labels: [...new Set(report.scans.map(s => `${s.route} ${s.label}: ${[...new Set(s.states)].join("/")}`))],
  serious_or_critical: report.scans.flatMap(s => s.violations.filter(v => ["serious", "critical"].includes(v.impact)).map(v => `${s.colorScheme} ${s.width} ${s.route} ${s.label} ${v.id}`)),
  violations: report.scans.reduce((n, s) => n + s.violations.length, 0),
  incomplete: [...new Set(report.scans.flatMap(s => s.incomplete.map(i => i.id)))],
  unexpected_page_errors: report.pageErrors.filter(e => !/RuntimeError: unreachable/.test(e.error)).length,
};
await writeFile(output, JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify(report.summary, null, 2));
if (report.summary.serious_or_critical.length) process.exitCode = 1;
