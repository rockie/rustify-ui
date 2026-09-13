import { expect, Page, test } from "@playwright/test";
import { percentile, R30_LARGE } from "./budgets";
import { B2, BASELINE } from "./loads";
import * as twin from "./dataset";

/// R30 AC2 and AC3 over B2, asserted.
///
/// Two things are gated here. The first is the interval between **effective
/// presentations** while a hundred thousand rows are scrolled: not the
/// interval between animation frame callbacks, which keeps arriving at 60 Hz
/// while the picture stands still. The content version is the table's own -
/// `window_version()` counts every time it works out its visible range - so a
/// frame in which it did not move is a frame in which the table presented
/// nothing, and it produces no sample.
///
/// The second is what a job over the whole sample costs: a single-column sort
/// and a text filter end to end, and how quickly a cancel is answered.
///
/// Where this runs: headless Chromium under SwiftShader. B2 is DOM work - the
/// rows are elements and the cells are text nodes - so the software rasteriser
/// is not the thing under test, which is why B2's gate lives here and B3's
/// lives in the headed project.

/// The five-second negative probe. The drive continues and the content is
/// frozen; the budget has to be missed. If it is met, the gate is measuring
/// animation frames rather than presentations and every figure below is
/// worthless.
const PROBE_MS = 5_000;

interface FrameRun {
    /// Intervals between effective presentations, in milliseconds.
    samples: number[];
    /// Frames driven, presentations that followed, animation frame callbacks,
    /// and the table's own count of how often it worked out a visible range.
    drives: number;
    effective: number;
    callbacks: number;
    recomputes: number;
}

function verdict(name: string, run: FrameRun) {
    const p95 = run.samples.length === 0 ? Infinity : percentile(run.samples, 95);
    const slow = run.samples.filter((ms) => ms > R30_LARGE.frame_slow_ms).length;
    const share = run.samples.length === 0 ? 1 : slow / run.samples.length;
    const holds = p95 <= R30_LARGE.frame_p95_ms && share <= R30_LARGE.frame_slow_share;
    console.log(
        `  ${name}: p95 ${p95.toFixed(2)} ms, >${R30_LARGE.frame_slow_ms} ms ` +
            `${(share * 100).toFixed(2)}% of ${run.samples.length}, ` +
            `${run.effective} presentations for ${run.drives} drives ` +
            `(${run.callbacks} callbacks, ${run.recomputes} recomputes)` +
            ` -> ${holds ? "holds" : "does not hold"}`
    );
    return { p95, share, holds };
}

async function openTable(page: Page) {
    await page.goto("./table");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 120_000,
    });
    await expect(page.getByTestId("table")).toHaveCount(1);
    // The pool is sized from the viewport, which is whatever the layout gave
    // it: until the table has measured itself there is no window to move.
    await expect
        .poll(async () => page.evaluate(() => window.__data_workbench.window_version()), {
            timeout: 60_000,
        })
        .toBeGreaterThan(0);
}

/// Scrolls the table from inside the page and records what it presented.
///
/// `frozen` re-drives the same scroll position every frame instead of
/// advancing it. That is the negative probe, and it needs no switch in the
/// application: the table's content version is a function of where it is
/// scrolled to, so driving it to where it already is leaves the picture
/// genuinely unchanged while the callbacks keep arriving at 60 Hz.
async function scrollTable(
    page: Page,
    ms: number,
    warmupMs: number,
    frozen: boolean
): Promise<FrameRun> {
    return page.evaluate(
        async ({ ms, warmupMs, rowsPerFrame, frozen }) => {
            const api = window.__data_workbench;
            const scroller = document.querySelector(
                '[data-testid="table-scroller"]'
            ) as HTMLElement;
            const rowHeight = 24;
            const limit = scroller.scrollHeight - scroller.clientHeight;
            return await new Promise<{
                samples: number[];
                drives: number;
                effective: number;
                callbacks: number;
                recomputes: number;
            }>((resolve) => {
                const samples: number[] = [];
                let drives = 0;
                let effective = 0;
                let callbacks = 0;
                let previous: number | null = null;
                let seen = api.window_version();
                let recomputesAtWarmup = seen;
                let top = 0;
                const started = performance.now();
                const tick = (now: number) => {
                    const measuring = now - started >= warmupMs;
                    callbacks += 1;
                    if (!frozen) {
                        top += rowsPerFrame * rowHeight;
                        if (top > limit) {
                            top = 0;
                        }
                    }
                    scroller.scrollTop = top;
                    if (measuring) {
                        drives += 1;
                    } else {
                        recomputesAtWarmup = api.window_version();
                    }
                    const version = api.window_version();
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
                        resolve({
                            samples,
                            drives,
                            effective,
                            callbacks,
                            recomputes: api.window_version() - recomputesAtWarmup,
                        });
                        return;
                    }
                    requestAnimationFrame(tick);
                };
                requestAnimationFrame(tick);
            });
        },
        { ms, warmupMs, rowsPerFrame: B2.scrollRowsPerFrame, frozen }
    );
}

test.describe("R30 AC2 over B2: scrolling a hundred thousand rows", () => {
    test(`${R30_LARGE.runs} runs of ${R30_LARGE.seconds} s, each under ${R30_LARGE.frame_p95_ms} ms`, async ({
        page,
    }) => {
        test.setTimeout(3_600_000);
        console.log(`baseline ${BASELINE}: ${B2.rows} rows x ${B2.columns} columns`);
        await openTable(page);

        // The negative probe, before any figure is believed. The drive
        // continues and the content does not change; this must miss the
        // budget.
        const frozen = await scrollTable(page, PROBE_MS, 0, true);
        const frozenVerdict = verdict("negative probe, content frozen", frozen);
        expect(frozen.callbacks, "the probe has to keep driving").toBeGreaterThan(60);
        expect(
            frozenVerdict.holds,
            "a frozen table met the frame budget: the gate is measuring callbacks, not presentations"
        ).toBe(false);

        // And the real runs. Each of the five has to hold on its own - the
        // budget is not the best of them.
        const held: boolean[] = [];
        for (let run = 0; run < R30_LARGE.runs; run++) {
            const measured = await scrollTable(
                page,
                R30_LARGE.seconds * 1_000,
                R30_LARGE.warmup_seconds * 1_000,
                false
            );
            const result = verdict(`B2 scroll, run ${run + 1}`, measured);
            // Every drive was taken, and almost every drive produced a frame:
            // a table that keeps up with two thirds of the input is not
            // keeping up.
            expect(measured.recomputes, `run ${run + 1}: drives against recomputes`).toBe(
                measured.drives
            );
            expect(measured.effective, `run ${run + 1}: presentations`).toBeGreaterThanOrEqual(
                Math.floor(measured.drives * R30_LARGE.effective_frame_share)
            );
            expect(result.p95, `run ${run + 1}: p95`).toBeLessThanOrEqual(
                R30_LARGE.frame_p95_ms
            );
            expect(result.share, `run ${run + 1}: slow share`).toBeLessThanOrEqual(
                R30_LARGE.frame_slow_share
            );
            held.push(result.holds);
        }
        expect(held).toEqual(Array(R30_LARGE.runs).fill(true));
    });
});

test.describe("R30 AC3 over B2: what a job over the whole sample costs", () => {
    test(`${R30_LARGE.job_rounds} sorts and ${R30_LARGE.job_rounds} filters, each under ${R30_LARGE.job_p95_ms} ms`, async ({
        page,
    }) => {
        test.setTimeout(3_600_000);
        await openTable(page);

        const settle = () =>
            page.evaluate(async () => {
                const api = window.__data_workbench;
                await new Promise<void>((resolve) => {
                    const wait = () => {
                        if (api.snapshot().table.job.running) {
                            requestAnimationFrame(wait);
                        } else {
                            resolve();
                        }
                    };
                    requestAnimationFrame(wait);
                });
                return api.snapshot().table.job.ended;
            });

        // Sorts, timed in the page from the ask to the frame that shows the
        // result. A round trip through the harness costs more than the budget.
        const sorts: number[] = [];
        for (let round = 0; round < R30_LARGE.job_rounds; round++) {
            const ms = await page.evaluate(async ({ column, ascending }) => {
                const api = window.__data_workbench;
                const started = performance.now();
                api.sort(column, ascending);
                await new Promise<void>((resolve) => {
                    const wait = () => {
                        if (api.snapshot().table.job.running) {
                            requestAnimationFrame(wait);
                        } else {
                            resolve();
                        }
                    };
                    requestAnimationFrame(wait);
                });
                await new Promise((resolve) => requestAnimationFrame(resolve));
                return performance.now() - started;
            }, { column: round % B2.columns, ascending: round % 2 === 0 });
            sorts.push(ms);
            expect(await settle()).toBe("done");
        }
        const sortP95 = percentile(sorts, 95);
        console.log(
            `B2 sort of ${B2.rows} rows: p50 ${percentile(sorts, 50).toFixed(0)} ms, ` +
                `p95 ${sortP95.toFixed(0)} ms, worst ${Math.max(...sorts).toFixed(0)} ms`
        );

        // Filters, the same way.
        const filters: number[] = [];
        for (let round = 0; round < R30_LARGE.job_rounds; round++) {
            const needle = twin.cell(round * 4_001, round % B2.columns).slice(0, 3);
            const ms = await page.evaluate(async (needle) => {
                const api = window.__data_workbench;
                const field = document.querySelector(
                    '[data-testid="table-filter"]'
                ) as HTMLInputElement;
                const started = performance.now();
                field.value = needle;
                field.dispatchEvent(new Event("input", { bubbles: true }));
                await new Promise<void>((resolve) => {
                    const wait = () => {
                        if (api.snapshot().table.job.running) {
                            requestAnimationFrame(wait);
                        } else {
                            resolve();
                        }
                    };
                    requestAnimationFrame(wait);
                });
                await new Promise((resolve) => requestAnimationFrame(resolve));
                return performance.now() - started;
            }, needle);
            filters.push(ms);
            expect(await settle()).toBe("done");
        }
        const filterP95 = percentile(filters, 95);
        console.log(
            `B2 filter over ${B2.rows} rows: p50 ${percentile(filters, 50).toFixed(0)} ms, ` +
                `p95 ${filterP95.toFixed(0)} ms, worst ${Math.max(...filters).toFixed(0)} ms`
        );

        expect(sorts).toHaveLength(R30_LARGE.job_rounds);
        expect(filters).toHaveLength(R30_LARGE.job_rounds);
        expect(sortP95).toBeLessThanOrEqual(R30_LARGE.job_p95_ms);
        expect(filterP95).toBeLessThanOrEqual(R30_LARGE.job_p95_ms);
    });

    test(`${R30_LARGE.job_rounds} cancels, each answered within ${R30_LARGE.cancel_p95_ms} ms`, async ({
        page,
    }) => {
        test.setTimeout(1_800_000);
        await openTable(page);

        const times: number[] = [];
        for (let round = 0; round < R30_LARGE.job_rounds; round++) {
            const ms = await page.evaluate(async (column) => {
                const api = window.__data_workbench;
                api.sort(column, true);
                // A cancel before the job has taken a slice is a cancel of
                // nothing: wait until it is genuinely running.
                await new Promise<void>((resolve) => {
                    const wait = () => {
                        if (api.snapshot().table.job.slices > 0) {
                            resolve();
                        } else {
                            requestAnimationFrame(wait);
                        }
                    };
                    requestAnimationFrame(wait);
                });
                // Timed from the ask to the frame that presents the answer.
                const started = performance.now();
                api.cancel_job();
                await new Promise((resolve) => requestAnimationFrame(resolve));
                return performance.now() - started;
            }, round % B2.columns);
            times.push(ms);
            expect(
                (await page.evaluate(() => window.__data_workbench.snapshot())).table.job.ended
            ).toBe("cancelled");
        }
        const p95 = percentile(times, 95);
        console.log(
            `B2 cancel to shown: p50 ${percentile(times, 50).toFixed(1)} ms, ` +
                `p95 ${p95.toFixed(1)} ms, worst ${Math.max(...times).toFixed(1)} ms`
        );
        expect(times).toHaveLength(R30_LARGE.job_rounds);
        expect(p95).toBeLessThanOrEqual(R30_LARGE.cancel_p95_ms);
    });

    test("and the results are the independent list's, not the application's own", async ({
        page,
    }) => {
        test.setTimeout(1_800_000);
        await openTable(page);

        // C-5: the expectation comes from a second implementation of the
        // dataset, not from what the application produced last time.
        for (const column of [0, 7, 19]) {
            await page.evaluate((column) => window.__data_workbench.sort(column, true), column);
            await expect
                .poll(
                    async () =>
                        (await page.evaluate(() => window.__data_workbench.snapshot())).table.job
                            .running,
                    { timeout: 120_000 }
                )
                .toBe(false);
            const shown = await page.evaluate(
                () => window.__data_workbench.view(0, 200) as number[]
            );
            expect(shown, `column ${column}`).toEqual(twin.sorted(column, true).slice(0, 200));
        }
    });
});
