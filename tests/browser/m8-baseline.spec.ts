import { expect, Page, test } from "@playwright/test";
import { settle, waitForReady } from "./support";

/// Every number here is taken inside the page. A round trip through the test
/// harness costs ten to thirty times what is being measured, so a figure taken
/// out here would describe the harness.
///
/// None of these are the PRD's B0/B1 loads. The fixture is P1's own: one scope,
/// one region, a thousand objects. The report says so rather than letting a
/// small load stand in for a large one.

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());

const stats = (page: Page) => page.evaluate(() => window.__property_workbench.stats());

test.describe("M8 V11: what this build costs to start", () => {
    test("thirty mounts, measured in the page", async ({ page }) => {
        test.setTimeout(600_000);
        await waitForReady(page);
        await page.evaluate(() => window.__property_workbench.dispose());

        const samples = await page.evaluate(async () => {
            const taken: number[] = [];
            for (let round = 0; round < 30; round++) {
                const started = performance.now();
                window.__property_workbench.mount();
                // Mounting is synchronous up to the first frame the region
                // asks for; that is the number an application waits on.
                await new Promise((resolve) => requestAnimationFrame(resolve));
                taken.push(performance.now() - started);
                window.__property_workbench.dispose();
                await new Promise((resolve) => requestAnimationFrame(resolve));
            }
            return taken;
        });

        expect(samples).toHaveLength(30);
        const sorted = [...samples].sort((a, b) => a - b);
        const median = sorted[15];
        const worst = sorted[29];
        console.log(
            `startup: median ${median.toFixed(1)} ms, worst ${worst.toFixed(1)} ms, ` +
                `samples ${sorted.map((s) => s.toFixed(0)).join(",")}`
        );
        // No budget is claimed here; what is asserted is that the figure is a
        // figure, and that thirty starts in a row do not drift upwards.
        expect(median).toBeGreaterThan(0);
        const firstTen = samples.slice(0, 10).reduce((a, b) => a + b, 0) / 10;
        const lastTen = samples.slice(-10).reduce((a, b) => a + b, 0) / 10;
        expect(lastTen).toBeLessThan(firstTen * 3);

        await page.evaluate(() => window.__property_workbench.mount());
        expect(await snapshot(page)).toMatchObject({ region: "ready" });
    });

    test("a thousand inputs reach the application, and their cost is a figure", async ({
        page,
    }) => {
        test.setTimeout(600_000);
        await waitForReady(page);
        await settle(page.getByTestId("workbench-gpu"));
        const before = await snapshot(page);

        // Driven in the page: a thousand round trips through the harness would
        // measure the harness. What is timed is a keystroke reaching the
        // application and the control coming back in step with it - the whole
        // controlled round trip, not a frame being presented.
        const result = await page.evaluate(async () => {
            const field = document.querySelector('[data-testid="name-input"]') as HTMLInputElement;
            const taken: number[] = [];
            for (let round = 0; round < 1_000; round++) {
                const started = performance.now();
                field.value = `n-${round}`;
                field.dispatchEvent(new Event("input", { bubbles: true }));
                const shown = field.value;
                taken.push(performance.now() - started);
                if (shown !== `n-${round}`) {
                    return { taken, mismatch: round };
                }
            }
            return { taken, mismatch: -1 };
        });

        expect(result.mismatch).toBe(-1);
        expect(result.taken).toHaveLength(1_000);
        const sorted = [...result.taken].sort((a, b) => a - b);
        console.log(
            `input: median ${sorted[500].toFixed(3)} ms, p95 ${sorted[950].toFixed(3)} ms, ` +
                `worst ${sorted[999].toFixed(3)} ms`
        );

        // Every one of them was taken: the last value is the application's,
        // and nothing was refused along the way.
        expect(await snapshot(page)).toMatchObject({
            name: "n-999",
            refusals: before.refusals,
        });
    });
});

test.describe("M8 V12: what a long run leaves behind", () => {
    test("a hundred mount rounds, twenty to warm up and eighty measured", async ({ page }) => {
        test.setTimeout(900_000);
        await waitForReady(page);

        const measured = await page.evaluate(async () => {
            const api = window.__property_workbench;
            const round = async () => {
                api.dispose();
                await new Promise((resolve) => requestAnimationFrame(resolve));
                api.mount();
                await new Promise((resolve) => requestAnimationFrame(resolve));
            };
            // Twenty to warm up: the first rounds pay for caches and atlases
            // that a hundredth round does not.
            for (let i = 0; i < 20; i++) {
                await round();
            }
            const start = api.stats();
            const samples: number[] = [];
            for (let i = 0; i < 80; i++) {
                await round();
                samples.push(api.stats().memory);
            }
            return { start, end: api.stats(), samples };
        });

        console.log(
            `mount rounds: memory ${measured.start.memory} -> ${measured.end.memory}, ` +
                `regions ${measured.end.regions}, timers ${measured.end.timers}, ` +
                `frames ${measured.end.animation_frames}, tasks ${measured.end.tasks}`
        );

        // Browser resources return to their baseline: one live region, and
        // nothing left ticking.
        expect(measured.end.regions).toBe(1);
        expect(measured.end.timers).toBe(0);
        expect(measured.end.tasks).toBe(0);

        // Linear memory never shrinks, so the test is a trend, not a level:
        // the last forty rounds must not keep pushing it up.
        const firstForty = measured.samples[39];
        const lastForty = measured.samples[79];
        expect(lastForty).toBeGreaterThanOrEqual(firstForty - 1);
        expect(lastForty - firstForty).toBeLessThan(firstForty * 0.1);

        expect(await snapshot(page)).toMatchObject({ region: "ready" });
    });

    test("a hundred hide and restore rounds leave the region working", async ({ page }) => {
        test.setTimeout(600_000);
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const before = await stats(page);

        const outcome = await page.evaluate(async () => {
            const canvas = document.querySelector('[data-testid="workbench-gpu"]') as HTMLElement;
            const host = canvas.parentElement as HTMLElement;
            for (let round = 0; round < 100; round++) {
                host.hidden = true;
                await new Promise((resolve) => requestAnimationFrame(resolve));
                host.hidden = false;
                await new Promise((resolve) => requestAnimationFrame(resolve));
            }
            return window.__property_workbench.stats();
        });

        console.log(`hide/restore: memory ${before.memory} -> ${outcome.memory}`);
        expect(outcome.regions).toBe(1);
        expect(outcome.errors).toBe(before.errors);

        // Still a working region afterwards: it answers a real pointer.
        await expect.poll(async () => (await snapshot(page)).region, { timeout: 20_000 }).toBe("ready");
        await settle(region);
        const box = (await region.boundingBox())!;
        await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.7);
        await expect.poll(async () => (await snapshot(page)).selected).not.toBe(1);
    });
});
