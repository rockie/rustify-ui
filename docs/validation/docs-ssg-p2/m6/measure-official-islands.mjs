// Validation only; no frozen product or existing meter changes.
// Instrumentation, FCP/clock/identity/shift gates and cold CDP setup are copied
// from tests/docs-ssg/harness/measure-islands.mjs. Differences: one official
// island, CATALOG/registry-derived pages, and official.spec.ts input results.
// Like attached() in that spec, actual font readiness precedes input; any wait
// stays inside FCP→input and the pre-input layout window.
// Native keyboard defaults use Playwright; their IPC remains inside the budget.
// node docs/validation/docs-ssg-p2/m6/measure-official-islands.mjs \
//   --origin http://127.0.0.1:4199/rustify-ui/ --samples 5 --out a-official.json
// --page components/data-table/ can be repeated; omitted means all 24 pages.

import { chromium } from "@playwright/test";
import { createHash } from "node:crypto";
import { readFile, writeFile } from "node:fs/promises";
import { arch, cpus, platform, release } from "node:os";
import { fileURLToPath } from "node:url";
import { gzipSync } from "node:zlib";
import { FROZEN_CHROMIUM } from "../../../../tests/docs-ssg/harness/measure-load.mjs";

const root = new URL("../../../../", import.meta.url);
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

export async function discoverCategories() {
  const files = new Map();
  async function source(path) {
    if (!files.has(path)) files.set(path, await readFile(new URL(path, root)));
    return files.get(path).toString("utf8");
  }
  const catalog = await source("crates/rustify-components/src/catalog.rs");
  const registry = await source("site/src/lib.rs");
  const spec = await source("tests/docs-ssg/official.spec.ts");
  const names = new Map([...catalog.matchAll(/Self::(\w+) => "([^"]+)"/g)].map((m) => [m[1], m[2]]));
  const variants = [...catalog.slice(catalog.indexOf("pub const CATALOG:")).matchAll(/category: Category::(\w+)/g)].map((m) => m[1]);
  const pages = [...registry.matchAll(/\w+ \{ route: "([^"]+)", title: "([^"]+)"/g)].map((m) => ({ route: m[1], title: m[2] }));
  const demos = [...registry.matchAll(/(\w+Demo) => crate::demos::(\w+)::(\w+)/g)].map((m) => ({ demo: m[1], module: m[2], function: m[3] }));
  const specProbes = new Set([...spec.slice(spec.indexOf("async function exercise"), spec.indexOf('test("home')).matchAll(/case "([^"]+)":/g)].map((m) => m[1]));
  if (variants.length !== 24 || new Set(variants).size !== 24 || specProbes.size !== 24) throw new Error("CATALOG and official interaction spec must each contain 24 categories");
  const entries = [];
  for (const variant of variants) {
    const name = names.get(variant);
    const route = `components/${name?.replaceAll(" ", "-")}/`;
    const page = pages.find((page) => page.route === `/${route}`);
    const demo = demos.find((demo) => demo.demo === `${variant}Demo`);
    if (!name || !page || !demo || !specProbes.has(name)) throw new Error(`missing official page/demo/probe for CATALOG ${variant}`);
    const file = `site/src/demos/${demo.module}.rs`;
    if (!(await source(file)).includes(`pub fn ${demo.function}(`)) throw new Error(`missing actual source ${file}::${demo.function}`);
    entries.push({ name, route, title: page.title, demo: demo.demo, file, function: demo.function });
  }
  for (const path of ["tests/docs-ssg/harness/measure-islands.mjs", "crates/rustify-docs/assets/islands.js"]) await source(path);
  files.set("docs/validation/docs-ssg-p2/m6/measure-official-islands.mjs", await readFile(fileURLToPath(import.meta.url)));
  return { entries, source_sha256: Object.fromEntries([...files].map(([path, bytes]) => [path, sha256(bytes)])) };
}

function installMeter(entry) {
  const meter = { timeOrigin: performance.timeOrigin, clockBefore: { epoch: Date.now(), monotonic: performance.now() },
    earlyDom: null, before: null, after: null, runtimeReady: null, hydrateStart: null, hydrateEnd: null,
    ready: null, fontsReady: null, hydrationWindowEnd: null, inputStart: null, inputEnd: null,
    probes: [], wasm: [], mutations: [], shifts: [], violations: [], done: false, failure: null };
  const originals = new Map();
  const rect = (node) => { const r = node.getBoundingClientRect(); return { x: r.x, y: r.y, width: r.width, height: r.height }; };
  const properties = (nodes) => nodes.map((node) => ({ id: node.id, testId: node.getAttribute("data-testid"),
    value: "value" in node ? node.value : null, checked: "checked" in node ? node.checked : null,
    disabled: "disabled" in node ? node.disabled : null, readOnly: "readOnly" in node ? node.readOnly : null,
    selected: node.getAttribute("aria-selected"), expanded: node.getAttribute("aria-expanded"),
    pressed: node.getAttribute("aria-pressed"), ariaChecked: node.getAttribute("aria-checked"), glyph: node.getAttribute("data-glyph") }));
  function captureDom() {
    if (meter.earlyDom !== null || document.readyState === "loading") return;
    const start = performance.now();
    for (const block of document.querySelectorAll("[data-rustify-island]")) {
      const canvas = block.querySelector("fieldset[data-rd-island-controls]");
      const scope = canvas?.firstElementChild;
      if (!scope) continue;
      const nodes = [scope], walk = document.createTreeWalker(scope, NodeFilter.SHOW_ALL);
      while (walk.nextNode()) nodes.push(walk.currentNode);
      const propertyNodes = [...scope.querySelectorAll("input,textarea,select,button,[role],[data-testid],[data-glyph]")];
      originals.set(block.getAttribute("data-rustify-island"), { block, scope, nodes, propertyNodes,
        properties: properties(propertyNodes), ids: [...scope.querySelectorAll("[id]")].map((node) => [node, node.id]),
        regions: [canvas, scope, ...scope.querySelectorAll("[data-name],input,textarea")] });
    }
    meter.earlyDom = { at: start, duration: performance.now() - start, islands: originals.size };
  }
  function captureBefore() {
    captureDom(); const start = performance.now();
    for (const original of originals.values()) original.boxes = original.regions.map((node) => ({ node, box: rect(node) }));
    meter.before = { at: start, duration: performance.now() - start, islands: originals.size };
  }
  function captureAfter() {
    meter.after = [...originals].map(([key, original]) => ({ key, nodes: original.nodes.length,
      retained: original.nodes.filter((node) => original.scope.contains(node)).length,
      sameRoot: original.block.querySelector("fieldset[data-rd-island-controls]")?.firstElementChild === original.scope,
      sameIds: original.ids.every(([node, id]) => node.id === id),
      sameProperties: JSON.stringify(properties(original.propertyNodes)) === JSON.stringify(original.properties),
      propertiesBefore: original.properties, propertiesAfter: properties(original.propertyNodes),
      boxes: original.boxes.map(({ node, box }) => { const after = rect(node); return {
        region: node.getAttribute("data-name") || node.id || node.tagName, before: box, after,
        displacement: Math.max(...Object.keys(box).map((field) => Math.abs(box[field] - after[field]))) }; }),
    }));
  }
  document.addEventListener("readystatechange", captureDom);
  document.addEventListener("DOMContentLoaded", captureDom);
  new MutationObserver((records) => {
    const at = performance.now();
    for (const record of records) {
      const element = record.target.nodeType === Node.ELEMENT_NODE ? record.target : record.target.parentElement;
      const canvas = element?.closest?.("fieldset[data-rd-island-controls]");
      if (canvas) meter.mutations.push({ at, key: canvas.closest("[data-rustify-island]")?.getAttribute("data-rustify-island"),
        type: record.type, attribute: record.attributeName, added: record.addedNodes.length, removed: record.removedNodes.length });
    }
  }).observe(document, { subtree: true, childList: true, attributes: true, characterData: true });
  new PerformanceObserver((list) => {
    for (const e of list.getEntries()) meter.shifts.push({ at: e.startTime, value: e.value, recentInput: e.hadRecentInput,
      sources: (e.sources || []).map((s) => ({ key: s.node?.closest?.("[data-rustify-island]")?.getAttribute("data-rustify-island") || null,
        before: s.previousRect.toJSON(), after: s.currentRect.toJSON() })) });
  }).observe({ type: "layout-shift", buffered: true });
  document.addEventListener("securitypolicyviolation", (e) => meter.violations.push({ directive: e.violatedDirective, blocked: e.blockedURI }));
  for (const name of ["compileStreaming", "compile", "instantiateStreaming", "instantiate"]) {
    const original = WebAssembly[name]; if (typeof original !== "function") continue;
    WebAssembly[name] = function (...args) {
      const record = { name, start: performance.now(), end: null, failure: null }; meter.wasm.push(record);
      return Reflect.apply(original, WebAssembly, args).then((result) => { record.end = performance.now(); return result; },
        (error) => { record.end = performance.now(); record.failure = String(error); throw error; });
    };
  }
  const until = (check) => new Promise((resolve, reject) => {
    const start = performance.now();
    const poll = () => {
      try { if (check()) { resolve(); return; } } catch (error) { reject(error); return; }
      if (performance.now() - start >= 5000) reject(new Error("public input result did not change within five seconds"));
      else requestAnimationFrame(poll);
    }; poll();
  });
  async function runProbes() {
    try {
      const blocks = [...document.querySelectorAll("[data-rustify-island]")];
      if (blocks.length !== 1 || !blocks.every((block) => block.getAttribute("data-rd-island-state") === "ready")) throw new Error("all official islands must be ready before input");
      const block = blocks[0], field = block.querySelector("fieldset[data-rd-island-controls]");
      const metadata = JSON.parse(block.getAttribute("data-rustify-island-metadata"));
      meter.metadata = metadata;
      if (metadata.target.type !== "demo" || metadata.target.name !== entry.demo || metadata.environment.lang !== "en"
          || field.disabled || field.inert || field.firstElementChild?.getAttribute("data-rustify-mounted") !== metadata.seed
          || document.querySelector("[data-component-category]")?.getAttribute("data-component-category") !== entry.name
          || field.querySelector("[style]")) throw new Error("the real registered official demo did not attach");
      const ids = [...document.querySelectorAll("[id]")].map((node) => node.id);
      if (new Set(ids).size !== ids.length) throw new Error("duplicate document IDs");
      const one = (selector, predicate = () => true, within = field) => {
        const nodes = [...within.querySelectorAll(selector)].filter(predicate);
        if (nodes.length !== 1) throw new Error(`expected one ${selector}, got ${nodes.length}`); return nodes[0];
      };
      const text = (node) => node.textContent.trim();
      const accessible = (node) => {
        const named = node.getAttribute("aria-label") || (node.getAttribute("aria-labelledby") || "").split(/\s+/).filter(Boolean)
          .map((id) => document.getElementById(id)?.textContent.trim() || "").join(" ");
        if (named) return named;
        const copy = node.cloneNode(true);
        for (const hidden of copy.querySelectorAll('[aria-hidden="true"]')) hidden.remove();
        return text(copy);
      };
      const button = (label) => one("button", (node) => text(node) === label);
      const role = (name, label = null) => one(`[role="${name}"]`, (node) => label === null || accessible(node) === label);
      const labeled = (label) => { const node = document.getElementById(one("label", (node) => text(node) === label).htmlFor); if (!field.contains(node)) throw new Error(`missing labeled field ${label}`); return node; };
      const state = one("[data-demo-state]");
      const click = (node) => { node.focus(); node.click(); };
      const key = async (node, value) => { node.focus(); await window.__officialNativeKey(value); };
      const input = (node, value) => {
        node.focus(); const type = node instanceof HTMLTextAreaElement ? HTMLTextAreaElement : HTMLInputElement;
        Object.getOwnPropertyDescriptor(type.prototype, "value").set.call(node, value);
        node.dispatchEvent(new InputEvent("input", { bubbles: true, inputType: "insertText", data: value }));
      };
      const step = async (label, action, check) => {
        const record = { name: entry.name, label, start: performance.now(), end: null, failure: null, before: text(state) }; meter.probes.push(record);
        try { await action(); await until(check); record.end = performance.now(); record.after = text(state); }
        catch (error) { record.failure = String(error); throw new Error(`${entry.name}/${label}: ${error}`); }
      };
      if (!text(state)) throw new Error("official demo has no public initial state");
      if (meter.hydrationWindowEnd === null || performance.now() < meter.hydrationWindowEnd) throw new Error("hydration window is not closed");
      meter.inputStart = performance.now();
      switch (entry.name) {
        case "button": await step("increment", () => key(button("Increment"), "Enter"), () => text(state) === "1" && button("Unavailable").disabled); break;
        case "label": await step("label focus", () => one("label").click(), () => document.activeElement === labeled("Project name")); await step("edit", () => input(labeled("Project name"), "Official reader"), () => text(state) === "Official reader"); break;
        case "link": {
          const link = one('a[data-name="Link"]'), destination = link.getAttribute("href");
          await step("destination", () => click(link), () => location.hash === destination);
          await step("availability", () => button("Toggle availability").click(), () => text(state) === "Unavailable" && link.getAttribute("aria-disabled") === "true" && !link.hasAttribute("href")); break;
        }
        case "icon": await step("change", () => button("Change icon").click(), () => text(state) === "Close" && !!field.querySelector('[role="img"][aria-label="Not selected"]')); break;
        case "text field": await step("edit", () => input(labeled("Project name"), "A real edit — résumé"), () => text(state) === "A real edit — résumé"); break;
        case "text area": await step("edit", () => input(labeled("Notes"), "First line\nSecond line — résumé"), () => text(state) === "First line\nSecond line — résumé"); break;
        case "checkbox": await step("toggle", () => key(labeled("Include this item"), "Space"), () => text(state) === "Excluded" && !labeled("Include this item").checked); break;
        case "radio": await step("compact", () => key(one('[data-name="RadioItem"][data-value="compact"] input[type="radio"]'), "Space"), () => text(state) === "compact" && one('[data-name="RadioItem"][data-value="compact"] input[type="radio"]').checked && one('[data-name="RadioItem"][data-value="custom"] input[type="radio"]').disabled); break;
        case "switch": await step("enable", () => key(labeled("Enable notifications"), "Space"), () => text(state) === "Enabled" && labeled("Enable notifications").getAttribute("aria-checked") === "true"); break;
        case "select": {
          const trigger = role("combobox"); await step("open", () => click(trigger), () => !!field.querySelector('[role="listbox"]'));
          await step("choose", () => click(role("option", "JavaScript")), () => text(state) === "javascript" && !field.querySelector('[role="listbox"]') && document.activeElement === trigger);
          await step("reopen", () => click(trigger), () => !!field.querySelector('[role="listbox"]'));
          await step("escape", () => key(document.activeElement, "Escape"), () => !field.querySelector('[role="listbox"]') && document.activeElement === trigger); break;
        }
        case "slider": await step("step", () => key(labeled("Value"), "ArrowRight"), () => text(state) === "30"); break;
        case "progress": await step("advance", () => key(button("Advance"), "Enter"), () => text(state) === "50" && role("progressbar").getAttribute("aria-valuenow") === "50"); break;
        case "loading": await step("toggle", () => button("Toggle loading").click(), () => text(state) === "Ready"); break;
        case "tooltip": {
          const trigger = button("Hover or focus for help"); await step("focus", () => trigger.focus(), () => text(state) === "Open" && field.querySelector('[role="tooltip"]')?.id === trigger.getAttribute("aria-describedby"));
          await step("escape", () => key(trigger, "Escape"), () => text(state) === "Closed" && !field.querySelector('[role="tooltip"]') && document.activeElement === trigger); break;
        }
        case "menu": {
          const trigger = button("Open menu"); await step("open", () => click(trigger), () => !!field.querySelector('[role="menu"]'));
          await step("last enabled", () => key(document.activeElement, "End"), () => document.activeElement === role("menuitem", "Rename"));
          await step("activate", () => key(document.activeElement, "Enter"), () => text(state) === "rename" && !field.querySelector('[role="menu"]') && document.activeElement === trigger);
          await step("reopen", () => click(trigger), () => !!field.querySelector('[role="menu"]'));
          await step("escape", () => key(document.activeElement, "Escape"), () => !field.querySelector('[role="menu"]') && document.activeElement === trigger); break;
        }
        case "dialog": {
          const trigger = button("Open dialog"); await step("open", () => click(trigger), () => !!field.querySelector('[role="dialog"]'));
          await step("tab containment", () => key(document.activeElement, "Tab"), () => role("dialog").contains(document.activeElement));
          await step("escape", () => key(document.activeElement, "Escape"), () => !field.querySelector('[role="dialog"]') && document.activeElement === trigger);
          await step("reopen", () => click(trigger), () => !!field.querySelector('[role="dialog"]'));
          await step("confirm", () => click(button("Confirm")), () => text(state) === "Confirmed" && !field.querySelector('[role="dialog"]') && document.activeElement === trigger); break;
        }
        case "tabs": await step("next", () => key(role("tab", "Overview"), "ArrowRight"), () => document.activeElement === role("tab", "Details")); await step("activate", () => key(role("tab", "Details"), "Enter"), () => text(state) === "details" && !role("tabpanel", "Details").hidden); break;
        case "scroll area": await step("add", () => button("Add item").click(), () => text(state) === "9"); await step("native scroll", () => key(role("group", "Example items"), "End"), () => role("group", "Example items").scrollTop > 0); break;
        case "data table": {
          const cell = one('[role="gridcell"]', (node) => text(node) === "Button"); await step("select", () => key(cell, "Space"), () => text(state) === "Button");
          await step("activate", () => key(cell, "Enter"), () => [...field.querySelectorAll("p")].some((node) => text(node) === "Activated: Button")); break;
        }
        case "tree": {
          const parent = role("treeitem", "Components"); await step("child focus", () => key(parent, "ArrowDown"), () => document.activeElement === role("treeitem", "Button"));
          await step("select", () => key(document.activeElement, "Enter"), () => text(state) === "button");
          await step("parent", () => key(document.activeElement, "ArrowLeft"), () => document.activeElement === parent);
          await step("collapse", () => key(parent, "ArrowLeft"), () => parent.getAttribute("aria-expanded") === "false"); break;
        }
        case "toast": await step("show", () => button("Show notification").click(), () => text(state) === "The example action completed."); await step("dismiss", () => one('[data-name="ToastDismiss"]').click(), () => text(state) === "None"); break;
        case "number field": await step("step", () => key(labeled("Amount"), "ArrowUp"), () => text(state) === "13"); break;
        case "color field": await step("commit", async () => { input(labeled("Color"), "ff000080"); await key(labeled("Color"), "Enter"); }, () => text(state) === "#ff000080" && getComputedStyle(one('[data-testid="color-demo-swatch"]')).backgroundColor === "rgba(255, 0, 0, 0.5)"); break;
        case "toggle group": await step("press", () => key(button("Bold"), "Space"), () => text(state) === "bold, italic"); await step("roving focus", () => key(button("Bold"), "ArrowRight"), () => document.activeElement === button("Italic")); break;
        default: throw new Error(`CATALOG ${entry.name} has no real input probe`);
      }
      meter.inputEnd = performance.now();
    } catch (error) { meter.failure = String(error); }
    finally { meter.done = true; }
  }
  let coordinator;
  Object.defineProperty(window, "__rustify_docs_islands", { configurable: true, get: () => coordinator, set(value) {
    coordinator = value;
    const bind = value.bind;
    value.bind = function (started) {
      meter.runtimeReady = performance.now(); const hydrate = started.app.docs_hydrate;
      started.app.docs_hydrate = function (...args) { captureBefore(); meter.hydrateStart = performance.now();
        try { return Reflect.apply(hydrate, this, args); } finally { meter.hydrateEnd = performance.now(); } };
      return Reflect.apply(bind, this, [started]);
    };
    const complete = value.complete;
    value.complete = function (...args) {
      const completed = Reflect.apply(complete, this, args);
      // The product already waits two RAFs before complete; no extra settle timer.
      if (completed) {
        meter.ready = performance.now();
        queueMicrotask(async () => {
          try {
            await document.fonts.ready; meter.fontsReady = performance.now();
            captureAfter(); meter.hydrationWindowEnd = performance.now(); await runProbes();
          } catch (error) { meter.failure = String(error); meter.done = true; }
        });
      }
      return completed;
    };
    const fail = value.fail;
    value.fail = function (...args) { const failed = Reflect.apply(fail, this, args); if (failed) { meter.failure = String(args[0]); meter.done = true; } return failed; };
  } });
  window.__officialIslandsPerf = { done: () => meter.done, snapshot: () => ({ ...meter,
    fcp: performance.getEntriesByType("paint").find((e) => e.name === "first-contentful-paint")?.startTime ?? null,
    clockAfter: { epoch: Date.now(), monotonic: performance.now() },
    resources: performance.getEntriesByType("resource").map((e) => ({ name: e.name, initiator: e.initiatorType,
      start: e.startTime, requestStart: e.requestStart, responseStart: e.responseStart, responseEnd: e.responseEnd,
      duration: e.duration, transfer: e.transferSize, encoded: e.encodedBodySize, decoded: e.decodedBodySize })),
    navigation: performance.getEntriesByType("navigation")[0]?.toJSON() ?? null,
  }) };
}

export async function sampleOfficial(browser, url, entry) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 }, colorScheme: "light", serviceWorkers: "block" });
  const page = await context.newPage(), errors = [], requests = [], responses = [], assets = [];
  page.on("pageerror", (e) => errors.push(String(e)));
  page.on("console", (m) => { if (m.type() === "error") errors.push(m.text()); });
  page.on("request", (r) => { requests.push(r.url()); if (new URL(r.url()).origin !== new URL(url).origin || /makepad/i.test(r.url())) errors.push(`unexpected request ${r.url()}`); });
  page.on("requestfailed", (r) => errors.push(`${r.url()}: ${r.failure()?.errorText}`));
  page.on("response", (r) => {
    const type = r.request().resourceType(); responses.push({ url: r.url(), status: r.status(), type, mime: r.headers()["content-type"] || "" });
    if (r.status() >= 400) errors.push(`HTTP ${r.status()}: ${r.url()}`);
    assets.push(r);
  });
  try {
    await page.exposeBinding("__officialNativeKey", async (_, key) => { await page.keyboard.press(key); });
    await context.addInitScript(installMeter, entry);
    const cdp = await context.newCDPSession(page);
    await cdp.send("Network.enable");
    await cdp.send("Network.setCacheDisabled", { cacheDisabled: true });
    await cdp.send("Network.clearBrowserCache");
    await cdp.send("Network.emulateNetworkConditions", { offline: false, latency: 0, downloadThroughput: -1, uploadThroughput: -1 });
    await cdp.send("Emulation.setCPUThrottlingRate", { rate: 1 });
    const response = await page.goto(url, { waitUntil: "domcontentloaded" });
    let timeout = null;
    try { await page.waitForFunction(() => window.__officialIslandsPerf?.done(), null, { timeout: 35000 }); } catch (e) { timeout = String(e); }
    const raw = await page.evaluate(() => window.__officialIslandsPerf?.snapshot() ?? null);
    const policy = await page.evaluate(() => document.querySelector('meta[http-equiv="Content-Security-Policy"]')?.getAttribute("content") ?? null);
    if (!policy?.includes("script-src") || policy.includes("'unsafe-inline'") || policy.includes("frame-ancestors")) errors.push("missing or invalid official meta CSP");
    const sizes = await Promise.all(assets.map(async (asset) => {
      try { const body = await asset.body(); return { url: asset.url(), type: asset.request().resourceType(), bytes: body.length,
        gzip9: gzipSync(body, { level: 9 }).length, sha256: sha256(body), encoding: asset.headers()["content-encoding"] ?? null }; }
      catch (e) { return { url: asset.url(), failure: String(e) }; }
    }));
    if (!raw) return { valid: false, failure: timeout ?? "measurement hook missing", errors, requests, responses, sizes };
    const offset = (clock) => clock.epoch - raw.timeOrigin - clock.monotonic;
    const clockJump = Math.abs(offset(raw.clockAfter) - offset(raw.clockBefore));
    const hydrationShifts = raw.shifts.filter((e) => e.at >= raw.before?.at && e.at <= raw.hydrationWindowEnd && e.sources.some((s) => s.key));
    const identity = raw.after?.every((i) => i.sameRoot && i.sameIds && i.sameProperties && i.nodes === i.retained) ?? false;
    const displacement = Math.max(0, ...raw.after?.flatMap((i) => i.boxes.map((b) => b.displacement)) ?? []);
    const wasm = raw.resources.filter((r) => /\.wasm(?:[?#]|$)/.test(r.name));
    const streaming = raw.wasm.find((e) => e.name === "compileStreaming"), instances = raw.wasm.filter((e) => e.name === "instantiate");
    const instanceEnd = instances.length ? Math.max(...instances.map((e) => e.end)) : null;
    const phases = {
      wasm_download_ms: wasm.length === 1 ? wasm[0].responseEnd - wasm[0].start : null,
      streaming_compile_window_ms: streaming?.end === null || !streaming ? null : streaming.end - streaming.start,
      compile_tail_after_download_ms: streaming?.end === null || !streaming || wasm.length !== 1 ? null : Math.max(0, streaming.end - wasm[0].responseEnd),
      wasm_instantiate_ms: instances.length === 1 ? instances[0].end - instances[0].start : null,
      runtime_after_instantiate_ms: instanceEnd === null || raw.runtimeReady === null ? null : raw.runtimeReady - instanceEnd,
      hydrate_call_ms: raw.hydrateStart === null || raw.hydrateEnd === null ? null : raw.hydrateEnd - raw.hydrateStart,
      attachment_settle_ms: raw.ready === null || raw.hydrateEnd === null ? null : raw.ready - raw.hydrateEnd,
      font_settle_after_attachment_ms: raw.fontsReady === null || raw.ready === null ? null : raw.fontsReady - raw.ready,
      input_results_ms: raw.inputEnd === null || raw.inputStart === null ? null : raw.inputEnd - raw.inputStart,
    };
    const wasmResponses = responses.filter((r) => /\.wasm(?:[?#]|$)/.test(r.url));
    const valid = response?.status() === 200 && raw.failure === null && timeout === null && raw.fcp !== null && raw.inputEnd !== null
      && raw.earlyDom?.islands === 1 && raw.before?.islands === 1 && raw.after?.length === 1 && identity && displacement === 0 && hydrationShifts.length === 0
      && raw.inputEnd >= raw.inputStart && raw.inputStart >= raw.fcp && raw.inputStart >= raw.hydrationWindowEnd
      && raw.probes.length > 0 && raw.probes.every((p) => p.end !== null && p.failure === null)
      && errors.length === 0 && raw.violations.length === 0 && clockJump <= 5 && wasm.length === 1 && wasmResponses.length === 1
      && wasmResponses[0].status === 200 && wasmResponses[0].mime.split(";")[0] === "application/wasm" && sizes.every((a) => !a.failure);
    return { valid, status: response?.status() ?? null, failure: timeout ?? raw.failure, errors, requests, responses, sizes, raw, phases,
      csp: { meta: policy, header: response?.headers()["content-security-policy"] ?? null },
      fcp_to_inputs_ms: raw.inputEnd === null || raw.fcp === null ? null : raw.inputEnd - raw.fcp,
      hydration_identity: identity, hydration_displacement: displacement, hydration_shifts: hydrationShifts, clock_jump_ms: clockJump };
  } catch (e) {
    const raw = await page.evaluate(() => window.__officialIslandsPerf?.snapshot() ?? null).catch(() => null);
    return { valid: false, failure: String(e), errors, requests, responses, raw };
  }
  finally { await context.close(); }
}

export async function measureOfficial({ origin, pages = [], samples = 5 }) {
  if (samples !== 5) throw new Error("the formal A gate requires exactly five cold samples per selected page");
  const discovery = await discoverCategories();
  const base = new URL(origin); if (!base.pathname.endsWith("/")) base.pathname += "/";
  const chosen = (pages.length ? pages : discovery.entries.map((e) => e.route)).map((path) => {
    const url = new URL(path, base), entry = discovery.entries.find((e) => url.pathname.endsWith(`/${e.route}`));
    if (url.origin !== base.origin || !entry || url.search || url.hash) throw new Error(`not an official CATALOG island page: ${path}`);
    return { path, url: url.href, entry };
  });
  if (new Set(chosen.map((e) => e.url)).size !== chosen.length) throw new Error("duplicate selected pages");
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_PERF_CHROMIUM || undefined });
  try {
    const version = browser.version();
    if (version !== FROZEN_CHROMIUM && !process.env.RUSTIFY_PERF_CHROMIUM) throw new Error(`Chromium ${version} is not frozen ${FROZEN_CHROMIUM}`);
    const results = [];
    for (const selection of chosen) {
      const runs = []; for (let index = 0; index < samples; index++) runs.push(await sampleOfficial(browser, selection.url, selection.entry));
      const valid = runs.every((r) => r.valid), sorted = valid ? runs.map((r) => r.fcp_to_inputs_ms).sort((a, b) => a - b) : [];
      const elapsed = valid ? sorted[Math.floor(sorted.length / 2)] : null;
      results.push({ path: selection.path, url: selection.url, category: selection.entry.name, demo: selection.entry.demo,
        first_load_bytes: runs.map((r) => r.sizes?.every((a) => !a.failure) ? r.sizes.reduce((total, a) => total + a.bytes, 0) : null),
        first_load_gzip9_bytes: runs.map((r) => r.sizes?.every((a) => !a.failure) ? r.sizes.reduce((total, a) => total + a.gzip9, 0) : null),
        passed: valid && elapsed <= 1000, fcp_to_inputs_median_ms: elapsed, budget_remaining_ms: elapsed === null ? null : 1000 - elapsed, samples: runs });
    }
    return { schema_version: 1, reference: "A official 24-category site", chromium: version,
      hardware: { platform: platform(), arch: arch(), os: release(), cpu: cpus()[0]?.model, cores: cpus().length },
      conditions: { samples, viewport: { width: 1280, height: 900 }, cpu_slowdown: 1, network: "unthrottled",
        cache: "fresh context, HTTP cache disabled and cleared, service workers blocked", start: "first-contentful-paint",
        endpoint: "all islands ready and hydration window closed, then official public input results; keyboard IPC included",
        budget_ms: 1000, hydration_displacement_px: 0, compression: "gzip level 9 from actual responses after input endpoint",
        streaming_compile: "overlaps download; raw WebAssembly and ResourceTiming windows retained" },
      source_sha256: discovery.source_sha256, categories: discovery.entries, pages: results };
  } finally { await browser.close(); }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const options = { origin: null, pages: [], samples: 5, out: null }, argv = process.argv.slice(2);
  for (let i = 0; i < argv.length; i++) {
    const value = argv[++i]; if (!value) throw new Error(`missing value for ${argv[i - 1]}`);
    switch (argv[i - 1]) {
      case "--origin": options.origin = value; break; case "--page": options.pages.push(value); break;
      case "--samples": options.samples = Number(value); break; case "--out": options.out = value; break;
      default: throw new Error(`unknown option ${argv[i - 1]}`);
    }
  }
  if (!options.origin || !options.out) throw new Error("--origin and --out are required to retain every raw sample");
  const report = await measureOfficial(options);
  await writeFile(options.out, JSON.stringify(report, null, 2) + "\n");
  for (const page of report.pages) console.log(`${page.category}: ${page.passed ? "passed" : "failed"}, valid ${page.samples.filter((r) => r.valid).length}/5, FCP→inputs ${page.fcp_to_inputs_median_ms ?? "invalid"} ms, remaining ${page.budget_remaining_ms ?? "unavailable"} ms`);
  if (report.pages.some((page) => !page.passed)) process.exitCode = 1;
}
