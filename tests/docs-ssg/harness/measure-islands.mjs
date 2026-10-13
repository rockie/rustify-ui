// Release proxy measurement, with cold HTTP/SW caches and no throttling.
// The endpoint requires real DOM activation/input handlers to change all 24
// catalog results, the splitter and all three counters. Boot alone cannot pass.
//
// node tests/docs-ssg/harness/measure-islands.mjs --origin http://127.0.0.1:4191 \
//   --page /docs/proxy/ --samples 5 --out report.json

import { chromium } from "@playwright/test";
import { writeFile } from "node:fs/promises";
import { arch, cpus, platform, release } from "node:os";
import { fileURLToPath } from "node:url";
import { gzipSync } from "node:zlib";
import { FROZEN_CHROMIUM } from "./measure-load.mjs";

function installMeter() {
  const meter = {
    timeOrigin: performance.timeOrigin,
    clockBefore: { epoch: Date.now(), monotonic: performance.now() },
    earlyDom: null, before: null, after: null, runtimeReady: null, hydrateStart: null,
    hydrateEnd: null, ready: null, inputStart: null, inputEnd: null,
    probes: [], wasm: [], mutations: [], shifts: [], violations: [],
    done: false, failure: null,
  };
  const originals = new Map();
  const rect = (node) => {
    const box = node.getBoundingClientRect();
    return { x: box.x, y: box.y, width: box.width, height: box.height };
  };
  const properties = (nodes) => [...nodes].map((node) => ({
    id: node.getAttribute("data-testid"),
    value: "value" in node ? node.value : null,
    checked: "checked" in node ? node.checked : null,
    disabled: "disabled" in node ? node.disabled : null,
    readOnly: "readOnly" in node ? node.readOnly : null,
    selected: node.getAttribute("aria-selected"),
    expanded: node.getAttribute("aria-expanded"),
    pressed: node.getAttribute("aria-pressed"),
    ariaChecked: node.getAttribute("aria-checked"),
    glyph: node.getAttribute("data-glyph"),
  }));
  function captureDom() {
    if (meter.earlyDom !== null || document.readyState === "loading") return;
    const start = performance.now();
    for (const block of document.querySelectorAll("[data-rustify-island]")) {
      const canvas = block.querySelector("fieldset[data-rd-island-controls]");
      const root = canvas?.firstElementChild;
      if (!root) continue;
      const nodes = [root];
      const walk = document.createTreeWalker(root, NodeFilter.SHOW_ALL);
      while (walk.nextNode()) nodes.push(walk.currentNode);
      const regions = [canvas, ...root.querySelectorAll("[data-category], [data-extra], [data-counter]")];
      const propertyNodes = [...root.querySelectorAll("[data-testid]")];
      originals.set(block.getAttribute("data-rustify-island"), {
        block, root, nodes, propertyNodes, properties: properties(propertyNodes),
        ids: [...root.querySelectorAll("[id]")].map((node) => [node, node.id]),
        regions,
      });
    }
    meter.earlyDom = { at: start, duration: performance.now() - start, islands: originals.size };
  }
  function captureBefore() {
    captureDom();
    const start = performance.now();
    for (const original of originals.values()) {
      original.boxes = original.regions.map((node) => ({ node, box: rect(node) }));
    }
    meter.before = { at: start, duration: performance.now() - start, islands: originals.size };
  }
  function captureAfter() {
    meter.after = [...originals].map(([key, original]) => ({
      key, nodes: original.nodes.length,
      retained: original.nodes.filter((node) => original.root.contains(node)).length,
      sameRoot: original.block.querySelector("fieldset[data-rd-island-controls]")?.firstElementChild === original.root,
      sameIds: original.ids.every(([node, id]) => node.id === id),
      sameProperties: JSON.stringify(properties(original.propertyNodes)) === JSON.stringify(original.properties),
      boxes: original.boxes.map(({ node, box }) => {
        const after = rect(node);
        return { region: node.getAttribute("data-category") || node.getAttribute("data-extra") || node.tagName,
          before: box, after, displacement: Math.max(...Object.keys(box).map((field) => Math.abs(box[field] - after[field]))) };
      }),
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
    for (const entry of list.getEntries()) meter.shifts.push({ at: entry.startTime, value: entry.value,
      recentInput: entry.hadRecentInput, sources: (entry.sources || []).map((source) => ({
        key: source.node?.closest?.("[data-rustify-island]")?.getAttribute("data-rustify-island") || null,
        before: source.previousRect.toJSON(), after: source.currentRect.toJSON(),
      })) });
  }).observe({ type: "layout-shift", buffered: true });
  document.addEventListener("securitypolicyviolation", (event) => {
    meter.violations.push({ directive: event.violatedDirective, blocked: event.blockedURI });
  });
  for (const name of ["compileStreaming", "compile", "instantiateStreaming", "instantiate"]) {
    const original = WebAssembly[name];
    if (typeof original !== "function") continue;
    WebAssembly[name] = function (...args) {
      const record = { name, start: performance.now(), end: null, failure: null };
      meter.wasm.push(record);
      return Reflect.apply(original, WebAssembly, args).then((result) => {
        record.end = performance.now(); return result;
      }, (error) => {
        record.end = performance.now(); record.failure = String(error); throw error;
      });
    };
  }
  const until = (check, timeout = 5000) => new Promise((resolve, reject) => {
    const start = performance.now();
    const poll = () => {
      try { if (check()) { resolve(); return; } } catch (error) { reject(error); return; }
      if (performance.now() - start >= timeout) reject(new Error("input result did not change within five seconds"));
      else requestAnimationFrame(poll);
    };
    poll();
  });
  async function probe(name, action, check) {
    const record = { name, start: performance.now(), end: null, failure: null };
    meter.probes.push(record);
    try { action(); await until(check); record.end = performance.now(); }
    catch (error) { record.failure = String(error); throw new Error(`${name}: ${error}`); }
  }
  async function runProbes() {
    meter.inputStart = performance.now();
    try {
      const catalog = document.querySelector("[data-catalog-proxy]");
      if (!catalog) throw new Error("measurement page has no CatalogProxy");
      const island = catalog.closest("[data-rustify-island]");
      const scope = catalog.getAttribute("data-catalog-proxy");
      const control = (part) => {
        const node = island.querySelector(`[data-testid="${CSS.escape(`${scope}-${part}`)}"]`);
        if (!node) throw new Error(`missing catalog control ${part}`);
        return node;
      };
      const text = (part) => control(part).textContent.trim();
      const key = (node, value) => {
        node.focus(); node.dispatchEvent(new KeyboardEvent("keydown", { key: value, bubbles: true, cancelable: true }));
      };
      const input = (node, value) => {
        node.focus();
        const type = node instanceof HTMLTextAreaElement ? HTMLTextAreaElement : HTMLInputElement;
        Object.getOwnPropertyDescriptor(type.prototype, "value").set.call(node, value);
        node.dispatchEvent(new InputEvent("input", { bubbles: true, inputType: "insertText", data: value }));
      };
      const categories = [...catalog.querySelectorAll("[data-category]")].map((node) => node.getAttribute("data-category"));
      if (categories.length !== 24 || new Set(categories).size !== 24) throw new Error(`expected 24 unique catalog categories, got ${categories.length}`);
      const simple = [
        ["button", () => control("button").click(), () => text("count") === "8"],
        ["label", () => control("label-toggle").click(), () => text("label") === "Updated field name"],
        ["link", () => control("link-toggle").click(), () => control("link").getAttribute("aria-disabled") === "true" && !control("link").hasAttribute("href")],
        ["icon", () => control("icon-toggle").click(), () => !island.querySelector(`[data-testid="${CSS.escape(`${scope}-icon`)}"]`)],
        ["text field", () => input(control("text"), "Edited text"), () => text("text-result") === "Edited text"],
        ["text area", () => input(control("area"), "Edited\nmultiline"), () => text("area-result") === "Edited\nmultiline"],
        ["checkbox", () => control("checkbox").click(), () => text("checkbox-result") === "false" && !control("checkbox").checked],
        ["radio", () => control("radio-south").click(), () => text("radio-result") === "south" && control("radio-south").checked],
        ["switch", () => control("switch").click(), () => control("switch").getAttribute("aria-checked") === "true"],
        ["slider", () => input(control("slider"), "45"), () => text("slider-result") === "45"],
        ["loading", () => control("loading-toggle").click(), () => !island.querySelector(`[data-testid="${CSS.escape(`${scope}-loading`)}"]`)],
        ["tabs", () => [...control("tabs").querySelectorAll('[role="tab"]')].find((node) => node.textContent === "Summary").click(),
          () => control("tabs").querySelector('[role="tabpanel"]').textContent === "Summary panel"],
        ["scroll area", () => control("scroll-toggle").click(), () => control("scroll").textContent.includes("Added scroll item")],
        ["data table", () => key(control("table-row-0").querySelector('[role="gridcell"]'), " "), () => control("table-row-0").getAttribute("aria-selected") === "true"],
        ["tree", () => control("tree-beta").click(), () => control("tree-beta").getAttribute("aria-selected") === "true"],
        ["toast", () => control("toast-toggle").click(), () => !island.querySelector(`[data-testid="${CSS.escape(`${scope}-static-toast`)}"]`)],
        ["number field", () => { input(control("number"), "6.75"); key(control("number"), "Enter"); }, () => text("number-result") === "6.75"],
        ["color field", () => { input(control("color"), "#abc123ff"); key(control("color"), "Enter"); }, () => getComputedStyle(control("color-swatch")).backgroundColor === "rgb(171, 193, 35)"],
        ["toggle group", () => control("toggle-bold").click(), () => control("toggle-bold").getAttribute("aria-pressed") === "true"],
      ];
      const counters = [...document.querySelectorAll("[data-counter]")];
      if (counters.length !== 3) throw new Error(`proxy needs three counter islands, got ${counters.length}`);
      const counterProbes = counters.map((counter, index) => {
        const output = counter.querySelector('[data-testid="counter-value"]');
        const expected = Number(output.textContent) + 1;
        return probe(`counter-${index}`, () => counter.querySelector('[data-testid="counter-increment"]').click(), () => Number(output.textContent) === expected);
      });
      await Promise.all([...simple.map(([name, action, check]) => probe(name, action, check)), ...counterProbes,
        probe("splitter", () => key(control("splitter").querySelector('[role="separator"]'), "ArrowRight"),
          () => getComputedStyle(control("splitter").querySelector('[role="separator"]')).left === "151px")]);
      await probe("progress", () => control("progress-change").click(), () => control("progress").getAttribute("aria-valuenow") === "60");
      await probe("select", () => control("select").click(), () => island.querySelector('[role="listbox"]'));
      [...island.querySelectorAll('[role="option"]')].find((node) => node.textContent.trim() === "Blue").click();
      await until(() => text("select") === "Blue" && !island.querySelector('[role="listbox"]'));
      await probe("tooltip", () => control("tooltip-trigger").focus(), () => island.querySelector('[role="tooltip"]'));
      control("menu-trigger").focus();
      await until(() => !island.querySelector('[role="tooltip"]') && document.activeElement === control("menu-trigger"));
      await probe("menu", () => control("menu-trigger").click(), () => island.querySelector('[role="menu"]'));
      [...island.querySelectorAll('[role="menuitem"]')].find((node) => node.textContent.trim() === "Copy").click();
      await until(() => !island.querySelector('[role="menu"]'));
      await probe("dialog", () => control("dialog-trigger").click(), () => island.querySelector('[role="dialog"]'));
      control("dialog-close").click();
      await until(() => !island.querySelector('[role="dialog"]'));
      const measured = new Set(meter.probes.map((record) => record.name));
      if (!categories.every((name) => measured.has(name))) throw new Error("a catalog category lacks an input result");
      meter.inputEnd = performance.now();
    } catch (error) { meter.failure = String(error); }
    finally { meter.done = true; }
  }
  let coordinator;
  Object.defineProperty(window, "__rustify_docs_islands", {
    configurable: true,
    get: () => coordinator,
    set(value) {
      coordinator = value;
      const bind = value.bind;
      value.bind = function (started) {
        meter.runtimeReady = performance.now();
        const hydrate = started.app.docs_hydrate;
        started.app.docs_hydrate = function (...args) {
          captureBefore();
          meter.hydrateStart = performance.now();
          try { return Reflect.apply(hydrate, this, args); }
          finally { meter.hydrateEnd = performance.now(); }
        };
        return Reflect.apply(bind, this, [started]);
      };
      const complete = value.complete;
      value.complete = function (...args) {
        const completed = Reflect.apply(complete, this, args);
        if (completed) {
          meter.ready = performance.now();
          captureAfter();
          queueMicrotask(runProbes);
        }
        return completed;
      };
      const fail = value.fail;
      value.fail = function (...args) {
        const failed = Reflect.apply(fail, this, args);
        if (failed) { meter.failure = String(args[0]); meter.done = true; }
        return failed;
      };
    },
  });
  window.__rustifyIslandsPerf = {
    done: () => meter.done,
    snapshot: () => ({ ...meter,
      fcp: performance.getEntriesByType("paint").find((entry) => entry.name === "first-contentful-paint")?.startTime ?? null,
      clockAfter: { epoch: Date.now(), monotonic: performance.now() },
      resources: performance.getEntriesByType("resource").map((entry) => ({ name: entry.name, initiator: entry.initiatorType,
        start: entry.startTime, requestStart: entry.requestStart, responseStart: entry.responseStart, responseEnd: entry.responseEnd,
        duration: entry.duration, transfer: entry.transferSize, encoded: entry.encodedBodySize, decoded: entry.decodedBodySize })),
      navigation: performance.getEntriesByType("navigation")[0]?.toJSON() ?? null,
    }),
  };
}

export async function sampleIslands(browser, url) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 }, colorScheme: "light", serviceWorkers: "block" });
  const page = await context.newPage();
  const errors = [];
  const assets = [];
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  page.on("requestfailed", (request) => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
  page.on("response", (response) => {
    if (["document", "stylesheet", "script"].includes(response.request().resourceType()) || /\.wasm(?:[?#]|$)/.test(response.url())) assets.push(response);
  });
  try {
    await context.addInitScript(installMeter);
    const cdp = await context.newCDPSession(page);
    await cdp.send("Network.enable");
    await cdp.send("Network.setCacheDisabled", { cacheDisabled: true });
    await cdp.send("Network.clearBrowserCache");
    await cdp.send("Network.emulateNetworkConditions", { offline: false, latency: 0, downloadThroughput: -1, uploadThroughput: -1 });
    await cdp.send("Emulation.setCPUThrottlingRate", { rate: 1 });
    const response = await page.goto(url, { waitUntil: "domcontentloaded" });
    let timeout = null;
    try { await page.waitForFunction(() => window.__rustifyIslandsPerf?.done(), null, { timeout: 35000 }); }
    catch (error) { timeout = String(error); }
    const raw = await page.evaluate(() => window.__rustifyIslandsPerf?.snapshot() ?? null);
    const sizes = await Promise.all(assets.map(async (asset) => {
      try {
        const body = await asset.body();
        return { url: asset.url(), bytes: body.length, gzip9: gzipSync(body, { level: 9 }).length,
          encoding: asset.headers()["content-encoding"] ?? null };
      } catch (error) { return { url: asset.url(), failure: String(error) }; }
    }));
    if (!raw) return { valid: false, failure: timeout ?? "measurement hook is missing", errors, sizes };
    const offset = (clock) => clock.epoch - raw.timeOrigin - clock.monotonic;
    const clockJump = Math.abs(offset(raw.clockAfter) - offset(raw.clockBefore));
    const hydrationShifts = raw.shifts.filter((entry) => entry.at >= raw.before?.at && entry.at <= raw.ready && entry.sources.some((source) => source.key));
    const identity = raw.after?.every((island) => island.sameRoot && island.sameIds && island.sameProperties && island.nodes === island.retained) ?? false;
    const displacement = Math.max(0, ...raw.after?.flatMap((island) => island.boxes.map((box) => box.displacement)) ?? []);
    const wasm = raw.resources.filter((resource) => /\.wasm(?:[?#]|$)/.test(resource.name));
    const streaming = raw.wasm.find((entry) => entry.name === "compileStreaming");
    const instances = raw.wasm.filter((entry) => entry.name === "instantiate");
    const instanceEnd = instances.length ? Math.max(...instances.map((entry) => entry.end)) : null;
    const phases = {
      wasm_download_ms: wasm.length === 1 ? wasm[0].responseEnd - wasm[0].start : null,
      streaming_compile_window_ms: streaming?.end === null || !streaming ? null : streaming.end - streaming.start,
      compile_tail_after_download_ms: streaming?.end === null || !streaming || wasm.length !== 1 ? null : Math.max(0, streaming.end - wasm[0].responseEnd),
      wasm_instantiate_ms: instances.length === 1 ? instances[0].end - instances[0].start : null,
      runtime_after_instantiate_ms: instanceEnd === null || raw.runtimeReady === null ? null : raw.runtimeReady - instanceEnd,
      hydrate_call_ms: raw.hydrateStart === null || raw.hydrateEnd === null ? null : raw.hydrateEnd - raw.hydrateStart,
      attachment_settle_ms: raw.ready === null || raw.hydrateEnd === null ? null : raw.ready - raw.hydrateEnd,
      input_results_ms: raw.inputEnd === null || raw.inputStart === null ? null : raw.inputEnd - raw.inputStart,
    };
    const valid = response?.status() === 200 && raw.failure === null && timeout === null && raw.fcp !== null && raw.inputEnd !== null
      && raw.before?.islands === 4 && identity && displacement === 0 && hydrationShifts.length === 0
      && errors.length === 0 && raw.violations.length === 0 && clockJump <= 5 && wasm.length === 1 && sizes.every((asset) => !asset.failure);
    return { valid, status: response?.status() ?? null, failure: timeout ?? raw.failure, errors, sizes, raw, phases,
      fcp_to_inputs_ms: raw.inputEnd === null || raw.fcp === null ? null : raw.inputEnd - raw.fcp,
      input_epoch_ms: raw.inputEnd === null ? null : raw.timeOrigin + raw.inputEnd,
      hydration_identity: identity, hydration_displacement: displacement, hydration_shifts: hydrationShifts, clock_jump_ms: clockJump };
  } catch (error) { return { valid: false, failure: String(error), errors }; }
  finally { await context.close(); }
}

const median = (values) => [...values].sort((left, right) => left - right)[Math.floor(values.length / 2)];

export async function measureIslands({ origin, pages, samples = 5 }) {
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_PERF_CHROMIUM || undefined });
  const version = browser.version();
  try {
    if (version !== FROZEN_CHROMIUM && !process.env.RUSTIFY_PERF_CHROMIUM) throw new Error(`Chromium ${version} is not frozen ${FROZEN_CHROMIUM}`);
    const results = [];
    for (const path of pages) {
      const runs = [];
      for (let index = 0; index < samples; index++) runs.push(await sampleIslands(browser, new URL(path, origin).href));
      const allValid = runs.every((run) => run.valid);
      const elapsed = allValid ? median(runs.map((run) => run.fcp_to_inputs_ms)) : null;
      results.push({ path, passed: allValid && elapsed <= 1000, fcp_to_inputs_median_ms: elapsed,
        budget_remaining_ms: elapsed === null ? null : 1000 - elapsed, samples: runs });
    }
    return { chromium: version, hardware: { platform: platform(), arch: arch(), os: release(), cpu: cpus()[0]?.model, cores: cpus().length },
      conditions: { cpu_slowdown: 1, network: "unthrottled", cache: "fresh context, HTTP cache disabled and cleared, service workers blocked",
        viewport: { width: 1280, height: 900 }, input: "native DOM activation plus InputEvent/KeyboardEvent handlers; application results required",
        samples, compression: "gzip level 9 measured from actual response bodies after input endpoint",
        streaming_compile: "overlaps download; raw WebAssembly and ResourceTiming windows retained" }, pages: results };
  } finally { await browser.close(); }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const options = { origin: null, pages: [], samples: 5, out: null };
  const argv = process.argv.slice(2);
  for (let index = 0; index < argv.length; index++) {
    const value = argv[++index];
    switch (argv[index - 1]) {
      case "--origin": options.origin = value; break;
      case "--page": options.pages.push(value); break;
      case "--samples": options.samples = Number(value); break;
      case "--out": options.out = value; break;
      default: throw new Error(`unknown option ${argv[index - 1]}`);
    }
  }
  if (!options.origin || !options.pages.length || !Number.isInteger(options.samples) || options.samples < 1) throw new Error("--origin, --page and a positive sample count are required");
  const report = await measureIslands(options);
  if (options.out) await writeFile(options.out, JSON.stringify(report, null, 2) + "\n");
  for (const page of report.pages) console.log(`${page.path}: ${page.passed ? "passed" : "failed"}, FCP to all inputs ${page.fcp_to_inputs_median_ms ?? "invalid sample"} ms, budget remaining ${page.budget_remaining_ms ?? "unavailable"} ms`);
  if (report.pages.some((page) => !page.passed)) process.exitCode = 1;
}
