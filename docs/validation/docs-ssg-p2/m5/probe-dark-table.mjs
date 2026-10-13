import fs from "node:fs";
import path from "node:path";
import { spawn } from "node:child_process";
import { chromium } from "@playwright/test";

const out = path.resolve("docs/validation/docs-ssg-p2/m5/dark-table-probe-r1"); fs.mkdirSync(out);
const child = spawn(path.resolve("target/docs-cli/rustify-docs"), ["preview", "--dir", "target/docs-ssg/http/official/subpath/dist", "--port", "0"], { detached: true });
let log = "", browser, context, cdp;
const url = await new Promise((resolve, reject) => { let text = ""; child.stdout.on("data", bytes => { text += bytes; const match = /preview: (\S+)/.exec(text); if (match) resolve(match[1]); }); child.stderr.on("data", bytes => log += bytes); child.on("error", reject); child.on("exit", code => reject(new Error(`preview exit${code}: ${log}`))); });
const report = { artifact: "target/docs-ssg/http/official/subpath/dist", url, preview_pid: child.pid, nodes: [], errors: [] };
try {
  browser = await chromium.launch(); report.chromium = browser.version(); context = await browser.newContext({ viewport: { width: 1280, height: 900 } }); const page = await context.newPage();
  page.on("pageerror", error => report.errors.push(String(error)));
  await page.goto(new URL("components/data-table/", url).href); await page.locator("[data-rd-island-state=ready]").waitFor();
  await page.locator('[data-rd-mode="dark"]').click(); await page.evaluate(() => document.fonts.ready);
  cdp = await context.newCDPSession(page); await cdp.send("DOM.enable"); await cdp.send("CSS.enable"); const { root } = await cdp.send("DOM.getDocument");
  for (const selector of ['[data-rd-island-controls] [role="grid"]', '[data-rd-island-controls] [role="gridcell"]', '.rd-prose th', '.rd-prose td', '.rd-prose']) {
    const locator = page.locator(selector).first(); const { nodeId } = await cdp.send("DOM.querySelector", { nodeId: root.nodeId, selector });
    const matched = await cdp.send("CSS.getMatchedStylesForNode", { nodeId });
    const raw = await locator.evaluate(node => { const ancestors = []; for (let current = node; current; current = current.parentElement) { const style = getComputedStyle(current); ancestors.push({ tag: current.tagName, attrs: [...current.attributes].map(attr => [attr.name, attr.value]), color: style.color, background: style.backgroundColor, foreground: style.getPropertyValue("--foreground"), background_token: style.getPropertyValue("--background"), card: style.getPropertyValue("--card"), card_foreground: style.getPropertyValue("--card-foreground") }); } return { text: node.textContent, ancestors }; });
    const relevant = rules => (rules || []).map(({ rule }) => ({ selector: rule.selectorList.text, origin: rule.origin, styleSheetId: rule.styleSheetId, layers: rule.layers, matching: rule.style.cssProperties.filter(property => /^(color|background(?:-color)?|--foreground|--background|--card|--card-foreground)$/.test(property.name)) })).filter(rule => rule.matching.length);
    report.nodes.push({ selector, raw, matched: relevant(matched.matchedCSSRules), inherited: (matched.inherited || []).map(entry => ({ rules: relevant(entry.matchedCSSRules) })).filter(entry => entry.rules.length) });
  }
  fs.writeFileSync(path.join(out, "computed-and-matched.json"), JSON.stringify(report, null, 2) + "\n");
  console.log(JSON.stringify({ artifact: report.artifact, errors: report.errors, nodes: report.nodes.map(node => ({ selector: node.selector, text: node.raw.text.slice(0, 32), color: node.raw.ancestors[0].color, background: node.raw.ancestors[0].background, first_background: node.raw.ancestors.find(parent => parent.background !== "rgba(0, 0, 0, 0)"), own_rule_selectors: node.matched.map(rule => rule.selector) })) }));
} finally {
  if (cdp) await cdp.detach(); if (context) await context.close(); if (browser) await browser.close();
  if (child.exitCode === null && child.signalCode === null) { const exit = new Promise(resolve => child.once("exit", resolve)); process.kill(-child.pid, "SIGTERM"); await exit; }
  fs.writeFileSync(path.join(out, "preview.log"), log);
}
