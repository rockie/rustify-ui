import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { spawn } from "node:child_process";
import { chromium, expect } from "@playwright/test";

const repo = process.cwd(), prefix = path.resolve("docs/validation/docs-ssg-p2/m6"), state = JSON.parse(fs.readFileSync("target/docs-ssg/m6-installed-c.json", "utf8"));
const report = { scope: "one actual unmodified installed-C template browser path; no visual review, physical device, human or performance claim", candidate: state.candidate_sha, framework: state.framework_revision, errors: [], csp: [], failures: [], responses: [] };
const source = fs.readFileSync(path.join(state.template_site, "src/demos.rs"), "utf8"), fn = source.match(/^pub fn counter_demo\(\)[\s\S]*?^\}(?:\n|$)/m)[0];
const child = spawn(state.binary, ["preview", "--dir", path.join(state.template_site, "dist"), "--port", "0"], { cwd: state.template_site, detached: true }); let log = "", browser, context;
const url = await new Promise((resolve, reject) => { let text = ""; child.stdout.on("data", bytes => { text += bytes; const match = /preview: (\S+)/.exec(text); if (match) resolve(match[1]); }); child.stderr.on("data", bytes => log += bytes); child.on("error", reject); child.on("exit", code => reject(new Error(`preview exit${code} ${log}`))); });
report.preview = { pid: child.pid, command: [state.binary, "preview", "--dir", path.join(state.template_site, "dist"), "--port", "0"], url };
try {
  browser = await chromium.launch(); report.browser_version = browser.version(); report.browser_executable = chromium.executablePath(); context = await browser.newContext();
  await context.tracing.start({ snapshots: true, screenshots: false }); const page = await context.newPage();
  await page.exposeBinding("__installed_c_csp", (_, value) => report.csp.push(value)); await page.addInitScript(() => document.addEventListener("securitypolicyviolation", event => window.__installed_c_csp(`${event.violatedDirective} ${event.blockedURI}`)));
  page.on("pageerror", error => report.errors.push(String(error))); page.on("console", message => { if (message.type() === "error") report.errors.push(message.text()); }); page.on("requestfailed", request => report.failures.push(`${request.url()} ${request.failure()?.errorText}`)); page.on("response", response => report.responses.push({ url: response.url(), status: response.status(), mime: response.headers()["content-type"] || "" }));
  let release; const held = new Promise(resolve => release = resolve); await page.route("**/assets/islands/**/app_bg.wasm", async route => { await held; await route.continue(); });
  await page.goto(url, { waitUntil: "commit" }); const islands = page.locator("[data-rustify-island]"); await expect(islands).toHaveCount(2); await expect(islands.first()).toHaveAttribute("data-rd-island-state", "loading");
  const fields = page.locator("fieldset[data-rd-island-controls]"); await expect(fields.nth(0).getByTestId("counter-value")).toHaveText("2"); await expect(fields.nth(1).getByTestId("counter-value")).toHaveText("7");
  report.before = await page.evaluate(() => {
    window.__installed_c_original = [...document.querySelectorAll("fieldset[data-rd-island-controls]")].map(field => { const root = field.firstElementChild, nodes = [], ids = [...root.querySelectorAll("[id]")].map(node => node.id), walk = document.createTreeWalker(root, NodeFilter.SHOW_ALL); for (let node = walk.currentNode; node; node = walk.nextNode()) nodes.push(node); return { root, nodes, ids }; });
    return window.__installed_c_original.map(entry => ({ nodes: entry.nodes.length, ids: entry.ids }));
  });
  await expect(page.locator(".rd-demo-source [data-rd-source-path]")).toHaveText("src/demos.rs");
  const raw = page.locator(".rd-demo-source template.rd-code-raw"); assert.deepEqual(await raw.evaluateAll(nodes => nodes.map(node => node.content.textContent)), [fn, source]);
  release(); await expect(islands.nth(0)).toHaveAttribute("data-rd-island-state", "ready"); await expect(islands.nth(1)).toHaveAttribute("data-rd-island-state", "ready"); await page.evaluate(() => document.fonts.ready);
  report.after = await page.evaluate(() => window.__installed_c_original.map(entry => ({ same_root: entry.root.parentElement.matches("fieldset[data-rd-island-controls]"), retained_nodes: entry.nodes.filter(node => node.isConnected && entry.root.contains(node)).length, ids: [...entry.root.querySelectorAll("[id]")].map(node => node.id) })));
  for (let index = 0; index < 2; index++) { assert.equal(report.after[index].same_root, true); assert.equal(report.after[index].retained_nodes, report.before[index].nodes); assert.deepEqual(report.after[index].ids, report.before[index].ids); }
  await fields.nth(0).getByTestId("counter-increment").click(); await expect(fields.nth(0).getByTestId("counter-value")).toHaveText("3"); await expect(fields.nth(1).getByTestId("counter-value")).toHaveText("7");
  await fields.nth(1).getByTestId("counter-input").fill("31"); await expect(fields.nth(1).getByTestId("counter-value")).toHaveText("31"); await expect(fields.nth(0).getByTestId("counter-value")).toHaveText("3");
  await fields.nth(0).getByTestId("counter-increment").press("Enter"); await expect(fields.nth(0).getByTestId("counter-value")).toHaveText("4");
  const full = page.locator(".rd-demo-source").getByRole("tab", { name: "Full file", exact: true }); await full.click(); await expect(full).toHaveAttribute("aria-selected", "true"); assert.deepEqual(await raw.evaluateAll(nodes => nodes.map(node => node.content.textContent)), [fn, source]);
  await expect(fields.locator("[style]")).toHaveCount(0); assert.deepEqual(report.errors, []); assert.deepEqual(report.csp, []); assert.deepEqual(report.failures, []); assert(report.responses.every(response => response.status < 400));
  assert(report.responses.some(response => response.url.endsWith(".wasm") && response.status === 200 && response.mime === "application/wasm")); assert(!report.responses.some(response => /makepad/i.test(response.url)));
  report.final_values = await fields.getByTestId("counter-value").allTextContents(); report.source_exact = true; report.passed = true;
} catch (error) { report.passed = false; report.failure = String(error); console.error(error); process.exitCode = 1; }
finally {
  if (context) { await context.tracing.stop({ path: path.join(prefix, "installed-c-template-browser-r1-trace.zip") }); await context.close(); } if (browser) await browser.close();
  if (child.exitCode === null && child.signalCode === null) { const exit = new Promise(resolve => child.once("exit", resolve)); process.kill(-child.pid, "SIGTERM"); await exit; }
  fs.writeFileSync(path.join(prefix, "installed-c-template-preview-r1.log"), log); fs.writeFileSync(path.join(prefix, "installed-c-template-browser-r1.json"), JSON.stringify(report, null, 2) + "\n"); console.log(JSON.stringify({ passed: report.passed, failure: report.failure, final_values: report.final_values }));
}
