// Reader-side load measurement for Rustify Docs pages, under the plan's
// frozen conditions (docs/plan/DOCS-SSG-P1.md §9.3): the Chromium that
// @playwright/test 1.63.0 pins, a 375 px mobile viewport, CPU slowed 4x and
// Lighthouse's simulated Slow 4G figures applied through CDP. Each sample is
// a fresh browser context, so nothing is cached; the figures are what the
// page recorded about itself (perf-observer.js).
//
//   node tests/docs-ssg/harness/measure-load.mjs --origin http://127.0.0.1:4191 \
//     --page / --page /zh/g08/page-020/ [--samples 5] [--out report.json]

import { chromium } from "@playwright/test";
import { writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";

/** The Chromium every performance figure is taken with. */
export const FROZEN_CHROMIUM = "153.0.8010.12";

/** Lighthouse's simulated Slow 4G: 150 ms, 1.6 Mibit/s down, 750 Kibit/s up. */
export const SLOW_4G = {
  offline: false,
  latency: 150,
  downloadThroughput: (1.6 * 1024 * 1024) / 8,
  uploadThroughput: (750 * 1024) / 8,
};
export const CPU_SLOWDOWN = 4;
export const VIEWPORT = { width: 375, height: 667 };

const observer = fileURLToPath(new URL("./perf-observer.js", import.meta.url));

function args() {
  const argv = process.argv.slice(2);
  const options = { pages: [], samples: 5, origin: null, out: null, settle: 1500 };
  for (let i = 0; i < argv.length; i++) {
    const value = argv[i + 1];
    switch (argv[i]) {
      case "--origin": options.origin = value; i++; break;
      case "--page": options.pages.push(value); i++; break;
      case "--samples": options.samples = Number(value); i++; break;
      case "--out": options.out = value; i++; break;
      case "--settle": options.settle = Number(value); i++; break;
      default: throw new Error(`unknown option ${argv[i]}`);
    }
  }
  if (!options.origin || !options.pages.length) throw new Error("--origin and at least one --page are required");
  return options;
}

const median = (values) => {
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.floor(sorted.length / 2)];
};

export async function sample(browser, url, settle) {
  const context = await browser.newContext({ viewport: VIEWPORT, deviceScaleFactor: 2, isMobile: true, hasTouch: true });
  try {
    await context.addInitScript({ path: observer });
    const page = await context.newPage();
    const cdp = await context.newCDPSession(page);
    await cdp.send("Network.enable");
    await cdp.send("Network.setCacheDisabled", { cacheDisabled: true });
    await cdp.send("Network.emulateNetworkConditions", SLOW_4G);
    await cdp.send("Emulation.setCPUThrottlingRate", { rate: CPU_SLOWDOWN });
    const response = await page.goto(url, { waitUntil: "load" });
    // Late paints and shifts after load still count; give them time to land.
    await page.waitForTimeout(settle);
    const measured = await page.evaluate(() => window.__rustifyPerf.snapshot());
    return { status: response?.status() ?? null, ...measured };
  } finally {
    await context.close();
  }
}

export async function measure({ origin, pages, samples, settle }) {
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_PERF_CHROMIUM || undefined });
  const version = browser.version();
  if (version !== FROZEN_CHROMIUM && !process.env.RUSTIFY_PERF_CHROMIUM) {
    await browser.close();
    throw new Error(`Chromium ${version} is not the frozen ${FROZEN_CHROMIUM}`);
  }
  const results = [];
  try {
    for (const path of pages) {
      const url = new URL(path, origin).href;
      const runs = [];
      for (let i = 0; i < samples; i++) runs.push(await sample(browser, url, settle));
      const wasm = runs.flatMap((run) => run.resources).filter((resource) => /\.wasm(\?|$)/.test(resource.name));
      const foreign = runs.flatMap((run) => run.resources).filter((resource) => new URL(resource.name).origin !== new URL(origin).origin);
      results.push({
        path,
        status: runs[0].status,
        lcp_ms: median(runs.map((run) => run.lcp ?? Infinity)),
        cls: median(runs.map((run) => run.cls)),
        fcp_ms: median(runs.map((run) => run.fcp ?? Infinity)),
        lcp_element: runs[0].lcpElement,
        transfer_bytes: median(runs.map((run) => (run.navigation?.transfer ?? 0) + run.resources.reduce((sum, r) => sum + r.transfer, 0))),
        wasm_requests: wasm.length,
        cross_origin_requests: foreign.length,
        csp_violations: runs.flatMap((run) => run.violations),
        samples: runs.map(({ lcp, cls, fcp, shifts, navigation }) => ({ lcp, cls, fcp, shifts, navigation })),
      });
    }
  } finally {
    await browser.close();
  }
  return {
    chromium: version,
    conditions: { viewport: VIEWPORT, cpu_slowdown: CPU_SLOWDOWN, network: SLOW_4G, cache: "disabled; a fresh context per sample", samples, settle_ms: settle },
    pages: results,
  };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const options = args();
  const report = await measure(options);
  const text = JSON.stringify(report, null, 2) + "\n";
  if (options.out) await writeFile(options.out, text);
  for (const page of report.pages) {
    console.log(`${page.path}: LCP ${page.lcp_ms.toFixed(0)} ms, CLS ${page.cls.toFixed(3)}, FCP ${page.fcp_ms.toFixed(0)} ms, ${page.transfer_bytes} bytes, wasm ${page.wasm_requests}, cross-origin ${page.cross_origin_requests}, CSP violations ${page.csp_violations.length}`);
  }
  console.log(`Chromium ${report.chromium}`);
}
