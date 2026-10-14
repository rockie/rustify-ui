import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { tmpdir } from "node:os";
import { spawn } from "node:child_process";
import { get } from "node:http";
import { createHash } from "node:crypto";
import { chromium, expect } from "@playwright/test";

// One installed pair, one generated project and one live dev process. This
// automated author path does not stand in for first users or performance.
const repo = process.cwd();
const pair = JSON.parse(fs.readFileSync("target/docs-ssg/m6-installed-c.json", "utf8"));
const out = path.resolve(process.argv[2] || "docs/validation/docs-ssg-p2/m6/uat-author-continuous-r1");
fs.mkdirSync(out);
const work = fs.mkdtempSync(path.join(tmpdir(), "rustify-m6-continuous-author-"));
const site = path.join(work, "author-docs"), bin = pair.binary;
const env = { ...process.env, ...pair.transport_environment, PATH: `${pair.process_path_prepend}:${process.env.PATH}`, CARGO_NET_OFFLINE: "true" };
const timeout = 600_000, hardDeadline = Date.now() + 1_800_000;
const sha = bytes => createHash("sha256").update(bytes).digest("hex");
const wait = ms => new Promise(resolve => setTimeout(resolve, ms));
const owned = new Set(), states = [], audit = { requests: [], failures: [], expectedStreamClosures: [], responses: [], console: [], consoleEvents: [], expectedConsole404: [], pageErrors: [], csp: [], expected404: [], expected404Windows: [] };
const report = { scope: "automated continuous author path; real installed C/R, no original template edits, no human-user or performance claim", candidate: pair.candidate_sha, framework: pair.framework_revision, binary: bin, work, site, phases: [], audit };
let dev, url, stream, devLog = "", browser, context, stopping = false, lock, originalManifest, activeExpected404 = null;
const file = name => path.join(site, name), read = name => fs.readFileSync(file(name), "utf8"), write = (name, text) => fs.writeFileSync(file(name), text);

async function until(predicate, label, limit = timeout) {
  const deadline = Math.min(hardDeadline, Date.now() + limit);
  while (Date.now() < deadline) { const value = await predicate(); if (value) return value; await wait(20); }
  throw new Error(`timeout: ${label}; latest dev state ${JSON.stringify(states.at(-1))}`);
}
async function bounded(promise, label, limit = timeout) {
  let timer;
  try { return await Promise.race([promise, new Promise((_, reject) => timer = setTimeout(() => reject(new Error(`timeout: ${label}`)), Math.max(1, Math.min(limit, hardDeadline - Date.now()))))]); }
  finally { clearTimeout(timer); }
}
async function stop(child) {
  if (!child?.pid || child.exitCode !== null || child.signalCode !== null) return;
  const exited = new Promise(resolve => child.once("exit", resolve));
  process.kill(-child.pid, "SIGTERM");
  let timer;
  try { await Promise.race([exited, new Promise(resolve => timer = setTimeout(resolve, 5000))]); }
  finally { clearTimeout(timer); }
  if (child.exitCode === null && child.signalCode === null) { process.kill(-child.pid, "SIGKILL"); await exited; }
}
async function run(args, name, expected = 0) {
  if (stopping) throw new Error("continuous author path is stopping");
  const child = spawn(bin, args, { cwd: site, env: { ...env, RUSTIFY_DOCS_CARGO: compiler }, detached: true }); owned.add(child);
  let stdout = "", stderr = ""; child.stdout.on("data", bytes => stdout += bytes); child.stderr.on("data", bytes => stderr += bytes);
  try {
    const exit = await bounded(new Promise((resolve, reject) => { child.on("exit", resolve); child.on("error", reject); }), name);
    fs.writeFileSync(path.join(out, `${name}.stdout`), stdout); fs.writeFileSync(path.join(out, `${name}.stderr`), stderr);
    report.commands ||= []; report.commands.push({ command: [bin, ...args], cwd: site, exit_code: exit, stdout: `${name}.stdout`, stderr: `${name}.stderr` }); assert.equal(exit, expected, stderr || stdout);
    return stdout;
  } finally { await stop(child); }
}
function record(name, details) {
  const sourceDir = path.join(out, `step-${report.phases.length + 1}-sources`); fs.mkdirSync(sourceDir);
  for (const name of ["src/demos.rs", "src/lib.rs", "content/index.md", "rustify-docs.toml", "Cargo.toml", "Cargo.lock", "README.md"]) { const target = path.join(sourceDir, name); fs.mkdirSync(path.dirname(target), { recursive: true }); fs.copyFileSync(file(name), target); }
  report.phases.push({ name, source_directory: sourceDir, ...details }); fs.writeFileSync(path.join(out, "report.json"), JSON.stringify(report, null, 2) + "\n"); console.log(name);
}
function replaceOnce(text, expression, replacement) { assert.equal([...text.matchAll(new RegExp(expression.source, expression.flags.includes("g") ? expression.flags : expression.flags + "g"))].length, 1); return text.replace(expression, replacement); }
function appendRegistration(text, anchor, addition) { const match = text.match(anchor); assert(match); return text.replace(match[0], `${match[0]}\n${addition.trimEnd().split("\n").map(line => "        " + line).join("\n")}`); }
function treeHashes(root) { const result = {}; if (!fs.existsSync(root)) return result; const walk = dir => { for (const entry of fs.readdirSync(dir, { withFileTypes: true })) { const name = path.join(dir, entry.name); if (entry.isDirectory()) walk(name); else result[path.relative(root, name)] = sha(fs.readFileSync(name)); } }; walk(root); return result; }
function noLockChange() { assert.deepEqual(fs.readFileSync(file("Cargo.lock")), lock); }
function successful(state, previous) {
  assert.equal(state.ok, true); assert.equal(state.errors, 0); const compile = state.compile; assert(compile && compile.generation);
  assert.equal(compile.completed_at_ms, Math.max(compile.host.completed_at_ms, compile.wasm.completed_at_ms)); assert.equal(compile.host.profile, "docs-host"); assert.equal(compile.wasm.profile, "docs-wasm");
  assert.equal(compile.host.success, true); assert.equal(compile.wasm.success, true); for (const target of [compile.host, compile.wasm]) assert(target.flags.includes("--locked"));
  if (previous) assert(compile.completed_at_ms > previous.compile.completed_at_ms);
  noLockChange(); return state;
}
async function settled(previous = -1, ok = true) { return until(() => { const state = states.at(-1); return state && !state.building && state.revision > previous && state.ok === ok ? state : null; }, `committed revision after ${previous}, ok=${ok}`); }
async function ready(page, expected) {
  await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(expected.length, { timeout }); await expect(page.getByTestId("counter-value")).toHaveText(expected.map(String), { timeout });
  const metadata = await page.locator("[data-rustify-island]").evaluateAll(nodes => nodes.map(node => JSON.parse(node.getAttribute("data-rustify-island-metadata")))); assert.equal(new Set(metadata.map(item => item.seed)).size, metadata.length);
  for (const island of await page.locator("[data-rustify-island]").all()) { const data = JSON.parse(await island.getAttribute("data-rustify-island-metadata")); const field = island.locator("fieldset[data-rd-island-controls]"); await expect(field).toHaveJSProperty("inert", false); await expect(field).toHaveJSProperty("disabled", false); await expect(field.locator(":scope > [data-rustify-scope]")).toHaveAttribute("data-rustify-mounted", data.seed); await expect(field.locator("[style]")).toHaveCount(0); }
  assert(await page.evaluate(() => { const ids = [...document.querySelectorAll("[id]")].map(node => node.id); return ids.length === new Set(ids).size; })); await page.evaluate(() => document.fonts.ready);
}
async function sourceExact(page, text, expectedDemos) {
  const blocks = page.locator("[data-rustify-island]"); const found = [];
  for (const block of await blocks.all()) {
    const target = JSON.parse(await block.getAttribute("data-rustify-island-metadata")).target; if (target.type !== "demo") continue;
    const name = { CounterDemo: "counter_demo", DoubleCounterDemo: "double_counter_demo" }[target.name]; assert(name);
    const fn = text.match(new RegExp(`(?:\\/\\/\\/[^\\r\\n]*\\r?\\n)*pub fn ${name}\\([^)]*\\)[\\s\\S]*?^\\}(?:\\r?\\n|$)`, "m"))?.[0]; assert(fn);
    await expect(block.locator("[data-rd-source-path]")).toHaveText("src/demos.rs"); assert.deepEqual(await block.locator("template.rd-code-raw").evaluateAll(nodes => nodes.map(node => node.content.textContent)), [fn, text]); found.push(target.name);
  }
  assert.deepEqual(found, expectedDemos);
}
async function capture(page) {
  return page.evaluate(() => {
    const nodes = root => { const list = [root], walk = document.createTreeWalker(root, NodeFilter.SHOW_ALL); while (walk.nextNode()) list.push(walk.currentNode); return list; };
    window.__continuous_doc = "original complete last good document";
    window.__continuous_trees = [...document.querySelectorAll("[data-rustify-island]")].map(block => { const root = block.querySelector("fieldset").firstElementChild; const source = block.querySelector(".rd-demo-source"); return { root, nodes: nodes(root), ids: [...root.querySelectorAll("[id]")].map(node => [node, node.id]), inputs: [...root.querySelectorAll("input")].map(node => node.value), source, sourceNodes: source ? nodes(source) : [] }; });
    return window.__continuous_trees.map(tree => ({ nodes: tree.nodes.length, ids: tree.ids.map(([, id]) => id), sourceNodes: tree.sourceNodes.length }));
  });
}
async function retained(page) {
  const data = await page.evaluate(() => ({ same_document: window.__continuous_doc === "original complete last good document", trees: window.__continuous_trees.map(tree => ({ nodes: tree.nodes.length, retained: tree.nodes.filter(node => node.isConnected && tree.root.contains(node)).length, root: [...document.querySelectorAll("fieldset[data-rd-island-controls]")].some(field => field.firstElementChild === tree.root), ids: tree.ids.every(([node, id]) => node.id === id), source: !tree.source || (tree.source.isConnected && tree.sourceNodes.every(node => node.isConnected && tree.source.contains(node))), inputs: [...tree.root.querySelectorAll("input")].map(node => node.value), originalInputs: tree.inputs })) }));
  assert.equal(data.same_document, true); for (const tree of data.trees) { assert.equal(tree.retained, tree.nodes); assert(tree.root && tree.ids && tree.source); assert.deepEqual(tree.inputs, tree.originalInputs); } return data;
}
async function newSourceDocument(page, text) { await page.waitForFunction(source => window.__continuous_doc === undefined && [...document.querySelectorAll(".rd-demo-source template.rd-code-raw")].some(node => node.content.textContent === source), text, { timeout }); }
async function assetGeneration(page, state) {
  const module = await page.locator('script[type="module"][src*="/assets/islands/"]').getAttribute("src"); assert(module); const wasmUrl = new URL("app_bg.wasm", new URL(module, url)).href; const response = await fetch(wasmUrl, { signal: AbortSignal.timeout(30_000) }); assert.equal(response.status, 200); assert.equal(response.headers.get("content-type"), "application/wasm"); const bytes = Buffer.from(await response.arrayBuffer());
  const bindings = path.join(site, ".rustify-docs/bindings"); assert(fs.readdirSync(bindings).some(directory => { const file = path.join(bindings, directory, "app_bg.wasm"); return fs.existsSync(file) && sha(fs.readFileSync(file)) === sha(bytes); }));
  return { revision: state.revision, generation: state.compile.generation, completed_at_ms: state.compile.completed_at_ms, module_url: new URL(module, url).href, wasm_url: wasmUrl, actual_http_wasm_sha256: sha(bytes) };
}

// As in dev-components.spec.ts, observe the real compiler lifecycle. The
// wrapper lives outside the watched author project and changes no arguments.
const compiler = path.join(work, "cargo-observe.cjs"), calls = path.join(work, "compiler.jsonl");
fs.writeFileSync(compiler, `#!${process.execPath}\nconst fs=require("node:fs"),cp=require("node:child_process"),args=process.argv.slice(2); const record=value=>fs.appendFileSync(${JSON.stringify(calls)},JSON.stringify({args,at:Date.now(),...value})+"\\n"); record({phase:"start"}); const result=cp.spawnSync("mbx",args,{stdio:"inherit",env:process.env}); record({phase:"end",status:result.status}); if(result.error){console.error(result.error);process.exit(1)} process.exit(result.status===null?1:result.status);\n`, { mode: 0o755 });
fs.mkdirSync(site);
async function exercise() {
  assert.equal(sha(fs.readFileSync(bin)), pair.binary_sha256); const identity = await run(["--build-info"], "identity"); assert(identity.includes(`\nsource: ${pair.candidate_sha}\n`)); assert(identity.includes("\npanic: unwind\n"));
  await run(["new", site, "--template", "components"], "new"); const initialFiles = treeHashes(site); assert.equal(Object.keys(initialFiles).length, 11); lock = fs.readFileSync(file("Cargo.lock")); originalManifest = read("Cargo.toml"); const readme = read("README.md"); assert(readme.includes(pair.candidate_sha) && readme.includes(pair.framework_revision));
  const rust = [...readme.matchAll(/^```rust\n([\s\S]*?)^```/gm)].map(match => match[1]); const double = rust.find(block => block.startsWith("pub fn double_counter_demo")); const demoRegistration = rust.find(block => block.startsWith("demos {")); const pagesAndFooter = rust.find(block => block.startsWith("pub fn examples_page")); const registrations = rust.find(block => block.startsWith("pages {")); assert(double && demoRegistration && pagesAndFooter && registrations);
  const fence = [...readme.matchAll(/^````markdown\n([\s\S]*?)^````/gm)].map(match => match[1]).find(block => block.includes("name: DoubleCounterDemo")); assert(fence); const pageBinding = registrations.split("\nslots {")[0]; const footerBinding = "slots {" + registrations.split("\nslots {")[1];
  const nav = [...readme.matchAll(/^```toml\n([\s\S]*?)^```/gm)].map(match => match[1]).find(block => block.includes('routes = ["/examples/"]')); assert(nav);

  dev = spawn(bin, ["dev", "--port", "0"], { cwd: site, detached: true, env: { ...env, RUSTIFY_DOCS_CARGO: compiler } }); owned.add(dev); dev.stderr.on("data", bytes => devLog += bytes);
  url = await bounded(new Promise((resolve, reject) => { let text = ""; dev.stdout.on("data", bytes => { text += bytes; const match = /^dev: (\S+)\n/m.exec(text); if (match) resolve(match[1]); }); dev.on("error", reject); dev.on("exit", code => reject(new Error(`dev exited${code}: ${devLog}`))); }), "dev address"); report.dev = { pid: dev.pid, url, command: [bin, "dev", "--port", "0"] };
  stream = get(new URL(`/__rustify-docs/events?path=${encodeURIComponent(new URL(url).pathname)}&good=0`, url), response => { assert.equal(response.statusCode, 200); response.setEncoding("utf8"); let pending = ""; response.on("data", bytes => { pending += bytes; let end; while ((end = pending.indexOf("\n\n")) >= 0) { const message = pending.slice(0, end); pending = pending.slice(end + 2); if (message.startsWith("event: state\ndata: ")) states.push(JSON.parse(message.slice(19))); } }); response.on("error", error => { if (!stopping) audit.failures.push(`SSE ${error}`); }); }); stream.on("error", error => { if (!stopping) audit.failures.push(`SSE ${error}`); });
  let current = successful(await settled());
  browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined }); report.browser = { version: browser.version(), executable: process.env.RUSTIFY_CHROMIUM || chromium.executablePath() }; context = await browser.newContext({ viewport: { width: 1280, height: 900 } }); await context.tracing.start({ snapshots: true, screenshots: false });
  await context.exposeBinding("__continuous_csp", (_, value) => audit.csp.push(value)); await context.addInitScript(() => document.addEventListener("securitypolicyviolation", event => window.__continuous_csp(`${event.violatedDirective} ${event.blockedURI}`)));
  const page = await context.newPage(); page.setDefaultTimeout(30_000); page.on("pageerror", error => audit.pageErrors.push(String(error))); page.on("request", request => audit.requests.push(request.url())); page.on("response", response => audit.responses.push({ url: response.url(), status: response.status(), mime: response.headers()["content-type"] || "" }));
  page.on("requestfailed", request => { const value = `${request.url()} ${request.failure()?.errorText}`; if (new URL(request.url()).pathname === "/__rustify-docs/events" && request.failure()?.errorText === "net::ERR_ABORTED") audit.expectedStreamClosures.push(value); else audit.failures.push(value); });
  page.on("console", message => {
    const event = { type: message.type(), text: message.text(), url: message.location().url || "", time_ms: Date.now() }; audit.consoleEvents.push(event);
    if (event.type !== "error") return;
    if (activeExpected404 && event.text === "Failed to load resource: the server responded with a status of 404 (Not Found)" && (event.url === "" || event.url === activeExpected404.url)) audit.expectedConsole404.push(event);
    else audit.console.push(event);
  });
  let release; const held = new Promise(resolve => release = resolve); await page.route("**/assets/islands/**/app_bg.wasm", async route => { await held; await route.continue(); }); await page.goto(url, { waitUntil: "commit" }); await expect(page.locator('[data-rd-island-state="loading"]')).toHaveCount(2, { timeout }); const original = read("src/demos.rs"); const initialTrees = await capture(page); await sourceExact(page, original, ["CounterDemo"]); release(); await ready(page, [2, 7]); await retained(page); await page.unroute("**/assets/islands/**/app_bg.wasm");
  await page.getByTestId("counter-increment").first().click(); await expect(page.getByTestId("counter-value")).toHaveText(["3", "7"]); await page.getByTestId("counter-input").nth(1).fill("19"); await expect(page.getByTestId("counter-value")).toHaveText(["3", "19"]); record("unmodified template input and source", { state: current, original_trees: initialTrees, asset: await assetGeneration(page, current), source_sha256: sha(Buffer.from(original)) });

  await capture(page); let previous = current; write("src/demos.rs", original + "\n" + double); write("src/lib.rs", replaceOnce(read("src/lib.rs"), /^        demos \{[\s\S]*?^        \}/m, demoRegistration.trimEnd().split("\n").map(line => "        " + line).join("\n"))); write("content/index.md", read("content/index.md") + "\n" + fence);
  current = successful(await settled(previous.revision), previous); await newSourceDocument(page, read("src/demos.rs")); await ready(page, [2, 7, 14]); await sourceExact(page, read("src/demos.rs"), ["CounterDemo", "DoubleCounterDemo"]); await page.getByTestId("counter-increment").nth(2).click(); await expect(page.getByTestId("counter-value")).toHaveText(["2", "7", "15"]); record("README function, registration and Markdown fence", { state: current, asset: await assetGeneration(page, current), source_sha256: sha(Buffer.from(read("src/demos.rs"))) });

  previous = current; write("src/demos.rs", read("src/demos.rs") + "\n" + pagesAndFooter.split("\npub fn footer")[0] + "\n"); write("src/lib.rs", appendRegistration(read("src/lib.rs"), /^        demos \{[\s\S]*?^        \}/m, pageBinding)); write("rustify-docs.toml", read("rustify-docs.toml") + "\n" + nav);
  current = successful(await settled(previous.revision), previous); await newSourceDocument(page, read("src/demos.rs")); await ready(page, [2, 7, 14]); await page.goto(new URL("examples/", url).href); await expect(page.locator("h1")).toHaveText("Examples"); await ready(page, [7, 7]); await sourceExact(page, read("src/demos.rs"), ["CounterDemo", "CounterDemo"]); await page.getByTestId("counter-increment").first().click(); await expect(page.getByTestId("counter-value")).toHaveText(["8", "7"]); record("README registered page with independent demos", { state: current, asset: await assetGeneration(page, current), source_sha256: sha(Buffer.from(read("src/demos.rs"))) });

  previous = current; write("src/demos.rs", read("src/demos.rs") + "\npub fn footer" + pagesAndFooter.split("\npub fn footer")[1]); write("src/lib.rs", appendRegistration(read("src/lib.rs"), /^        pages \{[\s\S]*?^        \}/m, footerBinding)); current = successful(await settled(previous.revision), previous);
  await newSourceDocument(page, read("src/demos.rs")); await ready(page, [7, 7]); await sourceExact(page, read("src/demos.rs"), ["CounterDemo", "CounterDemo"]); await expect(page.getByRole("contentinfo").getByRole("link", { name: "Home", exact: true })).toHaveAttribute("href", "/");
  const missing = new URL("not-present/deep/", url).href; audit.expected404.push(missing); activeExpected404 = { url: missing, started_at_ms: Date.now() };
  try { assert.equal((await page.goto(missing)).status(), 404); }
  finally { activeExpected404.ended_at_ms = Date.now(); audit.expected404Windows.push(activeExpected404); activeExpected404 = null; }
  await expect(page.locator("h1")).toHaveText("Page not found"); await expect(page.getByRole("contentinfo").getByRole("link", { name: "Home", exact: true })).toHaveAttribute("href", "/"); await page.getByRole("contentinfo").getByRole("link", { name: "Home", exact: true }).click(); await ready(page, [2, 7, 14]); await sourceExact(page, read("src/demos.rs"), ["CounterDemo", "DoubleCounterDemo"]); record("README shared footer including deep404", { state: current, source_sha256: sha(Buffer.from(read("src/demos.rs"))) });
  const published = JSON.parse(await run(["build", "--format", "json"], "last-good-build")); assert.equal(published.summary.errors, 0); const dist = treeHashes(file("dist")); assert(Object.keys(dist).length > 0); fs.writeFileSync(path.join(out, "last-good-dist.json"), JSON.stringify(dist, null, 2) + "\n");

  await page.goto(new URL("examples/", url).href); await ready(page, [7, 7]); await page.getByTestId("counter-input").first().fill("30"); await page.getByTestId("counter-input").nth(1).fill("40"); await expect(page.getByTestId("counter-value")).toHaveText(["30", "40"]); const goodSource = read("src/demos.rs"); await sourceExact(page, goodSource, ["CounterDemo", "CounterDemo"]); await capture(page); const goodHttp = Buffer.from(await (await fetch(page.url())).arrayBuffer()); const bad = replaceOnce(goodSource, /RwSignal::new\(initial\)/, "RwSignal::new(initial + missing_author_step)"); const line = bad.slice(0, bad.indexOf("missing_author_step")).split("\n").length, column = bad.split("\n")[line - 1].indexOf("missing_author_step") + 1;
  previous = current; write("src/demos.rs", bad); const failed = await settled(previous.revision, false); assert.equal(failed.good, previous.good); assert.equal(failed.compile.host.success, false); assert.equal(failed.compile.wasm.success, false); const diagnostics = failed.diagnostics.filter(diagnostic => diagnostic.severity === "error" && diagnostic.rule === "rust-compile" && diagnostic.file === "src/demos.rs" && diagnostic.line === line && diagnostic.column === column); assert(diagnostics.length > 0);
  const panel = page.locator(".rd-dev-panel"); await expect(panel).toBeVisible(); const group = panel.locator(".rd-dev-list > section").filter({ has: page.getByRole("heading", { name: "src/demos.rs", exact: true }) }); await expect(group.locator('.rd-dev-item[data-severity="error"] .rd-dev-location')).toHaveText(Array(diagnostics.length).fill(`${line}:${column}`)); await expect(panel).toContainText("showing the last good version"); await retained(page); await sourceExact(page, goodSource, ["CounterDemo", "CounterDemo"]); assert.deepEqual(Buffer.from(await (await fetch(page.url())).arrayBuffer()), goodHttp);
  for (const command of ["check", "build"]) { const result = JSON.parse(await run([command, "--format", "json"], `failed-${command}`, 1)); assert(result.summary.errors > 0); assert(result.diagnostics.some(diagnostic => diagnostic.file === "src/demos.rs" && diagnostic.line === line && diagnostic.column === column)); assert.deepEqual(treeHashes(file("dist")), dist); await retained(page); await sourceExact(page, goodSource, ["CounterDemo", "CounterDemo"]); }
  await panel.getByRole("button", { name: "Hide problems", exact: true }).click(); await page.getByTestId("counter-increment").first().click(); await expect(page.getByTestId("counter-value")).toHaveText(["31", "40"]); record("bad Rust retains same original interactive/source document and whole dist", { state: failed, diagnostics, retained: await page.evaluate(() => window.__continuous_trees.map(tree => ({ root: tree.root.isConnected, nodes: tree.nodes.length }))), dist_files: Object.keys(dist).length });

  const repaired = replaceOnce(goodSource, /counter\(14, "Double counter"/, 'counter(21, "Double counter"'); write("src/demos.rs", repaired); current = successful(await settled(failed.revision), previous); await newSourceDocument(page, repaired); await ready(page, [7, 7]); await expect(page.locator(".rd-dev-panel")).toBeHidden(); await page.goto(url); await ready(page, [2, 7, 21]); await sourceExact(page, repaired, ["CounterDemo", "DoubleCounterDemo"]); await page.getByTestId("counter-increment").nth(2).click(); await expect(page.getByTestId("counter-value")).toHaveText(["2", "7", "22"]); record("repair publishes the matching complete new source and real wasm behavior", { state: current, asset: await assetGeneration(page, current), source_sha256: sha(Buffer.from(repaired)) });
  const finalCheck = JSON.parse(await run(["check", "--format", "json"], "final-check")); const finalBuild = JSON.parse(await run(["build", "--format", "json"], "final-build")); assert.equal(finalCheck.summary.errors, 0); assert.equal(finalBuild.summary.errors, 0); assert.notDeepEqual(treeHashes(file("dist")), dist); noLockChange(); assert.equal(read("README.md"), readme); assert.equal(read("Cargo.toml"), originalManifest);
  assert.equal(new Set(states.map(state => state.session)).size, 1); assert(states.every((state, index) => index === 0 || state.sequence > states[index - 1].sequence)); const lifecycle = fs.readFileSync(calls, "utf8").trim().split("\n").map(line => JSON.parse(line)); for (let index = 0; index < lifecycle.length; index += 2) { assert.equal(lifecycle[index].phase, "start"); assert.equal(lifecycle[index + 1].phase, "end"); assert.deepEqual(lifecycle[index].args, lifecycle[index + 1].args); if (index > 0) assert(lifecycle[index].at >= lifecycle[index - 1].at); }
  assert.deepEqual(audit.failures, []); assert.deepEqual(audit.console, []); assert.deepEqual(audit.pageErrors, []); assert.deepEqual(audit.csp, []); const httpErrors = audit.responses.filter(response => response.status >= 400); assert.equal(audit.expected404.length, 1); assert.equal(httpErrors.length, 1); assert.equal(httpErrors[0].status, 404); assert.equal(httpErrors[0].url, audit.expected404[0]); assert(!audit.requests.some(request => new URL(request).origin !== new URL(url).origin || /makepad/i.test(request))); record("final installed-C check/build and unchanged lock/README", { check: finalCheck.summary, build: finalBuild.summary, lock_sha256: sha(lock), final_dist: treeHashes(file("dist")) }); report.passed = true;
}
try { await bounded(exercise(), "whole continuous author path", 1_800_000); }
catch (error) { report.passed = false; report.failure = String(error); console.error(error); process.exitCode = 1; }
finally {
  stopping = true; stream?.destroy();
  const cleanupError = error => { report.cleanup_errors ||= []; report.cleanup_errors.push(String(error)); report.passed = false; process.exitCode = 1; };
  try { if (context) { await context.tracing.stop({ path: path.join(out, "browser-dom-trace.zip") }).catch(error => report.trace_error = String(error)); await context.close(); } }
  catch (error) { cleanupError(error); }
  finally {
    try { if (browser) await browser.close(); } catch (error) { cleanupError(error); }
    finally { for (const child of owned) { try { await stop(child); } catch (error) { cleanupError(error); } } }
  }
  fs.writeFileSync(path.join(out, "dev.log"), devLog); fs.writeFileSync(path.join(out, "states.json"), JSON.stringify(states, null, 2) + "\n"); if (fs.existsSync(calls)) fs.copyFileSync(calls, path.join(out, "compiler-lifecycle.jsonl")); report.all_own_processes_stopped = true; fs.writeFileSync(path.join(out, "report.json"), JSON.stringify(report, null, 2) + "\n"); console.log(JSON.stringify({ passed: report.passed, phases: report.phases.length, failure: report.failure, out }));
}
