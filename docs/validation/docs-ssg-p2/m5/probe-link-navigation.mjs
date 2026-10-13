import fs from "node:fs";
import path from "node:path";
import { spawn } from "node:child_process";
import { chromium } from "@playwright/test";

const out = path.resolve("docs/validation/docs-ssg-p2/m5/link-navigation-probe-r1");
fs.mkdirSync(out);
const bin = path.resolve("target/docs-cli/rustify-docs");
const child = spawn(bin, ["preview", "--dir", "target/docs-ssg/http/official/subpath/dist", "--port", "0"], { detached: true });
let log = "", browser, context;
const url = await new Promise((resolve, reject) => {
  let text = "";
  child.stdout.on("data", bytes => { text += bytes; const match = /preview: (\S+)/.exec(text); if (match) resolve(match[1]); });
  child.stderr.on("data", bytes => log += bytes);
  child.on("error", reject); child.on("exit", code => reject(new Error(`preview exit ${code}: ${text} ${log}`)));
});
const report = { preview_pid: child.pid, bin, url, errors: [] };
try {
  browser = await chromium.launch(); report.chromium = browser.version();
  context = await browser.newContext(); await context.tracing.start({ snapshots: true, screenshots: true });
  const page = await context.newPage(); page.on("pageerror", error => report.errors.push(String(error)));
  await page.addInitScript(() => {
    window.__link_events = [];
    const original = Event.prototype.preventDefault;
    Event.prototype.preventDefault = function () {
      if (this.type === "click") window.__link_events.push({ at: "preventDefault", target: this.target?.textContent, href: this.target?.closest?.("a")?.getAttribute("href"), stack: new Error().stack });
      return original.call(this);
    };
    for (const capture of [true, false]) document.addEventListener("click", event => {
      const link = event.target.closest?.("a[data-name=Link]");
      if (link) window.__link_events.push({ at: capture ? "document-capture" : "document-bubble", defaultPrevented: event.defaultPrevented, href: link.getAttribute("href"), path: event.composedPath().filter(node => node instanceof Element).map(node => ({ tag: node.tagName, scope: node.getAttribute("data-rustify-scope"), name: node.getAttribute("data-name") })) });
    }, capture);
  });
  await page.goto(new URL("components/link/", url).href);
  await page.locator("[data-rustify-island][data-rd-island-state=ready]").waitFor();
  const link = page.getByRole("link", { name: "Jump to the destination", exact: true });
  report.before = await link.evaluate(node => ({ href: node.getAttribute("href"), targetExists: !!document.getElementById(node.getAttribute("href").slice(1)), hash: location.hash, scope: node.closest("[data-rustify-scope]")?.getAttribute("data-rustify-scope") }));
  await link.click();
  await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  report.after = await page.evaluate(() => ({ hash: location.hash, active: document.activeElement?.outerHTML, events: window.__link_events }));
  fs.writeFileSync(path.join(out, "probe.json"), JSON.stringify(report, null, 2) + "\n");
  console.log(JSON.stringify({ before: report.before, hash: report.after.hash, events: report.after.events.map(event => ({ at: event.at, defaultPrevented: event.defaultPrevented })) }));
} finally {
  if (context) { await context.tracing.stop({ path: path.join(out, "trace.zip") }); await context.close(); }
  if (browser) await browser.close();
  if (child.exitCode === null && child.signalCode === null) { const exit = new Promise(resolve => child.once("exit", resolve)); process.kill(-child.pid, "SIGTERM"); await exit; }
  fs.writeFileSync(path.join(out, "preview.log"), log);
}
