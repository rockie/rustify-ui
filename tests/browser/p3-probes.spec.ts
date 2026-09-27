import { chromium, expect, Page, test } from "@playwright/test";
import { fusionPort } from "../../playwright.config";
import { percentile, R30_LARGE } from "./budgets";
import { BASELINE, B2, B3 } from "./loads";
import * as twin from "./dataset";
import { EVIDENCE } from "../tier";

/// The five things P3's design assumes and has not measured.
///
/// Each of these prints a number and reaches a verdict; none of them is a
/// gate. A probe that finds against the design sends the phase down a branch
/// that was written before the probe ran, so what these must never do is pass
/// quietly on a measurement they did not actually take: every one of them
/// asserts that it has samples, and asserts whatever correctness it can, but
/// the budget comparison is printed rather than enforced. The budgets become
/// assertions in M8, on runs that are five times as long.
///
/// Every figure is taken **inside the page**. A navigation costs about eleven
/// seconds of wall clock on this machine whatever the page does, which is two
/// orders of magnitude above the intervals being measured here.

/// What one run of a frame probe reports.
interface FrameRun {
    /// Intervals between effective presentations, in milliseconds.
    samples: number[];
    /// Frames driven, presentations that followed, and animation frame
    /// callbacks, all counted after the warm-up.
    drives: number;
    effective: number;
    callbacks: number;
    /// Pumps the runtime ran over the same window, when the run drove a
    /// region. A presentation needs a pump, so the two together say whether a
    /// frame was never asked for or was asked for and not delivered.
    pumps?: number;
}

interface Verdict {
    p95: number;
    share: number;
    holds: boolean;
}

function report(name: string, run: FrameRun): Verdict {
    const p95 = percentile(run.samples, 95);
    const slow = run.samples.filter((ms) => ms > R30_LARGE.frame_slow_ms).length;
    const share = run.samples.length === 0 ? 1 : slow / run.samples.length;
    const holds = p95 <= R30_LARGE.frame_p95_ms && share <= R30_LARGE.frame_slow_share;
    console.log(
        `  ${name}: p95 ${p95.toFixed(2)} ms, >${R30_LARGE.frame_slow_ms} ms ` +
            `${(share * 100).toFixed(2)}% of ${run.samples.length}, ` +
            `${run.effective} presentations for ${run.drives} drives ` +
            `(${run.callbacks} callbacks${run.pumps === undefined ? "" : `, ${run.pumps} pumps`})` +
            ` -> ${holds ? "holds" : "does not hold"}`
    );
    return { p95, share, holds };
}

const PROBE_MS = 10_000;
const WARMUP_MS = 2_000;

async function waitForReady(page: Page, path: string) {
    await page.goto(path);
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 120_000,
    });
}

/// Builds a bare windowed grid in the page's scratch area and scrolls it.
///
/// No application, no framework, no wasm: sixty rows of twelve cells, a pool of
/// row elements moved rather than replaced, and the cell text rewritten from a
/// scroll handler. If a browser cannot hold twenty milliseconds doing only
/// this, no table built on top of it will either, and B2 belongs on the GPU.
async function bareGrid(page: Page, ms: number, warmupMs: number): Promise<FrameRun> {
    return page.evaluate(
        async ({ ms, warmupMs, rows, rowsPerFrame }) => {
            const VISIBLE = 60;
            const COLUMNS = 12;
            const OVERSCAN = 4;
            const ROW_HEIGHT = 24;
            const scratch = document.getElementById("scratch")!;
            scratch.replaceChildren();
            const viewport = document.createElement("div");
            viewport.style.cssText =
                `height:${VISIBLE * ROW_HEIGHT}px;width:${COLUMNS * 110}px;` +
                "overflow:auto;position:relative;contain:strict";
            const spacer = document.createElement("div");
            spacer.style.height = `${rows * ROW_HEIGHT}px`;
            const pool = document.createElement("div");
            pool.style.cssText = "position:absolute;top:0;left:0";
            const rowElements: HTMLElement[] = [];
            const texts: Text[] = [];
            for (let slot = 0; slot < VISIBLE + OVERSCAN; slot++) {
                const row = document.createElement("div");
                row.style.cssText = `position:absolute;height:${ROW_HEIGHT}px;display:flex`;
                for (let column = 0; column < COLUMNS; column++) {
                    const box = document.createElement("span");
                    box.style.cssText = "width:110px;overflow:hidden;white-space:nowrap";
                    const text = document.createTextNode("");
                    box.appendChild(text);
                    row.appendChild(box);
                    texts.push(text);
                }
                pool.appendChild(row);
                rowElements.push(row);
            }
            viewport.append(spacer, pool);
            scratch.appendChild(viewport);

            // Values to put in the cells: a pool rather than generated text,
            // because what is being measured is the cost of putting characters
            // into the document, not the cost of making them.
            const values: string[] = [];
            for (let i = 0; i < 997; i++) {
                values.push(`CELL${String(i).padStart(5, "0")}XYZAB`.slice(0, 16));
            }

            let version = 0;
            const render = () => {
                const first = Math.max(
                    0,
                    Math.min(
                        rows - (VISIBLE + OVERSCAN),
                        Math.floor(viewport.scrollTop / ROW_HEIGHT) - OVERSCAN / 2
                    )
                );
                for (let slot = 0; slot < rowElements.length; slot++) {
                    const row = first + slot;
                    rowElements[slot].style.transform = `translateY(${row * ROW_HEIGHT}px)`;
                    rowElements[slot].setAttribute("aria-rowindex", String(row + 1));
                    for (let column = 0; column < COLUMNS; column++) {
                        texts[slot * COLUMNS + column].nodeValue =
                            values[(row * COLUMNS + column) % values.length];
                    }
                }
                version += 1;
            };
            viewport.addEventListener("scroll", render);
            render();

            return await new Promise<{
                samples: number[];
                drives: number;
                effective: number;
                callbacks: number;
                cells: number;
            }>((resolve) => {
                const samples: number[] = [];
                let drives = 0;
                let effective = 0;
                let callbacks = 0;
                let previous: number | null = null;
                let seen = version;
                let top = 0;
                const started = performance.now();
                const tick = (now: number) => {
                    const measuring = now - started >= warmupMs;
                    callbacks += 1;
                    top += rowsPerFrame * ROW_HEIGHT;
                    if (top > (rows - VISIBLE) * ROW_HEIGHT) {
                        top = 0;
                    }
                    viewport.scrollTop = top;
                    if (measuring) {
                        drives += 1;
                    }
                    // Only a frame whose content moved is a frame: the
                    // callbacks keep arriving whether or not it did.
                    if (version !== seen) {
                        seen = version;
                        if (measuring) {
                            effective += 1;
                            if (previous !== null) {
                                samples.push(now - previous);
                            }
                        }
                        previous = now;
                    }
                    if (now - started >= ms + warmupMs) {
                        scratch.replaceChildren();
                        resolve({ samples, drives, effective, callbacks, cells: texts.length });
                        return;
                    }
                    requestAnimationFrame(tick);
                };
                requestAnimationFrame(tick);
            });
        },
        { ms, warmupMs, rows: B2.rows, rowsPerFrame: B2.scrollRowsPerFrame }
    );
}

/// Pans the scene and samples the intervals between presentations.
///
/// The content version is the runtime's own presentation count, so a frame the
/// region did not draw produces no sample. The camera is driven through the
/// application rather than by a wheel event: the input path is M4's, and what
/// this probe is about is whether a rasteriser can draw a window of B3 inside
/// twenty milliseconds at all.
async function panScene(page: Page, ms: number, warmupMs: number): Promise<FrameRun> {
    return page.evaluate(
        async ({ ms, warmupMs, step, extent }) => {
            const api = window.__data_workbench;
            return await new Promise<{
                samples: number[];
                drives: number;
                effective: number;
                callbacks: number;
                pumps: number;
            }>((resolve) => {
                const samples: number[] = [];
                let drives = 0;
                let effective = 0;
                let callbacks = 0;
                let previous: number | null = null;
                let seen = api.stats().frames;
                let pumpsAtWarmup = 0;
                let x = 0;
                let y = 0;
                const started = performance.now();
                const tick = (now: number) => {
                    const measuring = now - started >= warmupMs;
                    callbacks += 1;
                    x = x + step[0] > extent[0] ? 0 : x + step[0];
                    y = y + step[1] > extent[1] ? 0 : y + step[1];
                    api.look_at(x, y);
                    const stats = api.stats();
                    if (measuring) {
                        drives += 1;
                    } else {
                        pumpsAtWarmup = stats.pumps;
                    }
                    const frames = stats.frames;
                    if (frames !== seen) {
                        seen = frames;
                        if (measuring) {
                            effective += 1;
                            if (previous !== null) {
                                samples.push(now - previous);
                            }
                        }
                        previous = now;
                    }
                    if (now - started >= ms + warmupMs) {
                        resolve({
                            samples,
                            drives,
                            effective,
                            callbacks,
                            pumps: api.stats().pumps - pumpsAtWarmup,
                        });
                        return;
                    }
                    requestAnimationFrame(tick);
                };
                requestAnimationFrame(tick);
            });
        },
        { ms, warmupMs, step: B3.panPerFrame, extent: B3.extent }
    );
}

/// Opens the scene view and waits until the region has drawn once: until then
/// its pane is nothing and a camera would be clamped against nothing.
async function openScene(page: Page) {
    await waitForReady(page, "./scene");
    await expect(page.getByTestId("scene-gpu")).toHaveCount(1);
    await expect
        .poll(
            async () =>
                (await page.evaluate(() => window.__data_workbench.snapshot())).scene.drawn,
            { timeout: 60_000 }
        )
        .toBeGreaterThan(0);
}

/// A hundred actions on the first instance, and how many of them it took.
///
/// Health is measured by what still works, not by an absence of console noise:
/// a dead instance's closures can be finalised later and complain, which says
/// nothing about the instance beside it.
async function liveActions(page: Page): Promise<number> {
    return page.evaluate(async () => {
        const button = document.querySelector('[data-testid="scope-a-dom-increment"]');
        if (!(button instanceof HTMLElement)) {
            return -1;
        }
        const count = () =>
            Number(document.querySelector('[data-testid="scope-a-dom-count"]')?.textContent ?? "0");
        const before = count();
        for (let i = 0; i < 100; i++) {
            button.click();
        }
        await new Promise((resolve) => setTimeout(resolve, 250));
        // Read through the instance as well: a count that moved while the
        // module was dead would be the DOM lying about it.
        window.__fusion_instances[1].stats();
        return count() - before;
    });
}

/// Records every policy violation the page reports, from before its own
/// scripts run. A page that boots is not a page that boots cleanly: a blocked
/// stylesheet or a refused module shows up here and nowhere else.
async function watchPolicy(page: Page) {
    await page.addInitScript(() => {
        (window as unknown as { __csp: string[] }).__csp = [];
        document.addEventListener("securitypolicyviolation", (event) => {
            (window as unknown as { __csp: string[] }).__csp.push(
                `${event.violatedDirective} ${event.blockedURI}`
            );
        });
    });
}

const policyViolations = (page: Page) =>
    page.evaluate(() => (window as unknown as { __csp: string[] }).__csp);

test.describe("M1 · the five things P3 assumes", { tag: EVIDENCE }, () => {
    test("the skeleton boots under the release policy with nothing refused", async ({ page }) => {
        await watchPolicy(page);
        await waitForReady(page, "./");
        // The root is neither view: arriving there is arriving at the table.
        await expect(page.getByTestId("table-view")).toHaveCount(1);
        expect(await policyViolations(page)).toEqual([]);
        const snapshot = await page.evaluate(() => window.__data_workbench.snapshot());
        console.log(
            `  the sample: ${snapshot.rows} rows generated in ${snapshot.generated_ms.toFixed(0)} ms, ` +
                `version ${snapshot.version}`
        );
        expect(snapshot.rows).toBe(twin.ROWS);
        expect(snapshot.path).toBe("/table");
        await page.getByTestId("go-scene").click();
        await expect(page.getByTestId("scene-view")).toHaveCount(1);
        expect(await policyViolations(page)).toEqual([]);
        // Informational entries are a report, not a fault. Neither is a
        // refused page-level capability: Makepad names its window as it
        // starts, so every region on every example asks for the document
        // title and is told no, and that record is the contract working
        // (`docs/compatibility.md`). Anything else here is a fault.
        const faults = await page.evaluate(() =>
            window.__data_workbench
                .diagnostics()
                .entries.filter(
                    (entry) => entry.severity === "error" && entry.kind !== "UnsupportedCapability"
                )
                .map((entry) => `${entry.kind}: ${entry.detail}`)
        );
        expect(faults).toEqual([]);
    });

    test("the runtime counts the frames it presents", async ({ page }) => {
        await openScene(page);
        const before = await page.evaluate(() => window.__data_workbench.stats().frames);
        expect(before).toBeGreaterThan(0);
        await page.evaluate(() => window.__data_workbench.look_at(400, 200));
        await expect
            .poll(async () => page.evaluate(() => window.__data_workbench.stats().frames))
            .toBeGreaterThan(before);
        // Frozen, the region goes on being pumped and stops presenting. This is
        // what makes the frame budget's own negative check possible: a budget
        // that samples animation frames passes this at a steady 16.7 ms.
        await page.evaluate(() => window.__data_workbench.freeze_scene(true));
        const frozen = await page.evaluate(() => window.__data_workbench.stats().frames);
        const after = await page.evaluate(async () => {
            const api = window.__data_workbench;
            for (let i = 0; i < 30; i++) {
                api.look_at(500 + i * 17, 300 + i * 7);
                await new Promise((resolve) => requestAnimationFrame(resolve));
            }
            return api.stats().frames;
        });
        expect(after).toBe(frozen);
    });

    test("probe 1 (A-2): a bare windowed grid under the frame budget", async ({ page }) => {
        // Sixty rows of twenty-four pixels need a viewport that really is
        // fourteen hundred tall, or the browser paints a fraction of what the
        // probe claims to be measuring.
        await page.setViewportSize({ width: 1440, height: 1560 });
        await waitForReady(page, "./table");
        const run = await bareGrid(page, PROBE_MS, WARMUP_MS);
        const verdict = report("bare DOM grid", run);
        expect(run.samples.length).toBeGreaterThan(100);
        // The grid has to be keeping up with the scrolling, or the interval is
        // the interval between the frames it managed rather than the ones it
        // was asked for.
        expect(run.effective).toBeGreaterThan(run.drives * R30_LARGE.effective_frame_share);
        console.log(`  A-2: DOM windowing ${verdict.holds ? "is viable" : "is not viable"} for B2`);
    });

    test("probe 2 (A-3): the scene's frame interval, headless and headed", async ({ page }, info) => {
        test.setTimeout(240_000);
        await openScene(page);
        // The region has to be drawing a window rather than the whole scene, or
        // the figures below are about a different load than B3.
        const snapshot = await page.evaluate(() => window.__data_workbench.snapshot());
        const visible = await page.evaluate(() => window.__data_workbench.scene_visible());
        expect(visible).toBe(
            twin.sceneVisible(
                snapshot.scene.camera[0],
                snapshot.scene.camera[1],
                snapshot.scene.pane[0],
                snapshot.scene.pane[1]
            )
        );
        expect(visible).toBeLessThan(B3.objects / 4);
        expect(snapshot.scene.drawn).toBe(visible);

        const software = report("scene, headless SwiftShader", await panScene(page, PROBE_MS, WARMUP_MS));

        // The same scene in the browser a person would be using. Headless
        // Chromium rasterises in software here, and R30's figures are about a
        // machine with a GPU: a pass or a failure under SwiftShader is not a
        // statement about either.
        let hardware: Verdict | null = null;
        let unavailable: string | null = null;
        const base = info.project.use.baseURL!;
        try {
            const browser = await chromium.launch({ channel: "chrome", headless: false });
            try {
                const headed = await browser.newPage();
                await headed.goto(new URL("scene", base).href);
                await expect(headed.getByTestId("status")).toHaveAttribute("data-status", "ready", {
                    timeout: 120_000,
                });
                await expect
                    .poll(
                        async () =>
                            (await headed.evaluate(() => window.__data_workbench.snapshot())).scene
                                .drawn,
                        { timeout: 60_000 }
                    )
                    .toBeGreaterThan(0);
                hardware = report("scene, headed Chrome", await panScene(headed, PROBE_MS, WARMUP_MS));
            } finally {
                await browser.close();
            }
        } catch (error) {
            unavailable = String(error).split("\n")[0];
        }

        console.log(
            `  A-3: headless ${software.holds ? "holds" : "does not hold"}; headed ` +
                (unavailable === null
                    ? `${hardware?.holds ? "holds" : "does not hold"}`
                    : `not measured (${unavailable})`)
        );
        expect(software.p95).toBeGreaterThan(0);
    });

    test("probe 3 (A-5): two instances, three traps and one restart", async ({ page }) => {
        test.setTimeout(300_000);
        const pageErrors: string[] = [];
        page.on("pageerror", (error) => pageErrors.push(String(error)));
        await watchPolicy(page);
        await page.goto(`http://127.0.0.1:${fusionPort}/`);
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
            timeout: 120_000,
        });

        // Instance 1's health is measured below by how many of a hundred
        // actions a counter scope takes, and since P3 M6 the page's own mount
        // is B0 - so the scope this probe measures is one it mounts itself.
        await page.evaluate(() => window.__fusion_basic.mount("scope-a"));

        // A second instance of the same module: its own memory, its own
        // runtime, one compilation.
        expect(await page.evaluate(() => window.__fusion_basic.boot_instance("instance-two"))).toBe(2);
        const separate = await page.evaluate(() => {
            const one = window.__fusion_instances[1];
            const two = window.__fusion_instances[2];
            const named = (part: string) =>
                performance.getEntriesByType("resource").filter((entry) => entry.name.includes(part))
                    .length;
            return {
                sharedMemory: one.wasm.exports.memory === two.wasm.exports.memory,
                wasmRequests: named(".wasm"),
                glueRequests: named("bindgen.js"),
                bytes: [
                    one.wasm.exports.memory.buffer.byteLength,
                    two.wasm.exports.memory.buffer.byteLength,
                ],
            };
        });
        console.log(
            `  two instances: shared memory ${separate.sharedMemory}, ` +
                `${separate.wasmRequests} wasm request(s), ${separate.glueRequests} glue request(s), ` +
                `${separate.bytes.map((b) => (b / 1024 / 1024).toFixed(1)).join(" + ")} MB`
        );
        expect(separate.sharedMemory).toBe(false);
        expect(separate.wasmRequests).toBe(1);
        // A same-origin module with a query string is what a second instance
        // costs, and `script-src 'self'` has to allow it.
        expect(await policyViolations(page)).toEqual([]);
        await expect(page.getByTestId("trap-in-handler")).toHaveCount(1);

        // A real trap through an export call. The boundary reports it to the
        // instance it happened in and rethrows, so the caller still sees it.
        expect(
            await page.evaluate(() => {
                try {
                    window.__fusion_instances[2].trap();
                    return "no error";
                } catch (error) {
                    return (error as Error).constructor.name;
                }
            })
        ).toBe("RuntimeError");
        expect(await page.evaluate(() => window.__fusion_instances[2].fatal())).not.toBeNull();
        expect(await page.evaluate(() => window.__fusion_instances[1].fatal())).toBeNull();
        const afterExport = await liveActions(page);
        console.log(`  after an export trap: instance 1 took ${afterExport}/100 actions`);
        expect(afterExport).toBe(100);
        // A dead instance refuses further calls rather than trapping again.
        expect(
            await page.evaluate(() => {
                try {
                    window.__fusion_instances[2].live_regions();
                    return "no error";
                } catch (error) {
                    return (error as Error).constructor.name;
                }
            })
        ).toBe("InstanceDead");

        // One restart of the same slot, and what the dead instance leaves
        // behind: a glue URL's module record lives as long as the document, so
        // the expectation is that its memory never comes back.
        const restarted = await page.evaluate(async () => {
            const dead = window.__fusion_instances[2];
            const bytes = dead.wasm.exports.memory.buffer.byteLength;
            (window as unknown as { __dead: WeakRef<object> }).__dead = new WeakRef(
                dead.wasm.exports.memory
            );
            const next = await dead.restart("instance-two");
            delete window.__fusion_instances[2];
            return { bytes, next, restarts: next === null ? -1 : window.__fusion_instances[next].restarts };
        });
        expect(restarted.next).toBe(3);
        expect(restarted.restarts).toBe(1);
        const client = await page.context().newCDPSession(page);
        await client.send("HeapProfiler.enable");
        await client.send("HeapProfiler.collectGarbage");
        const retained = await page.evaluate(
            () => (window as unknown as { __dead: WeakRef<object> }).__dead.deref() !== undefined
        );
        console.log(
            `  after one restart the dead instance's ` +
                `${(restarted.bytes / 1024 / 1024).toFixed(1)} MB is still reachable: ${retained}`
        );

        // The third way in: a trap inside a DOM event handler, which leaves
        // wasm through the browser's own dispatch with nothing of ours on the
        // stack. Attribution is by the glue frame in the uncaught error.
        await page.getByTestId("trap-in-handler").click();
        let attributed = true;
        try {
            await expect
                .poll(async () => page.evaluate(() => window.__fusion_instances[3]?.fatal()), {
                    timeout: 10_000,
                })
                .not.toBeNull();
        } catch {
            attributed = false;
        }
        console.log(`  a handler trap was attributed to its own instance: ${attributed}`);
        expect(await page.evaluate(() => window.__fusion_instances[1].fatal())).toBeNull();
        const afterHandler = await liveActions(page);
        console.log(
            `  after a handler trap: instance 1 took ${afterHandler}/100 actions, ` +
                `${pageErrors.length} page error(s)`
        );
        expect(afterHandler).toBe(100);
    });

    test("probe 4 (A-6): the memory and CPU readings the budgets need", async ({ page }) => {
        await openScene(page);
        const inPage = await page.evaluate(() => {
            const memory = (performance as unknown as { memory?: { usedJSHeapSize: number } }).memory;
            return {
                usedJSHeapSize: memory?.usedJSHeapSize ?? null,
                wasm: window.__data_workbench.stats().memory,
            };
        });
        const client = await page.context().newCDPSession(page);
        let heap: number | null = null;
        let taskDuration: number | null = null;
        try {
            heap = ((await client.send("Runtime.getHeapUsage")) as { usedSize: number }).usedSize;
        } catch {
            heap = null;
        }
        try {
            await client.send("Performance.enable");
            const metrics = (await client.send("Performance.getMetrics")) as {
                metrics: { name: string; value: number }[];
            };
            taskDuration = metrics.metrics.find((m) => m.name === "TaskDuration")?.value ?? null;
        } catch {
            taskDuration = null;
        }
        console.log(
            `  A-6: performance.memory ${inPage.usedJSHeapSize}, ` +
                `Runtime.getHeapUsage ${heap}, Performance.TaskDuration ${taskDuration}, ` +
                `wasm committed ${inPage.wasm}`
        );
        // The wasm figure is the one the runtime reports about itself, and
        // R32's CPU side is nothing without it.
        expect(inPage.wasm).toBeGreaterThan(0);
    });

    test("probe 5 (A-7): a sliced sort of the whole sample", async ({ page }) => {
        test.setTimeout(600_000);
        await waitForReady(page, "./table");
        // Before anything is sorted: the application and the second
        // implementation have to be holding the same thirty-two megabytes.
        expect(await page.evaluate(() => window.__data_workbench.dataset_hash())).toBe(twin.hash());
        const sampled = await page.evaluate(() => {
            const api = window.__data_workbench;
            const cells: string[] = [];
            const ids: number[] = [];
            // A thousand cells spread over the whole sample, by an arithmetic
            // both sides can agree on without exchanging a list.
            for (let i = 0; i < 1_000; i++) {
                const row = (i * 97) % 100_000;
                cells.push(api.cell(row, (i * 7) % 20));
                ids.push(api.row_id(row));
            }
            return { cells, ids };
        });
        for (let i = 0; i < 1_000; i++) {
            const row = (i * 97) % twin.ROWS;
            expect(sampled.cells[i]).toBe(twin.cell(row, (i * 7) % twin.COLUMNS));
            expect(sampled.ids[i]).toBe(twin.rowId(row));
        }

        const runs: { ms: number; slices: number }[] = [];
        let framesWhileSorting: number[] = [];
        let lastColumn = 0;
        for (let round = 0; round < R30_LARGE.job_rounds; round++) {
            const column = round % twin.COLUMNS;
            lastColumn = column;
            const result = await page.evaluate(
                async ({ column, sample }) => {
                    const api = window.__data_workbench;
                    // An empty animation frame loop beside the job: a slice
                    // that holds the main thread shows up here as a frame that
                    // never arrived.
                    const intervals: number[] = [];
                    let previous: number | null = null;
                    let running = true;
                    const tick = (now: number) => {
                        if (previous !== null) {
                            intervals.push(now - previous);
                        }
                        previous = now;
                        if (running) {
                            requestAnimationFrame(tick);
                        }
                    };
                    requestAnimationFrame(tick);
                    api.sort_probe(column, true);
                    for (;;) {
                        await new Promise((resolve) => setTimeout(resolve, 4));
                        const state = api.sort_probe_state();
                        if (!state.running) {
                            running = false;
                            return {
                                ms: state.ms,
                                slices: state.slices,
                                rows: state.rows,
                                head: api.sort_probe_order(0, sample),
                                tail: api.sort_probe_order(state.rows - sample, sample),
                                intervals,
                            };
                        }
                    }
                },
                { column, sample: 32 }
            );
            expect(result.rows).toBe(twin.ROWS);
            const expected = twin.sorted(column, true);
            expect(result.head).toEqual(expected.slice(0, 32));
            expect(result.tail).toEqual(expected.slice(-32));
            runs.push({ ms: result.ms, slices: result.slices });
            if (round === 0) {
                framesWhileSorting = result.intervals;
            }
        }
        // One run compared row by row rather than at its ends: two orders that
        // agree on sixty-four rows out of a hundred thousand agree on nothing.
        const whole: number[] = [];
        for (let from = 0; from < twin.ROWS; from += 25_000) {
            whole.push(
                ...(await page.evaluate(
                    ({ from, count }) => window.__data_workbench.sort_probe_order(from, count),
                    { from, count: 25_000 }
                ))
            );
        }
        expect(whole).toEqual(twin.sorted(lastColumn, true));

        const times = runs.map((run) => run.ms);
        const p95 = percentile(times, 95);
        const slices = runs.map((run) => run.slices);
        console.log(
            `  A-7: sort p95 ${p95.toFixed(1)} ms over ${times.length} runs, ` +
                `median ${percentile(times, 50).toFixed(1)} ms in ${percentile(slices, 50)} slices ` +
                `(budget ${R30_LARGE.job_p95_ms} ms) -> ${p95 <= R30_LARGE.job_p95_ms ? "holds" : "does not hold"}`
        );
        console.log(
            `  frames while sorting: p95 ${percentile(framesWhileSorting, 95).toFixed(2)} ms ` +
                `over ${framesWhileSorting.length} intervals; baseline ${BASELINE}`
        );
        expect(times.length).toBe(R30_LARGE.job_rounds);
    });
});
