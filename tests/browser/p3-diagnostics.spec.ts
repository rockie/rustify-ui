import { expect, test } from "@playwright/test";
import { percentile } from "./budgets";
import { settle, waitForReady } from "./support";

/// M7 · what keeping the record costs the thing it is recording.
///
/// R39 AC2: the diagnostics may not make the application slower than it is
/// without them. The measurement is the one R30 uses - a thousand actions
/// across the region boundary, timed inside the page to the frame that
/// presents each result - run twice on the same page: once with the record
/// on, which is how it ships, and once with it off.
///
/// Both halves of the boundary, alternating, because one of them alone is
/// half of what the budget is about: a click the region receives whose answer
/// the DOM shows, and an edit the DOM receives whose answer the region draws.

const ACTIONS = 1_000;
/// What the record may cost, as a share of the p95 without it.
const ALLOWED_GROWTH = 0.05;

test.describe("M7 V7 / R39 AC2: the cost of the record", () => {
    test(`${ACTIONS} cross-region actions with the record on and off`, async ({ page }) => {
        test.setTimeout(1_800_000);
        await waitForReady(page);
        await settle(page.getByTestId("workbench-gpu"));

        // The names have to differ between runs: two objects may not share
        // one, and the application refuses the second - which would look
        // exactly like an action that never arrived.
        const run = (actions: number, tag: string) =>
            page.evaluate(async ({ actions, tag }) => {
                const api = window.__property_workbench;
                const canvas = document.querySelector(
                    '[data-testid="workbench-gpu"]'
                ) as HTMLElement;
                const field = document.querySelector(
                    '[data-testid="name-input"]'
                ) as HTMLInputElement;
                const shown = () =>
                    document.querySelector('[data-testid="selected-id"]')?.textContent ?? "";
                const frame = () => new Promise((resolve) => requestAnimationFrame(resolve));
                const box = canvas.getBoundingClientRect();
                const at = { x: box.left + 100, y: box.top + box.height * 0.06 };
                const click = () => {
                    for (const type of ["pointerdown", "pointerup"]) {
                        canvas.dispatchEvent(
                            new PointerEvent(type, {
                                bubbles: true,
                                cancelable: true,
                                clientX: at.x,
                                clientY: at.y,
                                pointerId: 1,
                                pointerType: "mouse",
                                button: 0,
                                buttons: type === "pointerdown" ? 1 : 0,
                            })
                        );
                    }
                };

                const before = api.snapshot();
                const taken: number[] = [];
                let stalled = -1;
                for (let round = 0; round < actions; round++) {
                    const started = performance.now();
                    let waited = 0;
                    if (round % 2 === 0) {
                        const was = shown();
                        click();
                        while (shown() === was && waited < 180) {
                            await frame();
                            waited += 1;
                        }
                    } else {
                        const wanted = `${tag}-${round}`;
                        field.value = wanted;
                        field.dispatchEvent(new Event("input", { bubbles: true }));
                        while (api.snapshot().name !== wanted && waited < 180) {
                            await frame();
                            waited += 1;
                        }
                    }
                    if (waited >= 180) {
                        stalled = round;
                        break;
                    }
                    // The frame that presents it.
                    await frame();
                    taken.push(performance.now() - started);
                }
                const after = api.snapshot();
                return {
                    taken,
                    stalled,
                    refused: after.refused - before.refused,
                    errors: api.stats().errors,
                    region: after.region,
                };
            }, { actions, tag });

        // The record on, which is how it ships. A thousand actions before the
        // comparison as well, so neither run is the one that warms the page.
        expect(await page.evaluate(() => window.__property_workbench.diagnostics().recording)).toBe(
            true
        );
        await run(200, "warm");
        const on = await run(ACTIONS, "on");
        expect(on.stalled).toBe(-1);
        expect(on.taken).toHaveLength(ACTIONS);
        expect(on.refused).toBe(0);
        expect(on.errors).toBe(0);

        // The switch answers with the value it replaced, so what says it
        // took is the record itself.
        await page.evaluate(() => window.__property_workbench.set_diagnostics(false));
        expect(await page.evaluate(() => window.__property_workbench.diagnostics().recording)).toBe(
            false
        );
        const off = await run(ACTIONS, "off");
        expect(off.stalled).toBe(-1);
        expect(off.taken).toHaveLength(ACTIONS);
        expect(off.refused).toBe(0);
        expect(off.errors).toBe(0);

        const onP95 = percentile(on.taken, 95);
        const offP95 = percentile(off.taken, 95);
        const growth = (onP95 - offP95) / offP95;
        console.log(
            `diagnostics: p95 ${onP95.toFixed(2)} ms recording, ${offP95.toFixed(2)} ms not; ` +
                `p50 ${percentile(on.taken, 50).toFixed(2)} against ` +
                `${percentile(off.taken, 50).toFixed(2)}; ` +
                `growth ${(growth * 100).toFixed(1)}%`
        );
        expect(growth).toBeLessThanOrEqual(ALLOWED_GROWTH);

        // And the record is a record again afterwards, so nothing later in
        // the project runs against a page that stopped keeping one.
        await page.evaluate(() => window.__property_workbench.set_diagnostics(true));
        expect(await page.evaluate(() => window.__property_workbench.diagnostics().recording)).toBe(
            true
        );
        expect(await page.evaluate(() => window.__property_workbench.snapshot())).toMatchObject({
            region: "ready",
        });
    });
});
