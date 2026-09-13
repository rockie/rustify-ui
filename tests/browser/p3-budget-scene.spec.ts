import { expect, Page, test } from "@playwright/test";
import { percentile, R30_LARGE } from "./budgets";
import { B3, BASELINE } from "./loads";
import * as twin from "./dataset";

/// R30 AC2 over B3, asserted - on headed Chrome and nowhere else.
///
/// M1's second probe measured the same scene, driven the same way, under both:
/// p95 50.10 ms under headless Chromium's SwiftShader against 17.60 ms on
/// headed Chrome 152. SwiftShader is a software rasteriser and R30's figures
/// are about a machine with a GPU, so a failure under it would say nothing
/// about the budget and a pass would say nothing about the machine. That is
/// why this file is its own project rather than another describe block in
/// `p3-budget-data.spec.ts`, and why CI does not run it.
///
/// The gate is the interval between **effective presentations** - frames in
/// which the region actually drew something new, counted by the runtime's own
/// `stats().frames` - and not the interval between animation frame callbacks,
/// which keeps arriving at 60 Hz in front of a frozen picture.

/// The negative probe: the drive continues, the scene is frozen, and the
/// budget has to be missed.
const PROBE_MS = 5_000;

interface FrameRun {
    samples: number[];
    drives: number;
    effective: number;
    callbacks: number;
    /// Pumps the runtime ran over the same window. A presentation needs a
    /// pump, so the two together say whether a frame was never asked for or
    /// was asked for and not delivered.
    pumps: number;
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
            `(${run.callbacks} callbacks, ${run.pumps} pumps)` +
            ` -> ${holds ? "holds" : "does not hold"}`
    );
    return { p95, share, holds };
}

async function openScene(page: Page) {
    await page.goto("./scene");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 120_000,
    });
    await expect(page.getByTestId("scene-gpu")).toHaveCount(1);
    // Until the region has drawn once its pane is nothing, and a camera would
    // be clamped against nothing.
    await expect
        .poll(
            async () =>
                (await page.evaluate(() => window.__data_workbench.snapshot())).scene.drawn,
            { timeout: 60_000 }
        )
        .toBeGreaterThan(0);
}

/// Pans the camera from inside the page and records what the region presented.
///
/// `freeze_scene(true)` is the application's own switch for the negative
/// probe: the region still pumps and still receives props, but it does not
/// apply the camera, so the picture stops while the drive continues.
async function panScene(
    page: Page,
    ms: number,
    warmupMs: number,
    frozen: boolean
): Promise<FrameRun> {
    await page.evaluate((frozen) => window.__data_workbench.freeze_scene(frozen), frozen);
    try {
        return await page.evaluate(
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
                        if (stats.frames !== seen) {
                            seen = stats.frames;
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
    } finally {
        await page.evaluate(() => window.__data_workbench.freeze_scene(false));
    }
}

test.describe("R30 AC2 over B3: panning ten thousand objects", () => {
    test(`${R30_LARGE.runs} runs of ${R30_LARGE.seconds} s, each under ${R30_LARGE.frame_p95_ms} ms`, async ({
        page,
        browserName,
    }, info) => {
        test.setTimeout(3_600_000);
        // Refusing to run is right; passing quietly under a software
        // rasteriser and calling it R30 is not.
        const headed = !(info.project.use.headless ?? true);
        test.skip(
            !headed || browserName !== "chromium",
            "R30 AC2 over B3 is gated on headed Chrome only (A-3, M1 probe 2): " +
                "under SwiftShader the same scene measures p95 50.10 ms against 17.60 ms headed, " +
                "so neither a pass nor a failure here would be about the budget"
        );
        console.log(
            `baseline ${BASELINE}: ${B3.objects} objects, ${B3.layers} layers, ` +
                `labels of ${B3.labelLength} characters`
        );
        await openScene(page);

        // The negative probe first. If a frozen scene meets the budget, the
        // gate is counting callbacks and every figure below is worthless.
        const frozen = await panScene(page, PROBE_MS, 0, true);
        const frozenVerdict = verdict("negative probe, scene frozen", frozen);
        expect(frozen.callbacks, "the probe has to keep driving").toBeGreaterThan(60);
        expect(
            frozenVerdict.holds,
            "a frozen scene met the frame budget: the gate is measuring callbacks, not presentations"
        ).toBe(false);

        // Five independent runs, each of which has to hold on its own.
        const held: boolean[] = [];
        for (let run = 0; run < R30_LARGE.runs; run++) {
            const measured = await panScene(
                page,
                R30_LARGE.seconds * 1_000,
                R30_LARGE.warmup_seconds * 1_000,
                false
            );
            const result = verdict(`B3 pan, run ${run + 1}`, measured);
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

        // Every drive the region accepted was a camera it actually moved to:
        // a run whose drives were dropped would have a flat frame interval and
        // nothing on screen.
        const asked = await page.evaluate(
            () => window.__data_workbench.snapshot().scene.asked
        );
        expect(asked).toBeGreaterThan(0);
    });

    test("and what it drew is the independent list's, not its own", async ({
        page,
        browserName,
    }, info) => {
        test.setTimeout(600_000);
        const headed = !(info.project.use.headless ?? true);
        test.skip(!headed || browserName !== "chromium", "headed Chrome only, as above");
        await openScene(page);

        // C-5 again: how many objects the scene should be able to see from a
        // given camera comes from a second implementation of the layout.
        for (const [x, y] of [
            [0, 0],
            [B3.extent[0] / 2, B3.extent[1] / 2],
            [B3.extent[0], B3.extent[1]],
        ]) {
            await page.evaluate(([x, y]) => window.__data_workbench.look_at(x, y), [x, y]);
            // The camera the application settled on, not the one asked for:
            // it is clamped to the extent, and the twin has to be asked about
            // the same place.
            const scene = (await page.evaluate(() => window.__data_workbench.snapshot())).scene;
            const [cx, cy] = scene.camera;
            const [width, height] = scene.pane;
            expect(
                await page.evaluate(() => window.__data_workbench.scene_visible()),
                `camera ${x},${y}`
            ).toBe(twin.sceneVisible(cx, cy, width, height));
        }
    });
});
