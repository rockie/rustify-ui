import AxeBuilder from "@axe-core/playwright";
import { chromium } from "@playwright/test";
import { readFile, writeFile } from "node:fs/promises";

const origin = process.argv[2];
if (!origin || new URL(origin).hostname !== "127.0.0.1") throw new Error("supply the owned local preview URL");
const registry = await readFile("site/src/lib.rs", "utf8");
const routes = [...registry.matchAll(/\w+ \{ route: "([^\"]+)"/g)].map(match => match[1]);
const pages = [...routes.map(route => route.slice(1)), "quickstart/", "__gaps_missing__/deeper/"];
const out = new URL("./accessibility.json", import.meta.url);
const report = { origin, cli: "0.1.15", build_id: "b9267374bcc32cc6", tags: ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"], checks: [] };
const browser = await chromium.launch();
report.browser = browser.version();
try {
  for (const colorScheme of ["light", "dark"]) {
    for (const width of [1280, 375]) {
      const context = await browser.newContext({ colorScheme, viewport: { width, height: 900 }, serviceWorkers: "block" });
      try {
        const page = await context.newPage();
        for (const route of pages) {
          const response = await page.goto(new URL(route, origin).href);
          if (route.startsWith("components/")) {
            await page.waitForFunction(() => document.querySelector("[data-rustify-island]")?.getAttribute("data-rd-island-state") === "ready");
          }
          await page.evaluate(() => document.fonts.ready);
          const { violations, incomplete } = await new AxeBuilder({ page }).withTags(report.tags).analyze();
          report.checks.push({ route, colorScheme, width, status: response.status(), violations, incomplete });
          await writeFile(out, JSON.stringify(report, null, 2) + "\n");
        }
      } finally { await context.close(); }
    }
  }
} finally { await browser.close(); }
report.serious_or_critical = report.checks.flatMap(check => check.violations.filter(item => ["serious", "critical"].includes(item.impact)).map(item => ({ route: check.route, colorScheme: check.colorScheme, width: check.width, id: item.id, impact: item.impact, targets: item.nodes.map(node => node.target) })));
report.passed = report.serious_or_critical.length === 0;
await writeFile(out, JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify({ browser: report.browser, checks: report.checks.length, passed: report.passed, serious_or_critical: report.serious_or_critical }, null, 2));
process.exitCode = report.passed ? 0 : 1;
