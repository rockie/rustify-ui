// Test-only M2 docs-host/docs-wasm compile/render/load proxy. The original
// sampleIslands catalog/counter probes and FCP calculation remain unchanged.
// Start the native ignored compile_proxy test and the existing static host:
//   RUSTIFY_COMPILE_PROXY_WORK=<work> RUSTIFY_DOCS_CARGO=mbx \
//     mbx test --locked --profile docs-cli -p rustify-docs-cli --bin rustify-docs \
//     compile_proxy::compile_to_islands_proxy -- --ignored --exact --nocapture
//   node tests/docs-ssg/static-host.mjs <work>/dist <free-port> /docs/
//   node tests/docs-ssg/harness/measure-compile-islands.mjs \
//     --work <work> --origin http://127.0.0.1:<free-port> --out report.json

import { chromium } from "@playwright/test";
import { createHash } from "node:crypto";
import { readFile, writeFile, rename, mkdir } from "node:fs/promises";
import { arch, cpus, hostname, platform, release } from "node:os";
import { join, resolve } from "node:path";
import { performance as nodePerformance } from "node:perf_hooks";
import { fileURLToPath } from "node:url";
import { FROZEN_CHROMIUM } from "./measure-load.mjs";
import { sampleIslands } from "./measure-islands.mjs";

const sleep = (ms) => new Promise((done) => setTimeout(done, ms));
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
const median = (values) => [...values].sort((a, b) => a - b)[Math.floor(values.length / 2)];

async function atomicJson(path, value) {
  const temporary = `${path}.tmp`;
  await writeFile(temporary, JSON.stringify(value, null, 2) + "\n");
  await rename(temporary, path);
}

async function waitJson(path, timeout = 900000) {
  const start = nodePerformance.now();
  while (nodePerformance.now() - start < timeout) {
    try { return JSON.parse(await readFile(path, "utf8")); }
    catch (error) { if (error.code !== "ENOENT") throw error; }
    await sleep(10);
  }
  throw new Error(`native descriptor did not arrive: ${path}`);
}

function installCompileProof(expected) {
  const state = {
    timeOrigin: performance.timeOrigin,
    clockBefore: { epoch: Date.now(), monotonic: performance.now() },
    ssr: null, inputStart: null, inputEnd: null, paint: null,
    observedBefore: null, observedAfter: null, done: false, failure: null,
  };
  let actionStarted = null;
  let painting = false;
  const finish = (failure) => { state.failure = failure; state.done = true; };
  const tick = () => {
    if (state.done) return;
    const root = document.querySelector("[data-compile-proxy-version]");
    const output = root?.querySelector('[data-testid="compile-proxy-value"]');
    const button = root?.querySelector('[data-testid="compile-proxy-change"]');
    const marker = root?.getAttribute("data-compile-proxy-version");
    const value = Number(output?.textContent);
    if (state.ssr === null && marker === expected.marker && value === expected.initial) {
      state.ssr = { at: performance.now(), marker, value };
    }
    if (window.__rustifyIslandsPerf?.done()) {
      if (state.ssr === null || !root || !output || !button) {
        finish("the new generation's SSR marker/initial value was not present"); return;
      }
      if (actionStarted === null) {
        if (marker !== expected.marker || value !== expected.initial) {
          finish("hydration adopted a different generation's view state"); return;
        }
        state.observedBefore = value;
        actionStarted = state.inputStart = performance.now();
        button.click();
      } else if (marker === expected.marker && value === expected.after_input && !painting) {
        state.observedAfter = value;
        state.inputEnd = performance.now();
        painting = true;
        requestAnimationFrame(() => requestAnimationFrame(() => {
          state.paint = performance.now(); state.done = true;
        }));
      } else if (performance.now() - actionStarted > 5000) {
        finish("the actual wasm input handler did not produce this generation's new value"); return;
      }
    }
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
  window.__compileIslandsProof = {
    done: () => state.done,
    snapshot: () => ({ ...state, clockAfter: { epoch: Date.now(), monotonic: performance.now() } }),
  };
}

export async function sampleCompile(browser, descriptor, origin) {
  const declaredSizes = Object.values(descriptor.resources).map((resource) => resource.bytes);
  if (descriptor.page_bytes != null) declaredSizes.push(descriptor.page_bytes);
  if (!declaredSizes.length || declaredSizes.some((bytes) => !Number.isSafeInteger(bytes) || bytes < 0)) {
    throw new Error("inspector capture requires actual finite resource byte sizes");
  }
  // Inspector response-body retention is separate from the disabled HTTP/SW
  // cache. Keep the actual dev wasm and document available for both SHA and
  // the unchanged meter's gzip accounting, without fetching anything twice.
  const captureLimits = {
    maxResourceBufferSize: Math.max(...declaredSizes) * 2 + 1024 * 1024,
    maxTotalBufferSize: declaredSizes.reduce((sum, bytes) => sum + bytes, 0) * 2 + 4 * 1024 * 1024,
  };
  const wasmUrl = new URL(`${descriptor.base}${descriptor.wasm_path}`, origin).href;
  const wasmRequests = new Map();
  const wasmCapture = { url: wasmUrl, source: "same HTTP response via its own buffered CDP session",
    playwright_responses: 0, requests: [], body_reads: 0, body_bytes: null,
    captured_epoch_ms: null, after_input_paint: false, failure: null };
  let wasmBody;
  const capturedBody = (response) => {
    if (!wasmBody) wasmBody = (async () => {
      try {
        const page = response.frame().page();
        await page.waitForFunction(() => window.__rustifyIslandsPerf?.done()
          && window.__compileIslandsProof?.done(), null, { timeout: 45000 });
        const started = nodePerformance.now();
        let record;
        while (nodePerformance.now() - started < 5000) {
          if (wasmRequests.size > 1 || wasmCapture.playwright_responses > 1) throw new Error("duplicate HTTP wasm request/response; the body cannot be attributed to one generation");
          record = [...wasmRequests.values()][0];
          if (record?.failure) throw new Error(`the actual wasm request failed: ${record.failure}`);
          if (record?.finished && record.response) break;
          await sleep(10);
        }
        if (!record?.finished || !record.response || wasmCapture.playwright_responses !== 1) throw new Error("missing wasm responseReceived/loadingFinished/requestId in the capture session");
        if (record.response.status !== 200 || record.response.fromDiskCache || record.response.fromServiceWorker) throw new Error("wasm capture must be a successful actual uncached HTTP response");
        wasmCapture.body_reads++;
        const body = await record.send("Network.getResponseBody", { requestId: record.requestId });
        const bytes = Buffer.from(body.body, body.base64Encoded ? "base64" : "utf8");
        if (bytes.length !== descriptor.resources[descriptor.wasm_path]?.bytes) throw new Error("the actual CDP wasm body length differs from the generation descriptor");
        wasmCapture.body_bytes = bytes.length;
        wasmCapture.captured_epoch_ms = Date.now();
        wasmCapture.after_input_paint = true;
        return bytes;
      } catch (error) { wasmCapture.failure = String(error); throw error; }
    })();
    return wasmBody;
  };
  const responses = [];
  const pending = [];
  let proof = null;
  let proofFailure = null;
  const before = { epoch: Date.now(), monotonic: nodePerformance.now() };
  // sampleIslands owns its context and unchanged real 24-category input meter.
  // Its close hook waits for the additional source-specific input/paint proof.
  const observedBrowser = {
    async newContext(options) {
      const context = await browser.newContext(options);
      const newCDPSession = context.newCDPSession.bind(context);
      context.newCDPSession = async (...args) => {
        const cdp = await newCDPSession(...args);
        const send = cdp.send.bind(cdp);
        cdp.on("Network.requestWillBeSent", (event) => {
          if (event.request.url !== wasmUrl) return;
          if (wasmRequests.has(event.requestId)) {
            wasmCapture.failure = "duplicate or redirected wasm requestId";
            return;
          }
          const record = { requestId: event.requestId, url: event.request.url, send,
            response: null, finished: null, failure: null };
          wasmRequests.set(event.requestId, record);
          wasmCapture.requests.push({ requestId: event.requestId, url: event.request.url });
        });
        cdp.on("Network.responseReceived", (event) => {
          if (event.response.url !== wasmUrl) return;
          let record = wasmRequests.get(event.requestId);
          if (!record) {
            record = { requestId: event.requestId, url: event.response.url, send,
              response: null, finished: null, failure: null };
            wasmRequests.set(event.requestId, record);
            wasmCapture.requests.push({ requestId: event.requestId, url: event.response.url });
          }
          if (record.response) wasmCapture.failure = "duplicate wasm responseReceived event";
          record.response = event.response;
        });
        cdp.on("Network.loadingFinished", (event) => {
          const record = wasmRequests.get(event.requestId);
          if (record) record.finished = { timestamp: event.timestamp, encodedDataLength: event.encodedDataLength };
        });
        cdp.on("Network.loadingFailed", (event) => {
          const record = wasmRequests.get(event.requestId);
          if (record) record.failure = event.errorText;
        });
        cdp.send = (method, params) => send(method, method === "Network.enable"
          ? { ...params, ...captureLimits } : params);
        return cdp;
      };
      await context.addInitScript(installCompileProof, descriptor);
      // Context responses are emitted before page responses. Patch only the
      // descriptor's wasm object before either SHA or original sizes/gzip
      // listener sees it; Playwright's primary session keeps its own buffers.
      context.on("response", (response) => {
        if (response.url() !== wasmUrl) return;
        wasmCapture.playwright_responses++;
        response.body = () => capturedBody(response);
      });
      context.on("response", (response) => {
        const kind = response.request().resourceType();
        if (!["document", "script", "stylesheet"].includes(kind) && !/\.wasm(?:[?#]|$)/.test(response.url())) return;
        pending.push(response.body().then((body) => responses.push({
          url: response.url(), kind, status: response.status(), bytes: body.length, sha256: sha256(body),
        }), (error) => responses.push({ url: response.url(), kind, failure: String(error) })));
      });
      const close = context.close.bind(context);
      context.close = async () => {
        try {
          const page = context.pages()[0];
          await page.waitForFunction(() => window.__compileIslandsProof?.done(), null, { timeout: 8000 });
          proof = await page.evaluate(() => window.__compileIslandsProof.snapshot());
        } catch (error) { proofFailure = String(error); }
        await Promise.allSettled(pending);
        await close();
      };
      return context;
    },
  };
  const url = new URL(descriptor.page, origin);
  url.searchParams.set("compile-proxy", descriptor.generation);
  const measured = await sampleIslands(observedBrowser, url.href);
  const after = { epoch: Date.now(), monotonic: nodePerformance.now() };
  const nodeClockJump = Math.abs((after.epoch - before.epoch) - (after.monotonic - before.monotonic));
  const browserClock = proof ? {
    before_offset_ms: proof.clockBefore.epoch - proof.timeOrigin - proof.clockBefore.monotonic,
    after_offset_ms: proof.clockAfter.epoch - proof.timeOrigin - proof.clockAfter.monotonic,
  } : null;
  const clockValid = browserClock !== null && Math.abs(browserClock.before_offset_ms) <= 5
    && Math.abs(browserClock.after_offset_ms) <= 5
    && Math.abs(browserClock.after_offset_ms - browserClock.before_offset_ms) <= 5
    && nodeClockJump <= 5 && Math.abs(descriptor.native_clock.wall_monotonic_difference_ms) <= 5;
  const document = responses.find((response) => response.kind === "document" && response.status === 200);
  const groupResponses = responses.filter((response) => new URL(response.url).pathname.startsWith(`${descriptor.base}assets/islands/`));
  const checkedResources = groupResponses.map((response) => {
    const path = decodeURIComponent(new URL(response.url).pathname).slice(descriptor.base.length);
    return { ...response, path, expected_sha256: descriptor.resources[path]?.sha256,
      matched: !response.failure && descriptor.resources[path]?.sha256 === response.sha256 };
  });
  const sameGeneration = document?.sha256 === descriptor.page_sha256
    && checkedResources.length > 0 && checkedResources.every((response) => response.matched)
    && checkedResources.some((response) => response.path === descriptor.wasm_path)
    && wasmRequests.size === 1 && wasmCapture.playwright_responses === 1
    && wasmCapture.body_reads === 1 && wasmCapture.failure === null;
  const end = proof?.paint === null || !proof || measured.raw?.inputEnd == null
    ? null : proof.timeOrigin + Math.max(proof.paint, measured.raw.inputEnd);
  const elapsed = end === null ? null : end - descriptor.compile_complete_epoch_ms;
  const valid = descriptor.success && measured.valid && proof?.failure === null && proofFailure === null
    && proof?.ssr?.marker === descriptor.marker && proof?.observedAfter === descriptor.after_input
    && clockValid && sameGeneration && elapsed !== null && elapsed >= 0;
  return { valid, within_budget: valid && elapsed <= 2000, compile_to_paint_input_ms: elapsed,
    paint_input_epoch_ms: end, descriptor_seen_epoch_ms: before.epoch,
    same_generation: sameGeneration, clock_valid: clockValid, clock: { browser: browserClock, node_jump_ms: nodeClockJump, before, after },
    inspector_capture_limits: captureLimits,
    wasm_capture: wasmCapture,
    proof, proof_failure: proofFailure, document, resources: checkedResources, islands: measured };
}

export async function measureCompileIslands({ work, origin, out }) {
  work = resolve(work);
  out = resolve(out || join(work, "browser-results.json"));
  const url = new URL(origin);
  if (!["localhost", "127.0.0.1", "[::1]"].includes(url.hostname)) throw new Error("compile epoch calibration requires a server on this same machine");
  const session = await waitJson(join(work, "session.json"));
  const report = { schema_version: 1, session: session.session, native_session: session,
    scope: "M2 docs-host/docs-wasm compile/render/load proxy; unchanged catalog input/FCP probes; not the M4 dev endpoint or A/B largest-page gate",
    hardware: { hostname: hostname(), platform: platform(), arch: arch(), os: release(), cpu: cpus()[0]?.model, cores: cpus().length },
    samples: [], failures: [] };
  let browser;
  let ready = false;
  try {
    if (session.hostname !== hostname() || session.os !== "macos" || platform() !== "darwin"
      || session.arch !== "aarch64" || arch() !== "arm64") throw new Error("native and Chromium sampling must use the same Apple Silicon macOS machine");
    if (session.samples !== 5 || JSON.stringify(session.profiles) !== JSON.stringify(["docs-host", "docs-wasm"])) throw new Error("the compile proxy needs five samples in the actual dev compiler profiles");
    if (session.cli_build?.debug_assertions !== false || session.cli_build?.panic_strategy !== "unwind") throw new Error("run the actual CLI test with --profile docs-cli; an unoptimized test binary is not the CLI performance baseline");
    browser = await chromium.launch({ executablePath: process.env.RUSTIFY_PERF_CHROMIUM || undefined });
    report.chromium = browser.version();
    if (report.chromium !== FROZEN_CHROMIUM && !process.env.RUSTIFY_PERF_CHROMIUM) throw new Error(`Chromium ${report.chromium} is not frozen ${FROZEN_CHROMIUM}`);
    await mkdir(join(work, "acks"), { recursive: true });
    await atomicJson(join(work, "browser-ready.json"), { session: session.session, ready: true, node_epoch_ms: Date.now() });
    ready = true;
    for (let index = 0; index <= session.samples; index++) {
      const descriptor = await waitJson(join(work, "descriptors", `sample-${index}.json`));
      const seen = Date.now();
      let result;
      try {
        if (descriptor.session !== session.session || descriptor.sample !== index) throw new Error("a descriptor belongs to another generation/run");
        if (!descriptor.success) throw new Error("the native compile/render/publish failed; the descriptor and all diagnostics are retained");
        if (descriptor.host.profile !== "docs-host" || descriptor.wasm.profile !== "docs-wasm"
          || !descriptor.host.success || !descriptor.wasm.success
          || descriptor.compile_complete_epoch_ms !== Math.max(descriptor.host.completed_at_ms, descriptor.wasm.completed_at_ms)) throw new Error("the actual successful dev compiler epochs do not match the descriptor");
        if (!descriptor.compile_attempts?.length) throw new Error("actual cold/warm/bounded-retry compile history is missing");
        result = await sampleCompile(browser, descriptor, origin);
      } catch (error) { result = { valid: false, within_budget: false, failure: String(error) }; }
      result.descriptor_observed_epoch_ms = seen;
      const sample = { native: descriptor, browser: result };
      report.samples.push(sample);
      if (!result.valid) report.failures.push({ sample: index, failure: result.failure || result.proof_failure || "invalid sample" });
      await atomicJson(out, report);
      await atomicJson(join(work, "acks", `sample-${index}.json`), { session: session.session, sample: index, ...result });
      console.log(`compile/${index}: ${result.valid ? "valid" : "invalid"}, dev compiler end to paint+input ${result.compile_to_paint_input_ms ?? "unavailable"} ms`);
    }
    const incremental = report.samples.filter((sample) => sample.native.kind === "incremental");
    const valid = incremental.length === 5 && incremental.every((sample) => sample.browser.valid);
    const elapsed = valid ? median(incremental.map((sample) => sample.browser.compile_to_paint_input_ms)) : null;
    report.compile_to_paint_input_median_ms = elapsed;
    report.over_budget_samples = incremental.filter((sample) => !sample.browser.within_budget).map((sample) => sample.native.sample);
    report.budget_remaining_ms = elapsed === null ? null : 2000 - elapsed;
    report.passed = report.failures.length === 0 && valid && elapsed <= 2000;
    report.conditions = { start: "max actual successful docs-host/docs-wasm completed_at_ms; includes bindgen/helper/CSS generation/publication/descriptor transport/navigation/loading/hydration and real input/paint",
      endpoint: "original 24-category/three-counter input completion plus source-specific real input and two animation frames",
      samples: 5, warmup: "one cold build/sample retained separately", cache: "original sampleIslands fresh HTTP/SW contexts; actual Cargo fresh/compiled counts and all retry history retained",
      clock: "same-machine hostname/OS/arch; SystemTime, browser timeOrigin and Node wall/monotonic checks; clock jumps invalidate samples", cpu_slowdown: 1, network: "unthrottled" };
    report.conditions.inspector_capture = "per-sample Network.enable body buffers derived from actual declared resource bytes; HTTP cache remains disabled/cleared, SW blocked, FCP and network/CPU settings unchanged; no second fetch";
  } catch (error) {
    report.passed = false;
    report.failures.push({ phase: "harness", failure: String(error) });
    if (!ready) await atomicJson(join(work, "browser-ready.json"), { session: session.session, ready: false, failure: String(error) });
  } finally {
    if (browser) await browser.close();
    await atomicJson(out, report);
  }
  return report;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const options = {};
  const args = process.argv.slice(2);
  for (let index = 0; index < args.length; index += 2) {
    const key = args[index];
    if (!["--work", "--origin", "--out"].includes(key) || !args[index + 1]) throw new Error(`unknown or incomplete option ${key}`);
    options[key.slice(2)] = args[index + 1];
  }
  if (!options.work || !options.origin) throw new Error("--work and --origin are required");
  const report = await measureCompileIslands(options);
  console.log(`compile proxy: ${report.passed ? "passed" : "failed"}, median ${report.compile_to_paint_input_median_ms ?? "invalid"} ms`);
  if (!report.passed) process.exitCode = 1;
}
