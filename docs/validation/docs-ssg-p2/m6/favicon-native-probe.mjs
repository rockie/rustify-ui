import assert from "node:assert/strict";
import { chromium } from "@playwright/test";
import { spawn, spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

const bin = path.resolve(process.argv[2]), out = path.resolve(process.argv[3]);
const work = mkdtempSync(path.join(tmpdir(), "rustify-favicon-native-"));
const site = path.join(work, "content-site");
const report = { scope: "development CLI content shell regression; not frozen pair acceptance", bin, work, commands: [], requests: [], responses: [], console: [], pageErrors: [] };
let server, browser;
function run(args) {
  const result = spawnSync(bin, args, { encoding: "utf8", timeout: 60_000 });
  report.commands.push({ args, exit: result.status, stdout: result.stdout, stderr: result.stderr });
  assert.equal(result.status, 0, result.stderr);
}
try {
  run(["--build-info"]);
  run(["new", site, "--template", "content"]);
  run(["build", "--config", path.join(site, "rustify-docs.toml"), "--format", "json"]);
  const html = readFileSync(path.join(site, "dist/index.html"), "utf8");
  assert(html.includes('<link rel="icon" href="data:image/svg+xml,'));
  server = spawn(bin, ["preview", "--dir", path.join(site, "dist"), "--port", "0"]);
  const address = await new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error("preview startup timeout")), 10_000);
    let text = "";
    server.stdout.on("data", bytes => { text += bytes; const match = text.match(/http:\/\/\S+/); if (match) { clearTimeout(timer); resolve(match[0]); } });
    server.on("error", reject); server.on("exit", code => reject(new Error(`preview exited ${code}`)));
  });
  browser = await chromium.launch({ executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" });
  report.chrome = browser.version();
  const page = await browser.newPage();
  page.on("request", request => report.requests.push(request.url()));
  page.on("response", response => report.responses.push({ url: response.url(), status: response.status() }));
  page.on("console", message => { if (message.type() === "error") report.console.push({ text: message.text(), url: message.location().url }); });
  page.on("pageerror", error => report.pageErrors.push(String(error)));
  await page.goto(address, { waitUntil: "networkidle" });
  report.icon = await page.locator('head link[rel="icon"]').getAttribute("href");
  assert(report.icon.startsWith("data:image/svg+xml,"));
  await page.waitForTimeout(1000);
  assert.deepEqual(report.console, []); assert.deepEqual(report.pageErrors, []);
  assert(report.responses.every(response => response.status === 200));
  assert(!report.requests.some(url => new URL(url).pathname.endsWith("/favicon.ico")));
  report.passed = true;
} catch (error) { report.passed = false; report.failure = String(error); process.exitCode = 1; }
finally {
  if (browser) await browser.close();
  if (server?.exitCode === null && server.signalCode === null) { const done = new Promise(resolve => server.once("exit", resolve)); server.kill("SIGTERM"); await done; }
  writeFileSync(out, JSON.stringify(report, null, 2) + "\n");
  console.log(JSON.stringify({ passed: report.passed, chrome: report.chrome, failure: report.failure, out }));
}
