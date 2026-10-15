// Groups axe "incomplete" checks for the loading and failed scans by reason and by
// where the node sits. node state-incomplete.mjs <origin> <loading|failed>
import { chromium } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
const [origin, state] = process.argv.slice(2);
const browser = await chromium.launch();
const out = {};
for (const route of (process.argv[4] || "components/button/,components/tabs/").split(",")) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 }, colorScheme: "light" });
  const held = [];
  await context.route(/\.wasm(\?|$)/, r => state === "failed" ? r.abort("failed") : held.push(r));
  const page = await context.newPage();
  await page.goto(new URL(route, origin).href);
  await page.waitForFunction(s => document.querySelector("[data-rustify-island]")?.getAttribute("data-rd-island-state") === s, state);
  await page.evaluate(() => document.fonts.ready);
  const scan = await new AxeBuilder({ page }).withTags(["wcag2a","wcag2aa","wcag21a","wcag21aa","wcag22aa"]).analyze();
  const reasons = {};
  for (const v of scan.incomplete) for (const n of v.nodes) for (const c of [...n.any, ...n.all, ...n.none]) {
    const key = (c.data && c.data.messageKey) || c.id; const where = await page.evaluate(sel => { const e = document.querySelector(sel); return e?.closest("pre") ? "code block" : e?.closest("[data-rustify-scope] [data-rustify-scope], .rd-demo-canvas") ? "demo canvas" : e?.closest("nav, aside") ? "navigation" : "other"; }, n.target[0]);
    const k = key + " (" + where + ")"; reasons[k] = (reasons[k] || 0) + 1;
  }
  out[route] = reasons;
  for (const r of held) await r.abort("failed").catch(() => null);
  await context.close();
}
await browser.close();
console.log(JSON.stringify(out, null, 1));
