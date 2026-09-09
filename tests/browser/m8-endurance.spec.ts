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
        // allocated but whether it is still being allocated at the end. Early
        // growth is a run warming up - atlases, caches, the allocator reaching
        // its shape - and it stops. A leak does not. So what is asserted is
        // that the last quarter of the run is flat, and the whole curve is
        // printed so a reader can see the shape rather than trust a threshold.
        //
        // How long a run has to be before a plateau can be told from a slow
        // climb is a question a two-minute run cannot answer. That is what the
        // two-hour setting is for; the report says which was run.
        const samples = result.memory;
        const tail = samples.slice(Math.floor(samples.length * 0.75));
        console.log(
            `endurance memory: total growth ${samples[samples.length - 1] - samples[0]} bytes; ` +
                `curve ${samples.join(",")}`
        );
        expect(tail.length).toBeGreaterThan(1);
        expect(tail[tail.length - 1] - tail[0]).toBeLessThan(tail[0] * 0.01);
        expect(result.listeners[result.listeners.length - 1]).toBeLessThanOrEqual(
            result.listeners[0] + 1
        );

        // And it is still an application afterwards.
        await expect.poll(async () => (await page.evaluate(() => window.__property_workbench.snapshot())).region).toBe(
            "ready"
        );
    });
});
