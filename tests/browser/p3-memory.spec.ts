import { CDPSession, expect, Page, test } from "@playwright/test";
import { R32 } from "./budgets";
import { B4 } from "./loads";
import { waitForQuiet } from "./support";

/// M6 · what the loads hold, and what a hundred rounds of mounting adds.
///
/// Two figures per load. The CPU one is committed wasm memory plus the
/// JavaScript heap, read through the debugger rather than through
/// `performance.memory`, which this browser rounds to the nearest ten
/// megabytes. The GPU one is a ledger the host keeps - every buffer and
/// texture the renderer uploaded - and not a reading from the driver, which
/// the web has no way to ask.
///
/// B4 is the one that is a trend rather than a level: linear memory never
/// shrinks, so what a mount-and-unmount round may not do is keep pushing it
/// up.

const MiB = 1024 * 1024;

/// Bytes of JavaScript heap in use, as the debugger reports them. Precise,
/// unlike `performance.memory`, which M1's fourth probe measured being
/// quantised to ten million.
async function jsHeap(cdp: CDPSession): Promise<number> {
    const { usedSize } = (await cdp.send("Runtime.getHeapUsage")) as { usedSize: number };
    return usedSize;
}

/// The same, after collecting garbage first.
///
/// What a growth budget asks about is what is still held, and an uncollected
/// heap answers a different question: the sawtooth between two collections is
/// larger than the budget for eighty rounds, so a trend drawn through it
/// would be a reading of when the collector last ran.
async function retainedHeap(cdp: CDPSession): Promise<number> {
    await cdp.send("HeapProfiler.collectGarbage");
    return jsHeap(cdp);
}

function slope(samples: number[]): number {
    const n = samples.length;
    const meanX = (n - 1) / 2;
    const meanY = samples.reduce((sum, value) => sum + value, 0) / n;
    let top = 0;
    let bottom = 0;
    for (let i = 0; i < n; i++) {
        top += (i - meanX) * (samples[i] - meanY);
        bottom += (i - meanX) * (i - meanX);
    }
    return top / bottom;
}

test.describe("M6 · B0 holds what B0 is worth", () => {
    test.beforeEach(({}, info) => {
        test.skip(info.project.name !== "fusion-basic", "B0 is the fusion-basic page");
    });

    test("the smallest application, at its peak", async ({ page }) => {
        test.setTimeout(300_000);
        await page.goto("./");
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
            timeout: 120_000,
        });
        await expect
            .poll(
                async () => (await page.evaluate(() => window.__fusion_basic.b0_state())).region,
                { timeout: 60_000 }
            )
            .toBe("ready");
        await waitForQuiet(page);
        const cdp = await page.context().newCDPSession(page);

        // The peak, not the moment after loading: every control is worked
        // once, because a texture is uploaded when something first needs it.
        let cpuPeak = 0;
        let gpuPeak = 0;
        const sample = async () => {
            const stats = await page.evaluate(() => window.__fusion_basic.stats());
            cpuPeak = Math.max(cpuPeak, stats.memory + (await jsHeap(cdp)));
            gpuPeak = Math.max(gpuPeak, stats.gpu_bytes);
        };
        await sample();
        for (let round = 0; round < 5; round++) {
            await page.evaluate(() => {
                const api = window.__fusion_basic;
                const click = (id: string) =>
                    (document.querySelector(`[data-testid="${id}"]`) as HTMLElement)?.click();
                click("b0-bump");
                click("b0-visible");
                click("b0-compact");
                return api.b0_state().count;
            });
            await page.waitForTimeout(200);
            await sample();
        }

        console.log(
            `B0: cpu peak ${cpuPeak} bytes (${(cpuPeak / MiB).toFixed(1)} MiB), ` +
                `gpu ledger ${gpuPeak} bytes (${(gpuPeak / MiB).toFixed(1)} MiB)`
        );
        expect(cpuPeak).toBeLessThanOrEqual(R32.b0_cpu_bytes);
        expect(gpuPeak).toBeLessThanOrEqual(R32.b0_gpu_bytes);
        // A ledger that never counts anything would pass the line above and
        // mean nothing: a region that has drawn has uploaded something.
        expect(gpuPeak).toBeGreaterThan(0);
    });

    test("two instances, a hundred rounds, and nothing kept", async ({ page }) => {
        test.setTimeout(900_000);
        await page.goto("./");
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
            timeout: 120_000,
        });
        await waitForQuiet(page);
        const cdp = await page.context().newCDPSession(page);

        // Two instances, booted once. A round mounts one scope of two regions
        // in each of them and takes it away again; restarting an instance
        // would be a different measurement, because a dead instance's linear
        // memory never comes back.
        const second = await page.evaluate(() => window.__fusion_basic.boot_second_instance());
        expect(second).toBe(B4.instances);

        const wasmBytes = () =>
            page.evaluate(
                () =>
                    window.__fusion_instances[1].stats().memory +
                    window.__fusion_instances[2].stats().memory
            );
        const round = () =>
            page.evaluate(async () => {
                const first = window.__fusion_instances[1];
                const second = window.__fusion_instances[2];
                first.mount("scope-a");
                second.mount("instance-two");
                await new Promise((resolve) => setTimeout(resolve, 20));
                first.dispose("scope-a");
                second.dispose("instance-two");
                await new Promise((resolve) => setTimeout(resolve, 20));
            });

        for (let warm = 0; warm < B4.warmupRounds; warm++) {
            await round();
        }
        const start = (await wasmBytes()) + (await retainedHeap(cdp));
        const samples: number[] = [];
        for (let measured = 0; measured < B4.measuredRounds; measured++) {
            await round();
            samples.push((await wasmBytes()) + (await retainedHeap(cdp)));
        }
        const end = samples[samples.length - 1];
        const tail = samples.slice(-40);

        console.log(
            `B4: ${B4.warmupRounds} warm-up + ${B4.measuredRounds} rounds, ` +
                `${start} -> ${end} bytes (${((end - start) / MiB).toFixed(2)} MiB), ` +
                `last forty ${slope(tail).toFixed(0)} bytes/round`
        );

        expect(end - start).toBeLessThanOrEqual(R32.growth_bytes);
        expect(slope(tail)).toBeLessThanOrEqual(R32.growth_bytes_per_round);

        // And when the last scope goes, nothing of the application is left in
        // either instance: no region, no timer, no task, no GPU bytes, and
        // none of its own components. The B0 scope the page mounted at load
        // goes too - it is the last one, and "nothing left" cannot be asked
        // of an instance that is still showing something.
        const residue = await page.evaluate(async () => {
            window.__fusion_instances[1].dispose("b0");
            await new Promise((resolve) => setTimeout(resolve, 500));
            return [1, 2].map((number) => {
                const instance = window.__fusion_instances[number];
                const stats = instance.stats();
                return {
                    regions: stats.regions,
                    timers: stats.timers,
                    tasks: stats.tasks,
                    gpu_bytes: stats.gpu_bytes,
                    components: instance.live_components(),
                };
            });
        });
        for (const [index, instance] of residue.entries()) {
            expect(instance, `instance ${index + 1}`).toEqual({
                regions: 0,
                timers: 0,
                tasks: 0,
                gpu_bytes: 0,
                components: 0,
            });
        }
    });
});

test.describe("M6 · B2 holds what B2 is worth", () => {
    test.beforeEach(({}, info) => {
        test.skip(info.project.name !== "data-workbench", "B2 is the data-workbench page");
    });

    test("a hundred thousand rows, one sort and one filter", async ({ page }) => {
        test.setTimeout(300_000);
        await page.setViewportSize({ width: 1440, height: 1200 });
        await page.goto("./table");
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
            timeout: 120_000,
        });
        await expect(page.getByTestId("table-view")).toHaveCount(1);
        await waitForQuiet(page);
        const cdp = await page.context().newCDPSession(page);

        let cpuPeak = 0;
        let gpuPeak = 0;
        const sample = async () => {
            const stats = await page.evaluate(() => window.__data_workbench.stats());
            cpuPeak = Math.max(cpuPeak, stats.memory + (await jsHeap(cdp)));
            gpuPeak = Math.max(gpuPeak, stats.gpu_bytes);
        };
        await sample();

        // The load as R32 defines it: the whole sample in memory, put in
        // order once and filtered once. Both are jobs, and a job holds a
        // second copy of the order while it runs - so the peak is sampled
        // while one is running as well as after it.
        const snapshot = () => page.evaluate(() => window.__data_workbench.snapshot());
        const slicesRun = async () => (await snapshot()).table.job.slices;
        const finished = async (what: string) => {
            // A job that is asked for is not a job that has started: polling
            // for "not running" answers yes before the first slice. What says
            // it ran is a slice having been taken.
            await expect.poll(slicesRun, { timeout: 60_000 }).toBeGreaterThan(0);
            await sample();
            await expect
                .poll(async () => (await snapshot()).table.job.running, { timeout: 60_000 })
                .toBe(false);
            expect((await snapshot()).table.job.ended, what).toBe("done");
            await sample();
        };

        await page.getByTestId("table-column-0").click();
        await finished("the sort");
        expect((await snapshot()).table.sorted).toBe("0:asc");

        await page.getByTestId("table-filter").fill("AB");
        await page.getByTestId("table-filter").press("Enter");
        await finished("the filter");
        const filtered = (await snapshot()).table.shown;
        expect(filtered).toBeGreaterThan(0);
        console.log(`B2: the filter left ${filtered} of ${(await snapshot()).rows} rows`);

        console.log(
            `B2: cpu peak ${cpuPeak} bytes (${(cpuPeak / MiB).toFixed(1)} MiB), ` +
                `gpu ledger ${gpuPeak} bytes (${(gpuPeak / MiB).toFixed(1)} MiB)`
        );
        expect(cpuPeak).toBeLessThanOrEqual(R32.b2_cpu_bytes);
        expect(gpuPeak).toBeLessThanOrEqual(R32.b2_gpu_bytes);
        expect(gpuPeak).toBeGreaterThan(0);
    });
});
