// A-2 probe: what each way of isolating one browser test from the next costs.
//
// Usage (the example's release build must already be served):
//   node docs/validation/dx/probes/isolation.mjs property-workbench http://127.0.0.1:4174/ [rounds]
//   node docs/validation/dx/probes/isolation.mjs vellum http://127.0.0.1:4179/ [rounds]
//
// Every method is timed from its start to "ready + quiet" (the 500 ms quiet
// window of tests/browser/support.ts is included, the same for all four):
//   context   a new browser context and a navigation per test (the status quo)
//   page      one context, a new page and a navigation per test
//   remount   one page; dispose() + mount() of the example's scope
//   instance  one page; a second wasm instance booted and mounted beside it
// Wall-clock is taken in this process; "in page" is performance.now() in the
// page (for a navigation that is the time since its navigation start).

import { chromium, devices } from "@playwright/test";

const [example, baseURL, roundsArg] = process.argv.slice(2);
const rounds = Number(roundsArg ?? 5);
const vellum = example === "vellum";

const contextOptions = {
    ...devices["Desktop Chrome"],
    baseURL,
    ...(vellum ? { viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 } : {}),
};
const launchOptions = vellum ? { args: ["--enable-unsafe-swiftshader", "--use-angle=swiftshader"] } : {};

const QUIET = ([quietMs, timeoutMs]) =>
    new Promise((resolve) => {
        const deadline = performance.now() + timeoutMs;
        let previous = performance.now();
        let since = previous;
        const tick = () => {
            const now = performance.now();
            if (now - previous > 100) since = now;
            previous = now;
            if (now - since >= quietMs || now > deadline) resolve(performance.now());
            else requestAnimationFrame(tick);
        };
        requestAnimationFrame(tick);
    });

async function waitReady(page) {
    const status = vellum ? "#status" : '[data-testid="status"]';
    await page.waitForSelector(`${status}[data-status="ready"]`, { state: "attached", timeout: 60_000 });
    if (vellum) {
        await page.waitForFunction(() => window.vellum?.ready && window.vellum.renderer.instanceCount > 150, null, { timeout: 60_000 });
    }
    return page.evaluate(QUIET, [500, 60_000]);
}

async function navigate(page) {
    const started = Date.now();
    await page.goto("./");
    const inPage = await waitReady(page);
    return { wall: Date.now() - started, inPage };
}

async function remount(page) {
    const started = Date.now();
    const inPage = await page.evaluate(async ([vellum, quiet]) => {
        const quietFn = new Function(`return (${quiet})`)();
        const t0 = performance.now();
        if (vellum) {
            window.__vellum.dispose();
            window.__vellum.mount();
            await new Promise((resolve) => {
                const check = () => (window.vellum.ready && window.vellum.renderer.instanceCount > 150 ? resolve() : requestAnimationFrame(check));
                requestAnimationFrame(check);
            });
        } else {
            window.__property_workbench.dispose();
            window.__property_workbench.mount();
        }
        const t1 = await quietFn([500, 60_000]);
        return t1 - t0;
    }, [vellum, QUIET.toString()]);
    return { wall: Date.now() - started, inPage };
}

async function secondInstance(page, n) {
    const started = Date.now();
    const inPage = await page.evaluate(async ([vellum, n, quiet]) => {
        const quietFn = new Function(`return (${quiet})`)();
        const t0 = performance.now();
        // The build's own base, not the address bar: an application that
        // routes has moved the address away from where its files are.
        const root = new URL(window.makepad_resource_base ?? "/", location.origin);
        const { boot } = await import(new URL("./loader.js", root).href);
        const host = document.createElement("div");
        host.id = `probe-instance-${n}`;
        document.body.append(host);
        const wasm_url = new URL(vellum ? "./vellum.wasm" : "./property-workbench.wasm", root);
        const started = await boot({ wasm_url, on_fatal: () => {} });
        const { app, build, instance } = started;
        if (vellum) {
            app.vellum_identify(instance, build);
            app.vellum_mount(host.id);
            await new Promise((resolve) => {
                const check = () => (JSON.parse(app.vellum_snapshot()).ready ? resolve() : requestAnimationFrame(check));
                requestAnimationFrame(check);
            });
        } else {
            app.workbench_set_base(started.base);
            app.workbench_identify(instance, build);
            app.workbench_mount(host.id);
        }
        const t1 = await quietFn([500, 60_000]);
        return t1 - t0;
    }, [vellum, n, QUIET.toString()]);
    return { wall: Date.now() - started, inPage };
}

const summary = (samples) => {
    const wall = samples.map((s) => s.wall).sort((a, b) => a - b);
    const inPage = samples.map((s) => s.inPage).sort((a, b) => a - b);
    const med = (xs) => xs[Math.floor(xs.length / 2)];
    return {
        wall_ms: wall.map(Math.round),
        in_page_ms: inPage.map(Math.round),
        wall_median_ms: Math.round(med(wall)),
        in_page_median_ms: Math.round(med(inPage)),
    };
};

const browser = await chromium.launch(launchOptions);
const results = { example, rounds, chromium: browser.version() };

{
    const samples = [];
    for (let i = 0; i < rounds; i++) {
        const context = await browser.newContext(contextOptions);
        const page = await context.newPage();
        samples.push(await navigate(page));
        await context.close();
    }
    results.context = summary(samples);
}
{
    const context = await browser.newContext(contextOptions);
    const samples = [];
    for (let i = 0; i < rounds; i++) {
        const page = await context.newPage();
        samples.push(await navigate(page));
        await page.close();
    }
    await context.close();
    results.page = summary(samples);
}
{
    const context = await browser.newContext(contextOptions);
    const page = await context.newPage();
    await navigate(page);
    const samples = [];
    for (let i = 0; i < rounds; i++) samples.push(await remount(page));
    results.remount = summary(samples);
    const instances = [];
    for (let i = 0; i < rounds; i++) instances.push(await secondInstance(page, i));
    results.instance = summary(instances);
    await context.close();
}
await browser.close();
console.log(JSON.stringify(results, null, 2));
