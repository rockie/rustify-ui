import { chromium } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { writeFile } from "node:fs/promises";

const origin = process.argv[2];
const browser = await chromium.launch();
const report = { browser: browser.version(), cases: [] };
try {
  for (const width of [1280, 375]) {
    const context = await browser.newContext({ viewport: { width, height: 900 }, colorScheme: "light", reducedMotion: "reduce" });
    try {
      const page = await context.newPage();
      for (const entry of [{ route: "components/button/", target: '.rd-demo-canvas [data-name="Button"]', hover: true }, { route: "components/tabs/", target: '[role="tab"][aria-selected="false"]:not([disabled])', hover: false }]) {
        await page.goto(new URL(entry.route, origin).href);
        await page.waitForFunction(() => document.querySelector("[data-rustify-island]")?.getAttribute("data-rd-island-state") === "ready");
        await page.evaluate(() => document.fonts.ready);
        await page.mouse.move(0, 0);
        const control = page.locator(entry.target).first();
        for (const hover of entry.hover ? [false, true] : [false]) {
          if (hover) await control.hover();
          await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
          const computed = await control.evaluate(node => {
            const css = getComputedStyle(node);
            const parents = [];
            for (let at = node; at; at = at.parentElement) {
              const style = getComputedStyle(at);
              parents.push({ tag: at.tagName, name: at.getAttribute("data-name"), class: at.className, opacity: style.opacity, color: style.color, background: style.backgroundColor });
            }
            return { html: node.outerHTML, hovered: node.matches(":hover"), disabled: node.disabled, color: css.color, background: css.backgroundColor, primary: css.getPropertyValue("--primary"), mutedForeground: css.getPropertyValue("--muted-foreground"), colorMutedForeground: css.getPropertyValue("--color-muted-foreground"), fontSize: css.fontSize, parents };
          });
          const scan = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
          const name = `${entry.route.includes("tabs") ? "tabs" : "button"}-${width}-${hover ? "hover" : "rest"}`;
          await page.screenshot({ path: new URL(`./${name}.png`, import.meta.url).pathname, fullPage: false });
          report.cases.push({ name, width, hover, computed, violations: scan.violations.filter(item => item.id === "color-contrast") });
        }
      }
    } finally { await context.close(); }
  }
} finally { await browser.close(); }
await writeFile(new URL("./contrast-confirmation.json", import.meta.url), JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify(report.cases.map(item => ({ name: item.name, computed: { color: item.computed.color, background: item.computed.background, primary: item.computed.primary, mutedForeground: item.computed.mutedForeground }, contrastViolations: item.violations.map(violation => violation.nodes.map(node => node.any.map(check => check.data))) })), null, 2));
