import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import os from "node:os";
import { spawn } from "node:child_process";
import { get } from "node:http";
import { createHash } from "node:crypto";
import { chromium } from "@playwright/test";

const round = process.argv[2] || "r1"; assert(/^r\d+$/.test(round));
const repo = process.cwd(), out = path.resolve(`docs/validation/docs-ssg-p2/m5/official-dev-${round}`);
fs.mkdirSync(out);
const sha = bytes => createHash("sha256").update(bytes).digest("hex");
const wait = ms => new Promise(resolve => setTimeout(resolve, ms));
const token = `m5-live-${process.pid}`;
const readme = path.resolve("README.md"), guide = path.resolve("docs/quickstart.md");
const originals = new Map([readme, guide].map(file => [file, fs.readFileSync(file)]));
const expected = new Map(originals), extras = new Map();
const report = { scope: "actual repository root dev; actual source probes with byte guarded restoration; no performance claim", repo, token, source_before: [...originals].map(([file, bytes]) => ({ file, sha256: sha(bytes) })), phases: [], errors: [], expectedStreamClosures: [], restoration: [] };
const states = [];
let child, stream, browser, context, cliLog = "", browserRequests = [], responses = [];
const journal = path.join(out, "cargo-calls.jsonl"), wrapper = path.join(out, "cargo-observe");
fs.writeFileSync(wrapper, `#!/usr/bin/env python3\nimport sys,json,time,subprocess,os\nwith open(${JSON.stringify(journal)},'a') as f: f.write(json.dumps({'argv':sys.argv[1:],'at_ms':time.time()*1000,'pid':os.getpid()})+'\\n')\nraise SystemExit(subprocess.call(['mbx']+sys.argv[1:]))\n`, { mode: 0o755 });
const calls = () => fs.existsSync(journal) ? fs.readFileSync(journal, "utf8").trim().split("\n").filter(Boolean).map(line => JSON.parse(line)) : [];
const unchanged = (file, bytes) => assert.deepEqual(fs.readFileSync(file), bytes, `unexpected concurrent source change: ${file}`);
function writeOwned(file, bytes) { unchanged(file, expected.get(file)); fs.writeFileSync(file, bytes); expected.set(file, Buffer.from(bytes)); }
function extraFile(file, bytes) { assert(!fs.existsSync(file)); fs.writeFileSync(file, bytes, { flag: "wx" }); extras.set(file, { bytes: Buffer.from(bytes) }); }
function editExtra(file, bytes) { unchanged(file, extras.get(file).bytes); fs.writeFileSync(file, bytes); extras.set(file, { bytes: Buffer.from(bytes) }); }
function aliasTo(file, target) { if (extras.has(file)) { assert.equal(fs.readlinkSync(file), extras.get(file).link); fs.unlinkSync(file); } else assert(!fs.existsSync(file)); fs.symlinkSync(target, file); extras.set(file, { link: target }); }
async function until(predicate, timeout = 600_000) { const began = Date.now(); while (Date.now() - began < timeout) { const state = states.at(-1); if (state && predicate(state)) return state; await wait(20); } throw new Error(`actual dev state timeout: ${JSON.stringify(states.at(-1))}`); }
async function settled(previous) { return until(state => !state.building && state.sequence > previous.sequence); }
async function documentAt(url) { const response = await fetch(url); assert.equal(response.status, 200); const bytes = Buffer.from(await response.arrayBuffer()); return { bytes, sha256: sha(bytes) }; }
// dev::stamped changes only this served-script good revision for every document.
// Keep all content, source, ids, module URLs and the session untouched.
function withoutServedRevision(bytes) { const text = bytes.toString(); const stamp = /src="\/__rustify-docs\/dev\.js\?session=[^"&]+&amp;good=\d+"/g; assert.equal([...text.matchAll(stamp)].length, 1); return text.replace(stamp, value => value.replace(/good=\d+/, "good=CURRENT")); }
function sameCompile(state, initial, count) { assert.deepEqual(state.compile, initial.compile, "content/existence changes must keep the exact compiler epoch"); assert.equal(calls().length, count, "no Cargo command, including metadata, may run after content-only saves"); }
async function quiet(name, mutate, current, initial, count) {
  const before = states.length, started = Date.now(); mutate(); await wait(1500);
  assert.equal(states.length, before, `${name} must not emit a new state or start a build`);
  sameCompile(states.at(-1), initial, count); report.phases.push({ name, started_at_ms: started, observed_ms: Date.now() - started, no_new_state: true, state: current, cargo_calls: count });
}

const loose = path.resolve(`docs/${token}-unselected.md`), binary = path.resolve(`docs/${token}-unreferenced.bin`);
const validation = path.resolve(`docs/validation/docs-ssg-p2/m5/${token}-excluded.md`);
const targetA = path.resolve(`docs/${token}-a.md`), targetB = path.resolve(`docs/${token}-b.md`), alias = path.resolve(`docs/${token}-alias.md`);
const externalDir = fs.mkdtempSync(path.join(os.tmpdir(), "rustify-m5-retarget-")), outside = path.join(externalDir, "outside.md");
fs.writeFileSync(outside, Buffer.from([0xff, 0xfe]));
try {
  extraFile(loose, Buffer.from([0xff, 0xfe])); extraFile(binary, Buffer.from([0, 255])); extraFile(validation, "excluded validation evidence\n");
  extraFile(targetA, Buffer.from([0xff, 0xfe, 1])); extraFile(targetB, Buffer.from([0xfd, 0xfc, 2])); aliasTo(alias, path.basename(targetA));
  const bin = path.resolve(process.env.RUSTIFY_DOCS_BIN || "target/docs-cli/rustify-docs");
  report.binary_sha256 = sha(fs.readFileSync(bin));
  child = spawn(bin, ["dev", "--config", "rustify-docs.toml", "--port", "0"], { cwd: repo, detached: true, env: { ...process.env, CARGO_NET_OFFLINE: "true", RUSTIFY_DOCS_CARGO: wrapper } });
  report.process = { pid: child.pid, command: [bin, "dev", "--config", "rustify-docs.toml", "--port", "0"], cwd: repo };
  child.stderr.on("data", bytes => cliLog += bytes);
  const url = await new Promise((resolve, reject) => { let text = ""; child.stdout.on("data", bytes => { text += bytes; const match = /^dev: (\S+)\n/m.exec(text); if (match) resolve(match[1]); }); child.on("error", reject); child.on("exit", code => reject(new Error(`dev exited ${code}: ${cliLog}`))); });
  report.url = url;
  stream = get(new URL(`/__rustify-docs/events?path=${encodeURIComponent(new URL(url).pathname)}&good=0`, url), response => { response.setEncoding("utf8"); let text = ""; response.on("data", bytes => { text += bytes; let cut; while ((cut = text.indexOf("\n\n")) >= 0) { const message = text.slice(0, cut); text = text.slice(cut + 2); if (message.startsWith("event: state\ndata: ")) states.push(JSON.parse(message.slice(19))); } }); response.on("error", () => {}); }); stream.on("error", () => {});
  const initial = await until(state => !state.building && state.good !== null); assert.equal(initial.ok, true, JSON.stringify(initial));
  const count = calls().length; report.initial = { state: initial, cargo_calls: calls() }; let current = initial;
  browser = await chromium.launch(); report.chromium = browser.version(); context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  await context.exposeBinding("__m5_csp", (_, value) => report.errors.push(`CSP ${value}`)); await context.addInitScript(() => document.addEventListener("securitypolicyviolation", event => window.__m5_csp(`${event.violatedDirective} ${event.blockedURI}`)));
  context.on("page", page => { page.on("pageerror", error => report.errors.push(String(error))); page.on("console", message => { if (message.type() === "error") report.errors.push(message.text()); }); page.on("request", request => browserRequests.push({ page: page.url(), url: request.url() })); page.on("response", response => responses.push({ url: response.url(), status: response.status() })); page.on("requestfailed", request => { const failure = `${request.url()} ${request.failure()?.errorText}`; if (new URL(request.url()).pathname === "/__rustify-docs/events" && request.failure()?.errorText === "net::ERR_ABORTED") report.expectedStreamClosures.push(failure); else report.errors.push(failure); }); });
  const page = await context.newPage(), demoPage = await context.newPage();
  await page.goto(url); await page.evaluate(() => document.fonts.ready);
  const demoUrl = new URL("components/button/", url).href; await demoPage.goto(demoUrl); await demoPage.locator("[data-rd-island-state=ready]").waitFor(); await demoPage.evaluate(() => document.fonts.ready);
  await demoPage.getByRole("button", { name: "Increment", exact: true }).click(); assert.equal(await demoPage.locator("[data-demo-state]").textContent(), "1");
  const originalDemo = await documentAt(demoUrl), moduleUrl = await demoPage.locator('script[type="module"][src*="/assets/islands/"]').getAttribute("src");
  const wasmUrl = new URL("app_bg.wasm", new URL(moduleUrl, url)).href, wasmResponse = await fetch(wasmUrl); assert.equal(wasmResponse.headers.get("content-type"), "application/wasm"); const wasmBytes = Buffer.from(await wasmResponse.arrayBuffer());
  await demoPage.evaluate(() => { window.__m5_original_scope = document.querySelector("fieldset[data-rd-island-controls]").firstElementChild; window.__m5_original_nodes = []; const walk = document.createTreeWalker(window.__m5_original_scope, NodeFilter.SHOW_ALL); for (let node = walk.currentNode; node; node = walk.nextNode()) window.__m5_original_nodes.push(node); });
  const demoWasmCount = () => browserRequests.filter(request => request.url.endsWith(".wasm")).length;
  const requestedWasm = demoWasmCount(); report.initial.demo = { html_sha256: originalDemo.sha256, wasm_url: wasmUrl, wasm_sha256: sha(wasmBytes), wasm_requests: requestedWasm };

  const readmeMarker = `${token} README introduction changed from its actual source.`;
  const readmeProbe = Buffer.from(originals.get(readme).toString().replace("\n## Quick start", `\n\n${readmeMarker}\n\n## Quick start`)); assert(!readmeProbe.equals(originals.get(readme)));
  writeOwned(readme, readmeProbe); current = await settled(current); assert.equal(current.ok, true); sameCompile(current, initial, count);
  await page.waitForFunction(marker => document.querySelector(".rd-home-prose")?.textContent.includes(marker), readmeMarker);
  const home = await documentAt(url); assert(home.bytes.includes(Buffer.from(readmeMarker))); report.phases.push({ name: "root README actual source save", state: current, html_sha256: home.sha256, marker: readmeMarker, no_new_cargo: true });
  const guideMarker = `${token} selected guide changed from its actual source.`, repositoryHref = `https://github.com/rockie/rustify-ui/blob/main/docs/${path.basename(alias)}`;
  const guideProbe = Buffer.concat([originals.get(guide), Buffer.from(`\n\n${guideMarker}\n\n[Unpublished source probe](${path.basename(alias)})\n`)]);
  writeOwned(guide, guideProbe); current = await settled(current); assert.equal(current.ok, true); sameCompile(current, initial, count);
  const guideUrl = new URL("quickstart/", url).href; await page.goto(guideUrl); await page.evaluate(() => document.fonts.ready); assert((await page.locator("main").textContent()).includes(guideMarker)); assert.equal(await page.getByRole("link", { name: "Unpublished source probe", exact: true }).getAttribute("href"), repositoryHref);
  const goodGuide = await documentAt(guideUrl); assert(goodGuide.bytes.includes(Buffer.from(guideMarker))); assert(goodGuide.bytes.includes(Buffer.from(repositoryHref))); report.phases.push({ name: "selected guide actual source save and excluded link repository mapping", state: current, html_sha256: goodGuide.sha256, marker: guideMarker, repository_href: repositoryHref, no_new_cargo: true });
  await quiet("unselected Markdown, binary and validation body saves", () => { editExtra(loose, Buffer.from([0xfd])); editExtra(binary, Buffer.from([1, 0, 255])); editExtra(validation, "changed validation evidence\n"); }, current, initial, count);
  await quiet("Exists-only referenced file body is not decoded or watched", () => editExtra(targetA, Buffer.from([0xfc, 0xfb, 3])), current, initial, count);
  const bodyA = extras.get(targetA).bytes; unchanged(targetA, bodyA); fs.unlinkSync(targetA); extras.set(targetA, { absent: true }); current = await settled(current); assert.equal(current.ok, false); assert(current.diagnostics.some(diagnostic => diagnostic.rule === "broken-link" && diagnostic.file === "docs/quickstart.md")); sameCompile(current, initial, count); assert.deepEqual((await documentAt(guideUrl)).bytes, goodGuide.bytes);
  report.phases.push({ name: "Exists-only dependency deletion refuses candidate and preserves exact whole last-good document", state: current, html_sha256: goodGuide.sha256, no_new_cargo: true });
  assert(!fs.existsSync(targetA)); fs.writeFileSync(targetA, bodyA, { flag: "wx" }); extras.set(targetA, { bytes: bodyA }); current = await settled(current); assert.equal(current.ok, true); sameCompile(current, initial, count); report.phases.push({ name: "Exists-only dependency restoration recovers", state: current, no_new_cargo: true });
  const previousSequence = current.sequence; aliasTo(alias, path.basename(targetB)); current = await settled(current); assert.equal(current.ok, true); assert(current.sequence > previousSequence); sameCompile(current, initial, count); report.phases.push({ name: "existing symlink retarget observes identity without reading the new body", state: current, alias_target: path.basename(targetB), no_new_cargo: true });
  await quiet("new Exists-only real target body stays excluded", () => editExtra(targetB, Buffer.from([0xfb, 0xfa])), current, initial, count);
  const beforeRetarget = await documentAt(guideUrl); aliasTo(alias, outside); current = await settled(current); assert.equal(current.ok, false); assert(current.errors > 0); sameCompile(current, initial, count); assert.deepEqual((await documentAt(guideUrl)).bytes, beforeRetarget.bytes); report.phases.push({ name: "symlink retarget outside project rejects candidate and preserves whole last-good document", state: current, no_new_cargo: true });
  aliasTo(alias, path.basename(targetB)); current = await settled(current); assert.equal(current.ok, true); sameCompile(current, initial, count); report.phases.push({ name: "safe symlink retarget recovery", state: current, no_new_cargo: true });
  const servedDemo = await documentAt(demoUrl); assert.equal(withoutServedRevision(servedDemo.bytes), withoutServedRevision(originalDemo.bytes), "content-only edits must not mix a new category document with old compiled source/wasm"); report.category_document = { before_sha256: originalDemo.sha256, after_sha256: servedDemo.sha256, only_served_dev_revision_normalized: true }; assert.equal(demoWasmCount(), requestedWasm, "unaffected island page must not load a new wasm");
  report.retained_demo = await demoPage.evaluate(() => ({ same_root: document.querySelector("fieldset[data-rd-island-controls]").firstElementChild === window.__m5_original_scope, all_original_nodes: window.__m5_original_nodes.every(node => node.isConnected && window.__m5_original_scope.contains(node)), state: document.querySelector("[data-demo-state]").textContent })); assert.deepEqual(report.retained_demo, { same_root: true, all_original_nodes: true, state: "1" });
  const finalWasmResponse = await fetch(wasmUrl); assert.equal(finalWasmResponse.status, 200); assert.equal(sha(Buffer.from(await finalWasmResponse.arrayBuffer())), sha(wasmBytes));
  writeOwned(guide, originals.get(guide)); current = await settled(current); assert.equal(current.ok, true); sameCompile(current, initial, count);
  writeOwned(readme, originals.get(readme)); current = await settled(current); assert.equal(current.ok, true); sameCompile(current, initial, count); await page.goto(url); await page.evaluate(() => document.fonts.ready); assert(!(await page.locator(".rd-home-prose").textContent()).includes(readmeMarker));
  report.phases.push({ name: "actual root source bytes restored and served", state: current, no_new_cargo: true });
  assert.deepEqual(report.errors, []); assert(responses.every(response => response.status < 400)); report.responses = responses; report.browser_requests = browserRequests; report.passed = true;
} catch (error) { report.passed = false; report.failure = String(error); console.error(error); process.exitCode = 1; }
finally {
  if (context) await context.close(); if (browser) await browser.close(); if (stream) stream.destroy();
  if (child && child.exitCode === null && child.signalCode === null) { const exit = new Promise(resolve => child.once("exit", resolve)); process.kill(-child.pid, "SIGTERM"); await exit; }
  for (const [file, original] of originals) {
    try { unchanged(file, expected.get(file)); fs.writeFileSync(file, original); report.restoration.push({ file, restored: true, sha256: sha(fs.readFileSync(file)) }); }
    catch (error) { report.restoration.push({ file, restored: false, conflict: String(error) }); fs.writeFileSync(path.join(out, `${path.basename(file)}.original`), original); fs.copyFileSync(file, path.join(out, `${path.basename(file)}.concurrent`)); report.passed = false; process.exitCode = 1; }
  }
  for (const [file, value] of extras) {
    try { if (value.absent) assert(!fs.existsSync(file)); else { if (value.link) assert.equal(fs.readlinkSync(file), value.link); else unchanged(file, value.bytes); fs.unlinkSync(file); } report.restoration.push({ file, removed_own_probe: true }); }
    catch (error) { report.restoration.push({ file, removed_own_probe: false, conflict: String(error) }); report.passed = false; process.exitCode = 1; }
  }
  fs.rmSync(externalDir, { recursive: true }); fs.writeFileSync(path.join(out, "cli.log"), cliLog); fs.writeFileSync(path.join(out, "states.json"), JSON.stringify(states, null, 2) + "\n"); fs.writeFileSync(path.join(out, "report.json"), JSON.stringify(report, null, 2) + "\n"); console.log(JSON.stringify({ passed: report.passed, phases: report.phases.length, failure: report.failure, restoration: report.restoration.every(entry => entry.restored || entry.removed_own_probe) }));
}
