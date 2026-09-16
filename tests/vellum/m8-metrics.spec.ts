import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { expect, test, waitForReady } from "./support";

for (const sample of [1, 2, 3]) {
    test(`cold browser context records startup sample ${sample}`, async ({ page, context, browser }, info) => {
        const session = await context.newCDPSession(page);
        await session.send("Network.enable");
        await session.send("Network.setCacheDisabled", { cacheDisabled: true });
        await page.addInitScript(() => {
            const times = { readyMs: null as number | null, firstFrameMs: null as number | null, refinedMs: null as number | null };
            (window as any).__vellumStartupTimes = times;
            const observe = () => {
                if (times.readyMs === null && document.querySelector('#status[data-status="ready"]')) times.readyMs = performance.now();
                const runtime = window.__vellum;
                if (runtime && runtime.stats().frames > 0) {
                    if (times.firstFrameMs === null) times.firstFrameMs = performance.now();
                    if (times.refinedMs === null && runtime.rasterStats().pending === 0) times.refinedMs = performance.now();
                }
                if (times.readyMs === null || times.firstFrameMs === null || times.refinedMs === null) requestAnimationFrame(observe);
            };
            requestAnimationFrame(observe);
        });
        await waitForReady(page);
        await page.waitForFunction(() => Object.values((window as any).__vellumStartupTimes).every(value => value !== null));
        const metrics = await page.evaluate(() => ({
            ...((window as any).__vellumStartupTimes as { readyMs: number; firstFrameMs: number; refinedMs: number }),
            timeOrigin: performance.timeOrigin,
            backend: window.vellum.renderer.backend,
            ledger: window.__vellum.stats(),
            regions: window.__vellum.live_regions(),
            nodes: window.vellum.doc.nodes.length,
            viewport: { width: innerWidth, height: innerHeight, dpr: devicePixelRatio },
            navigation: performance.getEntriesByType("navigation")[0].toJSON(),
            resources: performance.getEntriesByType("resource").map(entry => {
                const resource = entry as PerformanceResourceTiming;
                return { name: new URL(resource.name).pathname, duration: resource.duration, transferSize: resource.transferSize, decodedBodySize: resource.decodedBodySize };
            }),
        }));
        expect(metrics.regions).toBe(1);
        expect(metrics.nodes).toBe(171);
        expect(metrics.ledger.frames).toBeGreaterThan(0);
        expect(metrics.readyMs).toBeGreaterThan(0);
        expect(metrics.refinedMs).toBeGreaterThanOrEqual(metrics.firstFrameMs);
        const report = { sample, browser: browser.version(), coldBrowserContext: true, httpCacheDisabled: true, physicalDisplayLatency: false, ...metrics };
        const json = JSON.stringify(report, null, 2);
        const directory = path.resolve(__dirname, "../../test-results/vellum");
        await mkdir(directory, { recursive: true });
        await writeFile(path.join(directory, `m8-startup-${sample}.json`), json);
        await info.attach("startup-metrics", { body: json, contentType: "application/json" });
        console.log(`Cold startup ${sample}: ${JSON.stringify({ readyMs: metrics.readyMs, firstFrameMs: metrics.firstFrameMs, refinedMs: metrics.refinedMs, frames: metrics.ledger.frames })}`);
    });
}
