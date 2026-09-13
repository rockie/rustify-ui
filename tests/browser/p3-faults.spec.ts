import { expect, Page, test } from "@playwright/test";
import { settle, waitForReady } from "./support";

/// M7 · four kinds of failure, twenty times each, against a written
/// expectation.
///
/// The four are not variations of one thing. They fail in different places
/// and they cost different things, and the point of running them together is
/// that each one has an answer written down before it is run:
///
/// | Fault | What it costs | What must be true afterwards |
/// | --- | --- | --- |
/// | A font that does not arrive | the glyphs a region would have drawn | the runtime starts, both halves move one value, the asset is named in the record |
/// | An answer that arrives after a newer one | nothing | the newer answer stands, and the late one changes neither it nor the state |
/// | A scope closed from inside a region action | the scope, deliberately | the runtime is alive and unfailed, no region is left, and a fresh scope works |
/// | A GPU context lost and given back | the pixels, until the rebuild | every confirmed value is still the application's, and the region draws again |
///
/// "Confirmed" means what the application has accepted and reported, not what
/// a control shows: the page's own snapshot is the record, and a fault may
/// cost a frame, a context or an answer without costing one of those.

const ROUNDS = 20;
const FONT = "makepad_widgets/resources/IBMPlexSans-Text.ttf";

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());
const diagnostics = (page: Page) =>
    page.evaluate(() => window.__property_workbench.diagnostics());
const runtime = (page: Page) =>
    page.evaluate(() => ({
        regions: window.__property_workbench.live_regions(),
        errors: window.__property_workbench.hooks.runtime.errors.length,
    }));

/// What the application has confirmed: the values it accepted, and whether
/// its region is usable.
async function confirmed(page: Page) {
    const state = await snapshot(page);
    return {
        selected: state.selected,
        name: state.name,
        count: state.count,
        region: state.region,
    };
}

/// Gives the application something to lose: an object selected by name and a
/// value only this round typed.
async function establish(page: Page, round: number) {
    await page.getByTestId("name-input").fill(`round ${round}`);
    await expect.poll(async () => (await snapshot(page)).name).toBe(`round ${round}`);
    return confirmed(page);
}

/// Sets what the server does wrong from now on; the document that reads it is
/// the next one loaded.
async function fault(page: Page, spec: string) {
    const response = await page.request.get(`./__fault/${spec}`);
    expect(response.ok()).toBe(true);
}

test.afterEach(async ({ page }) => {
    await fault(page, "none");
});

test.describe("M7 V7: a font that does not arrive", () => {
    test(`${ROUNDS} loads without it cost glyphs and nothing else`, async ({ page }) => {
        test.setTimeout(1_800_000);
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await fault(page, `missing:${FONT}`);

        for (let round = 0; round < ROUNDS; round++) {
            // One document per occurrence: a 404 for a font happens while a
            // region is starting, so twenty of them are twenty starts.
            await waitForReady(page);
            expect(await page.evaluate(() => window.__property_workbench.live_regions()), `round ${round}`).toBe(1);

            // Both halves still move one value together.
            await page.getByTestId("select-next").click();
            await expect(page.getByTestId("selected-id")).toHaveText("2");
            const after = await establish(page, round);
            expect(
                { selected: after.selected, name: after.name, region: after.region },
                `round ${round}`
            ).toEqual({ selected: 2, name: `round ${round}`, region: "ready" });
            expect(after.count, `round ${round}`).toBeGreaterThan(0);

            // The file that did not arrive is named, with what to do about it.
            const assets = (await diagnostics(page)).entries.filter(
                (entry) => entry.kind === "AssetLoadFailed"
            );
            expect(assets.length, `round ${round}`).toBeGreaterThan(0);
            expect(assets[0].asset).toContain("IBMPlexSans-Text.ttf");
            expect(assets[0].suggestion).toContain("deployed beside the build");
        }
        expect(failures).toEqual([]);
    });
});

test.describe("M7 V7: an answer that arrives after a newer one", () => {
    test(`${ROUNDS} pairs answered backwards leave the newer answer standing`, async ({ page }) => {
        test.setTimeout(900_000);
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await waitForReady(page);
        const details = page.getByTestId("details");

        for (let round = 0; round < ROUNDS; round++) {
            const before = await establish(page, round);
            await page.evaluate(
                (round) => window.__property_workbench.start_load(400, `stale ${round}`),
                round
            );
            await page.evaluate(
                (round) => window.__property_workbench.start_load(20, `fresh ${round}`),
                round
            );
            await expect(details).toHaveText(`fresh ${round}`);
            // Long enough for the older request to answer into a page that
            // has moved on.
            await page.waitForTimeout(500);
            await expect(details, `round ${round}`).toHaveText(`fresh ${round}`);
            expect(await confirmed(page), `round ${round}`).toEqual(before);
        }

        expect(await runtime(page)).toEqual({ regions: 1, errors: 0 });
        expect(failures).toEqual([]);
    });
});

test.describe("M7 V7: a scope closed from inside a region action", () => {
    test(`${ROUNDS} closures leave the runtime alive and a fresh scope working`, async ({
        page,
    }) => {
        test.setTimeout(1_800_000);
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await waitForReady(page);

        for (let round = 0; round < ROUNDS; round++) {
            const region = page.getByTestId("workbench-gpu");
            await settle(region);
            // Measured every round: the scope is torn down and built again
            // between them, and a pointer aimed at where the canvas was is a
            // pointer aimed at nothing.
            const box = (await region.boundingBox())!;
            const before = await snapshot(page);
            await page.evaluate(() => window.__property_workbench.close_on_next_action());
            // The region's own "next" control. The application closes its
            // scope from inside the callback this click delivers, while the
            // pump that produced it is still on the JS stack.
            await page.mouse.click(box.x + 100, box.y + box.height * 0.06);
            await expect(page.getByTestId("workbench-gpu"), `round ${round}`).toHaveCount(0);

            // Nothing is left of the scope, and the runtime that ran it is
            // neither failed nor holding anything.
            await expect
                .poll(async () => await runtime(page), { timeout: 15_000 })
                .toEqual({ regions: 0, errors: 0 });
            await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready");

            // A new scope works. The objects the application holds outlive
            // the scope that was showing them - that is where they live - so
            // what a closure may not cost is one of them, and what the new
            // scope has to do is take an action and report it.
            await page.evaluate(() => window.__property_workbench.mount());
            await expect(page.getByTestId("workbench-gpu")).toHaveCount(1);
            const after = await snapshot(page);
            expect(after.count, `round ${round}`).toBe(before.count);
            const at = Number(await page.getByTestId("selected-id").textContent());
            await page.getByTestId("select-next").click();
            await expect(page.getByTestId("selected-id"), `round ${round}`).toHaveText(
                String(at + 1)
            );
        }
        expect(failures).toEqual([]);
    });
});

test.describe("M7 V7: a GPU context lost and given back", () => {
    test(`${ROUNDS} losses cost pixels, not values`, async ({ page }) => {
        test.setTimeout(1_800_000);
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);

        for (let round = 0; round < ROUNDS; round++) {
            const before = await establish(page, round);
            expect(
                await page.evaluate(() =>
                    window.__property_workbench.lose_context("workbench-gpu")
                ),
                `round ${round}`
            ).toBe(true);
            await expect
                .poll(async () => (await snapshot(page)).region, { timeout: 15_000 })
                .toBe("lost");
            // Nothing the application holds was in the region.
            expect(await confirmed(page), `round ${round} while lost`).toEqual({
                ...before,
                region: "lost",
            });

            expect(
                await page.evaluate(() =>
                    window.__property_workbench.restore_context("workbench-gpu")
                )
            ).toBe(true);
            await expect
                .poll(async () => (await snapshot(page)).region, { timeout: 20_000 })
                .toBe("ready");
            expect(await confirmed(page), `round ${round} after the rebuild`).toEqual(before);
            await settle(region);
        }

        // Each loss is recorded once, as something to do nothing about.
        const lost = (await diagnostics(page)).entries.filter(
            (entry) => entry.kind === "GpuContextLost"
        );
        expect(lost.length).toBe(ROUNDS);
        expect(lost[0].suggestion).toContain("rebuilt with the current state");
        // The rebuilt region answers a real pointer.
        const box = (await region.boundingBox())!;
        await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.7);
        await expect.poll(async () => (await snapshot(page)).selected).not.toBe(1);
        expect(failures).toEqual([]);
    });
});
