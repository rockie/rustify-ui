// Replays the first coverage report's reduced-motion counterexample (an
// unselected Tabs control at 375px under `reduce`) on a site the installed C'
// built, and counts every transition or CSS animation the whole page starts,
// shell included, from the first script on. Normal preference is the control.
// node reduced-motion.mjs <site origin with base> <output json>
import { chromium } from "@playwright/test";
import { writeFile } from "node:fs/promises";

const [origin, output] = process.argv.slice(2);
const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
const report = { origin, browser: browser.version(), runs: [] };
const pages = [
  { route: "components/tabs/", target: '[role="tab"][aria-selected="false"]:not([disabled])', act: "click" },
  { route: "components/button/", target: '.rd-demo-canvas [data-name="Button"]', act: "hover" },
  { route: "components/switch/", target: '[role="switch"]', act: "space" },
  { route: "components/loading/", target: '[data-name="Spinner"] > svg', act: null },
];
const settle = page => page.evaluate(() => new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r))));
const sample = (page, target) => page.evaluate(target => {
  const node = document.querySelector(target);
  const css = node && getComputedStyle(node);
  const scope = node?.closest("[data-rustify-scope]");
  return {
    found: !!node,
    transitionProperty: css?.transitionProperty, transitionDuration: css?.transitionDuration, transitionDelay: css?.transitionDelay,
    animationName: css?.animationName,
    scrollBehavior: getComputedStyle(document.documentElement).scrollBehavior,
    reduce: matchMedia("(prefers-reduced-motion: reduce)").matches,
    island: document.querySelector("[data-rustify-island]")?.getAttribute("data-rd-island-state"),
    live: document.getAnimations().map(a => ({ type: a.constructor.name, name: a.transitionProperty || a.animationName, state: a.playState,
      duration: a.effect?.getTiming().duration, inScope: !!scope && scope.contains(a.effect?.target ?? null) })),
    started: window.__motion.slice(),
  };
}, target);

try {
  for (const reducedMotion of ["reduce", "no-preference"]) for (const width of [375, 1280]) {
    const context = await browser.newContext({ viewport: { width, height: 900 }, reducedMotion });
    await context.addInitScript(() => {
      window.__motion = [];
      for (const type of ["transitionrun", "animationstart"]) document.addEventListener(type, e => {
        const t = e.target;
        window.__motion.push({ type, name: e.propertyName || e.animationName, node: t.tagName?.toLowerCase() + (t.getAttribute?.("data-name") ? `[${t.getAttribute("data-name")}]` : ""), at: Math.round(performance.now()) });
      }, true);
    });
    try {
      const page = await context.newPage();
      for (const entry of pages) {
        await page.goto(new URL(entry.route, origin).href);
        await page.waitForFunction(() => document.querySelector("[data-rustify-island]")?.getAttribute("data-rd-island-state") === "ready");
        await page.evaluate(() => document.fonts.ready);
        await page.mouse.move(0, 0);
        await settle(page);
        const ready = await sample(page, entry.target);
        let acted = null;
        if (entry.act) {
          const control = page.locator(entry.target).first();
          if (entry.act === "click") await control.click();
          if (entry.act === "hover") await control.hover();
          if (entry.act === "space") { await control.focus(); await page.keyboard.press("Space"); }
          await settle(page);
          acted = await sample(page, entry.target);
        }
        report.runs.push({ reducedMotion, width, route: entry.route, act: entry.act, ready, acted });
      }
    } finally { await context.close(); }
  }
} finally { await browser.close(); }

const reduce = report.runs.filter(r => r.reducedMotion === "reduce");
const normal = report.runs.filter(r => r.reducedMotion === "no-preference");
const moving = s => s && (s.started.length > 0 || s.live.some(a => a.state === "running"));
report.summary = {
  reduce_samples: reduce.length,
  reduce_any_motion: reduce.filter(r => moving(r.ready) || moving(r.acted)).map(r => `${r.route}@${r.width}`),
  reduce_tab_computed: reduce.filter(r => r.route === "components/tabs/").map(r => [r.width, r.ready.transitionDuration, r.ready.transitionDelay]),
  reduce_spinner: reduce.filter(r => r.route === "components/loading/").map(r => [r.width, r.ready.animationName, r.ready.live.filter(a => a.type === "CSSAnimation").length]),
  reduce_scroll_behavior: [...new Set(reduce.map(r => r.ready.scrollBehavior))],
  normal_tab_click_transitions: normal.filter(r => r.route === "components/tabs/").map(r => [r.width, r.acted.started.filter(e => e.type === "transitionrun").length]),
  normal_spinner_running: normal.filter(r => r.route === "components/loading/").map(r => [r.width, r.ready.live.filter(a => a.type === "CSSAnimation" && a.state === "running").length]),
  normal_scroll_behavior: [...new Set(normal.map(r => r.ready.scrollBehavior))],
};
await writeFile(output, JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify(report.summary, null, 2));
if (report.summary.reduce_any_motion.length) process.exitCode = 1;
