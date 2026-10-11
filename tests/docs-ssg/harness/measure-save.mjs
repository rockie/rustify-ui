// Author-side save measurement for `rustify-docs dev` (docs/plan/DOCS-SSG-P1.md
// §9.3, V-14): from a save completing to the browser showing its result.
//
// A copy of a site is served by `rustify-docs dev`; Chromium (the version
// @playwright/test pins) keeps one of its pages open. Each sample writes a
// file with something new in it, takes the time the write returned, then
// waits for the page to show it. The page records when it painted, on its own
// clock, and the two clocks are put on one axis by the offset measured from
// round trips just before. Kinds:
//
//   body    the open page's Markdown, a new paragraph under its title
//   config  the site's name in rustify-docs.toml (every page changes)
//   static  the logo every page shows (renamed, so every page changes)
//
// Then a short round: saves of the page, one after another, each waiting
// until the page shows it, so no save is merged into the next. With
// `--long N`, the same again N times on the same `dev` (the 1,000-save run),
// sampling the server's resident memory as it goes.
//
//   node tests/docs-ssg/harness/measure-save.mjs --bin target/docs-cli/rustify-docs \
//     --site target/docs-ssg/perf/b --page /zh/g04/page-024/ [--samples 5] \
//     [--short 20] [--long 1000] [--out report.json]

import { chromium } from "@playwright/test";
import { spawn, execFileSync } from "node:child_process";
import { cpSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { performance } from "node:perf_hooks";

import { FROZEN_CHROMIUM } from "./measure-load.mjs";

const LIMIT_MS = 1000;

function args() {
  const argv = process.argv.slice(2);
  const options = { bin: null, site: null, page: null, samples: 5, short: 20, long: 0, out: null };
  for (let i = 0; i < argv.length; i++) {
    const value = argv[i + 1];
    switch (argv[i]) {
      case "--bin": options.bin = value; i++; break;
      case "--site": options.site = value; i++; break;
      case "--page": options.page = value; i++; break;
      case "--samples": options.samples = Number(value); i++; break;
      case "--short": options.short = Number(value); i++; break;
      case "--long": options.long = Number(value); i++; break;
      case "--out": options.out = value; i++; break;
      default: throw new Error(`unknown option ${argv[i]}`);
    }
  }
  if (!options.bin || !options.site || !options.page) throw new Error("--bin, --site and --page are required");
  options.bin = path.resolve(options.bin);
  return options;
}

const now = () => performance.timeOrigin + performance.now();
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const median = (values) => {
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.floor(sorted.length / 2)];
};
const round = (value) => Math.round(value * 10) / 10;
const percentile = (values, p) => {
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.min(sorted.length - 1, Math.ceil((p / 100) * sorted.length) - 1)];
};

/** The process's resident set, in KiB. */
function rss(pid) {
  try {
    return Number(execFileSync("ps", ["-o", "rss=", "-p", String(pid)], { encoding: "utf8" }).trim());
  } catch {
    return null;
  }
}

/** Starts `dev` on `root` and resolves with its address. */
function startDev(bin, root) {
  const child = spawn(bin, ["dev", "--port", "0"], { cwd: root });
  let log = "";
  child.stderr.on("data", (chunk) => (log += chunk));
  return new Promise((resolve, reject) => {
    let out = "";
    child.stdout.on("data", (chunk) => {
      out += chunk;
      const match = /^dev: (\S+)\n/m.exec(out);
      if (match) resolve({ child, url: match[1], log: () => log });
    });
    child.on("exit", (code) => reject(new Error(`dev exited ${code}: ${out}${log}`)));
  });
}

// In every document: when the page has painted after its content loaded,
// and when the header's logo has decoded and painted too.
function witness() {
  const at = () => performance.timeOrigin + performance.now();
  window.__witness = { painted: null, logo: null };
  const twoFrames = (then) => requestAnimationFrame(() => requestAnimationFrame(then));
  document.addEventListener("DOMContentLoaded", () => {
    twoFrames(() => (window.__witness.painted = at()));
    const logos = Array.from(document.querySelectorAll(".rd-brand img")).filter((img) => img.offsetParent !== null);
    Promise.all(logos.map((img) => img.decode().catch(() => undefined))).then(() =>
      twoFrames(() => (window.__witness.logo = at())),
    );
  });
}

/** Waits until `check` holds in the page, across reloads. */
async function until(page, check, arg, timeout = 15_000) {
  const start = Date.now();
  for (;;) {
    try {
      const value = await page.evaluate(check, arg);
      if (value) return value;
    } catch {
      // The page is between documents.
    }
    if (Date.now() - start > timeout) throw new Error(`timed out waiting for ${check}`);
    await sleep(5);
  }
}

/** The browser clock minus this process's, from the tightest round trip. */
async function clockOffset(page) {
  let best = null;
  for (let i = 0; i < 15; i++) {
    const before = now();
    const browser = await page.evaluate(() => performance.timeOrigin + performance.now());
    const after = now();
    const trip = after - before;
    if (!best || trip < best.trip) best = { trip, offset: browser - (before + after) / 2 };
  }
  return best;
}

async function main() {
  const options = args();
  const work = mkdtempSync(path.join(tmpdir(), "rustify-docs-save-"));
  const root = path.join(work, "site");
  cpSync(options.site, root, { recursive: true });
  const markdown = path.join(root, "content", options.page.replace(/^\/|\/$/g, "") + ".md");
  const config = path.join(root, "rustify-docs.toml");
  const logo = path.join(root, "static", "logo.svg");
  const originals = { markdown: readFileSync(markdown, "utf8"), config: readFileSync(config, "utf8"), logo: readFileSync(logo, "utf8") };
  // A paragraph under the title, which the samples rewrite.
  const heading = originals.markdown.search(/^# .*$/m);
  if (heading < 0) throw new Error(`${markdown} has no title`);
  const titleEnd = originals.markdown.indexOf("\n", heading);
  const body = (token) => `${originals.markdown.slice(0, titleEnd)}\n\n${token}\n${originals.markdown.slice(titleEnd)}`;
  if (!originals.config.includes('name = "Reference site B"')) throw new Error("unexpected site name");
  if (!originals.logo.includes("</svg>")) throw new Error("unexpected logo");

  const buildInfo = execFileSync(options.bin, ["--build-info"], { encoding: "utf8" });
  const dev = await startDev(options.bin, root);
  const browser = await chromium.launch();
  const report = {
    chromium: browser.version(),
    binary: buildInfo.trim().split("\n"),
    page: options.page,
    conditions: { viewport: { width: 1280, height: 720 }, throttling: "none: the author's own machine", limit_ms: LIMIT_MS },
    kinds: {},
    short: null,
    checks: [],
  };
  try {
    const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
    await context.addInitScript(witness);
    const page = await context.newPage();
    await page.goto(`${dev.url}${options.page.replace(/^\//, "")}`);
    await until(page, () => document.querySelector(".rd-dev")?.hasAttribute("data-revision"));
    report.clock = await clockOffset(page);
    const offset = report.clock.offset;
    let serial = 0;
    const token = (kind) => `SAVE-${kind}-${process.pid}-${++serial}`;

    const kinds = {
      body: {
        write: (t) => writeFileSync(markdown, body(t)),
        seen: (t) => until(page, (t) => document.body.innerText.includes(t) && window.__witness.painted, t),
      },
      config: {
        write: (t) => writeFileSync(config, originals.config.replace('name = "Reference site B"', `name = "Reference site B ${t}"`)),
        seen: (t) => until(page, (t) => document.querySelector(".rd-brand")?.textContent.includes(t) && window.__witness.painted, t),
      },
      static: {
        write: (t) => writeFileSync(logo, originals.logo.replace("</svg>", `<!-- ${t} --></svg>`)),
        seen: async (t, previous) =>
          until(
            page,
            (previous) => {
              const img = document.querySelector(".rd-brand img");
              return img && img.getAttribute("src") !== previous && window.__witness.logo;
            },
            previous,
          ),
      },
    };

    async function sample(kind) {
      const t = token(kind);
      const previous = await until(page, () => document.querySelector(".rd-brand img")?.getAttribute("src") ?? "none");
      kinds[kind].write(t);
      const saved = now();
      const painted = await kinds[kind].seen(t, previous);
      const latency = painted - offset - saved;
      return { token: t, saved_epoch_ms: round(saved), painted_epoch_ms: round(painted - offset), latency_ms: round(latency) };
    }

    // Warm the caches the way an author's session would be warm.
    await sample("body");
    await sample("body");

    for (const kind of Object.keys(kinds)) {
      const samples = [];
      for (let i = 0; i < options.samples; i++) {
        samples.push(await sample(kind));
        await sleep(300);
      }
      const latencies = samples.map((s) => s.latency_ms);
      report.kinds[kind] = { median_ms: median(latencies), max_ms: Math.max(...latencies), samples };
      report.checks.push({
        check: `${kind} save visible in the browser (median of ${options.samples})`,
        passed: median(latencies) <= LIMIT_MS,
        detail: `${median(latencies)} ms (limit ${LIMIT_MS} ms)`,
      });
      // Back to the original for the next kind.
      if (kind === "config") writeFileSync(config, originals.config);
      if (kind === "static") writeFileSync(logo, originals.logo);
      await sleep(500);
    }

    const short = [];
    for (let i = 0; i < options.short; i++) short.push((await sample("body")).latency_ms);
    report.short = {
      saves: options.short,
      seen: short.length,
      median_ms: median(short),
      max_ms: Math.max(...short),
      latencies_ms: short,
    };
    report.checks.push({
      check: `short round: ${options.short} saves one after another, each seen before the next`,
      passed: short.length === options.short && Math.max(...short) <= LIMIT_MS,
      detail: `median ${median(short)} ms, max ${Math.max(...short)} ms`,
    });
    if (options.long > 0) {
      const latencies = [];
      const memory = [{ saves: 0, rss_kib: rss(dev.child.pid) }];
      for (let i = 1; i <= options.long; i++) {
        latencies.push((await sample("body")).latency_ms);
        if (i % 100 === 0) memory.push({ saves: i, rss_kib: rss(dev.child.pid) });
      }
      const alive = dev.child.exitCode === null && dev.child.signalCode === null;
      report.long = {
        saves: options.long,
        seen: latencies.length,
        server_restarted: !alive,
        median_ms: median(latencies),
        p95_ms: percentile(latencies, 95),
        p99_ms: percentile(latencies, 99),
        max_ms: Math.max(...latencies),
        over_limit: latencies.filter((latency) => latency > LIMIT_MS).length,
        memory,
        latencies_ms: latencies,
      };
      report.checks.push({
        check: `${options.long} saves one after another on one dev, each seen before the next, no restart`,
        passed: latencies.length === options.long && alive,
        detail: `median ${median(latencies)} ms, p95 ${percentile(latencies, 95)} ms, max ${Math.max(...latencies)} ms; ${report.long.over_limit} over ${LIMIT_MS} ms; RSS ${memory[0].rss_kib} → ${memory.at(-1).rss_kib} KiB`,
      });
    }
    report.dev_log_tail = dev.log().split("\n").slice(-6);
  } finally {
    await browser.close();
    dev.child.kill();
    rmSync(work, { recursive: true, force: true });
  }
  if (report.chromium !== FROZEN_CHROMIUM) {
    report.checks.push({ check: "the frozen Chromium", passed: false, detail: `${report.chromium} is not ${FROZEN_CHROMIUM}` });
  }
  const text = JSON.stringify(report, null, 2);
  if (options.out) await writeFile(options.out, `${text}\n`);
  console.log(text);
  if (report.checks.some((check) => !check.passed)) process.exitCode = 1;
}

await main();
