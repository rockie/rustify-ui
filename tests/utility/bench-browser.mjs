import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { createHash } from "node:crypto";
import { dirname, resolve } from "node:path";
import { chromium } from "@playwright/test";

const output = process.argv.includes("--out")
  ? process.argv[process.argv.indexOf("--out") + 1]
  : "target/utility-browser-benchmark.json";
const children = [];
let browser;

function server(args) {
  const child = spawn("target/debug/xtask", [...args, "--port", "0"], { stdio: ["ignore", "pipe", "pipe"] });
  children.push(child);
  return new Promise((resolveUrl, reject) => {
    let errors = "";
    const timer = setTimeout(() => reject(new Error("Preview server startup deadline")), 10000);
    child.stderr.on("data", chunk => { errors += chunk; });
    child.stdout.on("data", chunk => {
      const match = String(chunk).match(/http:\/\/\S+/);
      if (match) { clearTimeout(timer); resolveUrl(match[0]); }
    });
    child.once("exit", code => { clearTimeout(timer); reject(new Error(`Preview exited ${code}: ${errors}`)); });
  });
}

function quantiles(samples) {
  assert.ok(samples.length > 0);
  const ordered = [...samples].sort((a, b) => a - b);
  const percentile = p => ordered[Math.ceil(p * ordered.length) - 1];
  return { samples: samples.length, p50Ms: percentile(0.5), p95Ms: percentile(0.95), valuesMs: samples };
}

try {
  const probeUrl = await server(["utility-baseline", "--serve"]);
  const fusionUrl = await server(["serve", "--example", "fusion-basic", "--release"]);
  browser = await chromium.launch({ channel: "chrome", headless: false });
  const page = await browser.newPage();
  const errors = [];
  page.on("pageerror", error => errors.push(error.message));
  page.on("requestfailed", request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
  await page.goto(probeUrl);
  const style = await page.evaluate(async () => {
    const facade = await import("./extractor/style.js");
    await facade.default({ module_or_path: new URL("./extractor/style_bg.wasm", location.href) });
    const source = "w-[137px] hover:bg-[#123456] ".repeat(2000);
    const bytes = new TextEncoder().encode(source).length;
    const scan = () => JSON.parse(facade.scan_content(source, "html"));
    for (let i = 0; i < 10; i++) scan();
    const scanner = [];
    for (let i = 0; i < 100; i++) {
      const start = performance.now();
      const result = scan();
      if (result.candidates.join(" ") !== "hover:bg-[#123456] w-[137px]") throw new Error("Scanner benchmark output differs");
      scanner.push(performance.now() - start);
    }
    const css = await fetch("./official.css").then(response => response.text());
    const original = [...document.adoptedStyleSheets];
    const staging = [], installation = [];
    for (let i = 0; i < 100; i++) {
      const sheet = new CSSStyleSheet();
      const start = performance.now();
      sheet.replaceSync(css);
      staging.push(performance.now() - start);
      const install = performance.now();
      document.adoptedStyleSheets = [...original, sheet];
      installation.push(performance.now() - install);
      document.adoptedStyleSheets = original;
    }
    return { scanner, bytes, staging, installation, userAgent: navigator.userAgent,
      clockResolutionNote: "performance.now without cross-origin isolation; zero sub-resolution samples retained",
      scannerTiming: "full WASM facade call including JS/WASM copies, extraction, JSON serialization and JSON parse",
      cssomTiming: "CSSStyleSheet.replaceSync and adoptedStyleSheets setter separately; no forced style/layout or GPU draw included" };
  });
  await page.goto(fusionUrl);
  await page.waitForFunction(() => window.__fusion_basic?.b0_state().region === "ready", null, { timeout: 120000 });
  const gpu = await page.evaluate(async () => {
    const api = window.__fusion_basic;
    const canvas = document.querySelector('[data-testid="b0-gpu"]');
    const gl = canvas.getContext("webgl2");
    const info = gl.getExtension("WEBGL_debug_renderer_info");
    const renderer = info ? gl.getParameter(info.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER);
    if (/swiftshader|llvmpipe|software/i.test(renderer)) throw new Error(`Real GPU required: ${renderer}`);
    const before = api.stats();
    const button = document.querySelector('[data-testid="b0-bump"]');
    let seen = before.frames, previous = null;
    const samples = [];
    for (let i = 0; i < 150; i++) {
      const now = await new Promise(resolveFrame => requestAnimationFrame(resolveFrame));
      const frame = api.stats().frames;
      if (i >= 30 && frame > seen) {
        if (previous !== null) samples.push(now - previous);
        previous = now;
      }
      seen = frame;
      button.click();
    }
    return { samples, renderer, before, after: api.stats(), region: api.b0_state().region,
      measurement: "intervals between rAF observations with increased actual BeginRenderCanvas presentation counter; 30-frame warmup, 120 measured drives" };
  });
  assert.equal(gpu.region, "ready");
  assert.equal(gpu.after.errors, 0);
  assert.ok(gpu.after.frames > gpu.before.frames);
  assert.deepEqual(errors, []);
  const bytes = await readFile("target/makepad-wasm-app/release/fusion-basic/build-manifest.json");
  const manifest = JSON.parse(bytes);
  const { scanner, staging, installation, ...styleEnvironment } = style;
  const { samples, ...gpuEnvironment } = gpu;
  const report = {
    recordedAt: new Date().toISOString(), browser: browser.version(), channel: "chrome", headed: true,
    styleEnvironment, scannerWasmFacade: { ...quantiles(scanner), bytesPerCall: style.bytes,
      bytesPerSecond: style.bytes * scanner.length / (scanner.reduce((a, b) => a + b, 0) / 1000) },
    cssomStaging: quantiles(staging), cssomInstallation: quantiles(installation),
    gpu: { ...gpuEnvironment, ...quantiles(samples) },
    existingStaticBuild: { example: manifest.example, buildId: manifest.build_id, sizeReport: manifest.size_report,
      manifestSha256: createHash("sha256").update(bytes).digest("hex") },
    pending: ["Rust cn native/WASM internal and crossing timings", "Rust compiler cold/warm/no-op timings", "Final utility-lab and static/dynamic byte comparison"],
  };
  await mkdir(dirname(resolve(output)), { recursive: true });
  await writeFile(output, JSON.stringify(report, null, 2) + "\n");
  console.log(`Measured headed Chrome baseline: ${output}`);
} finally {
  await browser?.close();
  for (const child of children) child.kill();
}
