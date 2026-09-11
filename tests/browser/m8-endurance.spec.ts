import { expect, test } from "@playwright/test";
import { settle, waitForReady } from "./support";

/// P1's own endurance fixture: one scope, one region, a thousand objects,
/// ten discrete actions a second. It is not the PRD's B0/B1 load and does not
/// stand in for one; what it answers is whether a run that lasts loses,
/// duplicates or delays an action, and whether anything grows while it does.
///
/// The duration is a parameter because the useful default for a change is a
/// couple of minutes and the useful default for an acceptance is two hours.
/// `RUSTIFY_ENDURANCE_MINUTES=120 npx playwright test m8-endurance` takes the
/// long one; the report says which was run.
const MINUTES = Number(process.env.RUSTIFY_ENDURANCE_MINUTES ?? "2");
const RATE_HZ = 10;

test.describe("M8 V12: a run that lasts", () => {
    test(`${MINUTES} minutes at ten actions a second, losing none of them`, async ({ page }) => {
        test.setTimeout(MINUTES * 60_000 + 300_000);
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;

        // The working set first, and *then* the run.
        //
        // Every object this fixture holds has a name, a position and a size
        // that the region draws, and drawing a string it has not drawn before
        // fills a cache: the shaper's and the layouter's, four thousand
        // entries each. Walking a thousand objects is therefore a thousand
        // first sights, and a two-minute run at ten a second is *shorter than
        // one lap*. Measuring the tail of that run measured a cache filling
        // and called it a leak - which is what it looked like for as long as
        // nobody walked the lap first.
        //
        // So the lap is walked here, as fast as the frames allow, and what the
        // run that follows measures is a region that has already seen
        // everything it is about to be shown again. What is left growing then
        // is growth with nothing to blame it on.
        const warm = await page.evaluate(
            async ({ rounds, left, top }) => {
                const api = window.__property_workbench;
                const canvas = document.querySelector('[data-testid="workbench-gpu"]')!;
                const before = api.stats().memory;
                const start = api.snapshot();
                for (let round = 0; round < rounds; round++) {
                    for (const type of ["pointerdown", "pointerup"]) {
                        canvas.dispatchEvent(
                            new PointerEvent(type, {
                                bubbles: true,
                                cancelable: true,
                                clientX: left,
                                clientY: top,
                                pointerId: 1,
                                pointerType: "mouse",
                                button: 0,
                                buttons: type === "pointerdown" ? 1 : 0,
                            })
                        );
                    }
                    await new Promise((resolve) => requestAnimationFrame(resolve));
                }
                await new Promise((resolve) => setTimeout(resolve, 1_000));
                const after = api.snapshot();
                return {
                    rounds,
                    accepted: after.accepted - start.accepted,
                    refused: after.refused - start.refused,
                    before,
                    after: api.stats().memory,
                };
            },
            { rounds: 1_050, left: box.x + 100, top: box.y + box.height * 0.06 }
        );
        console.log(
            `endurance warm-up: ${warm.rounds} objects walked, accepted ${warm.accepted}, ` +
                `refused ${warm.refused}, memory ${warm.before} -> ${warm.after}`
        );
        // The lap is a lap: a thousand and fifty steps over a thousand objects
        // passes every one of them, and every step was taken - a walk the
        // application refused half of would warm half a working set.
        expect(warm.accepted).toBe(warm.rounds);
        expect(warm.refused).toBe(0);

        // Paced by a real timer inside the page. A round trip through the
        // harness per action would set the rate rather than measure it, and at
        // ten a second for two hours it would also never finish.
        const result = await page.evaluate(
            async ({ minutes, rate, left, top }) => {
                const api = window.__property_workbench;
                const canvas = document.querySelector('[data-testid="workbench-gpu"]')!;
                const start = api.snapshot();
                const started = performance.now();
                const deadline = started + minutes * 60_000;
                const interval = 1_000 / rate;
                let sent = 0;
                const memory: number[] = [];
                const listeners: number[] = [];

                // The region's own "next" button, clicked where the region
                // drew it: a real pointer on a real control, not a synthetic
                // call into the application.
                const click = () => {
                    for (const type of ["pointerdown", "pointerup"]) {
                        canvas.dispatchEvent(
                            new PointerEvent(type, {
                                bubbles: true,
                                cancelable: true,
                                clientX: left,
                                clientY: top,
                                pointerId: 1,
                                pointerType: "mouse",
                                button: 0,
                                buttons: type === "pointerdown" ? 1 : 0,
                            })
                        );
                    }
                };

                let next = started;
                while (performance.now() < deadline) {
                    click();
                    sent += 1;
                    if (sent % (rate * 10) === 0) {
                        const stats = api.stats();
                        memory.push(stats.memory);
                        listeners.push(stats.timers + stats.animation_frames + stats.tasks);
                    }
                    next += interval;
                    const wait = next - performance.now();
                    await new Promise((resolve) => setTimeout(resolve, Math.max(0, wait)));
                }

                // A moment for the last batch to be delivered before counting.
                await new Promise((resolve) => setTimeout(resolve, 1_000));
                return {
                    sent,
                    accepted: api.snapshot().accepted - start.accepted,
                    refused: api.snapshot().refused - start.refused,
                    elapsed_ms: performance.now() - started,
                    memory,
                    listeners,
                    errors: api.stats().errors,
                    regions: api.stats().regions,
                };
            },
            {
                minutes: MINUTES,
                rate: RATE_HZ,
                left: box.x + 100,
                top: box.y + box.height * 0.06,
            }
        );

        const rate = (result.sent / (result.elapsed_ms / 1_000)).toFixed(2);
        console.log(
            `endurance: ${MINUTES} min, sent ${result.sent} at ${rate}/s, ` +
                `accepted ${result.accepted}, refused ${result.refused}, ` +
                `memory ${result.memory[0]} -> ${result.memory[result.memory.length - 1]}, ` +
                `live listeners ${result.listeners[result.listeners.length - 1]}, ` +
                `errors ${result.errors}`
        );

        // The rate is the rate that was asked for, near enough to describe.
        expect(Number(rate)).toBeGreaterThan(RATE_HZ * 0.8);
        // Every action arrived exactly once: accepted plus refused accounts
        // for all of them, and nothing was refused on this load.
        expect(result.refused).toBe(0);
        expect(result.accepted).toBe(result.sent);
        // Nothing grew and nothing was left ticking.
        expect(result.regions).toBe(1);
        expect(result.errors).toBe(0);
        // Linear memory never shrinks, so the question is not how much was
        // allocated but whether it is still being allocated at the end. The
        // caches that early growth pays for have been paid for above, in the
        // warm-up lap; what grows *here* is growth in a steady state, and a
        // steady state that keeps growing is a leak. So the last quarter of
        // the run has to be flat, and the whole curve is printed so a reader
        // can see the shape rather than trust a threshold.
        //
        // The lap is also the comparison: seeing a thousand objects for the
        // first time costs real memory, and seeing them again costs almost
        // none. Asserting that difference is what says the first number was a
        // cache filling rather than a leak starting.
        const samples = result.memory;
        const tail = samples.slice(Math.floor(samples.length * 0.75));
        console.log(
            `endurance memory: total growth ${samples[samples.length - 1] - samples[0]} bytes; ` +
                `curve ${samples.join(",")}`
        );
        expect(tail.length).toBeGreaterThan(1);
        expect(tail[tail.length - 1] - tail[0]).toBeLessThan(tail[0] * 0.01);
        // A lap of first sights against a run of repeats. The run is longer
        // than the lap and still costs a fraction of it; if the two ever come
        // out alike, something is growing that the lap did not explain.
        const lap = warm.after - warm.before;
        const run = samples[samples.length - 1] - samples[0];
        console.log(
            `endurance: the first lap cost ${lap} bytes, the ${MINUTES}-minute run ${run}`
        );
        expect(run).toBeLessThan(Math.max(lap / 4, tail[0] * 0.01));
        expect(result.listeners[result.listeners.length - 1]).toBeLessThanOrEqual(
            result.listeners[0] + 1
        );

        // And it is still an application afterwards.
        await expect.poll(async () => (await page.evaluate(() => window.__property_workbench.snapshot())).region).toBe(
            "ready"
        );
    });
});
